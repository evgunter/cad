//! **ROUND measurement M2: the closed-form "radius at the anchor"
//! against the fit boundary.**
//!
//! For an anchor-fit refusal at a corner, `r_P` on a leg is the radius
//! of the fillet circle tangent to that leg's carrier exactly AT its
//! anchor P (centre `O = P + r·m`, `m` the carrier's left normal at P
//! times the corner's turn side σ) and tangent to the other carrier on
//! the kernel's own branch (σ-side of a line; offset radius
//! `R − σ·τ·r` on a circle). Linear in `r` on every carrier pair. The
//! candidate a leg's solution belongs to is read off which side of the
//! carriers' axis its centre lies; a candidate's bound is the least of
//! its legs' `r_P`, and the corner's is the larger over its corner-side
//! candidates.
//!
//! Each entry's bound is compared with the flip point the kernel itself
//! has — the largest radius at which the corner no longer refuses on the
//! anchor fit, bisected on the replay to adjacent floats — and replayed
//! at the bound (exact, a relative 1e-12 below, and floored to 4 dp) to
//! count what an author following it gets.
//!
//! M3: below each entry's r_P (40 radii log-spaced from r_P·(1 − 1e-9)
//! to r_P·1e-3, and r_P floored to 4 dp and to 10 significant figures)
//! the corner never refuses on the anchor fit again, so "r_P or less"
//! holds wherever this corner is observed; on grid A, 52 entries have a
//! probe radius at which the loop builds at its OTHER derived corner,
//! which leaves this one unobserved there.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::too_many_lines, clippy::similar_names)]

use crate::common::{anchor_fit_entries, arc_arc, grid_a, line_arc, on_circle, past_the_corner};
use geom_core::{Point2, Tol};
use profile::{ArcSweep, FilletLeg, PathError, ProfileLoop, Start};

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
    /// The offset radius on the kernel's branch (`None` on a line).
    fn rho(&self, sigma: f64, r: f64) -> Option<f64> {
        match self.kind {
            Kind::Line { .. } => None,
            Kind::Circle { rr, tau, .. } => Some(rr - sigma * tau * r),
        }
    }
    /// `r_P`: the fillet tangent to THIS carrier at its anchor and to
    /// `other` on the σ branch.
    fn r_p(&self, other: &Carrier, sigma: f64) -> Option<(f64, V)> {
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
    /// Signed setback of `t` from corner `x` along the leg, and the
    /// leg's length from `x` to its anchor.
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

/// The candidate centres at radius `r`, each with its index (the side
/// of the carriers' axis it lies on — fixed along a branch).
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
            // O = A1 + σ r n1 + s u1 with σ n2·(O − A2) = r.
            let (n1, n2) = (left(u1), left(u2));
            let base = add(inc.anchor, mul(n1, sigma * r));
            let s = (r - sigma * dot(n2, sub(base, out.anchor))) / (sigma * dot(n2, u1));
            vec![add(base, mul(u1, s))]
        }
    }
}

fn index_of(inc: &Carrier, out: &Carrier, o: V) -> usize {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Builds,
    Past,
    SameAnchorFit,
    Other,
}

fn classify(res: &Built, at: Point2<f64>) -> Outcome {
    match res {
        Ok(_) => Outcome::Builds,
        Err(e) if past_the_corner(e).is_some() => Outcome::Past,
        Err(e) => {
            if anchor_fit_entries(e)
                .iter()
                .any(|(q, ..)| (q.x - at.x).abs() < 1e-9 && (q.y - at.y).abs() < 1e-9)
            {
                Outcome::SameAnchorFit
            } else {
                Outcome::Other
            }
        }
    }
}

/// The kernel's own flip point below `r`: the largest radius at which
/// the corner does not refuse on the anchor fit, bisected to adjacent
/// floats from `seeds` (the first satisfied one is the lower end).
fn flip(build: &dyn Fn(f64) -> Built, at: Point2<f64>, r: f64, seeds: &[f64]) -> Option<f64> {
    let sat = |x: f64| classify(&build(x), at) != Outcome::SameAnchorFit;
    let mut lo = *seeds.iter().find(|&&s| s > 0.0 && s < r && sat(s))?;
    let mut hi = r;
    // An upper end above the seed that is still refusing is all a
    // bisection needs; the refusing `r` is that end.
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if mid <= lo || mid >= hi {
            break;
        }
        if sat(mid) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    Some(lo)
}

#[derive(Default, Debug)]
struct Census {
    entries: usize,
    no_sigma: usize,
    candidate_mismatch: usize,
    no_bound: usize,
    no_bound_at: Vec<usize>,
    bound_above_flip: usize,
    both_overrun: Vec<usize>,
    hidden: Vec<usize>,
    retrip: Vec<(usize, f64, bool)>,
    bound_not_below_r: usize,
    at_exact: [usize; 4],
    at_eps: [usize; 4],
    at_4dp: [usize; 4],
    four_dp_nonpositive: usize,
    no_flip: usize,
    flip_rel: Vec<f64>,
    flip_rel_by_kind: [Vec<f64>; 3],
    pick_differs: Vec<usize>,
    simple_differs: Vec<usize>,
    ratio_equal: usize,
    ratio_rel: Vec<f64>,
    worst: Vec<(f64, usize, f64, f64)>,
    other_names: std::collections::BTreeMap<String, usize>,
    detail: Vec<String>,
    by_authoring: std::collections::BTreeMap<usize, (f64, f64)>,
    pinned: Vec<(usize, f64, f64, f64)>,
}

fn name(res: &Built) -> String {
    match res {
        Ok(_) => "Ok".into(),
        Err(PathError::Escalated { source }) => {
            format!("Escalated({})", source.predicate.unwrap_or("?"))
        }
        Err(e) => {
            let reasons: Vec<String> = crate::common::corners(e)
                .iter()
                .map(|c| {
                    let d = format!("{:?}", c.reason);
                    d.split([' ', '{', '(']).next().unwrap_or("").to_string()
                })
                .collect();
            format!("{:?}{reasons:?}", e.kind())
        }
    }
}

fn slot(o: Outcome) -> usize {
    match o {
        Outcome::Builds => 0,
        Outcome::Past => 1,
        Outcome::SameAnchorFit => 2,
        Outcome::Other => 3,
    }
}

#[allow(clippy::too_many_arguments)]
fn analyse(
    cen: &mut Census,
    n: usize,
    inc: Carrier,
    out: Carrier,
    entry: (Point2<f64>, FilletLeg, f64, f64),
    r: f64,
    build: &dyn Fn(f64) -> Built,
    pins: &[usize],
) {
    cen.entries += 1;
    let (at, side, setback, available) = entry;
    let x = p(at);
    let turn = cross(inc.dir_at(x), out.dir_at(x));
    if turn == 0.0 {
        cen.no_sigma += 1;
        return;
    }
    let sigma = turn.signum();
    // The reported candidate, reconstructed at r and matched on the
    // reported leg's setback.
    let legs = [(inc, FilletLeg::Incoming), (out, FilletLeg::Outgoing)];
    let cands: Vec<(usize, [f64; 2], [f64; 2])> = centres(&inc, &out, sigma, r)
        .into_iter()
        .map(|o| {
            let mut sb = [0.0; 2];
            let mut len = [0.0; 2];
            for (i, (c, s)) in legs.iter().enumerate() {
                let t = c.tangent_point(o, sigma, r);
                (sb[i], len[i]) = c.setback_len(*s, t, x);
            }
            (index_of(&inc, &out, o), sb, len)
        })
        .collect();
    let si = usize::from(side == FilletLeg::Outgoing);
    let reported = cands.iter().find(|(_, sb, len)| {
        (sb[si] - setback).abs() < 1e-9 * (1.0 + setback.abs())
            && (len[si] - available).abs() < 1e-9 * (1.0 + available.abs())
    });
    let Some(&(reported_idx, ..)) = reported else {
        cen.candidate_mismatch += 1;
        return;
    };
    let corner_side: Vec<usize> = cands
        .iter()
        .filter(|(_, sb, _)| sb[0] >= 0.0 && sb[1] >= 0.0)
        .map(|c| c.0)
        .collect();
    // Each leg's r_P and the candidate its solution belongs to.
    let rp_in = inc.r_p(&out, sigma);
    let rp_out = out.r_p(&inc, sigma);
    // A corner-side candidate's bound: every leg it overruns at r must
    // reach its anchor ON THIS candidate's branch, at that leg's r_P;
    // an overrun leg whose solution belongs to the other candidate (or
    // has none) leaves this candidate unfixable by reducing r.
    let rps = [rp_in, rp_out];
    let bound_of = |k: usize| -> Option<f64> {
        let (_, sb, len) = cands.iter().find(|c| c.0 == k)?;
        let mut b = f64::INFINITY;
        for i in 0..2 {
            if sb[i] - len[i] > 0.0 {
                let (rv, o) = rps[i]?;
                if index_of(&inc, &out, o) != k {
                    return None;
                }
                b = b.min(rv);
            }
        }
        b.is_finite().then_some(b)
    };
    let simple = rps
        .iter()
        .flatten()
        .map(|(rv, _)| *rv)
        .fold(f64::INFINITY, f64::min);
    let feasible: Vec<(usize, f64)> = corner_side
        .iter()
        .filter_map(|&k| bound_of(k).map(|b| (k, b)))
        .collect();
    let Some(&(best_k, bound)) = feasible.iter().max_by(|a, b| a.1.total_cmp(&b.1)) else {
        cen.no_bound += 1;
        cen.no_bound_at.push(n);
        return;
    };
    if best_k != reported_idx {
        cen.pick_differs.push(n);
    }
    if (simple - bound).abs() > 1e-12 * bound {
        cen.simple_differs.push(n);
    }
    if bound >= r {
        cen.bound_not_below_r += 1;
    }
    // C's straight-pair ratio against the reported leg's own r_P.
    let ratio = r * available / setback;
    let own = if si == 0 { rp_in } else { rp_out };
    if let Some((rv, _)) = own {
        let rel = (ratio - rv).abs() / rv;
        cen.ratio_rel.push(rel);
        if rel < 1e-12 {
            cen.ratio_equal += 1;
        }
    }
    // Replays at the bound.
    cen.at_exact[slot(classify(&build(bound), at))] += 1;
    let below = build(bound * (1.0 - 1e-12));
    let o = classify(&below, at);
    cen.at_eps[slot(o)] += 1;
    if o == Outcome::Other {
        *cen.other_names
            .entry(format!("1e-12 below: {}", name(&below)))
            .or_default() += 1;
    }
    let exact = build(bound);
    if classify(&exact, at) == Outcome::Other {
        *cen.other_names
            .entry(format!("exact: {}", name(&exact)))
            .or_default() += 1;
    }
    let four = (bound * 1e4).floor() / 1e4;
    if four > 0.0 {
        let res = build(four);
        let o = classify(&res, at);
        cen.at_4dp[slot(o)] += 1;
        if o == Outcome::Other || o == Outcome::Past {
            *cen.other_names
                .entry(format!("4dp: {}", name(&res)))
                .or_default() += 1;
        }
    } else {
        cen.four_dp_nonpositive += 1;
    }
    // M3: below r_P, does this corner ever refuse on the anchor fit again?
    let rsi = cands.iter().find(|c| c.0 == reported_idx).unwrap();
    if rsi.1[0] - rsi.2[0] > 0.0 && rsi.1[1] - rsi.2[1] > 0.0 {
        cen.both_overrun.push(n);
    }
    let sig10 = {
        let e = bound.log10().floor() - 9.0;
        let q = 10f64.powf(e);
        (bound / q).floor() * q
    };
    let mut probes: Vec<f64> = (0..40)
        .map(|i| bound * (1.0 - 1e-9) * 1e-3_f64.powf(f64::from(i) / 39.0))
        .collect();
    probes.push(four);
    probes.push(sig10);
    let mut tripped: Option<(f64, Option<usize>)> = None;
    let mut hidden = false;
    let other_corner = match (inc.kind, out.kind) {
        (Kind::Circle { c: c1, .. }, Kind::Circle { c: c2, .. }) => {
            let d = sub(c2, c1);
            let foot = add(c1, mul(d, dot(sub(x, c1), d) / dot(d, d)));
            Some(sub(mul(foot, 2.0), x))
        }
        _ => None,
    };
    for &rr in probes.iter().filter(|&&x| x > 0.0 && x < bound) {
        let res = build(rr);
        if let (Ok(lp), Some(oc)) = (&res, other_corner) {
            // Built at the OTHER derived corner: this one is not observed.
            let used_other = lp.segments().iter().any(|sg| match sg {
                profile::Segment::Arc(a) if a.radius == rr => {
                    norm(sub(p(a.centre), oc)) < norm(sub(p(a.centre), x))
                }
                _ => false,
            });
            hidden |= used_other;
        }
        if classify(&res, at) == Outcome::SameAnchorFit {
            // Which candidate the re-trip reports, matched on its setback.
            let e = res.as_ref().err().unwrap();
            let (_, sd, sb, av) = *anchor_fit_entries(e)
                .iter()
                .find(|(q, ..)| (q.x - at.x).abs() < 1e-9 && (q.y - at.y).abs() < 1e-9)
                .unwrap();
            let j = usize::from(sd == FilletLeg::Outgoing);
            let k = centres(&inc, &out, sigma, rr).into_iter().find_map(|o| {
                let (c, s) = legs[j];
                let (b, l) = c.setback_len(s, c.tangent_point(o, sigma, rr), x);
                ((b - sb).abs() < 1e-9 && (l - av).abs() < 1e-9).then(|| index_of(&inc, &out, o))
            });
            tripped = Some((rr / bound, k));
            break;
        }
    }
    if hidden {
        cen.hidden.push(n);
    }
    if let Some((frac, k)) = tripped {
        cen.retrip.push((n, frac, k == Some(best_k)));
    }
    // The kernel's own flip point.
    match flip(build, at, r, &[bound * (1.0 - 1e-9), bound * 0.5, r * 1e-3]) {
        Some(f) => {
            let rel = (bound - f).abs() / f;
            cen.flip_rel.push(rel);
            let kind = match (inc.is_line(), out.is_line()) {
                (true, true) => 0,
                (false, false) => 2,
                _ => 1,
            };
            cen.flip_rel_by_kind[kind].push(rel);
            cen.worst.push((rel, n, bound, f));
            if rel >= 1e-12 && bound > f {
                cen.bound_above_flip += 1;
            }
            let agg = cen.by_authoring.entry(n).or_insert((f64::NEG_INFINITY, f));
            agg.0 = agg.0.max(bound);
            if rel >= 1e-9 || bound >= r || best_k != reported_idx {
                cen.detail.push(format!(
                    "at-r {} below-flip {} used-reported-corner {:?} rX {:?} n {n} r {r} at ({:.4},{:.4}) side {side:?} sb {setback:.6} avail {available:.6} \
                     rp_in {:?} rp_out {:?} corner_side {corner_side:?} reported {reported_idx} \
                     cand {:?} bound {bound:.6} flip {f:.6} at-flip+ {}",
                    name(&build(r)),
                    name(&build(f * (1.0 - 1e-6))),
                    {
                        let rr = f * (1.0 - 1e-6);
                        build(rr).ok().map(|lp| {
                            let o = lp.segments().iter().find_map(|sg| match sg {
                                profile::Segment::Arc(a) if a.radius == rr => Some(p(a.centre)),
                                _ => None,
                            });
                            o.map(|o| {
                                // The other carrier crossing: X reflected across the carriers' axis.
                                let other = match (inc.kind, out.kind) {
                                    (Kind::Circle { c: c1, .. }, Kind::Circle { c: c2, .. }) => {
                                        let d = sub(c2, c1);
                                        let t = dot(sub(x, c1), d) / dot(d, d);
                                        let foot = add(c1, mul(d, t));
                                        sub(mul(foot, 2.0), x)
                                    }
                                    _ => x,
                                };
                                norm(sub(o, x)) <= norm(sub(o, other))
                            })
                        })
                    },
                    {
                        let at_x = |c: Carrier| Carrier { anchor: x, ..c };
                        let a = at_x(inc).r_p(&at_x(out), sigma).map(|(v, o)| (v, index_of(&inc, &out, o)));
                        let b = at_x(out).r_p(&at_x(inc), sigma).map(|(v, o)| (v, index_of(&inc, &out, o)));
                        (a, b)
                    },
                    at.x, at.y,
                    rp_in.map(|(v, o)| (v, index_of(&inc, &out, o))),
                    rp_out.map(|(v, o)| (v, index_of(&inc, &out, o))),
                    cands.iter().map(|(k, sb, len)| (*k, sb[0] - len[0], sb[1] - len[1])).collect::<Vec<_>>(),
                    name(&build(f * (1.0 + 1e-12)))
                ));
            }
            if pins.contains(&n) {
                cen.pinned.push((n, bound, f, simple));
                println!("  pinned authoring {n} corner {:?}", at);
            }
        }
        None => cen.no_flip += 1,
    }
}

/// The default ε row's pins (the bucket counts move with ε through the
/// reads past the corner; the r_P/flip agreement does not).
fn default_row() -> bool {
    format!("{:e}", crate::common::tol().eps()) == "1e-9"
}

fn agree(cen: &Census) -> usize {
    cen.flip_rel.iter().filter(|&&x| x < 1e-12).count()
}

fn pin(cen: &Census, want: Pins) {
    assert_eq!(
        cen.no_sigma + cen.candidate_mismatch + cen.no_bound,
        0,
        "every entry reconstructs"
    );
    assert_eq!(cen.no_flip, 0);
    assert_eq!(cen.bound_above_flip, 0, "r_P never exceeds the flip point");
    assert_eq!(cen.bound_not_below_r, 0);
    assert_eq!(
        cen.pick_differs.len(),
        0,
        "the nearest-setback pick is the larger-r_P pick"
    );
    assert_eq!(
        agree(cen),
        want.agree,
        "entries whose r_P is the flip point to 1e-12"
    );
    assert_eq!(cen.simple_differs.len(), want.simple_differs);
    assert_eq!(cen.ratio_equal, want.ratio_equal);
    assert_eq!(
        cen.both_overrun.len(),
        want.both_overrun,
        "both legs overrun at r"
    );
    if default_row() {
        assert!(
            cen.retrip.is_empty(),
            "M3: a radius below r_P re-trips: {:?}",
            cen.retrip
        );
        assert_eq!(
            cen.hidden.len(),
            want.hidden,
            "M3: probe radii built at the other corner"
        );
        assert_eq!(cen.at_exact, want.exact, "replayed at the exact r_P");
        assert_eq!(cen.at_eps, want.eps, "replayed 1e-12 below");
        assert_eq!(cen.at_4dp, want.four, "replayed at r_P floored to 4 dp");
    }
}

struct Pins {
    agree: usize,
    simple_differs: usize,
    ratio_equal: usize,
    exact: [usize; 4],
    eps: [usize; 4],
    four: [usize; 4],
    both_overrun: usize,
    hidden: usize,
}

fn report(name: &str, cen: &mut Census) {
    let bucket = |v: &[f64]| {
        let a = v.iter().filter(|&&x| x < 1e-12).count();
        let b = v.iter().filter(|&&x| (1e-12..1e-9).contains(&x)).count();
        let c = v.iter().filter(|&&x| (1e-9..1e-6).contains(&x)).count();
        let d = v.len() - a - b - c;
        format!("n={} <1e-12:{a} <1e-9:{b} <1e-6:{c} >=1e-6:{d}", v.len())
    };
    println!("== {name}");
    println!(
        "entries {} no_sigma {} candidate_mismatch {} no_bound {} bound>=r {}",
        cen.entries, cen.no_sigma, cen.candidate_mismatch, cen.no_bound, cen.bound_not_below_r
    );
    println!(
        "replay at bound [builds, past, same-anchor-fit, other]: exact {:?} 1e-12-below {:?} floor-4dp {:?} (4dp<=0: {})",
        cen.at_exact, cen.at_eps, cen.at_4dp, cen.four_dp_nonpositive
    );
    println!("flip found: {} none: {}", cen.flip_rel.len(), cen.no_flip);
    println!(
        "no bound (no positive solution on a feasible candidate): {:?}",
        cen.no_bound_at
    );
    println!("|bound-flip|/flip all: {}", bucket(&cen.flip_rel));
    for (k, label) in ["line×line", "mixed", "arc×arc"].iter().enumerate() {
        println!("  {label}: {}", bucket(&cen.flip_rel_by_kind[k]));
    }
    cen.worst.sort_by(|a, b| b.0.total_cmp(&a.0));
    for w in cen.worst.iter().take(8) {
        println!(
            "  worst: authoring {} rel {:.3e} bound {} flip {}",
            w.1, w.0, w.2, w.3
        );
    }
    println!(
        "pick differs (nearest-setback vs larger bound): {} {:?}",
        cen.pick_differs.len(),
        &cen.pick_differs[..cen.pick_differs.len().min(20)]
    );
    println!(
        "candidate-agnostic min differs from the bound: {} {:?}",
        cen.simple_differs.len(),
        &cen.simple_differs[..cen.simple_differs.len().min(20)]
    );
    println!(
        "C ratio r·available/setback == own r_P (<1e-12): {} of {}; {}",
        cen.ratio_equal,
        cen.ratio_rel.len(),
        bucket(&cen.ratio_rel)
    );
    println!(
        "M3 re-trips below r_P: {} {:?}",
        cen.retrip.len(),
        &cen.retrip[..cen.retrip.len().min(30)]
    );
    println!(
        "M3 entries with a probe radius built at the other corner (unobserved there): {}",
        cen.hidden.len()
    );
    println!(
        "M3 reported candidate overruns both legs at r: {} {:?}",
        cen.both_overrun.len(),
        &cen.both_overrun[..cen.both_overrun.len().min(30)]
    );
    for (k, v) in &cen.other_names {
        println!("  other: {k}: {v}");
    }
    let agg: Vec<f64> = cen
        .by_authoring
        .values()
        .map(|(b, f)| (b - f).abs() / f)
        .collect();
    println!(
        "authoring-level max-bound vs whole-loop flip: {}",
        bucket(&agg)
    );
    for (n, (b, f)) in &cen.by_authoring {
        if (b - f).abs() / f >= 1e-9 {
            println!("  authoring-level disagreement: {n} max-bound {b} flip {f}");
        }
    }
    if std::env::var("ROUND_PROBE_DETAIL").is_ok() {
        for d in &cen.detail {
            println!("  detail: {d}");
        }
    }
    for (n, b, f, s) in &cen.pinned {
        println!("  pinned authoring {n}: bound {b} flip {f} candidate-agnostic {s}");
    }
}

#[test]
fn m2_grid_a() {
    let mut cen = Census::default();
    let mut authorings = 0;
    grid_a(|n, case, r, out| {
        authorings += 1;
        let Err(err) = out else { return };
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
        for e in anchor_fit_entries(err) {
            analyse(
                &mut cen,
                n,
                inc,
                outc,
                e,
                r,
                &|x| arc_arc(case, x),
                &[3608, 4831, 10829],
            );
        }
    });
    assert_eq!(authorings, 18_144);
    report("grid A (arc×arc)", &mut cen);
    assert_eq!(cen.entries, 3_185);
    pin(
        &cen,
        Pins {
            agree: 2_923,
            simple_differs: 74,
            ratio_equal: 0,
            exact: [1_177, 504, 1_005, 499],
            eps: [18, 1_635, 0, 1_532],
            four: [2_337, 848, 0, 0],
            both_overrun: 506,
            hidden: 52,
        },
    );
}

#[test]
fn m2_line_arc_grid() {
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
                        let Err(err) = build(r) else { continue };
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
                        for e in anchor_fit_entries(&err) {
                            analyse(&mut cen, n, inc, outc, e, r, &build, &[]);
                        }
                    }
                }
            }
        }
    }
    report("line×arc grid (mixed)", &mut cen);
    assert_eq!(cen.entries, 246);
    pin(
        &cen,
        Pins {
            agree: 246,
            simple_differs: 27,
            ratio_equal: 0,
            exact: [222, 0, 0, 24],
            eps: [0, 57, 0, 189],
            four: [181, 1, 0, 64],
            both_overrun: 54,
            hidden: 0,
        },
    );
}

/// A straight×straight grid of the builder's ray form: the ray from the
/// origin east, filleted, arriving along heading `phi` at the anchor
/// `l2` past the corner `(l1, 0)`, closed.
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

#[test]
fn m2_line_line_grid() {
    let mut cen = Census::default();
    let mut n = 0;
    for l1 in [0.5, 1.0, 2.0] {
        for l2 in [0.5, 1.0, 2.0] {
            for phi in [0.3, 0.8, 1.4, 2.0, 2.6, -0.3, -0.8, -1.4, -2.0, -2.6] {
                for m in 1..=12 {
                    n += 1;
                    let r = 0.25 * f64::from(m);
                    let build = |x: f64| line_line(l1, phi, l2, x);
                    let Err(err) = build(r) else { continue };
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
                    for e in anchor_fit_entries(&err) {
                        analyse(&mut cen, n, inc, outc, e, r, &build, &[]);
                    }
                }
            }
        }
    }
    report("line×line grid", &mut cen);
    assert_eq!(cen.entries, 642);
    pin(
        &cen,
        Pins {
            agree: 642,
            simple_differs: 0,
            ratio_equal: 642,
            exact: [382, 38, 200, 22],
            eps: [0, 414, 0, 228],
            four: [642, 0, 0, 0],
            both_overrun: 474,
            hidden: 0,
        },
    );
}

/// The three census boundaries, replayed directly. 10829's largest
/// building radius belongs to its OTHER derived corner, (-0.4739,
/// -0.3931), whose r_P is that flip bitwise; the origin's own r_P is
/// 0.1332.
#[test]
fn m2_pinned_boundaries() {
    // The anchor fit is satisfied: no anchor-fit entry at all (a radius
    // this close to the boundary leaves a remnant the closer can refuse).
    let sat = |case: [f64; 8], r: f64| match arc_arc(case, r) {
        Ok(_) => true,
        Err(e) => anchor_fit_entries(&e).is_empty(),
    };
    let builds = |case: [f64; 8], r: f64| match arc_arc(case, r) {
        Ok(_) => true,
        Err(e) => past_the_corner(&e).is_some(),
    };
    let c3608 = [0.0, 0.2, -1.0, 0.3, 0.8, 0.15, -1.0, 0.3];
    assert!(sat(c3608, 0.361_629_48));
    assert!(!sat(c3608, 0.361_629_49));
    assert!(
        sat(c3608, 0.361_627_5),
        "the census comment's 0.3616275035 is inside, not at, the boundary"
    );
    let c4831 = [0.0, 0.2, 1.0, 0.3, 2.0, 0.5, -1.0, 0.3];
    assert!(sat(c4831, 0.132_122_180_911_66));
    assert!(!sat(c4831, 0.132_122_180_911_67));
    let c10829 = [0.0, 0.4, 1.0, 2.984_513_020_910_303_5, 1.6, 0.5, -1.0, 0.3];
    assert!(sat(c10829, 0.242_314_393_716_877));
    assert!(!sat(c10829, 0.242_314_393_716_878));
    for (name, case, lo, hi) in [
        ("3608", c3608, 0.3616, 0.4),
        ("4831", c4831, 0.1321, 0.35),
        ("10829", c10829, 0.2423, 0.25),
    ] {
        let (mut lo, mut hi) = (lo, hi);
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            if mid <= lo || mid >= hi {
                break;
            }
            if builds(case, mid) {
                lo = mid
            } else {
                hi = mid
            }
        }
        println!(
            "{name}: largest building radius {lo}; just above: {:?}",
            arc_arc(case, hi)
                .err()
                .map(|e| format!("{e:?}").chars().take(160).collect::<String>())
        );
    }
    // Floored to 4 dp, each builds.
    assert!(builds(c3608, 0.3616) && builds(c4831, 0.1321) && builds(c10829, 0.2423));
}
