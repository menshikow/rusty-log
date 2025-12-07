use crate::config::Config;
use crate::filter::Filter;
use crate::color::Colorizer;
use crate::query::Query;
use anyhow::{Context, Result};
use notify::{Config as NotifyConfig, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;
use std::sync::mpsc;

pub struct Tailer {
    config: Config,
    filter: Filter,
    colorizer: Colorizer,
    line_number: usize,
}

impl Tailer {
    pub fn new(config: Config) -> Result<Self> {
        let query = Query::new(config.filter.clone(), config.query.clone())
            .context("Failed to create query")?;
        let filter = Filter::new(query);
        let colorizer = Colorizer::new(!config.no_color, config.highlight);

        Ok(Self {
            config,
            filter,
            colorizer,
            line_number: 1,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        // Read initial lines
        self.read_initial_lines()?;

        if !self.config.follow {
            return Ok(());
        }

        // Watch for changes
        self.watch_file()?;

        Ok(())
    }

    fn read_initial_lines(&mut self) -> Result<()> {
        let file = File::open(&self.config.file)
            .with_context(|| format!("Failed to open file: {}", self.config.file))?;

        let reader = BufReader::new(file);
        let lines: Vec<String> = reader
            .lines()
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to read lines")?;

        // Show last N lines
        let start = if lines.len() > self.config.lines {
            lines.len() - self.config.lines
        } else {
            0
        };

        for line in &lines[start..] {
            self.process_line(line);
        }

        Ok(())
    }

    fn watch_file(&mut self) -> Result<()> {
        let (tx, rx) = mpsc::channel();
        let notify_config = NotifyConfig::default().with_poll_interval(std::time::Duration::from_secs(1));
        let mut watcher: RecommendedWatcher = Watcher::new(tx, notify_config)
            .context("Failed to create file watcher")?;

        watcher
            .watch(Path::new(&self.config.file), RecursiveMode::NonRecursive)
            .with_context(|| format!("Failed to watch file: {}", self.config.file))?;

        let mut last_size = std::fs::metadata(&self.config.file)
            .map(|m| m.len())
            .unwrap_or(0);

        loop {
            match rx.recv() {
                Ok(Ok(Event {
                    kind: EventKind::Modify(_),
                    ..
                })) => {
                    // File was modified, read new content
                    self.read_new_content(last_size)?;
                    last_size = std::fs::metadata(&self.config.file)
                        .map(|m| m.len())
                        .unwrap_or(last_size);
                }
                Ok(Ok(Event {
                    kind: EventKind::Remove(_),
                    ..
                })) => {
                    eprintln!("File was removed, stopping...");
                    break;
                }
                Ok(Err(e)) => {
                    eprintln!("Watcher error: {}", e);
                    break;
                }
                Err(e) => {
                    eprintln!("Channel error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn read_new_content(&mut self, last_size: u64) -> Result<()> {
        let mut file = File::open(&self.config.file)
            .with_context(|| format!("Failed to open file: {}", self.config.file))?;

        file.seek(SeekFrom::Start(last_size))
            .context("Failed to seek to last position")?;

        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.context("Failed to read line")?;
            self.process_line(&line);
        }

        Ok(())
    }

    fn process_line(&mut self, line: &str) {
        // Apply filter
        if !self.filter.should_show(line) {
            return;
        }

        // Get matches for highlighting
        let matches = self.filter.get_matches(line);

        // Colorize
        let colored = self.colorizer.colorize(line, &matches);

        // Print with optional line number
        if self.config.line_numbers {
            println!("{:6} {}", self.line_number, colored);
        } else {
            println!("{}", colored);
        }

        self.line_number += 1;
    }
}
