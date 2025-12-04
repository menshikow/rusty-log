use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "logtail")]
#[command(about = "coolstory")]

pub struct Args {
    // File to tail
    #[arg(required = true)]
    pub file: String,

    // Number of lines to show initially
    #[args(short = 'n', long, default_value = "10")]
    pub lines: usize,
}

