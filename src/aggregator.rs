use prometheus::{Registry, IntCounter, TextEncoder, Encoder, IntCounterVec, GaugeVec, HistogramVec, Opts, Result, Histogram, HistogramOpts, CounterVec, Counter};
use std::collections::HashMap;
use prometheus::core::Collector;
use std::sync::Arc;

pub const DEFAULT_BUCKETS: [f64; 5] = [100f64, 200f64, 400f64, 800f64, 1600f64];

pub struct Aggregator {
    pub registry: Registry,
    pub int_counters: HashMap<String, IntCounterVec>,
    pub counters: HashMap<String, CounterVec>,
    pub histograms: HashMap<String, HistogramVec>,
}

impl Default for Aggregator {
    fn default() -> Self {
        Aggregator {
            registry: Registry::new(),
            int_counters: HashMap::new(),
            counters: HashMap::new(),
            histograms: HashMap::new(),
        }
    }
}


impl Aggregator {
    pub fn gather_str(&self) -> String {
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }

    pub fn get_int_counter_with_labels(&mut self, metric_name: &str, help: &str, labels: &HashMap<&str, &str>) -> Result<IntCounter> {
        if !self.int_counters.contains_key(metric_name) {
            let re = IntCounterVec::new(Opts::new(metric_name, help), &labels.keys().map(|&k| k).collect::<Vec<&str>>())?;
            self.registry.register(Box::new(re.clone())).unwrap();
            self.int_counters.insert(metric_name.to_string(), re);
        }
        let counter = self.int_counters.get(metric_name).unwrap();
        Ok(counter.with(labels))
    }

    pub fn get_int_counter(&mut self, metric_name: &str, help: &str) -> Result<IntCounter> {
        self.get_int_counter_with_labels(metric_name, help, &HashMap::new())
    }

    pub fn get_counter_with_labels(&mut self, metric_name: &str, help: &str, labels: &HashMap<&str, &str>) -> Result<Counter> {
        if !self.counters.contains_key(metric_name) {
            let re = CounterVec::new(Opts::new(metric_name, help), &labels.keys().map(|&k| k).collect::<Vec<&str>>())?;
            self.registry.register(Box::new(re.clone())).unwrap();
            self.counters.insert(metric_name.to_string(), re);
        }
        let counter = self.counters.get(metric_name).unwrap();
        Ok(counter.with(labels))
    }

    pub fn get_counter(&mut self, metric_name: &str, help: &str) -> Result<Counter> {
        self.get_counter_with_labels(metric_name, help, &HashMap::new())
    }

    pub fn get_histogram_with_buckets_labels(&mut self, metric_name: &str, help: &str, buckets: Vec<f64>, labels: &HashMap<&str, &str>) -> Result<Histogram> {
        if !self.histograms.contains_key(metric_name) {
            let re = HistogramVec::new(HistogramOpts::new(metric_name, help).buckets(buckets), &labels.keys().map(|&k| k).collect::<Vec<&str>>())?;
            self.registry.register(Box::new(re.clone()))?;
            self.histograms.insert(metric_name.to_string(), re);
        }
        let counter = self.histograms.get(metric_name).unwrap();
        Ok(counter.with(labels))
    }

    pub fn get_histogram_with_labels(&mut self, metric_name: &str, help: &str, labels: &HashMap<&str, &str>) -> Result<Histogram> {
        self.get_histogram_with_buckets_labels(metric_name, help, DEFAULT_BUCKETS.to_vec(), labels)
    }

    pub fn get_histogram(&mut self, metric_name: &str, help: &str) -> Result<Histogram> {
        self.get_histogram_with_labels(metric_name, help, &HashMap::new())
    }
}