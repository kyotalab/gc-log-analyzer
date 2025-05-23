use chrono::{DateTime, FixedOffset};
use serde::Serialize;

// GC Eventマッピング用の構造体の定義
#[derive(Debug, Serialize, Clone)]
pub struct GCEvent {
    pub time: Option<DateTime<FixedOffset>>,
    pub gc_type: GcType,
    pub has_pause: bool,
    pub pause_tile_ms: Option<f64>,
    pub eden_before: Option<f64>,
    pub eden_after: Option<f64>,
    pub eden_total: Option<f64>,
    pub survivor_before: Option<f64>,
    pub survivor_after: Option<f64>,
    pub survivor_total: Option<f64>,
    pub old_before: Option<f64>,
    pub old_after: Option<f64>,
    pub humongous_before: Option<f64>,
    pub humongous_after: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum GcType {
    Young,
    Full,
    Concurrent,
    Unknown,
}

pub fn detect_gc_type(line: &str) -> GcType {
    if line.contains("Pause Young")
        || line.contains("Evacuation Pause")
        || line.contains("G1 Humongous Allocation")
    {
        GcType::Young
    } else if line.contains("Full GC") || line.contains("Pause Full") || line.contains("G1 Full GC")
    {
        GcType::Full
    } else if line.contains("GC concurrent") || line.contains("Concurrent Cycle") {
        GcType::Concurrent
    } else {
        GcType::Unknown
    }
}
