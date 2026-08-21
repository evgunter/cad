//! M2 PR 3 adversarial review — certification-layer attacks
//! (falsification targets 1, 3, 4, 7 of the review spec).
//!
//! Convention: tests named `survives_*` are attacks the implementation
//! correctly resisted, promoted to CI verbatim. Tests named `fixed_*`
//! were the review's `finding_*` pins of defective behavior, FLIPPED by
//! the fix pass to assert the corrected behavior — each keeps its
//! finding lineage in its doc comment.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, FRAC_PI_6, PI, TAU};

use geom::Curve3;
use geom::Surface;
use geom_brep::{
    CertCheck, CertifyError, DihedralClass, EdgeCurve, EdgeCurveSpec, EdgeGeometry, MappedCurve,
    NewellError, SketchSegment, SurfaceKey, classify_dihedral, newell_plane,
};
use geom_core::{Affine3, Band, Point2, Point3, Vec3};
use geom_core::Tol;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn eps() -> f64 {
    Tol::witness().get().eps
}

/// A resolver over a fixed table (keys minted through a local slotmap,
/// mirroring how a Body resolves) — the same shape as the crate's own
/// unit-test helper.
fn table(
    surfs: Vec<Surface<f64>>,
) -> (Vec<SurfaceKey>, impl Fn(SurfaceKey) -> Option<Surface<f64>>) {
    let mut map: slotmap::SlotMap<SurfaceKey, Surface<f64>> = slotmap::SlotMap::with_key();
    let keys: Vec<SurfaceKey> = surfs.into_iter().map(|s| map.insert(s)).collect();
    (keys, move |k| map.get(k).cloned())
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

/// FIXED (was `finding_winding_aliased_arc_interval_certifies`): a
/// quarter-arc description against a carrier whose stored interval
/// winds EIGHT extra full turns — every schedule sample lands on the
/// true point (the extra 8·tau contributes i whole turns at sample i),
/// so the 9-sample schedule alone was aliased. The fix (S1): circle
/// carriers enforce the winding bound |t1 − t0| ≤ tau
/// (`CertifyError::WindingExceeded`), and since the only exact aliases
/// of the uniform schedule are the 8k·tau family, the length bound
/// kills every k ≠ 0 — the pointwise sample matches then pin the
/// winding within the period.
#[test]
fn fixed_winding_aliased_arc_interval_refused() {
    let bulge = (PI / 8.0).tan(); // quarter arc, CCW, unit circle
    let desc = MappedCurve::PlacedSegment {
        segment: SketchSegment::Arc {
            a: Point2::new(1.0, 0.0),
            b: Point2::new(0.0, 1.0),
            bulge,
        },
        place: Affine3::identity(),
    };
    let mk = |t1: f64| EdgeCurveSpec {
        description: EdgeGeometry::MappedCurve(desc),
        carrier: Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        },
        param_start: 0.0,
        param_end: t1,
    };
    let p0 = Point3::new(1.0, 0.0, 0.0);
    let p1 = Point3::new(0.0, 1.0, 0.0);
    // WRONG: 8 extra full windings — the exact schedule alias. Refused
    // by the winding bound, by detection.
    let aliased = mk(FRAC_PI_2 + 8.0 * TAU);
    assert_eq!(
        EdgeCurve::certify(aliased, p0, p1, |_| None, band()).unwrap_err(),
        CertifyError::WindingExceeded
    );
    // The correct interval still certifies.
    EdgeCurve::certify(mk(FRAC_PI_2), p0, p1, |_| None, band()).unwrap();
}

/// FIXED (was `finding_winding_aliased_full_period_certifies`, same
/// family): a one-revolution RevolvedPoint description against a
/// NINE-revolution carrier interval — refused by the winding bound.
#[test]
fn fixed_winding_aliased_full_period_refused() {
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
    assert_eq!(
        EdgeCurve::certify(spec, p, p, |_| None, band()).unwrap_err(),
        CertifyError::WindingExceeded
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

/// FIXED (was `finding_intersection_arc_side_and_winding_unpinned`):
/// with the same endpoints, the carrier taking the COMPLEMENTARY arc
/// and the carrier winding an extra 1.5 turns both certified — every
/// sample lay on both surfaces and the endpoints pinned, so the stored
/// interval's arc/winding was unverified between endpoints. The fixes
/// (S2 + S1 + N1): the witness is now contractually the edge's
/// **mid-parameter point** (`CertCheck::WitnessMidpoint` pins
/// `carrier((t0+t1)/2)` to it), the winding bound caps circle spans at
/// one period, and the forward gate refuses decreasing intervals. For
/// these endpoints the complementary arc is traversable only with a
/// decreasing interval (endpoint pinning forces t0 ≡ 0, t1 ≡ π mod
/// tau), so it dies at the forward gate; the 1.5-winding interval dies
/// at the winding bound; and a complementary-arc witness would die at
/// the mid-parameter pin. Residual freedom after all three: a joint
/// whole-period translation of both ends — geometrically invisible.
#[test]
fn fixed_intersection_arc_side_and_winding_pinned() {
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
    // The complementary (lower) arc: t runs 0 -> −pi — a decreasing
    // interval, refused by the forward gate (N1).
    assert_eq!(
        EdgeCurve::certify(mk(-PI), p0, p1, &lookup, band()).unwrap_err(),
        CertifyError::IntervalNotForward
    );
    // An extra 1.5 windings: t runs 0 -> 3·pi — over one period,
    // refused by the winding bound (S1).
    assert_eq!(
        EdgeCurve::certify(mk(3.0 * PI), p0, p1, &lookup, band()).unwrap_err(),
        CertifyError::WindingExceeded
    );
    // The honest upper arc (witness at its mid-parameter point (0,1,0))
    // still certifies.
    EdgeCurve::certify(mk(PI), p0, p1, &lookup, band()).unwrap();
    // And a wrong-arc WITNESS on an otherwise-honest interval is caught
    // by the mid-parameter pin (S2): witness at the lower arc's
    // midpoint, interval traversing the upper arc.
    let mut wrong_side = mk(PI);
    let EdgeGeometry::Intersection { s1, s2, .. } = wrong_side.description else {
        panic!("mk builds an Intersection description");
    };
    wrong_side.description = EdgeGeometry::Intersection {
        s1,
        s2,
        witness: Point3::new(0.0, -1.0, 0.0),
    };
    assert_eq!(
        EdgeCurve::certify(wrong_side, p0, p1, &lookup, band()).unwrap_err(),
        CertifyError::ResidualExceeded {
            check: CertCheck::WitnessMidpoint,
            sample: 4
        }
    );
}

/// FIXED (was
/// `finding_full_period_rim_intersection_refused_via_zero_chord`, the
/// PR 5 handoff blocker): a FULL-PERIOD rim — the circle where a cap
/// plane meets a cylinder at a genuine right angle, stored as a
/// self-loop edge — was refused as an Intersection because the
/// transversality margin folded in the edge-chord extent, and a closed
/// edge's chord is ZERO (margin 0 -> "definitely Smooth" ->
/// NotTransverse). The fix (B2): the extent arm is now
/// `geom_brep::edge_extent` — for circle carriers
/// `max(chord, r·(1 − cos(Δt/2)))`, which reaches the honest carrier
/// diameter 2r at closure — so the 90° rim classifies Transverse and
/// certifies. The witness sits at the carrier's mid-parameter point
/// (t = π ⇒ (−1, 0, 0)), per the sharpened S2 witness contract.
#[test]
fn fixed_full_period_rim_intersection_certifies() {
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
            // The mid-parameter point of t: 0 → tau is carrier(pi).
            witness: Point3::new(-1.0, 0.0, 0.0),
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
    EdgeCurve::certify(spec, p, p, &lookup, band()).unwrap();
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

/// (a) FIXED (was
/// `finding_sub_epsilon_cone_arm_reads_definitely_smooth`): near the
/// cone apex the arms collapse; a collapsed lever arm used to map
/// every angle into the Zero band, so a ~60-degree transverse wedge at
/// rho <= eps read **definitely Smooth**. The fix (B2): the
/// collapsed-arm gate — `classify_dihedral` decides the arm first
/// (predicate "dihedral_arm") and a coincident-with-zero arm ESCALATES
/// (Indeterminate), never classifies definitely.
#[test]
fn fixed_sub_epsilon_cone_arm_escalates() {
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
    // In-band arm: honest escalation, as before.
    let p = at(3.0 * eps());
    assert!(classify_dihedral(&cone, &plane_through(p), p, 1.0, band()).is_err());
    // Collapsed arm: now an honest escalation naming the arm gate —
    // never a definite classification.
    let p = at(0.5 * eps());
    let err = classify_dihedral(&cone, &plane_through(p), p, 1.0, band()).unwrap_err();
    assert_eq!(err.predicate, Some("dihedral_arm"));
}

/// (c) FIXED (was `finding_sub_epsilon_chord_true_corner_reads_smooth`):
/// a true right-angle corner on a sub-epsilon (or exactly zero) extent
/// used to read definitely Smooth. Now the collapsed-arm gate
/// escalates both cases — "arm too small to say" is an escalation,
/// never a classification (B2). (Closed edges no longer reach this
/// gate through their chord at all: certification and tier 3 pass the
/// honest carrier-derived extent — see
/// `fixed_full_period_rim_intersection_certifies`.)
#[test]
fn fixed_sub_epsilon_extent_true_corner_escalates() {
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
    let err = classify_dihedral(&floor, &wall, Point3::origin(), 0.5 * eps(), band()).unwrap_err();
    assert_eq!(err.predicate, Some("dihedral_arm"), "sub-eps extent");
    let err = classify_dihedral(&floor, &wall, Point3::origin(), 0.0, band()).unwrap_err();
    assert_eq!(err.predicate, Some("dihedral_arm"), "zero extent");
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

/// FIXED (was `finding_reversed_interval_certifies`): a DECREASING
/// parameter interval certified when description and carrier were
/// arranged consistently, violating the ratified he_plus-forward
/// increasing-parameter convention. The fix (N1): the forward span
/// gate — `CertifyError::IntervalNotForward`, a typed error, since the
/// convention is ratified.
#[test]
fn fixed_reversed_interval_refused() {
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
    assert_eq!(
        EdgeCurve::certify(spec, p1, p0, |_| None, band()).unwrap_err(),
        CertifyError::IntervalNotForward
    );
}

/// FIXED (was `finding_zero_length_edge_certifies`): a ZERO-LENGTH
/// edge — coincident endpoints, degenerate interval (t0 == t1),
/// constant description — certified (all nine samples coincide), and
/// its dihedral margin was vacuously zero. The call (N2): REFUSE at
/// certification, through the same forward span gate as N1 (a
/// degenerate span is indistinguishable from a reversed one at the
/// gate, and no M2 construction needs zero-length edges — seed states
/// are edge-free `mvfs` products, coincident-endpoint chords already
/// refused as poison, self-loop scaffolding carries the full period).
#[test]
fn fixed_zero_length_edge_refused() {
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
    assert_eq!(
        EdgeCurve::certify(spec, p, p, |_| None, band()).unwrap_err(),
        CertifyError::IntervalNotForward
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

/// FIXED-BY-DOCUMENTATION (was
/// `finding_near_collinear_normal_is_noise_determined_but_certified`,
/// N3): NEAR-collinear loops whose lifts stay under eps are certified
/// with a noise-determined normal — two inputs differing by 0.2·eps
/// get near-OPPOSITE certified normals. The residual contract ("every
/// vertex within eps of the plane") is honestly met; the normal's
/// DIRECTION is inherently under-determined by such data (exact
/// collinearity escalates; the near-collinear band cannot). The N3
/// resolution: no behavior change — the under-determination is now
/// documented on `newell_plane`'s module docs ("What certification
/// does NOT pin"), and this test PINS the documented behavior.
#[test]
fn fixed_n3_near_collinear_normal_documented_under_determination() {
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
        "documented under-determination (newell docs): sub-eps perturbation \
         swings the certified normal (dot = {})",
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

    /// FIXED (was
    /// `finding_interval_winding_alias_refused_only_by_the_poison`):
    /// the interval lane used to refuse the 9-revolution alias only via
    /// the blanket norm-sqrt-clamp poison (every inexact distance
    /// enclosure degraded to Trv). With the poison fixed (B1: tight
    /// per-component squares in `norm_squared`), the alias must STILL
    /// be refused — and now it is, by DETECTION: the circle winding
    /// bound classifies `(tau − 9·tau)·r` definitely negative through
    /// a clean (Com-decorated) enclosure and returns the typed
    /// `WindingExceeded`, not an escalation.
    #[test]
    fn fixed_interval_winding_alias_refused_by_detection() {
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
        assert_eq!(
            err,
            CertifyError::WindingExceeded,
            "the fixed lane must refuse the alias by detection, not poison"
        );
    }

    /// FIXED (was
    /// `finding_interval_scaffolding_self_loop_circle_uncertifiable`,
    /// BLOCKER B1): the canonical SCAFFOLDING SELF-LOOP CIRCLE — the
    /// spec the sugar attaches at every self-loop site — was
    /// uncertifiable at the interval scalar: `carrier(tau)` does not
    /// reproduce the anchor's singleton exactly, the difference
    /// enclosure straddles zero, and `norm`'s old `sqrt(dot(v, v))`
    /// squared the straddle through plain interval `Mul` (negative
    /// lo), so the sqrt clamped, degraded the decoration to Trv, and
    /// every decision downstream read poison. The fix: `norm_squared`
    /// sums tight per-component squares (`powi(2)` — the interval
    /// backend's dedicated integer power),
    /// whose enclosures are `[0, hi]` with decoration preserved, so
    /// the sqrt never clamps and the endpoint residual classifies
    /// Zero through a clean enclosure. The scaffolding convention now
    /// certifies at Interval.
    #[test]
    fn fixed_interval_scaffolding_self_loop_circle_certifies() {
        let p = Point3::new(
            Interval::from_f64(7.0),
            Interval::from_f64(7.0),
            Interval::from_f64(7.0),
        );
        let spec = EdgeCurveSpec::self_loop_circle_at(p);
        let curve = EdgeCurve::certify(spec, p, p, |_| None, band()).unwrap();
        let r = curve.certificate().max_residual;
        assert!(r.hi().is_finite(), "clean certificate, no poison: {r:?}");
    }
}
