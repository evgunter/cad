//! **`∇φ`'s zero read by the Krawczyk operator** (C5's critical points,
//! C2's node end): `φ = f∘S` for a NURBS wall `S` and an implicit
//! partner's form `f`, its gradient and Hessian enclosed over a box of
//! the wall's chart through the wall's jet. With `m` the box's middle
//! and `Y` the inverse of the Hessian's midpoint,
//! `K = m − Y·∇φ(m) + (I − Y·∇²φ(X))·(X − m)` holds every zero of `∇φ`
//! in `X`, and where `K` lies strictly inside `X` it holds one: the
//! zero exists, and every matrix in `∇²φ(X)` is invertible, so it is the
//! only one.
//!
//! The regularity sweep decides its critical points by it
//! (`pair_section::spline_implicit::sweep::critical`), and limb 3 reads a
//! chain's end box by it ([`holds_node`]): a chain ending at a node ends
//! in a box over which the claim is the node's own reading.

use geom::NurbsSurface;
use geom_core::interval::certification::Certification;
use geom_core::k_stats::{Magnitude, decide_magnitude_reported};
use geom_core::{Band, Bounds, Interval, Margin, Vec3};

use super::exhaust::UvRect;
use super::residual::ImplicitForm;

/// The most times a box `K` does not fit is cut to `K ∩ X` and read
/// again (D9, a fixed rule).
pub(crate) const KRAWCZYK_ROUNDS: u32 = 12;

/// The wall's chart, read in interval arithmetic: its jet over a box.
pub(crate) struct Jets {
    pub(crate) net: NurbsSurface<Interval>,
}

impl Jets {
    /// The wall's net, its knots, points and weights as point enclosures;
    /// `None` where it does not lift.
    pub(crate) fn of(net: &NurbsSurface<f64>) -> Option<Self> {
        let ku = net.knot_values_u().map(Interval::point).ok()?;
        let kv = net.knot_values_v().map(Interval::point).ok()?;
        let control = net
            .control()
            .iter()
            .map(|p| p.map(Interval::point))
            .collect();
        let weights = net.weights().iter().map(|&w| Interval::point(w)).collect();
        NurbsSurface::new(ku, kv, control, weights)
            .ok()
            .map(|net| Self { net })
    }
}

/// `∇φ` and `∇²φ` over the box `(u, v)`, and `φ` there as an enclosure
/// with a ceiling in metres on its distance from the partner.
pub(crate) struct Read {
    pub(crate) gradient: [Interval; 2],
    pub(crate) hessian: [[Interval; 2]; 2],
    pub(crate) value: Interval,
    pub(crate) ceiling: f64,
}

/// Reads `φ` of `phi` over the box `(u, v)` of the wall's chart.
pub(crate) fn read(jets: &Jets, phi: &ImplicitForm, u: Interval, v: Interval) -> Read {
    let jet = jets.net.ders(u, v);
    let p = Vec3::new(jet.point.x, jet.point.y, jet.point.z);
    let q = phi.framed(crate::ssi::enclose::Box3 {
        x: p.x,
        y: p.y,
        z: p.z,
    });
    let value = phi.value(q);
    let ceiling = phi.ceiling_metres(value.lo().abs().max(value.hi().abs()), q);
    let (g, h) = (phi.frame_gradient(q), phi.frame_hessian(q));
    // Every vector in the frame `g` and `h` are written in.
    let framed = |d: Vec3<Interval>| phi.frame_vector(d);
    let (su, sv) = (framed(jet.du), framed(jet.dv));
    let (suu, suv, svv) = (framed(jet.duu), framed(jet.duv), framed(jet.dvv));
    let quad = |a: Vec3<Interval>, b: Vec3<Interval>| {
        let a = [a.x, a.y, a.z];
        let b = [b.x, b.y, b.z];
        let mut acc = Interval::point(0.0);
        for (i, row) in h.iter().enumerate() {
            for (j, e) in row.iter().enumerate() {
                acc = acc + a[i] * *e * b[j];
            }
        }
        acc
    };
    let huv = quad(su, sv) + g.dot(suv);
    Read {
        gradient: [g.dot(su), g.dot(sv)],
        hessian: [
            [quad(su, su) + g.dot(suu), huv],
            [huv, quad(sv, sv) + g.dot(svv)],
        ],
        value,
        ceiling,
    }
}

/// The inverse of a 2×2 matrix, `None` where it is singular or not
/// finite.
fn inverse2(m: [[f64; 2]; 2]) -> Option<[[f64; 2]; 2]> {
    let det = m[0][0] * m[1][1] - m[0][1] * m[1][0];
    if !det.is_finite() || det == 0.0 {
        return None;
    }
    Some([
        [m[1][1] / det, -m[0][1] / det],
        [-m[1][0] / det, m[0][0] / det],
    ])
}

/// `[lo, hi]` as an interval, or `None` where it is inverted or not
/// finite.
pub(crate) fn range((lo, hi): (f64, f64)) -> Option<Interval> {
    (lo <= hi && lo.is_finite() && hi.is_finite()).then(|| Interval::from_bounds(lo, hi))
}

/// The middle of an interval.
pub(crate) fn mid(x: Interval) -> f64 {
    geom::mid_param(x.lo(), x.hi())
}

/// `φ`'s Hessian over `x`, the hull of its readings over a
/// `splits × splits` grid of `x`'s parts ([`krawczyk_split`]).
fn hessian_over(jets: &Jets, phi: &ImplicitForm, x: [Interval; 2], splits: u32) -> Read {
    let whole = read(jets, phi, x[0], x[1]);
    if splits <= 1 {
        return whole;
    }
    let part = |i: u32, k: usize| {
        let (lo, hi) = (x[k].lo(), x[k].hi());
        let at = |j: u32| lo + (hi - lo) * f64::from(j) / f64::from(splits);
        Interval::from_bounds(at(i), if i + 1 == splits { hi } else { at(i + 1) })
    };
    let mut hessian: Option<[[Interval; 2]; 2]> = None;
    for i in 0..splits {
        for j in 0..splits {
            let h = read(jets, phi, part(i, 0), part(j, 1)).hessian;
            hessian = Some(match hessian {
                None => h,
                Some(acc) => core::array::from_fn(|a| {
                    core::array::from_fn(|b| Interval::hull(acc[a][b], h[a][b]))
                }),
            });
        }
    }
    Read {
        hessian: hessian.unwrap_or(whole.hessian),
        ..whole
    }
}

/// **The Krawczyk box of `x`**, `K ∩ x`, and whether `K` lies strictly
/// inside `x` (module docs). `None` where a reading does not certify or
/// the Hessian's midpoint is singular.
pub(crate) fn krawczyk(
    jets: &Jets,
    phi: &ImplicitForm,
    x: [Interval; 2],
) -> Option<([Interval; 2], bool)> {
    krawczyk_split(jets, phi, x, 1)
}

/// [`krawczyk`] with `∇²φ` over `x` read as the hull of its readings
/// over a `splits × splits` grid of `x`'s parts: the wall's jet over a
/// box widens past its true range faster than the box does, and a wide
/// node box reads it no wider than its parts do.
fn krawczyk_split(
    jets: &Jets,
    phi: &ImplicitForm,
    x: [Interval; 2],
    splits: u32,
) -> Option<([Interval; 2], bool)> {
    let m = [mid(x[0]), mid(x[1])];
    let at = read(jets, phi, Interval::point(m[0]), Interval::point(m[1]));
    let over = hessian_over(jets, phi, x, splits);
    let certified = |i: &Interval| i.is_certified();
    if !(at.gradient.iter().all(certified) && over.hessian.iter().flatten().all(certified)) {
        return None;
    }
    let y = inverse2(over.hessian.map(|row| row.map(mid)))?;
    let mut k = [Interval::point(0.0); 2];
    for i in 0..2 {
        let mut acc = Interval::point(m[i]);
        for j in 0..2 {
            acc = acc - Interval::point(y[i][j]) * at.gradient[j];
            let mut c = Interval::point(if i == j { 1.0 } else { 0.0 });
            for (l, row) in over.hessian.iter().enumerate() {
                c = c - Interval::point(y[i][l]) * row[j];
            }
            acc = acc + c * (x[j] - Interval::point(m[j]));
        }
        k[i] = acc;
    }
    if !k.iter().all(certified) {
        return None;
    }
    let inside = (0..2).all(|i| x[i].lo() < k[i].lo() && k[i].hi() < x[i].hi());
    let meet = [
        range((x[0].lo().max(k[0].lo()), x[0].hi().min(k[0].hi())))?,
        range((x[1].lo().max(k[1].lo()), x[1].hi().min(k[1].hi())))?,
    ];
    Some((meet, inside))
}

/// What a critical point decided Zero is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    /// A definite Hessian: the surfaces touch at one point.
    Touch,
    /// An indefinite Hessian: the section's arcs cross at the point.
    Node,
}

/// **A critical point of `φ` read in a box and decided Zero**
/// ([`decide`]).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Decided {
    /// The first box the operator proved holds the box's one zero of
    /// `∇φ` (`K` strictly inside it).
    pub(crate) first: [Interval; 2],
    /// The box `K` contracted to, which holds the zero.
    pub(crate) last: [Interval; 2],
    /// A touch or a node, by the sign of the Hessian's determinant over
    /// `last`.
    pub(crate) kind: Kind,
    /// `φ`'s Hessian over `last`.
    pub(crate) hessian: [[Interval; 2]; 2],
    /// `φ`'s value's margin over `last`, decided Zero.
    pub(crate) margin: geom_core::MarginDiag,
}

/// **The one zero of `∇φ` in `x`, decided** (C5's critical point, C2's
/// node end): the box `K` is cut to `K ∩ x` and read again, at most
/// [`KRAWCZYK_ROUNDS`] times, each cut keeping every zero of `x`, so a
/// zero proved alone in a cut box (`K` strictly inside it) is `x`'s only
/// one. Over the box it contracts to, the Hessian's determinant decides
/// a touch (positive) or a node (negative), and `φ` in metres is
/// decided Zero (`spline_critical_value`).
///
/// `None` wherever any of that is not proved: no zero, a zero not
/// proved alone, a determinant or a value that straddles or does not
/// certify, or a value past the band.
pub(crate) fn decide(jets: &Jets, phi: &ImplicitForm, x: UvRect, band: Band) -> Option<Decided> {
    decide_split(jets, phi, x, 1, band)
}

/// [`decide`], each round's `∇²φ` read over a `splits × splits` grid
/// of the box ([`krawczyk_split`]).
fn decide_split(
    jets: &Jets,
    phi: &ImplicitForm,
    x: UvRect,
    splits: u32,
    band: Band,
) -> Option<Decided> {
    let mut b = [range(x.u)?, range(x.v)?];
    let mut first: Option<[Interval; 2]> = None;
    for _ in 0..KRAWCZYK_ROUNDS {
        let (next, inside) = krawczyk_split(jets, phi, b, splits)?;
        let narrower = next[0].hi() - next[0].lo() < b[0].hi() - b[0].lo()
            || next[1].hi() - next[1].lo() < b[1].hi() - b[1].lo();
        if inside && first.is_none() {
            first = Some(b);
        }
        b = next;
        if !narrower {
            break;
        }
    }
    let first = first?;
    let at = read(jets, phi, b[0], b[1]);
    let det = at.hessian[0][0] * at.hessian[1][1] - at.hessian[0][1] * at.hessian[1][0];
    if !det.is_certified() || !at.value.is_certified() {
        return None;
    }
    let kind = if det.lo() > 0.0 {
        Kind::Touch
    } else if det.hi() < 0.0 {
        Kind::Node
    } else {
        return None;
    };
    match decide_magnitude_reported("spline_critical_value", Margin::of(at.ceiling), band) {
        Ok((Magnitude::Zero, margin)) => Some(Decided {
            first,
            last: b,
            kind,
            hessian: at.hessian,
            margin,
        }),
        Ok((Magnitude::Positive, _)) | Err(_) => None,
    }
}

/// How many of the band's escalations, through each axis's chart units
/// per metre, the box about a chain's end reaches that the node must
/// be read in for the end to be at it (D9): the carrier's end within
/// the band of the node, read on the box's own scale.
pub(crate) const NODE_END_SEED: f64 = 4.0;

/// How many parts along each axis the node box's `∇²φ` is read over
/// (D9, [`krawczyk_split`]): a node box may reach past the box the
/// march entered, where one reading of the jet over the whole box is
/// already too wide for the operator to contract.
pub(crate) const NODE_BOX_SPLITS: u32 = 8;

/// **The node box at a chain's end** (C2's node end), as its half-widths
/// along `u` and `v` about `end`, the chain's end on the wall's chart:
/// `unit` is each axis's chart units per metre, `domain` the wall's knot
/// rectangle, and `reach` how far the chain's end span reaches from
/// `end`, in metres.
///
/// - **The end is at the node:** the one zero of `∇φ` in the box of
///   [`NODE_END_SEED`] escalations about `end` is decided a node
///   ([`decide`]). A chain whose end stands farther than that from the
///   node, or at no node, has no node end.
/// - **The box is the node's:** [`NODE_BOX_REACH`] times `reach`, where
///   `reach` is at most [`NODE_BOX_REACH`] times the node's settle reach
///   `√(NODE_SETTLE_FLOOR·ε′/σ)` metres, `ε′` the band's escalation and
///   `σ` the least singular value of the node's Hessian in metres per
///   metre² (its form's Hessian over a bound on its gradient's length),
///   so `√(ε′/σ)` is C2's `√(εR)`. A march's settled samples stop where
///   `|∇φ|·d` reaches [`NODE_SETTLE_FLOOR`] of its tolerance
///   (`super::ends::Ends::enter`), within the settle reach of the node,
///   or at the box the march entered, whichever is farther; an end span
///   reaching past that is no node's, and the box is never wider than
///   [`NODE_BOX_REACH`]² settle reaches.
/// - **It holds the node alone:** read again over the whole box, its
///   one zero of `∇φ` is the node, and the box lies strictly inside
///   `domain`.
///
/// `None` wherever any of that is not proved.
///
/// [`NODE_BOX_REACH`]: super::certify::NODE_BOX_REACH
/// [`NODE_SETTLE_FLOOR`]: super::ends::NODE_SETTLE_FLOOR
pub(crate) fn node_box(
    jets: &Jets,
    phi: &ImplicitForm,
    (end, unit, domain, reach): ((f64, f64), (f64, f64), UvRect, f64),
    band: Band,
) -> Option<(f64, f64)> {
    let about = |(hu, hv): (f64, f64)| UvRect {
        u: (end.0 - hu, end.0 + hu),
        v: (end.1 - hv, end.1 + hv),
    };
    let inside = |r: UvRect| {
        domain.u.0 < r.u.0 && r.u.1 < domain.u.1 && domain.v.0 < r.v.0 && r.v.1 < domain.v.1
    };
    let seed = NODE_END_SEED * band.escalate();
    let seeded = about((seed * unit.0, seed * unit.1));
    if !inside(seeded) {
        return None;
    }
    let at = decide(jets, phi, seeded, band).filter(|d| d.kind == Kind::Node)?;
    let reach_cap = super::certify::NODE_BOX_REACH * settle_reach(jets, phi, &at, unit, band)?;
    if !(reach > 0.0 && reach <= reach_cap) {
        return None;
    }
    let metres = super::certify::NODE_BOX_REACH * reach;
    let half = (metres * unit.0, metres * unit.1);
    let held = about(half);
    (inside(held)
        && decide_split(jets, phi, held, NODE_BOX_SPLITS, band).is_some_and(|d| d.kind == Kind::Node))
    .then_some(half)
}

/// **How far from a node a march's settled samples stop**, in metres
/// (`super::ends::Ends::enter`): `√(NODE_SETTLE_FLOOR·ε′/σ)`, `ε′` the
/// band's escalation and `σ` the least singular value of the node's
/// Hessian `at.hessian` in metres per metre², each chart axis through
/// `unit` (its chart units per metre) and the form through a bound on
/// its gradient's length there. `None` where it does not read finite.
fn settle_reach(
    jets: &Jets,
    phi: &ImplicitForm,
    at: &Decided,
    unit: (f64, f64),
    band: Band,
) -> Option<f64> {
    let p = jets.net.eval(at.last[0], at.last[1]);
    let g = phi.gradient_sup(phi.framed(super::enclose::Box3 {
        x: p.x,
        y: p.y,
        z: p.z,
    }));
    let h = at.hessian.map(|row| row.map(mid));
    let (a, b, c) = (
        h[0][0] * unit.0 * unit.0 / g,
        h[0][1] * unit.0 * unit.1 / g,
        h[1][1] * unit.1 * unit.1 / g,
    );
    // A symmetric 2×2's eigenvalues; the least magnitude is its least
    // singular value.
    let (mean, spread) = (0.5 * (a + c), (0.5 * (a - c)).hypot(b));
    let sigma = (mean - spread).abs().min((mean + spread).abs());
    let reach = (super::ends::NODE_SETTLE_FLOOR * band.escalate() / sigma).sqrt();
    (reach.is_finite() && reach > 0.0).then_some(reach)
}
