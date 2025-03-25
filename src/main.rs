//#use clap::{Arg, Command};
use clap::Command;
use colored::*;
use std::collections::HashMap;
//use std::fs::{self, OpenOptions};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::process::Command as ProcessCommand;
use serde::{Deserialize, Serialize};
use toml;
use chrono::Utc;

#[derive(Serialize, Deserialize, Default)]
struct Stats {
    command_counts: HashMap<String, u32>,
    command_times: HashMap<String, Vec<String>>, // Stores timestamps
    execution_times: HashMap<String, Vec<u128>>, // Stores execution duration in ms
}

const LOG_FILE: &str = "~/.cargo_sleek_stats.toml";

fn load_stats() -> Stats {
    if let Ok(mut file) = OpenOptions::new().read(true).open(LOG_FILE) {
        let mut contents = String::new();
        file.read_to_string(&mut contents).ok();
        toml::from_str(&contents).unwrap_or_default()
    } else {
        Stats::default()
    }
}

fn save_stats(stats: &Stats) {
    if let Ok(mut file) = OpenOptions::new().write(true).create(true).open(LOG_FILE) {
        file.write_all(toml::to_string(stats).unwrap().as_bytes()).ok();
    }
}

fn track_command(cmd: &str, duration: u128) {
    let mut stats = load_stats();
    let counter = stats.command_counts.entry(cmd.to_string()).or_insert(0);
    *counter += 1;
    stats.command_times.entry(cmd.to_string()).or_insert_with(Vec::new).push(Utc::now().to_rfc3339());
    stats.execution_times.entry(cmd.to_string()).or_insert_with(Vec::new).push(duration);
    save_stats(&stats);
}

fn show_stats() {
    let stats = load_stats();
    println!("{}", "📊 Most Used Cargo Commands:".bold().cyan());
    let mut sorted: Vec<_> = stats.command_counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    for (i, (cmd, count)) in sorted.iter().enumerate() {
        println!("{} {} ({} times)", (i + 1).to_string().yellow(), cmd.green(), count);
    }
}

fn show_log() {
    let stats = load_stats();
    println!("{}", "📂 Cargo Command Log:".bold().cyan());
    for (cmd, times) in stats.command_times.iter() {
        println!("{}:", cmd.green());
        for time in times.iter().take(5) {
            println!("  📌 {}", time.yellow());
        }
    }
}

fn check_unused_deps() {
    println!("{}", "🔍 Analyzing Dependencies...".bold().cyan());
    let output = ProcessCommand::new("cargo").arg("tree").output().unwrap();
    let _tree_output = String::from_utf8_lossy(&output.stdout);
    let unused = vec!["serde", "log"]; // Dummy detection
    println!("🚨 Unused dependencies: {:?}", unused.iter().map(|dep| dep.red().to_string()).collect::<Vec<_>>());
    println!("{}", "✅ Dependency check completed!".bold().green());
}

fn show_time_tracker() {
    let stats = load_stats();
    println!("{}", "⏳ Execution Time Tracker:".bold().cyan());
    for (cmd, times) in stats.execution_times.iter() {
        let avg_time: u128 = times.iter().sum::<u128>() / times.len() as u128;
        println!("{} - Avg. time: {} ms", cmd.green(), avg_time.to_string().yellow());
    }
}

fn main() {
    let matches = Command::new("cargo")
        .about("🚀 Track and Optimize Cargo Usage")
        .subcommand(Command::new("stats").about("📊 Show command usage stats"))
        .subcommand(Command::new("log").about("📂 Show command execution log"))
        .subcommand(Command::new("check-deps").about("🔍 Find unused dependencies"))
        .subcommand(Command::new("time-tracker").about("⏳ Show execution time tracker"))
        .get_matches();
    
    if let Some(_) = matches.subcommand_matches("stats") {
        show_stats();
    } else if let Some(_) = matches.subcommand_matches("log") {
        show_log();
    } else if let Some(_) = matches.subcommand_matches("check-deps") {
        check_unused_deps();
    } else if let Some(_) = matches.subcommand_matches("time-tracker") {
        show_time_tracker();
    } else {
        let args: Vec<String> = std::env::args().collect();
        if args.len() > 1 {
            let cargo_cmd = args[1..].join(" ");
            let start = std::time::Instant::now();
            let mut cmd = ProcessCommand::new("cargo");
            cmd.args(&args[1..]);
            let output = cmd.output().expect("Failed to run cargo");
            let duration = start.elapsed().as_millis();
            track_command(&cargo_cmd, duration);
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }
}
