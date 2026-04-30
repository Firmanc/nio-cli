use anyhow::{Context, Result};
use std::process::Command;
use sysinfo::{Pid, System, SystemExt, ProcessExt, Signal};
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
