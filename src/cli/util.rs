use crate::{interpreter::Analytics, program::Instruction};
use colored::Colorize;
use std::{collections::HashMap, time::Duration};
use tabled::{
    builder::Builder,
    settings::{Style, Width, peaker::Priority, style::BorderSpanCorrection},
};

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

fn table_builder(columns: &[&str]) -> Builder {
    let mut builder = Builder::default();

    let col_header: Vec<_> = columns.iter().map(|t| t.bold().to_string()).collect();
    builder.push_record(col_header);
    builder
}

fn build_table(builder: Builder) -> String {
    builder
        .build()
        .with(Style::modern())
        .with(BorderSpanCorrection)
        .with(Width::wrap(80).priority(Priority::max(true)))
        .to_string()
}

pub fn build_frequency_table(analytics: &Analytics) -> String {
    let mut builder = table_builder(&["Instruction", "Count"]);
    let mut merged = HashMap::new();

    // Merge instructions by type
    for (key, count) in &analytics.frequency {
        let (instr, multiplier) = match key {
            Instruction::MoveRight(value) => ("MoveRight", *value as u64),
            Instruction::MoveLeft(value) => ("MoveLeft", *value as u64),
            Instruction::Add(value) => ("Add", *value as u64),
            Instruction::Sub(value) => ("Sub", *value as u64),
            Instruction::Loop { .. } => todo!("Needs to accurately track [ and ]"),
            Instruction::Print => ("Print", 1u64),
            Instruction::Read => ("Read", 1u64),
            Instruction::Set(_) => ("Set", 1u64),
        };
        *merged.entry(instr).or_insert(0) += count * multiplier;
    }

    let mut entries: Vec<_> = merged.iter().collect();
    entries.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));

    for (key, value) in entries {
        builder.push_record([(*key).to_string(), value.to_string()]);
    }

    build_table(builder)
}

pub fn build_loop_patterns_table(analytics: &Analytics) -> String {
    let mut builder = table_builder(&["Loop Patterns", "Count"]);
    let mut entries: Vec<_> = analytics.loop_patterns.iter().collect();
    entries.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));

    let top_entries = &entries[..entries.len().min(10)];
    for (pattern, count) in top_entries {
        let pattern = (**pattern)
            .iter()
            .map(|instr| format!("{instr:?}"))
            .collect::<Vec<_>>()
            .join(" ");
        builder.push_record([pattern.to_string(), count.to_string()]);
    }

    build_table(builder)
}

pub fn build_misc_table(analytics: &Analytics) -> String {
    let mut builder = table_builder(&["Metric", "Value"]);
    // Plus 1 since the memory is zero-indexed
    let highest_memory_access = analytics.highest_memory_access + 1;
    builder.push_record(["Highest Memory Access", &format!("{highest_memory_access}")]);
    build_table(builder)
}
