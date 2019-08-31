mod performance_entry;
mod aggregator;
mod resource_aggregator;
mod custom_metrics;

use actix_web::{Responder, HttpResponse, HttpServer, App, web, HttpRequest, error, Error};
use performance_entry::*;
use aggregator::Aggregator;
use std::sync::{Mutex, Arc};
use crate::resource_aggregator::ResourceAggregator;
use std::collections::HashMap;
use crate::custom_metrics::Metric;
use std::ops::Deref;
use futures::{Future, Stream};
use actix_web::web::BytesMut;

type AggState = web::Data<Arc<Mutex<Aggregator>>>;

fn main() {
    let data = Arc::new(Mutex::new(Aggregator::default()));
    HttpServer::new(move || {
        App::new()
            .register_data(web::Data::new(Arc::clone(&data)))
            .route("/", web::get().to(index))
            .route("/resources", web::post().to_async(resources))
            .route("/custom_metrics", web::post().to_async(custom_metrics))
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

fn resources(payload: web::Payload, agg: AggState) -> impl Future<Item = HttpResponse, Error = Error> {
    read_payload(payload)
        .and_then(move |body| {
            let entries = serde_json::from_slice::<Vec<PerformanceEntry>>(&body)?;
            agg.lock().unwrap().receive_entries(&entries);
            Ok(HttpResponse::Ok().finish())
        })
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

const MAX_SIZE: usize = 262_144; // max payload size is 256k

fn custom_metrics(payload: web::Payload, agg: AggState) -> impl Future<Item = HttpResponse, Error = Error> {
    read_payload(payload)
        .and_then(move |body| {
            let metrics = serde_json::from_slice::<Vec<Metric>>(&body)?;
            for metric in metrics {
                match metric {
                    Metric::COUNTER(m) => {
                        for mv in m.metrics {
                            agg.lock()
                                .unwrap()
                                .get_counter_with_labels(
                                    &m.name,
                                    &m.help,
                                    &mv.labels.as_str()
                                )
                                .unwrap()
                                .inc_by(mv.value)
                        }
                    },
                    Metric::HISTOGRAM(m) => {
                        unimplemented!()
                    }
                }
            }
            Ok(HttpResponse::Ok().finish())
        })
}

fn read_payload(payload: web::Payload) -> impl Future<Item=BytesMut, Error = Error> {
    payload
        .from_err()
        .fold(BytesMut::new(), move |mut body, chunk| {
            // limit max size of in-memory payload
            if (body.len() + chunk.len()) > MAX_SIZE {
                Err(error::ErrorBadRequest("overflow"))
            } else {
                body.extend_from_slice(&chunk);
                Ok(body)
            }
        })
}