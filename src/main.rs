use anyhow::Result;
use rusty_log::cli::Args;
use rusty_log::config::Config;
use rusty_log::tail::Tailer;

fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::from_args(&args);
    let mut tailer = Tailer::new(config)?;
    tailer.run()?;
    Ok(())
}
