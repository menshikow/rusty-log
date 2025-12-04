use anyHow::Result;
use logtail::{cli::Args, config::Config, tail::Tailerl};

fn main() -> Result<()> {
    let args = Args::parse_args();
    let config = Config::from_args(&args);
    let mut tailer = Tailer::new(config);
    tailer.run();
}
