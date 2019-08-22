mod performance_entry;
mod aggregator;
mod resource_aggregator;

use actix_web::{Responder, HttpResponse, HttpServer, App, web};
use performance_entry::*;
use aggregator::Aggregator;
use std::sync::{Mutex, Arc};
use crate::resource_aggregator::ResourceAggregator;
use std::collections::HashMap;

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
    agg.lock().unwrap().get_counter("index_page_total", "count for index route").unwrap().inc();
    HttpResponse::Ok().body("This is a frontend metrics aggregator for prometheus")
}

fn resources(entries: web::Json<Vec<PerformanceEntry>>, agg: AggState) -> impl Responder {
    agg.lock().unwrap().receive_entries(&entries);
    HttpResponse::Ok().body(serde_json::to_string(&entries.0).unwrap())
}

fn metrics(agg: AggState) -> String {
    agg.lock().unwrap().gather_str()
}