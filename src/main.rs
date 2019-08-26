mod performance_entry;
mod aggregator;
mod resource_aggregator;
mod custom_metrics;

use actix_web::{Responder, HttpResponse, HttpServer, App, web};
use performance_entry::*;
use aggregator::Aggregator;
use std::sync::{Mutex, Arc};
use crate::resource_aggregator::ResourceAggregator;
use std::collections::HashMap;
use crate::custom_metrics::Metric;
use std::ops::Deref;

type AggState = web::Data<Arc<Mutex<Aggregator>>>;

fn main() {
    let data = Arc::new(Mutex::new(Aggregator::default()));
    HttpServer::new(move || {
        App::new()
            .register_data(web::Data::new(Arc::clone(&data)))
            .route("/", web::get().to(index))
            .route("/resources", web::post().to(resources))
            .route("/metrics", web::get().to(metrics))
    })
        .bind("0.0.0.0:8080")
        .unwrap()
        .run()
        .unwrap();
}

fn index(agg: AggState) -> impl Responder {
    agg.lock().unwrap().get_int_counter("index_page_total", "count for index route").unwrap().inc();
    HttpResponse::Ok().body("This is a frontend metrics aggregator for prometheus")
}

fn resources(entries: web::Json<Vec<PerformanceEntry>>, agg: AggState) -> impl Responder {
    agg.lock().unwrap().receive_entries(&entries);
    HttpResponse::Ok()
}

fn metrics(agg: AggState) -> String {
    agg.lock().unwrap().gather_str()
}

trait AsStr {
    fn as_str(&self) -> HashMap<&str, &str>;
}

impl AsStr for HashMap<String, String> {
    fn as_str(&self) -> HashMap<&str, &str> {
        let mut result: HashMap<&str, &str> = HashMap::new();
        for (key, value) in self {
            result.insert(key, value);
        }
        result
    }
}

fn custom_metrics(metrics: web::Json<Vec<Metric>>, agg: AggState) -> impl Responder {
    for metric in metrics.into_inner() {
        match metric {
            Metric::Counter(m) => agg.lock().unwrap().get_counter_with_labels(&m.metric, &m.help, &m.labels.as_str()).unwrap().inc_by(m.value),
            Metric::Histogram(m) => agg.lock().unwrap().get_histogram_with_buckets_labels(&m.metric, &m.help, m.buckets, &m.labels.as_str()).unwrap().observe(m.value)
        }
    }
    HttpResponse::Ok()
}