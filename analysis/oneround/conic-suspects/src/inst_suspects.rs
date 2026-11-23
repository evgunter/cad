//! SCRATCH (oneround/conic-suspects-probe, never merges): a tally of who
//! reads the DERIVED centre of a large arc-held ellipse. No behaviour
//! changes: `Ellipse::frame`'s arc arm calls [`note`], which records the
//! call chain when `SUSPECT_TRACE` is set.
#![allow(clippy::all, clippy::pedantic, missing_docs)]
use std::collections::BTreeMap;
use std::sync::Mutex;

static LOG: Mutex<BTreeMap<String, u64>> = Mutex::new(BTreeMap::new());

pub fn note(size: &dyn core::fmt::Debug) {
    if std::env::var_os("SUSPECT_TRACE").is_none() {
        return;
    }
    let Ok(size) = format!("{size:?}").parse::<f64>() else { return };
    let min = std::env::var("SUSPECT_MIN").ok().and_then(|s| s.parse().ok()).unwrap_or(1e3);
    if size.abs() < min {
        return;
    }
    let bt = std::backtrace::Backtrace::force_capture().to_string();
    if std::env::var_os("SUSPECT_RAW").is_some() { eprintln!("{bt}"); }
    let mut names: Vec<String> = Vec::new();
    let mut seen = false;
    let lines: Vec<&str> = bt.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let l = lines[i].trim();
        i += 1;
        let Some((idx, rest)) = l.split_once(": ") else { continue };
        if !idx.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        let at = lines.get(i).map_or("", |a| a.trim());
        if !seen {
            seen = rest == "note" || rest.contains("suspects::note");
            continue;
        }
        if at.contains("/rustc/") || at.contains("curves/conic.rs") {
            continue;
        }
        let file = at.trim_start_matches("at ./crates/");
        let file = file.rsplitn(2, ':').nth(1).unwrap_or(file);
        let name = rest.replace("::<f64>", "").replace("<f64>", "");
        let name = name.rsplit("::").find(|s| !s.starts_with("{closure")).unwrap_or(&name).to_owned();
        names.push(format!("{name}@{file}"));
        if names.len() == 6 {
            break;
        }
    }
    *LOG.lock().unwrap().entry(names.join(" < ")).or_insert(0) += 1;
}

pub fn take() -> Vec<(String, u64)> {
    std::mem::take(&mut *LOG.lock().unwrap()).into_iter().collect()
}
