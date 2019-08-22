use crate::aggregator::Aggregator;
use crate::performance_entry::{PerformanceEntry, PerformanceNavigationTiming, PerformanceResourceTiming, PerformanceMeasure, PerformanceMark, PerformancePaintTiming, EntryBase};
use std::collections::HashMap;
use prometheus::Result;
use prometheus::core::Collector;

pub const RESOURCE_DURATION_METRIC: &'static str = "resource_duration_milliseconds";
pub const RESOURCE_DURATION_HELP: &'static str = "Duration of frontend resources in milliseconds.";

pub trait ResourceAggregator {
    fn receive_entries(&mut self, entries: &[PerformanceEntry]) -> Result<&Self> {
        for entry in entries {
            match entry {
                PerformanceEntry::navigation(entry) => self.receive_navigation(entry)?,
                PerformanceEntry::paint(entry) => self.receive_paint(entry)?,
                PerformanceEntry::resource(entry) => self.receive_resource(entry)?,
                PerformanceEntry::mark(entry) => self.receive_mark(entry)?,
                PerformanceEntry::measure(entry) => self.receive_measure(entry)?,
            };
        }
        Ok(self)
    }

    fn receive_navigation(&mut self, entry: &PerformanceNavigationTiming) -> Result<&Self> {
        Ok(self)
    }
    fn receive_resource(&mut self, entry: &PerformanceResourceTiming) -> Result<&Self> {
        Ok(self)
    }
    fn receive_mark(&mut self, entry: &PerformanceMark) -> Result<&Self> {
        Ok(self)
    }
    fn receive_measure(&mut self, entry: &PerformanceMeasure) -> Result<&Self> {
        Ok(self)
    }
    fn receive_paint(&mut self, entry: &PerformancePaintTiming) -> Result<&Self> {
        Ok(self)
    }
}

fn get_labels_base(entry_base: &EntryBase) -> HashMap<&str, &str> {
    let mut labels: HashMap<&str, &str> = HashMap::new();
    labels.insert("name", &entry_base.name);
    labels
}

impl ResourceAggregator for Aggregator {
    fn receive_resource(&mut self, entry: &PerformanceResourceTiming) -> Result<&Self> {
        let mut labels = get_labels_base(&entry.base);
        labels.insert("initiatorType", &entry.initiatorType);
        labels.insert("nextHopProtocol", &entry.nextHopProtocol);

        let histogram = self.get_histogram_with_labels(RESOURCE_DURATION_METRIC, RESOURCE_DURATION_HELP, &labels)?;
        histogram.observe(entry.base.duration);
        Ok(self)
    }
}
