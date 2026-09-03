//! The analytic offset door (`geom_brep::offset_surface`): closed-form
//! unit rows for the mint table, its round trips, and its refusals.
//!
//! Three families:
//!
//! - **Round trips** — `offset(offset(S, d), −d)` reproduces `S`
//!   bit-exactly at dyadic fixtures for plane/cylinder/sphere/torus
//!   (the mint is pure parameter arithmetic both ways, so bit identity
//!   holds exactly where IEEE addition is exact — the fixtures are
//!   chosen dyadic to make it so), and for the cone to one rounded
//!   operation per leg (two per round trip): the apex coordinate
//!   add/subtract pair `fl(fl(a − t) + t)` (the slide `t` itself is
//!   bit-identical both ways, since `(−d)/sin α = −(d/sin α)` and
//!   per-component products negate exactly).
//! - **Defining-equation rows** — independent per-kind closed-form
//!   signed-distance spellings (never the mint's algebra): the normal
//!   pushforward of the base lands ON the minted locus, and sampled
//!   points OF the minted locus lie at signed distance exactly `d`
//!   from the base. Both `d` signs per kind, both cone nappe-side
//!   behaviors via both slide signs.
//! - **Planted reds** — each refusal red before the fix would be
//!   green: the radius floor at zero and below, the floor on the
//!   REALIZED radius at a large-scale fixture (the collapse regime an
//!   exact-real check would wave through), the torus ring convention,
//!   the NURBS non-closure, and honest in-band escalation at f64 and
//!   at the interval scalar.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_6, PI};

use crate::shared::tol::band;
use geom::Surface;
use geom_brep::{OffsetError, SurfaceKind, offset_surface};
use geom_core::{Point3, Tol, Vec3};

// ---------------------------------------------------------------------
// Fixtures: the exactly orthonormal Pythagorean frame (components in
// thirds — orthonormality holds to a few ulps) for the defining-
// equation rows, and axis-aligned dyadic fixtures for the bit-exact
// round trips.
// ---------------------------------------------------------------------

fn t_axis() -> Vec3<f64> {
    Vec3::new(2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0)
}

fn t_uref() -> Vec3<f64> {
    Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0)
}

fn t_center() -> Point3<f64> {
    Point3::new(-0.5, 4.0, 1.25)
}

/// The five analytic kinds on the tilted frame, with radii/angles
/// roomy enough that both `d = ±0.3` offsets mint.
fn tilted_surfaces() -> Vec<(&'static str, Surface<f64>)> {
    vec![
        (
            "plane",
            Surface::Plane {
                origin: t_center(),
                normal: t_axis(),
                u_ref: t_uref(),
            },
        ),
        (
            "cylinder",
            Surface::Cylinder {
                origin: t_center(),
                axis: t_axis(),
                radius: 2.5,
                u_ref: t_uref(),
            },
        ),
        (
            "cone",
            Surface::Cone {
                apex: t_center(),
                axis: t_axis(),
                half_angle: FRAC_PI_6,
                u_ref: t_uref(),
            },
        ),
        (
            "sphere",
            Surface::Sphere {
                center: t_center(),
                radius: 2.5,
                axis: t_axis(),
                u_ref: t_uref(),
            },
        ),
        (
            "torus",
            Surface::Torus {
                center: t_center(),
                axis: t_axis(),
                major_radius: 4.0,
                minor_radius: 1.0,
                u_ref: t_uref(),
            },
        ),
    ]
}

// ---------------------------------------------------------------------
// Independent signed-distance spellings, one per kind — closed forms
// over the stored fields, NOT the mint's parameter arithmetic. Each is
// the exact signed distance from the point to the surface, positive on
// the chart normal's side.
// ---------------------------------------------------------------------

/// Plane: the coordinate along the unit normal.
fn plane_dist(origin: Point3<f64>, normal: Vec3<f64>, q: Point3<f64>) -> f64 {
    (q - origin).dot(normal)
}

/// Sphere: distance to the center, less the radius.
fn sphere_dist(center: Point3<f64>, r: f64, q: Point3<f64>) -> f64 {
    (q - center).norm() - r
}

/// Cylinder: distance to the axis line, less the radius.
fn cylinder_dist(origin: Point3<f64>, axis: Vec3<f64>, r: f64, q: Point3<f64>) -> f64 {
    let rel = q - origin;
    let h = rel.dot(axis);
    (rel - axis * h).norm() - r
}

/// Torus: distance to the core circle (radius R about the axis), less
/// the minor radius.
fn torus_dist(center: Point3<f64>, axis: Vec3<f64>, big_r: f64, r: f64, q: Point3<f64>) -> f64 {
    let rel = q - center;
    let h = rel.dot(axis);
    let rho = (rel - axis * h).norm();
    ((rho - big_r).powi(2) + h * h).sqrt() - r
}

/// Cone (`v > 0` nappe, points with positive axial height): the
/// perpendicular distance to the generator line in the meridian
/// half-plane through the point, positive outside the nappe.
fn cone_dist(apex: Point3<f64>, axis: Vec3<f64>, alpha: f64, q: Point3<f64>) -> f64 {
    let rel = q - apex;
    let h = rel.dot(axis);
    let rho = (rel - axis * h).norm();
    rho * alpha.cos() - h * alpha.sin()
}

/// The signed distance of `q` from an analytic surface, dispatched to
/// the closed-form spelling for its kind.
fn signed_dist(s: &Surface<f64>, q: Point3<f64>) -> f64 {
    match *s {
        Surface::Plane { origin, normal, .. } => plane_dist(origin, normal, q),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => cylinder_dist(origin, axis, radius, q),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => cone_dist(apex, axis, half_angle, q),
        Surface::Sphere { center, radius, .. } => sphere_dist(center, radius, q),
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => torus_dist(center, axis, major_radius, minor_radius, q),
        // No closed-form distance for a spline or its offset.
        Surface::Nurbs(_) | Surface::Approx(_) => f64::NAN,
    }
}

/// Sample parameters per kind: azimuths crossing the seam and both
/// hemispheres, `v` rows chosen inside each kind's honest chart region
/// (`v > 0` cone nappe away from the apex even after a `∓0.3·cot α`
/// parameter shift; sphere latitudes away from the poles).
fn samples(name: &str) -> (Vec<f64>, Vec<f64>) {
    let u = vec![0.0, 0.9, 2.4, -1.8, PI];
    let v = match name {
        "plane" => vec![-2.0, 0.0, 1.7],
        "cylinder" => vec![-1.5, 0.0, 2.25],
        "cone" => vec![0.8, 1.5, 3.0],
        "sphere" => vec![-1.0, 0.0, 0.7],
        "torus" => vec![0.0, 1.1, -2.5, PI],
        other => panic!("unknown kind {other}"),
    };
    (u, v)
}

/// All scalar fields of an analytic surface as IEEE bit patterns, for
/// bit-exact round-trip assertions.
fn bits(s: &Surface<f64>) -> Vec<u64> {
    fn p(out: &mut Vec<u64>, pt: Point3<f64>) {
        out.extend([pt.x.to_bits(), pt.y.to_bits(), pt.z.to_bits()]);
    }
    fn v(out: &mut Vec<u64>, w: Vec3<f64>) {
        out.extend([w.x.to_bits(), w.y.to_bits(), w.z.to_bits()]);
    }
    let mut out = Vec::new();
    match *s {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => {
            p(&mut out, origin);
            v(&mut out, normal);
            v(&mut out, u_ref);
        }
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => {
            p(&mut out, origin);
            v(&mut out, axis);
            out.push(radius.to_bits());
            v(&mut out, u_ref);
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => {
            p(&mut out, apex);
            v(&mut out, axis);
            out.push(half_angle.to_bits());
            v(&mut out, u_ref);
        }
        Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => {
            p(&mut out, center);
            out.push(radius.to_bits());
            v(&mut out, axis);
            v(&mut out, u_ref);
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => {
            p(&mut out, center);
            v(&mut out, axis);
            out.push(major_radius.to_bits());
            out.push(minor_radius.to_bits());
            v(&mut out, u_ref);
        }
        Surface::Nurbs(_) | Surface::Approx(_) => {
            panic!("no bit flattening for the spline kinds")
        }
    }
    out
}

// ---------------------------------------------------------------------
// Round trips
// ---------------------------------------------------------------------

/// Plane/cylinder/sphere/torus: `offset(offset(S, d), −d)` is `S`
/// bit-exactly, both `d` signs. Fixtures are dyadic (`radius 2.5`,
/// `minor 1.0`, `d ±0.25`, axis-aligned plane at dyadic origin) so the
/// parameter sums round nowhere and bit identity is the honest claim.
#[test]
fn round_trips_bit_exact_for_radius_kinds_and_plane() {
    let plane = Surface::Plane {
        origin: Point3::new(0.5, -2.0, 1.25),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let mut rows = vec![("plane", plane)];
    rows.extend(tilted_surfaces().into_iter().filter(|(n, _)| {
        // The tilted radius kinds round-trip bit-exactly too: only the
        // stored radius changes, by a dyadic ±0.25.
        matches!(*n, "cylinder" | "sphere" | "torus")
    }));
    for (name, base) in rows {
        for d in [0.25, -0.25] {
            let there = offset_surface(&base, d, band()).unwrap();
            let back = offset_surface(&there, -d, band()).unwrap();
            assert_eq!(
                bits(&back),
                bits(&base),
                "{name} round trip at d = {d} is not bit-exact"
            );
        }
    }
}

/// Cone round trip, apex axial coordinate zero: bit-exact even though
/// the slide is a rounded quotient, because `0 − t` and `−t + t` are
/// exact IEEE operations and the reverse slide is the exact negation
/// of the forward one (`(−d)/sin α = −(d/sin α)`; the per-component
/// products negate exactly). Both slide signs.
#[test]
fn cone_round_trip_bit_exact_at_zero_apex_height() {
    let base = Surface::Cone {
        apex: Point3::new(0.5, -2.0, 0.0),
        axis: Vec3::unit_z(),
        half_angle: FRAC_PI_6,
        u_ref: Vec3::unit_x(),
    };
    for d in [0.25, -0.25] {
        let there = offset_surface(&base, d, band()).unwrap();
        let back = offset_surface(&there, -d, band()).unwrap();
        assert_eq!(bits(&back), bits(&base), "cone round trip at d = {d}");
    }
}

/// Cone round trip, generic apex: every carried field is bit-exact;
/// the apex is reproduced to one rounded operation per leg (two per
/// round trip) — the apex coordinate add/subtract pair
/// `fl(fl(a − t) + t)` with the same `t` both ways. Both slide signs.
#[test]
fn cone_round_trip_generic_apex_rounds_once_per_leg() {
    let base = Surface::Cone {
        apex: Point3::new(0.5, -2.0, 1.25),
        axis: Vec3::unit_z(),
        half_angle: FRAC_PI_6,
        u_ref: Vec3::unit_x(),
    };
    let slide = 0.25 / FRAC_PI_6.sin();
    for d in [0.25, -0.25] {
        let there = offset_surface(&base, d, band()).unwrap();
        let back = offset_surface(&there, -d, band()).unwrap();
        let (Surface::Cone { apex: b, .. }, Surface::Cone { apex: t, .. }) = (&back, &there) else {
            panic!("cone offsets must stay cones");
        };
        // x, y ride along bit-exactly (axis is ẑ: their slide
        // component is exactly zero), as do the carried fields.
        assert_eq!(b.x.to_bits(), 0.5f64.to_bits());
        assert_eq!(b.y.to_bits(), (-2.0f64).to_bits());
        assert_eq!(
            bits(&back)[3..],
            bits(&base)[3..],
            "carried fields at d = {d}"
        );
        // z: one rounded addition each way.
        let bound = 4.0 * f64::EPSILON * (1.25 + slide.abs());
        assert!(
            (b.z - 1.25).abs() <= bound,
            "apex height after round trip at d = {d}: {} (bound {bound})",
            b.z
        );
        // And the intermediate really did move: the slide is along
        // −axis for d > 0, +axis for d < 0 (the sign is the chart
        // normal's axial coefficient −sin α — the closed-form row).
        let expected = 1.25 - d.signum() * slide;
        assert!(
            (t.z - expected).abs() <= bound,
            "slid apex height at d = {d}: {} vs {expected}",
            t.z
        );
    }
}

// ---------------------------------------------------------------------
// Defining-equation rows
// ---------------------------------------------------------------------

/// For every kind and both `d` signs: (a) the base's normal
/// pushforward `S(u,v) + d·n(u,v)` lies ON the minted offset (its
/// independent signed distance from the mint is 0), and (b) points
/// sampled ON the minted offset lie at signed distance exactly `d`
/// from the base — the offset's defining equation, spelled through the
/// closed-form distances above rather than the mint's algebra.
#[test]
fn defining_equation_rows_all_kinds_both_signs() {
    let tol = 1e-12;
    for (name, base) in tilted_surfaces() {
        for d in [0.3, -0.3] {
            let minted = offset_surface(&base, d, band()).unwrap();
            let (us, vs) = samples(name);
            for &u in &us {
                for &v in &vs {
                    // (a) pushforward lands on the mint.
                    let p = base.eval(u, v);
                    let n = base.normal(u, v);
                    let q = p + n * d;
                    let dist_minted = signed_dist(&minted, q);
                    assert!(
                        dist_minted.abs() < tol,
                        "{name} d={d} (u,v)=({u},{v}): pushforward off the mint by {dist_minted}"
                    );
                    // (b) the mint's own samples sit at distance d
                    // from the base, ON THE NORMAL SIDE (the signed
                    // spelling certifies the side, not just |d|).
                    let q2 = minted.eval(u, v);
                    let dist_base = signed_dist(&base, q2);
                    assert!(
                        (dist_base - d).abs() < tol,
                        "{name} d={d} (u,v)=({u},{v}): mint sample at {dist_base} from base"
                    );
                }
            }
        }
    }
}

/// The cone slide's closed form, both signs, at an axis-aligned
/// fixture where every quantity is hand-checkable: α = π/6, so the
/// slide magnitude is d/sin α ≈ 2d, and the apex moves along −axis
/// for d > 0 (the chart normal's axial coefficient is −sin α < 0),
/// along +axis for d < 0. The generator's foot distance row in
/// `defining_equation_rows_all_kinds_both_signs` proves the resulting
/// surface is the true offset; this row pins the closed form itself.
#[test]
fn cone_slide_closed_form_both_signs() {
    let base = Surface::Cone {
        apex: Point3::new(0.0, 0.0, 2.0),
        axis: Vec3::unit_z(),
        half_angle: FRAC_PI_6,
        u_ref: Vec3::unit_x(),
    };
    let s = FRAC_PI_6.sin();
    for d in [0.5, -0.5] {
        let minted = offset_surface(&base, d, band()).unwrap();
        let Surface::Cone {
            apex, half_angle, ..
        } = minted
        else {
            panic!("cone offset must be a cone");
        };
        assert_eq!(half_angle.to_bits(), FRAC_PI_6.to_bits());
        assert_eq!(apex.x.to_bits(), 0.0f64.to_bits());
        assert_eq!(apex.y.to_bits(), 0.0f64.to_bits());
        let expected = 2.0 - d / s;
        assert!(
            (apex.z - expected).abs() <= 4.0 * f64::EPSILON * expected.abs(),
            "apex z at d={d}: {} vs {expected}",
            apex.z
        );
        // Direction, stated as an inequality so the row cannot pass
        // on a sign error that happens to land near the magnitude.
        if d > 0.0 {
            assert!(apex.z < 2.0, "positive d slides along −axis");
        } else {
            assert!(apex.z > 2.0, "negative d slides along +axis");
        }
    }
}

// ---------------------------------------------------------------------
// Planted reds: the refusals
// ---------------------------------------------------------------------

/// **Deliberately not `shared::surf`.** Both this and `torus` below
/// stand on this suite's own tilted frame (`t_center`/`t_axis`/
/// `t_uref`), not the canonical one: an offset that is right only in
/// an axis-aligned frame is the defect these rows exist to catch.
fn cyl(radius: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin: t_center(),
        axis: t_axis(),
        radius,
        u_ref: t_uref(),
    }
}

fn torus(major: f64, minor: f64) -> Surface<f64> {
    Surface::Torus {
        center: t_center(),
        axis: t_axis(),
        major_radius: major,
        minor_radius: minor,
        u_ref: t_uref(),
    }
}

/// The radius floor: collapse to the axis/center/spine refuses at
/// exactly zero (a coincident-with-zero realized radius is never
/// minted) and below, for all three radius kinds.
#[test]
fn radius_floor_refuses_at_and_below_zero() {
    let sphere = Surface::Sphere {
        center: t_center(),
        radius: 2.0,
        axis: t_axis(),
        u_ref: t_uref(),
    };
    let rows: Vec<(&str, Surface<f64>, f64)> = vec![
        ("cylinder at zero", cyl(1.0), -1.0),
        ("cylinder below", cyl(1.0), -1.5),
        ("sphere at zero", sphere.clone(), -2.0),
        ("sphere below", sphere, -2.25),
        ("torus minor at zero", torus(2.0, 1.0), -1.0),
        ("torus minor below", torus(2.0, 1.0), -1.25),
    ];
    for (name, s, d) in rows {
        assert!(
            matches!(
                offset_surface(&s, d, band()),
                Err(OffsetError::RadiusFloor { .. })
            ),
            "{name} (d = {d}) must refuse on the radius floor"
        );
    }
}

/// The TUBEWALL collapse-regime lesson as a test: at radius 1e16 the
/// f64 spacing is 2.0, so `d = −(1e16 − 0.5)` IS `−1e16` after
/// rounding and the realized radius the mint would store is exactly
/// 0.0 — collapsed — even though the exact-real margin `radius + d =
/// 0.5 m` is comfortably positive at every run tolerance. A door that
/// metered the intent numbers would mint a zero-radius cylinder here;
/// this door meters the realized radius and refuses.
#[test]
fn radius_floor_meters_the_realized_radius_at_scale() {
    let d = -(1.0e16_f64 - 0.5);
    assert_eq!(
        d.to_bits(),
        (-1.0e16f64).to_bits(),
        "the fixture's rounding premise"
    );
    match offset_surface(&cyl(1.0e16), d, band()) {
        Err(OffsetError::RadiusFloor { kind, realized }) => {
            // The echo payload carries the refusing kind and the very
            // float the floor metered: the collapsed 0.0, not the
            // exact-real +0.5 m.
            assert_eq!(kind, SurfaceKind::Cylinder);
            assert_eq!(realized.to_bits(), 0.0f64.to_bits());
        }
        other => panic!("the large-scale collapse must refuse on the floor, got {other:?}"),
    }
    // The nearby offset whose realized radius is honestly positive
    // still mints, and stores exactly the metered float.
    let d2 = -(1.0e16 - 8.0);
    let minted = offset_surface(&cyl(1.0e16), d2, band()).unwrap();
    let Surface::Cylinder { radius, .. } = minted else {
        panic!("cylinder offset must stay a cylinder");
    };
    assert_eq!(radius.to_bits(), (1.0e16 + d2).to_bits());
}

/// The torus ring convention: an outward offset whose realized minor
/// radius reaches the major radius would mint a spindle/horn torus —
/// refused at the door, at the crossing and beyond, while the floor
/// (decided first) still owns the inward collapse.
#[test]
fn torus_ring_refuses_spindle_crossing() {
    for d in [1.0, 1.5] {
        match offset_surface(&torus(2.0, 1.0), d, band()) {
            Err(OffsetError::TorusRing { realized_minor }) => {
                // The echo payload is the realized minor the ring
                // margin folded against R.
                assert_eq!(realized_minor.to_bits(), (1.0 + d).to_bits());
            }
            other => panic!("torus d = {d} must refuse on the ring convention, got {other:?}"),
        }
    }
    // Well inside both margins, the mint carries R and updates r.
    let minted = offset_surface(&torus(2.0, 1.0), 0.5, band()).unwrap();
    let Surface::Torus {
        major_radius,
        minor_radius,
        ..
    } = minted
    else {
        panic!("torus offset must stay a torus");
    };
    assert_eq!(major_radius.to_bits(), 2.0f64.to_bits());
    assert_eq!(minor_radius.to_bits(), 1.5f64.to_bits());
}

/// NURBS is not closed under offset: typed refusal naming the
/// approximating-surface route as the coming door.
#[test]
fn nurbs_refuses_typed_naming_the_approximating_route() {
    let err = offset_surface(&Surface::<f64>::nurbs_placeholder(), 0.1, band()).unwrap_err();
    assert!(matches!(err, OffsetError::NotClosedUnderOffset));
    let msg = err.to_string();
    assert!(
        msg.contains("approximating-surface") && msg.contains("not closed under offset"),
        "the refusal must name the coming route: {msg}"
    );
}

/// Honest escalation at f64: a realized radius inside the ambiguity
/// band (between ε and K·ε of zero) is neither minted nor refused —
/// the door escalates, naming the floor predicate. Same for the ring
/// margin.
#[test]
fn ambiguity_band_escalates_with_predicate_names() {
    let eps = Tol::witness().get().eps;
    let d = -(1.0 - 3.0 * eps);
    match offset_surface(&cyl(1.0), d, band()) {
        Err(OffsetError::Escalated { source }) => {
            assert_eq!(source.predicate, Some("offset_radius_floor"));
        }
        other => panic!("expected the floor escalation, got {other:?}"),
    }
    let d_ring = 1.0 - 3.0 * eps;
    match offset_surface(&torus(2.0, 1.0), d_ring, band()) {
        Err(OffsetError::Escalated { source }) => {
            assert_eq!(source.predicate, Some("offset_torus_ring"));
        }
        other => panic!("expected the ring escalation, got {other:?}"),
    }
}

// ---------------------------------------------------------------------
// The interval lane: enclosures contain, refusals escalate honestly
// ---------------------------------------------------------------------

#[cfg(feature = "interval")]
mod interval {
    use super::*;
    use crate::shared::interval::{ip, iv3 as iv};
    use geom_core::{Bounds, Interval, Real};

    /// The mint runs at `T = Interval` and its enclosures contain the
    /// f64 mint: the stored radius encloses the rounded sum, and the
    /// cone's slid apex height encloses the f64 slide's — the
    /// containment claim per field, on the two arms with arithmetic
    /// (the others are verbatim copies).
    #[test]
    fn interval_mint_encloses_the_f64_mint() {
        let d = 0.3;
        let cyl = Surface::Cylinder {
            origin: ip(t_center()),
            axis: iv(t_axis()),
            radius: Interval::from_f64(2.5),
            u_ref: iv(t_uref()),
        };
        let minted = offset_surface(&cyl, Interval::from_f64(d), band()).unwrap();
        let Surface::Cylinder { radius, .. } = minted else {
            panic!("cylinder offset must stay a cylinder");
        };
        let exact = 2.5 + d;
        assert!(
            radius.lo() <= exact && exact <= radius.hi(),
            "radius enclosure [{}, {}] must contain {exact}",
            radius.lo(),
            radius.hi()
        );
        assert!(radius.hi() - radius.lo() < 1e-12, "thin inputs stay thin");

        let cone = Surface::Cone {
            apex: ip(Point3::new(0.0, 0.0, 2.0)),
            axis: iv(Vec3::unit_z()),
            half_angle: Interval::from_f64(FRAC_PI_6),
            u_ref: iv(Vec3::unit_x()),
        };
        let minted = offset_surface(&cone, Interval::from_f64(d), band()).unwrap();
        let Surface::Cone { apex, .. } = minted else {
            panic!("cone offset must stay a cone");
        };
        let expected = 2.0 - d / FRAC_PI_6.sin();
        assert!(
            apex.z.lo() <= expected && expected <= apex.z.hi(),
            "apex-height enclosure [{}, {}] must contain {expected}",
            apex.z.lo(),
            apex.z.hi()
        );
        assert!(apex.z.hi() - apex.z.lo() < 1e-12, "thin inputs stay thin");
    }

    /// A margin enclosure straddling the band cannot classify: the
    /// door escalates (naming the floor predicate) rather than
    /// guessing a mint or a refusal — while a definitely collapsed
    /// enclosure still refuses typed.
    #[test]
    fn interval_refusals_escalate_honestly() {
        let mk = |radius: Interval| Surface::Cylinder {
            origin: ip(t_center()),
            axis: iv(t_axis()),
            radius,
            u_ref: iv(t_uref()),
        };
        let d = Interval::from_f64(-1.0);
        match offset_surface(&mk(Interval::from_bounds(0.9, 1.1)), d, band()) {
            Err(OffsetError::Escalated { source }) => {
                assert_eq!(source.predicate, Some("offset_radius_floor"));
            }
            other => panic!("a straddling enclosure must escalate, got {other:?}"),
        }
        assert!(
            matches!(
                offset_surface(&mk(Interval::from_f64(0.5)), d, band()),
                Err(OffsetError::RadiusFloor { .. })
            ),
            "a definitely collapsed enclosure still refuses typed"
        );
    }

    /// Defining equation at the interval scalar: the pushforward's
    /// signed-distance residual against the minted surface encloses
    /// zero (narrowly), on cylinder and cone rows.
    #[test]
    fn interval_pushforward_residual_encloses_zero() {
        let d = Interval::from_f64(0.3);
        let rows: Vec<(&str, Surface<Interval>)> = vec![
            (
                "cylinder",
                Surface::Cylinder {
                    origin: ip(t_center()),
                    axis: iv(t_axis()),
                    radius: Interval::from_f64(2.5),
                    u_ref: iv(t_uref()),
                },
            ),
            (
                "cone",
                Surface::Cone {
                    apex: ip(t_center()),
                    axis: iv(t_axis()),
                    half_angle: Interval::from_f64(FRAC_PI_6),
                    u_ref: iv(t_uref()),
                },
            ),
        ];
        for (name, base) in rows {
            let minted = offset_surface(&base, d, band()).unwrap();
            for (u, v) in [(0.0, 1.5), (0.9, 0.8), (-1.8, 3.0)] {
                let p = base.eval(Interval::from_f64(u), Interval::from_f64(v));
                let n = base.normal(Interval::from_f64(u), Interval::from_f64(v));
                let q = p + n * d;
                let r = interval_signed_dist(&minted, q);
                assert!(
                    r.lo() <= 0.0 && 0.0 <= r.hi(),
                    "{name} at ({u},{v}): residual [{}, {}]",
                    r.lo(),
                    r.hi()
                );
                assert!(r.hi() - r.lo() < 1e-12, "{name} at ({u},{v}): width");
            }
        }
    }

    /// The independent signed-distance spellings at `T = Interval`,
    /// cylinder and cone arms (the two the residual row samples).
    fn interval_signed_dist(s: &Surface<Interval>, q: Point3<Interval>) -> Interval {
        match *s {
            Surface::Cylinder {
                origin,
                axis,
                radius,
                ..
            } => {
                let rel = q - origin;
                let h = rel.dot(axis);
                (rel - axis * h).norm() - radius
            }
            Surface::Cone {
                apex,
                axis,
                half_angle,
                ..
            } => {
                let rel = q - apex;
                let h = rel.dot(axis);
                let rho = (rel - axis * h).norm();
                rho * half_angle.cos() - h * half_angle.sin()
            }
            _ => panic!("only the sampled kinds have interval spellings here"),
        }
    }
}
