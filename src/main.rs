use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader};
use std::process::Command;
use clap::{ Command as ClapCommand};
use colored::*;
use std::ffi::OsString;
use std::{time::Instant};


const STATS_FILE: &str = "command_stats.json";
//const HISTORY_FILE: &str = "command_history.log";

/// Load command statistics from `command_stats.json`
fn load_stats() -> HashMap<String, u32> {
    let mut stats = HashMap::new();

    if let Ok(file) = File::open(STATS_FILE) {
        let reader = BufReader::new(file);
        match serde_json::from_reader(reader) {
            Ok(data) => stats = data,
            Err(e) => eprintln!("❌ Failed to parse {}: {}", STATS_FILE, e),
        }
    }

    stats
}

/// Save command statistics to `command_stats.json`
fn save_stats(stats: &HashMap<String, u32>) {
    if let Ok(json) = serde_json::to_string_pretty(stats) {
        if let Err(e) = fs::write(STATS_FILE, json) {
            eprintln!("❌ Failed to write to {}: {}", STATS_FILE, e);
        }
    } else {
        eprintln!("❌ Failed to serialize stats.");
    }
}

/// Track and log executed commands
fn track_command(command: &str) {
    let mut stats = load_stats();

    let count = stats.entry(command.to_string()).or_insert(0);
    *count += 1;

    save_stats(&stats);
}

/// Show the most frequently used Cargo commands
fn show_stats() {
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

fn analyze_build_time() {
    println!("📊 Analyzing build performance...\n");

    let start = Instant::now(); // Start timer

    let output = Command::new("cargo")
        .arg("build")
        .arg("--timings")
        .output()
        .expect("❌ Failed to execute `cargo build --timings`");

    let duration = start.elapsed(); // Get total build time

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Save full log
    fs::write("build_timings.log", stdout.as_bytes()).expect("❌ Failed to write build timings");

    // Extract key stats
    let slowest_task = stdout
        .lines()
        .filter(|line| line.contains("slowest"))
        .last(); // Get the slowest dependency

    println!("🚀 Build completed in {:.2?} seconds!\n", duration);

    if let Some(task) = slowest_task {
        println!("🐢 **Slowest crate:** {}", task);
        println!("💡 Try enabling incremental compilation (`CARGO_INCREMENTAL=1`).");
        println!("💡 Use `cargo build --release` for production.");
        println!("💡 Consider `cargo check` instead of `cargo build` for faster dev feedback.\n");
    } else {
        println!("✅ No slow dependencies found!\n");
    }

    println!("✅ Build timing analysis completed! Full report saved in `build_timings.log`.");
}

/// Check for unused dependencies in Cargo.toml
fn check_unused_deps() {
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

/// Main function to parse and execute commands
fn main() {
    let matches = ClapCommand::new("cargo-sleek")
        .version("1.0")
        .author("Arunmadhavan Evr <you@example.com>")
        .about("Tracks and analyzes your Cargo commands")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(ClapCommand::new("stats").about("Show command usage statistics"))
        .subcommand(ClapCommand::new("check-deps").about("Check for unused dependencies"))
        .subcommand(ClapCommand::new("build-time").about("Analyze build performance"))
        .allow_external_subcommands(true) // ✅ Allow passing cargo commands
        .get_matches();

        match matches.subcommand() {
            Some(("stats", _)) => show_stats(),
            Some(("check-deps", _)) => check_unused_deps(),
            Some(("build-time", _)) => analyze_build_time(),
            Some((external, args)) => {
                println!("🚀 Running Cargo command: cargo {}", external);
                track_command(external);
                let status = Command::new("cargo")
                    .arg(external)
                    .args(
                        args.get_many::<OsString>("")
                            .unwrap_or_default()
                            .map(|s| s.clone())
                            .collect::<Vec<_>>(),
                    )
                    .status()
                    .expect("❌ Failed to execute command");
        
                std::process::exit(status.code().unwrap_or(1));
            }
            _ => unreachable!(),
        }
}
