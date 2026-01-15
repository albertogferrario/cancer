use std::process::Command;

/// Remove artifacts older than N days (default: 30)
pub fn run(days: u32, toolchains: bool, skip_install_check: bool) {
    // Check if cargo-sweep is installed
    if !skip_install_check && !is_sweep_installed() {
        println!("cargo-sweep not found. Install it for automatic cleanup:");
        println!("  cargo install cargo-sweep");
        println!();
        println!("Or run with --skip-install-check to skip this message.");
        return;
    }

    println!("Cleaning build artifacts older than {} days...", days);

    // Run cargo sweep --time N
    let output = match Command::new("cargo")
        .args(["sweep", "--time", &days.to_string()])
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Failed to run cargo sweep: {}", e);
            return;
        }
    };

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Parse output to show cleaned size
        if let Some(cleaned) = parse_sweep_output(&stdout) {
            println!("{}", cleaned);
        } else {
            println!("Cleanup complete.");
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Sweep failed: {}", stderr);
    }

    // Optionally clean old toolchains
    if toolchains {
        println!("Cleaning artifacts from old toolchains...");
        let output = Command::new("cargo")
            .args(["sweep", "--installed"])
            .output();

        match output {
            Ok(o) if o.status.success() => {
                println!("Toolchain cleanup complete.");
            }
            Ok(o) => {
                let stderr = String::from_utf8_lossy(&o.stderr);
                eprintln!("Toolchain sweep failed: {}", stderr);
            }
            Err(e) => {
                eprintln!("Failed to run toolchain sweep: {}", e);
            }
        }
    }
}

/// Run sweep silently, return cleaned size if any
#[allow(dead_code)] // Used by serve command
pub fn run_silent(days: u32) -> Option<String> {
    if !is_sweep_installed() {
        return None;
    }

    let output = Command::new("cargo")
        .args(["sweep", "--time", &days.to_string()])
        .output()
        .ok()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        parse_sweep_output(&stdout)
    } else {
        None
    }
}

fn is_sweep_installed() -> bool {
    Command::new("cargo")
        .args(["sweep", "--version"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn parse_sweep_output(output: &str) -> Option<String> {
    // cargo-sweep outputs lines like "Cleaned 16.6GiB total"
    for line in output.lines() {
        if line.contains("Cleaned")
            && (line.contains("GiB") || line.contains("MiB") || line.contains("KiB"))
        {
            // Extract the size portion
            if let Some(start) = line.find("Cleaned") {
                let size_part = &line[start..];
                return Some(size_part.to_string());
            }
        }
    }
    None
}
