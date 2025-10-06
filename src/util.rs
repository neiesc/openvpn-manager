use std::process::Command;

pub fn run_cmd(cmd: &str, args: &[&str]) -> anyhow::Result<()> {
    eprintln!("> {} {}", cmd, args.join(" "));
    let status = Command::new(cmd).args(args).status()?;
    if !status.success() {
        anyhow::bail!("Command '{}' failed: {:?}", cmd, status);
    }
    Ok(())
}
