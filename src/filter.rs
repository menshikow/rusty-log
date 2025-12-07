use crate::query::Query;

pub struct Filter {
    query: Query,
}

impl Filter {
    pub fn new(query: Query) -> Self {
        Self { query }
    }

    pub fn should_show(&self, line: &str) -> bool {
        self.query.matches(line)
    }

    pub fn get_matches(&self, line: &str) -> Vec<(usize, usize)> {
        self.query.find_matches(line)
    }
}
