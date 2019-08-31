use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CounterValue {
    #[serde(default)]
    pub labels: HashMap<String, String>,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Counter {
    pub name: String,
    pub help: String,
    pub metrics: Vec<CounterValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistogramValue {
    #[serde(default)]
    pub labels: HashMap<String, String>,
    pub buckets: HashMap<String, f64>,
    pub count: i32,
    pub sum: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Histogram {
    pub name: String,
    pub help: String,
    pub metrics: Vec<HistogramValue>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Metric {
    COUNTER(Counter),
    HISTOGRAM(Histogram)
}