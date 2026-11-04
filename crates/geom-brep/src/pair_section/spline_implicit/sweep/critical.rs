//! **A critical point of `φ` decided** (C5): the cells the regularity
//! sweep cannot read at the floor, clustered, each cluster isolated as
//! one zero of `∇φ` by the Krawczyk operator and decided by its Hessian
//! and its value.
//!
//! On a box `X` of the wall's chart, `∇φ = (∇f(S)·S_u, ∇f(S)·S_v)` and
//! its Jacobian, `φ`'s Hessian
//! `φ_ab = S_aᵀ ∇²f S_b + ∇f·S_ab`, are enclosed through the wall's jet
//! over `X` in interval arithmetic ([`geom::NurbsSurface::ders`] at the
//! box's parameters) and the partner's form, its gradient and its
//! Hessian over the box of `S(X)` (a plane's `∇f` is its unit normal and
//! its `∇²f` zero). With `m` the box's middle and `Y` the inverse of the
//! Hessian's midpoint,
//! `K = m − Y·∇φ(m) + (I − Y·∇²φ(X))·(X − m)` holds every zero of `∇φ` in
//! `X`, and where `K` lies strictly inside `X` it holds one: **existence**
//! is what containment proves, and with it every matrix in `∇²φ(X)` is
//! invertible, so the zero is the only one (`crate::krawczyk`'s operator,
//! two dimensions down). Without it, a box over a regular stretch where
//! `∇φ` is small but nowhere zero would be read as a critical point that
//! is not there. A box `K` does not fit is cut to `K ∩ X` and read again,
//! a fixed number of times; each cut keeps every zero of the first box,
//! so the one zero found is the first box's only one.
//!
//! The point found is then decided:
//!
//! - **its value**, `φ` over the final box in metres, read on a ceiling
//!   against the band (`spline_critical_value`): decided zero, the point
//!   is a tangency decided Zero; in band or past it, the cluster is not
//!   decided here and the sweep declines;
//! - **its Hessian's determinant** over the final box: positive, a definite
//!   Hessian, a **touch**; negative, an indefinite one, a **node**, the
//!   arcs of the section crossing there; straddling zero, the sweep
//!   declines.
//!
//! A box over a knot reads the jets of every span it touches, hulled, so
//! a cluster at a knot reads a hull the Krawczyk test may not fit, and
//! is then not decided. The caller declines a decided point whose final
//! box is not strictly inside its piece of the rectangle.

use geom::NurbsSurface;
use geom_core::interval::certification::Certification;
use geom_core::k_stats::{Magnitude, decide_magnitude_reported};
use geom_core::{Band, Bounds, Interval, Margin, MarginDiag, Vec3};

use super::Phi;
use crate::ssi::exhaust::UvRect;

/// The most times a box `K` does not fit is cut to `K ∩ X` and read
/// again (D9, a fixed rule).
const KRAWCZYK_ROUNDS: u32 = 12;

/// What a critical point decided Zero is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::pair_section) enum Kind {
    /// A definite Hessian: the surfaces touch at one point.
    Touch,
    /// An indefinite Hessian: the section's arcs cross at the point.
    Node,
}

/// **A critical point of `φ` decided Zero**: its chart point (the last
/// box's middle), the box that holds it, what it is, and the margin of
/// its value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::pair_section) struct Critical {
    /// The chart point.
    pub at: (f64, f64),
    /// The box in which `∇φ` has this one zero.
    pub cell: UvRect,
    /// The first box the Krawczyk operator proved holds this one zero of
    /// `∇φ` (`K` strictly inside it), the widest such: the region the
    /// point claims, which seeding skips and accounting reads, and, at a
    /// node, the box a march ends on entering (C5's node end).
    pub region: UvRect,
    /// A touch or a node.
    pub kind: Kind,
    /// The value's margin, decided zero.
    pub margin: MarginDiag,
    /// A node's frame, read off the Hessian enclosure that decided it
    /// ([`NodeFrame`]); `None` at a touch.
    pub node: Option<NodeFrame>,
}

/// **A node as the value lists it** (C5), read off `φ`'s Hessian `H` over
/// the final box `b`, the enclosure that decided the node (`det H < 0`):
/// its point `S(b)`, the wall's unit normal `n = S_u × S_v` there, the
/// two null lines of `H` carried to the wall's tangent plane, the second
/// turned from the first counterclockwise about `n` by less than a half
/// turn, and whether the wall lies on `n`'s side of the partner in the
/// sector between the first line's and the second line's positive rays.
///
/// In the chart, `dᵀHd = a·du² + 2h·du·dv + c·dv²` vanishes on two
/// directions, written homogeneously so no division reads a straddling
/// entry: with `q = −(h + s·√(h² − ac))`, `s` the sign of `h`'s middle,
/// they are `(c, q)` and `(q, a)` (their slopes `q/c` and `a/q`, the
/// quadratic's two roots), each nonzero where `q` is certified off zero.
/// Each is carried to the tangent plane as `S_u·du + S_v·dv`. The turn
/// of the two about `n`, the sign of `H` on the sector's bisector, and
/// the sign of `∇f·n` (which side of the partner `f > 0` is along `n`)
/// are each read as the sign of an enclosure over `b`, as `det H` is;
/// an enclosure that straddles zero leaves the node undecided and the
/// sweep declines. In the sector, a wall point stands `δ` off the partner
/// along `n` and `φ ≈ δ·(∇f·n)`, so the wall is on `n`'s side where `H`'s
/// sign and `∇f·n`'s agree.
#[derive(Clone, Copy, Debug)]
pub(in crate::pair_section) struct NodeFrame {
    /// The node, `S(b)`'s middle.
    pub point: geom_core::Point3<f64>,
    /// The wall's unit normal there.
    pub normal: Vec3<f64>,
    /// The two null lines, unit, in the wall's tangent plane.
    pub lines: [Vec3<f64>; 2],
    /// Whether the wall lies on `normal`'s side of the partner in sector
    /// 0, from `+lines[0]` to `+lines[1]`.
    pub wall_above: bool,
    /// The widest coordinate of `S(b)`, in metres: the node is placed
    /// within the band where this is within its zero ([`NODE_BOX`]).
    pub width: f64,
}

impl PartialEq for NodeFrame {
    /// Bit for bit.
    fn eq(&self, other: &Self) -> bool {
        let bits = |f: &Self| {
            let [l0, l1] = f.lines;
            [
                f.point.x, f.point.y, f.point.z, f.normal.x, f.normal.y, f.normal.z, l0.x, l0.y,
                l0.z, l1.x, l1.y, l1.z,
            ]
            .map(f64::to_bits)
        };
        bits(self) == bits(other)
            && self.wall_above == other.wall_above
            && self.width.to_bits() == other.width.to_bits()
    }
}

/// Why a node's frame is not read: its point not placed within the band
/// (C5: "placed … or enclosed alone in a certified box").
pub(in crate::pair_section) const NODE_BOX: &str = "spline_node_box";

/// The wall's chart, read in interval arithmetic: its jet over a box.
pub(super) struct Jets {
    net: NurbsSurface<Interval>,
}

impl Jets {
    /// The wall's net, its knots, points and weights as point enclosures;
    /// `None` where it does not lift.
    pub(super) fn of(net: &NurbsSurface<f64>) -> Option<Self> {
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
struct Read {
    gradient: [Interval; 2],
    hessian: [[Interval; 2]; 2],
    value: Interval,
    ceiling: f64,
}

/// Reads `φ` of `phi` over the box `(u, v)` of the wall's chart.
fn read(jets: &Jets, phi: Phi<'_>, u: Interval, v: Interval) -> Read {
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

/// **`φ` along a crease line**, over the stretch `(u, v)` of it (one of
/// the two a point): `φ` there in metres, a floor on its distance,
/// signed; and the line's slope off the partner, `∇f·S_d` over
/// `sup‖∇f‖·sup‖S_d‖`, `d` the line's own direction (`along_u`), a
/// derivative continuous across the crease. Read through the wall's jet
/// at the stretch, so the derivative is the line's own and not the
/// cell's beside it.
pub(super) fn along_line(
    jets: &Jets,
    phi: Phi<'_>,
    (u, v): (Interval, Interval),
    along_u: bool,
) -> (Interval, Interval) {
    let jet = jets.net.ders(u, v);
    let p = Vec3::new(jet.point.x, jet.point.y, jet.point.z);
    let d = if along_u { jet.du } else { jet.dv };
    let q = phi.framed(crate::ssi::enclose::Box3 {
        x: p.x,
        y: p.y,
        z: p.z,
    });
    let (metres, g, d) = (
        phi.floor_metres(phi.value(q), q),
        phi.frame_gradient(q),
        phi.frame_vector(d),
    );
    let norm = |w: Vec3<Interval>| {
        Interval::point(Certification::sqrt(w.x.sqr() + w.y.sqr() + w.z.sqr()).hi())
    };
    (metres, super::sine(g, norm(g), d, norm(d)))
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
fn range((lo, hi): (f64, f64)) -> Option<Interval> {
    (lo <= hi && lo.is_finite() && hi.is_finite()).then(|| Interval::from_bounds(lo, hi))
}

/// The middle of an interval.
fn mid(x: Interval) -> f64 {
    0.5 * (x.lo() + x.hi())
}

/// **The Krawczyk box of `x`**, `K ∩ x`, and whether `K` lies strictly
/// inside `x` (module docs). `None` where a reading does not certify or
/// the Hessian's midpoint is singular.
fn krawczyk(jets: &Jets, phi: Phi<'_>, x: [Interval; 2]) -> Option<([Interval; 2], bool)> {
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

/// **The cluster in `x` decided** (module docs): the critical point it
/// holds decided Zero, or `None` where it is not decided here. `x` lies
/// inside one piece of the rectangle between its creases, over which
/// `∇φ` is continuous.
pub(super) fn decide(
    jets: &Jets,
    phi: Phi<'_>,
    (x, piece): (UvRect, UvRect),
    band: Band,
) -> Option<Critical> {
    let mut b = [range(x.u)?, range(x.v)?];
    let mut region: Option<[Interval; 2]> = None;
    for _ in 0..KRAWCZYK_ROUNDS {
        let (next, inside) = krawczyk(jets, phi, b)?;
        let narrower = next[0].hi() - next[0].lo() < b[0].hi() - b[0].lo()
            || next[1].hi() - next[1].lo() < b[1].hi() - b[1].lo();
        if inside && region.is_none() {
            region = Some(b);
        }
        b = next;
        if !narrower {
            break;
        }
    }
    let region = region?;
    let at = read(jets, phi, b[0], b[1]);
    let det = at.hessian[0][0] * at.hessian[1][1] - at.hessian[0][1] * at.hessian[1][0];
    if !det.is_certified() {
        return None;
    }
    let kind = if det.lo() > 0.0 {
        Kind::Touch
    } else if det.hi() < 0.0 {
        Kind::Node
    } else {
        return None;
    };
    if !at.value.is_certified() {
        return None;
    }
    let rect = |b: [Interval; 2]| UvRect {
        u: (b[0].lo(), b[0].hi()),
        v: (b[1].lo(), b[1].hi()),
    };
    match decide_magnitude_reported("spline_critical_value", Margin::of(at.ceiling), band) {
        Ok((Magnitude::Zero, margin)) => {
            let node = match kind {
                Kind::Touch => None,
                Kind::Node => Some(node_frame(jets, phi, b, at.hessian)?),
            };
            Some(Critical {
                at: (mid(b[0]), mid(b[1])),
                cell: rect(b),
                region: rect(grown(jets, phi, region, piece)),
                kind,
                margin,
                node,
            })
        }
        Ok((Magnitude::Positive, _)) | Err(_) => None,
    }
}

/// The most times a proved box is doubled about its middle (D9, a fixed
/// rule).
const GROW_ROUNDS: u32 = 24;

/// The widest a proved box grows, as a fraction of its piece's narrower
/// side, so the boxes of two points in one piece stay apart.
const GROW_SHARE: f64 = 1.0 / 32.0;

/// **The widest box about `b` the Krawczyk operator proves holds one
/// zero of `∇φ`** (`K` strictly inside it): `b` doubled about its middle
/// while the doubled box lies strictly inside `piece`, no wider than
/// [`GROW_SHARE`] of the piece, and is proved. A march from the arcs
/// ends on entering it, so it is grown past where the arcs' slopes read
/// in the band; every box it passes through is proved, so the zero it
/// holds is still the only one.
fn grown(jets: &Jets, phi: Phi<'_>, b: [Interval; 2], piece: UvRect) -> [Interval; 2] {
    let m = [mid(b[0]), mid(b[1])];
    let cap = GROW_SHARE * (piece.u.1 - piece.u.0).min(piece.v.1 - piece.v.0);
    let mut half = [0.5 * (b[0].hi() - b[0].lo()), 0.5 * (b[1].hi() - b[1].lo())];
    let mut best = b;
    for _ in 0..GROW_ROUNDS {
        half = half.map(|h| 2.0 * h);
        if half.iter().any(|&h| h > cap) {
            break;
        }
        let (u, v) = (
            (m[0] - half[0], m[0] + half[0]),
            (m[1] - half[1], m[1] + half[1]),
        );
        if !(u.0 > piece.u.0 && u.1 < piece.u.1 && v.0 > piece.v.0 && v.1 < piece.v.1) {
            break;
        }
        let (Some(bu), Some(bv)) = (range(u), range(v)) else {
            break;
        };
        match krawczyk(jets, phi, [bu, bv]) {
            Some((_, true)) => best = [bu, bv],
            _ => break,
        }
    }
    best
}

/// The certified sign of an enclosure: `Some(true)` positive, `None`
/// where it straddles zero or is refused.
fn sign(i: Interval) -> Option<bool> {
    if !i.is_certified() {
        return None;
    }
    if i.lo() > 0.0 {
        Some(true)
    } else if i.hi() < 0.0 {
        Some(false)
    } else {
        None
    }
}

/// **A node's frame** ([`NodeFrame`]) over the final box `b`, from the
/// Hessian enclosure `h` that decided it; `None` where any sign it reads
/// straddles zero.
fn node_frame(
    jets: &Jets,
    phi: Phi<'_>,
    b: [Interval; 2],
    h: [[Interval; 2]; 2],
) -> Option<NodeFrame> {
    let jet = jets.net.ders(b[0], b[1]);
    let (su, sv) = (jet.du, jet.dv);
    let n = su.cross(sv);
    let (a, hh, c) = (h[0][0], h[0][1], h[1][1]);
    let root = Certification::sqrt(hh.sqr() - a * c);
    let s = if mid(hh) >= 0.0 {
        Interval::point(1.0)
    } else {
        Interval::point(-1.0)
    };
    let q = Interval::point(0.0) - (hh + s * root);
    sign(q)?;
    let chart = [[c, q], [q, a]];
    let norm = |w: Vec3<Interval>| Certification::sqrt(w.x.sqr() + w.y.sqr() + w.z.sqr());
    let world = chart.map(|[du, dv]| su * du + sv * dv);
    let length = world.map(norm);
    if !length.iter().all(|l| sign(*l) == Some(true)) {
        return None;
    }
    let unit = [world[0] / length[0], world[1] / length[1]];
    // Order the lines so the second turns from the first counterclockwise
    // about `n` by less than a half turn.
    let turn = sign(unit[0].cross(unit[1]).dot(n))?;
    let (first, second) = if turn { (0, 1) } else { (1, 0) };
    // `H` on the sector's bisector, in the chart.
    let d = [
        chart[first][0] / length[first] + chart[second][0] / length[second],
        chart[first][1] / length[first] + chart[second][1] / length[second],
    ];
    let on_bisector = a * d[0].sqr() + Interval::point(2.0) * hh * d[0] * d[1] + c * d[1].sqr();
    let sector = sign(on_bisector)?;
    let box3 = crate::ssi::enclose::Box3 {
        x: jet.point.x,
        y: jet.point.y,
        z: jet.point.z,
    };
    let along = sign(phi.gradient(phi.framed(box3)).dot(n))?;
    let point = geom_core::Point3::new(mid(jet.point.x), mid(jet.point.y), mid(jet.point.z));
    let f64_unit = |w: Vec3<Interval>| {
        let v = Vec3::new(mid(w.x), mid(w.y), mid(w.z));
        v * (1.0 / v.norm())
    };
    Some(NodeFrame {
        point,
        normal: f64_unit(n),
        lines: [f64_unit(unit[first]), f64_unit(unit[second])],
        wall_above: sector == along,
        width: [jet.point.x, jet.point.y, jet.point.z]
            .map(|i| i.hi() - i.lo())
            .into_iter()
            .fold(0.0, f64::max),
    })
}
