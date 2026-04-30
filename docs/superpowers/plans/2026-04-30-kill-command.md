# Kill Command Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement `nio kill <identifiers>...` to kill processes by port or PID with interactive confirmation.

**Architecture:** A hybrid discovery mechanism that checks if an identifier is a port (listening process) or a PID. It collects all matches, displays them in a formatted table, and uses `dialoguer` for confirmation before killing via `sysinfo`.

**Tech Stack:** Rust, `clap`, `sysinfo`, `tabwriter`, `dialoguer`.

---

### Task 1: Add Dependencies

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1: Add `sysinfo` and `tabwriter` to `Cargo.toml`**

```toml
[dependencies]
# ... existing
sysinfo = "0.33.1"
tabwriter = "1.4"
```

- [ ] **Step 2: Run `cargo check` to verify dependencies**

Run: `cargo check`
Expected: Success

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "chore: add sysinfo and tabwriter dependencies"
```

---

### Task 2: Update CLI Definition

**Files:**
- Modify: `src/cli.rs`

- [ ] **Step 1: Add `Kill` subcommand to `Commands` enum**

```rust
#[derive(Subcommand)]
pub enum Commands {
    // ... existing
    /// Kill processes by port or PID
    Kill {
        /// Ports or PIDs to kill
        #[arg(required = true)]
        identifiers: Vec<String>,
    },
}
```

- [ ] **Step 2: Verify CLI parses new command**

Run: `cargo run -- kill 8000`
Expected: Compiles (will fail at `main.rs` match pattern, but confirms `cli.rs` is valid).

- [ ] **Step 3: Commit**

```bash
git add src/cli.rs
git commit -m "feat: add kill subcommand to CLI"
```

---

### Task 3: Implement Process Discovery Logic

**Files:**
- Create: `src/kill.rs`

- [ ] **Step 1: Create `src/kill.rs` with discovery structures and macOS port detection**

```rust
use anyhow::{Context, Result};
use std::process::Command;
use sysinfo::{Pid, ProcessExt, System, SystemExt, Signal};
use std::io::Write;
use tabwriter::TabWriter;
use dialoguer::{theme::ColorfulTheme, Confirm};

#[derive(Debug, Clone)]
pub struct ProcessMatch {
    pub pid: u32,
    pub name: String,
    pub match_type: MatchType,
    pub identifier: String,
}

#[derive(Debug, Clone, Copy)]
pub enum MatchType {
    Port,
    Pid,
}

pub fn find_processes(identifiers: &[String]) -> Result<Vec<ProcessMatch>> {
    let mut system = System::new_all();
    system.refresh_all();
    
    let mut matches = Vec::new();
    
    for id_str in identifiers {
        let id = id_str.parse::<u32>().context(format!("Invalid identifier: {}", id_str))?;
        
        // 1. Try Port (macOS/Darwin specific using lsof)
        let port_pids = find_pids_by_port(id)?;
        if !port_pids.is_empty() {
            for pid in port_pids {
                if let Some(proc) = system.process(Pid::from(pid as usize)) {
                    matches.push(ProcessMatch {
                        pid,
                        name: proc.name().to_string(),
                        match_type: MatchType::Port,
                        identifier: id_str.clone(),
                    });
                }
            }
            continue;
        }
        
        // 2. Try PID
        if let Some(proc) = system.process(Pid::from(id as usize)) {
            matches.push(ProcessMatch {
                pid: id,
                name: proc.name().to_string(),
                match_type: MatchType::Pid,
                identifier: id_str.clone(),
            });
        }
    }
    
    Ok(matches)
}

fn find_pids_by_port(port: u32) -> Result<Vec<u32>> {
    let output = Command::new("lsof")
        .args(["-i", &format!(":{}", port), "-t", "-s", "TCP:LISTEN"])
        .output()
        .context("Failed to execute lsof")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let pids: Vec<u32> = stdout
        .lines()
        .filter_map(|line| line.trim().parse::<u32>().ok())
        .collect();

    Ok(pids)
}
```

- [ ] **Step 2: Add placeholder `handle_command`**

```rust
pub fn handle_command(identifiers: &[String]) -> Result<()> {
    let matches = find_processes(identifiers)?;
    
    if matches.is_empty() {
        eprintln!("No processes found for identifiers: {}", identifiers.join(", "));
        return Ok(());
    }

    // Display table
    let mut tw = TabWriter::new(std::io::stderr());
    writeln!(tw, "PID\tNAME\tTYPE\tIDENTIFIER")?;
    for m in &matches {
        let type_str = match m.match_type {
            MatchType::Port => "Port",
            MatchType::Pid => "PID",
        };
        writeln!(tw, "{}\t{}\t{}\t{}", m.pid, m.name, type_str, m.identifier)?;
    }
    tw.flush()?;

    // Confirmation
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Kill these processes?")
        .default(false)
        .interact()? 
    {
        kill_processes(matches)?;
    } else {
        eprintln!("Cancelled.");
    }

    Ok(())
}

fn kill_processes(matches: Vec<ProcessMatch>) -> Result<()> {
    let mut system = System::new_all();
    system.refresh_all();

    for m in matches {
        if let Some(proc) = system.process(Pid::from(m.pid as usize)) {
            if proc.kill_with(Signal::Kill).unwrap_or(false) {
                eprintln!("Killed {} ({})", m.name, m.pid);
            } else {
                eprintln!("Failed to kill {} ({})", m.name, m.pid);
            }
        }
    }
    Ok(())
}
```

- [ ] **Step 3: Commit**

```bash
git add src/kill.rs
git commit -m "feat: implement process discovery and killing logic"
```

---

### Task 4: Integration and Routing

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Add `mod kill` and route in `main.rs`**

```rust
// src/main.rs
mod kill; // Add this

// ... in main match
        Commands::Init { shell } => {
            init::handle_init(*shell)?;
        }
        Commands::Kill { identifiers } => {
            kill::handle_command(identifiers)?;
        }
```

- [ ] **Step 2: Verify compilation**

Run: `cargo build`
Expected: Success

- [ ] **Step 3: Commit**

```bash
git add src/main.rs
git commit -m "feat: route kill command in main"
```

---

### Task 5: Manual Verification

- [ ] **Step 1: Start a dummy process listening on a port**

Run: `python3 -m http.server 8000 &`
Record the PID.

- [ ] **Step 2: Try killing it by port**

Run: `cargo run -- kill 8000`
Expected: Shows the process, asks for confirmation, kills it.

- [ ] **Step 3: Try killing it by PID**

Run: `python3 -m http.server 8000 &`
Run: `cargo run -- kill <PID>`
Expected: Shows the process as type PID, kills it.

---

### Task 6: Unit Testing

**Files:**
- Modify: `src/kill.rs`

- [ ] **Step 1: Add unit tests for identifier parsing (even if minimal)**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_pids_by_port_execution() {
        // Just verify it doesn't crash, even if no port is listening
        let _ = find_pids_by_port(9999);
    }
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test kill`
Expected: Pass

- [ ] **Step 3: Commit**

```bash
git add src/kill.rs
git commit -m "test: add basic tests for kill module"
```
