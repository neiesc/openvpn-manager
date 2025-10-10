use crate::cli::Args;

pub fn handle(args: &Args) -> anyhow::Result<()> {
    match args.command {
        crate::cli::Command::Start => start(&args.vpn_config)?,
        crate::cli::Command::Stop => stop(&args.vpn_config)?,
        crate::cli::Command::Status => status(&args.vpn_config)?,
    }
    Ok(())
}

fn start(config: &str) -> anyhow::Result<()> {
    println!("Starting OpenVPN 2 with {}", config);
    std::process::Command::new("openvpn")
        .args(["--config", config])
        .spawn()?;

    println!("OpenVPN 2 started (background).");
    Ok(())
}

fn stop(_config: &str) -> anyhow::Result<()> {
    // todo: stop specific openvpn2 process
    println!("Stopping OpenVPN 2 not yet implemented.");
    Ok(())
}

fn status(_config: &str) -> anyhow::Result<()> {
    let out = std::process::Command::new("pgrep")
        .arg("openvpn")
        .output()?;
    if out.status.success() {
        println!("✅ openvpn appears to be running.");
    } else {
        println!("❌ no openvpn process found.");
    }
    Ok(())
}
