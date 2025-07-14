//! StelLang Registry Server
//! 
//! A simple registry server for StelLang packages with package storage,
//! search, and download functionality.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};
use std::convert::Infallible;
use sha2::Digest;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PackageMetadata {
    name: String,
    version: String,
    description: Option<String>,
    authors: Option<Vec<String>>,
    dependencies: Option<HashMap<String, String>>,
    checksum: String,
    size: u64,
    upload_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchResponse {
    packages: Vec<PackageMetadata>,
    total: usize,
}

struct RegistryState {
    packages: RwLock<HashMap<String, HashMap<String, PackageMetadata>>>,
    storage_path: PathBuf,
}

impl RegistryState {
    fn new(storage_path: PathBuf) -> Self {
        Self {
            packages: RwLock::new(HashMap::new()),
            storage_path,
        }
    }

    async fn load_packages(&self) -> Result<(), Box<dyn std::error::Error>> {
        let packages_file = self.storage_path.join("packages.json");
        if packages_file.exists() {
            let content = fs::read_to_string(packages_file)?;
            let packages: HashMap<String, HashMap<String, PackageMetadata>> = serde_json::from_str(&content)?;
            *self.packages.write().await = packages;
        }
        Ok(())
    }

    async fn save_packages(&self) -> Result<(), Box<dyn std::error::Error>> {
        let packages = self.packages.read().await;
        let content = serde_json::to_string_pretty(&*packages)?;
        fs::write(self.storage_path.join("packages.json"), content)?;
        Ok(())
    }

    async fn add_package(&self, metadata: PackageMetadata, package_data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        // Save package file
        let package_file = self.storage_path.join("packages").join(format!("{}-{}.tar.gz", metadata.name, metadata.version));
        fs::create_dir_all(package_file.parent().unwrap())?;
        fs::write(&package_file, package_data)?;

        // Update metadata
        let mut packages = self.packages.write().await;
        packages.entry(metadata.name.clone()).or_insert_with(HashMap::new).insert(metadata.version.clone(), metadata);
        drop(packages);

        self.save_packages().await?;
        Ok(())
    }

    async fn get_package(&self, name: &str, version: &str) -> Option<PackageMetadata> {
        let packages = self.packages.read().await;
        packages.get(name)?.get(version).cloned()
    }

    async fn search_packages(&self, query: &str) -> Vec<PackageMetadata> {
        let packages = self.packages.read().await;
        let mut results = Vec::new();
        
        for (name, versions) in packages.iter() {
            if name.to_lowercase().contains(&query.to_lowercase()) {
                for metadata in versions.values() {
                    results.push(metadata.clone());
                }
            }
        }
        
        results
    }

    async fn get_package_file(&self, name: &str, version: &str) -> Option<Vec<u8>> {
        let package_file = self.storage_path.join("packages").join(format!("{}-{}.tar.gz", name, version));
        fs::read(package_file).ok()
    }
}

#[tokio::main]
async fn main() {
    let storage_path = PathBuf::from("registry_storage");
    fs::create_dir_all(&storage_path).unwrap();

    let state = Arc::new(RegistryState::new(storage_path.clone()));
    state.load_packages().await.unwrap();

    println!("StelLang Registry Server starting on http://localhost:8080");
    println!("Storage path: {}", storage_path.display());

    // Routes
    let search_route = warp::path!("api" / "search")
        .and(warp::query::<HashMap<String, String>>())
        .and(with_state(state.clone()))
        .and_then(search_packages);

    let package_info_route = warp::path!("api" / "packages" / String / String)
        .and(with_state(state.clone()))
        .and_then(get_package_info);

    let package_download_route = warp::path!("api" / "packages" / String / String / "download")
        .and(with_state(state.clone()))
        .and_then(download_package);

    let publish_route = warp::path!("api" / "packages")
        .and(warp::post())
        .and(warp::header::<String>("authorization"))
        .and(warp::body::bytes())
        .and(with_state(state.clone()))
        .and_then(publish_package);

    let routes = search_route
        .or(package_info_route)
        .or(package_download_route)
        .or(publish_route)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}

fn with_state(state: Arc<RegistryState>) -> impl Filter<Extract = (Arc<RegistryState>,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

async fn search_packages(
    query: HashMap<String, String>,
    state: Arc<RegistryState>,
) -> Result<impl Reply, Rejection> {
    let empty = String::new();
    let search_query = query.get("q").unwrap_or(&empty);
    let packages = state.search_packages(search_query).await;
    let total = packages.len();
    let response = SearchResponse {
        packages,
        total,
    };
    Ok(warp::reply::json(&response))
}

async fn get_package_info(
    name: String,
    version: String,
    state: Arc<RegistryState>,
) -> Result<impl Reply, Rejection> {
    match state.get_package(&name, &version).await {
        Some(metadata) => Ok(warp::reply::json(&metadata)),
        None => Err(warp::reject::not_found()),
    }
}

async fn download_package(
    name: String,
    version: String,
    state: Arc<RegistryState>,
) -> Result<impl Reply, Rejection> {
    match state.get_package_file(&name, &version).await {
        Some(data) => Ok(warp::reply::with_header(data, "Content-Type", "application/gzip")),
        None => Err(warp::reject::not_found()),
    }
}

async fn publish_package(
    auth_header: String,
    package_data: bytes::Bytes,
    state: Arc<RegistryState>,
) -> Result<impl Reply, Rejection> {
    // Simple token validation (in production, use proper JWT)
    if !auth_header.starts_with("Bearer ") {
        return Err(warp::reject::custom(AuthError));
    }
    
    let token = &auth_header[7..];
    if token != "test-token" {
        return Err(warp::reject::custom(AuthError));
    }

    // Extract package metadata from the archive
    let package_data = package_data.to_vec();
    let cursor = std::io::Cursor::new(&package_data);
    let gz = flate2::read::GzDecoder::new(cursor);
    let mut tar = tar::Archive::new(gz);
    
    let mut manifest_content = Vec::new();
    for entry in tar.entries().unwrap() {
        let mut entry = entry.unwrap();
        if entry.path().unwrap().to_str().unwrap() == "stel.toml" {
            std::io::copy(&mut entry, &mut manifest_content).unwrap();
            break;
        }
    }
    let manifest_str = String::from_utf8(manifest_content).unwrap();
    let manifest: serde_json::Value = toml::from_str(&manifest_str).unwrap();
    let package_info = &manifest["package"];
    
    let metadata = PackageMetadata {
        name: package_info["name"].as_str().unwrap().to_string(),
        version: package_info["version"].as_str().unwrap().to_string(),
        description: package_info["description"].as_str().map(|s| s.to_string()),
        authors: package_info["authors"].as_array().map(|arr| {
            arr.iter().map(|v| v.as_str().unwrap().to_string()).collect()
        }),
        dependencies: None, // TODO: Extract dependencies
        checksum: format!("sha256:{}", hex::encode(sha2::Sha256::digest(&package_data))),
        size: package_data.len() as u64,
        upload_date: chrono::Utc::now().to_rfc3339(),
    };
    
    state.add_package(metadata.clone(), package_data).await.unwrap();
    
    Ok(warp::reply::json(&serde_json::json!({
        "success": true,
        "package": metadata
    })))
}

#[derive(Debug)]
struct AuthError;

impl warp::reject::Reject for AuthError {}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Authentication failed")
    }
} 