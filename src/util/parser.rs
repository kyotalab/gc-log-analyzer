use crate::model::{GCEvent, GcType, detect_gc_type};
use chrono::{DateTime, FixedOffset};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

// 出力はResult型にラップされ、エラーをマッチできるようになる。
// ファイルの各行のReaderへのイテレータを返す。
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_gc_time(raw: &str) -> Option<DateTime<FixedOffset>> {
    DateTime::parse_from_str(raw, "%Y-%m-%dT%H:%M:%S%.3f%z").ok()
}

// GCログから構造体にマッピングして、データのデシリアライズを行う
pub fn parse_gc_events(lines: Lines<BufReader<File>>) -> Result<Vec<GCEvent>, anyhow::Error> {
    let mut events: Vec<GCEvent> = Vec::new();
    let mut current = GCEvent {
        time: None,
        gc_type: GcType::Unknown,
        has_pause: false,
        pause_time_ms: None,
        eden_before: None,
        eden_after: None,
        eden_total: None,
        survivor_before: None,
        survivor_after: None,
        survivor_total: None,
        old_before: None,
        old_after: None,
        humongous_before: None,
        humongous_after: None,
    };

    // パターン条件を定義
    let re_time = Regex::new(r"\[(.*?)\]").unwrap();
    let re_gc = Regex::new(r"GC\(").unwrap();
    let re_pause = Regex::new("Pause").unwrap();
    let re_pause_time = Regex::new(r"(\d+\.\d+)ms$").unwrap();
    let re_eden = Regex::new("Eden regions").unwrap();
    let re_survivor = Regex::new("Survivor regions").unwrap();
    let re_old = Regex::new("Old regions").unwrap();
    let re_humongous = Regex::new("Humongous regions").unwrap();

    // イテレータを消費し、Option型のStringを返す。
    for line in lines {
        if let Ok(log) = line {
            // ここでパターンマッチングをする
            if re_gc.is_match(&log) {
                if let Some(caps) = re_time.captures(&log) {
                    let time_str = &caps[1];
                    current.time = parse_gc_time(time_str);
                }

                let detected = detect_gc_type(&log);
                if detected != GcType::Unknown {
                    current.gc_type = detected.clone();

                    if re_pause.is_match(&log) {
                        current.has_pause = true;
                    } else if matches!(detected, GcType::Concurrent) {
                        // Concurrent GC はヒープ構成が出ないのでこの時点で push
                        events.push(current.clone()); // Cloneが必要
                        current = GCEvent {
                            time: None,
                            gc_type: GcType::Unknown,
                            has_pause: false,
                            pause_time_ms: None,
                            eden_before: None,
                            eden_after: None,
                            eden_total: None,
                            survivor_before: None,
                            survivor_after: None,
                            survivor_total: None,
                            old_before: None,
                            old_after: None,
                            humongous_before: None,
                            humongous_after: None,
                        };
                    }
                }
            }
            if re_eden.is_match(&log) {
                if let Some(caps) = Regex::new(r"Eden regions: (\d+)->(\d+)\((\d+)\)")
                    .unwrap()
                    .captures(&log)
                {
                    current.eden_before = caps.get(1).map(|m| m.as_str().parse().unwrap());
                    current.eden_after = caps.get(2).map(|m| m.as_str().parse().unwrap());
                    current.eden_total = caps.get(3).map(|m| m.as_str().parse().unwrap());
                }
            }

            if let Some(caps) = re_pause_time.captures(&log) {
                current.pause_time_ms = caps.get(1).map(|m| m.as_str().parse().unwrap());
            }
            if re_survivor.is_match(&log) {
                if let Some(caps) = Regex::new(r"Survivor regions: (\d+)->(\d+)\((\d+)\)")
                    .unwrap()
                    .captures(&log)
                {
                    current.survivor_before = caps.get(1).map(|m| m.as_str().parse().unwrap());
                    current.survivor_after = caps.get(2).map(|m| m.as_str().parse().unwrap());
                    current.survivor_total = caps.get(3).map(|m| m.as_str().parse().unwrap());
                }
            }
            if re_old.is_match(&log) {
                if let Some(caps) = Regex::new(r"Old regions: (\d+)->(\d+)")
                    .unwrap()
                    .captures(&log)
                {
                    current.old_before = caps.get(1).map(|m| m.as_str().parse().unwrap());
                    current.old_after = caps.get(2).map(|m| m.as_str().parse().unwrap());
                }
            }
            if re_humongous.is_match(&log) {
                if let Some(caps) = Regex::new(r"Humongous regions: (\d+)->(\d+)")
                    .unwrap()
                    .captures(&log)
                {
                    current.humongous_before = caps.get(1).map(|m| m.as_str().parse().unwrap());
                    current.humongous_after = caps.get(2).map(|m| m.as_str().parse().unwrap());

                    if current.has_pause {
                        events.push(current);
                    }
                    current = GCEvent {
                        time: None,
                        gc_type: GcType::Unknown,
                        has_pause: false,
                        pause_time_ms: None,
                        eden_before: None,
                        eden_after: None,
                        eden_total: None,
                        survivor_before: None,
                        survivor_after: None,
                        survivor_total: None,
                        old_before: None,
                        old_after: None,
                        humongous_before: None,
                        humongous_after: None,
                    };
                }
            }
        }
    }
    Ok(events)
}
