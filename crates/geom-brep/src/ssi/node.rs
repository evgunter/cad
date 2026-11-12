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
use geom_core::{Band, Bounds, Interval, Margin, Real, Vec3};

use super::exhaust::UvRect;
use super::residual::ImplicitForm;

/// The partner `φ` is read of: its form.
pub(crate) type Phi<'a> = &'a ImplicitForm;

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
pub(crate) fn read(jets: &Jets, phi: Phi<'_>, u: Interval, v: Interval) -> Read {
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

/// **The Krawczyk box of `x`**, `K ∩ x`, and whether `K` lies strictly
/// inside `x` (module docs). `None` where a reading does not certify or
/// the Hessian's midpoint is singular.
pub(crate) fn krawczyk(
    jets: &Jets,
    phi: Phi<'_>,
    x: [Interval; 2],
) -> Option<([Interval; 2], bool)> {
    let m = [mid(x[0]), mid(x[1])];
    let at = read(jets, phi, Interval::point(m[0]), Interval::point(m[1]));
    let over = read(jets, phi, x[0], x[1]);
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

impl Jets {
    /// The wall's net at a certifying scalar, each of its scalars its
    /// exact `f64` ([`super::certify::exact_certified`]); `None` where
    /// one is not exact or the net does not lift.
    pub(crate) fn of_exact<T: Real + geom_core::CertifiedEnclosure>(
        net: &NurbsSurface<T>,
    ) -> Option<Self> {
        let inexact = core::cell::Cell::new(false);
        let at_f64 = net.map_scalar(|x| {
            super::certify::exact_certified(x).unwrap_or_else(|| {
                inexact.set(true);
                f64::NAN
            })
        });
        if inexact.get() {
            return None;
        }
        Self::of(&at_f64)
    }
}

/// **The node's own reading over a box at a chain's end** (C2's node
/// end): `∇φ` has one zero in `x`, its Hessian's determinant over the
/// box `K` contracts to is decided negative (an indefinite Hessian, two
/// arcs crossing there), and `φ` over that box is decided Zero in metres
/// (`spline_critical_value`, the reading C5 lists the node by). The box
/// `K` is cut to `K ∩ x` and read again, at most [`KRAWCZYK_ROUNDS`]
/// times; each cut keeps every zero of `x`, so a zero proved alone in a
/// cut box is `x`'s only one.
///
/// `false` wherever any of that is not proved: no zero, a zero not
/// proved alone, a touch, a value not decided Zero, or a reading that
/// does not certify.
pub(crate) fn holds_node(jets: &Jets, phi: Phi<'_>, x: UvRect, band: Band) -> bool {
    let (Some(u), Some(v)) = (range(x.u), range(x.v)) else {
        return false;
    };
    let mut b = [u, v];
    let mut proved = false;
    for _ in 0..KRAWCZYK_ROUNDS {
        let Some((next, inside)) = krawczyk(jets, phi, b) else {
            return false;
        };
        proved |= inside;
        let narrower = next[0].hi() - next[0].lo() < b[0].hi() - b[0].lo()
            || next[1].hi() - next[1].lo() < b[1].hi() - b[1].lo();
        b = next;
        if !narrower {
            break;
        }
    }
    if !proved {
        return false;
    }
    let at = read(jets, phi, b[0], b[1]);
    let det = at.hessian[0][0] * at.hessian[1][1] - at.hessian[0][1] * at.hessian[1][0];
    if !(det.is_certified() && det.hi() < 0.0 && at.value.is_certified()) {
        return false;
    }
    matches!(
        decide_magnitude_reported("spline_critical_value", Margin::of(at.ceiling), band),
        Ok((Magnitude::Zero, _))
    )
}

/// **`∇φ·e` over the box `x`, read through `φ`'s Hessian** (the mean-value
/// form): `∇φ(m)·e + eᵀ∇²φ(x)·(x − m)`, `m` the box's middle, an
/// enclosure of the transverse derivative limb 3's chart tube reads
/// (C2). Near a node, where the surfaces nearly osculate, `∇φ` is the
/// small difference of two nearly equal terms, and the direct reading
/// `∇f(S(x))·S_d(x)` spreads by either term's own variation over the box
/// while this one spreads by `φ`'s Hessian's alone. Refused where a
/// reading does not certify.
pub(crate) fn transverse(jets: &Jets, phi: Phi<'_>, x: UvRect, e: (f64, f64)) -> Interval {
    let (Some(u), Some(v)) = (range(x.u), range(x.v)) else {
        return Interval::refused();
    };
    let m = [mid(u), mid(v)];
    let at = read(jets, phi, Interval::point(m[0]), Interval::point(m[1]));
    let over = read(jets, phi, u, v);
    let e = [Interval::point(e.0), Interval::point(e.1)];
    let d = [u - Interval::point(m[0]), v - Interval::point(m[1])];
    let mut acc = at.gradient[0] * e[0] + at.gradient[1] * e[1];
    for (b, db) in d.iter().enumerate() {
        acc = acc + (e[0] * over.hessian[0][b] + e[1] * over.hessian[1][b]) * *db;
    }
    acc
}
