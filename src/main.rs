// Copyright 2021 Cognite AS
use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct Metric {
    metric: HashMap<String, String>,
    values: Vec<u64>,
    timestamps: Vec<u64>,
}

fn main() -> Result<()> {
    let mut types: HashSet<String> = HashSet::new();
    for line in stdin().lock().lines() {
        let line = line?;
        let mut metric: Metric =
            serde_json::from_str(&line).with_context(|| format!("{}", line))?;
        let (_, mut metric_name) = metric.metric.remove_entry("__name__").unwrap();
        if !types.contains(&metric_name.clone()) {
            println!("# TYPE {} counter", metric_name);
            types.insert(metric_name.clone());
        }
        metric_name.push_str("{");
        let mut first = true;
        for (k, v) in metric.metric.into_iter() {
            if !first {
                metric_name.push_str(",");
            }
            first = false;
            metric_name.push_str(&format!("{}=\"{}\"", k, v));
        }
        metric_name.push_str("}");
        for (value, timestamp) in metric.values.into_iter().zip(metric.timestamps.into_iter()) {
            println!("{} {} {}", metric_name, value, timestamp / 1000);
        }
    }
    println!("# EOF");
    Ok(())
}
