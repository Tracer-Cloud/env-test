use std::env;
use std::fs;
//use std::process::Command;

fn detect_environment_type() -> String {
    if env::var("CODESPACES").is_ok() || env::var("CODESPACE_NAME").is_ok() {
        return "GitHub Codespaces".into();
    }

    if env::var("GITHUB_ACTIONS").is_ok_and(|v| v == "true") {
        return "GitHub Actions".into();
    }

    // won't be testing this lol
    if env::var("AWS_BATCH_JOB_ID").is_ok() {
        return "AWS Batch".into();
    }

    if let Ok(uuid) = fs::read_to_string("/sys/devices/virtual/dmi/id/product_uuid") {
        if uuid.to_lowercase().starts_with("ec2") {
            return "AWS EC2".into();
        }
    }

    if std::path::Path::new("/.dockerenv").exists() {
        return "Docker".into();
    }

    if let Ok(content) = fs::read_to_string("/proc/1/cgroup") {
        if content.contains("docker") || content.contains("containerd") {
            return "Docker".into();
        }
    }

    "Local".into()
}

fn main() {
    let env_type = detect_environment_type();
    println!("Detected environment: {}", env_type);
}
