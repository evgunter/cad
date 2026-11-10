//! **The anchor-fit refusal's radius lever, against the kernel's own
//! boundary.**
//!
//! `CornerReason::AnchorOutsideTrimmedExtent::radius_at_anchor` is the
//! radius of the fillet tangent to an overrun leg's carrier exactly at
//! its anchor and to the other carrier, on the reported candidate's
//! branch, the smaller over its overrun legs. These rows re-derive it
//! here from the carriers alone — centre `P + r·m`, `m` the anchor's
//! normal toward the turn, tangency to the other carrier linear in `r`,
//! each leg's solution assigned to the candidate whose side of the
//! carriers' axis its centre lies on — and replay every anchor-fit
//! entry of three grids (grid A's arc×arc corners, the line×arc grid,
//! and a straight×straight one) against it:
//!
//! - the payload is the re-derived bound, and it is the largest bound
//!   over the corner's candidates, so the setback-nearest pick never
//!   names a candidate with a smaller one;
//! - the bound is never above the FLIP — the largest radius at which
//!   this corner stops refusing on the anchor fit, bisected on the
//!   replay to adjacent floats — and equals it up to the band the fit
//!   gate decides on, except where the loop builds below it at the
//!   pair's other corner;
//! - the sentence's X, typed back, clears this corner's anchor fit, and
//!   so does every radius in a sweep below it ("X m or less");
//! - authoring 3608's X is 0.361629480311458.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::too_many_lines, clippy::similar_names)]

use crate::common::{arc_arc, corners, grid_a, line_arc, on_circle, tol};
use geom_core::{Point2, Tol};
use profile::{
    ArcSweep, CornerReason, CornerRefusal, FilletLeg, LegFit, PathError, ProfileLoop, Start,
};

type V = (f64, f64);
type Built = Result<ProfileLoop<f64>, PathError<f64>>;

fn sub(a: V, b: V) -> V {
    (a.0 - b.0, a.1 - b.1)
}
fn add(a: V, b: V) -> V {
    (a.0 + b.0, a.1 + b.1)
}
fn mul(a: V, s: f64) -> V {
    (a.0 * s, a.1 * s)
}
fn dot(a: V, b: V) -> f64 {
    a.0 * b.0 + a.1 * b.1
}
fn cross(a: V, b: V) -> f64 {
    a.0 * b.1 - a.1 * b.0
}
fn norm(a: V) -> f64 {
    dot(a, a).sqrt()
}
fn left(a: V) -> V {
    (-a.1, a.0)
}
fn p(q: Point2<f64>) -> V {
    (q.x, q.y)
}
fn ang(a: V) -> f64 {
    a.1.atan2(a.0)
}
const TAU: f64 = core::f64::consts::TAU;
fn swept(from: f64, to: f64, turn: f64) -> f64 {
    ((to - from) * turn).rem_euclid(TAU)
}
fn signed_swept(from: f64, to: f64, turn: f64) -> f64 {
    let x = (to - from) * turn;
    x - TAU * (x / TAU).round()
}

#[derive(Clone, Copy, Debug)]
enum Kind {
    Line { u: V },
    Circle { c: V, rr: f64, tau: f64 },
}

/// One leg's carrier and the anchor that pins it.
#[derive(Clone, Copy, Debug)]
struct Carrier {
    kind: Kind,
    anchor: V,
}

impl Carrier {
    fn dir_at(&self, q: V) -> V {
        match self.kind {
            Kind::Line { u } => u,
            Kind::Circle { c, rr, tau } => mul(left(sub(q, c)), tau / rr),
        }
    }
    fn is_line(&self) -> bool {
        matches!(self.kind, Kind::Line { .. })
    }
    /// The offset radius on the corner's branch (`None` on a line).
    fn rho(&self, sigma: f64, r: f64) -> Option<f64> {
        match self.kind {
            Kind::Line { .. } => None,
            Kind::Circle { rr, tau, .. } => Some(rr - sigma * tau * r),
        }
    }
    /// The fillet tangent to THIS carrier at its anchor and to `other`
    /// on the σ side, and its centre; `None` where it is no fillet.
    fn r_p(&self, other: &Self, sigma: f64) -> Option<(f64, V)> {
        let pa = self.anchor;
        let m = mul(left(self.dir_at(pa)), sigma);
        let r = match other.kind {
            Kind::Line { u } => {
                let n2 = left(u);
                sigma * dot(n2, sub(pa, other.anchor)) / (1.0 - sigma * dot(n2, m))
            }
            Kind::Circle { c, rr, tau } => {
                let w = sub(pa, c);
                (rr * rr - dot(w, w)) / (2.0 * (dot(w, m) + sigma * tau * rr))
            }
        };
        let ok = r.is_finite()
            && r > 0.0
            && self.rho(sigma, r).is_none_or(|x| x > 0.0)
            && other.rho(sigma, r).is_none_or(|x| x > 0.0);
        ok.then(|| (r, add(pa, mul(m, r))))
    }
    fn tangent_point(&self, o: V, sigma: f64, r: f64) -> V {
        match self.kind {
            Kind::Line { u } => sub(o, mul(left(u), sigma * r)),
            Kind::Circle { c, rr, .. } => {
                let s = sub(o, c);
                let rho = self.rho(sigma, r).unwrap();
                add(c, mul(s, rr / norm(s).copysign(rho)))
            }
        }
    }
    /// The signed setback of `t` from the corner `x` along the leg, and
    /// the leg's extent from `x` to its anchor.
    fn setback_len(&self, side: FilletLeg, t: V, x: V) -> (f64, f64) {
        match self.kind {
            Kind::Line { u } => {
                let sb = match side {
                    FilletLeg::Incoming => dot(sub(x, t), u),
                    FilletLeg::Outgoing => dot(sub(t, x), u),
                };
                (sb, norm(sub(x, self.anchor)))
            }
            Kind::Circle { c, rr, tau } => {
                let (at, ax, aa) = (ang(sub(t, c)), ang(sub(x, c)), ang(sub(self.anchor, c)));
                match side {
                    FilletLeg::Incoming => {
                        (rr * signed_swept(at, ax, tau), rr * swept(aa, ax, tau))
                    }
                    FilletLeg::Outgoing => {
                        (rr * signed_swept(ax, at, tau), rr * swept(ax, aa, tau))
                    }
                }
            }
        }
    }
}

/// The candidate centres at radius `r`.
fn centres(inc: &Carrier, out: &Carrier, sigma: f64, r: f64) -> Vec<V> {
    match (inc.kind, out.kind) {
        (Kind::Circle { c: c1, .. }, Kind::Circle { c: c2, .. }) => {
            let (r1, r2) = (
                inc.rho(sigma, r).unwrap().abs(),
                out.rho(sigma, r).unwrap().abs(),
            );
            let d = sub(c2, c1);
            let dd = dot(d, d);
            let k = (dd + r1 * r1 - r2 * r2) / (2.0 * dd);
            let h2 = r1 * r1 / dd - k * k;
            if h2 < 0.0 {
                return vec![];
            }
            let mid = add(c1, mul(d, k));
            let h = h2.sqrt();
            vec![add(mid, mul(left(d), h)), sub(mid, mul(left(d), h))]
        }
        (Kind::Line { u }, Kind::Circle { .. }) | (Kind::Circle { .. }, Kind::Line { u }) => {
            let (line, circ) = if inc.is_line() {
                (inc, out)
            } else {
                (out, inc)
            };
            let Kind::Circle { c, .. } = circ.kind else {
                unreachable!()
            };
            let rho = circ.rho(sigma, r).unwrap().abs();
            let q = add(line.anchor, mul(left(u), sigma * r));
            let w = sub(c, q);
            let along = dot(w, u);
            let off = cross(u, w);
            let h2 = rho * rho - off * off;
            if h2 < 0.0 {
                return vec![];
            }
            let h = h2.sqrt();
            vec![add(q, mul(u, along - h)), add(q, mul(u, along + h))]
        }
        (Kind::Line { u: u1 }, Kind::Line { u: u2 }) => {
            let (n1, n2) = (left(u1), left(u2));
            let base = add(inc.anchor, mul(n1, sigma * r));
            let s = (r - sigma * dot(n2, sub(base, out.anchor))) / (sigma * dot(n2, u1));
            vec![add(base, mul(u1, s))]
        }
    }
}

/// Which branch a centre is on: the side of the carriers' axis it lies
/// on, which a branch keeps as the radius moves.
fn branch_of(inc: &Carrier, out: &Carrier, o: V) -> usize {
    match (inc.kind, out.kind) {
        (Kind::Circle { c: c1, .. }, Kind::Circle { c: c2, .. }) => {
            usize::from(cross(sub(c2, c1), sub(o, c1)) <= 0.0)
        }
        (Kind::Line { u }, Kind::Circle { c, .. }) | (Kind::Circle { c, .. }, Kind::Line { u }) => {
            usize::from(dot(sub(o, c), u) >= 0.0)
        }
        _ => 0,
    }
}

/// The anchor-fit entry at `at` in a refusal, if there is one.
fn entry_at(res: &Built, at: Point2<f64>) -> Option<CornerRefusal<f64>> {
    let e = res.as_ref().err()?;
    corners(e)
        .iter()
        .find(|c| {
            matches!(c.reason, CornerReason::AnchorOutsideTrimmedExtent { .. })
                && c.at.distance(at) < 1e-9
        })
        .cloned()
}

/// The radius the sentence names for this one entry, as an author
/// reads it off: the number in "use a radius of X m or less".
fn rendered_x(entry: &CornerRefusal<f64>, radius: f64) -> Option<f64> {
    let said = PathError::NoCornerOfPair {
        radius,
        corners: vec![entry.clone()],
    }
    .to_string();
    let (_, tail) = said.split_once("use a radius of ")?;
    let (x, _) = tail.split_once(" m or less")?;
    Some(x.parse().expect("the sentence's X is a number"))
}

/// The largest radius below `r` at which the corner `at` does not
/// refuse on the anchor fit, bisected to adjacent floats from the
/// first of `seeds` that does not.
fn flip(build: &dyn Fn(f64) -> Built, at: Point2<f64>, r: f64, seeds: &[f64]) -> Option<f64> {
    let clear = |x: f64| entry_at(&build(x), at).is_none();
    let mut lo = *seeds.iter().find(|&&s| s > 0.0 && s < r && clear(s))?;
    let mut hi = r;
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if mid <= lo || mid >= hi {
            break;
        }
        if clear(mid) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    Some(lo)
}

/// What the rows count, over every anchor-fit entry of one grid.
#[derive(Default, Debug)]
struct Census {
    entries: usize,
    /// Entries whose bound is the flip up to the band.
    at_the_flip: usize,
    /// Entries whose loop builds below the bound at the other corner.
    below_the_flip: usize,
    /// The largest `(flip − bound) / ε` over `at_the_flip`.
    widest_band: f64,
    failures: Vec<String>,
}

impl Census {
    fn fail(&mut self, n: usize, what: String) {
        if self.failures.len() < 20 {
            self.failures.push(format!("authoring {n}: {what}"));
        }
    }

    fn assert_clean(&self, grid: &str) {
        assert!(
            self.failures.is_empty(),
            "{grid}: {} failures: {:#?}",
            self.failures.len(),
            self.failures
        );
    }
}

/// A bound and its flip agree up to the band when they are this many
/// ε apart or fewer. Measured at the default row: every bound at its
/// flip is under 10³ ε from it, and every loop that builds below it at
/// the other corner over 10⁶ ε.
const BAND_REACH: f64 = 1e4;

/// The radii below `x` an author reaching for "or less" might type.
fn below(x: f64) -> impl Iterator<Item = f64> {
    (0..40)
        .map(move |i| x * (1.0 - 1e-9) * 1e-3_f64.powf(f64::from(i) / 39.0))
        .chain([(x * 1e4).floor() / 1e4])
        .filter(move |&y| y > 0.0 && y < x)
}

/// One anchor-fit entry against the re-derived bound and the replay.
fn analyse(
    cen: &mut Census,
    n: usize,
    inc: Carrier,
    out: Carrier,
    entry: &CornerRefusal<f64>,
    r: f64,
    build: &dyn Fn(f64) -> Built,
) {
    cen.entries += 1;
    let CornerReason::AnchorOutsideTrimmedExtent {
        legs,
        radius_at_anchor,
    } = &entry.reason
    else {
        unreachable!("an anchor-fit entry")
    };
    let x = p(entry.at);
    let sigma = cross(inc.dir_at(x), out.dir_at(x)).signum();
    let Some(bound) = *radius_at_anchor else {
        return cen.fail(n, format!("no radius at the anchor: {legs:?}"));
    };
    let sides = [(inc, FilletLeg::Incoming), (out, FilletLeg::Outgoing)];
    // Every candidate at r: its branch, its legs' setbacks and extents.
    let cands: Vec<(usize, [(f64, f64); 2])> = centres(&inc, &out, sigma, r)
        .into_iter()
        .map(|o| {
            let legs = sides.map(|(c, s)| c.setback_len(s, c.tangent_point(o, sigma, r), x));
            (branch_of(&inc, &out, o), legs)
        })
        .collect();
    let matches = |got: &[(f64, f64); 2], want: &[LegFit<f64>; 2]| {
        got.iter().zip(want).all(|(&(sb, len), leg)| {
            (sb - leg.setback).abs() < 1e-9 * (1.0 + sb.abs())
                && (len - leg.available).abs() < 1e-9 * (1.0 + len.abs())
        })
    };
    let Some(reported) = cands
        .iter()
        .find(|(_, got)| matches(got, legs))
        .map(|c| c.0)
    else {
        return cen.fail(
            n,
            format!("no candidate carries the payload's legs {legs:?}"),
        );
    };
    // A candidate's bound: each leg it overruns reaches its anchor ON
    // this candidate's branch, at that leg's r_P.
    let r_ps = [inc.r_p(&out, sigma), out.r_p(&inc, sigma)];
    let bound_of = |k: usize| -> Option<f64> {
        let (_, got) = cands.iter().find(|c| c.0 == k)?;
        let mut b = f64::INFINITY;
        for (i, &(sb, len)) in got.iter().enumerate() {
            if sb > len {
                let (rv, o) = r_ps[i]?;
                if branch_of(&inc, &out, o) != k {
                    return None;
                }
                b = b.min(rv);
            }
        }
        b.is_finite().then_some(b)
    };
    let Some(derived) = bound_of(reported) else {
        return cen.fail(n, "the reported candidate has no bound here".into());
    };
    if (derived - bound).abs() > 1e-12 * bound {
        cen.fail(
            n,
            format!("payload {bound} against the re-derived {derived}"),
        );
    }
    let largest = cands
        .iter()
        .filter(|(_, got)| got.iter().all(|&(sb, _)| sb >= 0.0))
        .filter_map(|c| bound_of(c.0))
        .fold(f64::NEG_INFINITY, f64::max);
    if largest > derived {
        cen.fail(
            n,
            format!("another candidate's bound {largest} is larger than {derived}"),
        );
    }
    // The flip: never below the bound, and the bound's to the band
    // unless the loop builds below it at the other corner.
    let at = entry.at;
    match flip(build, at, r, &[bound * (1.0 - 1e-9), bound * 0.5, r * 1e-3]) {
        None => cen.fail(n, "no flip found below r".into()),
        Some(f) if f < bound => cen.fail(n, format!("bound {bound} above the flip {f}")),
        Some(f) => {
            let gap = (f - bound) / tol().eps();
            if gap <= BAND_REACH {
                cen.at_the_flip += 1;
                cen.widest_band = cen.widest_band.max(gap);
            } else {
                cen.below_the_flip += 1;
            }
        }
    }
    // The sentence's X, typed back, and every radius below it.
    let Some(typed) = rendered_x(entry, r) else {
        return cen.fail(n, "the sentence names no radius".into());
    };
    if typed >= bound {
        cen.fail(n, format!("X {typed} is not below the bound {bound}"));
    }
    for y in core::iter::once(typed).chain(below(bound)) {
        if entry_at(&build(y), at).is_some() {
            return cen.fail(n, format!("r = {y} below the bound {bound} refuses again"));
        }
    }
}

/// The straight×straight grid: the ray from the origin east, filleted,
/// arriving along heading `phi` at the anchor `l2` past the corner
/// `(l1, 0)`, closed.
fn line_line(l1: f64, phi: f64, l2: f64, r: f64) -> Built {
    let t = Tol::witness();
    let b = Point2::new(l1 + l2 * phi.cos(), l2 * phi.sin());
    profile::Open
        .at(Point2::new(0.0, 0.0))
        .toward(1.0, 0.0, t)?
        .fillet(r, t)?
        .toward(phi.cos(), phi.sin(), t)?
        .to(b, t)?
        .line_to(Start, t)
        .map(|c| c.loop_.into_loop())
}

fn anchor_fits(res: &Built) -> Vec<CornerRefusal<f64>> {
    res.as_ref()
        .err()
        .map(|e| {
            corners(e)
                .iter()
                .filter(|c| matches!(c.reason, CornerReason::AnchorOutsideTrimmedExtent { .. }))
                .cloned()
                .collect()
        })
        .unwrap_or_default()
}

/// Whether this run is the default ε row, where the counts are pinned.
fn default_row() -> bool {
    format!("{:e}", tol().eps()) == "1e-9"
}

#[test]
fn grid_a_bounds_are_the_anchor_fit_boundary() {
    let mut cen = Census::default();
    grid_a(|n, case, r, out| {
        let [
            a_in,
            r_in,
            tau_in,
            delta_in,
            a_out,
            r_out,
            tau_out,
            delta_out,
        ] = case;
        let c1 = (-r_in * a_in.cos(), -r_in * a_in.sin());
        let c2 = (-r_out * a_out.cos(), -r_out * a_out.sin());
        let head = on_circle(Point2::new(c1.0, c1.1), r_in, a_in - tau_in * delta_in);
        let next = on_circle(Point2::new(c2.0, c2.1), r_out, a_out + tau_out * delta_out);
        let inc = Carrier {
            kind: Kind::Circle {
                c: c1,
                rr: r_in,
                tau: tau_in,
            },
            anchor: p(head),
        };
        let outc = Carrier {
            kind: Kind::Circle {
                c: c2,
                rr: r_out,
                tau: tau_out,
            },
            anchor: p(next),
        };
        for e in anchor_fits(out) {
            analyse(&mut cen, n, inc, outc, &e, r, &|x| arc_arc(case, x));
        }
    });
    eprintln!("grid A: {cen:?}");
    cen.assert_clean("grid A");
    if default_row() {
        assert_eq!(
            (cen.entries, cen.at_the_flip, cen.below_the_flip),
            (3_185, 2_923, 262),
            "grid A: entries, bounds at the flip, and loops that build below it at the \
             other corner"
        );
    }
}

#[test]
fn line_arc_bounds_are_the_anchor_fit_boundary() {
    let mut cen = Census::default();
    let mut n = 0;
    for big_r in [2.0, 1.0, 0.5] {
        for sx in [0.2, 0.8, 1.4, 1.9] {
            for winding in [ArcSweep::Ccw, ArcSweep::Cw] {
                for a in [0.3, 1.0, 2.0, 2.9] {
                    for m in 1..=10 {
                        n += 1;
                        let r = 0.05 * f64::from(m) * big_r;
                        let build = |x: f64| line_arc(big_r, sx, winding, a, x);
                        let tau = if winding == ArcSweep::Ccw { 1.0 } else { -1.0 };
                        let inc = Carrier {
                            kind: Kind::Line { u: (1.0, 0.0) },
                            anchor: (sx * big_r / 2.0, 0.0),
                        };
                        let outc = Carrier {
                            kind: Kind::Circle {
                                c: (0.0, 0.0),
                                rr: big_r,
                                tau,
                            },
                            anchor: p(on_circle(Point2::new(0.0, 0.0), big_r, a)),
                        };
                        for e in anchor_fits(&build(r)) {
                            analyse(&mut cen, n, inc, outc, &e, r, &build);
                        }
                    }
                }
            }
        }
    }
    eprintln!("line×arc: {cen:?}");
    cen.assert_clean("line×arc");
    assert_eq!(cen.below_the_flip, 0, "every line×arc bound is its flip");
    if default_row() {
        assert_eq!(cen.entries, 246, "line×arc anchor-fit entries");
    }
}

#[test]
fn line_line_bounds_are_the_anchor_fit_boundary() {
    let mut cen = Census::default();
    let mut n = 0;
    for l1 in [0.5, 1.0, 2.0] {
        for l2 in [0.5, 1.0, 2.0] {
            for phi in [0.3, 0.8, 1.4, 2.0, 2.6, -0.3, -0.8, -1.4, -2.0, -2.6] {
                for m in 1..=12 {
                    n += 1;
                    let r = 0.25 * f64::from(m);
                    let build = |x: f64| line_line(l1, phi, l2, x);
                    let inc = Carrier {
                        kind: Kind::Line { u: (1.0, 0.0) },
                        anchor: (0.0, 0.0),
                    };
                    let outc = Carrier {
                        kind: Kind::Line {
                            u: (phi.cos(), phi.sin()),
                        },
                        anchor: (l1 + l2 * phi.cos(), l2 * phi.sin()),
                    };
                    for e in anchor_fits(&build(r)) {
                        analyse(&mut cen, n, inc, outc, &e, r, &build);
                    }
                }
            }
        }
    }
    eprintln!("line×line: {cen:?}");
    cen.assert_clean("line×line");
    assert_eq!(cen.below_the_flip, 0, "every straight bound is its flip");
    if default_row() {
        assert_eq!(cen.entries, 642, "straight×straight anchor-fit entries");
    }
}

/// **Authoring 3608**, the census's widest miss: r = 0.4 refuses at
/// the origin, where the setback excess was 0.0013 m and 0.0384 m of
/// radius had to go. The radius at the anchor is 0.361629480311458,
/// the sentence names it rounded down, and that radius clears the
/// corner's anchor fit while the authored one does not.
#[test]
fn authoring_3608_names_the_radius_that_ends_at_its_anchor() {
    let case = [0.0, 0.2, -1.0, 0.3, 0.8, 0.15, -1.0, 0.3];
    let refused = arc_arc(case, 0.4);
    let origin = Point2::new(0.0, 0.0);
    let entry =
        entry_at(&refused, origin).expect("r = 0.4 refuses on the anchor fit at the origin");
    let CornerReason::AnchorOutsideTrimmedExtent {
        radius_at_anchor: Some(x),
        ..
    } = entry.reason
    else {
        panic!("{entry:?}")
    };
    assert!((x - 0.361_629_480_311_458).abs() < 1e-15, "X = {x}");
    let said = refused.as_ref().unwrap_err().to_string();
    assert!(
        said.contains("or use a radius of 0.3616294803 m or less"),
        "{said}"
    );
    assert!(entry_at(&arc_arc(case, 0.361_629_480_3), origin).is_none());
}
