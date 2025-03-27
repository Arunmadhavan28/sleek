use clap::{ArgMatches, Command as ClapCommand};
use colored::*;
use serde_json;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::BufReader;
use std::process::Command;
use std::time::Instant;

const STATS_FILE: &str = "command_stats.json";

/// **Module for command statistics tracking**
mod stats {
    use super::*;

    /// Load command statistics from `command_stats.json`
    pub fn load_stats() -> HashMap<String, u32> {
        let mut stats = HashMap::new();
        if let Ok(file) = File::open(STATS_FILE) {
            let reader = BufReader::new(file);
            if let Ok(data) = serde_json::from_reader(reader) {
                stats = data;
            }
        }
        stats
    }

    /// Save command statistics to `command_stats.json`
    pub fn save_stats(stats: &HashMap<String, u32>) {
        if let Ok(json) = serde_json::to_string_pretty(stats) {
            fs::write(STATS_FILE, json).expect("❌ Failed to write stats file");
        }
    }

    /// Track executed commands
    pub fn track_command(command: &str) {
        let mut stats = load_stats();
        let full_command = format!("cargo {}", command);
        *stats.entry(full_command).or_insert(0) += 1;
        save_stats(&stats);
    }

    /// Show the most frequently used Cargo commands
    pub fn show_stats() {
        let stats = load_stats();
        if stats.is_empty() {
            println!("{}", "📊 No command usage data available.".yellow());
            return;
        }

        println!("{}", "📊 Most Used Cargo Commands:".bold().cyan());
        let mut sorted_stats: Vec<(&String, &u32)> = stats.iter().collect();
        sorted_stats.sort_by(|a, b| b.1.cmp(a.1));

        for (i, (command, count)) in sorted_stats.iter().enumerate() {
            println!("{}️⃣ {} ({} times)", i + 1, command.green().bold(), count);
        }
    }

    /// Reset statistics
    pub fn reset_stats() {
        fs::write(STATS_FILE, "{}").expect("❌ Failed to reset stats file");
        println!("✅ Command stats have been reset!");
    }
}

/// **Module for performance analysis**
mod performance {
    use super::*;

    /// Analyze Cargo build performance
    pub fn analyze_build_time() {
        println!("📊 Analyzing build performance...\n");
        let start = Instant::now();

        let output = Command::new("cargo")
            .arg("build")
            .arg("--timings")
            .output()
            .expect("❌ Failed to execute `cargo build --timings`");

        let duration = start.elapsed();
        let stdout = String::from_utf8_lossy(&output.stdout);

        fs::write("build_timings.log", stdout.as_bytes()).expect("❌ Failed to write build timings");

        println!("🚀 Build completed in {:.2?} seconds!\n", duration);
        println!("✅ Build timing analysis completed! Full report saved in `build_timings.log`.");
    }
}

/// **Module for dependency management**
mod dependencies {
    use super::*;

    /// Check for unused dependencies in Cargo.toml
    pub fn check_unused_deps() {
        println!("🔍 Checking unused dependencies...");
        let cargo_lock = fs::read_to_string("Cargo.lock").unwrap_or_default();
        let cargo_toml = fs::read_to_string("Cargo.toml").unwrap_or_default();

        let mut dependencies = Vec::new();
        let mut in_dependencies_section = false;

        for line in cargo_toml.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("[dependencies]") {
                in_dependencies_section = true;
                continue;
            }
            if trimmed.starts_with('[') {
                in_dependencies_section = false;
            }

            if in_dependencies_section {
                if let Some(dep) = trimmed.split('=').next() {
                    let dep = dep.trim();
                    if !cargo_lock.contains(dep) {
                        dependencies.push(dep.to_string());
                    }
                }
            }
        }

        if dependencies.is_empty() {
            println!("{}", "✅ No unused dependencies found!".green());
        } else {
            println!("{}", "🚨 Unused dependencies found:".red());
            for dep in dependencies {
                println!("🔹 {}", dep);
            }
        }
    }
}

/// **Module for executing external Cargo commands**
mod executor {
    use super::*;

    /// Execute unknown Cargo commands
    pub fn execute_cargo_command(command: &str, args: &ArgMatches) {
        println!("🚀 Running Cargo command: cargo {}", command);
        stats::track_command(command);
    
        let mut cmd = Command::new("cargo");
        cmd.arg(command);
    
      
        // Properly pass additional arguments
        if let Some(extra_args) = args.get_many::<String>("args") {
            cmd.args(extra_args.map(|s| s.as_str()));
        }
    
        let status = cmd.status().expect("❌ Failed to execute command");
    
        if !status.success() {
            println!("❌ Command failed with exit code: {:?}", status.code());
        }
    }
    
}

/// **Main function to parse and execute commands**
fn main() {
    let matches = ClapCommand::new("cargo-sleek")
        .version("1.0")
        .about("Tracks and analyzes your Cargo commands")
        .subcommand(ClapCommand::new("stats").about("Show command usage statistics"))
        .subcommand(ClapCommand::new("reset").about("Reset command usage statistics"))
        .subcommand(ClapCommand::new("check-deps").about("Check for unused dependencies"))
        .subcommand(ClapCommand::new("build-time").about("Analyze build performance"))
        .subcommand(ClapCommand::new("build").about("Build the project"))
        .subcommand(ClapCommand::new("clean").about("Clean the project"))
        .get_matches();

    match matches.subcommand() {
        Some(("stats", _)) => stats::show_stats(),
        Some(("reset", _)) => stats::reset_stats(),
        Some(("check-deps", _)) => dependencies::check_unused_deps(),
        Some(("build-time", _)) => performance::analyze_build_time(),
        Some(("run", sub_matches)) => executor::execute_cargo_command("run", sub_matches),
        Some(("build", sub_matches)) => executor::execute_cargo_command("build", sub_matches),
        Some(("clean", sub_matches)) => executor::execute_cargo_command("clean", sub_matches),
        _ => {
            println!("❌ Unknown command. Use `cargo-sleek --help`.");
        }
    }
}
