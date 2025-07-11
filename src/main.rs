use std::env;
use std::fs;
use std::path::Path;

async fn detect_environment_type() -> String {
    let running_in_docker = is_docker();

    if is_codespaces() {
        return "GitHub Codespaces".into();
    }

    if env::var("GITHUB_ACTIONS").is_ok_and(|v| v == "true") {
        return "GitHub Actions".into();
    }

    if env::var("AWS_BATCH_JOB_ID").is_ok() {
        return "AWS Batch".into();
    }

    if let Some(_) = detect_ec2_environment().await {
        if running_in_docker {
            return "AWS EC2 (Docker)".into();
        } else {
            return "AWS EC2".into();
        }
    }

    if is_docker() {
        return "Docker".into();
    }

    "Local".into()
}

fn is_codespaces() -> bool {
    env::var("CODESPACES").is_ok()
        || env::var("CODESPACE_NAME").is_ok()
        || env::var("HOSTNAME").map_or(false, |v| v.contains("codespaces-"))
}

async fn detect_ec2_environment() -> Option<String> {
    // Try DMI UUID
    if let Ok(uuid) = fs::read_to_string("/sys/devices/virtual/dmi/id/product_uuid") {
        if uuid.to_lowercase().starts_with("ec2") {
            return Some("AWS EC2".into());
        }
    }
    println!("using fallback mech");
    // Fallback to metadata service
    let url = "http://169.254.169.254/latest/meta-data/instance-id";
    if let Ok(resp) = reqwest::get(url).await {
        if resp.status() == 200 {
            return Some("AWS EC2 (via metadata)".into());
        }
    }

    None
}

fn is_docker() -> bool {
    // 1. Check for /.dockerenv
    if Path::new("/.dockerenv").exists() {
        return true;
    }

    // 2. Inspect /proc/1/cgroup for docker or containerd references
    if let Ok(content) = fs::read_to_string("/proc/1/cgroup") {
        if content.contains("docker") || content.contains("containerd") {
            return true;
        }
    }

    false
}

#[tokio::main]
async fn main() {
    let env_type = detect_environment_type().await;
    println!("Detected environment: {}", env_type);
    println!("All ENV vars:");

    for (key, value) in env::vars() {
        println!("{key}={value}");
    }
}
