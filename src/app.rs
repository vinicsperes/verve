use std::time::Instant;

use crate::confetti::{self, Particle};
use crate::words;

#[derive(Clone)]
pub enum Source {
    Random(usize),
    Fixed(String),
}

impl Source {
    fn text(&self) -> String {
        match self {
            Source::Random(count) => words::sample(*count).join(" "),
            Source::Fixed(text) => text.clone(),
        }
    }
}

pub struct App {
    pub target: Vec<char>,
    pub typed: Vec<char>,
    pub word_count: usize,
    pub confetti: Vec<Particle>,
    source: Source,
    best_wpm: f64,
    record: bool,
    started_at: Option<Instant>,
    finished_at: Option<Instant>,
    celebration_at: Option<Instant>,
}

impl App {
    pub fn new(source: Source) -> Self {
        let text = source.text();
        let word_count = text.split_whitespace().count();
        Self {
            target: text.chars().collect(),
            typed: Vec::new(),
            word_count,
            confetti: Vec::new(),
            source,
            best_wpm: 0.0,
            record: false,
            started_at: None,
            finished_at: None,
            celebration_at: None,
        }
    }

    pub fn reset(&mut self) {
        let best = self.best_wpm;
        *self = App::new(self.source.clone());
        self.best_wpm = best;
    }

    pub fn push(&mut self, c: char) {
        if self.is_finished() {
            return;
        }
        self.started_at.get_or_insert_with(Instant::now);
        self.typed.push(c);
        if self.typed.len() >= self.target.len() {
            self.finished_at = Some(Instant::now());
            self.finish();
        }
    }

    fn finish(&mut self) {
        let wpm = self.wpm();
        if wpm > self.best_wpm {
            self.best_wpm = wpm;
            self.record = true;
            self.confetti = confetti::burst(80);
            self.celebration_at = Some(Instant::now());
        }
    }

    pub fn backspace(&mut self) {
        if self.is_finished() {
            return;
        }
        self.typed.pop();
    }

    pub fn delete_word(&mut self) {
        if self.is_finished() {
            return;
        }
        while matches!(self.typed.last(), Some(&' ')) {
            self.typed.pop();
        }
        while let Some(&c) = self.typed.last() {
            if c == ' ' {
                break;
            }
            self.typed.pop();
        }
    }

    pub fn is_started(&self) -> bool {
        self.started_at.is_some()
    }

    pub fn is_finished(&self) -> bool {
        self.finished_at.is_some()
    }

    pub fn is_record(&self) -> bool {
        self.record
    }

    pub fn best_wpm(&self) -> f64 {
        self.best_wpm
    }

    pub fn celebration_elapsed(&self) -> Option<f32> {
        let t = self.celebration_at?.elapsed().as_secs_f32();
        (t < confetti::DURATION).then_some(t)
    }

    pub fn elapsed_secs(&self) -> f64 {
        match (self.started_at, self.finished_at) {
            (Some(start), Some(end)) => (end - start).as_secs_f64(),
            (Some(start), None) => start.elapsed().as_secs_f64(),
            _ => 0.0,
        }
    }

    pub fn cursor(&self) -> usize {
        self.typed.len()
    }

    pub fn correct_chars(&self) -> usize {
        self.typed
            .iter()
            .zip(self.target.iter())
            .filter(|(a, b)| a == b)
            .count()
    }

    pub fn accuracy(&self) -> f64 {
        if self.typed.is_empty() {
            return 100.0;
        }
        self.correct_chars() as f64 / self.typed.len() as f64 * 100.0
    }

    pub fn wpm(&self) -> f64 {
        let minutes = self.elapsed_secs() / 60.0;
        if minutes <= 0.0 {
            return 0.0;
        }
        (self.correct_chars() as f64 / 5.0) / minutes
    }
}
