//! Persists the best WPM between sessions.
//!
//! Scores are kept per random-word count (e.g. a 25-word run and a 50-word run
//! have separate records, since they aren't comparable). Custom-text runs are
//! not persisted — there's no meaningful key to compare them across sessions.

use std::fs;
use std::path::{Path, PathBuf};

use crate::app::Source;

fn key(source: &Source) -> Option<String> {
    match source {
        Source::Random(count) => Some(format!("words:{count}")),
        Source::Fixed(_) => None,
    }
}

fn dir() -> Option<PathBuf> {
    dirs::data_dir().map(|mut d| {
        d.push("verve");
        d
    })
}

fn read_entries(path: &Path) -> Vec<(String, f64)> {
    let Ok(contents) = fs::read_to_string(path) else {
        return Vec::new();
    };
    contents
        .lines()
        .filter_map(|line| {
            let (k, v) = line.split_once(' ')?;
            Some((k.to_string(), v.parse().ok()?))
        })
        .collect()
}

fn write_entries(path: &Path, entries: &[(String, f64)]) {
    let body: String = entries.iter().map(|(k, v)| format!("{k} {v}\n")).collect();
    let _ = fs::write(path, body);
}

fn upsert(entries: &mut Vec<(String, f64)>, key: String, wpm: f64) {
    match entries.iter_mut().find(|(k, _)| *k == key) {
        Some(entry) => entry.1 = wpm,
        None => entries.push((key, wpm)),
    }
}

/// Best WPM recorded for this source, or 0.0 if none (or not persisted).
pub fn load_best(source: &Source) -> f64 {
    let Some(key) = key(source) else {
        return 0.0;
    };
    let Some(path) = dir().map(|d| d.join("highscores")) else {
        return 0.0;
    };
    read_entries(&path)
        .into_iter()
        .find(|(k, _)| *k == key)
        .map(|(_, wpm)| wpm)
        .unwrap_or(0.0)
}

/// Store `wpm` as the best for this source. No-op for non-persisted sources.
pub fn save_best(source: &Source, wpm: f64) {
    let Some(key) = key(source) else {
        return;
    };
    let Some(dir) = dir() else {
        return;
    };
    let path = dir.join("highscores");

    let mut entries = read_entries(&path);
    upsert(&mut entries, key, wpm);

    let _ = fs::create_dir_all(&dir);
    write_entries(&path, &entries);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_and_updates_entries() {
        let path = std::env::temp_dir().join(format!("verve-test-{}", std::process::id()));
        let _ = fs::remove_file(&path);

        let mut entries = read_entries(&path);
        assert!(entries.is_empty());

        upsert(&mut entries, "words:25".into(), 80.0);
        upsert(&mut entries, "words:50".into(), 60.0);
        write_entries(&path, &entries);

        let mut reloaded = read_entries(&path);
        assert_eq!(reloaded.len(), 2);
        assert_eq!(
            reloaded.iter().find(|(k, _)| k == "words:25").unwrap().1,
            80.0
        );

        // Upsert overwrites the existing key rather than appending.
        upsert(&mut reloaded, "words:25".into(), 95.0);
        assert_eq!(reloaded.len(), 2);
        assert_eq!(
            reloaded.iter().find(|(k, _)| k == "words:25").unwrap().1,
            95.0
        );

        let _ = fs::remove_file(&path);
    }
}
