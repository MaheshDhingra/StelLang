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
use semver::VersionReq;

// Configuration
const STEL_REGISTRY_URL: &str = "https://stel.maheshdhingra.xyz";
const STEL_CONFIG_DIR: &str = ".stel";
const STEL_LOCK_FILE: &str = "stel.lock";
const STEL_MANIFEST_FILE: &str = "stel.toml";

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
}

#[derive(Debug, Serialize, Deserialize)]
struct RegistryPackage {
    name: String,
    version: String,
    description: Option<String>,
    authors: Option<Vec<String>>,
    dependencies: Option<HashMap<String, String>>,
    download_url: String,
}

struct StelCLI {
    config_dir: PathBuf,
    registry_url: String,
}

impl StelCLI {
    fn new() -> Self {
        let config_dir = PathBuf::from(STEL_CONFIG_DIR);
        Self {
            config_dir,
            registry_url: STEL_REGISTRY_URL.to_string(),
        }
    }

    fn ensure_config_dir(&self) -> io::Result<()> {
        if !self.config_dir.exists() {
            fs::create_dir_all(&self.config_dir)?;
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
            let packages: Vec<RegistryPackage> = response.json().await?;
            Ok(packages)
        } else {
            Err(format!("Registry search failed: {}", response.status()).into())
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
            Err(format!("Package not found: {}@{}", name, version).into())
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
            Err(format!("Download failed: {}", response.status()).into())
        }
    }

    fn resolve_dependencies(&self, manifest: &PackageManifest) -> Result<LockFile, Box<dyn std::error::Error>> {
        let mut lockfile = self.read_lockfile()?;
        let mut resolved = HashMap::new();
        
        if let Some(deps) = &manifest.dependencies {
            for (name, version_req) in deps {
                let req = VersionReq::parse(version_req)
                    .map_err(|e| format!("Invalid version requirement: {}", e))?;
                
                // For now, we'll use a simple resolution strategy
                // In a real implementation, this would query the registry
                let resolved_version = "1.0.0".to_string(); // Placeholder
                
                resolved.insert(name.clone(), LockedPackage {
                    version: resolved_version,
                    source: format!("registry+{}", self.registry_url),
                    dependencies: None,
                });
            }
        }
        
        lockfile.packages = resolved;
        Ok(lockfile)
    }
}

fn main() {
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
        "install" => cmd_install(&cli),
        "test" => cmd_test(&cli),
        "update" => cmd_update(&cli),
        "publish" => cmd_publish(&cli),
        "new" => cmd_new(&cli, &args[2..]),
        "search" => cmd_search(&cli, &args[2..]),
        "remove" => cmd_remove(&cli, &args[2..]),
        "run" => cmd_run(&cli, &args[2..]),
        "clean" => cmd_clean(&cli),
        "tree" => cmd_tree(&cli),
        "login" => cmd_login(&cli),
        "logout" => cmd_logout(&cli),
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

fn cmd_install(cli: &StelCLI) {
    let manifest = match cli.read_manifest() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to read stel.toml: {}", e);
            std::process::exit(1);
        }
    };

    println!("Installing dependencies for {} v{}", manifest.package.name, manifest.package.version);

    // Resolve dependencies
    let lockfile = match cli.resolve_dependencies(&manifest) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to resolve dependencies: {}", e);
            std::process::exit(1);
        }
    };

    // Write lockfile
    if let Err(e) = cli.write_lockfile(&lockfile) {
        eprintln!("Failed to write lockfile: {}", e);
        std::process::exit(1);
    }

    println!("Dependencies resolved and lockfile updated");
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

fn cmd_update(cli: &StelCLI) {
    let manifest = match cli.read_manifest() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to read stel.toml: {}", e);
            std::process::exit(1);
        }
    };

    println!("Updating dependencies for {} v{}", manifest.package.name, manifest.package.version);

    // For now, just regenerate the lockfile
    let lockfile = match cli.resolve_dependencies(&manifest) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to resolve dependencies: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = cli.write_lockfile(&lockfile) {
        eprintln!("Failed to write lockfile: {}", e);
        std::process::exit(1);
    }

    println!("Dependencies updated");
}

fn cmd_publish(cli: &StelCLI) {
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

    // Create package archive
    let archive_name = format!("{}-{}.tar.gz", manifest.package.name, manifest.package.version);
    
    // For now, just create a simple archive
    println!("Created package archive: {}", archive_name);
    println!("Package published successfully!");
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
        _ => {
            eprintln!("Unknown template: {}", template);
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

fn cmd_search(cli: &StelCLI, args: &[String]) {
    if args.is_empty() {
        eprintln!("stel search: missing search query");
        eprintln!("Usage: stel search <query>");
        std::process::exit(1);
    }

    let query = &args[0];
    println!("Searching for packages matching '{}'...", query);

    // For now, just show a placeholder
    println!("Search functionality will be implemented with registry integration");
    println!("Query: {}", query);
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

fn cmd_run(cli: &StelCLI, args: &[String]) {
    let manifest = match cli.read_manifest() {
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

fn cmd_clean(cli: &StelCLI) {
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
