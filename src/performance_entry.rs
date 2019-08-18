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
pub struct PerformanceResourceTiming {
    // base
    initiatorType: String,
    name: String,
    nextHopProtocol: String,
    duration: f64,
    startTime: f64,

    // size
    transferSize: i32,
    encodedBodySize: i32,
    decodedBodySize: i32,

    // timing
    redirectStart: f64,
    redirectEnd: f64,
    fetchStart: f64,
    domainLookupStart: f64,
    domainLookupEnd: f64,
    connectStart: f64,
    connectEnd: f64,
    requestStart: f64,
    responseStart: f64,
    responseEnd: f64,
    secureConnectionStart: f64,
    workerStart: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PerformancePaintTiming {
//    entryType: String,
    name: String,
    startTime: f64,
    duration: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PerformanceNavigationTiming {
    // base
    initiatorType: String,
    name: String,
    nextHopProtocol: String,
    duration: f64,
    startTime: f64,

    // size
    transferSize: i32,
    encodedBodySize: i32,
    decodedBodySize: i32,

    // timing
    redirectStart: f64,
    redirectEnd: f64,
    fetchStart: f64,
    domainLookupStart: f64,
    domainLookupEnd: f64,
    connectStart: f64,
    connectEnd: f64,
    requestStart: f64,
    responseStart: f64,
    responseEnd: f64,
    secureConnectionStart: f64,
    workerStart: f64,

    // dom event
    domInteractive: f64,
    domContentLoadedEventStart: f64,
    domContentLoadedEventEnd: f64,
    domComplete: f64,
    loadEventStart: f64,
    loadEventEnd: f64,
    unloadEventStart: f64,
    unloadEventEnd: f64,
    redirectCount: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PerformanceMark {
    name: String,
    startTime: f64,
    duration: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PerformanceMeasure {
    name: String,
    startTime: f64,
    duration: f64,
}
