//! Pico: StelLang Package Manager (CLI Skeleton)

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

// Registry directory for local simulation
const REGISTRY_DIR: &str = ".pico/registry";

// Helper to ensure registry exists
fn ensure_registry() {
    let reg = Path::new(REGISTRY_DIR);
    if !reg.exists() {
        fs::create_dir_all(reg).expect("Failed to create local registry");
    }
}

// Helper to simulate publishing a package to the local registry
fn publish_to_registry(name: &str, version: &str) {
    ensure_registry();
    let pkg_dir = Path::new(REGISTRY_DIR).join(name);
    if !pkg_dir.exists() {
        fs::create_dir_all(&pkg_dir).expect("Failed to create package dir in registry");
    }
    let ver_file = pkg_dir.join(format!("{}.toml", version));
    fs::write(ver_file, format!("name = \"{}\"\nversion = \"{}\"\n", name, version)).expect("Failed to write package version");
}

// Helper to simulate searching the registry
fn search_registry(query: &str) -> Vec<(String, String)> {
    ensure_registry();
    let mut results = Vec::new();
    if let Ok(entries) = fs::read_dir(REGISTRY_DIR) {
        for entry in entries.flatten() {
            let pkg_name = entry.file_name().to_string_lossy().to_string();
            let pkg_dir = entry.path();
            if pkg_name.contains(query) && pkg_dir.is_dir() {
                if let Ok(vers) = fs::read_dir(&pkg_dir) {
                    for ver in vers.flatten() {
                        let ver_name = ver.file_name().to_string_lossy().to_string();
                        if ver_name.ends_with(".toml") {
                            let version = ver_name.trim_end_matches(".toml").to_string();
                            results.push((pkg_name.clone(), version));
                        }
                    }
                }
            }
        }
    }
    results
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("pico: missing command. Try 'pico help'.");
        return;
    }
    match args[1].as_str() {
        "init" => cmd_init(),
        "add" => cmd_add(),
        "build" => cmd_build(),
        "install" => cmd_install(),
        "publish" => cmd_publish(),
        "help" => print_help(),
        "version" => print_version(),
        "bench" => print_stub("bench"),
        "check" => print_stub("check"),
        "clean" => cmd_clean(),
        "clippy" => print_stub("clippy"),
        "doc" => print_stub("doc"),
        "fetch" => print_stub("fetch"),
        "fix" => print_stub("fix"),
        "fmt" => print_stub("fmt"),
        "miri" => print_stub("miri"),
        "report" => print_stub("report"),
        "run" => cmd_run(),
        "rustc" => print_stub("rustc"),
        "rustdoc" => print_stub("rustdoc"),
        "test" => cmd_test(),
        "remove" => cmd_remove(),
        "tree" => cmd_tree(),
        "update" => cmd_update(),
        "vendor" => print_stub("vendor"),
        "generate-lockfile" => print_stub("generate-lockfile"),
        "locate-project" => print_stub("locate-project"),
        "metadata" => cmd_metadata(),
        "pkgid" => cmd_pkgid(),
        "search" => cmd_search(),
        "uninstall" => cmd_uninstall(),
        "login" => cmd_login(),
        "logout" => cmd_logout(),
        "owner" => cmd_owner(),
        "package" => cmd_package(),
        "yank" => cmd_yank(),
        "new" => cmd_new(),
        _ => eprintln!("pico: unknown command '{}'. Try 'pico help'.", args[1]),
    }
}

fn cmd_init() {
    let pico_toml = Path::new("pico.toml");
    if pico_toml.exists() {
        println!("pico.toml already exists.");
    } else {
        let manifest = "# pico.toml - StelLang Package Manifest\n\n[package]\nname = \"my_stellang_project\"\nversion = \"0.1.0\"\nauthors = [\"Your Name <you@example.com>\"]\ndescription = \"A new StelLang project.\"\n\n[dependencies]\n# Add dependencies here, e.g.:\n# cool_lib = \"1.0.0\"\n";
        fs::write(pico_toml, manifest).expect("Failed to write pico.toml");
        println!("Created pico.toml");
    }
    let src_dir = Path::new("src");
    if src_dir.exists() {
        println!("src directory already exists.");
    } else {
        fs::create_dir(src_dir).expect("Failed to create src directory");
        println!("Created src directory");
    }
}

fn cmd_add() {
    print!("Enter dependency name: ");
    io::stdout().flush().unwrap();
    let mut dep = String::new();
    io::stdin().read_line(&mut dep).expect("Failed to read input");
    let dep = dep.trim();
    if dep.is_empty() {
        println!("No dependency name entered.");
        return;
    }
    print!("Enter version (e.g. 1.0.0): ");
    io::stdout().flush().unwrap();
    let mut ver = String::new();
    io::stdin().read_line(&mut ver).expect("Failed to read input");
    let ver = ver.trim();
    if ver.is_empty() {
        println!("No version entered.");
        return;
    }
    // Check if package exists in registry
    ensure_registry();
    let pkg_file = Path::new(REGISTRY_DIR).join(dep).join(format!("{}.toml", ver));
    if !pkg_file.exists() {
        println!("Dependency '{}@{}' not found in registry. Try 'pico search {}' or 'pico install' to fetch.", dep, ver, dep);
        return;
    }
    let pico_toml = Path::new("pico.toml");
    if !pico_toml.exists() {
        println!("pico.toml not found. Run 'pico init' first.");
        return;
    }
    let content = fs::read_to_string(pico_toml).expect("Failed to read pico.toml");
    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
    let dep_line = format!("{} = \"{}\"", dep, ver);
    let mut inserted = false;
    for (i, line) in lines.iter().enumerate() {
        if line.trim() == "[dependencies]" {
            // Check if already present
            if lines.iter().any(|l| l.trim_start().starts_with(&format!("{} =", dep))) {
                println!("Dependency '{}' already exists.", dep);
                return;
            }
            lines.insert(i + 1, dep_line.clone());
            inserted = true;
            break;
        }
    }
    if inserted {
        fs::write(pico_toml, lines.join("\n")).expect("Failed to update pico.toml");
        println!("Added dependency: {} = \"{}\"", dep, ver);
    } else {
        println!("[dependencies] section not found in pico.toml");
    }
}

fn cmd_build() {
    let pico_toml = Path::new("pico.toml");
    if !pico_toml.exists() {
        println!("pico.toml not found. Run 'pico init' first.");
        return;
    }
    let src_main = Path::new("src/main.stl");
    if !src_main.exists() {
        println!("src/main.stl not found. Please create your main StelLang file.");
        return;
    }
    println!("Building StelLang project...");
    // Dependency resolution: print dependencies
    let content = fs::read_to_string(pico_toml).expect("Failed to read pico.toml");
    let mut in_deps = false;
    let mut found = false;
    for line in content.lines() {
        let line = line.trim();
        if line == "[dependencies]" {
            in_deps = true;
            continue;
        }
        if in_deps {
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                continue;
            }
            if let Some((name, version)) = line.split_once('=') {
                let name = name.trim();
                let version = version.trim().trim_matches('"');
                println!("Including dependency: {} v{}", name, version);
                found = true;
            }
        }
    }
    if !found {
        println!("No dependencies to build.");
    }
    // Simulate compilation of main.stl
    println!("Compiling src/main.stl ...");
    // In a real implementation, this would invoke the StelLang compiler
    println!("Build successful!");
}

fn cmd_install() {
    let pico_toml = Path::new("pico.toml");
    if !pico_toml.exists() {
        println!("pico.toml not found. Run 'pico init' first.");
        return;
    }
    let content = fs::read_to_string(pico_toml).expect("Failed to read pico.toml");
    let mut in_deps = false;
    let mut found = false;
    for line in content.lines() {
        let line = line.trim();
        if line == "[dependencies]" {
            in_deps = true;
            continue;
        }
        if in_deps {
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                continue;
            }
            if let Some((name, version)) = line.split_once('=') {
                let name = name.trim();
                let version = version.trim().trim_matches('"');
                // Simulate fetching from registry
                ensure_registry();
                let pkg_file = Path::new(REGISTRY_DIR).join(name).join(format!("{}.toml", version));
                if pkg_file.exists() {
                    println!("Installed {} v{} from registry.", name, version);
                    found = true;
                } else {
                    println!("Dependency '{}@{}' not found in registry. Try 'pico search {}' or 'pico add' to add.", name, version, name);
                }
            }
        }
    }
    if !found {
        println!("No dependencies installed.");
    } else {
        println!("All dependencies installed.");
    }
}

fn cmd_publish() {
    let pico_toml = Path::new("pico.toml");
    if !pico_toml.exists() {
        println!("pico.toml not found. Run 'pico init' first.");
        return;
    }
    let content = fs::read_to_string(pico_toml).expect("Failed to read pico.toml");
    let mut name = None;
    let mut version = None;
    let mut description = None;
    let mut in_pkg = false;
    for line in content.lines() {
        let line = line.trim();
        if line == "[package]" {
            in_pkg = true;
            continue;
        }
        if in_pkg {
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                break;
            }
            if let Some((k, v)) = line.split_once('=') {
                let key = k.trim();
                let val = v.trim().trim_matches('"');
                match key {
                    "name" => name = Some(val.to_string()),
                    "version" => version = Some(val.to_string()),
                    "description" => description = Some(val.to_string()),
                    _ => {}
                }
            }
        }
    }
    let name = name.unwrap_or("unknown".to_string());
    let version = version.unwrap_or("unknown".to_string());
    let description = description.unwrap_or("none".to_string());
    publish_to_registry(&name, &version);
    println!("Publishing package...");
    println!("Name: {}", name);
    println!("Version: {}", version);
    println!("Description: {}", description);
    println!("Package published to local registry!");
}

fn cmd_new() {
    use std::process;
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("pico new <project_name>");
        process::exit(1);
    }
    let project = &args[2];
    let project_path = Path::new(project);
    if project_path.exists() {
        eprintln!("Directory '{}' already exists.", project);
        process::exit(1);
    }
    fs::create_dir(project_path).expect("Failed to create project directory");
    fs::create_dir(project_path.join("src")).expect("Failed to create src directory");
    let manifest = format!("# pico.toml - StelLang Package Manifest\n\n[package]\nname = \"{}\"\nversion = \"0.1.0\"\nauthors = [\"Your Name <you@example.com>\"]\ndescription = \"A new StelLang project.\"\n\n[dependencies]\n# Add dependencies here\n", project);
    fs::write(project_path.join("pico.toml"), manifest).expect("Failed to write pico.toml");
    fs::write(project_path.join("src/main.stl"), "# Your StelLang code here\n").expect("Failed to write main.stl");
    println!("Created new StelLang project '{}'!", project);
}

fn cmd_remove() {
    print!("Enter dependency name to remove: ");
    io::stdout().flush().unwrap();
    let mut dep = String::new();
    io::stdin().read_line(&mut dep).expect("Failed to read input");
    let dep = dep.trim();
    if dep.is_empty() {
        println!("No dependency name entered.");
        return;
    }
    let pico_toml = Path::new("pico.toml");
    if !pico_toml.exists() {
        println!("pico.toml not found. Run 'pico init' first.");
        return;
    }
    let content = fs::read_to_string(pico_toml).expect("Failed to read pico.toml");
    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
    let before = lines.len();
    lines.retain(|l| !l.trim_start().starts_with(&format!("{} =", dep)));
    if lines.len() == before {
        println!("Dependency '{}' not found.", dep);
    } else {
        fs::write(pico_toml, lines.join("\n")).expect("Failed to update pico.toml");
        println!("Removed dependency: {}", dep);
    }
}

fn cmd_run() {
    let src_main = Path::new("src/main.stl");
    if !src_main.exists() {
        println!("src/main.stl not found. Please create your main StelLang file.");
        return;
    }
    println!("Running src/main.stl ...");
    // In a real implementation, this would invoke the StelLang interpreter or compiler
    // For now, just print the contents as a placeholder
    match fs::read_to_string(src_main) {
        Ok(code) => println!("\n{}", code),
        Err(e) => println!("Failed to read main.stl: {}", e),
    }
}

fn cmd_update() {
    let pico_toml = Path::new("pico.toml");
    if !pico_toml.exists() {
        println!("pico.toml not found. Run 'pico init' first.");
        return;
    }
    let content = fs::read_to_string(pico_toml).expect("Failed to read pico.toml");
    let mut in_deps = false;
    let mut updated = false;
    for line in content.lines() {
        let line = line.trim();
        if line == "[dependencies]" {
            in_deps = true;
            continue;
        }
        if in_deps {
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                continue;
            }
            if let Some((name, _)) = line.split_once('=') {
                let name = name.trim();
                // Find latest version in registry
                ensure_registry();
                let pkg_dir = Path::new(REGISTRY_DIR).join(name);
                if pkg_dir.exists() {
                    let mut latest = None;
                    if let Ok(vers) = fs::read_dir(&pkg_dir) {
                        for ver in vers.flatten() {
                            let ver_name = ver.file_name().to_string_lossy().to_string();
                            if ver_name.ends_with(".toml") {
                                let version = ver_name.trim_end_matches(".toml").to_string();
                                if latest.as_ref().map_or(true, |v: &String| version > *v) {
                                    latest = Some(version);
                                }
                            }
                        }
                    }
                    if let Some(latest) = latest {
                        println!("Updated {} to {} (simulated)", name, latest);
                        updated = true;
                    }
                }
            }
        }
    }
    if !updated {
        println!("No dependencies updated.");
    } else {
        println!("All dependencies updated (simulated).");
    }
}

fn cmd_clean() {
    let build_artifacts = ["build", "target", "out"];
    let mut cleaned = false;
    for dir in &build_artifacts {
        let path = Path::new(dir);
        if path.exists() {
            if fs::remove_dir_all(path).is_ok() {
                println!("Removed directory: {}", dir);
                cleaned = true;
            }
        }
    }
    if !cleaned {
        println!("No build artifacts found to clean.");
    } else {
        println!("Clean complete.");
    }
}

fn cmd_search() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: pico search <query>");
        return;
    }
    let query = &args[2];
    let results = search_registry(query);
    if results.is_empty() {
        println!("No packages found for '{}'.", query);
    } else {
        println!("Results for '{}':", query);
        for (pkg, ver) in results {
            println!("  {} {}", pkg, ver);
        }
    }
}

fn cmd_test() {
    let test_file = Path::new("tests/main.stl");
    if !test_file.exists() {
        println!("No tests found (tests/main.stl missing).");
        return;
    }
    println!("Running tests in tests/main.stl ...");
    // In a real implementation, this would invoke the StelLang interpreter or test runner
    match fs::read_to_string(test_file) {
        Ok(code) => println!("\n{}", code),
        Err(e) => println!("Failed to read tests/main.stl: {}", e),
    }
}

fn print_version() {
    println!("pico {} (StelLang Package Manager)", env!("CARGO_PKG_VERSION"));
}

fn print_stub(cmd: &str) {
    println!("pico: '{}' command is not yet implemented. (stub)", cmd);
}

fn print_help() {
    println!("pico - StelLang Package Manager\n");
    println!("Commands:");
    println!("  init      Initialize a new StelLang package");
    println!("  add       Add a dependency");
    println!("  build     Build the project");
    println!("  install   Install dependencies");
    println!("  publish   Publish the package");
    println!("  help      Show this help message");
    println!("  version   Show pico version");
    println!("  bench     Benchmark the project (stub)");
    println!("  check     Check the project (stub)");
    println!("  clean     Clean the project (stub)");
    println!("  clippy    Lint the project (stub)");
    println!("  doc       Build documentation (stub)");
    println!("  fetch     Fetch dependencies (stub)");
    println!("  fix       Fix code (stub)");
    println!("  fmt       Format code (stub)");
    println!("  miri      Run Miri (stub)");
    println!("  report     Generate a report (stub)");
    println!("  run       Run the project");
    println!("  rustc     Invoke rustc directly (stub)");
    println!("  rustdoc   Generate rustdoc directly (stub)");
    println!("  test      Run tests");
    println!("  remove    Remove a dependency");
    println!("  tree      Show dependency tree (stub)");
    println!("  update     Update dependencies (stub)");
    println!("  vendor    Vendor dependencies (stub)");
    println!("  generate-lockfile  Generate a lockfile (stub)");
    println!("  locate-project    Locate a project (stub)");
    println!("  metadata  Print package metadata (stub)");
    println!("  pkgid     Print package ID (stub)");
    println!("  search    Search for a package");
    println!("  uninstall Uninstall a package (stub)");
    println!("  login     Login to the package registry (stub)");
    println!("  logout    Logout from the package registry (stub)");
    println!("  owner     Manage package owners (stub)");
    println!("  package    Package the project (stub)");
    println!("  yank      Yank a published package (stub)");
    println!("  new      Create a new project");
    println!();
    println!("For more information, visit the StelLang documentation.");
}

fn cmd_login() {
    println!("pico login: (stub) Authenticate with the StelLang package registry. Not yet implemented.");
}

fn cmd_logout() {
    println!("pico logout: (stub) Logout from the StelLang package registry. Not yet implemented.");
}

fn cmd_owner() {
    println!("pico owner: (stub) Manage package owners. Not yet implemented.");
}

fn cmd_package() {
    println!("pico package: (stub) Package the project for distribution. Not yet implemented.");
}

fn cmd_yank() {
    println!("pico yank: (stub) Yank a published package. Not yet implemented.");
}

fn cmd_uninstall() {
    print!("Enter dependency name to uninstall: ");
    io::stdout().flush().unwrap();
    let mut dep = String::new();
    io::stdin().read_line(&mut dep).expect("Failed to read input");
    let dep = dep.trim();
    if dep.is_empty() {
        println!("No dependency name entered.");
        return;
    }
    let pico_toml = Path::new("pico.toml");
    if !pico_toml.exists() {
        println!("pico.toml not found. Run 'pico init' first.");
        return;
    }
    let content = fs::read_to_string(pico_toml).expect("Failed to read pico.toml");
    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
    let before = lines.len();
    lines.retain(|l| !l.trim_start().starts_with(&format!("{} =", dep)));
    if lines.len() == before {
        println!("Dependency '{}' not found in pico.toml.", dep);
    } else {
        fs::write(pico_toml, lines.join("\n")).expect("Failed to update pico.toml");
        println!("Uninstalled dependency: {}", dep);
    }
}

fn cmd_tree() {
    let pico_toml = Path::new("pico.toml");
    if !pico_toml.exists() {
        println!("pico.toml not found. Run 'pico init' first.");
        return;
    }
    let content = fs::read_to_string(pico_toml).expect("Failed to read pico.toml");
    println!("Dependency tree:");
    let mut in_deps = false;
    for line in content.lines() {
        let line = line.trim();
        if line == "[dependencies]" {
            in_deps = true;
            continue;
        }
        if in_deps {
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                continue;
            }
            if let Some((name, version)) = line.split_once('=') {
                let name = name.trim();
                let version = version.trim().trim_matches('"');
                println!("- {} v{}", name, version);
            }
        }
    }
}

fn cmd_metadata() {
    let pico_toml = Path::new("pico.toml");
    if !pico_toml.exists() {
        println!("pico.toml not found. Run 'pico init' first.");
        return;
    }
    let content = fs::read_to_string(pico_toml).expect("Failed to read pico.toml");
    println!("Project metadata:");
    let mut in_pkg = false;
    for line in content.lines() {
        let line = line.trim();
        if line == "[package]" {
            in_pkg = true;
            continue;
        }
        if in_pkg {
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                break;
            }
            println!("  {}", line);
        }
    }
}

fn cmd_pkgid() {
    let pico_toml = Path::new("pico.toml");
    if !pico_toml.exists() {
        println!("pico.toml not found. Run 'pico init' first.");
        return;
    }
    let content = fs::read_to_string(pico_toml).expect("Failed to read pico.toml");
    let mut name = None;
    let mut version = None;
    let mut in_pkg = false;
    for line in content.lines() {
        let line = line.trim();
        if line == "[package]" {
            in_pkg = true;
            continue;
        }
        if in_pkg {
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                break;
            }
            if let Some((k, v)) = line.split_once('=') {
                let key = k.trim();
                let val = v.trim().trim_matches('"');
                match key {
                    "name" => name = Some(val.to_string()),
                    "version" => version = Some(val.to_string()),
                    _ => {}
                }
            }
        }
    }
    if let (Some(name), Some(version)) = (name, version) {
        println!("pkgid: {}-{}", name, version);
    } else {
        println!("Could not determine package id.");
    }
}
