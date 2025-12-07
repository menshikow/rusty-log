use regex::Regex;

#[derive(Debug, Clone)]
pub struct Query {
    pattern: Option<Regex>,
    text: Option<String>,
}

impl Query {
    pub fn new(filter: Option<String>, query: Option<String>) -> anyhow::Result<Self> {
        let pattern = if let Some(ref f) = filter {
            Some(Regex::new(f)?)
        } else {
            None
        };

        Ok(Self {
            pattern,
            text: query,
        })
    }

    pub fn matches(&self, line: &str) -> bool {
        // If we have a regex pattern, use it
        if let Some(ref pattern) = self.pattern {
            return pattern.is_match(line);
        }

        // If we have a text query, do case-insensitive search
        if let Some(ref text) = self.text {
            return line.to_lowercase().contains(&text.to_lowercase());
        }

        // No filter/query means match everything
        true
    }

    pub fn find_matches(&self, line: &str) -> Vec<(usize, usize)> {
        let mut matches = Vec::new();

        if let Some(ref pattern) = self.pattern {
            for mat in pattern.find_iter(line) {
                matches.push((mat.start(), mat.end()));
            }
        } else if let Some(ref text) = self.text {
            let lower_line = line.to_lowercase();
            let lower_text = text.to_lowercase();
            let mut start = 0;
            while let Some(pos) = lower_line[start..].find(&lower_text) {
                let actual_pos = start + pos;
                matches.push((actual_pos, actual_pos + text.len()));
                start = actual_pos + 1;
            }
        }

        matches
    }
}
