//! STAGE-1 MEASUREMENT PROBE. Does not ship: every reader of this module
//! goes with it. Environment switches, read once per process:
//!
//! - `CD_FACTOR` (default 0.5, today's headroom): the share of δ every
//!   chart-lane analytic face sizes at; `CD_FACTOR_CYLINDER`, `_CONE`,
//!   `_SPHERE`, `_TORUS`, `_SPINDLE` override it per chart.
//! - `CD_RULE` = `off` | `centroid` | `midpoint`: the point a refused
//!   triangle adds, on analytic rows. Spline rows always correct, at
//!   the centroid unless `midpoint` is named.
//! - `CD_BUDGET` = N: the correction's own round budget. Unset, the
//!   correction shares `MAX_GRID_RETRIES` as today.
//! - `CD_CURVED` = 1: the rectangle lane corrects too.
//! - `CD_LOG` = path: one line per face meshed, appended.

use std::cell::RefCell;
use std::io::Write;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Rule {
    Off,
    Centroid,
    Midpoint,
    /// The middle of the free side whose own chord sags most.
    Sag,
}

pub(crate) struct Cfg {
    factor: f64,
    by_chart: [Option<f64>; 5],
    pub(crate) rule: Rule,
    pub(crate) budget: Option<usize>,
    pub(crate) curved: bool,
    log: Option<String>,
}

fn num(name: &str) -> Option<f64> {
    std::env::var(name).ok().and_then(|v| v.parse().ok())
}

pub(crate) fn cfg() -> &'static Cfg {
    static CFG: OnceLock<Cfg> = OnceLock::new();
    CFG.get_or_init(|| Cfg {
        factor: num("CD_FACTOR").unwrap_or(0.5),
        by_chart: [
            num("CD_FACTOR_CYLINDER"),
            num("CD_FACTOR_CONE"),
            num("CD_FACTOR_SPHERE"),
            num("CD_FACTOR_TORUS"),
            num("CD_FACTOR_SPINDLE"),
        ],
        rule: match std::env::var("CD_RULE").as_deref() {
            Ok("centroid") => Rule::Centroid,
            Ok("midpoint") => Rule::Midpoint,
            Ok("sag") => Rule::Sag,
            _ => Rule::Off,
        },
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        budget: num("CD_BUDGET").map(|b| b as usize),
        curved: std::env::var("CD_CURVED").is_ok_and(|v| v == "1"),
        log: std::env::var("CD_LOG").ok(),
    })
}

pub(crate) fn chart_name(surface: &geom::Surface<f64>) -> &'static str {
    match surface {
        geom::Surface::Cylinder { .. } => "cylinder",
        geom::Surface::Cone { .. } => "cone",
        geom::Surface::Sphere { .. } => "sphere",
        geom::Surface::Torus { .. } => "torus",
        geom::Surface::Spindle { .. } => "spindle",
        geom::Surface::Plane { .. } => "plane",
        geom::Surface::Nurbs(_) | geom::Surface::Approx(_) => "spline",
    }
}

/// The share of δ a chart-lane analytic face on `surface` sizes at.
pub(crate) fn factor(surface: &geom::Surface<f64>) -> f64 {
    let c = cfg();
    let k = match chart_name(surface) {
        "cylinder" => 0,
        "cone" => 1,
        "sphere" => 2,
        "torus" => 3,
        "spindle" => 4,
        _ => return 1.0,
    };
    c.by_chart[k].unwrap_or(c.factor)
}

/// What one face's lane run notes for its log line.
#[derive(Default, Clone)]
pub(crate) struct Note {
    /// Triangulation passes.
    pub(crate) passes: usize,
    /// Rebuilds by cause: certificate, fold, shared side.
    pub(crate) cert_rounds: usize,
    pub(crate) fold_rounds: usize,
    pub(crate) shared_rounds: usize,
    /// Σ over passes of the points handed to the triangulation.
    pub(crate) points: usize,
    /// Points the correction added, all rounds.
    pub(crate) added: usize,
    /// The exit pass's worst certificate.
    pub(crate) worst: f64,
    /// Triangles over δ on the first pass.
    pub(crate) over_first: usize,
    pub(crate) boundary: usize,
    pub(crate) family: String,
}

thread_local! {
    static NOTE: RefCell<Note> = RefCell::new(Note::default());
}

pub(crate) fn note(f: impl FnOnce(&mut Note)) {
    NOTE.with(|n| f(&mut n.borrow_mut()));
}

fn take() -> Note {
    NOTE.with(|n| std::mem::take(&mut *n.borrow_mut()))
}

static CALLS: AtomicUsize = AtomicUsize::new(0);

/// This process's next `tessellate` call number.
pub(crate) fn next_call() -> usize {
    CALLS.fetch_add(1, Ordering::Relaxed)
}

/// The test this process runs, as far as it can be read.
pub(crate) fn test_name() -> String {
    let exe = std::env::current_exe()
        .ok()
        .and_then(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
        .unwrap_or_default();
    // Strip cargo's `-<hash>` so two builds join.
    let exe = exe.rsplit_once('-').map_or(exe.clone(), |(a, _)| a.to_owned());
    let test = std::env::var("NEXTEST_TEST_NAME").unwrap_or_else(|_| {
        std::thread::current()
            .name()
            .unwrap_or("?")
            .to_owned()
    });
    format!("{exe}::{test}")
}

/// Runs a face's lane, and logs it.
#[allow(clippy::too_many_arguments)]
pub(crate) fn logged<T>(
    test: &str,
    call: usize,
    face: topo::FaceKey,
    lane: &str,
    surface: &geom::Surface<f64>,
    delta: f64,
    eps: f64,
    triangles: impl Fn(&T) -> usize,
    run: impl FnOnce() -> Result<T, crate::types::TessellateError>,
) -> Result<T, crate::types::TessellateError> {
    let Some(path) = cfg().log.as_deref() else {
        return run();
    };
    let _ = take();
    let start = std::time::Instant::now();
    let out = run();
    let ns = start.elapsed().as_nanos();
    let n = take();
    let (result, tris) = match &out {
        Ok(p) => ("ok".to_owned(), triangles(p)),
        Err(e) => {
            let d = format!("{e:?}");
            let kind: String = d
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            (kind, 0)
        }
    };
    let line = format!(
        "{test}\t{call}\t{face:?}\t{lane}\t{chart}\t{family}\t{delta:e}\t{eps:e}\t{result}\t{tris}\t{ns}\t{passes}\t{cert}\t{fold}\t{shared}\t{points}\t{added}\t{worst:e}\t{over}\t{boundary}\n",
        chart = chart_name(surface),
        family = if n.family.is_empty() { "-" } else { &n.family },
        passes = n.passes,
        cert = n.cert_rounds,
        fold = n.fold_rounds,
        shared = n.shared_rounds,
        points = n.points,
        added = n.added,
        worst = n.worst / delta,
        over = n.over_first,
        boundary = n.boundary,
    );
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let _ = f.write_all(line.as_bytes());
    }
    out
}
