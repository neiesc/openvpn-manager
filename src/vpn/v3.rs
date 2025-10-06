use crate::{
    cli::{Args, Command},
    util::run_cmd,
};
use std::process::Command as SysCmd;

pub fn handle(args: &Args) -> anyhow::Result<()> {
    match args.command {
        Command::Start => start(&args.vpn_config)?,
        Command::Stop => stop(&args.vpn_config)?,
        Command::Status => status(&args.vpn_config)?,
    }
    Ok(())
}

fn start(config: &str) -> anyhow::Result<()> {
    println!("Starting VPN connection...");
    run_cmd("openvpn3", &["session-start", "--config", config])
}

fn stop(config: &str) -> anyhow::Result<()> {
    let output = SysCmd::new("openvpn3").arg("sessions-list").output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let mut current_path: Option<String> = None;
    let mut to_disconnect = Vec::new();

    for line in text.lines() {
        if let Some((key, val)) = line.split_once(':') {
            let key = key.trim();
            let val = val.trim();
            match key {
                "Path" => current_path = Some(val.to_string()),
                "Config name" if val == config => {
                    if let Some(path) = current_path.take() {
                        to_disconnect.push(path);
                    }
                }
                _ => {}
            }
        }
    }

    if to_disconnect.is_empty() {
        println!("No session found for {}", config);
    } else {
        for path in to_disconnect {
            println!("Disconnecting {}", path);
            run_cmd(
                "openvpn3",
                &["session-manage", "--session-path", &path, "--disconnect"],
            )?;
        }
    }
    Ok(())
}

fn status(config: &str) -> anyhow::Result<()> {
    let output = SysCmd::new("openvpn3").arg("sessions-list").output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let mut active = Vec::new();
    let mut current_name: Option<String> = None;
    let mut connected = false;

    for line in text.lines() {
        if let Some((k, v)) = line.split_once(':') {
            let k = k.trim();
            let v = v.trim();
            match k {
                "Config name" => current_name = Some(v.to_string()),
                "Status" if v == "Connected" => {
                    if let Some(name) = &current_name {
                        active.push(name.clone());
                        if name == config {
                            connected = true;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    if active.is_empty() {
        println!("❌ No active VPN.");
    } else {
        println!("Active: {:?}", active);
    }

    if connected {
        println!("✅ {} is connected.", config);
    } else {
        println!("❌ {} is not connected.", config);
    }

    Ok(())
}
