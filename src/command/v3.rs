use std::borrow::Cow;
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
        _ => todo!()
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
    let (to_disconnect, _) = get_current_active(config, text);

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
    let output = SysCmd::new("openvpn3")
        .arg("sessions-list")
        .output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let (path, connected) = get_current_active(config, text);

    if path.is_empty() {
        println!("❌ No active VPN.");
    } else {
        println!("Active: {:?}", path);
    }

    if connected {
        println!("✅ {:?} is connected.", config);
    } else {
        println!("❌ {:?} is not connected.", config);
    }

    Ok(())
}

fn get_current_active(config: &str, text: Cow<str>) -> (Vec<String>, bool) {
    let mut path = Vec::new();
    let mut current_name: Option<String> = None;
    let mut current_path: Option<String> = None;
    let mut connected = false;

    for line in text.lines() {
        if let Some((k, v)) = line.split_once(':') {
            let k = k.trim();
            let v = v.trim();
            match k {
                "Config name" => current_name = Some(v.to_string().replace("  (Config not available)", "")),
                "Path" => current_path = Some(v.to_string()),
                "Status" if v == "Connection, Client connected" => {
                    if let Some(name) = &current_name {
                        path.push(current_path.clone().unwrap());
                        if name == config {
                            connected = true;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    (path, connected)
}
