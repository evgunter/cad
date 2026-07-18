//! M2 PR 3 adversarial review — certification-layer attacks
//! (falsification targets 1, 3, 4, 7 of the review spec).
//!
//! Convention: tests named `finding_*` PIN CURRENT (defective or
//! questionable) behavior and carry a `FINDING:` comment — the fix pass
//! flips their assertions to the desired behavior. Tests named
//! `survives_*` are attacks the implementation correctly resisted and
//! can be promoted to `review_m2_pr3` CI verbatim.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, FRAC_PI_6, PI, TAU};

use geom_brep::{
    CertCheck, CertifyError, DihedralClass, EdgeCurve, EdgeCurveSpec, EdgeGeometry, MappedCurve,
    NewellError, SketchSegment, SurfaceKey, classify_dihedral, newell_plane,
};
use geom_core::{Affine3, Band, Point2, Point3, Tolerance, Vec3};
use geom_curves::Curve3;
use geom_surfaces::Surface;

fn band() -> Band {
    Band::linear().unwrap()
}

fn eps() -> f64 {
    Tolerance::get().eps
}

/// A resolver over a fixed table (keys minted through a local slotmap,
/// mirroring how a Body resolves) — the same shape as the crate's own
/// unit-test helper.
fn table(
    surfs: Vec<Surface<f64>>,
) -> (Vec<SurfaceKey>, impl Fn(SurfaceKey) -> Option<Surface<f64>>) {
    let mut map: slotmap::SlotMap<SurfaceKey, Surface<f64>> = slotmap::SlotMap::with_key();
    let keys: Vec<SurfaceKey> = surfs.into_iter().map(|s| map.insert(s)).collect();
    (keys, move |k| map.get(k).copied())
}

// =====================================================================
// Target 1 — certification aliasing: is the 9-sample schedule enough?
// =====================================================================
//
// Algebra: for a circle carrier certified pointwise against a mapped
// description at N = 9 uniform samples, matching at >= 3 distinct
// points forces carrier circle == description circle AS A SET, and the
// per-sample angular discrepancy must vanish mod tau. With uniform
// samples s = i/8 that means (Delta - theta)·i/8 ≡ 0 (mod tau), i.e.
// Delta = theta + 8·k·tau: ANY carrier interval that winds 8k extra
// full turns aliases the schedule EXACTLY. 9 samples do NOT pin the
// winding number — the counterexample family below. (Lines are safe:
// affine-in-s residuals vanish identically after 2 matches.)

/// FINDING (aliasing counterexample): a quarter-arc description
/// certifies against a carrier whose stored interval winds EIGHT extra
/// full turns — every schedule sample lands on the true point (the
/// extra 8·tau contributes i whole turns at sample i), while between
/// samples the "certified" affine alignment is off by up to a half
/// turn. The claimed invariant "sample i compares
/// carrier(t0 + (t1−t0)·s) against description(s) … certification is
/// exactly what makes it checked rather than trusted" (certify.rs
/// module docs) is falsified for the parameter cache.
#[test]
fn finding_winding_aliased_arc_interval_certifies() {
    let bulge = (PI / 8.0).tan(); // quarter arc, CCW, unit circle
    let desc = MappedCurve::PlacedSegment {
        segment: SketchSegment::Arc {
            a: Point2::new(1.0, 0.0),
            b: Point2::new(0.0, 1.0),
            bulge,
        },
        place: Affine3::identity(),
    };
    let spec = EdgeCurveSpec {
        description: EdgeGeometry::MappedCurve(desc),
        carrier: Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        },
        param_start: 0.0,
        // WRONG: 8 extra full windings. Correct value: FRAC_PI_2.
        param_end: FRAC_PI_2 + 8.0 * TAU,
    };
    let p0 = Point3::new(1.0, 0.0, 0.0);
    let p1 = Point3::new(0.0, 1.0, 0.0);
    let certified = EdgeCurve::certify(spec, p0, p1, |_| None, band());
    // FINDING: this certifies. The fix pass should make it a typed
    // rejection (e.g. a circle-carrier interval-length bound |t1 − t0|
    // <= tau, or a winding check against the description).
    let curve = certified.expect("FINDING: aliased 8-turn interval certifies today");
    // Demonstrate the harm: between schedule samples the carrier is
    // nowhere near the described locus point (here ~a half turn off).
    let (t0, t1) = curve.params();
    let s = 1.0 / 16.0;
    let carrier_mid = curve.carrier().eval(t0 + (t1 - t0) * s);
    let desc_mid = desc.eval(s);
    assert!(
        carrier_mid.distance(desc_mid) > 0.5,
        "the aliased cache really is wrong between samples: {carrier_mid:?} vs {desc_mid:?}"
    );
}

/// FINDING (same family, full period): a one-revolution RevolvedPoint
/// description certifies against a NINE-revolution carrier interval.
#[test]
fn finding_winding_aliased_full_period_certifies() {
    let center = Point3::new(1.0, 2.0, 3.0);
    let p = Point3::new(2.0, 2.0, 3.0);
    let spec = EdgeCurveSpec {
        description: EdgeGeometry::MappedCurve(MappedCurve::RevolvedPoint {
            point: Point2::new(2.0, 2.0),
            place: Affine3::translation(Vec3::new(0.0, 0.0, 3.0)),
            axis_origin: center,
            axis_dir: Vec3::unit_z(),
            angle: TAU,
        }),
        carrier: Curve3::Circle {
            center,
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        },
        param_start: 0.0,
        // WRONG: nine revolutions; the description makes one.
        param_end: 9.0 * TAU,
    };
    let r = EdgeCurve::certify(spec, p, p, |_| None, band());
    assert!(
        r.is_ok(),
        "FINDING: 9-revolution interval aliases the 9-sample schedule: {r:?}"
    );
}

/// SURVIVES: the non-aliased wrong carriers of the assignment are all
/// rejected — wrong radius, wrong center, shifted line, and an interval
/// misaligned with s ∈ [0, 1] (endpoint pinning catches it). Windings
/// that are NOT multiples of 8 full turns are rejected too (the
/// schedule only aliases at 8k·tau).
#[test]
fn survives_wrong_carriers_are_rejected() {
    let bulge = (PI / 8.0).tan();
    let arc = MappedCurve::PlacedSegment {
        segment: SketchSegment::Arc {
            a: Point2::new(1.0, 0.0),
            b: Point2::new(0.0, 1.0),
            bulge,
        },
        place: Affine3::identity(),
    };
    let p0 = Point3::new(1.0, 0.0, 0.0);
    let p1 = Point3::new(0.0, 1.0, 0.0);
    let base = |carrier, t0: f64, t1: f64| EdgeCurveSpec {
        description: EdgeGeometry::MappedCurve(arc),
        carrier,
        param_start: t0,
        param_end: t1,
    };
    // Wrong radius (center shifted to keep endpoints pinned is not even
    // possible for r != 1 through both endpoints + circular samples;
    // plain wrong radius fails at pinning already).
    let wrong_radius = base(
        Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.5,
            u_ref: Vec3::unit_x(),
        },
        0.0,
        FRAC_PI_2,
    );
    assert!(EdgeCurve::certify(wrong_radius, p0, p1, |_| None, band()).is_err());
    // Wrong center, radius adjusted so both endpoints still lie on the
    // carrier: center on the perpendicular bisector of the chord.
    // Interior samples leave the described locus -> rejected.
    let c = Point3::new(-0.5, -0.5, 0.0);
    let wrong_center = base(
        Curve3::Circle {
            center: c,
            axis: Vec3::unit_z(),
            radius: c.distance(p0),
            u_ref: (p0 - c) / c.distance(p0),
        },
        0.0,
        // Angle subtended at the false center.
        2.0 * ((p0.distance(p1) / 2.0) / c.distance(p0)).asin(),
    );
    assert!(EdgeCurve::certify(wrong_center, p0, p1, |_| None, band()).is_err());
    // A 1-turn (not 8k) extra winding IS caught by the interior samples.
    let one_turn = base(
        Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        },
        0.0,
        FRAC_PI_2 + TAU,
    );
    assert!(EdgeCurve::certify(one_turn, p0, p1, |_| None, band()).is_err());
    // Shifted line vs line description.
    let l0 = Point3::origin();
    let l1 = Point3::new(1.0, 0.0, 0.0);
    let mut line = EdgeCurveSpec::line_between(l0, l1);
    line.carrier = Curve3::Line {
        origin: Point3::new(0.0, 100.0 * eps(), 0.0),
        dir: Vec3::unit_x(),
    };
    assert!(EdgeCurve::certify(line, l0, l1, |_| None, band()).is_err());
    // Interval affinely misaligned with s ∈ [0,1]: same correct circle,
    // parameters shifted a quarter turn — endpoint pinning fires.
    let shifted = base(
        Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        },
        FRAC_PI_2,
        PI,
    );
    assert!(matches!(
        EdgeCurve::certify(shifted, p0, p1, |_| None, band()).unwrap_err(),
        CertifyError::ResidualExceeded {
            check: CertCheck::EndpointStart,
            ..
        }
    ));
}

// =====================================================================
// Target 1/4 — Intersection carriers: what does certification pin?
// =====================================================================

/// SURVIVES (baseline): the honest partial rim — a half-circle arc as
/// the Intersection of a cap plane and a cylinder — certifies, and is
/// classified transverse (gradients are exactly perpendicular).
#[test]
fn survives_plane_cylinder_partial_rim_certifies() {
    let (keys, lookup) = table(vec![
        Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        },
        Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        },
    ]);
    let p0 = Point3::new(1.0, 0.0, 0.0);
    let p1 = Point3::new(-1.0, 0.0, 0.0);
    let spec = EdgeCurveSpec {
        description: EdgeGeometry::Intersection {
            s1: keys[0],
            s2: keys[1],
            witness: Point3::new(0.0, 1.0, 0.0),
        },
        carrier: Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        },
        param_start: 0.0,
        param_end: PI,
    };
    EdgeCurve::certify(spec, p0, p1, &lookup, band()).unwrap();
}

/// FINDING (interval under-determination for Intersection): with the
/// same endpoints, the carrier that takes the COMPLEMENTARY arc
/// (reversed winding) and the carrier that winds an extra 1.5 turns
/// both certify — every sample lies on both surfaces and the endpoints
/// pin. An Intersection edge's stored interval (which arc, which
/// winding) is completely unverified beyond its endpoints; the module
/// docs only caveat *component* selection ("which component it selects
/// is unverifiable before marching exists — M3"), but this ambiguity is
/// within one connected component. Consumers of the cache (PR 6
/// tessellation chord points) would traverse the wrong arc.
#[test]
fn finding_intersection_arc_side_and_winding_unpinned() {
    let (keys, lookup) = table(vec![
        Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        },
        Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        },
    ]);
    let p0 = Point3::new(1.0, 0.0, 0.0);
    let p1 = Point3::new(-1.0, 0.0, 0.0);
    let mk = |t1: f64| EdgeCurveSpec {
        description: EdgeGeometry::Intersection {
            s1: keys[0],
            s2: keys[1],
            witness: Point3::new(0.0, 1.0, 0.0),
        },
        carrier: Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        },
        param_start: 0.0,
        param_end: t1,
    };
    // The complementary (lower) arc: t runs 0 -> −pi.
    assert!(
        EdgeCurve::certify(mk(-PI), p0, p1, &lookup, band()).is_ok(),
        "FINDING: reversed-winding complementary arc certifies"
    );
    // An extra 1.5 windings: t runs 0 -> 3·pi.
    assert!(
        EdgeCurve::certify(mk(3.0 * PI), p0, p1, &lookup, band()).is_ok(),
        "FINDING: 1.5-winding interval certifies"
    );
}

/// FINDING (BLOCKER-grade for the PR 5 handoff): a FULL-PERIOD rim —
/// the circle where a cap plane meets a cylinder at a genuine right
/// angle, stored as a self-loop edge (coincident endpoints, the shape
/// every full-revolve latitude circle takes) — is REFUSED as an
/// Intersection: the transversality margin sin(theta)·r folds in the
/// edge-chord extent, and a closed edge's chord is ZERO, so the margin
/// is 0 -> Sign::Zero -> "definitely Smooth" -> NotTransverse. The
/// chord is a dishonest lever arm for closed edges (the honest extent
/// of a full circle is its diameter, not the distance between its
/// coincident endpoints). Consequence: the promised PR 4/5 flow
/// "upgrade corner joins to Intersection via set_edge_curve" cannot
/// ever succeed for full-period rims, and tier 3's dihedral pass is
/// vacuously 'Smooth' on every self-loop edge (see the topo suite).
#[test]
fn finding_full_period_rim_intersection_refused_via_zero_chord() {
    let (keys, lookup) = table(vec![
        Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        },
        Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        },
    ]);
    let p = Point3::new(1.0, 0.0, 0.0);
    let spec = EdgeCurveSpec {
        description: EdgeGeometry::Intersection {
            s1: keys[0],
            s2: keys[1],
            witness: Point3::new(0.0, 1.0, 0.0),
        },
        carrier: Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        },
        param_start: 0.0,
        param_end: TAU,
    };
    let err = EdgeCurve::certify(spec, p, p, &lookup, band()).unwrap_err();
    // FINDING: a genuinely 90-degree wedge reports "tangent planes
    // coincide". The fix pass should make full-period Intersection
    // rims certifiable (honest extent for closed carriers).
    assert_eq!(
        err,
        CertifyError::NotTransverse { sample: 1 },
        "zero chord turned a right angle into 'definitely smooth'"
    );
}

// =====================================================================
// Target 3 — dihedral lever-arm honesty.
// =====================================================================

/// (b) SURVIVES: exact tangency never reads Transverse; a true corner
/// at a healthy arm never reads Smooth. (Re-run under the CI epsilon
/// matrix — thresholds here are run-scaled.)
#[test]
fn survives_exact_tangency_and_true_corner_classify_definitely() {
    let plane = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    // Cylinder resting on the plane: exact tangency along the contact
    // generator through the origin.
    let cyl = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 2.0),
        axis: Vec3::unit_y(),
        radius: 2.0,
        u_ref: Vec3::unit_x(),
    };
    assert_eq!(
        classify_dihedral(&plane, &cyl, Point3::origin(), 10.0, band()).unwrap(),
        DihedralClass::Smooth
    );
    // True corner: perpendicular planes at arm 1.
    let wall = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_x(),
        u_ref: Vec3::unit_y(),
    };
    assert_eq!(
        classify_dihedral(&plane, &wall, Point3::origin(), 1.0, band()).unwrap(),
        DihedralClass::Transverse
    );
}

/// (a) FINDING: near the cone apex the arms collapse — the claim is
/// "honest escalation, never a definite misclassification". The
/// escalation band exists (rho ~ 3·eps escalates, good), but INSIDE it
/// (rho <= eps) the margin classifies Sign::Zero and a ~60-degree
/// transverse wedge reads **definitely Smooth** — a definite
/// misclassification, not an escalation. Same root cause as the
/// zero-chord finding: a collapsed lever arm maps every angle into the
/// Zero band, and DihedralClass has no "arm too small to say" outcome.
#[test]
fn finding_sub_epsilon_cone_arm_reads_definitely_smooth() {
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: FRAC_PI_6,
        u_ref: Vec3::unit_x(),
    };
    let tan_a = FRAC_PI_6.tan();
    // A cone point at radial distance rho from the axis (on the locus).
    let at = |rho: f64| Point3::new(rho, 0.0, rho / tan_a);
    let plane_through = |p: Point3<f64>| Surface::Plane {
        origin: p,
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    // Healthy arm: definitely transverse (sin theta = cos alpha ~ 0.87).
    let p = at(1.0);
    assert_eq!(
        classify_dihedral(&cone, &plane_through(p), p, 1.0, band()).unwrap(),
        DihedralClass::Transverse
    );
    // In-band arm: honest escalation (the claimed behavior).
    let p = at(3.0 * eps());
    assert!(classify_dihedral(&cone, &plane_through(p), p, 1.0, band()).is_err());
    // Collapsed arm: FINDING — the same transverse wedge is now
    // "definitely Smooth" (margin 0.87·rho <= eps classifies Zero).
    let p = at(0.5 * eps());
    assert_eq!(
        classify_dihedral(&cone, &plane_through(p), p, 1.0, band()).unwrap(),
        DihedralClass::Smooth,
        "FINDING: sub-eps arm turns a 60-degree corner into definite Smooth"
    );
}

/// (c) FINDING: the chord extent direction — a true right-angle corner
/// on a sub-epsilon-chord edge reads definitely Smooth (extent < eps
/// forces margin <= eps). Defensible under the displacement philosophy
/// for genuinely tiny OPEN edges, but the same fold is what zeroes
/// closed edges (chord 0 with huge true extent) — see the full-period
/// finding above. Pinned here so the fix pass revisits both together.
#[test]
fn finding_sub_epsilon_chord_true_corner_reads_smooth() {
    let floor = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let wall = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_x(),
        u_ref: Vec3::unit_y(),
    };
    let c = classify_dihedral(&floor, &wall, Point3::origin(), 0.5 * eps(), band()).unwrap();
    assert_eq!(c, DihedralClass::Smooth, "FINDING: 90-degree corner");
    // Zero extent exactly (a closed edge's chord): also Smooth.
    let c = classify_dihedral(&floor, &wall, Point3::origin(), 0.0, band()).unwrap();
    assert_eq!(c, DihedralClass::Smooth, "FINDING: zero-chord corner");
}

/// (c) SURVIVES: huge edges on tiny features — the curvature arm caps
/// the chord, so a long edge on a tiny cylinder still escalates/
/// classifies through the honest small arm rather than the huge chord.
#[test]
fn survives_curvature_arm_caps_huge_chord() {
    // Tiny cylinder (r = 3·eps) against a plane tilted well clear of
    // tangency: margin = sin(theta)·r stays in/below the band despite a
    // kilometer-scale chord — never a false definite from the chord.
    let r = 3.0 * eps();
    let cyl = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, r),
        axis: Vec3::unit_y(),
        radius: r,
        u_ref: Vec3::unit_x(),
    };
    let tilted = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::new(FRAC_PI_2.sin(), 0.0, FRAC_PI_2.cos()),
        u_ref: Vec3::unit_y(),
    };
    // sin(theta) = 1, arm = min(r, 1000.0) = r = 3 eps -> in band.
    assert!(
        classify_dihedral(&tilted, &cyl, Point3::origin(), 1000.0, band()).is_err(),
        "tiny curvature arm must dominate a huge chord"
    );
}

// =====================================================================
// Target 4 — stored-interval reconciliation.
// =====================================================================

/// FINDING (documented, minor): a DECREASING parameter interval
/// certifies when description and carrier are arranged consistently —
/// the "increasing parameter runs start -> end of he_plus" contract of
/// the ratified vertices-derive-bounds rule is not enforced by
/// certification (nothing checks t0 < t1).
#[test]
fn finding_reversed_interval_certifies() {
    let p0 = Point3::origin();
    let p1 = Point3::new(1.0, 0.0, 0.0);
    let spec = EdgeCurveSpec {
        // Description runs p1 -> p0 over s in [0,1].
        description: EdgeGeometry::MappedCurve(MappedCurve::ExtrudedPoint {
            point: Point2::new(0.0, 0.0),
            place: Affine3::translation(p1 - Point3::origin()),
            vec: p0 - p1,
        }),
        // Carrier parameterized from p0, walked BACKWARD: t: 1 -> 0.
        carrier: Curve3::Line {
            origin: p0,
            dir: Vec3::unit_x(),
        },
        param_start: 1.0,
        param_end: 0.0,
    };
    assert!(
        EdgeCurve::certify(spec, p1, p0, |_| None, band()).is_ok(),
        "FINDING: decreasing intervals pass certification"
    );
}

/// FINDING (documented, minor): a ZERO-LENGTH edge — coincident
/// endpoints, degenerate interval (t0 == t1), constant description —
/// certifies (all nine samples coincide). Downstream, its dihedral
/// margin is identically zero (chord 0), i.e. vacuously Smooth.
#[test]
fn finding_zero_length_edge_certifies() {
    let p = Point3::new(2.0, -1.0, 5.0);
    let spec = EdgeCurveSpec {
        description: EdgeGeometry::MappedCurve(MappedCurve::ExtrudedPoint {
            point: Point2::new(0.0, 0.0),
            place: Affine3::translation(p - Point3::origin()),
            vec: Vec3::zero(),
        }),
        carrier: Curve3::Line {
            origin: p,
            dir: Vec3::unit_x(),
        },
        param_start: 0.0,
        param_end: 0.0,
    };
    assert!(
        EdgeCurve::certify(spec, p, p, |_| None, band()).is_ok(),
        "FINDING: zero-length edges are certifiable"
    );
}

/// SURVIVES: full-period MAPPED edges are total and certify with their
/// stored (0, tau) interval (the scaffolding convention), and a
/// vertices-disagreeing interval fails loudly at endpoint pinning.
#[test]
fn survives_full_period_mapped_and_endpoint_pinning() {
    let p = Point3::new(-2.0, 0.5, 7.0);
    let spec = EdgeCurveSpec::self_loop_circle_at(p);
    EdgeCurve::certify(spec, p, p, |_| None, band()).unwrap();
    // Same spec, but the edge's vertices sit elsewhere: pinning fires.
    let q = Point3::new(-2.0, 0.5, 8.0);
    let spec = EdgeCurveSpec::self_loop_circle_at(p);
    assert!(matches!(
        EdgeCurve::certify(spec, q, q, |_| None, band()).unwrap_err(),
        CertifyError::ResidualExceeded {
            check: CertCheck::EndpointStart,
            ..
        }
    ));
}

// =====================================================================
// Target 7 — Newell honesty.
// =====================================================================

/// SURVIVES: the translate-to-origin pin at 1e8 offset (re-verified
/// independently of the in-crate test), and exact collinearity
/// escalates rather than producing a garbage plane.
#[test]
fn survives_newell_far_offset_and_collinear_escalation() {
    let off = Point3::new(1.0e8 + 0.25, 1.0e8 + 0.5, 1.0e8 + 0.125);
    let u = Vec3::new(1.0, 0.0, 0.5);
    let v = Vec3::new(0.0, 1.0, -0.25);
    let true_n = u.cross(v).normalize();
    let corner = |a: f64, b: f64| off + u * a + v * b;
    let pts = [
        corner(0.0, 0.0),
        corner(1.0, 0.0),
        corner(1.0, 2.0),
        corner(0.0, 2.0),
    ];
    let Surface::Plane { normal, .. } = newell_plane(&pts, band()).unwrap() else {
        panic!("plane");
    };
    assert!(normal.cross(true_n).norm() < 1e-14);
    // Collinear chain: escalation, not a garbage certified plane.
    let line = [
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(2.0, 0.0, 0.0),
        Point3::new(3.0, 0.0, 0.0),
    ];
    assert!(matches!(
        newell_plane(&line, band()).unwrap_err(),
        NewellError::Escalated { .. }
    ));
}

/// FINDING (documented, minor): NEAR-collinear loops whose lifts stay
/// under eps are certified with a noise-determined normal — two inputs
/// differing by 0.2·eps get near-OPPOSITE certified normals. The
/// residual contract ("every vertex within eps of the plane") is
/// honestly met; what is under-determined is the normal's DIRECTION,
/// which downstream consumers (orientation contracts) read as exact.
/// Inherent to under-determined data; should at minimum be documented
/// on newell_plane.
#[test]
fn finding_near_collinear_normal_is_noise_determined_but_certified() {
    let d = 0.1 * eps();
    let chain = |sign: f64| {
        [
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, sign * d, 0.0),
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(1.0, -sign * d, d),
        ]
    };
    let normal = |pts: &[Point3<f64>]| {
        let Surface::Plane { normal, .. } = newell_plane(pts, band()).unwrap() else {
            panic!("plane");
        };
        normal
    };
    let n_pos = normal(&chain(1.0));
    let n_neg = normal(&chain(-1.0));
    assert!(
        n_pos.dot(n_neg) < 0.5,
        "FINDING pinned: sub-eps perturbation swings the certified normal \
         (dot = {})",
        n_pos.dot(n_neg)
    );
}

// =====================================================================
// Interval lane (target 3d): the f64::MAX plane-arm fix and poison
// hygiene, at the certified interval scalar.
// =====================================================================
#[cfg(feature = "interval")]
mod interval_lane {
    use super::*;
    use geom_core::{Bounds, Interval, Real};

    fn ipt(x: f64, y: f64, z: f64) -> Point3<Interval> {
        Point3::new(
            Interval::from_f64(x),
            Interval::from_f64(y),
            Interval::from_f64(z),
        )
    }

    fn ivec(x: f64, y: f64, z: f64) -> geom_core::Vec3<Interval> {
        geom_core::Vec3::new(
            Interval::from_f64(x),
            Interval::from_f64(y),
            Interval::from_f64(z),
        )
    }

    /// Regression for the fixed bug: plane-adjacent dihedrals must NOT
    /// poison through the curvature-arm min fold (from_f64(MAX) is a
    /// valid singleton; from_f64(inf) was NaI). Both the transverse and
    /// the smooth outcome must be definite in the interval lane.
    #[test]
    fn survives_interval_plane_dihedral_is_definite() {
        let floor: Surface<Interval> = Surface::Plane {
            origin: ipt(0.0, 0.0, 0.0),
            normal: ivec(0.0, 0.0, 1.0),
            u_ref: ivec(1.0, 0.0, 0.0),
        };
        let wall: Surface<Interval> = Surface::Plane {
            origin: ipt(0.0, 0.0, 0.0),
            normal: ivec(1.0, 0.0, 0.0),
            u_ref: ivec(0.0, 1.0, 0.0),
        };
        let c = classify_dihedral(
            &floor,
            &wall,
            ipt(0.0, 0.0, 0.0),
            Interval::from_f64(1.0),
            band(),
        )
        .unwrap();
        assert_eq!(c, DihedralClass::Transverse);
        // Coplanar pair: definite Smooth (no poison, no escalation).
        let coplanar: Surface<Interval> = Surface::Plane {
            origin: ipt(0.0, 0.0, 0.0),
            normal: ivec(0.0, 0.0, 1.0),
            u_ref: ivec(0.0, 1.0, 0.0),
        };
        let c = classify_dihedral(
            &floor,
            &coplanar,
            ipt(0.0, 0.0, 0.0),
            Interval::from_f64(1.0),
            band(),
        )
        .unwrap();
        assert_eq!(c, DihedralClass::Smooth);
    }

    /// Plane-plane mapped-line certification runs clean end to end in
    /// the interval lane (no NaI leaks anywhere in the schedule).
    #[test]
    fn survives_interval_line_certification() {
        let p0 = ipt(0.0, 0.0, 0.0);
        let p1 = ipt(1.0, 0.0, 0.0);
        let spec = EdgeCurveSpec::line_between(p0, p1);
        let c = EdgeCurve::certify(spec, p0, p1, |_| None, band()).unwrap();
        let r = c.certificate().max_residual;
        assert!(r.hi().is_finite(), "no poison in the certificate: {r:?}");
    }

    /// The interval lane DOES refuse the 9-revolution alias — but for
    /// the wrong reason: the refusal is the blanket
    /// norm-sqrt-clamp poison of the finding below (every inexact
    /// distance enclosure degrades to Trv), not a principled detection
    /// of the wrong winding. Pinned so the fix pass re-checks this
    /// after fixing the poison: once interval distances work, the
    /// alias must STILL be refused (by a real winding check).
    #[test]
    fn finding_interval_winding_alias_refused_only_by_the_poison() {
        let center = ipt(1.0, 2.0, 3.0);
        let p = ipt(2.0, 2.0, 3.0);
        let spec = EdgeCurveSpec {
            description: EdgeGeometry::MappedCurve(MappedCurve::RevolvedPoint {
                point: Point2::new(Interval::from_f64(2.0), Interval::from_f64(2.0)),
                place: Affine3::translation(ivec(0.0, 0.0, 3.0)),
                axis_origin: center,
                axis_dir: ivec(0.0, 0.0, 1.0),
                angle: Interval::from_f64(core::f64::consts::TAU),
            }),
            carrier: Curve3::Circle {
                center,
                axis: ivec(0.0, 0.0, 1.0),
                radius: Interval::from_f64(1.0),
                u_ref: ivec(1.0, 0.0, 0.0),
            },
            param_start: Interval::from_f64(0.0),
            param_end: Interval::from_f64(9.0 * core::f64::consts::TAU),
        };
        let err = EdgeCurve::certify(spec, p, p, |_| None, band()).unwrap_err();
        assert!(
            matches!(
                err,
                CertifyError::Escalated {
                    check: CertCheck::MappedSource,
                    ..
                }
            ),
            "interval lane refuses the alias (today: via the poison): {err:?}"
        );
    }

    /// FINDING (BLOCKER): the canonical SCAFFOLDING SELF-LOOP CIRCLE —
    /// the spec the sugar attaches at every self-loop site — is
    /// UNCERTIFIABLE at the interval scalar. `carrier(tau)` does not
    /// reproduce the anchor's singleton exactly, the difference
    /// enclosure straddles zero, `Vec3::norm`'s `sqrt(dot(v, v))`
    /// squares the straddling components through plain interval `Mul`
    /// (lo goes NEGATIVE, e.g. [-1e-33, …], instead of the tight
    /// `pown`-style [0, …]), the sqrt CLAMPS and degrades the
    /// decoration to Trv, and `Decide` reads decoration < Def as
    /// poison: `Escalated { EndpointEnd, margin: Invalid }`. The same
    /// mechanism refuses EVERY certification whose carrier evaluation
    /// is not exactly-dyadic-singleton — see the topo suite for the
    /// non-axis-aligned rim mint failing. The existing interval tests
    /// pass only because the unit cube's geometry is axis-aligned
    /// dyadic (all enclosures stay singletons).
    #[test]
    fn finding_interval_scaffolding_self_loop_circle_uncertifiable() {
        let p = Point3::new(
            Interval::from_f64(7.0),
            Interval::from_f64(7.0),
            Interval::from_f64(7.0),
        );
        let spec = EdgeCurveSpec::self_loop_circle_at(p);
        let err = EdgeCurve::certify(spec, p, p, |_| None, band()).unwrap_err();
        assert!(
            matches!(
                err,
                CertifyError::Escalated {
                    check: CertCheck::EndpointEnd,
                    ..
                }
            ),
            "FINDING pinned: the scaffolding convention dies at Interval: {err:?}"
        );
    }
}
