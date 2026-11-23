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
    /// The narrower face's run across the edge: of the two faces, the
    /// lesser of each one's farthest boundary vertex from the edge.
    pub narrow: f64,
    pub narrow_face: FaceKey,
}

/// While set, the departure levers at the narrower face's run across the
/// edge and sums the two faces' rates.
pub(crate) fn narrow_mode() -> bool {
    on() && NARROW.with(Cell::get)
}

pub(crate) fn fold_mode() -> bool {
    on() && FOLD.with(Cell::get)
}

pub(crate) fn refused_at(depth: u32) {
    REFUSED_AT.with(|c| {
        let (n, lo, hi) = c.get();
        c.set((n + 1, lo.min(depth), hi.max(depth)));
    });
}

thread_local! {
    static ON: Cell<bool> = const { Cell::new(false) };
    static DEPTH_GUARD: Cell<u32> = const { Cell::new(0) };
    pub(crate) static MUTANT: Cell<bool> = const { Cell::new(false) };
    pub(crate) static NARROW: Cell<bool> = const { Cell::new(false) };
    pub(crate) static FOLD: Cell<bool> = const { Cell::new(false) };
    /// Depths at which a cell pair beside an edge was refused for good
    /// this ask: (count, least, greatest).
    pub(crate) static REFUSED_AT: Cell<(usize, u32, u32)> = const { Cell::new((0, u32::MAX, 0)) };
    /// While set (inside the narrow mode's `opposite`), the summed reading.
    pub(crate) static SUMMED: Cell<bool> = const { Cell::new(false) };
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
    pub(crate) static PRES: RefCell<String> = const { RefCell::new(String::new()) };
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
    let Ok(dir) = log_dir() else {
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
    if log_dir().is_err() {
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
    MUTANT.with(|m| m.set(flag("CAD_WM_MUTANT")));
    NARROW.with(|m| m.set(flag("CAD_WM_NARROW")));
    FOLD.with(|m| m.set(flag("CAD_WM_FOLD")));
    REFUSED_AT.with(|c| c.set((0, u32::MAX, 0)));
    FLIPS.with(|f| f.borrow_mut().clear());
    COUNTS.with(|c| c.set([0; 5]));
    let in_scope: BTreeSet<FaceKey> = faces.iter().copied().collect();
    let mut map = BTreeMap::new();
    let reach_home = crate::WedgeReach::new(body);
    // Every face's boundary, sampled along its edges' curves (17 points
    // each), so a face whose vertices all lie on one edge (a disc on its
    // rim) still reads its run across it.
    let mut face_samples: BTreeMap<FaceKey, Vec<[f64; 3]>> = BTreeMap::new();
    {
        let m = |x: T| 0.5 * (geom_core::Bounds::lo(x) + geom_core::Bounds::hi(x));
        for (e, edge) in body.edges() {
            let Some(curve) = body.get_curve_geom(edge.curve).and_then(|c| c.certified()) else {
                continue;
            };
            let (t0, t1) = curve.params();
            let pts: Vec<[f64; 3]> = (0..=16)
                .map(|k| {
                    let p = curve.carrier().eval(t0 + (t1 - t0) * T::from_f64(f64::from(k) / 16.0));
                    [m(p.x), m(p.y), m(p.z)]
                })
                .collect();
            let [a, b] = crate::edge_faces(body, e);
            for f in [a, b] {
                face_samples.entry(f).or_default().extend(pts.iter().copied());
            }
        }
    }
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
        let m = |x: T| {
            let (lo, hi) = (geom_core::Bounds::lo(x), geom_core::Bounds::hi(x));
            0.5 * (lo + hi)
        };
        let p3 = |p: geom_core::Point3<T>| [m(p.x), m(p.y), m(p.z)];
        let samples: Vec<[f64; 3]> = (0..=64)
            .map(|k| {
                let t = t0 + (t1 - t0) * T::from_f64(f64::from(k) / 64.0);
                p3(curve.carrier().eval(t))
            })
            .collect();
        let dist = |q: [f64; 3]| {
            samples
                .windows(2)
                .map(|w| seg_dist(q, w[0], w[1]))
                .fold(f64::INFINITY, f64::min)
        };
        let fell_back = Cell::new(false);
        let run = |f: FaceKey| {
            let empty = Vec::new();
            let pts: &Vec<[f64; 3]> = face_samples.get(&f).unwrap_or(&empty);
            let on_curves = pts.iter().map(|&q| dist(q)).fold(0.0_f64, f64::max);
            let r = reach_home
                .face_points(f)
                .iter()
                .map(|&p| dist(p3(p)))
                .fold(on_curves, f64::max);
            // A face whose whole boundary is on this edge (a disc on its
            // one closed rim): half its boundary's diameter.
            let step = (pts.len() / 400).max(1);
            let diam = pts
                .iter()
                .step_by(step)
                .flat_map(|a| pts.iter().step_by(step).map(move |b| {
                    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
                }))
                .fold(0.0_f64, f64::max);
            if r <= 1e-12 * (1.0 + diam) && diam > 0.0 {
                fell_back.set(true);
                0.5 * diam
            } else {
                r
            }
        };
        let (ra, rb) = (run(a), run(b));
        let fell_back = fell_back.get();
        let (narrow, narrow_face) = if ra <= rb { (ra, a) } else { (rb, b) };
        let info = EdgeInfo {
            crease,
            reach: geom_core::Bounds::lo(reach.length()),
            fold: geom_core::Bounds::lo(fold.length()),
            narrow,
            narrow_face,
        };
        let kind = |s: &geom::Surface<T>| format!("{:?}", s.kind());
        emit(
            "WME",
            &format!(
                "{e:?}\t{}/{}\t{class}\treach={:e}\tfold={:e}\tnarrow={:e}\tnarrow_face={narrow_face:?}:{}\truns={ra:e}/{rb:e}\tfallback={fell_back}",
                kind(sa),
                kind(sb),
                info.reach,
                info.fold,
                info.narrow,
                kind(body.face_surface(narrow_face).unwrap_or(sa)),
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
    if log_dir().is_err() {
        return;
    }
    if !outer {
        DEPTH_GUARD.with(|d| d.set(d.get() - 1));
        return;
    }
    let c = COUNTS.with(Cell::get);
    let named = match verdict {
        crate::clearance::Embedding::Refused(crate::clearance::EngineRefusal::Uncertified {
            shared: crate::clearance::SharedCell::Edge(e),
            ..
        }) => EDGES.with(|m| {
            m.borrow().get(e).map_or("edge?".to_owned(), |i| {
                format!(
                    "edge={e:?} narrow={:e} narrow_face={:?}:{} reach={:e} fold={:e} crease={}",
                    i.narrow,
                    i.narrow_face,
                    body.face_surface(i.narrow_face).map_or("?".to_owned(), |s| format!("{:?}", s.kind())),
                    i.reach,
                    i.fold,
                    i.crease
                )
            })
        }),
        _ => String::new(),
    };
    emit(
        "WMV",
        &format!(
            "calls={}\tcrease_calls={}\tflips={}\tcrease_final={}\tfold_diff={}\tfaces={}\trefused_at={:?}\tnamed={named}\t{verdict:?}",
            c[0], c[1], c[2], c[3], c[4], body.faces().count(), REFUSED_AT.with(Cell::get)
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

fn seg_dist(q: [f64; 3], a: [f64; 3], b: [f64; 3]) -> f64 {
    let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let w = [q[0] - a[0], q[1] - a[1], q[2] - a[2]];
    let dd = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];
    let t = if dd > 0.0 {
        ((w[0] * d[0] + w[1] * d[1] + w[2] * d[2]) / dd).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let r = [w[0] - t * d[0], w[1] - t * d[1], w[2] - t * d[2]];
    (r[0] * r[0] + r[1] * r[1] + r[2] * r[2]).sqrt()
}

thread_local! {
    static OVERRIDE: RefCell<Option<(String, Vec<&'static str>)>> = const { RefCell::new(None) };
}

/// MEASUREMENT ONLY: a test's own switches, in place of the environment
/// (`None` hands back to it).
pub fn set_modes(log: Option<(String, Vec<&'static str>)>) {
    OVERRIDE.with(|o| *o.borrow_mut() = log);
}

fn log_dir() -> Result<String, std::env::VarError> {
    if let Some(d) = OVERRIDE.with(|o| o.borrow().as_ref().map(|(d, _)| d.clone())) {
        return Ok(d);
    }
    std::env::var("CAD_WM_LOG")
}

fn flag(name: &str) -> bool {
    OVERRIDE.with(|o| o.borrow().as_ref().map(|(_, f)| f.contains(&name)))
        .unwrap_or_else(|| std::env::var_os(name).is_some())
}
