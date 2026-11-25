//! The test oracles the conic doors' rows share — the doors'
//! rows, the subdivision's guard rows and `topo`'s clearance rows: a
//! random unit vector, the ellipse and torus fixtures, the TRUE signed
//! distance from a sphere, a wall or a torus, a function's crossings and
//! extremes along a parameter (each refined, so a pair of crossings
//! inside one sample cell is not missed), and a door's certified roots
//! matched against them.

use geom_core::{Point3, Vec3};

pub mod exact;

/// A uniformly drawn unit vector, away from the degenerate short ones,
/// each component drawn by `range(-1, 1)` (a fuzz harness's uniform
/// draw).
pub fn unit(mut range: impl FnMut(f64, f64) -> f64) -> Vec3<f64> {
    loop {
        let v = Vec3::new(range(-1.0, 1.0), range(-1.0, 1.0), range(-1.0, 1.0));
        if v.norm() > 0.2 && v.norm() < 1.0 {
            return v.normalize();
        }
    }
}

/// The ring torus `(center, axis, big, small)`, its axis normalized and
/// its `u_ref` any unit vector square to it — the one torus fixture the
/// conic rows build.
pub fn torus(center: Point3<f64>, axis: Vec3<f64>, big: f64, small: f64) -> geom::Surface<f64> {
    let axis = axis.normalize();
    let frame = geom::Frame::admitted(center, axis, axis.orthonormal_basis().0)
        .unwrap_or_else(|| geom::Frame::new(center, axis, axis.orthonormal_basis().0));
    geom::Surface::Torus {
        frame,
        major_radius: big,
        minor_radius: small,
    }
}

/// The true signed distance from a sphere, a cylinder wall or a torus
/// of the point whose offset from a given point is `from(anchor)`, read
/// from the surface's own STORED anchor (its centre or origin). A
/// caller far from the origin hands an offset it computes as
/// `(a − anchor) + …` from stored values a few metres apart, which is
/// exact, rather than a point it rounded at the scale of its
/// coordinates.
pub fn distance_from_anchor(
    s: &geom::Surface<f64>,
    from: impl Fn(Point3<f64>) -> Vec3<f64>,
) -> f64 {
    match *s {
        geom::Surface::Sphere { frame, radius, .. } => {
            let center = frame.origin();
            from(center).norm() - radius
        }
        geom::Surface::Cylinder { frame, radius, .. } => {
            let (origin, axis) = (frame.origin(), frame.axis());
            let q = from(origin);
            (q - axis * q.dot(axis)).norm() - radius
        }
        geom::Surface::Torus {
            frame,
            major_radius,
            minor_radius,
            ..
        } => {
            let (center, axis) = (frame.origin(), frame.axis());
            let q = from(center);
            let h = q.dot(axis);
            let rho = (q - axis * h).norm();
            (rho - major_radius).hypot(h) - minor_radius
        }
        _ => unreachable!("the oracle reads spheres, walls and tori"),
    }
}

/// The true signed distance of `p` from a sphere, a wall or a torus.
pub fn distance(s: &geom::Surface<f64>, p: Point3<f64>) -> f64 {
    distance_from_anchor(s, |anchor| p - anchor)
}

/// The ellipse `c + a·û cos θ + b·v̂ sin θ` on the unit normal `n`, its
/// `û` the part of `u` square to `n`.
pub fn ellipse(c: [f64; 3], n: [f64; 3], u: [f64; 3], a: f64, b: f64) -> geom::Curve3<f64> {
    let n = Vec3::from_array(n).normalize();
    let u = Vec3::from_array(u);
    geom::Curve3::Ellipse(geom::Ellipse::new(
        geom::Frame::new(Point3::from_array(c), n, (u - n * u.dot(n)).normalize()),
        a,
        b,
    ))
}

/// The samples [`crossings`] and [`extremes`] take per call.
const SAMPLES: u32 = 20_000;

/// `f`'s extremum in `[a, b]` by golden section, a minimum when `low`.
fn golden(f: &impl Fn(f64) -> f64, (mut a, mut b): (f64, f64), low: bool) -> f64 {
    let g = geom_core::GOLDEN_SECTION;
    for _ in 0..100 {
        let (x1, x2) = (b - g * (b - a), a + g * (b - a));
        if (f(x1) < f(x2)) == low {
            b = x2;
        } else {
            a = x1;
        }
    }
    geom::mid_param(a, b)
}

/// `f`'s root in `[a, b]`, whose ends `f` reads on opposite sides,
/// bisected to the bit.
fn bisect(f: &impl Fn(f64) -> f64, (mut a, mut b): (f64, f64)) -> f64 {
    for _ in 0..80 {
        let m = geom::mid_param(a, b);
        if (f(m) < 0.0) == (f(a) < 0.0) {
            a = m;
        } else {
            b = m;
        }
    }
    geom::mid_param(a, b)
}

/// The samples about sample `k` of `v` (its one neighbour at either end)
/// when they keep its sign and `|v|` is least at `k`, a tie going to the
/// earlier sample so one extremum is searched once: the bracket in which
/// `f`'s extremum near there may cross zero and come back.
fn trough(v: &[f64], k: usize) -> Option<(usize, usize)> {
    let (a, b) = (k.saturating_sub(1), (k + 1).min(v.len() - 1));
    let kept = |j: usize| (v[j] < 0.0) == (v[k] < 0.0);
    let before = a == k || (kept(a) && v[k].abs() < v[a].abs());
    let after = b == k || (kept(b) && v[k].abs() <= v[b].abs());
    (before && after).then_some((a, b))
}

/// A circle of radius `rho` through the point `depth` off a surface at
/// `on` along its unit `normal` there, tangent to it in the unit
/// direction `tangent`, its centre toward `n̂ cos b + (n̂ × t̂) sin b` from
/// that point, so `θ = 0` is that point.
pub fn graze(
    on: Point3<f64>,
    normal: Vec3<f64>,
    tangent: Vec3<f64>,
    (rho, depth, b): (f64, f64, f64),
) -> geom::Curve3<f64> {
    let toward = (normal * b.cos() + normal.cross(tangent) * b.sin()).normalize();
    geom::Curve3::circle(
        on + normal * depth - toward * rho,
        toward.cross(tangent).normalize(),
        rho,
        toward,
    )
}

/// The sign changes of `f` on `[t0, t1]`, each bisected to the bit. A
/// pair inside one sample cell is found too, the first and last cells
/// included: at every sampled local extremum of `|f|`, the extremum is
/// refined by golden section, and one that crosses zero is bisected on
/// both sides.
pub fn crossings(f: impl Fn(f64) -> f64, t0: f64, t1: f64) -> Vec<f64> {
    let at = |k: usize| t0 + (t1 - t0) * k as f64 / f64::from(SAMPLES);
    let v: Vec<f64> = (0..=SAMPLES as usize).map(|k| f(at(k))).collect();
    let mut out: Vec<f64> = (0..SAMPLES as usize)
        .filter(|&k| (v[k] < 0.0) != (v[k + 1] < 0.0))
        .map(|k| bisect(&f, (at(k), at(k + 1))))
        .collect();
    for k in 0..v.len() {
        let Some((a, b)) = trough(&v, k) else {
            continue;
        };
        let x = golden(&f, (at(a), at(b)), v[k] > 0.0);
        if (f(x) < 0.0) != (v[k] < 0.0) {
            out.push(bisect(&f, (at(a), x)));
            out.push(bisect(&f, (x, at(b))));
        }
    }
    out.sort_by(f64::total_cmp);
    out
}

/// `f`'s least and greatest values on `[t0, t1]`, every sampled local
/// extremum refined by golden section, the first and last cells' included.
pub fn extremes(f: impl Fn(f64) -> f64, t0: f64, t1: f64) -> (f64, f64) {
    let at = |k: usize| t0 + (t1 - t0) * k as f64 / f64::from(SAMPLES);
    let v: Vec<f64> = (0..=SAMPLES as usize).map(|k| f(at(k))).collect();
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for (k, &q) in v.iter().enumerate() {
        let (a, b) = (k.saturating_sub(1), (k + 1).min(v.len() - 1));
        lo = lo.min(q);
        hi = hi.max(q);
        if q <= v[a] && q <= v[b] {
            lo = lo.min(f(golden(&f, (at(a), at(b)), true)));
        }
        if q >= v[a] && q >= v[b] {
            hi = hi.max(f(golden(&f, (at(a), at(b)), false)));
        }
    }
    (lo, hi)
}

/// **The truth on a partial arc** ([`arc_truth`]).
#[derive(Debug)]
pub struct ArcTruth {
    /// The residual is within `2Kε` at an end: a meeting or a graze in
    /// band of an end is the end's to read, and a row sets the arc aside.
    pub at_an_end: bool,
    /// The least `|residual|` at an extreme of it inside the arc or at
    /// an end: what a band classifies the arc's grazes by (a constant
    /// residual's own).
    pub least: f64,
    /// The true crossings on the arc, at any turn of it.
    pub roots: Vec<f64>,
}

/// **The truth on the arc `[t0, t1]` alone**: `residual` the surface's
/// implicit residual along the carrier (what the band classifies), `roots`
/// the true crossings round the turn (`exact::truth`), `k_eps` the band's
/// escalation. A door reading local windows answers an arc whose own
/// extremes are clear of the band, whatever the rest of the turn holds.
pub fn arc_truth(
    residual: impl Fn(f64) -> f64,
    (t0, t1): (f64, f64),
    roots: &[f64],
    k_eps: f64,
) -> ArcTruth {
    // A conic's residual is a trigonometric polynomial of degree at most
    // four: its extremes' basins are far wider than these cells.
    const CELLS: usize = 4096;
    let at = |k: usize| t0 + (t1 - t0) * k as f64 / CELLS as f64;
    let v: Vec<f64> = (0..=CELLS).map(|k| residual(at(k))).collect();
    // Each sampled extreme of the residual itself (strict on the left, so
    // a flat stretch is none). One whose cell could swing it near the
    // band is refined: a pair of crossings inside one cell is a trough
    // near zero, which no sample reads. One past it is past it. A residual
    // flat within the band has a near extreme in every cell; the nearest
    // sixty-four are read, the stretch one-signed between them.
    let mut near: Vec<(f64, usize, bool)> = Vec::new();
    let mut least = residual(t0).abs().min(residual(t1).abs());
    for k in 1..CELLS {
        let swing = (v[k - 1] - v[k]).abs() + (v[k + 1] - v[k]).abs();
        let low = v[k] < v[k - 1] && v[k] <= v[k + 1];
        let high = v[k] > v[k - 1] && v[k] >= v[k + 1];
        if !(low || high) {
            continue;
        }
        if v[k].abs() < 4.0 * k_eps + 2.0 * swing {
            near.push((v[k].abs(), k, low));
        } else {
            least = least.min(v[k].abs());
        }
    }
    near.sort_by(|a, b| a.0.total_cmp(&b.0));
    for &(_, k, low) in near.iter().take(64) {
        let x = golden(&residual, (at(k - 1), at(k + 1)), low);
        least = least.min(residual(x).abs());
    }
    ArcTruth {
        at_an_end: residual(t0).abs() < 2.0 * k_eps || residual(t1).abs() < 2.0 * k_eps,
        least,
        roots: roots
            .iter()
            .copied()
            .filter(|&u| on_arc((t0, t1), u))
            .collect(),
    }
}

/// Whether `t` lies on the arc `[t0, t1]`, at any turn of it.
pub fn on_arc((t0, t1): (f64, f64), t: f64) -> bool {
    let mid = geom::mid_param(t0, t1);
    (geom_core::period::Period::angle().near(t, mid) - mid).abs() <= (t1 - t0) / 2.0
}

/// **Whether a door's roots hold the arc's truth** ([`arc_truth`]):
/// `None` where the row sets the arc aside, `Some(false)` where an arc
/// with an extreme inside `band` (its in-band reach) is answered or the
/// roots the door places on the arc are not the truth's in number.
/// `found` is the door's roots, none for a `Miss`; roots it reads off the
/// arc, outside every window it dropped there, are not judged here.
pub fn holds_on_arc(found: &[f64], truth: &ArcTruth, arc: (f64, f64), band: f64) -> Option<bool> {
    if truth.at_an_end {
        return None;
    }
    Some(
        truth.least >= band
            && found.iter().filter(|&&t| on_arc(arc, t)).count() == truth.roots.len(),
    )
}

/// **What `cross` answered, as the safety rows judge a door's reading**
/// ([`cross_reading`]).
#[derive(Debug)]
pub enum CrossReading {
    /// A refusal, or a decided touch: the root doors' `Unsettled` (a
    /// window, a tangency), which the rows permit wherever they permit
    /// one.
    Refused,
    /// The pose's `Lies`: the doors' `On`.
    Lies,
    /// Its meetings' parameters on the span (an end's at that end, and a
    /// `Through` pose's apex where the span holds it), none for `Apart`:
    /// the doors' certified roots, or their `Miss`.
    Roots(Vec<f64>),
}

impl CrossReading {
    /// The reading in the root doors' shape, for a row's judge of a door
    /// answer: a refusal is `Unsettled::Tangency` (the doors' limit, which
    /// every safety row permits), `Lies` is `On`, the roots `Miss` or a
    /// certified count (a count past the doors' room a `Meter` limit).
    #[must_use]
    pub fn as_door(
        &self,
    ) -> Result<super::circle_roots::CircleRoots<f64>, crate::roots::Unsettled> {
        use crate::roots::{OnCarrier, Roots, Unsettled};
        match self {
            Self::Refused => Err(Unsettled::Tangency),
            Self::Lies => Ok(Roots::On(OnCarrier)),
            Self::Roots(roots) => Roots::of(roots).ok_or(Unsettled::Meter {
                row: "cross_reading_count",
            }),
        }
    }
}

/// [`CrossReading`] of `cross`'s answer for `carrier` over `span` against
/// `surface` at `band`: the production reading, beside a door row's
/// wrapper leg.
pub fn cross_reading(
    carrier: &geom::Curve3<f64>,
    span: (f64, f64),
    surface: &geom::Surface<f64>,
    band: geom_core::Band,
) -> CrossReading {
    use super::{Crossings, MeetingKind, OnSpan, Pose};
    use crate::curve_pair::SpanEnd;
    let on = crate::curve_pair::Span::new(carrier, span);
    match super::cross(&on, surface, band) {
        Err(_) => CrossReading::Refused,
        Ok(Crossings {
            pose: Pose::Lies, ..
        }) => CrossReading::Lies,
        Ok(Crossings { pose, meetings }) => {
            if meetings
                .iter()
                .any(|m| matches!(m.kind, MeetingKind::Touch { .. }))
            {
                return CrossReading::Refused;
            }
            let mut roots: Vec<f64> = meetings
                .iter()
                .map(|m| match m.on_span {
                    OnSpan::Inside { t } => t,
                    OnSpan::End(SpanEnd::Start) => span.0,
                    OnSpan::End(SpanEnd::Stop) => span.1,
                })
                .collect();
            if let Pose::Through(apex) = pose
                && on_arc(span, apex.at)
            {
                roots.insert(0, apex.at);
            }
            CrossReading::Roots(roots)
        }
    }
}

/// The crossings of the true distance from `s` along `e` on `[t0, t1]`.
pub fn oracle(e: &geom::Curve3<f64>, s: &geom::Surface<f64>, t0: f64, t1: f64) -> Vec<f64> {
    crossings(|t| distance(s, e.eval(t)), t0, t1)
}

/// `door`'s answer on `[t0, t0 + 1]` is certified roots matching the
/// oracle in number and place, every one on the surface and within `π`
/// of the arc's midpoint.
#[allow(clippy::panic)]
pub fn assert_matches_oracle(
    label: &str,
    (e, s): (&geom::Curve3<f64>, &geom::Surface<f64>),
    t0: f64,
    want: usize,
    door: impl Fn(
        &geom::Curve3<f64>,
        &geom::Surface<f64>,
        f64,
        f64,
    ) -> Result<super::circle_roots::CircleRoots<f64>, crate::roots::Unsettled>,
) {
    use core::f64::consts::PI;
    let t1 = t0 + 1.0;
    let Ok(super::circle_roots::CircleRoots::Certified(found)) = door(e, s, t0, t1) else {
        panic!("{label}: certified roots, got {:?}", door(e, s, t0, t1));
    };
    let (count, thetas) = (found.count(), found.roots());
    assert_eq!(count, want, "{label}: the certified count");
    let mid = geom::mid_param(t0, t1);
    let mut got = thetas[..count].to_vec();
    for &t in &got {
        assert!((t - mid).abs() <= PI, "{label}: {t} within π of {mid}");
        let off = distance(s, e.eval(t)).abs();
        assert!(off < 1e-12, "{label}: root {t} lies {off} off the surface");
    }
    got.sort_by(f64::total_cmp);
    let truth = oracle(e, s, mid - PI, mid + PI);
    assert_eq!(got.len(), truth.len(), "{label}: {got:?} vs {truth:?}");
    for (a, b) in got.iter().zip(&truth) {
        assert!(
            (a - b).abs() < 1e-9,
            "{label}: root {a} vs the oracle's {b}"
        );
    }
}

/// **The first and last sample cells are searched like the rest.**
/// `cos(t − s) − cos w` crosses zero at `s ± w` and peaks at `s`; with
/// the pair a fifth of a cell wide and `s` inside the first or the last
/// cell, no sample sees the pair's sign, so only the refined extremum
/// finds it. On an arc and on a whole turn, at three places in each end
/// cell: both crossings, to the root's own rounding, and the peak `1`
/// to a few ulps, where an unrefined end cell reads it `1e-10` low.
#[test]
fn the_end_cells_are_searched() {
    use core::f64::consts::PI;
    for (t0, t1) in [(0.0, 1.0), (-PI, PI), (2.5, 2.5 + 2.0 * PI)] {
        let cell = (t1 - t0) / f64::from(SAMPLES);
        let w = 0.1 * cell;
        for frac in [0.3, 0.5, 0.7] {
            for s in [t0 + frac * cell, t1 - frac * cell] {
                let label = format!("[{t0}, {t1}], the pair about {s}");
                let got = crossings(|t| (t - s).cos() - w.cos(), t0, t1);
                assert_eq!(got.len(), 2, "{label}: {got:?}");
                for (g, want) in got.iter().zip([s - w, s + w]) {
                    assert!(
                        (g - want).abs() < 1e-9,
                        "{label}: a crossing {g} for {want}"
                    );
                }
                let (_, hi) = extremes(|t| (t - s).cos(), t0, t1);
                assert!(hi >= 1.0 - 1e-15, "{label}: the peak reads {hi}");
                let (lo, _) = extremes(|t| -(t - s).cos(), t0, t1);
                assert!(lo <= -1.0 + 1e-15, "{label}: the trough reads {lo}");
            }
        }
    }
}
