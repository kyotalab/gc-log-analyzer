use clap::Parser;
use csv::Writer;
use gc_log_analyzer::{arg::*, util::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let lines = read_lines(&args.input)?;
    let gc_events = parse_gc_events(lines)?;

    // CSV出力オプション対応
    if let Some(csv_path) = args.csv {
        let mut wtr = Writer::from_path(&csv_path)?;
        for event in &gc_events {
            wtr.serialize(event)?;
        }
        wtr.flush()?;
        println!("✅ Exported the CSV: {}", &csv_path);
    }

    match args.mode.as_str() {
        "heap" => {
            draw_heap_chart(&gc_events, &args.plot)?;
            println!("✅ Heap Graph saved: {}", &args.plot);
        }
        "pause" => {
            draw_pause_chart(&gc_events, &args.plot)?;
            println!("✅ Pause Graph saved: {}", &args.plot);
        }
        "combined" => {
            draw_combined_chart(&gc_events, &args.plot)?;
            println!("✅ Combined Graph saved: {}", &args.plot);
        }
        _ => {
            eprintln!("❌ Invalid mode specified（heap, pause, combined）");
        }
    }

    if args.summary {
        let gc_type_counts = count_gc_types(&gc_events);
        print_gc_type_summary(&gc_type_counts);
    }

    Ok(())
}
