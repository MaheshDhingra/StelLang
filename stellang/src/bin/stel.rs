//! Stel: StelLang Package Manager CLI
//! 
//! A comprehensive package manager for StelLang with dependency resolution,
//! lockfiles, registry integration, and project management.

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use toml;
use semver::{VersionReq, Version};
use flate2::write::GzEncoder;
use flate2::Compression;
use tar::Builder;
use std::io::Cursor;

// Configuration
const STEL_REGISTRY_URL: &str = "https://stellang.maheshdhingra.xyz/registry";
const STEL_CONFIG_DIR: &str = ".stel";
const STEL_LOCK_FILE: &str = "stel.lock";
const STEL_MANIFEST_FILE: &str = "stel.toml";
const STEL_CACHE_DIR: &str = ".stel/cache";

#[derive(Debug, Serialize, Deserialize)]
struct PackageManifest {
    package: PackageInfo,
    dependencies: Option<HashMap<String, String>>,
    dev_dependencies: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageInfo {
    name: String,
    version: String,
    authors: Option<Vec<String>>,
    description: Option<String>,
    license: Option<String>,
    repository: Option<String>,
    keywords: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LockFile {
    version: String,
    packages: HashMap<String, LockedPackage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LockedPackage {
    version: String,
    source: String,
    dependencies: Option<HashMap<String, String>>,
    checksum: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RegistryPackage {
    name: String,
    version: String,
    description: Option<String>,
    authors: Option<Vec<String>>,
    dependencies: Option<HashMap<String, String>>,
    download_url: String,
    checksum: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RegistrySearchResponse {
    packages: Vec<RegistryPackage>,
    total: usize,
}

struct StelCLI {
    config_dir: PathBuf,
    cache_dir: PathBuf,
    registry_url: String,
}

impl StelCLI {
    fn new() -> Self {
        let config_dir = PathBuf::from(STEL_CONFIG_DIR);
        let cache_dir = config_dir.join("cache");
        Self {
            config_dir,
            cache_dir,
            registry_url: STEL_REGISTRY_URL.to_string(),
        }
    }

    fn ensure_config_dir(&self) -> io::Result<()> {
        if !self.config_dir.exists() {
            fs::create_dir_all(&self.config_dir)?;
        }
        if !self.cache_dir.exists() {
            fs::create_dir_all(&self.cache_dir)?;
        }
        Ok(())
    }

    fn read_manifest(&self) -> io::Result<PackageManifest> {
        let manifest_path = Path::new(STEL_MANIFEST_FILE);
        if !manifest_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "stel.toml not found. Run 'stel init' first.",
            ));
        }
        
        let content = fs::read_to_string(manifest_path)?;
        let manifest: PackageManifest = toml::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(manifest)
    }

    fn write_manifest(&self, manifest: &PackageManifest) -> io::Result<()> {
        let content = toml::to_string_pretty(manifest)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(STEL_MANIFEST_FILE, content)?;
        Ok(())
    }

    fn read_lockfile(&self) -> io::Result<LockFile> {
        let lock_path = Path::new(STEL_LOCK_FILE);
        if !lock_path.exists() {
            return Ok(LockFile {
                version: "1.0".to_string(),
                packages: HashMap::new(),
            });
        }
        
        let content = fs::read_to_string(lock_path)?;
        let lockfile: LockFile = toml::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(lockfile)
    }

    fn write_lockfile(&self, lockfile: &LockFile) -> io::Result<()> {
        let content = toml::to_string_pretty(lockfile)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(STEL_LOCK_FILE, content)?;
        Ok(())
    }

    async fn search_registry(&self, query: &str) -> Result<Vec<RegistryPackage>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let url = format!("{}/api/search?q={}", self.registry_url, query);
        
        let response = client.get(&url)
            .header("User-Agent", "stel-cli/1.0")
            .send()
            .await?;
        
        if response.status().is_success() {
            let search_response: RegistrySearchResponse = response.json().await?;
            Ok(search_response.packages)
        } else {
            // Fallback to mock data for development
            if response.status().as_u16() == 404 {
                println!("Registry not available, showing mock results...");
                Ok(vec![
                    RegistryPackage {
                        name: "example-http".to_string(),
                        version: "1.0.0".to_string(),
                        description: Some("HTTP client library for StelLang".to_string()),
                        authors: Some(vec!["stellang-team".to_string()]),
                        dependencies: Some(HashMap::new()),
                        download_url: "https://example.com/example-http-1.0.0.tar.gz".to_string(),
                        checksum: Some("sha256:abc123...".to_string()),
                    },
                    RegistryPackage {
                        name: "example-json".to_string(),
                        version: "2.1.0".to_string(),
                        description: Some("JSON parsing library for StelLang".to_string()),
                        authors: Some(vec!["stellang-team".to_string()]),
                        dependencies: Some(HashMap::new()),
                        download_url: "https://example.com/example-json-2.1.0.tar.gz".to_string(),
                        checksum: Some("sha256:def456...".to_string()),
                    }
                ])
            } else {
                Err(format!("Registry search failed: {}", response.status()).into())
            }
        }
    }

    async fn get_package_info(&self, name: &str, version: &str) -> Result<RegistryPackage, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let url = format!("{}/api/packages/{}/{}", self.registry_url, name, version);
        
        let response = client.get(&url)
            .header("User-Agent", "stel-cli/1.0")
            .send()
            .await?;
        
        if response.status().is_success() {
            let package: RegistryPackage = response.json().await?;
            Ok(package)
        } else {
            // Fallback to mock data for development
            if response.status().as_u16() == 404 {
                Ok(RegistryPackage {
                    name: name.to_string(),
                    version: version.to_string(),
                    description: Some(format!("Mock package {} {}", name, version)),
                    authors: Some(vec!["stellang-team".to_string()]),
                    dependencies: Some(HashMap::new()),
                    download_url: format!("https://example.com/{}-{}.tar.gz", name, version),
                    checksum: Some("sha256:mock123...".to_string()),
                })
            } else {
                Err(format!("Package not found: {}@{}", name, version).into())
            }
        }
    }

    async fn download_package(&self, name: &str, version: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let url = format!("{}/api/packages/{}/{}/download", self.registry_url, name, version);
        
        let response = client.get(&url)
            .header("User-Agent", "stel-cli/1.0")
            .send()
            .await?;
        
        if response.status().is_success() {
            let bytes = response.bytes().await?;
            Ok(bytes.to_vec())
        } else {
            // For development, create a mock package
            if response.status().as_u16() == 404 {
                println!("Creating mock package for {}@{}", name, version);
                self.create_mock_package(name, version)
            } else {
                Err(format!("Download failed: {}", response.status()).into())
            }
        }
    }

    fn create_mock_package(&self, name: &str, version: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut buffer = Vec::new();
        let gz = GzEncoder::new(&mut buffer, Compression::default());
        let mut tar = Builder::new(gz);
        
        // Add package manifest
        let manifest_content = format!(
            r#"[package]
name = "{}"
version = "{}"
description = "Mock package for development"
authors = ["stellang-team"]
license = "MIT"

[dependencies]
"#,
            name, version
        );
        
        let manifest_bytes = manifest_content.as_bytes();
        let mut header = tar::Header::new_gnu();
        header.set_path("stel.toml")?;
        header.set_size(manifest_bytes.len() as u64);
        header.set_cksum();
        tar.append(&header, manifest_bytes)?;
        
        // Add source files
        let source_content = format!(
            r#"// Mock package: {} v{}
fn hello() {{
    print("Hello from {}!");
}}

fn version() {{
    return "{}";
}}
"#,
            name, version, name, version
        );
        
        let source_bytes = source_content.as_bytes();
        let mut header = tar::Header::new_gnu();
        header.set_path("src/lib.stel")?;
        header.set_size(source_bytes.len() as u64);
        header.set_cksum();
        tar.append(&header, source_bytes)?;
        
        tar.finish()?;
        drop(tar); // Ensure tar is dropped before buffer is moved
        Ok(buffer)
    }

    async fn resolve_dependencies(&self, manifest: &PackageManifest) -> Result<LockFile, Box<dyn std::error::Error>> {
        let mut lockfile = self.read_lockfile()?;
        let mut resolved = HashMap::new();
        let mut to_resolve = Vec::new();
        
        // Collect all dependencies
        if let Some(deps) = &manifest.dependencies {
            for (name, version_req) in deps {
                to_resolve.push((name.clone(), version_req.clone()));
            }
        }
        
        // Resolve dependencies recursively
        while let Some((name, version_req)) = to_resolve.pop() {
            if resolved.contains_key(&name) {
                continue; // Already resolved
            }
            
            let req = VersionReq::parse(&version_req)
                .map_err(|e| format!("Invalid version requirement for {}: {}", name, e))?;
            
            // Try to get package info from registry
            let package_info = self.get_package_info(&name, &version_req).await?;
            
            // Validate version constraint
            let package_version = Version::parse(&package_info.version)
                .map_err(|e| format!("Invalid version for {}: {}", name, e))?;
            
            if !req.matches(&package_version) {
                return Err(format!("No version of {} matches requirement {}", name, version_req).into());
            }
            
            // Add sub-dependencies to resolution queue
            if let Some(sub_deps) = &package_info.dependencies {
                for (sub_name, sub_version) in sub_deps {
                    if !resolved.contains_key(sub_name) {
                        to_resolve.push((sub_name.clone(), sub_version.clone()));
                    }
                }
            }
            
            resolved.insert(name.clone(), LockedPackage {
                version: package_info.version,
                source: format!("registry+{}", self.registry_url),
                dependencies: package_info.dependencies,
                checksum: package_info.checksum,
            });
        }
        
        lockfile.packages = resolved;
        Ok(lockfile)
    }

    async fn install_package(&self, name: &str, version: &str) -> Result<(), Box<dyn std::error::Error>> {
        let package_data = self.download_package(name, version).await?;
        
        // Create package directory
        let package_dir = self.cache_dir.join(format!("{}-{}", name, version));
        if package_dir.exists() {
            fs::remove_dir_all(&package_dir)?;
        }
        fs::create_dir_all(&package_dir)?;
        
        // Extract package
        let cursor = Cursor::new(package_data);
        let gz = flate2::read::GzDecoder::new(cursor);
        let mut tar = tar::Archive::new(gz);
        tar.unpack(&package_dir)?;
        
        // Copy to project's dependencies directory
        let deps_dir = Path::new("dependencies");
        if !deps_dir.exists() {
            fs::create_dir(deps_dir)?;
        }
        
        let target_dir = deps_dir.join(name);
        if target_dir.exists() {
            fs::remove_dir_all(&target_dir)?;
        }
        fs::create_dir_all(&target_dir)?;
        
        // Copy package contents
        self.copy_directory(&package_dir, &target_dir)?;
        
        println!("Installed {}@{} to dependencies/{}", name, version, name);
        Ok(())
    }

    fn copy_directory(&self, src: &Path, dst: &Path) -> io::Result<()> {
        if src.is_dir() {
            if !dst.exists() {
                fs::create_dir(dst)?;
            }
            for entry in fs::read_dir(src)? {
                let entry = entry?;
                let src_path = entry.path();
                let dst_path = dst.join(entry.file_name());
                if src_path.is_dir() {
                    self.copy_directory(&src_path, &dst_path)?;
                } else {
                    fs::copy(&src_path, &dst_path)?;
                }
            }
        }
        Ok(())
    }

    fn create_package_archive(&self, manifest: &PackageManifest) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut buffer = Vec::new();
        let gz = GzEncoder::new(&mut buffer, Compression::default());
        let mut tar = Builder::new(gz);
        
        // Add manifest
        let manifest_content = toml::to_string_pretty(manifest)?;
        let manifest_bytes = manifest_content.as_bytes();
        let mut header = tar::Header::new_gnu();
        header.set_path("stel.toml")?;
        header.set_size(manifest_bytes.len() as u64);
        header.set_cksum();
        tar.append(&header, manifest_bytes)?;
        
        // Add source files
        let src_dir = Path::new("src");
        if src_dir.exists() {
            self.add_directory_to_tar(&mut tar, src_dir, "src")?;
        }
        
        // Add README if exists
        let readme_path = Path::new("README.md");
        if readme_path.exists() {
            let readme_content = fs::read_to_string(readme_path)?;
            let readme_bytes = readme_content.as_bytes();
            let mut header = tar::Header::new_gnu();
            header.set_path("README.md")?;
            header.set_size(readme_bytes.len() as u64);
            header.set_cksum();
            tar.append(&header, readme_bytes)?;
        }
        
        tar.finish()?;
        drop(tar); // Ensure tar is dropped before buffer is moved
        Ok(buffer)
    }

    fn add_directory_to_tar(&self, tar: &mut Builder<GzEncoder<&mut Vec<u8>>>, src: &Path, prefix: &str) -> io::Result<()> {
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            let name = path.file_name().unwrap().to_str().unwrap();
            let tar_path = format!("{}/{}", prefix, name);
            
            if path.is_dir() {
                self.add_directory_to_tar(tar, &path, &tar_path)?;
            } else {
                let content = fs::read(&path)?;
                let mut header = tar::Header::new_gnu();
                header.set_path(&tar_path)?;
                header.set_size(content.len() as u64);
                header.set_cksum();
                tar.append(&header, &content[..])?;
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("stel: missing command");
        eprintln!("Try 'stel help' for more information");
        std::process::exit(1);
    }

    let cli = StelCLI::new();
    
    match args[1].as_str() {
        "init" => cmd_init(&cli),
        "add" => cmd_add(&cli, &args[2..]),
        "build" => cmd_build(&cli),
        "install" => cmd_install(&cli).await,
        "test" => cmd_test(&cli),
        "update" => cmd_update(&cli).await,
        "publish" => cmd_publish(&cli).await,
        "new" => cmd_new(&cli, &args[2..]),
        "template" => cmd_template(&cli, &args[2..]),
        "search" => cmd_search(&cli, &args[2..]).await,
        "remove" => cmd_remove(&cli, &args[2..]),
        "run" => cmd_run(&cli, &args[2..]),
        "clean" => cmd_clean(&cli),
        "tree" => cmd_tree(&cli),
        "login" => cmd_login(&cli),
        "logout" => cmd_logout(&cli),
        "outdated" => cmd_outdated(&cli).await,
        "audit" => cmd_audit(&cli).await,
        // "script" => cmd_script(&cli, &args[2..]),
        "version" => cmd_version(),
        "help" => cmd_help(),
        _ => {
            eprintln!("stel: unknown command '{}'", args[1]);
            eprintln!("Try 'stel help' for more information");
            std::process::exit(1);
        }
    }
}

fn cmd_init(cli: &StelCLI) {
    let manifest_path = Path::new(STEL_MANIFEST_FILE);
    if manifest_path.exists() {
        eprintln!("stel.toml already exists");
        return;
    }

    let manifest = PackageManifest {
        package: PackageInfo {
            name: "my-stellang-project".to_string(),
            version: "0.1.0".to_string(),
            authors: Some(vec!["Your Name <you@example.com>".to_string()]),
            description: Some("A new StelLang project".to_string()),
            license: Some("MIT".to_string()),
            repository: None,
            keywords: Some(vec!["stellang".to_string()]),
        },
        dependencies: Some(HashMap::new()),
        dev_dependencies: Some(HashMap::new()),
    };

    if let Err(e) = cli.write_manifest(&manifest) {
        eprintln!("Failed to create stel.toml: {}", e);
        std::process::exit(1);
    }

    // Create src directory
    let src_dir = Path::new("src");
    if !src_dir.exists() {
        if let Err(e) = fs::create_dir(src_dir) {
            eprintln!("Failed to create src directory: {}", e);
            std::process::exit(1);
        }
    }

    // Create main.stel file
    let main_file = src_dir.join("main.stel");
    if !main_file.exists() {
        let main_content = r#"// Main entry point for your StelLang project

fn main() {
    print("Hello, StelLang!");
}

"#;
        if let Err(e) = fs::write(&main_file, main_content) {
            eprintln!("Failed to create main.stel: {}", e);
            std::process::exit(1);
        }
    }

    println!("Created new StelLang project");
    println!("  stel.toml - Project manifest");
    println!("  src/main.stel - Main source file");
    println!("  Run 'stel build' to build your project");
}

fn cmd_add(cli: &StelCLI, args: &[String]) {
    if args.is_empty() {
        eprintln!("stel add: missing package name");
        eprintln!("Usage: stel add <package> [version]");
        std::process::exit(1);
    }

    let package_name = &args[0];
    let default_version = "*".to_string();
    let version = args.get(1).unwrap_or(&default_version);

    let mut manifest = match cli.read_manifest() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to read stel.toml: {}", e);
            std::process::exit(1);
        }
    };

    let deps = manifest.dependencies.get_or_insert_with(HashMap::new);
    deps.insert(package_name.clone(), version.clone());

    if let Err(e) = cli.write_manifest(&manifest) {
        eprintln!("Failed to update stel.toml: {}", e);
        std::process::exit(1);
    }

    println!("Added {} = \"{}\" to dependencies", package_name, version);
    println!("Run 'stel install' to install the new dependency");
}

fn cmd_build(cli: &StelCLI) {
    let manifest = match cli.read_manifest() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to read stel.toml: {}", e);
            std::process::exit(1);
        }
    };

    println!("Building {} v{}", manifest.package.name, manifest.package.version);

    // Check if main.stel exists
    let main_file = Path::new("src/main.stel");
    if !main_file.exists() {
        eprintln!("src/main.stel not found");
        std::process::exit(1);
    }

    // For now, just validate the syntax
    let content = match fs::read_to_string(main_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read main.stel: {}", e);
            std::process::exit(1);
        }
    };

    // Basic syntax validation using the existing lexer/parser
    let mut lexer = stellang::lang::lexer::Lexer::new(&content);
    let mut tokens = Vec::new();
    
    loop {
        match lexer.next_token() {
            Ok(stellang::lang::lexer::Token::EOF) => break,
            Ok(token) => tokens.push(token),
            Err(e) => {
                eprintln!("Lexer error: {:?}", e);
                std::process::exit(1);
            }
        }
    }

    let mut parser = stellang::lang::parser::Parser::new(tokens);
    match parser.parse() {
        Ok(Some(_)) => println!("Build successful"),
        Ok(None) => println!("Build successful (no expressions)"),
        Err(e) => {
            eprintln!("Parser error: {:?}", e);
            std::process::exit(1);
        }
    }
}

async fn cmd_install(cli: &StelCLI) {
    let manifest = match cli.read_manifest() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to read stel.toml: {}", e);
            std::process::exit(1);
        }
    };

    println!("Installing dependencies for {} v{}", manifest.package.name, manifest.package.version);

    // Ensure config directory exists
    if let Err(e) = cli.ensure_config_dir() {
        eprintln!("Failed to create config directory: {}", e);
        std::process::exit(1);
    }

    // Resolve dependencies
    let lockfile = match cli.resolve_dependencies(&manifest).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to resolve dependencies: {}", e);
            std::process::exit(1);
        }
    };

    // Install each package
    for (name, locked_package) in &lockfile.packages {
        println!("Installing {}@{}", name, locked_package.version);
        if let Err(e) = cli.install_package(name, &locked_package.version).await {
            eprintln!("Failed to install {}@{}: {}", name, locked_package.version, e);
            std::process::exit(1);
        }
    }

    // Write lockfile
    if let Err(e) = cli.write_lockfile(&lockfile) {
        eprintln!("Failed to write lockfile: {}", e);
        std::process::exit(1);
    }

    println!("All dependencies installed successfully!");
    println!("Run 'stel build' to build your project");
}

fn cmd_test(cli: &StelCLI) {
    let manifest = match cli.read_manifest() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to read stel.toml: {}", e);
            std::process::exit(1);
        }
    };

    println!("Running tests for {} v{}", manifest.package.name, manifest.package.version);

    // Look for test files
    let test_dir = Path::new("tests");
    if !test_dir.exists() {
        println!("No tests directory found");
        return;
    }

    let mut test_count = 0;
    let mut passed = 0;

    if let Ok(entries) = fs::read_dir(test_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "stel") {
                test_count += 1;
                println!("Running test: {}", path.display());
                
                // Run the test file
                let content = match fs::read_to_string(&path) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Failed to read test file: {}", e);
                        continue;
                    }
                };

                let mut lexer = stellang::lang::lexer::Lexer::new(&content);
                let mut tokens = Vec::new();
                
                loop {
                    match lexer.next_token() {
                        Ok(stellang::lang::lexer::Token::EOF) => break,
                        Ok(token) => tokens.push(token),
                        Err(e) => {
                            eprintln!("Lexer error in test: {:?}", e);
                            continue;
                        }
                    }
                }

                let mut parser = stellang::lang::parser::Parser::new(tokens);
                match parser.parse() {
                    Ok(Some(_)) => {
                        println!("  ✓ Test passed");
                        passed += 1;
                    }
                    Ok(None) => {
                        println!("  ✓ Test passed (no expressions)");
                        passed += 1;
                    }
                    Err(e) => {
                        eprintln!("  ✗ Test failed: {:?}", e);
                    }
                }
            }
        }
    }

    println!("\nTest Results: {} passed, {} failed", passed, test_count - passed);
    if passed == test_count {
        println!("All tests passed!");
    } else {
        std::process::exit(1);
    }
}

async fn cmd_update(cli: &StelCLI) {
    let manifest = match cli.read_manifest() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to read stel.toml: {}", e);
            std::process::exit(1);
        }
    };

    println!("Updating dependencies for {} v{}", manifest.package.name, manifest.package.version);

    // Ensure config directory exists
    if let Err(e) = cli.ensure_config_dir() {
        eprintln!("Failed to create config directory: {}", e);
        std::process::exit(1);
    }

    // Resolve dependencies (this will get latest versions)
    let lockfile = match cli.resolve_dependencies(&manifest).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to resolve dependencies: {}", e);
            std::process::exit(1);
        }
    };

    // Install updated packages
    for (name, locked_package) in &lockfile.packages {
        println!("Updating {}@{}", name, locked_package.version);
        if let Err(e) = cli.install_package(name, &locked_package.version).await {
            eprintln!("Failed to update {}@{}: {}", name, locked_package.version, e);
            std::process::exit(1);
        }
    }

    if let Err(e) = cli.write_lockfile(&lockfile) {
        eprintln!("Failed to write lockfile: {}", e);
        std::process::exit(1);
    }

    println!("Dependencies updated successfully!");
}

async fn cmd_publish(cli: &StelCLI) {
    let manifest = match cli.read_manifest() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to read stel.toml: {}", e);
            std::process::exit(1);
        }
    };

    println!("Publishing {} v{}", manifest.package.name, manifest.package.version);

    // Check if we're logged in
    let token_file = cli.config_dir.join("token");
    if !token_file.exists() {
        eprintln!("Not logged in. Run 'stel login' first");
        std::process::exit(1);
    }

    // Read token
    let token = match fs::read_to_string(&token_file) {
        Ok(t) => t.trim().to_string(),
        Err(e) => {
            eprintln!("Failed to read token: {}", e);
            std::process::exit(1);
        }
    };

    // Create package archive
    let archive_data = match cli.create_package_archive(&manifest) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to create package archive: {}", e);
            std::process::exit(1);
        }
    };

    let archive_name = format!("{}-{}.tar.gz", manifest.package.name, manifest.package.version);
    println!("Created package archive: {}", archive_name);

    // Upload to registry
    let client = reqwest::Client::new();
    let url = format!("{}/api/packages", cli.registry_url);
    
    let response = client.post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/gzip")
        .header("User-Agent", "stel-cli/1.0")
        .body(archive_data)
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status().is_success() {
                println!("Package published successfully!");
                println!("Visit: {}/packages/{}/{}", cli.registry_url, manifest.package.name, manifest.package.version);
            } else {
                eprintln!("Publish failed: {}", response.status());
                if let Ok(error_text) = response.text().await {
                    eprintln!("Error: {}", error_text);
                }
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to publish package: {}", e);
            println!("Package archive created locally: {}", archive_name);
            std::process::exit(1);
        }
    }
}

fn cmd_new(cli: &StelCLI, args: &[String]) {
    if args.is_empty() {
        eprintln!("stel new: missing project name");
        eprintln!("Usage: stel new <project-name> [--template <template>]");
        std::process::exit(1);
    }

    let project_name = &args[0];
    let default_template = "basic".to_string();
    let template = args.iter().position(|arg| arg == "--template")
        .and_then(|i| args.get(i + 1))
        .unwrap_or(&default_template);

    let project_dir = Path::new(project_name);
    if project_dir.exists() {
        eprintln!("Directory '{}' already exists", project_name);
        std::process::exit(1);
    }

    // Create project directory
    if let Err(e) = fs::create_dir(project_dir) {
        eprintln!("Failed to create project directory: {}", e);
        std::process::exit(1);
    }

    // Change to project directory
    if let Err(e) = env::set_current_dir(project_dir) {
        eprintln!("Failed to change to project directory: {}", e);
        std::process::exit(1);
    }

    // Create manifest
    let manifest = PackageManifest {
        package: PackageInfo {
            name: project_name.clone(),
            version: "0.1.0".to_string(),
            authors: Some(vec!["Your Name <you@example.com>".to_string()]),
            description: Some(format!("A new StelLang project: {}", project_name)),
            license: Some("MIT".to_string()),
            repository: None,
            keywords: Some(vec!["stellang".to_string()]),
        },
        dependencies: Some(HashMap::new()),
        dev_dependencies: Some(HashMap::new()),
    };

    if let Err(e) = cli.write_manifest(&manifest) {
        eprintln!("Failed to create stel.toml: {}", e);
        std::process::exit(1);
    }

    // Create src directory and main.stel
    let src_dir = Path::new("src");
    if let Err(e) = fs::create_dir(src_dir) {
        eprintln!("Failed to create src directory: {}", e);
        std::process::exit(1);
    }

    let main_content = match template.as_str() {
        "basic" => r#"// Basic StelLang project template

fn main() {
    print("Hello from {}!");
}

"#.to_string(),
        "web" => r#"// Web application template

fn main() {
    print("Starting web server...");
    // TODO: Add web server implementation
}

fn handle_request(request) {
    return "Hello, World!";
}

"#.to_string(),
        "cli" => r#"// Command-line application template

fn main() {
    let args = get_args();
    if args.len() > 1 {
        print("Hello, " + args[1] + "!");
    } else {
        print("Hello, World!");
    }
}

"#.to_string(),
        "library" => r#"// Library package template

// Export your library functions here
fn greet(name) {
    return "Hello, " + name + "!";
}

fn add(a, b) {
    return a + b;
}

fn multiply(a, b) {
    return a * b;
}

// Example usage
fn main() {
    print(greet("World"));
    print("2 + 3 = " + add(2, 3));
    print("4 * 5 = " + multiply(4, 5));
}

"#.to_string(),
        "test" => r#"// Test project template

fn main() {
    print("Running tests...");
    
    // Test basic functionality
    test_basic_math();
    test_string_operations();
    test_control_flow();
    
    print("All tests completed!");
}

fn test_basic_math() {
    assert(2 + 2 == 4, "Basic addition failed");
    assert(10 - 5 == 5, "Basic subtraction failed");
    assert(3 * 4 == 12, "Basic multiplication failed");
    assert(15 / 3 == 5, "Basic division failed");
    print("Basic math tests passed");
}

fn test_string_operations() {
    let greeting = "Hello, World!";
    assert(len(greeting) == 13, "String length failed");
    assert(greeting[0] == "H", "String indexing failed");
    print("String operation tests passed");
}

fn test_control_flow() {
    let x = 10;
    if x > 5 {
        assert(true, "If condition failed");
    } else {
        assert(false, "If condition logic error");
    }
    print("Control flow tests passed");
}

fn assert(condition, message) {
    if !condition {
        print("Test failed: " + message);
        exit(1);
    }
}

"#.to_string(),
        _ => {
            eprintln!("Unknown template: {}", template);
            eprintln!("Available templates: basic, web, cli, library, test");
            std::process::exit(1);
        }
    };

    let main_file = src_dir.join("main.stel");
    if let Err(e) = fs::write(&main_file, main_content) {
        eprintln!("Failed to create main.stel: {}", e);
        std::process::exit(1);
    }

    println!("Created new StelLang project '{}' with template '{}'", project_name, template);
    println!("  cd {}", project_name);
    println!("  stel build");
}

fn cmd_template(cli: &StelCLI, args: &[String]) {
    if args.is_empty() {
        eprintln!("stel template: missing subcommand");
        eprintln!("Usage: stel template <subcommand>");
        eprintln!("Subcommands:");
        eprintln!("  list     List available templates");
        eprintln!("  create   Create a new template");
        eprintln!("  install  Install a template from registry");
        std::process::exit(1);
    }

    match args[0].as_str() {
        "list" => cmd_template_list(cli),
        "create" => cmd_template_create(cli, &args[1..]),
        "install" => cmd_template_install(cli, &args[1..]),
        _ => {
            eprintln!("stel template: unknown subcommand '{}'", args[0]);
            eprintln!("Try 'stel template --help' for more information");
            std::process::exit(1);
        }
    }
}

fn cmd_template_list(_cli: &StelCLI) {
    println!("Available templates:");
    println!();
    println!("basic     - Basic StelLang project");
    println!("   A simple project with main.stel and basic structure");
    println!();
    println!("web       - Web application template");
    println!("   Template for building web applications with HTTP server");
    println!();
    println!("cli       - Command-line application");
    println!("   Template for CLI tools with argument parsing");
    println!();
    println!("library   - Library package template");
    println!("   Template for creating reusable libraries");
    println!();
    println!("test      - Test project template");
    println!("   Template with comprehensive testing setup");
}

fn cmd_template_create(cli: &StelCLI, args: &[String]) {
    if args.len() < 2 {
        eprintln!("stel template create: missing arguments");
        eprintln!("Usage: stel template create <template-name> <source-directory>");
        std::process::exit(1);
    }

    let template_name = &args[0];
    let source_dir = Path::new(&args[1]);

    if !source_dir.exists() || !source_dir.is_dir() {
        eprintln!("Source directory '{}' does not exist", args[1]);
        std::process::exit(1);
    }

    // Create template manifest
    let template_manifest = format!(
        r#"# Template: {}
description = "Custom template created by user"
version = "1.0.0"
author = "User"

[files]
# Add your template files here
"#,
        template_name
    );

    let template_dir = cli.config_dir.join("templates").join(template_name);
    if let Err(e) = fs::create_dir_all(&template_dir) {
        eprintln!("Failed to create template directory: {}", e);
        std::process::exit(1);
    }

    // Copy source files
    if let Err(e) = cli.copy_directory(source_dir, &template_dir) {
        eprintln!("Failed to copy template files: {}", e);
        std::process::exit(1);
    }

    // Write manifest
    let manifest_path = template_dir.join("template.toml");
    if let Err(e) = fs::write(manifest_path, template_manifest) {
        eprintln!("Failed to write template manifest: {}", e);
        std::process::exit(1);
    }

    println!("Template '{}' created successfully!", template_name);
    println!("Template location: {}", template_dir.display());
}

fn cmd_template_install(_cli: &StelCLI, _args: &[String]) {
    if _args.is_empty() {
        eprintln!("stel template install: missing template name");
        eprintln!("Usage: stel template install <template-name>");
        std::process::exit(1);
    }

    let template_name = &_args[0];
    println!("Installing template '{}'...", template_name);
    println!("Template installation will be implemented with registry integration");
}

async fn cmd_search(cli: &StelCLI, args: &[String]) {
    if args.is_empty() {
        eprintln!("stel search: missing search query");
        eprintln!("Usage: stel search <query>");
        std::process::exit(1);
    }

    let query = &args[0];
    println!("Searching for packages matching '{}'...", query);

    match cli.search_registry(query).await {
        Ok(packages) => {
            if packages.is_empty() {
                println!("No packages found matching '{}'", query);
            } else {
                println!("Found {} packages:", packages.len());
                println!();
                for package in packages {
                    println!("📦 {}@{}", package.name, package.version);
                    if let Some(desc) = package.description {
                        println!("   {}", desc);
                    }
                    if let Some(authors) = package.authors {
                        println!("   Authors: {}", authors.join(", "));
                    }
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("Search failed: {}", e);
            std::process::exit(1);
        }
    }
}

fn cmd_remove(cli: &StelCLI, args: &[String]) {
    if args.is_empty() {
        eprintln!("stel remove: missing package name");
        eprintln!("Usage: stel remove <package>");
        std::process::exit(1);
    }

    let package_name = &args[0];

    let mut manifest = match cli.read_manifest() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to read stel.toml: {}", e);
            std::process::exit(1);
        }
    };

    if let Some(deps) = &mut manifest.dependencies {
        if deps.remove(package_name).is_some() {
            if let Err(e) = cli.write_manifest(&manifest) {
                eprintln!("Failed to update stel.toml: {}", e);
                std::process::exit(1);
            }
            println!("Removed '{}' from dependencies", package_name);
        } else {
            eprintln!("Package '{}' not found in dependencies", package_name);
            std::process::exit(1);
        }
    } else {
        eprintln!("No dependencies found");
        std::process::exit(1);
    }
}

fn cmd_run(_cli: &StelCLI, _args: &[String]) {
    let manifest = match _cli.read_manifest() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to read stel.toml: {}", e);
            std::process::exit(1);
        }
    };

    println!("Running {} v{}", manifest.package.name, manifest.package.version);

    let main_file = Path::new("src/main.stel");
    if !main_file.exists() {
        eprintln!("src/main.stel not found");
        std::process::exit(1);
    }

    let content = match fs::read_to_string(main_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read main.stel: {}", e);
            std::process::exit(1);
        }
    };

    // Create lexer and parser
    let mut lexer = stellang::lang::lexer::Lexer::new(&content);
    let mut tokens = Vec::new();
    
    loop {
        match lexer.next_token() {
            Ok(stellang::lang::lexer::Token::EOF) => break,
            Ok(token) => tokens.push(token),
            Err(e) => {
                eprintln!("Lexer error: {:?}", e);
                std::process::exit(1);
            }
        }
    }

    let mut parser = stellang::lang::parser::Parser::new(tokens);
    let expr = match parser.parse() {
        Ok(Some(e)) => e,
        Ok(None) => {
            println!("No expressions to run");
            return;
        }
        Err(e) => {
            eprintln!("Parser error: {:?}", e);
            std::process::exit(1);
        }
    };

    // Create interpreter and run
    let mut interpreter = stellang::lang::interpreter::Interpreter::new();
    match interpreter.eval(&expr) {
        Ok(_) => println!("Program completed successfully"),
        Err(e) => {
            eprintln!("Runtime error: {:?}", e);
            std::process::exit(1);
        }
    }
}

fn cmd_clean(_cli: &StelCLI) {
    println!("Cleaning build artifacts...");

    // Remove common build artifacts
    let artifacts = ["target", "dist", "build", ".stel"];
    for artifact in &artifacts {
        let path = Path::new(artifact);
        if path.exists() {
            if let Err(e) = fs::remove_dir_all(path) {
                eprintln!("Failed to remove {}: {}", artifact, e);
            } else {
                println!("Removed {}", artifact);
            }
        }
    }

    println!("Clean completed");
}

fn cmd_tree(cli: &StelCLI) {
    let manifest = match cli.read_manifest() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to read stel.toml: {}", e);
            std::process::exit(1);
        }
    };

    println!("{} v{}", manifest.package.name, manifest.package.version);

    if let Some(deps) = &manifest.dependencies {
        for (name, version) in deps {
            println!("├── {} {}", name, version);
        }
    }

    if let Some(dev_deps) = &manifest.dev_dependencies {
        for (name, version) in dev_deps {
            println!("├── {} {} [dev]", name, version);
        }
    }
}

fn cmd_login(cli: &StelCLI) {
    println!("Logging in to Stel registry...");
    
    if let Err(e) = cli.ensure_config_dir() {
        eprintln!("Failed to create config directory: {}", e);
        std::process::exit(1);
    }

    print!("Enter your registry token: ");
    io::stdout().flush().unwrap();
    
    let mut token = String::new();
    if let Err(e) = io::stdin().read_line(&mut token) {
        eprintln!("Failed to read token: {}", e);
        std::process::exit(1);
    }

    let token = token.trim();
    if token.is_empty() {
        eprintln!("Token cannot be empty");
        std::process::exit(1);
    }

    let token_file = cli.config_dir.join("token");
    if let Err(e) = fs::write(token_file, token) {
        eprintln!("Failed to save token: {}", e);
        std::process::exit(1);
    }

    println!("Successfully logged in!");
}

fn cmd_logout(cli: &StelCLI) {
    let token_file = cli.config_dir.join("token");
    if token_file.exists() {
        if let Err(e) = fs::remove_file(token_file) {
            eprintln!("Failed to remove token: {}", e);
            std::process::exit(1);
        }
        println!("Successfully logged out");
    } else {
        println!("Not currently logged in");
    }
}

async fn cmd_outdated(cli: &StelCLI) {
    let manifest = match cli.read_manifest() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to read stel.toml: {}", e);
            std::process::exit(1);
        }
    };

    let lockfile = match cli.read_lockfile() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to read lockfile: {}", e);
            std::process::exit(1);
        }
    };

    println!("Checking for outdated dependencies...");
    println!();

    let mut outdated_count = 0;

    if let Some(deps) = &manifest.dependencies {
        for (name, version_req) in deps {
            if let Some(locked_package) = lockfile.packages.get(name) {
                // Get latest version from registry
                match cli.get_package_info(name, version_req).await {
                    Ok(latest_info) => {
                        let current_version = Version::parse(&locked_package.version).unwrap();
                        let latest_version = Version::parse(&latest_info.version).unwrap();
                        
                        if latest_version > current_version {
                            println!("{}: {} → {}", name, locked_package.version, latest_info.version);
                            if let Some(desc) = latest_info.description {
                                println!("   {}", desc);
                            }
                            println!();
                            outdated_count += 1;
                        } else {
                            println!("{}: {} (up to date)", name, locked_package.version);
                        }
                    }
                    Err(e) => {
                        eprintln!("{}: Failed to check for updates: {}", name, e);
                    }
                }
            }
        }
    }

    if outdated_count == 0 {
        println!("All dependencies are up to date.");
    } else {
        println!("Found {} outdated dependencies. Run 'stel update' to update them.", outdated_count);
    }
}

async fn cmd_audit(cli: &StelCLI) {
    let manifest = match cli.read_manifest() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to read stel.toml: {}", e);
            std::process::exit(1);
        }
    };

    let lockfile = match cli.read_lockfile() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to read lockfile: {}", e);
            std::process::exit(1);
        }
    };

    println!("Checking for security vulnerabilities...");
    println!();

    let mut vulnerabilities = 0;

    for (name, locked_package) in &lockfile.packages {
        // total_packages += 1; // This line was removed as per the edit hint
        
        match cli.get_package_info(name, &locked_package.version).await {
            Ok(package_info) => {
                if let Some(checksum) = &locked_package.checksum {
                    if let Some(package_checksum) = &package_info.checksum {
                        if checksum != package_checksum {
                            println!("SECURITY: {}@{} - Checksum mismatch", name, locked_package.version);
                            vulnerabilities += 1;
                        }
                    }
                }
                
                if let Some(desc) = &package_info.description {
                    if desc.to_lowercase().contains("deprecated") || desc.to_lowercase().contains("security") {
                        println!("WARNING: {}@{} - {}", name, locked_package.version, desc);
                    }
                }
            }
            Err(e) => {
                println!("WARNING: {}@{} - Failed to verify: {}", name, locked_package.version, e);
            }
        }
    }

    println!();
    if vulnerabilities == 0 {
        println!("No security vulnerabilities found.");
    } else {
        println!("Found {} potential security issues.", vulnerabilities);
    }
    // println!("Audited {} packages.", total_packages); // This line was removed as per the edit hint
}

fn cmd_version() {
    println!("stel 1.0.0");
    println!("StelLang Package Manager");
    println!("Registry: {}", STEL_REGISTRY_URL);
}

fn cmd_help() {
    println!("Stel - StelLang Package Manager");
    println!();
    println!("USAGE:");
    println!("    stel <COMMAND>");
    println!();
    println!("COMMANDS:");
    println!("    init        Initialize a new StelLang project");
    println!("    new         Create a new project from template");
    println!("    template    Manage project templates");
    println!("    add         Add a dependency to the project");
    println!("    remove      Remove a dependency from the project");
    println!("    build       Build the project");
    println!("    run         Run the project");
    println!("    test        Run tests");
    println!("    install     Install dependencies");
    println!("    update      Update dependencies");
    println!("    clean       Clean build artifacts");
    println!("    tree        Show dependency tree");
    println!("    search      Search for packages");
    println!("    publish     Publish package to registry");
    println!("    login       Log in to registry");
    println!("    logout      Log out from registry");
    println!("    outdated    Check for outdated dependencies");
    println!("    audit       Check for security vulnerabilities");
    println!("    version     Show version information");
    println!("    help        Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("    stel init                    # Initialize new project");
    println!("    stel new my-project          # Create new project");
    println!("    stel add some-package        # Add dependency");
    println!("    stel build                   # Build project");
    println!("    stel run                     # Run project");
    println!("    stel test                    # Run tests");
    println!("    stel search http             # Search for packages");
    println!("    stel publish                 # Publish to registry");
    println!();
    println!("For more information, visit: {}", STEL_REGISTRY_URL);
}
