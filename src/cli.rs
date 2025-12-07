use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "rusty-log")]
#[command(about = "A modern tail -f-style tool for log files with live filtering, coloring, and simple queries")]
#[command(version)]
pub struct Args {
    /// File to tail
    #[arg(required = true)]
    pub file: String,

    /// Number of lines to show initially
    #[arg(short = 'n', long, default_value = "10")]
    pub lines: usize,

    /// Filter pattern (regex) - only show lines matching this pattern
    #[arg(short = 'f', long)]
    pub filter: Option<String>,

    /// Query string - simple text search
    #[arg(short = 'q', long)]
    pub query: Option<String>,

    /// Disable coloring
    #[arg(long)]
    pub no_color: bool,

    /// Show line numbers
    #[arg(short = 'l', long)]
    pub line_numbers: bool,

    /// Follow file (tail -f behavior) - default is true
    #[arg(short = 'F', long, default_value = "true")]
    pub follow: bool,

    /// Highlight matching patterns
    #[arg(short = 'H', long)]
    pub highlight: bool,
}

impl Args {
    pub fn parse() -> Self {
        Parser::parse()
    }
}
