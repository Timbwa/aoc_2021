use anyhow::{Context, Result};
use std::path::PathBuf;

pub trait Day {
    fn read_input(&self, is_test: bool) -> Result<Vec<String>> {
        let file_path = self.file_path(is_test);
        let content = std::fs::read_to_string(&file_path)
            .with_context(|| format!("could not read file `{:?}`", &file_path))?;

        let mut content_lines: Vec<String> = Vec::new();

        content.lines().for_each(|line| {
            content_lines.push(String::from(line));
        });

        Ok(content_lines)
    }

    fn file_path(&self, is_test: bool) -> PathBuf;

    fn solution_1(&self, lines: &Vec<String>) -> usize;

    fn solution_2(&self, lines: &Vec<String>) -> usize;
}
