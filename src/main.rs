mod performance_entry;
mod aggregator;
mod resource_aggregator;

use actix_web::{Responder, HttpResponse, HttpServer, App, web};
use performance_entry::*;
use aggregator::Aggregator;
use std::sync::Mutex;
use crate::resource_aggregator::ResourceAggregator;

fn main() {
    HttpServer::new(|| {
        App::new()
            .register_data(web::Data::new(Mutex::new(Aggregator::default())))
            .route("/", web::get().to(index))
            .route("/resources", web::post().to(resources))
            .route("/metrics", web::get().to(metrics))
    })
        .bind("0.0.0.0:8080")
        .unwrap()
        .run()
        .unwrap();
}

fn index(agg: web::Data<Mutex<Aggregator>>) -> impl Responder {
    agg.lock().unwrap().get_counter("index_page_total", "count for index route").unwrap().inc();
    HttpResponse::Ok().body("This is a frontend metrics aggregator for prometheus")
}

fn resources(entries: web::Json<Vec<PerformanceEntry>>, agg: web::Data<Mutex<Aggregator>>) -> impl Responder {
    let mut agg = agg.lock().unwrap();
    for entry in entries {
        agg.receive_entries(entry);
    }
    HttpResponse::Ok().body(serde_json::to_string(&entries.0).unwrap())
}

fn metrics(agg: web::Data<Mutex<Aggregator>>) -> String {
    agg.lock().unwrap().gather_str()
}