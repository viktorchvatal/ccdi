
use std::path::{PathBuf, Path};

use ccdi_common::save_text_file;

// ============================================ PUBLIC =============================================

pub struct ProgrammableOutput {
    path: PathBuf,
    pattern: Vec<bool>,
    position: usize,
}

pub fn write_output(path: &Path, value: bool) -> Result<(), String> {
    save_text_file(
        match value {
            false => "0",
            true => "1",
        },
        path
    )
}

pub fn pattern_pwm(value: f32) -> Vec<bool> {
    let pivot = (value*100.0) as usize;

    (0..100usize)
        .map(|index| if index < pivot { true } else { false } )
        .collect()
}

pub fn status_healthy() -> Vec<bool> {
    (0..20)
        .map(|index| if index/5 == 2 || index/2 == 4 { true } else { false })
        .collect()
}

impl ProgrammableOutput {
    pub fn new(path: &str) -> Self {
        Self {
            path: PathBuf::from(path.to_owned()),
            pattern: vec![false],
            position: 0,
        }
    }

    pub fn set_pattern(&mut self, pattern: Vec<bool>) {
        self.pattern = pattern;
    }

    pub fn iterate(&mut self) -> Result<(), String> {
        self.position += 1;

        if self.position >= self.pattern.len() {
            self.position = 0;
        }

        match self.pattern.get(self.position) {
            None => Ok(()),
            Some(value) => write_output(&self.path, *value),
        }
    }
}

