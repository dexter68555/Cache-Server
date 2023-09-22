use actix_web::{web, App, HttpResponse, HttpServer, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

//AppState Struct
#[derive(Clone)]
struct AppState {
    cache: Arc<Mutex<HashMap<String, String>>>,
}

//Get cache data
async fn getCachedData(
    req: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let key = req.into_inner();

    // Try to get the value from the cache
    let cache = data.cache.lock().unwrap();
    if let Some(value) = cache.get(&key) {
        Ok(HttpResponse::Ok().body(value.clone()))
    } else {
        Ok(HttpResponse::NotFound().body("Key not found in cache"))
    }
}

//Set cache data
async fn setCachedData(
    req: web::Path<(String, String)>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let (key, value) = req.into_inner();

    // Insert the key-value pair into the cache
    let mut cache = data.cache.lock().unwrap();
    cache.insert(key.clone(), value.clone());

    Ok(HttpResponse::Ok().body(format!("Key '{}' is set in cache", key)))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Create a Mutex-protected HashMap for caching
    let cache = Arc::new(Mutex::new(HashMap::new()));

    // Create the Actix web app with state
    let app_state = web::Data::new(AppState {
        cache: cache.clone(),
    });

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/get/{key}", web::get().to(getCachedData))
            .route("/set/{key}/{value}", web::post().to(setCachedData))
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}