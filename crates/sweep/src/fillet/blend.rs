//! **The analytic blend arms** (C8's D3 payoff): the constant-radius
//! rolling ball over analytic supports lands on analytic surfaces in
//! exactly the cases that dominate practice, and those are the cases
//! this unit ships — nothing is fitted, nothing is approximated.
//!
//! | support pair | spine | blend | trimlines |
//! |---|---|---|---|
//! | plane–plane | line | cylinder | two lines |
//! | plane–sphere (a circular rim) | circle | torus | two circles |
//! | trihedral vertex (3 planes) | point | sphere | three circles |
//!
//! Every arm returns its blend surface, its spine, and the EXACT
//! setback on each support — and the battery consumes the setback
//! from HERE, not from a parallel first-order estimate. That is what
//! makes the ordering claim honest: the quantity predicate 2 refuses
//! on is bit-for-bit the quantity the constructor would use.
//!
//! **Sense, from stored structure** (S10/S11): a blend face's chart
//! normal agrees with the material's outward normal exactly when the
//! chain is CONVEX — the rolling ball is then inside the material and
//! the blend bulges away from it, so the chart normal (radially out
//! of the tube/ball) is the outward one. On a CONCAVE chain the ball
//! rolls in the void, material sits outside the tube, and the sense
//! bit is `false`. The bit is read off the chain's stored convexity
//! verdict — never from a sampled normal.
//!
//! Everything a general spine would need — a canal surface, the
//! kernel's first approximating SURFACE — is refused typed by
//! [`super::FilletError::SpineUnsupported`], which names that banked
//! unit as the front door that does not exist yet.

use geom_core::{Point3, Real, Vec3};
use geom_curves::Curve3;
use geom_surfaces::Surface;

/// A blend surface with its spine and its per-support trimlines —
/// the complete analytic answer for one link of a chain.
#[derive(Clone, Debug)]
pub struct EdgeBlend<T: Real> {
    /// The blend surface itself (a cylinder or a torus).
    pub surface: Surface<T>,
    /// The spine's curvature, 1/meters: `0` for a straight spine,
    /// `1/s` for a circular one. Predicate 3 reads this.
    pub spine_curvature: T,
    /// The trimline on the FIRST support, and the setback (meters)
    /// from the original edge to it, measured in that support.
    pub trim_a: (Curve3<T>, T),
    /// The trimline on the SECOND support, likewise.
    pub trim_b: (Curve3<T>, T),
}

/// The corner ball of a three-convex-edge vertex: a sphere patch (the
/// spherical triangle bounded by the three contact circles with the
/// three incident edge cylinders).
#[derive(Clone, Debug)]
pub struct CornerBall<T: Real> {
    /// The sphere the octant patch lies on.
    pub surface: Surface<T>,
    /// The ball centre — the point at distance `r` inside all three
    /// support planes.
    pub center: Point3<T>,
    /// `|det(n₁, n₂, n₃)|` for the three outward support normals —
    /// the independence margin predicate 6 classifies (times the
    /// radius, which the battery supplies as the lever arm).
    pub independence: T,
}

/// The rim blend of a plane meeting a sphere along a circle — the pip
/// rims. Named separately from [`EdgeBlend`] only in the docs: it is
/// produced by [`plane_sphere_blend`] and returned as an
/// [`EdgeBlend`] like every other arm.
pub type RimBlend<T> = EdgeBlend<T>;

/// Which analytic arm a link took — carried on the link so the report
/// and the tests can enumerate coverage without re-deriving it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendArm {
    /// Plane–plane edge → cylinder patch, straight spine.
    PlanePlaneCylinder,
    /// Plane–sphere circular rim → torus patch, circular spine.
    PlaneSphereTorus,
}

impl BlendArm {
    /// The arm's name, for refusal text and report rows.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::PlanePlaneCylinder => "plane–plane → cylinder",
            Self::PlaneSphereTorus => "plane–sphere → torus",
        }
    }
}

/// The unit component of `x` orthogonal to the unit direction `a` —
/// the seam reference every chart below takes from real geometry the
/// caller already stored (a support's own `u_ref`, a corner normal),
/// never from a coordinate-axis tie-break. Poison propagates when `x`
/// is parallel to `a`, which is the honest answer: there is no
/// distinguished perpendicular there.
fn perp_unit<T: Real>(x: Vec3<T>, a: Vec3<T>) -> Vec3<T> {
    // Named binding so the interval-square tripwire's self-multiplication
    // grep does not false-positive on `a * a.dot(x)` (a vector scaled by a
    // projection coefficient, not a scalar square) — the affine.rs
    // restructuring precedent; no numeric change.
    let along = a.dot(x);
    (x - a * along).normalize()
}

/// **The plane–plane edge blend**: the rolling ball of radius `r`
/// tangent to both planes, rolled along their common edge.
///
/// With outward normals `n_a`, `n_b` and any point `p` on the edge,
/// the ball centre solves `(c − p)·n_a = (c − p)·n_b = −r` (inside
/// the material by `r` from each support), giving the closed form
/// `c = p − r·(n_a + n_b)/(1 + n_a·n_b)` — the spine, a straight line
/// along the edge direction `tau`. The blend is the cylinder about
/// that line with radius `r`; the trimline on each support is the
/// ruling through the ball's foot on that plane, and the setback is
/// `r·√((1 − d)/(1 + d))` with `d = n_a·n_b` (equivalently
/// `r·tan(φ/2)` for the outward-normal angle φ) — derived here once
/// and consumed by predicate 2.
///
/// `convex` picks the side: on a concave chain the ball rolls on the
/// far side of both supports, which is the same formula with `r`
/// negated.
#[must_use]
pub fn plane_plane_blend<T: Real>(
    p: Point3<T>,
    tau: Vec3<T>,
    n_a: Vec3<T>,
    n_b: Vec3<T>,
    radius: T,
    convex: bool,
) -> EdgeBlend<T> {
    let signed = if convex { radius } else { -radius };
    let d = n_a.dot(n_b);
    let one = T::one();
    let center = p - (n_a + n_b) * (signed / (one + d));
    let foot_a = center + n_a * signed;
    let foot_b = center + n_b * signed;
    let setback = radius * ((one - d) / (one + d)).sqrt();
    EdgeBlend {
        surface: Surface::Cylinder {
            origin: center,
            axis: tau,
            radius,
            u_ref: (foot_a - center).normalize(),
        },
        spine_curvature: T::zero(),
        trim_a: (
            Curve3::Line {
                origin: foot_a,
                dir: tau,
            },
            setback,
        ),
        trim_b: (
            Curve3::Line {
                origin: foot_b,
                dir: tau,
            },
            setback,
        ),
    }
}

/// **The plane–sphere rim blend** (the pip rims): a plane with
/// outward normal `n` through `origin`, meeting a sphere of radius
/// `sphere_r` centred at `sphere_c` along a circle.
///
/// The ball centre sits at depth `r` below the plane and at distance
/// `sphere_r + r` from the sphere centre (the material is outside the
/// pip ball), so the spine is the CIRCLE of radius
/// `s = √((R + r)² − h²)`, `h = (c_s − o)·n + r`, in the offset plane
/// — and the blend is the torus about it. Both trimlines are circles
/// coaxial with the spine: radius `s` on the plane (a genuine
/// widening, `s > a` for the rim radius `a`, which is what makes the
/// blend eat into the flat face rather than into the pocket), and
/// radius `R·s/(R + r)` on the sphere. A configuration with no ring
/// torus (`s ≤ r`) is NOT gated here: it yields a poisoned spine
/// curvature, which predicate 3 escalates with an `Invalid` margin —
/// refused before the surface is ever used.
#[must_use]
pub fn plane_sphere_blend<T: Real>(
    origin: Point3<T>,
    n: Vec3<T>,
    plane_u_ref: Vec3<T>,
    sphere_c: Point3<T>,
    sphere_r: T,
    radius: T,
) -> EdgeBlend<T> {
    let depth = (sphere_c - origin).dot(n);
    let h = depth + radius;
    let s2 = (sphere_r + radius).powi(2) - h.powi(2);
    // No gate here: a configuration with no spine circle yields a
    // POISONED `s`, which flows into `spine_curvature` and escalates
    // at predicate 3 with an `Invalid` margin. Total arithmetic in,
    // classification at the caller — the crate's standing posture.
    let s = s2.sqrt();
    let spine_center = sphere_c - n * h;
    let u_ref = perp_unit(plane_u_ref, n);
    // The rim circle the blend replaces: the sphere ∩ plane locus.
    let rim2 = sphere_r.powi(2) - depth.powi(2);
    let rim = rim2.max(T::zero()).sqrt();
    // The contact circle on the sphere: the sphere point along the
    // line from its centre to the spine, scaled to radius R.
    let scale = sphere_r / (sphere_r + radius);
    let sphere_trim_c = sphere_c - n * (h * scale);
    let sphere_trim_r = s * scale;
    // Setback on the sphere: the EUCLIDEAN displacement of the
    // trimline from the rim. Predicate 2 compares it against face
    // extents measured the same way, so the two sides of that
    // inequality are in one metric — an arc-length setback against a
    // straight-line extent would be comparing different quantities.
    let rim_pt = sphere_c - n * depth + u_ref * rim;
    let con_pt = sphere_trim_c + u_ref * sphere_trim_r;
    EdgeBlend {
        surface: Surface::Torus {
            center: spine_center,
            axis: n,
            major_radius: s,
            minor_radius: radius,
            u_ref,
        },
        spine_curvature: T::one() / s,
        trim_a: (
            Curve3::Circle {
                center: spine_center + n * radius,
                axis: n,
                radius: s,
                u_ref,
            },
            s - rim,
        ),
        trim_b: (
            Curve3::Circle {
                center: sphere_trim_c,
                axis: n,
                radius: sphere_trim_r,
                u_ref,
            },
            (con_pt - rim_pt).norm(),
        ),
    }
}

/// **The corner ball** of a three-convex-edge vertex: the sphere of
/// radius `r` tangent to all three support planes from inside the
/// material.
///
/// The centre solves `(c − p_i)·n_i = −r` for the three outward
/// normals; the solution is `c = p + N⁻¹·(…)` written here as the
/// Cramer expansion so `|det(n₁, n₂, n₃)|` — the independence margin
/// predicate 6 classifies — falls out of the same computation.
///
/// The patch is bounded by three CIRCLES, one per incident edge
/// cylinder: the ball centre lies ON each cylinder's axis (the axis
/// is the locus at distance `r` from two of the planes, and the
/// centre is at distance `r` from all three), and both have radius
/// `r`, so sphere and cylinder are tangent along a full circle. That
/// is the configuration the jet certificate's circle arm certifies,
/// and its `κ_rel` is `1/r` — the sphere curves transverse to the
/// contact circle where the cylinder is flat along its ruling.
#[must_use]
pub fn corner_ball<T: Real>(
    verts: [Point3<T>; 3],
    normals: [Vec3<T>; 3],
    radius: T,
    convex: bool,
) -> CornerBall<T> {
    let [n1, n2, n3] = normals;
    let signed = if convex { -radius } else { radius };
    // Right-hand sides: c·n_i = p_i·n_i + signed.
    let rhs = [
        Vec3::new(verts[0].x, verts[0].y, verts[0].z).dot(n1) + signed,
        Vec3::new(verts[1].x, verts[1].y, verts[1].z).dot(n2) + signed,
        Vec3::new(verts[2].x, verts[2].y, verts[2].z).dot(n3) + signed,
    ];
    let det = n1.dot(n2.cross(n3));
    // Cramer: c = (rhs₁·(n₂×n₃) + rhs₂·(n₃×n₁) + rhs₃·(n₁×n₂)) / det.
    let num = n2.cross(n3) * rhs[0] + n3.cross(n1) * rhs[1] + n1.cross(n2) * rhs[2];
    let c = Point3::new(num.x / det, num.y / det, num.z / det);
    CornerBall {
        surface: Surface::Sphere {
            center: c,
            radius,
            axis: (n1 + n2 + n3).normalize(),
            u_ref: perp_unit(n1, (n1 + n2 + n3).normalize()),
        },
        center: c,
        independence: det.abs(),
    }
}

/// The contact circle between the corner ball and one incident edge
/// cylinder: centred at the ball centre, of radius `r`, in the plane
/// through the centre normal to the cylinder axis (the ball centre
/// lies on that axis, so the circle is the ball's equator there).
#[must_use]
pub fn corner_contact_circle<T: Real>(
    ball: &CornerBall<T>,
    axis: Vec3<T>,
    radius: T,
    u_ref: Vec3<T>,
) -> Curve3<T> {
    Curve3::Circle {
        center: ball.center,
        axis,
        radius,
        u_ref,
    }
}
