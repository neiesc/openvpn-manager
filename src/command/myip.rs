use std::process::{Command as SysCmd};

pub fn handle() -> anyhow::Result<()> {
    let output = SysCmd::new("curl")
        .args(["ipinfo.io"]).output()?;

    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}