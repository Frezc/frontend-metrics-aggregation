mod performance_entry;
mod aggregation;

use actix_web::{Responder, HttpResponse, HttpServer, App, web};
use performance_entry::*;
use aggregation::Aggregation;
use std::sync::Mutex;

fn main() {
    HttpServer::new(|| {
        App::new()
            .register_data(web::Data::new(Mutex::new(Aggregation::default())))
            .route("/", web::get().to(index))
            .route("/resources", web::post().to(resources))
            .route("/metrics", web::get().to(metrics))
    })
        .bind("0.0.0.0:8080")
        .unwrap()
        .run()
        .unwrap();
}

fn index(agg: web::Data<Mutex<Aggregation>>) -> impl Responder {
    agg.lock().unwrap().get_counter("index_page_total", "count for index route").unwrap().inc();
    HttpResponse::Ok().body("This is a frontend metrics aggregator for prometheus")
}

fn resources(entries: web::Json<Vec<PerformanceEntry>>) -> impl Responder {
    HttpResponse::Ok().body(serde_json::to_string(&entries.0).unwrap())
}

fn metrics(agg: web::Data<Mutex<Aggregation>>) -> String {
    agg.lock().unwrap().gather_str()
}