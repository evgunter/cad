//! MEASUREMENT ONLY (`embed/wedge-measure`, not for merge): the
//! departure re-decided beside each shared edge under three levers or
//! modes, logged. Every switch is an environment variable; with none set
//! nothing here runs.
//!
//! - `CAD_WM_LOG=<dir>`: on. Rows go to `<dir>/<pid>.log`.
//! - `CAD_WM_MUTANT=1`: the walk takes the mutant's outcome (a cell pair
//!   whose near edges all read a crease decides each departure by sign
//!   alone); otherwise PR 7329's.
#![allow(missing_docs, clippy::all, clippy::pedantic)]

use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write as _;

use crate::{Body, EdgeKey, FaceKey};

#[derive(Clone, Copy, Debug)]
pub(crate) struct EdgeInfo {
    pub crease: bool,
    pub reach: f64,
    pub fold: f64,
}

thread_local! {
    static ON: Cell<bool> = const { Cell::new(false) };
    static DEPTH_GUARD: Cell<u32> = const { Cell::new(0) };
    pub(crate) static MUTANT: Cell<bool> = const { Cell::new(false) };
    /// While set, `levered_sign` decides by the sine's sign alone.
    pub(crate) static SIGN_ALONE: Cell<bool> = const { Cell::new(false) };
    /// While set, the departure's arm is this length.
    pub(crate) static ARM: Cell<Option<f64>> = const { Cell::new(None) };
    /// Every sine `levered_sign` was handed since the last clear.
    pub(crate) static RATES: RefCell<Vec<(f64, f64)>> = const { RefCell::new(Vec::new()) };
    /// The far-side gap of the separator that certified, set at the
    /// certifying return.
    pub(crate) static GAP: Cell<Option<f64>> = const { Cell::new(None) };
    /// The bisector sum test, set where planes_through is read.
    pub(crate) static SUM: RefCell<Vec<SumRow>> = const { RefCell::new(Vec::new()) };
    pub(crate) static EDGES: RefCell<BTreeMap<EdgeKey, EdgeInfo>> = const { RefCell::new(BTreeMap::new()) };
    pub(crate) static CELL: Cell<(u32, u32)> = const { Cell::new((0, 0)) };
    static COUNTS: Cell<[usize; 5]> = const { Cell::new([0; 5]) };
    static FLIPS: RefCell<BTreeSet<(FaceKey, FaceKey)>> = const { RefCell::new(BTreeSet::new()) };
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct SumRow {
    pub bisector: bool,
    pub rx: Option<(f64, f64)>,
    pub ry: Option<(f64, f64)>,
    pub alone_x: bool,
    pub alone_y: bool,
    pub opposite: bool,
    pub sum_clears: bool,
}

pub(crate) fn on() -> bool {
    ON.with(Cell::get)
}

fn write(line: &str) {
    let Ok(dir) = std::env::var("CAD_WM_LOG") else {
        return;
    };
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("{dir}/{}.log", std::process::id()))
    {
        let _ = f.write_all(line.as_bytes());
    }
}

fn test_name() -> String {
    std::env::args()
        .skip(1)
        .find(|a| !a.starts_with('-'))
        .unwrap_or_default()
}

pub(crate) fn emit(tag: &str, line: &str) {
    let run = if MUTANT.with(Cell::get) { "M" } else { "P" };
    write(&format!("{tag}\t{run}\t{}\t{line}\n", test_name()));
}

/// The top-level ask begins: returns whether this is the outermost.
pub(crate) fn enter<T: geom_core::Decide + geom_core::Bounds>(
    body: &Body<T>,
    faces: &[FaceKey],
    band: geom_core::Band,
) -> bool {
    if std::env::var_os("CAD_WM_LOG").is_none() {
        return false;
    }
    let outer = DEPTH_GUARD.with(|d| {
        let v = d.get();
        d.set(v + 1);
        v == 0
    });
    if !outer {
        return false;
    }
    ON.with(|o| o.set(true));
    MUTANT.with(|m| m.set(std::env::var_os("CAD_WM_MUTANT").is_some()));
    FLIPS.with(|f| f.borrow_mut().clear());
    COUNTS.with(|c| c.set([0; 5]));
    let in_scope: BTreeSet<FaceKey> = faces.iter().copied().collect();
    let mut map = BTreeMap::new();
    let reach_home = crate::WedgeReach::new(body);
    for (e, edge) in body.edges() {
        let [a, b] = crate::edge_faces(body, e);
        if !(in_scope.contains(&a) && in_scope.contains(&b)) {
            continue;
        }
        let Some(verdict) = crate::edge_wedge(body, e, band) else {
            continue;
        };
        let Some(reach) = reach_home.edge(e) else {
            continue;
        };
        let (Some(sa), Some(sb)) = (body.face_surface(a), body.face_surface(b)) else {
            continue;
        };
        let Some(curve) = body.get_curve_geom(edge.curve).and_then(|c| c.certified()) else {
            continue;
        };
        let (t0, t1) = curve.params();
        let mid = curve.carrier().mid_point(t0, t1);
        let fold = geom_brep::folded_lever_arm(sa, sb, mid, reach);
        let crease = matches!(verdict, geom_brep::WedgeVerdict::Crease { .. });
        let class = match &verdict {
            geom_brep::WedgeVerdict::Crease { .. } => "Crease".to_owned(),
            geom_brep::WedgeVerdict::Seam { .. } => "Seam".to_owned(),
            geom_brep::WedgeVerdict::End { .. } => "End".to_owned(),
            geom_brep::WedgeVerdict::Lamina => "Lamina".to_owned(),
            geom_brep::WedgeVerdict::Mixed(_) => "Mixed".to_owned(),
            geom_brep::WedgeVerdict::InBand(_) => "InBand".to_owned(),
            _ => "Other".to_owned(),
        };
        let info = EdgeInfo {
            crease,
            reach: geom_core::Bounds::lo(reach.length()),
            fold: geom_core::Bounds::lo(fold.length()),
        };
        let kind = |s: &geom::Surface<T>| format!("{:?}", s.kind());
        emit(
            "WME",
            &format!(
                "{e:?}\t{}/{}\t{class}\treach={:e}\tfold={:e}",
                kind(sa),
                kind(sb),
                info.reach,
                info.fold
            ),
        );
        map.insert(e, info);
    }
    EDGES.with(|m| *m.borrow_mut() = map);
    true
}

pub(crate) fn flip(a: FaceKey, b: FaceKey) {
    FLIPS.with(|f| f.borrow_mut().insert((a.min(b), a.max(b))));
}

/// The top-level ask ends: each flipped face pair asked alone, in both
/// modes, then off.
pub(crate) fn leave<T: geom_core::Certifies + geom_core::CertifiedBounds>(
    outer: bool,
    body: &Body<T>,
    band: geom_core::Band,
    budgets: crate::clearance::Budgets,
    verdict: &crate::clearance::Embedding,
) {
    if std::env::var_os("CAD_WM_LOG").is_none() {
        return;
    }
    if !outer {
        DEPTH_GUARD.with(|d| d.set(d.get() - 1));
        return;
    }
    let c = COUNTS.with(Cell::get);
    emit(
        "WMV",
        &format!(
            "calls={}\tcrease_calls={}\tflips={}\tcrease_final={}\tfold_diff={}\t{verdict:?}",
            c[0], c[1], c[2], c[3], c[4]
        ),
    );
    let flips: Vec<(FaceKey, FaceKey)> = FLIPS.with(|f| f.borrow().iter().copied().collect());
    let was = MUTANT.with(Cell::get);
    for (a, b) in flips {
        let mut row = format!("{a:?}/{b:?}");
        for m in [false, true] {
            MUTANT.with(|c| c.set(m));
            let side = crate::clearance::Side {
                body,
                faces: vec![a, b],
                excluded: [(a, a), (b, b)]
                    .into_iter()
                    .collect::<crate::clearance::FacePairs>(),
            };
            let r = crate::clearance::embedded(&side, band, budgets);
            row.push_str(&format!(
                "\t{}={:?}",
                if m { "mutant" } else { "pr7329" },
                r.verdict
            ));
        }
        MUTANT.with(|c| c.set(was));
        emit("WMF", &row);
    }
    ON.with(|o| o.set(false));
    DEPTH_GUARD.with(|d| d.set(d.get() - 1));
}

/// Whether the ask running is the outermost (rows are logged there only).
pub(crate) fn top() -> bool {
    DEPTH_GUARD.with(Cell::get) == 1
}

pub(crate) fn count(crease: bool, flip: bool, fin: bool, fold_diff: bool) {
    COUNTS.with(|c| {
        let mut v = c.get();
        v[0] += 1;
        v[1] += usize::from(crease);
        v[2] += usize::from(flip);
        v[3] += usize::from(fin);
        v[4] += usize::from(fold_diff);
        c.set(v);
    });
}
