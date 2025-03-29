use std::time::Duration;

pub fn format_duration(d: Duration) -> String {
    let nanos = d.as_nanos();
    [
        (1_000_000_000, "s"),
        (1_000_000, "ms"),
        (1_000, "µs"),
        (1, "ns"),
    ]
    .iter()
    .find(|&&(factor, _)| nanos >= factor)
    .map(|&(factor, unit)| format!("{:.1}{unit}", nanos as f64 / factor as f64))
    .unwrap_or_else(|| "pretty quick".to_string())
}
