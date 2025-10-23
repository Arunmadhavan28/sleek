use clap::{Arg, ArgMatches, Command as ClapCommand};
use colored::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use anyhow::{Context, Result};

const STATS_FILE: &str = "command_stats.json";

#[derive(Serialize, Deserialize, Debug, Default)]
struct CommandStats {
    usage_count: u32,
    last_used: u64,
}

/// -------------------- MODULE: stats --------------------
mod stats {
    use super::*;
    use chrono::NaiveDateTime;

    pub fn load_stats() -> HashMap<String, CommandStats> {
        if !Path::new(STATS_FILE).exists() {
            return HashMap::new();
        }
        let file = fs::File::open(STATS_FILE).ok();
        if let Some(file) = file {
            let reader = std::io::BufReader::new(file);
            if let Ok(stats) = serde_json::from_reader(reader) {
                return stats;
            }
        }
        HashMap::new()
    }

    pub fn save_stats(stats: &HashMap<String, CommandStats>) -> Result<()> {
        let json = serde_json::to_string_pretty(stats)?;
        fs::write(STATS_FILE, json).context("Failed to write stats file")
    }

    pub fn track_command(command: &str) -> Result<()> {
        let mut stats = load_stats();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let entry = stats.entry(command.to_string()).or_default();
        entry.usage_count += 1;
        entry.last_used = now;

        save_stats(&stats)?;
        Ok(())
    }

    pub fn show_stats() -> Result<()> {
        let stats = load_stats();
        if stats.is_empty() {
            println!("{}", "📊 No command usage data available.".yellow());
            return Ok(());
        }

        let mut sorted: Vec<_> = stats.iter().collect();
        sorted.sort_by(|a, b| b.1.usage_count.cmp(&a.1.usage_count));

        println!("{}", "📊 Most Used Cargo Commands:".bold().cyan());
        println!("{:<4} {:<20} {:>8} {:>20}", "#", "Command", "Count", "Last Used");

        for (i, (cmd, data)) in sorted.iter().enumerate() {
            let last_used = NaiveDateTime::from_timestamp_opt(data.last_used as i64, 0)
                .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                .unwrap_or_else(|| "N/A".to_string());

            println!(
                "{:<4} {:<20} {:>8} {:>20}",
                i + 1,
                cmd.green().bold(),
                data.usage_count,
                last_used
            );
        }
        Ok(())
    }

    pub fn reset_stats(args: &ArgMatches) -> Result<()> {
        if args.get_flag("force") {
            fs::write(STATS_FILE, "{}")?;
            println!("✅ Command stats have been reset!");
        } else {
            println!("⚠️ Run with `cargo sleek reset --force` to confirm.");
        }
        Ok(())
    }
}

/// -------------------- MODULE: performance --------------------
mod performance {
    use super::*;

    pub fn analyze_build_time(verbose: bool) -> Result<()> {
        println!("📊 Analyzing build performance...\n");
        let start = Instant::now();

        let status = Command::new("cargo")
            .arg("build")
            .arg("--timings")
            .status()
            .context("Failed to execute cargo build --timings")?;

        let duration = start.elapsed();

        if status.success() {
            let size = fs::metadata("target/debug")
                .map(|m| m.len() / 1024)
                .unwrap_or_default();
            println!("🚀 Build completed in {:.2?}", duration);
            println!("📦 Approx. build size: {} KB", size);
            if verbose {
                println!("🕓 Timing report saved in `target/cargo-timings/`");
            }
        } else {
            println!("❌ Build failed. Check logs for details.");
        }

        Ok(())
    }
}

/// -------------------- MODULE: dependencies --------------------
mod dependencies {
    use super::*;

    pub fn check_unused_deps() -> Result<()> {
        println!("🔍 Checking unused dependencies...");
        let cargo_toml = fs::read_to_string("Cargo.toml").context("Failed to read Cargo.toml")?;
        let cargo_lock = fs::read_to_string("Cargo.lock").unwrap_or_default();

        let mut unused = vec![];
        let mut in_deps = false;

        for line in cargo_toml.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("[dependencies]") {
                in_deps = true;
                continue;
            }
            if trimmed.starts_with('[') {
                in_deps = false;
            }

            if in_deps {
                if let Some(dep) = trimmed.split('=').next() {
                    let dep = dep.trim();
                    if !cargo_lock.contains(dep) {
                        unused.push(dep.to_string());
                    }
                }
            }
        }

        if unused.is_empty() {
            println!("{}", "✅ No unused dependencies found!".green());
        } else {
            println!("{}", "🚨 Unused dependencies found:".red());
            for dep in unused {
                println!("   • {}", dep);
            }
        }
        Ok(())
    }
}

/// -------------------- MODULE: executor --------------------
mod executor {
    use super::*;

    pub fn execute_cargo_command(command: &str, args: &ArgMatches, verbose: bool) -> Result<()> {
        println!("🚀 Running Cargo command: {}", command.bold().cyan());
        stats::track_command(command)?;

        let mut cmd = Command::new("cargo");
        cmd.arg(command);

        if let Some(extra_args) = args.get_many::<String>("args") {
            cmd.args(extra_args.map(|s| s.as_str()));
        }

        if verbose {
            println!("🔧 Executing: {:?}", cmd);
        }

        let status = cmd.status().context("Failed to execute cargo command")?;
        if !status.success() {
            println!("❌ Command failed with exit code: {:?}", status.code());
        }

        Ok(())
    }
}

/// -------------------- MAIN --------------------
fn main() -> Result<()> {
    let matches = ClapCommand::new("cargo-sleek")
        .version("1.1")
        .about("Tracks, analyzes, and optimizes your Cargo workflow 🚀")
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .help("Enable verbose logging")
                .global(true),
        )
        .subcommand(ClapCommand::new("stats").about("Show command usage statistics"))
        .subcommand(
            ClapCommand::new("reset")
                .about("Reset usage statistics")
                .arg(Arg::new("force").long("force").help("Force reset stats")),
        )
        .subcommand(ClapCommand::new("check-deps").about("Check for unused dependencies"))
        .subcommand(ClapCommand::new("build-time").about("Analyze build performance"))
        .subcommand(ClapCommand::new("build").about("Run cargo build"))
        .subcommand(ClapCommand::new("clean").about("Run cargo clean"))
        .subcommand(ClapCommand::new("run").about("Run the project"))
        .get_matches();

    let verbose = matches.get_flag("verbose");

    match matches.subcommand() {
        Some(("stats", _)) => stats::show_stats()?,
        Some(("reset", sub)) => stats::reset_stats(sub)?,
        Some(("check-deps", _)) => dependencies::check_unused_deps()?,
        Some(("build-time", _)) => performance::analyze_build_time(verbose)?,
        Some(("run", sub)) => executor::execute_cargo_command("run", sub, verbose)?,
        Some(("build", sub)) => executor::execute_cargo_command("build", sub, verbose)?,
        Some(("clean", sub)) => executor::execute_cargo_command("clean", sub, verbose)?,
        _ => println!("❌ Unknown command. Use `cargo sleek --help`."),
    }

    Ok(())
}
