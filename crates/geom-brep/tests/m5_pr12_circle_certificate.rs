//! The jet certificate's **circle arm** (M5 PR 12): the fillet
//! trimlines the die mints are contact CIRCLES, not lines — the
//! corner ball against its edge cylinders, and the pip-rim torus
//! blend against the flat face and the pip sphere.
//!
//! The contract these rows pin is the equivariance one: in the
//! coaxial configuration (the only configuration in which two
//! distinct elementary surfaces of revolution are tangent along a
//! whole circle) both span bounds are EXACTLY zero, because `κ_rel`
//! and the implicit residual are isometry invariants and the
//! carrier's motion is a symmetry flow of both surfaces. A
//! configuration that misses coaxiality pays continuously — there is
//! no gate, so no tangency is ever demanded that cannot be certified.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::point::p3 as p;
use crate::shared::tol::band;
use geom::Curve3;
use geom::Surface;
use geom_brep::{
    CertCheck, CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, SurfaceKey,
    tangent_certificate_lane, tangent_jet,
};
use geom_core::{Bounds, Decide, Interval, Point3, Real, Vec3};
use slotmap::SlotMap;

/// The corner configuration of the filleted die: the corner ball
/// (radius r, centred on the edge cylinder's axis) and one edge
/// cylinder (radius r). They are tangent along the full circle at
/// the ball's latitude — the octant patch's trimline.
fn corner_pair(r: f64) -> (Surface<f64>, Surface<f64>, Curve3<f64>) {
    let sphere = Surface::Sphere {
        center: p(r, r, r),
        radius: r,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let cylinder = Surface::Cylinder {
        origin: p(r, r, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: r,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let circle = Curve3::Circle {
        center: p(r, r, r),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: r,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    (sphere, cylinder, circle)
}

#[test]
fn circle_carriers_are_in_the_certified_lane_on_every_revolution_kind() {
    let (sphere, cylinder, circle) = corner_pair(0.2);
    let plane = Surface::Plane {
        origin: p(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let torus = Surface::Torus {
        center: p(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        major_radius: 1.0,
        minor_radius: 0.2,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    for s2 in [&sphere, &cylinder, &plane, &torus] {
        assert!(
            tangent_certificate_lane(&circle, &torus, s2),
            "circle carriers admit every surface of revolution"
        );
    }
    // The LINE arm is unchanged: torus stays out of it.
    let line = Curve3::Line {
        origin: p(0.0, 0.0, 0.0),
        dir: Vec3::new(0.0, 0.0, 1.0),
    };
    assert!(!tangent_certificate_lane(&line, &torus, &plane));
    assert!(tangent_certificate_lane(&line, &cylinder, &plane));
    // A carrier kind outside both arms stays outside.
    let ellipse = Curve3::Ellipse {
        center: p(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        major: 2.0,
        minor: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    assert!(!tangent_certificate_lane(&ellipse, &sphere, &plane));
}

/// The die's corner trimline certifies as `TangentIntersection` with
/// the circle carrier — the acceptance shape of the whole arm.
#[test]
fn the_corner_ball_cylinder_circle_certifies_as_a_tangent_intersection() {
    let r = 0.2;
    let (sphere, cylinder, circle) = corner_pair(r);
    let mut surfaces: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let k_sphere = surfaces.insert(sphere.clone());
    let k_cyl = surfaces.insert(cylinder.clone());
    // A QUARTER of the contact circle — one octant patch edge.
    let (t0, t1) = (0.0, core::f64::consts::FRAC_PI_2);
    let start = p(r + r, r, r);
    let end = p(r, r + r, r);
    let witness = p(
        r + r * core::f64::consts::FRAC_PI_4.cos(),
        r + r * core::f64::consts::FRAC_PI_4.sin(),
        r,
    );
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::TangentIntersection {
            s1: k_sphere,
            s2: k_cyl,
            witness,
        },
        carrier: circle,
        param_start: t0,
        param_end: t1,
    };
    let curve = EdgeCurve::certify(spec, start, end, |k| surfaces.get(k).cloned(), band())
        .expect("the corner trimline is a certified tangent intersection");
    assert!(
        curve.certificate().max_residual < 1e-12,
        "the coaxial contact circle is EXACT geometry: {}",
        curve.certificate().max_residual
    );
}

/// The second-order separation the certificate rests on: the sphere
/// curves with radius r transverse to the contact circle while the
/// cylinder is FLAT there (its ruling), so `κ_rel = ±1/r` — bounded
/// away from zero, and constant along the circle by equivariance.
#[test]
fn the_corner_trimline_jet_is_one_over_the_radius_everywhere() {
    let r = 0.2;
    let (sphere, cylinder, circle) = corner_pair(r);
    let mut seen = Vec::new();
    for i in 0..9 {
        let t = core::f64::consts::FRAC_PI_2 * f64::from(i) / 8.0;
        let point = circle.eval(t);
        let tangent = circle.deriv(t);
        let jet = tangent_jet(&sphere, &cylinder, point, tangent);
        assert!(
            jet.sin_theta.abs() < 1e-12,
            "tangency is exact at t = {t}: sin θ = {}",
            jet.sin_theta
        );
        seen.push(jet.kappa_rel);
    }
    for k in &seen {
        assert!(
            (k.abs() - 1.0 / r).abs() < 1e-9,
            "κ_rel must be ±1/r = {} everywhere, got {k}",
            1.0 / r
        );
    }
    // Equivariance, made a row: the jet is CONSTANT along the circle
    // — which is exactly why the certificate's drift bound is zero.
    for k in &seen {
        assert!((k - seen[0]).abs() < 1e-12, "κ_rel drifts along the circle");
    }
}

/// A cylinder of radius r about z and the cap plane z = h crossing it
/// at a RIGHT angle: their intersection is the circle of radius r at
/// height h, and it is transverse — the ruled band's cut-off arc
/// against its cap.
fn cap_crossing(r: f64, h: f64) -> (Surface<f64>, Surface<f64>, Curve3<f64>) {
    let plane = Surface::Plane {
        origin: p(0.0, 0.0, h),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let cylinder = Surface::Cylinder {
        origin: p(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: r,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let circle = Curve3::Circle {
        center: p(0.0, 0.0, h),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: r,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    (plane, cylinder, circle)
}

/// A quarter of `cap_crossing`'s circle at the scalar `T`, described
/// by `describe` over the (plane, cylinder) keys, certified.
fn certify_cap_quarter<T: Decide>(
    r: f64,
    describe: impl Fn(SurfaceKey, SurfaceKey, Point3<T>) -> EdgeDescriptionSpec<T>,
) -> Result<EdgeCurve<T>, CertifyError> {
    let (plane, cylinder, circle) = cap_crossing(r, 0.5);
    let lift = |x: f64| T::from_f64(x);
    let circle = circle.map_scalar(lift);
    let mut surfaces: SlotMap<SurfaceKey, Surface<T>> = SlotMap::with_key();
    let k_plane = surfaces.insert(plane.map_scalar(lift));
    let k_cyl = surfaces.insert(cylinder.map_scalar(lift));
    let (t0, t1) = (T::zero(), lift(core::f64::consts::FRAC_PI_2));
    let witness = circle.eval(lift(core::f64::consts::FRAC_PI_4));
    let spec = EdgeCurveSpec {
        description: describe(k_plane, k_cyl, witness),
        carrier: circle.clone(),
        param_start: t0,
        param_end: t1,
    };
    EdgeCurve::certify(
        spec,
        circle.eval(t0),
        circle.eval(t1),
        |k| surfaces.get(k).cloned(),
        band(),
    )
}

/// The cap crossing described as a tangency at `T`, with the surfaces
/// in the (plane, cylinder) order or reversed.
fn tangent_cap_quarter<T: Decide>(r: f64, reversed: bool) -> Result<EdgeCurve<T>, CertifyError> {
    certify_cap_quarter(r, |plane, cylinder, witness| {
        let (s1, s2) = if reversed {
            (cylinder, plane)
        } else {
            (plane, cylinder)
        };
        EdgeDescriptionSpec::TangentIntersection { s1, s2, witness }
    })
}

/// The refusal a right-angle crossing described as a tangency owes, in
/// either order and at any scalar: its defect is first-order.
const PARALLELISM_DEFECT: CertifyError = CertifyError::ResidualExceeded {
    check: CertCheck::TangentParallel,
    sample: 1,
};

/// **A right-angle crossing described as a tangency is refused at the
/// parallelism check** — D4 ¶1's `sin θ ≤ ε·|κ_rel|`, i.e. the margin
/// `sin θ / |κ_rel|` against the band. Along the cap crossing the
/// normals are perpendicular (`sin θ = 1`) and the jet's lever
/// `1/|κ_rel|` is a length of the pair's own radius scale, so the
/// margin is that length, far above ε.
///
/// The same arc described as what it is (`Intersection`) certifies.
/// With the surfaces the other way round the jet's transverse
/// direction `n̂ × τ̂` is the cylinder's ruling, along which both
/// Hessian forms vanish: `κ_rel = 0`, so no lever `1/κ_rel` exists and
/// the second-order margin refuses. The refusal still names the
/// parallelism defect, metered at the folded arm — an osculating
/// cause would be the wrong one for a 90° crossing.
#[test]
fn a_right_angle_crossing_described_as_a_tangency_is_refused() {
    let r = 0.2;
    let (plane, cylinder, circle) = cap_crossing(r, 0.5);
    for i in 1..8 {
        let t = core::f64::consts::FRAC_PI_2 * f64::from(i) / 8.0;
        let jet = tangent_jet(&plane, &cylinder, circle.eval(t), circle.deriv(t));
        assert!(
            (jet.sin_theta - 1.0).abs() < 1e-12,
            "the normals cross at a right angle: sin θ = {}",
            jet.sin_theta
        );
        let margin = jet.sin_theta / jet.kappa_rel.abs();
        assert!(
            margin.is_finite() && margin >= 0.5 * r,
            "the parallelism margin is a radius-scale length, got {margin} at r = {r}"
        );
        let reversed = tangent_jet(&cylinder, &plane, circle.eval(t), circle.deriv(t));
        assert!(
            reversed.kappa_rel.abs() < 1e-12,
            "reversed, the transverse direction is the ruling and κ_rel vanishes: {}",
            reversed.kappa_rel
        );
    }

    certify_cap_quarter::<f64>(r, |s1, s2, witness| EdgeDescriptionSpec::Intersection {
        s1,
        s2,
        witness,
    })
    .expect("the cap crossing is a certified transverse intersection");

    for reversed in [false, true] {
        assert_eq!(
            tangent_cap_quarter::<f64>(r, reversed).err(),
            Some(PARALLELISM_DEFECT),
            "a right-angle crossing is not a tangency (reversed: {reversed})"
        );
    }
}

/// **The same refusal at `Interval`.** Where the normals are
/// perpendicular, `∇F₂·n̂` is an enclosure straddling zero, so the
/// jet's `σ₂ = 1.copysign(∇F₂·n̂)` hulls to `[−1, 1]` and `κ_rel`'s
/// enclosure straddles zero — the second-order margin is in-band in
/// the (plane, cylinder) order, which refuses definitely in `f64`. The
/// refusal still names the first-order defect, in either order.
#[test]
fn a_right_angle_crossing_described_as_a_tangency_is_refused_at_interval() {
    let r = 0.2;
    let (plane, cylinder, circle) = cap_crossing(r, 0.5);
    let lift = Interval::from_f64;
    let (plane, cylinder) = (plane.map_scalar(lift), cylinder.map_scalar(lift));
    let circle = circle.map_scalar(lift);
    let t = lift(core::f64::consts::FRAC_PI_2 / 8.0);
    let jet = tangent_jet(&plane, &cylinder, circle.eval(t), circle.deriv(t));
    assert!(
        jet.kappa_rel.lo() < 0.0 && jet.kappa_rel.hi() > 0.0,
        "the hulled σ₂ leaves κ_rel's sign undecided: {:?}",
        jet.kappa_rel
    );
    assert!(
        jet.sin_theta.lo() > 0.99,
        "the normals cross at a right angle: sin θ = {:?}",
        jet.sin_theta
    );

    for reversed in [false, true] {
        assert_eq!(
            tangent_cap_quarter::<Interval>(r, reversed).err(),
            Some(PARALLELISM_DEFECT),
            "a right-angle crossing is not a tangency at Interval (reversed: {reversed})"
        );
    }
}
