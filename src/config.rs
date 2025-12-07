use crate::cli::Args;

#[derive(Debug, Clone)]
pub struct Config {
    pub file: String,
    pub lines: usize,
    pub filter: Option<String>,
    pub query: Option<String>,
    pub no_color: bool,
    pub line_numbers: bool,
    pub follow: bool,
    pub highlight: bool,
}

impl Config {
    pub fn from_args(args: &Args) -> Self {
        Self {
            file: args.file.clone(),
            lines: args.lines,
            filter: args.filter.clone(),
            query: args.query.clone(),
            no_color: args.no_color,
            line_numbers: args.line_numbers,
            follow: args.follow,
            highlight: args.highlight || args.filter.is_some() || args.query.is_some(),
        }
    }
}
