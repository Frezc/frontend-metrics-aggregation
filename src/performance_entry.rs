#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "entryType")]
pub enum PerformanceEntry {
    resource(PerformanceResourceTiming),
    paint(PerformancePaintTiming),
    navigation(PerformanceNavigationTiming),
    mark(PerformanceMark),
    measure(PerformanceMeasure),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EntryBase {
    pub name: String,
    pub duration: f64,
    pub startTime: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PerformanceResourceTiming {
    #[serde(flatten)]
    pub base: EntryBase,

    pub initiatorType: String,
    pub nextHopProtocol: String,

    // size
    pub transferSize: i32,
    pub encodedBodySize: i32,
    pub decodedBodySize: i32,

    // timing
    pub redirectStart: f64,
    pub redirectEnd: f64,
    pub fetchStart: f64,
    pub domainLookupStart: f64,
    pub domainLookupEnd: f64,
    pub connectStart: f64,
    pub connectEnd: f64,
    pub requestStart: f64,
    pub responseStart: f64,
    pub responseEnd: f64,
    pub secureConnectionStart: f64,
    pub workerStart: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PerformanceNavigationTiming {
    #[serde(flatten)]
    pub resource: PerformanceResourceTiming,

    // dom event
    pub domInteractive: f64,
    pub domContentLoadedEventStart: f64,
    pub domContentLoadedEventEnd: f64,
    pub domComplete: f64,
    pub loadEventStart: f64,
    pub loadEventEnd: f64,
    pub unloadEventStart: f64,
    pub unloadEventEnd: f64,
    pub redirectCount: i32,
}

pub type PerformanceMark = EntryBase;
pub type PerformanceMeasure = EntryBase;
pub type PerformancePaintTiming = EntryBase;
