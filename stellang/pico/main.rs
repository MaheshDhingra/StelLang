//! Pico: StelLang Package Manager (CLI Skeleton)

// This file has been moved to src/bin/pico.rs for cargo bin target support.

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

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
                println!("Installing {} v{}...", name, version);
                found = true;
            }
        }
    }
    if !found {
        println!("No dependencies to install.");
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
    println!("Publishing package...");
    println!("Name: {}", name.unwrap_or("unknown".to_string()));
    println!("Version: {}", version.unwrap_or("unknown".to_string()));
    println!("Description: {}", description.unwrap_or("none".to_string()));
    // In a real implementation, this would upload to a registry
    println!("Package published successfully!");
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
}
