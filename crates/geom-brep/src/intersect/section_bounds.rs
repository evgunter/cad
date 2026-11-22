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
//! near-miss regions alike. The carrier lies in
//! `O` (limb 2) and is connected, so its distance from `Γ_c` is
//! continuous and never crosses the moat: one point of it within `T`
//! puts all of it within `w`.
//!
//! **The stretch** is `Γ` inside the carrier's control hulls padded by
//! `T`: each span the edge's interval meets has its Bernstein net read
//! off its homogeneous coefficients by the blossom and, where its box
//! could set a floor, is cut into pieces by de Casteljau splits, in
//! certification arithmetic, and a span or a piece lies in the box of
//! its Cartesian control points, so a point of
//! the carrier and its nearest point on `Γ_c` within `T` both lie in one
//! of the padded boxes. Each floor is the least over the boxes. **The
//! one point within `T`** is the carrier's point at an interior
//! parameter, enclosed by evaluating the carrier in certification
//! arithmetic, beside which Poincaré–Miranda
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
//! - **sphere × cylinder**, `δ = O − C` from the sphere's centre to the
//!   cylinder's anchor and `δ⊥ = δ − (δ·â)â`: on `Γ`, `v = u + δ` with
//!   `u = y − O`, `‖v‖ = Rₛ` and the radial part `ρ = u − (u·â)â` of
//!   length `R_c`, so `v·ρ = R_c² + δ⊥·u` and `cos θ = (R_c² +
//!   δ⊥·u)/(Rₛ R_c)`, affine in `y` (constant `R_c/Rₛ` when the centre is
//!   on the axis); `m = 2Rₛ` and `2R_c`.
//!
//! Every other pair — the torus rows among them — states nothing, and
//! owes the tube.

use geom::{NurbsCurve3, Surface, SurfaceKind};
use geom_core::interval::certification::Certification;
use geom_core::spline::algebra::certainly_positive;
use geom_core::spline::pieces::span_row;
use geom_core::{Band, Bounds, CertifiedBounds, Interval, Point3, Vec3};

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
    if !classified(a, b, pair) {
        return Err(pair);
    }
    let eps = band.zero();
    let unread =
        Unstated::Pose("the carrier's control hull over the edge's interval is not certified");
    let spans = control_hulls(carrier, params).ok_or(unread)?;
    // The stretch is the section inside the hulls padded by `T = 16ε/s`,
    // and `s` is the floor over that stretch, so the pad is fixed first:
    // the floor `s₀` over the bare hulls pads them by `32ε/s₀`, and the
    // floor `s` over those padded boxes fits `T` inside the pad exactly
    // when `16ε/s ≤ 32ε/s₀`, that is `s ≥ s₀/2`.
    //
    // Each span is read whole, and cut into its pieces only where its
    // whole box could lower the floors read so far or states none. A span
    // left whole is read at its whole box, which holds it, so the floors
    // stay floors; and that box's floors bound its pieces' from below
    // (the pieces lie in it, up to the splits' rounding), so the floors
    // are those of every span cut. The spans are read least first, so
    // that few are cut.
    let over = |pad: f64| -> Result<Floors, Unstated> {
        let whole: Vec<Result<Floors, Unstated>> = spans
            .iter()
            .map(|span| floors(a, b, span.whole.pad(pad), pair))
            .collect();
        let mut order: Vec<usize> = (0..spans.len()).collect();
        order.sort_by(|&i, &j| {
            let least =
                |f: &Result<Floors, Unstated>| f.as_ref().map_or(f64::NEG_INFINITY, |f| f.sin);
            least(&whole[i]).total_cmp(&least(&whole[j]))
        });
        let mut least = Floors {
            sin: f64::INFINITY,
            m: [f64::INFINITY; 2],
        };
        for i in order {
            let read = match whole[i] {
                Ok(f) if !f.below(&least) => f,
                _ => spans[i].pieces().ok_or(unread)?.iter().try_fold(
                    Floors {
                        sin: f64::INFINITY,
                        m: [f64::INFINITY; 2],
                    },
                    |acc, h| Ok(acc.min(&floors(a, b, h.pad(pad), pair)?)),
                )?,
            };
            least = least.min(&read);
        }
        Ok(least)
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
                - Interval::point(q.hess()) * t.sqr()
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
        origin: V3,
        normal: V3,
    },
    Cylinder {
        origin: V3,
        axis: V3,
        radius: Interval,
    },
    Sphere {
        center: V3,
        radius: Interval,
    },
    Cone {
        apex: V3,
        axis: V3,
        sin_a: Interval,
        cos_a: Interval,
    },
}

/// A point or a direction in certification arithmetic.
type V3 = Vec3<Interval>;

fn point<T: CertifiedBounds>(p: Point3<T>) -> V3 {
    Vec3::new(
        Interval::from_certified(p.x),
        Interval::from_certified(p.y),
        Interval::from_certified(p.z),
    )
}

fn unit<T: CertifiedBounds>(v: Vec3<T>) -> V3 {
    let v = Vec3::new(
        Interval::from_certified(v.x),
        Interval::from_certified(v.y),
        Interval::from_certified(v.z),
    );
    v.normalize()
}

fn boxed(b: Box3) -> V3 {
    Vec3::new(b.x, b.y, b.z)
}

impl Quadric {
    fn of<T: CertifiedBounds>(s: &Surface<T>) -> Option<Self> {
        Some(match *s {
            Surface::Plane { frame, .. } => {
                let (origin, normal) = (frame.origin(), frame.axis());
                Self::Plane {
                    origin: point(origin),
                    normal: unit(normal),
                }
            }
            Surface::Cylinder { frame, radius, .. } => {
                let (origin, axis) = (frame.origin(), frame.axis());
                Self::Cylinder {
                    origin: point(origin),
                    axis: unit(axis),
                    radius: Interval::from_certified(radius),
                }
            }
            Surface::Sphere { frame, radius, .. } => {
                let center = frame.origin();
                Self::Sphere {
                    center: point(center),
                    radius: Interval::from_certified(radius),
                }
            }
            Surface::Cone {
                frame, half_angle, ..
            } => {
                let (apex, axis) = (frame.origin(), frame.axis());
                // Lane evaluation at `T`, crossed in through its bracket
                // as the composite's cone form crosses it: certification
                // calls no transcendental (C9).
                let (sin_a, cos_a) = half_angle.sin_cos();
                Self::Cone {
                    apex: point(apex),
                    axis: unit(axis),
                    sin_a: Interval::from_certified(sin_a),
                    cos_a: Interval::from_certified(cos_a),
                }
            }
            Surface::Torus { .. }
            | Surface::Spindle { .. }
            | Surface::Nurbs(_)
            | Surface::Approx(_) => return None,
        })
    }

    /// `Q` at `y`.
    fn value(self, y: V3) -> Interval {
        match self {
            Self::Plane { origin, normal } => (y - origin).dot(normal),
            Self::Cylinder {
                origin,
                axis,
                radius,
            } => {
                let v = y - origin;
                v.norm_squared() - v.dot(axis).sqr() - radius.sqr()
            }
            Self::Sphere { center, radius } => (y - center).norm_squared() - radius.sqr(),
            Self::Cone {
                apex, axis, cos_a, ..
            } => {
                let v = y - apex;
                cos_a.sqr() * v.norm_squared() - v.dot(axis).sqr()
            }
        }
    }

    /// `∇Q` at `y`.
    fn grad(self, y: V3) -> V3 {
        let two = Interval::point(2.0);
        match self {
            Self::Plane { normal, .. } => normal,
            Self::Cylinder { origin, axis, .. } => {
                let v = y - origin;
                (v - axis * v.dot(axis)) * two
            }
            Self::Sphere { center, .. } => (y - center) * two,
            Self::Cone {
                apex, axis, cos_a, ..
            } => {
                let v = y - apex;
                (v * cos_a.sqr() - axis * v.dot(axis)) * two
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
#[derive(Clone, Copy)]
struct Floors {
    sin: f64,
    m: [f64; 2],
}

impl Floors {
    /// The least of each floor.
    fn min(&self, o: &Self) -> Self {
        Self {
            sin: self.sin.min(o.sin),
            m: [self.m[0].min(o.m[0]), self.m[1].min(o.m[1])],
        }
    }

    /// Whether any floor is below `o`'s.
    fn below(&self, o: &Self) -> bool {
        self.sin < o.sin || self.m[0] < o.m[0] || self.m[1] < o.m[1]
    }
}

/// **The pair's closed forms over `region`** (module docs, each pair's
/// closed forms), the plane first where there is one.
fn floors(a: Quadric, b: Quadric, region: Box3, pair: Unstated) -> Result<Floors, Unstated> {
    let one = Interval::point(1.0);
    let two = Interval::point(2.0);
    let zero = Interval::point(0.0);
    let y = boxed(region);
    let (cos, m): (Interval, [Interval; 2]) = match (a, b) {
        (Quadric::Plane { normal: n1, .. }, Quadric::Plane { normal: n2, .. }) => {
            return finish(n1.cross(n2).norm_squared().sqrt(), [one, one]);
        }
        (
            Quadric::Plane { origin, normal },
            Quadric::Cylinder {
                origin: c,
                axis,
                radius,
            },
        ) => {
            let delta = (c - origin).dot(normal);
            let z = (y - c).dot(axis);
            (
                zero - (delta + z * normal.dot(axis)) / radius,
                [one, two * radius],
            )
        }
        (Quadric::Plane { origin, normal }, Quadric::Sphere { center, radius }) => {
            let delta = (center - origin).dot(normal);
            (zero - delta / radius, [one, two * radius])
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
            let delta = (apex - origin).dot(normal);
            let z = (y - apex).dot(axis);
            if !(z.is_certified() && (z.lo() > 0.0 || z.hi() < 0.0)) {
                return Err(Unstated::Pose(
                    "the carrier's stretch reaches the height of the cone's apex",
                ));
            }
            let side = Interval::point(if z.lo() > 0.0 { 1.0 } else { -1.0 });
            let height = z * side;
            (
                zero - (delta * cos_a.sqr() / height + side * normal.dot(axis)) / sin_a,
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
            if !(same(r1, r2) && same(c1.x, c2.x) && same(c1.y, c2.y) && same(c1.z, c2.z)) {
                return Err(Unstated::Pose(
                    "the cylinders' radii or the point their axes share are not one stored value",
                ));
            }
            let v = y - c1;
            let k = a1.dot(a2);
            let (minus, plus) = (v.dot(a2 - a1), v.dot(a2 + a1));
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
            let zeta = v.dot(a1);
            (one - zeta.sqr() * lean / r1.sqr(), [two * r1, two * r1])
        }
        (
            Quadric::Sphere { center, radius: rs },
            Quadric::Cylinder {
                origin,
                axis,
                radius: rc,
            },
        ) => (
            sphere_cylinder_cos(y, center, rs, origin, axis, rc),
            [two * rs, two * rc],
        ),
        (
            Quadric::Cylinder {
                origin,
                axis,
                radius: rc,
            },
            Quadric::Sphere { center, radius: rs },
        ) => (
            sphere_cylinder_cos(y, center, rs, origin, axis, rc),
            [two * rc, two * rs],
        ),
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

/// Whether [`floors`] has a closed form for the pair, read before the
/// carrier's hulls are composed so that a pair with none costs nothing.
/// It is `floors`' own answer: the pair refusal is its fallback arm, read
/// whatever the region, so `floors` at any one region names exactly the
/// pairs it has no arm for.
fn classified(a: Quadric, b: Quadric, pair: Unstated) -> bool {
    let origin = Interval::point(0.0);
    let region = Box3 {
        x: origin,
        y: origin,
        z: origin,
    };
    !matches!(floors(a, b, region, pair), Err(Unstated::Pair { .. }))
}

/// `cos θ = v̂·ρ̂` of a sphere `(C, Rₛ)` and a cylinder `(O, â, R_c)` at
/// every point `y` of `region` on `Γ` (module docs): affine in `y`, so
/// each datum is read once.
fn sphere_cylinder_cos(
    y: V3,
    center: V3,
    rs: Interval,
    origin: V3,
    axis: V3,
    rc: Interval,
) -> Interval {
    let d = origin - center;
    let normal_part = d - axis * d.dot(axis);
    (rc.sqr() + normal_part.dot(y - origin)) / (rs * rc)
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

/// How many pieces a span is cut into where its whole box could set a
/// floor ([`tube_bounds`]). A piece's control net stands off its arc by
/// an amount that falls with the square of the piece's length (a
/// quadratic's middle control point stands off its arc by half its
/// sagitta), and its box is what the floors are read over, so cutting
/// buys floors that read the carrier rather than its net: the rows of
/// `tests/section_bounds.rs` pin how close they come.
const PIECES: u8 = 8;

/// One span's Bernstein rows: the homogeneous points `w·P` and the
/// weights `w`.
type Rows = (Vec<V3>, Vec<Interval>);

/// A span the edge's interval meets: its rows, the box of its whole net,
/// and its [`PIECES`] pieces' boxes, cut when first asked for.
struct Span {
    rows: Rows,
    whole: Box3,
    pieces: core::cell::OnceCell<Option<Vec<Box3>>>,
}

impl Span {
    /// The boxes of the span's [`PIECES`] pieces, cut by de Casteljau
    /// splits ([`split`]) at equal steps of its parameter; `None` where
    /// a piece's box is not certified.
    fn pieces(&self) -> Option<&[Box3]> {
        self.pieces
            .get_or_init(|| {
                let mut rest = self.rows.clone();
                let mut boxes = Vec::with_capacity(usize::from(PIECES));
                for j in 0..PIECES - 1 {
                    // The piece is the first `1/(PIECES − j)` of what is left.
                    let at = Interval::point(1.0 / f64::from(PIECES - j));
                    let (points, rest_points) = split(&rest.0, at);
                    let (masses, rest_masses) = split(&rest.1, at);
                    boxes.push(hull_of(&(points, masses))?);
                    rest = (rest_points, rest_masses);
                }
                boxes.push(hull_of(&rest)?);
                Some(boxes)
            })
            .as_deref()
    }
}

/// **The spans of the carrier over `params`**: each span the edge's
/// interval meets has its Bernstein net read off its homogeneous
/// coefficients `(w·P, w)` by the blossom
/// ([`geom_core::spline::pieces::span_row`]) over the held knots, in
/// certification arithmetic. A span or a piece of it lies in the convex
/// hull of its Cartesian control points `w·P/w` where every weight is
/// certified positive, so the box of those points holds it for every
/// member of the box. `None` where a span's box is not certified or the
/// interval meets none.
fn control_hulls<T: CertifiedBounds>(
    carrier: &NurbsCurve3<T>,
    params: (T, T),
) -> Option<Vec<Span>> {
    let (lo, hi) = (Bounds::lo(params.0), Bounds::hi(params.1));
    let p = carrier.degree();
    let weights: Vec<Interval> = carrier
        .weights()
        .iter()
        .map(|&w| Interval::from_certified(w))
        .collect();
    let lifted: Vec<V3> = carrier
        .control()
        .iter()
        .zip(&weights)
        .map(|(c, &w)| point(*c) * w)
        .collect();
    let knots: Vec<Interval> = carrier
        .knot_values()
        .values()
        .iter()
        .map(|&k| Interval::from_certified(k))
        .collect();
    // Each span is the gap between two consecutive knot runs; its window
    // ends at the last knot of the first run.
    let runs: Vec<(T, usize)> = carrier.knot_values().runs().collect();
    let mut last = 0_usize;
    let mut spans = Vec::new();
    for pair in runs.windows(2) {
        let ((a, ma), (b, _)) = (pair[0], pair[1]);
        last += ma;
        let s = last - 1;
        if !(Bounds::hi(b) >= lo && Bounds::lo(a) <= hi) || s < p {
            continue;
        }
        let ends = (*knots.get(s)?, *knots.get(s + 1)?);
        let window = s - p..=s;
        let rows = (
            span_row(lifted.get(window.clone())?, &knots, s - p, ends)?,
            span_row(weights.get(window.clone())?, &knots, s - p, ends)?,
        );
        spans.push(Span {
            whole: hull_of(&rows)?,
            rows,
            pieces: core::cell::OnceCell::new(),
        });
    }
    (!spans.is_empty()).then_some(spans)
}

/// The box of a piece's Cartesian control points, `None` where a weight
/// is not certified positive or a bound is not certified.
fn hull_of((points, masses): &Rows) -> Option<Box3> {
    let mut hull: Option<Box3> = None;
    for (&pw, &w) in points.iter().zip(masses) {
        if !certainly_positive(&[w]) {
            return None;
        }
        let c = pw / w;
        let corner = Box3 {
            x: c.x,
            y: c.y,
            z: c.z,
        };
        hull = Some(hull.map_or(corner, |h| h.hull(corner)));
    }
    let hull = hull?;
    [hull.x, hull.y, hull.z]
        .iter()
        .all(|i| i.is_certified())
        .then_some(hull)
}

/// A Bernstein row split at `at` by de Casteljau: the rows of the piece
/// before `at` and of the piece after it. Each is the exact split's at
/// the stored `at`, enclosed, and the two pieces meet there, so together
/// they hold the row's whole curve.
///
/// Not `geom_core`'s `split_restrict`: that one restricts a scalar row to
/// one sub-range by two splits, crate-private to `geom_core`, where a
/// span here is cut whole into its pieces, a vector row and its weights,
/// one split per piece. Its width caution (a split row read unmet is
/// wider than the blossom's) holds here too, and the tightness rows of
/// `tests/section_bounds.rs` pin what the floors lose to it.
fn split<V>(row: &[V], at: Interval) -> (Vec<V>, Vec<V>)
where
    V: Copy
        + core::ops::Add<Output = V>
        + core::ops::Sub<Output = V>
        + core::ops::Mul<Interval, Output = V>,
{
    // Level `r` of the triangle overwrites `after[..len − r]`, leaving
    // each earlier level's last entry in place behind it: what is left
    // is the second piece's row, and each level's first entry the first's.
    let mut after = row.to_vec();
    let mut before = Vec::with_capacity(row.len());
    for top in (0..row.len()).rev() {
        before.push(after[0]);
        for i in 0..top {
            after[i] = after[i] + (after[i + 1] - after[i]) * at;
        }
    }
    (before, after)
}

/// The carrier's point at a parameter inside `params` for every member
/// of the box, evaluated in certification arithmetic.
// `!(lo < hi)` is deliberate (NaN-catching).
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn witness<T: CertifiedBounds>(carrier: &NurbsCurve3<T>, params: (T, T)) -> Option<V3> {
    let (lo, hi) = (Bounds::hi(params.0), Bounds::lo(params.1));
    if !(lo < hi) {
        return None;
    }
    let lifted = carrier.map_scalar(Interval::from_certified);
    let p = lifted.eval(Interval::point(geom::mid_param(lo, hi)));
    [p.x, p.y, p.z]
        .iter()
        .all(|c| c.is_certified())
        .then_some(Vec3::new(p.x, p.y, p.z))
}

/// **A common zero of `Q₁` and `Q₂` within `reach` of every point of
/// `at`** (Poincaré–Miranda, module docs): on the square
/// `y = c + a·u₁ + b·u₂`, `|a|, |b| ≤ A`, `Gu = I` for the two gradients
/// `G` at the centre `c`, `Qᵢ` takes opposite certified signs on the
/// sides `±A` along `uᵢ`. Each side's value is `Qᵢ(c) + ∇Qᵢ(c)·(y − c)`
/// within `qᵢ‖y − c‖²`, exact for a quadric.
fn miranda(a: Quadric, b: Quadric, at: V3, reach: f64) -> bool {
    let mid = |i: Interval| geom::mid_param(i.lo(), i.hi());
    let c = Vec3::new(
        Interval::point(mid(at.x)),
        Interval::point(mid(at.y)),
        Interval::point(mid(at.z)),
    );
    let g = [a.grad(c), b.grad(c)].map(|g| Vec3::new(mid(g.x), mid(g.y), mid(g.z)));
    // `u = Gᵀ(GGᵀ)⁻¹`, at `f64`: a preconditioner, so any choice is sound.
    let (g11, g12, g22) = (g[0].dot(g[0]), g[0].dot(g[1]), g[1].dot(g[1]));
    let det = g11 * g22 - g12.powi(2);
    if !(det.is_finite() && det > 0.0) {
        return false;
    }
    let u = [
        (g[0] * g22 - g[1] * g12) / det,
        (g[1] * g11 - g[0] * g12) / det,
    ]
    .map(|v| {
        Vec3::new(
            Interval::point(v.x),
            Interval::point(v.y),
            Interval::point(v.z),
        )
    });
    let len = |v: V3| v.norm_squared().sqrt();
    let side = 0.99 * reach / (len(u[0]).hi() + len(u[1]).hi());
    if !(side.is_finite() && side > 0.0) {
        return false;
    }
    let off = at - c;
    let span = Interval::from_bounds(-side, side);
    let radius =
        (len(off) + Interval::point(side) * len(u[0]) + Interval::point(side) * len(u[1])).sqr();
    for (i, q) in [a, b].into_iter().enumerate() {
        let (value, grad) = (q.value(c), q.grad(c));
        let curve = Interval::point(q.hess()) * radius;
        let along = |k: f64| {
            let lin = grad.dot(off)
                + Interval::point(k * side) * grad.dot(u[i])
                + span * grad.dot(u[1 - i]);
            value + lin + Interval::from_bounds(-1.0, 1.0) * curve
        };
        let (hi_side, lo_side) = (along(1.0), along(-1.0));
        if !(hi_side.is_certified() && lo_side.is_certified()) {
            return false;
        }
        let opposite = (hi_side.lo() > 0.0 && lo_side.hi() < 0.0)
            || (hi_side.hi() < 0.0 && lo_side.lo() > 0.0);
        if !opposite {
            return false;
        }
    }
    true
}
