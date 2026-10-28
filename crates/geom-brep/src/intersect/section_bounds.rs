//! **The bounds a classified pair's section states, and the tube they
//! retire** (C2): limb 3 is owed unless the pair's section function
//! states, for the pose, a lower bound on the sine of the surfaces'
//! crossing angle over the carrier's stretch of the section and the
//! clearance of the tube around the stretch from every other piece of
//! the two surfaces' ε-overlap. This module is the one place those two
//! numbers are stated ([`tube_bounds`]); the analytic rung-3 certificate
//! is the one place that reads them (`edge_nurbs::analytic_rung3`).
//!
//! Everything here is certification arithmetic ([`Interval`]): every
//! datum crosses through its certified bracket, so at a parameter-family
//! scalar a statement holds for every member of the box, and a bound
//! that cannot be certified leaves the tube owed exactly as before.
//!
//! # The theorem
//!
//! Each surface `Sᵢ` is written `Qᵢ = hᵢ·mᵢ` with `Qᵢ` a polynomial of
//! degree at most two, `|hᵢ| ≤ dist(·, Sᵢ)`, `hᵢ = 0` exactly on `Sᵢ`,
//! `‖∇hᵢ‖ = 1` where `hᵢ` is smooth, and `mᵢ` Lipschitz with constant
//! `λᵢ`:
//!
//! | surface | `Qᵢ` | `hᵢ` | `mᵢ` | `λᵢ` | `qᵢ` |
//! |---|---|---|---|---|---|
//! | plane `(o, n̂)` | `(y − o)·n̂` | `Q` | `1` | `0` | `0` |
//! | cylinder `(C, â, R)` | `‖v‖² − (v·â)² − R²` | `ρ − R` | `ρ + R` | `1` | `1` |
//! | sphere `(C, R)` | `‖v‖² − R²` | `r − R` | `r + R` | `1` | `1` |
//! | cone `(A, â, α)` | `cos²α‖v‖² − (v·â)²` | `ρ cos α − |z| sin α` | `ρ cos α + |z| sin α` | `1` | `1` |
//!
//! (`v = y − C`, `ρ` the distance from the axis, `z = v·â`, `r = ‖v‖`;
//! `qᵢ` bounds the quadratic part, `|dᵀAᵢd| ≤ qᵢ‖d‖²`.) The cone's `h`
//! is the distance from the nearer generator line of the meridian
//! half-plane, which bounds the distance from the double cone from
//! below by Cauchy–Schwarz; `ρ`, `|z|` and so `m` are 1-Lipschitz.
//!
//! On the section `Γ` away from the apex `∇Qᵢ = mᵢ∇hᵢ`. Take a point
//! `y` of the ε-overlap `O = {dist(·, S₁) ≤ ε, dist(·, S₂) ≤ ε}` at
//! distance `t` from a component `Γ_c`, its nearest point `γ`, and
//! `d = y − γ`, normal to `Γ_c`, so in the plane the two unit normals
//! `∇hᵢ(γ)` span: one of them has `|∇hᵢ·d| ≥ t·sin θ/2`. Since `Qᵢ` is
//! quadratic, `Qᵢ(y) = mᵢ(γ)∇hᵢ·d + dᵀAᵢd` exactly, while `y ∈ O` gives
//! `|Qᵢ(y)| = |hᵢ(y)|·mᵢ(y) ≤ ε(mᵢ(γ) + λᵢt)`. So with `s` a floor on
//! `sin θ` and `mᵢ` floored by `m̲ᵢ` over the section's stretch, `y` is
//! not in `O` wherever, for both surfaces,
//!
//! ```text
//! Wᵢ(t) = m̲ᵢ·(t·s/2 − ε) − qᵢ·t² − ε·λᵢ·t > 0.
//! ```
//!
//! `Wᵢ` is concave, so it holds on `[w, T]` where it holds at both ends.
//! With `w = 8ε/s` and `T = 2w` checked, no point of `O` whose nearest
//! point on `Γ_c` lies in the stretch stands between `w` and `T` of
//! `Γ_c` — the **clearance** `T − w` of the tube of radius `w` from every
//! other piece of `O` there, a second component of the section and the
//! near-miss regions alike (the same inequality at `y ∈ Γ` puts no other
//! section point within `T·(1 − 2ε/(sT))` of `Γ_c`). The carrier lies in
//! `O` (limb 2) and is connected, so its distance from `Γ_c` is
//! continuous and never crosses the moat: one point of it within `T`
//! puts all of it within `w`.
//!
//! **The stretch** is `Γ` inside the carrier's control hulls padded by
//! `T`: each span the edge's interval meets is cut into pieces by knot
//! insertion in certification arithmetic, and a piece lies in the box
//! of its control points, so a point of the carrier and its nearest
//! point on `Γ_c` within `T` both lie in one of the padded boxes. Each
//! floor is the least over the boxes. **The one point within `T`** is the
//! carrier's point at an interior parameter, enclosed by evaluating the
//! carrier in certification arithmetic, beside which Poincaré–Miranda
//! finds a common zero of `Q₁` and `Q₂` on a square whose sides are
//! preconditioned by the two gradients: `Qᵢ` takes opposite certified
//! signs on its two opposite sides (each side's value enclosed by `Qᵢ`'s
//! exact second-order expansion about the square's centre). `Γ_c` is the
//! component through that zero.
//!
//! # Each pair's closed forms
//!
//! Over the stretch, with `δ` the second surface's anchor's signed height
//! over the plane `(o, n̂)` and `c = n̂·â`:
//!
//! - **plane × plane**: `sin θ = ‖n̂₁ × n̂₂‖`, and `m = 1` on both.
//! - **plane × cylinder**: on `Γ`, `ρ̂ = (v − z·â)/R` and `n̂·v = −δ`, so
//!   `cos θ = n̂·ρ̂ = −(δ + z·c)/R`, affine in `z = v·â` over the stretch;
//!   `m = 2R` on the cylinder.
//! - **plane × sphere**: `cos θ = n̂·v/R = −δ/R`, constant; `m = 2R`.
//! - **plane × cone**: on `Γ`, `ρ = |z| tan α` and `∇h = cos α·ρ̂ −
//!   sin α·sgn(z)·â`, so `cos θ = −(δ·cos²α/|z| + sgn(z)·c)/sin α`, affine
//!   in `1/|z|` on the nappe the stretch lies on; `m = 2|z| sin α`. A
//!   stretch reaching the apex's height straddles `z = 0` and states
//!   nothing; one near it has `m` small, so the moat fails there: the
//!   apex neighbourhood at a small stand-off, where the two branches of a
//!   hyperbola meet in the overlap, owes the tube.
//! - **equal-radius cylinder × cylinder**, both axes through one stored
//!   point `C` and the radii one stored value: `Q₁ − Q₂ =
//!   (v·(â₂ − â₁))·(v·(â₂ + â₁))`, so `Γ` lies in the two bisector planes,
//!   and where the stretch excludes one of them the other holds it, with
//!   `v·â₂ = ±v·â₁ = ±ζ`. There `ρ̂₁·ρ̂₂ = 1 − ζ²(1 ∓ k)/R²`, `k = â₁·â₂`;
//!   `m = 2R` on both. The two ellipses cross where `ζ = 0`, tangent, so a
//!   stretch reaching it states nothing.
//!
//! Every other pair — the torus rows among them — states nothing, and
//! owes the tube.

use geom::{NurbsCurve3, Surface, SurfaceKind};
use geom_core::interval::certification::Certification;
use geom_core::{Band, Bounds, CertifiedBounds, Interval, Point3};

use crate::ssi::enclose::Box3;

/// **What a pair's section states for a pose over a carrier's stretch**
/// (module docs): the crossing angle's floor and the tube it buys.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SectionBounds {
    /// A certified lower bound on `sin θ` over the carrier's stretch of
    /// the section.
    pub sin_floor: f64,
    /// The tube's radius, `8ε/sin θ_min`: the carrier lies within it of
    /// the section's component.
    pub radius: f64,
    /// The tube's clearance: no point of the ε-overlap near the stretch
    /// lies between `radius` and `radius + clearance` of the component.
    pub clearance: f64,
}

/// Why a pair's section states no bounds for a pose, so the tube stays
/// owed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unstated {
    /// The pair's section has no closed form for the bounds (a torus
    /// row, a spline operand, or a pair of quadrics other than the
    /// classified ones).
    Pair {
        /// The first operand's kind.
        a: SurfaceKind,
        /// The second's.
        b: SurfaceKind,
    },
    /// The pose: the clause names the bound that could not be certified.
    Pose(&'static str),
}

impl core::fmt::Display for Unstated {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Pair { a, b } => write!(
                f,
                "a {a:?}×{b:?} section states no crossing-angle floor in closed form"
            ),
            Self::Pose(why) => write!(f, "the section states no bounds for this pose: {why}"),
        }
    }
}

/// **The bounds the pair `(s1, s2)`'s section states over `carrier` on
/// `params`** (module docs): `Ok` where the tube of radius `8ε/sin θ_min`
/// around the section's component is certified clear of the rest of the
/// ε-overlap, `ε` the band's zero, and the carrier, which limb 2 has put
/// within `ε` of both surfaces, lies inside it.
///
/// # Errors
///
/// [`Unstated::Pair`] for a pair with no closed form, and
/// [`Unstated::Pose`] naming the bound this pose could not certify.
pub fn tube_bounds<T: CertifiedBounds>(
    carrier: &NurbsCurve3<T>,
    params: (T, T),
    s1: &Surface<T>,
    s2: &Surface<T>,
    band: Band,
) -> Result<SectionBounds, Unstated> {
    let pair = Unstated::Pair {
        a: s1.kind(),
        b: s2.kind(),
    };
    let (a, b) = (Quadric::of(s1).ok_or(pair)?, Quadric::of(s2).ok_or(pair)?);
    // Order the pair as the closed forms read it: a plane first.
    let (a, b) = match (a, b) {
        (q, p @ Quadric::Plane { .. }) if !matches!(q, Quadric::Plane { .. }) => (p, q),
        ab => ab,
    };
    let eps = band.zero();
    let hulls = control_hulls(carrier, params).ok_or(Unstated::Pose(
        "the carrier's control hull over the edge's interval is not certified",
    ))?;
    // The stretch is the section inside the hulls padded by `T`, and `T`
    // is read off the floor over it: a floor over the bare hulls sizes a
    // pad of twice the `T` it implies, and the floor over those padded
    // boxes must keep at least half the first for its own `T` to fit.
    let over = |pad: f64| -> Result<Floors, Unstated> {
        hulls.iter().try_fold(
            Floors {
                sin: f64::INFINITY,
                m: [f64::INFINITY; 2],
            },
            |acc, h| {
                let f = floors(a, b, h.pad(pad), pair)?;
                Ok(Floors {
                    sin: acc.sin.min(f.sin),
                    m: [acc.m[0].min(f.m[0]), acc.m[1].min(f.m[1])],
                })
            },
        )
    };
    let first = over(0.0)?;
    let floor = over(32.0 * eps / first.sin)?;
    if floor.sin < 0.5 * first.sin {
        return Err(Unstated::Pose(
            "the crossing angle's floor halves over the padded stretch",
        ));
    }
    let s = floor.sin;
    let (w, t_out) = (8.0 * eps / s, 16.0 * eps / s);
    for (q, m) in [(a, floor.m[0]), (b, floor.m[1])] {
        for t in [w, t_out] {
            let (t, s, e, m) = (
                Interval::point(t),
                Interval::point(s),
                Interval::point(eps),
                Interval::point(m),
            );
            let moat = m * (t * s / Interval::point(2.0) - e)
                - Interval::point(q.hess()) * t.powi(2)
                - e * Interval::point(q.lipschitz()) * t;
            if !(moat.is_certified() && moat.lo() > 0.0) {
                return Err(Unstated::Pose(
                    "the ε-overlap may reach the tube's moat (another component of the section, \
                     or a near miss of the two surfaces)",
                ));
            }
        }
    }
    let at = witness(carrier, params).ok_or(Unstated::Pose(
        "the carrier has no certified point inside the edge's interval",
    ))?;
    if !miranda(a, b, at, t_out) {
        return Err(Unstated::Pose(
            "no point of the section is certified within the moat of the carrier's point",
        ));
    }
    Ok(SectionBounds {
        sin_floor: s,
        radius: w,
        clearance: t_out - w,
    })
}

/// A surface as `Q = h·m` (module docs' table), its data crossed into
/// certification arithmetic and its directions made unit there.
#[derive(Clone, Copy, Debug)]
enum Quadric {
    Plane {
        origin: [Interval; 3],
        normal: [Interval; 3],
    },
    Cylinder {
        origin: [Interval; 3],
        axis: [Interval; 3],
        radius: Interval,
    },
    Sphere {
        center: [Interval; 3],
        radius: Interval,
    },
    Cone {
        apex: [Interval; 3],
        axis: [Interval; 3],
        sin_a: Interval,
        cos_a: Interval,
    },
}

fn point<T: CertifiedBounds>(p: Point3<T>) -> [Interval; 3] {
    [p.x, p.y, p.z].map(Interval::from_certified)
}

fn unit<T: CertifiedBounds>(v: geom_core::Vec3<T>) -> [Interval; 3] {
    let v = [v.x, v.y, v.z].map(Interval::from_certified);
    let n = norm_sq(v).sqrt();
    v.map(|c| c / n)
}

fn dot(a: [Interval; 3], b: [Interval; 3]) -> Interval {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// `‖a‖²` as a sum of squares, so a component straddling zero keeps the
/// floor at zero.
fn norm_sq(a: [Interval; 3]) -> Interval {
    a[0].sqr() + a[1].sqr() + a[2].sqr()
}

fn sub(a: [Interval; 3], b: [Interval; 3]) -> [Interval; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn add(a: [Interval; 3], b: [Interval; 3]) -> [Interval; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn scale(a: [Interval; 3], k: Interval) -> [Interval; 3] {
    a.map(|c| c * k)
}

fn cross(a: [Interval; 3], b: [Interval; 3]) -> [Interval; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn boxed(b: Box3) -> [Interval; 3] {
    [b.x, b.y, b.z]
}

impl Quadric {
    fn of<T: CertifiedBounds>(s: &Surface<T>) -> Option<Self> {
        Some(match *s {
            Surface::Plane { origin, normal, .. } => Self::Plane {
                origin: point(origin),
                normal: unit(normal),
            },
            Surface::Cylinder {
                origin,
                axis,
                radius,
                ..
            } => Self::Cylinder {
                origin: point(origin),
                axis: unit(axis),
                radius: Interval::from_certified(radius),
            },
            Surface::Sphere { center, radius, .. } => Self::Sphere {
                center: point(center),
                radius: Interval::from_certified(radius),
            },
            Surface::Cone {
                apex,
                axis,
                half_angle,
                ..
            } => {
                // Lane evaluation at `T`, crossed in through its bracket
                // as the composite's cone form crosses it: certification
                // calls no transcendental (C9).
                let (sin_a, cos_a) = half_angle.sin_cos();
                let (sin_a, cos_a) = (
                    Interval::from_certified(sin_a),
                    Interval::from_certified(cos_a),
                );
                Self::Cone {
                    apex: point(apex),
                    axis: unit(axis),
                    sin_a,
                    cos_a,
                }
            }
            Surface::Torus { .. } | Surface::Nurbs(_) | Surface::Approx(_) => return None,
        })
    }

    /// `Q` at `y`.
    fn value(self, y: [Interval; 3]) -> Interval {
        match self {
            Self::Plane { origin, normal } => dot(sub(y, origin), normal),
            Self::Cylinder {
                origin,
                axis,
                radius,
            } => {
                let v = sub(y, origin);
                norm_sq(v) - dot(v, axis).powi(2) - radius.powi(2)
            }
            Self::Sphere { center, radius } => {
                let v = sub(y, center);
                norm_sq(v) - radius.powi(2)
            }
            Self::Cone {
                apex, axis, cos_a, ..
            } => {
                let v = sub(y, apex);
                cos_a.powi(2) * norm_sq(v) - dot(v, axis).powi(2)
            }
        }
    }

    /// `∇Q` at `y`.
    fn grad(self, y: [Interval; 3]) -> [Interval; 3] {
        let two = Interval::point(2.0);
        match self {
            Self::Plane { normal, .. } => normal,
            Self::Cylinder { origin, axis, .. } => {
                let v = sub(y, origin);
                scale(sub(v, scale(axis, dot(v, axis))), two)
            }
            Self::Sphere { center, .. } => scale(sub(y, center), two),
            Self::Cone {
                apex, axis, cos_a, ..
            } => {
                let v = sub(y, apex);
                scale(sub(scale(v, cos_a.powi(2)), scale(axis, dot(v, axis))), two)
            }
        }
    }

    /// `q`: a bound on the quadratic part over unit vectors.
    fn hess(self) -> f64 {
        match self {
            Self::Plane { .. } => 0.0,
            Self::Cylinder { .. } | Self::Sphere { .. } | Self::Cone { .. } => 1.0,
        }
    }

    /// `λ`: `m`'s Lipschitz constant.
    fn lipschitz(self) -> f64 {
        match self {
            Self::Plane { .. } => 0.0,
            Self::Cylinder { .. } | Self::Sphere { .. } | Self::Cone { .. } => 1.0,
        }
    }
}

/// The floors over a stretch: `sin θ` and each surface's `m` on `Γ`.
struct Floors {
    sin: f64,
    m: [f64; 2],
}

/// **The pair's closed forms over `region`** (module docs, each pair's
/// closed forms), the plane first where there is one.
fn floors(a: Quadric, b: Quadric, region: Box3, pair: Unstated) -> Result<Floors, Unstated> {
    let one = Interval::point(1.0);
    let two = Interval::point(2.0);
    let y = boxed(region);
    let (cos, m): (Interval, [Interval; 2]) = match (a, b) {
        (Quadric::Plane { normal: n1, .. }, Quadric::Plane { normal: n2, .. }) => {
            let x = cross(n1, n2);
            return finish(norm_sq(x).sqrt(), [one, one]);
        }
        (
            Quadric::Plane { origin, normal },
            Quadric::Cylinder {
                origin: c,
                axis,
                radius,
            },
        ) => {
            let delta = dot(sub(c, origin), normal);
            let z = dot(sub(y, c), axis);
            (
                Interval::point(0.0) - (delta + z * dot(normal, axis)) / radius,
                [one, two * radius],
            )
        }
        (Quadric::Plane { origin, normal }, Quadric::Sphere { center, radius }) => {
            let delta = dot(sub(center, origin), normal);
            (Interval::point(0.0) - delta / radius, [one, two * radius])
        }
        (
            Quadric::Plane { origin, normal },
            Quadric::Cone {
                apex,
                axis,
                sin_a,
                cos_a,
            },
        ) => {
            let delta = dot(sub(apex, origin), normal);
            let z = dot(sub(y, apex), axis);
            if !(z.is_certified() && (z.lo() > 0.0 || z.hi() < 0.0)) {
                return Err(Unstated::Pose(
                    "the carrier's stretch reaches the height of the cone's apex",
                ));
            }
            let side = Interval::point(if z.lo() > 0.0 { 1.0 } else { -1.0 });
            let height = z * side;
            (
                Interval::point(0.0)
                    - (delta * cos_a.powi(2) / height + side * dot(normal, axis)) / sin_a,
                [one, two * Interval::point(height.lo()) * sin_a],
            )
        }
        (
            Quadric::Cylinder {
                origin: c1,
                axis: a1,
                radius: r1,
            },
            Quadric::Cylinder {
                origin: c2,
                axis: a2,
                radius: r2,
            },
        ) => {
            let same = |x: Interval, y: Interval| {
                x.is_certified() && x.lo() == x.hi() && y.lo() == x.lo() && y.hi() == x.hi()
            };
            if !(same(r1, r2) && (0..3).all(|i| same(c1[i], c2[i]))) {
                return Err(Unstated::Pose(
                    "the cylinders' radii or the point their axes share are not one stored value",
                ));
            }
            let v = sub(y, c1);
            let k = dot(a1, a2);
            let (minus, plus) = (dot(v, sub(a2, a1)), dot(v, add(a2, a1)));
            let clear = |p: Interval| p.is_certified() && (p.lo() > 0.0 || p.hi() < 0.0);
            // `Γ` lies where `minus` or `plus` vanishes; the bisector the
            // stretch keeps clear of is not the one holding it.
            let lean = if clear(plus) {
                one - k
            } else if clear(minus) {
                one + k
            } else {
                return Err(Unstated::Pose(
                    "the carrier's stretch reaches the line where the two ellipses cross",
                ));
            };
            let zeta = dot(v, a1);
            (one - zeta.powi(2) * lean / r1.powi(2), [two * r1, two * r1])
        }
        _ => return Err(pair),
    };
    let square = one - cos.sqr();
    if !(square.is_certified() && square.lo() > 0.0) {
        return Err(Unstated::Pose(
            "the crossing angle's sine has no positive floor over the carrier's stretch",
        ));
    }
    finish(square.sqrt(), m)
}

fn finish(sin: Interval, m: [Interval; 2]) -> Result<Floors, Unstated> {
    let floor = |x: Interval| (x.is_certified() && x.lo() > 0.0).then(|| x.lo());
    let sin = floor(sin).ok_or(Unstated::Pose(
        "the crossing angle's sine has no positive floor over the carrier's stretch",
    ))?;
    let (Some(m0), Some(m1)) = (floor(m[0]), floor(m[1])) else {
        return Err(Unstated::Pose(
            "a surface's factor has no positive floor over the carrier's stretch",
        ));
    };
    Ok(Floors { sin, m: [m0, m1] })
}

/// How many pieces each Bézier span of the carrier is cut into for its
/// hulls: a quadratic's control polygon stands off its arc by a quarter
/// of its sagitta's scale, so a piece's hull is the arc's own to the
/// square of the cut.
const PIECES: u32 = 8;

/// **The hulls of the carrier over `params`**: for each span the edge's
/// interval meets, its homogeneous control points `(w·P, w)` crossed into
/// certification arithmetic, and the box of the piece's Cartesian control
/// points, every weight certified positive (a rational curve lies in the
/// convex hull of its span's control points). A span whose ends carry
/// full multiplicity is a Bézier segment, cut into [`PIECES`] by its
/// blossom at dyadic parameters, exact in its own `[0, 1]`; any other
/// span keeps the box of its whole window.
fn control_hulls<T: CertifiedBounds>(
    carrier: &NurbsCurve3<T>,
    params: (T, T),
) -> Option<Vec<Box3>> {
    let (lo, hi) = (Bounds::lo(params.0), Bounds::hi(params.1));
    let p = carrier.degree();
    let homogeneous: Vec<([Interval; 3], Interval)> = carrier
        .control()
        .iter()
        .zip(carrier.weights())
        .map(|(c, &w)| {
            let w = Interval::from_certified(w);
            (scale(point(*c), w), w)
        })
        .collect();
    // Each span is the gap between two consecutive knot runs; its window
    // ends at the last knot of the first run.
    let runs: Vec<(T, usize)> = carrier.knot_values().runs().collect();
    let mut last = 0_usize;
    let mut hulls = Vec::new();
    for pair in runs.windows(2) {
        let ((a, ma), (b, mb)) = (pair[0], pair[1]);
        last += ma;
        let s = last - 1;
        if !(Bounds::hi(b) >= lo && Bounds::lo(a) <= hi) || s < p {
            continue;
        }
        let window = homogeneous.get(s - p..=s)?;
        let bezier = ma >= p && mb >= p;
        let pieces = if bezier { PIECES } else { 1 };
        for j in 0..pieces {
            let net: Vec<([Interval; 3], Interval)> = if bezier {
                let (t0, t1) = (
                    f64::from(j) / f64::from(pieces),
                    f64::from(j + 1) / f64::from(pieces),
                );
                (0..=p)
                    .map(|i| {
                        let mut ts = vec![t0; p - i];
                        ts.extend(vec![t1; i]);
                        blossom(window, &ts)
                    })
                    .collect()
            } else {
                window.to_vec()
            };
            let mut hull: Option<Box3> = None;
            for (pw, w) in net {
                if !(w.is_certified() && w.lo() > 0.0) {
                    return None;
                }
                let c = pw.map(|x| x / w);
                let corner = Box3 {
                    x: c[0],
                    y: c[1],
                    z: c[2],
                };
                hull = Some(hull.map_or(corner, |h| h.hull(corner)));
            }
            let hull = hull?;
            if ![hull.x, hull.y, hull.z].iter().all(|i| i.is_certified()) {
                return None;
            }
            hulls.push(hull);
        }
    }
    (!hulls.is_empty()).then_some(hulls)
}

/// The blossom of a homogeneous Bézier net at `ts` (de Casteljau with
/// one parameter per level), each `t` and `1 − t` exact.
fn blossom(net: &[([Interval; 3], Interval)], ts: &[f64]) -> ([Interval; 3], Interval) {
    let mut row = net.to_vec();
    for &t in ts {
        let (t, u) = (Interval::point(t), Interval::point(1.0 - t));
        row = row
            .windows(2)
            .map(|w| {
                (
                    add(scale(w[0].0, u), scale(w[1].0, t)),
                    w[0].1 * u + w[1].1 * t,
                )
            })
            .collect();
    }
    row[0]
}

/// The carrier's point at a parameter inside `params` for every member
/// of the box, evaluated in certification arithmetic.
// `!(lo < hi)` is deliberate (NaN-catching).
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn witness<T: CertifiedBounds>(carrier: &NurbsCurve3<T>, params: (T, T)) -> Option<[Interval; 3]> {
    let (lo, hi) = (Bounds::hi(params.0), Bounds::lo(params.1));
    if !(lo < hi) {
        return None;
    }
    let lifted = carrier.map_scalar(Interval::from_certified);
    let p = lifted.eval(Interval::point(0.5 * lo + 0.5 * hi));
    let p = [p.x, p.y, p.z];
    p.iter().all(|c| c.is_certified()).then_some(p)
}

/// **A common zero of `Q₁` and `Q₂` within `reach` of every point of
/// `at`** (Poincaré–Miranda, module docs): on the square
/// `y = c + a·u₁ + b·u₂`, `|a|, |b| ≤ A`, `Gu = I` for the two gradients
/// `G` at the centre `c`, `Qᵢ` takes opposite certified signs on the
/// sides `±A` along `uᵢ`. Each side's value is `Qᵢ(c) + ∇Qᵢ(c)·(y − c)`
/// within `qᵢ‖y − c‖²`, exact for a quadric.
fn miranda(a: Quadric, b: Quadric, at: [Interval; 3], reach: f64) -> bool {
    let mid = |i: Interval| 0.5 * i.lo() + 0.5 * i.hi();
    let c = at.map(|i| Interval::point(mid(i)));
    let g = [a.grad(c), b.grad(c)].map(|g| g.map(mid));
    // `u = Gᵀ(GGᵀ)⁻¹`, at `f64`: a preconditioner, so any choice is sound.
    let d = |x: [f64; 3], y: [f64; 3]| x[0] * y[0] + x[1] * y[1] + x[2] * y[2];
    let (g11, g12, g22) = (d(g[0], g[0]), d(g[0], g[1]), d(g[1], g[1]));
    let det = g11 * g22 - g12.powi(2);
    if !(det.is_finite() && det > 0.0) {
        return false;
    }
    let inv = [[g22 / det, -g12 / det], [-g12 / det, g11 / det]];
    let u: [[f64; 3]; 2] =
        [0, 1].map(|j| [0, 1, 2].map(|k| g[0][k] * inv[0][j] + g[1][k] * inv[1][j]));
    let len = |v: [f64; 3]| {
        let v = v.map(Interval::point);
        norm_sq(v).sqrt().hi()
    };
    let side = 0.99 * reach / (len(u[0]) + len(u[1]));
    if !(side.is_finite() && side > 0.0) {
        return false;
    }
    let off = sub(at, c);
    let off_len = norm_sq(off).sqrt();
    let u = u.map(|v| v.map(Interval::point));
    let span = Interval::from_bounds(-side, side);
    let radius = (off_len
        + Interval::point(side) * norm_sq(u[0]).sqrt()
        + Interval::point(side) * norm_sq(u[1]).sqrt())
    .powi(2);
    let mut sign = [0.0_f64; 2];
    for (i, q) in [a, b].into_iter().enumerate() {
        let (value, grad) = (q.value(c), q.grad(c));
        let curve = Interval::point(q.hess()) * radius;
        let along = |k: f64| {
            let lin = dot(grad, off)
                + Interval::point(k * side) * dot(grad, u[i])
                + span * dot(grad, u[1 - i]);
            value + lin + Interval::from_bounds(-1.0, 1.0) * curve
        };
        let (hi_side, lo_side) = (along(1.0), along(-1.0));
        if !(hi_side.is_certified() && lo_side.is_certified()) {
            return false;
        }
        sign[i] = if hi_side.lo() > 0.0 && lo_side.hi() < 0.0 {
            1.0
        } else if hi_side.hi() < 0.0 && lo_side.lo() > 0.0 {
            -1.0
        } else {
            return false;
        };
    }
    sign.iter().all(|&s| s != 0.0)
}
