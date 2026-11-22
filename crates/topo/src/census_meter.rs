//! LANE-LOCAL MEASUREMENT PATCH (never committed): each census run's
//! site and seconds, one line appended to `$CAD_CENSUS_METER`.
use std::cell::Cell;
use std::io::Write as _;

thread_local! {
    static SITE: Cell<&'static str> = const { Cell::new("outside-finish") };
}

/// Runs `f` with this thread's census site named `site`.
pub fn at<R>(site: &'static str, f: impl FnOnce() -> R) -> R {
    let before = SITE.with(|s| s.replace(site));
    let out = f();
    SITE.with(|s| s.set(before));
    out
}

/// The site a finish is reached from where none is named: the caller's.
#[track_caller]
pub fn here() -> &'static str {
    let at = core::panic::Location::caller();
    Box::leak(format!("{}:{}", at.file(), at.line()).into_boxed_str())
}

/// Names the site `site` unless a door already named one.
pub fn at_default<R>(site: &'static str, f: impl FnOnce() -> R) -> R {
    if SITE.with(Cell::get) == "outside-finish" {
        at(site, f)
    } else {
        f()
    }
}

pub(crate) fn timed<R>(f: impl FnOnce() -> R) -> R {
    let Some(path) = std::env::var_os("CAD_CENSUS_METER") else {
        return f();
    };
    let from = std::time::Instant::now();
    let out = f();
    let secs = from.elapsed().as_secs_f64();
    let site = SITE.with(Cell::get);
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let _ = file.write_all(format!("{site}\t{secs:.9}\n").as_bytes());
    }
    out
}

/// One line per confirm pass to `$CAD_RECORD_READS`: how many records it
/// read, and each finding's leading words.
pub(crate) fn read(records: usize, findings: &[crate::ValidationError]) {
    let Some(path) = std::env::var_os("CAD_RECORD_READS") else {
        return;
    };
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let said: Vec<String> = findings
            .iter()
            .map(|f| format!("{f:?}").chars().take(160).collect())
            .collect();
        let _ = file.write_all(
            format!(
                "{records}	{}	{}
",
                findings.len(),
                said.join(" | ")
            )
            .as_bytes(),
        );
    }
}
