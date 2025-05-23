use crate::model::{GCEvent, GcType};
use plotters::prelude::*;
use std::collections::HashMap;

pub fn draw_heap_chart(
    events: &[GCEvent],
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output_path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let filtered: Vec<&GCEvent> = events
        .iter()
        .filter(|e| e.has_pause && e.time.is_some())
        .collect();
    if filtered.is_empty() {
        return Err("No drawable events".into());
    }

    let min_time = filtered.first().unwrap().time.unwrap();
    let max_time = filtered.last().unwrap().time.unwrap();

    let max_y = filtered
        .iter()
        .flat_map(|e| {
            vec![
                e.eden_before.unwrap_or(0.0),
                e.eden_after.unwrap_or(0.0),
                e.survivor_before.unwrap_or(0.0),
                e.survivor_after.unwrap_or(0.0),
                e.old_before.unwrap_or(0.0),
                e.old_after.unwrap_or(0.0),
                e.humongous_before.unwrap_or(0.0),
                e.humongous_after.unwrap_or(0.0),
            ]
        })
        .fold(0.0, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("GC heap space usage (Before/After)", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(min_time..max_time, 0.0..(max_y + 20.0))?;

    chart
        .configure_mesh()
        .x_labels(10)
        .x_label_formatter(&|dt| dt.format("%H:%M:%S").to_string())
        .y_desc("Number of Regions")
        .x_desc("time")
        .label_style(("sans-serif", 20))
        .draw()?;

    macro_rules! draw_dual_series {
        ($name:expr, $color:expr, $after_accessor:expr, $before_accessor:expr) => {{
            let after_style = ShapeStyle::from($color).stroke_width(2);
            let before_style = ShapeStyle::from($color.mix(0.4)).stroke_width(2);

            chart
                .draw_series(LineSeries::new(
                    filtered
                        .iter()
                        .map(|e| (e.time.unwrap(), $after_accessor(e))),
                    after_style,
                ))?
                .label(format!("{} After", $name))
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], after_style));

            chart
                .draw_series(LineSeries::new(
                    filtered
                        .iter()
                        .map(|e| (e.time.unwrap(), $before_accessor(e))),
                    before_style,
                ))?
                .label(format!("{} Before", $name))
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], before_style));
        }};
    }

    draw_dual_series!(
        "Eden",
        &BLUE,
        |e: &GCEvent| e.eden_after.unwrap_or(0.0),
        |e: &GCEvent| e.eden_before.unwrap_or(0.0)
    );
    draw_dual_series!(
        "Survivor",
        &RED,
        |e: &GCEvent| e.survivor_after.unwrap_or(0.0),
        |e: &GCEvent| e.survivor_before.unwrap_or(0.0)
    );
    draw_dual_series!(
        "Old",
        &GREEN,
        |e: &GCEvent| e.old_after.unwrap_or(0.0),
        |e: &GCEvent| e.old_before.unwrap_or(0.0)
    );
    draw_dual_series!(
        "Humongous",
        &MAGENTA,
        |e: &GCEvent| e.humongous_after.unwrap_or(0.0),
        |e: &GCEvent| e.humongous_before.unwrap_or(0.0)
    );

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .label_font(("sans-serif", 15))
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

pub fn draw_pause_chart(
    events: &[GCEvent],
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output_path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let filtered: Vec<&GCEvent> = events
        .iter()
        .filter(|e| e.has_pause && e.time.is_some() && e.pause_time_ms.is_some())
        .collect();
    if filtered.is_empty() {
        return Err("No drawable events".into());
    }

    let min_time = filtered.first().unwrap().time.unwrap();
    let max_time = filtered.last().unwrap().time.unwrap();

    let max_y = filtered
        .iter()
        .map(|e| e.pause_time_ms.unwrap_or(0.0))
        .fold(0.0, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("GC Pause Time", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(min_time..max_time, 0.0..(max_y + 20.0))?;

    chart
        .configure_mesh()
        .x_labels(10)
        .x_label_formatter(&|dt| dt.format("%H:%M:%S").to_string())
        .y_desc("Pause time (ms)")
        .x_desc("time")
        .label_style(("sans-serif", 20))
        .draw()?;

    let pause_style = ShapeStyle::from(&BLACK).stroke_width(2);
    chart
        .draw_series(LineSeries::new(
            filtered
                .iter()
                .map(|e| (e.time.unwrap(), e.pause_time_ms.unwrap())),
            pause_style,
        ))?
        .label("Pause Time (ms)")
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], pause_style));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .label_font(("sans-serif", 15))
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

pub fn draw_combined_chart(
    events: &[GCEvent],
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output_path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let filtered: Vec<&GCEvent> = events
        .iter()
        .filter(|e| e.has_pause && e.time.is_some())
        .collect();
    if filtered.is_empty() {
        return Err("No drawable events".into());
    }

    let min_time = filtered.first().unwrap().time.unwrap();
    let max_time = filtered.last().unwrap().time.unwrap();

    let max_y = filtered
        .iter()
        .flat_map(|e| {
            vec![
                e.eden_before.unwrap_or(0.0),
                e.eden_after.unwrap_or(0.0),
                e.survivor_before.unwrap_or(0.0),
                e.survivor_after.unwrap_or(0.0),
                e.old_before.unwrap_or(0.0),
                e.old_after.unwrap_or(0.0),
                e.humongous_before.unwrap_or(0.0),
                e.humongous_after.unwrap_or(0.0),
                e.pause_time_ms.unwrap_or(0.0),
            ]
        })
        .fold(0.0, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("GC Heap Usage & Pause Time (Overlay)", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(min_time..max_time, 0.0..(max_y + 20.0))?;

    chart
        .configure_mesh()
        .x_labels(10)
        .x_label_formatter(&|dt| dt.format("%H:%M:%S").to_string())
        .y_desc("Number of Regions / Pause Time (ms)")
        .x_desc("時刻")
        .label_style(("sans-serif", 20))
        .draw()?;

    macro_rules! draw_dual_series {
        ($name:expr, $color:expr, $after_accessor:expr, $before_accessor:expr) => {{
            let after_style = ShapeStyle::from($color).stroke_width(2);
            let before_style = ShapeStyle::from($color.mix(0.4)).stroke_width(2);

            chart
                .draw_series(LineSeries::new(
                    filtered
                        .iter()
                        .map(|e| (e.time.unwrap(), $after_accessor(e))),
                    after_style,
                ))?
                .label(format!("{} After", $name))
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], after_style));

            chart
                .draw_series(LineSeries::new(
                    filtered
                        .iter()
                        .map(|e| (e.time.unwrap(), $before_accessor(e))),
                    before_style,
                ))?
                .label(format!("{} Before", $name))
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], before_style));
        }};
    }

    draw_dual_series!(
        "Eden",
        &BLUE,
        |e: &GCEvent| e.eden_after.unwrap_or(0.0),
        |e: &GCEvent| e.eden_before.unwrap_or(0.0)
    );
    draw_dual_series!(
        "Survivor",
        &RED,
        |e: &GCEvent| e.survivor_after.unwrap_or(0.0),
        |e: &GCEvent| e.survivor_before.unwrap_or(0.0)
    );
    draw_dual_series!(
        "Old",
        &GREEN,
        |e: &GCEvent| e.old_after.unwrap_or(0.0),
        |e: &GCEvent| e.old_before.unwrap_or(0.0)
    );
    draw_dual_series!(
        "Humongous",
        &MAGENTA,
        |e: &GCEvent| e.humongous_after.unwrap_or(0.0),
        |e: &GCEvent| e.humongous_before.unwrap_or(0.0)
    );

    if filtered.iter().any(|e| e.pause_time_ms.is_some()) {
        let pause_style = ShapeStyle::from(&BLACK).stroke_width(2);
        chart
            .draw_series(LineSeries::new(
                filtered
                    .iter()
                    .filter(|e| e.pause_time_ms.is_some())
                    .map(|e| (e.time.unwrap(), e.pause_time_ms.unwrap())),
                pause_style,
            ))?
            .label("Pause Time (ms)")
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], pause_style));
    }

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .label_font(("sans-serif", 15))
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

pub fn count_gc_types(events: &[GCEvent]) -> HashMap<GcType, usize> {
    let mut counts = HashMap::new();
    for event in events {
        *counts.entry(event.gc_type.clone()).or_insert(0) += 1;
    }
    counts
}

pub fn print_gc_type_summary(counts: &HashMap<GcType, usize>) {
    println!("GCイベント種別別の発生回数");
    println!("────────────────────────");
    for (gc_type, count) in counts {
        println!("{:<15} : {:>4} 回", format!("{:?}", gc_type), count);
    }
}
