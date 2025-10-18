// SPDX-License-Identifier: GPL-3.0-only

use std::process::{Command, Stdio};
use std::sync::OnceLock;

pub async fn set_openrc_environment(key: &str, value: &str) {
    run_optional_command("env-update", &["--user", format!("{}={}"), key, value].as_ref())
}

pub async fn start_openrc_target() {
    run_optional_command("rc-service", &["start", "cosmic-session.target", "quiet"])
}

pub fn stop_openrc_target() {
    run_optional_command("rc-service", &["stop", "cosmic-session.target", "quiet"])
}

/// Determine if OpenRC is used as the init system.
pub fn is_openrc_used() -> bool {
    match Command::new("runscript").arg("-f").arg("/etc/rc.conf").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Spawn an OpenRC service with the given name and PIDs.
pub async fn spawn_openrc_service(command: String, pids: Vec<u32>) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("rc-service");
    cmd.args(["start", &command]);

    // OpenRC expects PIDs in a specific format for service monitoring
    if !pids.is_empty() {
        let pid_list: String = pids.iter().map(|pid| pid.to_string()).collect::<Vec<String>>().join(" ");
        cmd.arg(format!("-d {}", pid_list));
    }

    cmd.env("RC_SERVICE_STARTUP_MODE", "replace");
    cmd.spawn()?.wait()?
        .success()
        .then(|_| Ok(()))
        .unwrap_or(Err("service spawn failed".into()));
    Ok(())
}

/// Generic helper function to run shell commands with OpenRC compatibility
fn run_optional_command(cmd: &str, args: &[&str]) {
    match Command::new(cmd).args(args).stdin(Stdio::null()).status() {
        Ok(status) => {
            if !status.success() {
                match status.code() {
                    Some(code) => warn!("{} {}: exit code {}", cmd, args.join(" "), code),
                    None => warn!("{} {}: terminated by signal", cmd, args.join(" ")),
                }
            }
        }
        Err(error) => {
            warn!("unable to start {} {}: {}", cmd, args.join(" "), error);
        }
    }
}

