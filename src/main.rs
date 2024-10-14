use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use dashmap::DashMap;
use tokio::time::sleep;

// Structure to represent cached assets with TTL
#[derive(Clone)]
struct CachedAsset {
    data: Vec<u8>,
    expiry: Instant,
}

// The in-memory cache (thread-safe using DashMap)
struct Cache {
    store: DashMap<String, CachedAsset>,
}

impl Cache {
    fn new() -> Cache {
        Cache {
            store: DashMap::new(),
        }
    }

    fn set(&self, key: String, value: Vec<u8>, ttl: Duration) {
        let expiry = Instant::now() + ttl;
        self.store.insert(key, CachedAsset { data: value, expiry });
    }

    fn get(&self, key: &str) -> Option<Vec<u8>> {
        if let Some(asset) = self.store.get(key) {
            if asset.expiry > Instant::now() {
                return Some(asset.data.clone());
            } else {
                // Asset expired, remove it
                self.store.remove(key);
            }
        }
        None
    }

    fn remove(&self, key: &str) {
        self.store.remove(key);
    }
}

// Data structure to handle asset upload via the API
#[derive(Serialize, Deserialize)]
struct AssetPayload {
    key: String,
    data: String, // Base64-encoded string for simplicity
    ttl_seconds: u64,
}

// RESTful API to cache assets
async fn cache_asset(cache: web::Data<Cache>, payload: web::Json<AssetPayload>) -> impl Responder {
    let data = base64::decode(&payload.data).unwrap();
    let ttl = Duration::new(payload.ttl_seconds, 0);
    
    cache.set(payload.key.clone(), data, ttl);
    
    HttpResponse::Ok().json("Asset cached")
}

// RESTful API to retrieve assets
async fn get_asset(cache: web::Data<Cache>, key: web::Path<String>) -> impl Responder {
    if let Some(data) = cache.get(&key) {
        HttpResponse::Ok().body(data)
    } else {
        HttpResponse::NotFound().json("Asset not found or expired")
    }
}

// RESTful API to remove asset
async fn remove_asset(cache: web::Data<Cache>, key: web::Path<String>) -> impl Responder {
    cache.remove(&key);
    HttpResponse::Ok().json("Asset removed")
}

// Cache cleaning task to remove expired assets
async fn cleanup_cache(cache: web::Data<Cache>) {
    loop {
        sleep(Duration::from_secs(60)).await;
        let now = Instant::now();
        
        cache.store.retain(|_, asset| asset.expiry > now);
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cache = web::Data::new(Cache::new());

    // Spawn a task to periodically clean the cache
    let cache_clone = cache.clone();
    tokio::spawn(async move {
        cleanup_cache(cache_clone).await;
    });

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(cache.clone())
            .route("/cache", web::post().to(cache_asset))
            .route("/cache/{key}", web::get().to(get_asset))
            .route("/cache/{key}", web::delete().to(remove_asset))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
