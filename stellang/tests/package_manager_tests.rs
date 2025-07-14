use std::process::Command;
use std::fs;

#[test]
fn test_project_init() {
    let test_dir = "test_stel_project";
    let _ = fs::remove_dir_all(test_dir); // Clean up if exists
    let output = Command::new("cargo")
        .args(["run", "--bin", "stel", "--", "init"])
        .current_dir("./")
        .output()
        .expect("failed to run stel init");
    assert!(output.status.success(), "stel init failed: {}", String::from_utf8_lossy(&output.stderr));
    assert!(fs::metadata(format!("{}/stel.toml", test_dir)).is_ok(), "stel.toml not created");
    let _ = fs::remove_dir_all(test_dir);
}

#[test]
fn test_add_and_install_dependency() {
    let test_dir = "test_stel_add";
    let _ = fs::remove_dir_all(test_dir);
    fs::create_dir(test_dir).unwrap();
    Command::new("cargo").args(["run", "--bin", "stel", "--", "init"]).current_dir(test_dir).output().unwrap();
    let output = Command::new("cargo")
        .args(["run", "--bin", "stel", "--", "add", "examplepkg@1.0.0"])
        .current_dir(test_dir)
        .output()
        .expect("failed to run stel add");
    assert!(output.status.success(), "stel add failed: {}", String::from_utf8_lossy(&output.stderr));
    let output = Command::new("cargo")
        .args(["run", "--bin", "stel", "--", "install"])
        .current_dir(test_dir)
        .output()
        .expect("failed to run stel install");
    assert!(output.status.success(), "stel install failed: {}", String::from_utf8_lossy(&output.stderr));
    let _ = fs::remove_dir_all(test_dir);
}

#[test]
fn test_registry_search() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "stel", "--", "search", "example"])
        .output()
        .expect("failed to run stel search");
    assert!(output.status.success(), "stel search failed: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("example"), "search output missing expected package");
}

#[test]
fn test_publish_error_without_auth() {
    let test_dir = "test_stel_publish";
    let _ = fs::remove_dir_all(test_dir);
    fs::create_dir(test_dir).unwrap();
    Command::new("cargo").args(["run", "--bin", "stel", "--", "init"]).current_dir(test_dir).output().unwrap();
    let output = Command::new("cargo")
        .args(["run", "--bin", "stel", "--", "publish"])
        .current_dir(test_dir)
        .output()
        .expect("failed to run stel publish");
    assert!(!output.status.success(), "stel publish should fail without auth");
    let _ = fs::remove_dir_all(test_dir);
} 