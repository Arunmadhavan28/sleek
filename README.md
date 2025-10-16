# 🚀 Cargo-Sleek  

[![License](https://img.shields.io/badge/license-Apache--2.0%20%2F%20MIT-blue)](LICENSE)  
[![Rust Version](https://img.shields.io/badge/Rust-Edition%202021-orange)](https://www.rust-lang.org/)  
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/your-repo/cargo-sleek/actions)  

Cargo-Sleek is a **lightweight, intelligent Cargo command tracking and optimization tool** that enhances the Rust development experience by providing command insights, dependency analysis, and build performance tracking.  

---

## ☕ Support My Work

[![Patreon](https://img.shields.io/badge/Support-Patreon-orange?logo=patreon)](https://patreon.com/Arunmadhavan28)  
[![Buy Me a Coffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-Support-yellow?logo=buymeacoffee)](https://buymeacoffee.com/arunmadhavh)


## 🎯 **Why Cargo-Sleek?**  

### ✅ **Unique Features**  

- **📊 Command Usage Analytics** – Tracks and displays your most frequently used Cargo commands.  
- **🔍 Unused Dependency Check** – Scans your `Cargo.toml` and `Cargo.lock` for unused dependencies.  
- **🚀 Build Performance Analysis** – Measures build times and saves detailed performance reports.  
- **🧹 Direct Cargo Execution** – Enables quick execution of `cargo build` and `cargo clean` directly.  
- **🔄 Reset Functionality** – Clear command usage statistics when needed.  

### 🛠 **Who is this for?**  


Cargo-Sleek is built for **Rust developers** who want to:  
```
✅ Optimize their Cargo workflow by analyzing command usage patterns.  
✅ Keep their projects clean from unnecessary dependencies.  
✅ Improve compile times with build performance tracking.  
✅ Have a single tool to execute and analyze Cargo operations.  
```
---

## 🚀 **Installation**  

To install Cargo-Sleek, clone the repository and build it manually:  

```
git clone https://github.com/your-repo/cargo-sleek.git
cd cargo-sleek
cargo build --release
```

Move the binary to a location in your $PATH for global access:
```
mv target/release/cargo-sleek /usr/local/bin/
```
Now, you can use cargo-sleek from anywhere in your terminal.

📖 Usage Guide

📊 1. Track Most Used Commands
```
cargo-sleek stats
```
Description: Displays a ranked list of your most frequently used Cargo commands.

🔄 2. Reset Command Usage Statistics
```
cargo-sleek reset
```
Description: Clears all tracked Cargo command usage data.

🔍 3. Check for Unused Dependencies
```
cargo-sleek check-deps
```
Description: Scans Cargo.toml and Cargo.lock to find dependencies that are no longer needed.

⏱️ 4. Analyze Build Performance
```
cargo-sleek build-time
```
Description: Runs cargo build, records build time, and saves a detailed report in build_timings.log.

🔨 5. Build the Project
```
cargo-sleek build
```
Description: Runs cargo build to compile your Rust project.

🧹 6. Clean the Project
```
cargo-sleek clean
```
Description: Runs cargo clean to remove the target directory and free up space.

🛠 How it Works

```Cargo-Sleek``` wraps around the Cargo command-line tool and tracks executed commands. It stores command statistics in command_stats.json and provides insights through JSON parsing, execution tracking, and build performance analysis.
```
Core Working Modules
Command Tracking – Logs how often each Cargo command is used.
Dependency Analysis – Reads Cargo.toml and Cargo.lock to find unused dependencies.
Build Timing Analysis – Measures cargo build execution time and stores logs.
🔑 Dual Licensing: Apache-2.0 & MIT
```
Cargo-Sleek is licensed under a dual Apache-2.0 and MIT license, allowing users and contributors maximum flexibility.
```
What This Means for You
You can use, modify, and distribute this software under either the Apache-2.0 OR MIT license.
You are free to use Cargo-Sleek in both open-source and commercial projects.
If you modify and distribute the project, attribution is required.
For more details, check out:
```
Apache License 2.0
MIT License
👨‍💻 Contributing
```
We welcome contributions to Cargo-Sleek! To get started:
```
Fork the repository

Create a new branch
```
git checkout -b feature-your-feature-name
```
Make your changes and commit
```
git commit -m "Added new feature"
Push to your branch
```
```git push origin feature-your-feature-name```
```
Create a Pull Request on GitHub.
```
```
🪄 Example Output
📊 Top 5 Cargo Commands
1. build       → 48 times
2. run         → 31 times
3. test        → 12 times
4. check       → 8 times
5. clean       → 3 times
```
Contribution Guidelines
```
✅ Keep code clean and well-documented.
✅ Follow Rust best practices and formatting (cargo fmt).
✅ Test your changes before submitting (cargo test).
```
📝 Future Improvements
```
🔧 Customizable configuration file for tracking preferences.
📊 More in-depth performance insights on Cargo builds.
🚀 Support for benchmarking and cache optimization.
🔄 Add automatic dependency removal for unused dependencies.
📜 License
```
This project is dual-licensed under both Apache-2.0 and MIT, giving you the flexibility to choose the license that works best for your project.

```

💬 Feedback & Support

We’d love to hear your feedback! If you encounter issues or have feature suggestions:

Submit an issue on GitHub.
Join discussions in the community forums.
⭐ Enjoy using Cargo-Sleek?
Give the project a star ⭐ on GitHub and help spread the word!

🚀 Happy Coding with Rust! 🦀
```

---
