use prometheus::{Registry, IntCounter, TextEncoder, Encoder, IntCounterVec, HistogramVec, Opts, Result};
use std::collections::HashMap;

pub struct Aggregation {
    registry: Registry,
    // counter & histogram use Atomic* internally, so we need not wrap for multi-threads
    counters: HashMap<String, IntCounterVec>,
    histograms: HashMap<String, HistogramVec>,
}

impl Default for Aggregation {
    fn default() -> Self {
        let counter = IntCounter::new("example_count", "This is a example counter").unwrap();
        Aggregation {
            registry: Registry::new(),
            counters: HashMap::new(),
            histograms: HashMap::new(),
        }
    }
}

impl Aggregation {
    pub fn gather_str(&self) -> String {
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        dbg!(&metric_families);
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }

    pub fn get_counter_with_labels(&mut self, metric_name: &str, help: &str, labels: &HashMap<&str, &str>) -> Result<IntCounter> {
        if !self.counters.contains_key(metric_name) {
            let re = IntCounterVec::new(Opts::new(metric_name, help), &labels.keys().map(|&k| k).collect::<Vec<&str>>())?;
            self.registry.register(Box::new(re.clone())).unwrap();
            self.counters.insert(metric_name.to_string(), re);
        }
        let counter = self.counters.get(metric_name).unwrap();
        Ok(counter.with(labels))
    }

    pub fn get_counter(&mut self, metric_name: &str, help: &str) -> Result<IntCounter> {
        self.get_counter_with_labels(metric_name, help, &HashMap::new())
    }
}