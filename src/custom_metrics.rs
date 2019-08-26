use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Counter {
    pub labels: HashMap<String, String>,
    pub metric: String,
    pub help: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Histogram {
    pub labels: HashMap<String, String>,
    pub metric: String,
    pub help: String,
    pub value: f64,
    pub buckets: Vec<f64>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Metric {
    Counter(Counter),
    Histogram(Histogram)
}