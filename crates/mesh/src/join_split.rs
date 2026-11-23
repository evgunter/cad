//! STAGE-1 INSTRUMENT (not shipped): the split of a side two patches
//! both have inside, at the point of each face's own surface over the
//! side's chart midpoint, measured on each lane's own certificate.
//!
//! Switches (all off by default, so a plain run is the tree as it is):
//! `JS_RULES_OFF` (the polygon's shared-side test answers none, which
//! turns off the CDT arms' insert and the walk's cut), `JS_CHORD_FLOOR`,
//! `JS_FOLD_CURE`, `JS_SPLIT` (the join splits instead of refusing),
//! `JS_LOG=<dir>` (one file per process of split and call lines).
#![allow(clippy::all, clippy::pedantic, clippy::nursery, missing_docs)]

use std::collections::HashMap;
use std::io::Write as _;

use geom_core::Point3;

use crate::cert;
use crate::tessellate::PatchVertex;

pub(crate) fn on(name: &str) -> bool {
    std::env::var_os(name).is_some()
}

/// One half's reading.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Half {
    pub bound: f64,
    pub verdict: cert::Verdict,
    pub unresolved: bool,
    /// `Some(true)` folded against the face's outward normal; `None`
    /// where the lane reads no fold (planar, rectangle).
    pub folded: Option<bool>,
    pub folded_vs_parent: bool,
}

/// One candidate split a lane offers: a side between two shared ids,
/// the point it would be split at, and the halves of each triangle of
/// the patch on it.
#[derive(Clone, Debug)]
pub(crate) struct SideSplit {
    pub side: (u32, u32),
    pub point: Point3<f64>,
    pub halves: Vec<Half>,
    pub parent_worst: f64,
    /// A second triangle computed another point.
    pub disagree: bool,
    /// An end is a singular point (apex, pole).
    pub apex: bool,
    /// Distance from `point` to the surface foot of the side's 3-D
    /// midpoint (the designers' probe's point); NaN where no foot.
    pub foot_gap: f64,
    /// Distance from `point` to the side's 3-D midpoint.
    pub sag: f64,
    pub kind: &'static str,
    /// On a cone: the development's midpoint's distance from `point`,
    /// and the worst of the halves at it, and whether one refuses.
    pub dev_gap: f64,
    pub dev_worst: f64,
    pub dev_refused: bool,
}

#[derive(Default, Clone, Debug)]
pub(crate) struct Splits {
    pub list: Vec<SideSplit>,
    /// `sin α` where the face is a cone.
    pub develop: Option<f64>,
    at: HashMap<(u32, u32), usize>,
}

/// What a lane hands [`Splits::offer`] per triangle, in emitted winding.
pub(crate) struct Offer<'a> {
    pub ids: [PatchVertex; 3],
    pub uv: [[f64; 2]; 3],
    pub singular: [bool; 3],
    pub shared: &'a [Point3<f64>],
    pub interior: &'a [Point3<f64>],
}

fn sub(a: Point3<f64>, b: Point3<f64>) -> [f64; 3] {
    [a.x - b.x, a.y - b.y, a.z - b.z]
}
fn cross(u: [f64; 3], v: [f64; 3]) -> [f64; 3] {
    [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]]
}
fn dot(u: [f64; 3], v: [f64; 3]) -> f64 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}
fn norm(u: [f64; 3]) -> f64 {
    dot(u, u).sqrt()
}

impl Splits {
    pub(crate) fn armed() -> bool {
        on("JS_SPLIT")
    }

    /// Offers every side of one emitted triangle whose ends are both
    /// shared. `cert` is the lane's own certificate (bound, scale) on
    /// (positions, chart corners, singular flags); `eval` the face's
    /// surface at a chart point; `outward` the face's outward normal at a
    /// point, where the lane reads folds.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn offer(
        &mut self,
        o: &Offer<'_>,
        kind: &'static str,
        delta: f64,
        eps: crate::sizing::Eps,
        cert: &dyn Fn([Point3<f64>; 3], [[f64; 2]; 3], [bool; 3]) -> (f64, f64),
        eval: Option<&dyn Fn(f64, f64) -> Point3<f64>>,
        outward: Option<&dyn Fn(Point3<f64>) -> Option<[f64; 3]>>,
        foot: &dyn Fn(Point3<f64>) -> Option<Point3<f64>>,
    ) {
        let pos = |v: PatchVertex| v.position(o.shared, o.interior);
        let tri = o.ids.map(pos);
        for q in 0..3 {
            let (x, y, z) = (q, (q + 1) % 3, (q + 2) % 3);
            let (PatchVertex::Shared(a), PatchVertex::Shared(b)) = (o.ids[x], o.ids[y]) else {
                continue;
            };
            let key = (a.min(b), a.max(b));
            // The chart midpoint; an apex end has no azimuth, so the other
            // end's `u` is taken.
            let u = match (o.singular[x], o.singular[y]) {
                (true, false) => o.uv[y][0],
                (false, true) => o.uv[x][0],
                _ => 0.5 * (o.uv[x][0] + o.uv[y][0]),
            };
            let v = 0.5 * (o.uv[x][1] + o.uv[y][1]);
            let muv = [u, v];
            let (pa, pb) = (tri[x], tri[y]);
            let mid = Point3::new(0.5 * (pa.x + pb.x), 0.5 * (pa.y + pb.y), 0.5 * (pa.z + pb.z));
            let m = eval.map_or(mid, |e| e(u, v));
            let idx = match self.at.get(&key) {
                Some(&i) => {
                    let s = &mut self.list[i];
                    if s.point.x.to_bits() != m.x.to_bits()
                        || s.point.y.to_bits() != m.y.to_bits()
                        || s.point.z.to_bits() != m.z.to_bits()
                    {
                        s.disagree = true;
                    }
                    i
                }
                None => {
                    let foot_gap = foot(mid).map_or(f64::NAN, |f| norm(sub(m, f)));
                    self.list.push(SideSplit {
                        side: key,
                        point: m,
                        halves: Vec::new(),
                        parent_worst: 0.0,
                        disagree: false,
                        apex: o.singular[x] || o.singular[y],
                        foot_gap,
                        sag: norm(sub(m, mid)),
                        kind,
                        dev_gap: f64::NAN,
                        dev_worst: 0.0,
                        dev_refused: false,
                    });
                    self.at.insert(key, self.list.len() - 1);
                    self.list.len() - 1
                }
            };
            let m = self.list[idx].point;
            // The development's midpoint on a cone (`trimmed::developed`).
            let dev = self.develop.and_then(|sin_alpha| {
                let e = eval?;
                let sigma = if o.uv[x][1] + o.uv[y][1] >= 0.0 { 1.0 } else { -1.0 };
                let d = |p: [f64; 2]| {
                    let (r, t) = (sigma * p[1], sigma * sin_alpha * p[0]);
                    (r * t.sin(), r * t.cos())
                };
                let (a, b) = (d(o.uv[x]), d(o.uv[y]));
                let (mx, my) = (0.5 * (a.0 + b.0), 0.5 * (a.1 + b.1));
                let r = (mx * mx + my * my).sqrt();
                // Unwrap to the chart branch nearest the chart midpoint.
                let t0 = mx.atan2(my);
                let u0 = t0 / (sigma * sin_alpha);
                let period = 2.0 * std::f64::consts::PI / sin_alpha;
                let k = ((u - u0) / period).round();
                let uu = u0 + k * period;
                let vv = r / sigma;
                Some(([uu, vv], e(uu, vv)))
            });
            let (pbound, _) = cert(tri, o.uv, o.singular);
            let pn = cross(sub(tri[1], tri[0]), sub(tri[2], tri[0]));
            let s = &mut self.list[idx];
            s.parent_worst = s.parent_worst.max(pbound);
            // [x, m, z] and [m, y, z], wound as the parent.
            if let Some((duv, dp)) = dev {
                s.dev_gap = norm(sub(dp, m));
                for (c, cuv, cs) in [
                    ([tri[x], dp, tri[z]], [o.uv[x], duv, o.uv[z]], [o.singular[x], false, o.singular[z]]),
                    ([dp, tri[y], tri[z]], [duv, o.uv[y], o.uv[z]], [false, o.singular[y], o.singular[z]]),
                ] {
                    let (bound, scale) = cert(c, cuv, cs);
                    s.dev_worst = s.dev_worst.max(bound);
                    s.dev_refused |= cert::verdict(bound, delta, scale, eps) != cert::Verdict::Certified;
                }
            }
            for (c, cuv, cs) in [
                (
                    [tri[x], m, tri[z]],
                    [o.uv[x], muv, o.uv[z]],
                    [o.singular[x], false, o.singular[z]],
                ),
                (
                    [m, tri[y], tri[z]],
                    [muv, o.uv[y], o.uv[z]],
                    [false, o.singular[y], o.singular[z]],
                ),
            ] {
                let (bound, scale) = cert(c, cuv, cs);
                let verdict = cert::verdict(bound, delta, scale, eps);
                let unresolved = crate::validate::resolved_normal(c).is_err();
                let cn = cross(sub(c[1], c[0]), sub(c[2], c[0]));
                let centroid = Point3::new(
                    (c[0].x + c[1].x + c[2].x) / 3.0,
                    (c[0].y + c[1].y + c[2].y) / 3.0,
                    (c[0].z + c[1].z + c[2].z) / 3.0,
                );
                let folded = outward.map(|f| f(centroid).is_some_and(|n| !(dot(cn, n) > 0.0)));
                s.halves.push(Half {
                    bound,
                    verdict,
                    unresolved,
                    folded,
                    folded_vs_parent: !(dot(cn, pn) > 0.0),
                });
            }
        }
    }

    pub(crate) fn get(&self, side: (u32, u32)) -> Option<&SideSplit> {
        self.at.get(&side).map(|&i| &self.list[i])
    }
}

fn log_file() -> Option<std::fs::File> {
    let dir = std::env::var("JS_LOG").ok()?;
    let name = format!("{dir}/{}.log", std::process::id());
    std::fs::OpenOptions::new().create(true).append(true).open(name).ok()
}

pub(crate) fn log(line: &str) {
    if let Some(mut f) = log_file() {
        let test = std::env::args().skip(1).find(|a| !a.starts_with('-')).unwrap_or_default();
        let _ = writeln!(f, "{line} test={test}");
    }
}

/// The outward normal of `surface` under `sense`, as an array, `None`
/// where it is not finite.
pub(crate) fn outward_of(
    surface: &geom::Surface<f64>,
    sense: bool,
) -> impl Fn(Point3<f64>) -> Option<[f64; 3]> + '_ {
    move |p| {
        let v = geom_brep::implicit_outward_normal(surface, sense, p).vec();
        let a = [v.x, v.y, v.z];
        a.iter().all(|c| c.is_finite()).then_some(a)
    }
}

pub(crate) fn surface_kind(surface: &geom::Surface<f64>) -> &'static str {
    use geom::Surface;
    match surface {
        Surface::Plane { .. } => "plane",
        Surface::Cylinder { .. } => "cyl",
        Surface::Cone { .. } => "cone",
        Surface::Sphere { .. } => "sphere",
        Surface::Torus { .. } => "torus",
        Surface::Spindle { .. } => "spindle",
        Surface::Nurbs(_) => "nurbs",
        Surface::Approx(_) => "approx",
    }
}

pub(crate) fn wound<T: Copy>(flip: bool, a: [T; 3]) -> [T; 3] {
    if flip { [a[0], a[2], a[1]] } else { a }
}

impl Splits {
    pub(crate) fn for_surface(surface: &geom::Surface<f64>) -> Self {
        let mut s = Self::default();
        if let geom::Surface::Cone { half_angle, .. } = *surface {
            s.develop = Some(half_angle.sin());
        }
        s
    }

    /// One line over every candidate inside the patch (four halves).
    pub(crate) fn log_offered(&self, surface: &'static str, delta: f64) {
        let inside: Vec<&SideSplit> = self.list.iter().filter(|c| c.halves.len() == 4).collect();
        if inside.is_empty() {
            return;
        }
        let refused = |c: &SideSplit| c.halves.iter().any(|h| h.verdict != cert::Verdict::Certified || h.unresolved || h.folded == Some(true));
        let worst = |c: &SideSplit| c.halves.iter().map(|h| h.bound).fold(0.0, f64::max);
        let apex: Vec<&&SideSplit> = inside.iter().filter(|c| c.apex).collect();
        log(&format!(
            "OFFER kind={}:{surface} n={} refused={} worst_d={:.4} apex={} apex_refused={} apex_worst_d={:.4} dev_gap_max_d={:.4e} dev_worst_d={:.4} dev_refused={}",
            inside[0].kind,
            inside.len(),
            inside.iter().filter(|c| refused(c)).count(),
            inside.iter().map(|c| worst(c)).fold(0.0, f64::max) / delta,
            apex.len(),
            apex.iter().filter(|c| refused(c)).count(),
            apex.iter().map(|c| worst(c)).fold(0.0, f64::max) / delta,
            inside.iter().map(|c| c.dev_gap).filter(|g| g.is_finite()).fold(0.0, f64::max) / delta,
            inside.iter().map(|c| c.dev_worst).fold(0.0, f64::max) / delta,
            inside.iter().filter(|c| c.dev_refused).count(),
        ));
        let count = |f: &dyn Fn(&Half) -> bool| inside.iter().filter(|c| c.halves.iter().any(|h| f(h))).count();
        log(&format!(
            "WHY kind={}:{surface} exceeded={} unres_cert={} unres_wind={} fold={} fold_parent={} unres_wind_parent_thin={}",
            inside[0].kind,
            count(&|h| h.verdict == cert::Verdict::Exceeded),
            count(&|h| matches!(h.verdict, cert::Verdict::Unresolved { .. })),
            count(&|h| h.unresolved),
            count(&|h| h.folded == Some(true)),
            count(&|h| h.folded_vs_parent),
            inside.iter().filter(|c| c.halves.iter().any(|h| h.unresolved) && c.halves.iter().all(|h| h.folded != Some(true))).count(),
        ));
    }
}
