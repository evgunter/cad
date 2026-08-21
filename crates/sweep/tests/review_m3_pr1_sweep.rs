//! M3 PR 1 adversarial review, sweep-consumer half: split_edge on real
//! arc geometry (the bulge' restriction formula, derived independently
//! from bulge = tan(theta/4)), and revert driven on genuinely swept
//! bodies (prism posture; curved refusal).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::EdgeGeometry;
use geom_core::Point2;
use geom_core::Tol;
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, extrude};
use topo::{Body, ValidationError, validate_closed, validate_geometric};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn validated(loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .unwrap()
}

/// A D-shaped profile: one arc (bulge = tan(pi/8), a 90-degree sweep)
/// plus lines - gives cap rims carrying Arc sketch segments.
fn d_profile() -> ValidatedProfile<f64> {
    validated(vec![ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        // 90-degree arc
        ProfileVertex::new(p2(1.0, 0.0), (core::f64::consts::PI / 8.0).tan()),
        ProfileVertex::new(p2(1.0, 1.0), 0.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ])])
}

/// The all-line L profile (prism: every face a plane).
fn l_profile() -> ValidatedProfile<f64> {
    validated(vec![ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(2.0, 0.0),
        p2(2.0, 1.0),
        p2(1.0, 1.0),
        p2(1.0, 2.0),
        p2(0.0, 2.0),
    ])])
}

/// TARGET 3 (the bulge' formula, checked against the repo's own
/// convention). SCOPE FINDING pinned here: extruded bodies at rest
/// carry NO PlacedSegment descriptions (the prefer-intrinsic pass has
/// already re-described every edge as Intersection), so split_edge's
/// SketchSegment::restrict lane is unreachable from any current public
/// at-rest body - this test exercises it at the geom-brep consumer
/// level instead. With bulge = tan(theta/4), the sub-arc over param
/// fractions [s0, s1] has theta' = theta * (s1 - s0), so bulge' must
/// be EXACTLY tan(atan(b) * (s1 - s0)); endpoints are eval(s0)/
/// eval(s1) bitwise; and the reparameterization law
/// restrict(s0, s1).eval(s) ~= eval(s0 + (s1 - s0) * s) holds.
#[test]
fn arc_bulge_restriction_formula_derived_independently() {
    use geom_brep::SketchSegment;
    let bulge = (core::f64::consts::PI / 8.0).tan(); // 90-degree arc
    let seg = SketchSegment::Arc {
        a: p2(1.0, 0.0),
        b: p2(1.0, 1.0),
        bulge,
    };
    let (s0, s1) = (0.3_f64, 0.85_f64);
    let sub = seg.restrict(s0, s1);
    let SketchSegment::Arc { a, b, bulge: bp } = sub else {
        panic!("restriction changed the segment kind");
    };
    // Independent derivation of the sub-arc bulge.
    let theta = 4.0 * bulge.atan();
    let expected = (theta * (s1 - s0) / 4.0).tan();
    assert_eq!(bp.to_bits(), expected.to_bits(), "bulge' formula mismatch");
    let (ea, eb) = (seg.eval(s0), seg.eval(s1));
    assert_eq!(
        (a.x.to_bits(), a.y.to_bits()),
        (ea.x.to_bits(), ea.y.to_bits())
    );
    assert_eq!(
        (b.x.to_bits(), b.y.to_bits()),
        (eb.x.to_bits(), eb.y.to_bits())
    );
    // Reparameterization law, sampled densely (float slack only).
    for i in 0..=16 {
        let s = f64::from(i) / 16.0;
        let via_sub = sub.eval(s);
        let via_parent = seg.eval(s0 + (s1 - s0) * s);
        let d = ((via_sub.x - via_parent.x).powi(2) + (via_sub.y - via_parent.y).powi(2)).sqrt();
        assert!(d < 1e-12, "restrict/eval law broken at s={s}: {d}");
    }
    // Degenerate probe: restricting to a sliver stays finite and lands
    // on the parent (certification, not restrict, is the gate).
    let sliver = seg.restrict(0.5, 0.5 + 1e-9);
    let SketchSegment::Arc { bulge: bs, .. } = sliver else {
        panic!("kind");
    };
    assert!(bs.is_finite() && bs > 0.0);
}

/// TARGET 3 on a REAL curved body: split the D-body's arc wall rim (a
/// circle-carrier plane-x-cylinder Intersection edge) - children stay
/// Intersection with bitwise mid-sample witnesses, tier 3 is preserved,
/// and interiority is metered in METERS (angle x radius): an angular
/// margin that is comfortably over eps must still refuse when the
/// radius shrinks it into the band.
#[test]
fn split_circle_carrier_intersection_edge() {
    let out = extrude(&d_profile(), Extrusion::Distance(1.0), Tol::witness()).unwrap();
    let mut body: Body<f64> = out.body;
    assert_eq!(validate_geometric(&body, Tol::witness()), Ok(()));
    let (edge, parent) = body
        .edges()
        .find_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?.clone();
            matches!(c.carrier(), geom::Curve3::Circle { .. }).then_some((k, c))
        })
        .expect("the D-body must carry a circular rim");
    let (t0, t1) = parent.params();
    let t = t0 + (t1 - t0) * 0.3;
    let created = body.split_edge(edge, t, Tol::witness()).unwrap();
    // RE-PINNED at the M5 PR 9 fix pass: this row used to pin the
    // NotIsoRectangle/VolumeUncomputable refusal here, and that pin's
    // failure under PR 9 is the EVIDENCE the du_of_rims repair works.
    // The M2-era rule took the FIRST rim arc's span as the face's Δu,
    // which on the merge base is silently WRONG the moment a rim
    // arrives as several EQUAL-span arcs (two public split_edge calls
    // suffice: volume 0.6545 vs the true 0.7854 with
    // validate_geometric green — the silent-wrong-volume class the
    // review executed at merge base). PR 9 sums spans per (rim level,
    // direction) group, so a split rim integrates EXACTLY: tier 3
    // holds straight through the split.
    assert_eq!(
        validate_geometric(&body, Tol::witness()),
        Ok(()),
        "tier 3 must survive a rim split since the du_of_rims repair"
    );
    assert_eq!(validate_closed(&body), Ok(()), "tier 2 survives too");
    for (curve_key, (ta, tb)) in [
        (created.first_curve, (t0, t)),
        (created.second_curve, (t, t1)),
    ] {
        let child = body.get_curve_geom(curve_key).unwrap().certified().unwrap();
        let EdgeGeometry::Intersection { witness, .. } = *child.description() else {
            panic!("child lost its Intersection description");
        };
        let expected = parent.carrier().eval(ta + (tb - ta) * 0.5);
        assert_eq!(
            (
                witness.x.to_bits(),
                witness.y.to_bits(),
                witness.z.to_bits()
            ),
            (
                expected.x.to_bits(),
                expected.y.to_bits(),
                expected.z.to_bits()
            ),
            "circle-carrier witness is not the bitwise mid-sample"
        );
    }
    // Meters metering: the second child's angular span is ~0.7 * 3pi/4;
    // ask for a split whose arc-length margin (angle * radius,
    // r = 1/sqrt(2)) sits inside the zero band although the raw
    // angular margin is above eps.
    let eps = Tol::witness().get().eps;
    let geom::Curve3::Circle { radius, .. } = parent.carrier().clone() else {
        panic!("circle carrier vanished");
    };
    assert!(radius < 1.0);
    let (c0, c1) = body
        .get_curve_geom(body.get_edge(created.new_edge).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap()
        .params();
    let angular_margin = eps / radius * 0.9; // meters-margin inside band
    assert!(angular_margin > eps, "probe needs r < 0.9 to be meaningful");
    let err = body
        .split_edge(created.new_edge, c0 + angular_margin, Tol::witness())
        .unwrap_err();
    assert!(
        matches!(
            err,
            topo::EulerOpError::SplitParamNotInterior { .. }
                | topo::EulerOpError::SplitParamEscalated { .. }
        ),
        "arc-length (meters) metering did not gate the split: {err:?}"
    );
    let _ = c1;
}

/// TARGET 4: revert on a genuinely swept planar prism - tier-2
/// currency, tier 3 failing with EXACTLY NegativeVolume, bitwise
/// involution, D9 determinism, and the exact negated volume.
#[test]
fn revert_extruded_prism_posture() {
    let out = extrude(&l_profile(), Extrusion::Distance(2.0), Tol::witness()).unwrap();
    let body: Body<f64> = out.body;
    assert_eq!(validate_geometric(&body, Tol::witness()), Ok(()));
    let original = format!("{body:?}");
    let vol = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
    assert!(vol > 0.0);
    let reverted = body.revert().unwrap();
    assert_eq!(format!("{body:?}"), original, "revert mutated its operand");
    assert_eq!(validate_closed(&reverted), Ok(()));
    assert_eq!(
        validate_geometric(&reverted, Tol::witness()),
        Err(vec![ValidationError::NegativeVolume]),
        "tier 3 on a reverted prism must be exactly NegativeVolume"
    );
    let rvol = topo::mass_properties(&reverted, Tol::witness())
        .unwrap()
        .volume;
    assert_eq!(rvol.to_bits(), (-vol).to_bits());
    // Involution + determinism, bitwise through the Debug channel.
    assert_eq!(format!("{:?}", reverted.revert().unwrap()), original);
    assert_eq!(
        format!("{:?}", body.revert().unwrap()),
        format!("{reverted:?}")
    );
}

/// TARGET 4, FLIPPED TO A CONSTRUCTION ROW (M5 S12; was
/// `revert_curved_body_refuses_typed`). The D-body's arc wall is a
/// cylinder, which had no reversed form in the closed analytic enum —
/// S10 ratified `Face::sense`, S11 made the incoming bits honest and
/// S12 wired the flip, so this body now REVERTS. What TARGET 4 really
/// pinned was the operand-untouched and typed-frontier discipline;
/// what survives it is the full M3 contract on a CURVED body: the
/// operand is untouched, the result is tier-2 currency but exactly
/// `NegativeVolume` at tier 3, the volume is the bitwise negation, and
/// `revert` is a bitwise involution and a deterministic function.
#[test]
fn revert_curved_body_reverts_via_the_sense_bit() {
    let out = extrude(&d_profile(), Extrusion::Distance(1.0), Tol::witness()).unwrap();
    let body: Body<f64> = out.body;
    assert_eq!(validate_geometric(&body, Tol::witness()), Ok(()));
    let before = format!("{body:?}");
    let vol = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
    assert!(vol > 0.0);
    // There really is a non-plane surface here (the arc wall).
    let curved: Vec<_> = body
        .faces()
        .filter(|(_, f)| {
            !matches!(
                body.get_surface(f.surface).unwrap(),
                topo::Surface::Plane { .. }
            )
        })
        .map(|(k, f)| (k, f.sense))
        .collect();
    assert!(!curved.is_empty(), "the D-body has a cylinder wall");

    let reverted = body.revert().expect("S12: curved revert is wired");
    assert_eq!(format!("{body:?}"), before, "revert mutated its operand");
    // Exactly the curved faces flipped, and they flipped from their
    // OWN bit (not stamped to a constant).
    for (k, was) in &curved {
        assert_eq!(
            reverted.get_face(*k).unwrap().sense,
            !*was,
            "curved face {k:?} did not flip"
        );
    }
    // The M3 contract, unchanged, on a curved body.
    assert_eq!(validate_closed(&reverted), Ok(()));
    assert_eq!(
        validate_geometric(&reverted, Tol::witness()),
        Err(vec![ValidationError::NegativeVolume]),
        "tier 3 on a reverted curved body must be exactly NegativeVolume"
    );
    let rvol = topo::mass_properties(&reverted, Tol::witness())
        .unwrap()
        .volume;
    assert_eq!(rvol.to_bits(), (-vol).to_bits());
    assert_eq!(format!("{:?}", reverted.revert().unwrap()), before);
    assert_eq!(
        format!("{:?}", body.revert().unwrap()),
        format!("{reverted:?}")
    );
}

/// TARGETS 2+3 on swept bodies: a split then a null strut on the
/// prism - split_edge keeps the prism tier-3; the null edge then
/// closes the tier-3/mass-props doors until killed (the fail-loud
/// lifecycle on a real consumer body, not a fixture).
#[test]
fn split_then_null_lifecycle_on_prism() {
    let out = extrude(&l_profile(), Extrusion::Distance(1.0), Tol::witness()).unwrap();
    let mut body: Body<f64> = out.body;
    let vol0 = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
    // Split some line edge mid-span.
    let (edge, curve) = body
        .edges()
        .find_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?.clone();
            matches!(c.carrier(), geom::Curve3::Line { .. }).then_some((k, c))
        })
        .unwrap();
    let (t0, t1) = curve.params();
    body.split_edge(edge, t0 + (t1 - t0) * 0.5, Tol::witness())
        .unwrap();
    assert_eq!(validate_geometric(&body, Tol::witness()), Ok(()));
    assert_eq!(
        topo::mass_properties(&body, Tol::witness())
            .unwrap()
            .volume
            .to_bits(),
        vol0.to_bits(),
        "split changed the prism's exact volume"
    );
    // Null strut: the doors close, typed.
    let v = body.vertices().next().map(|(k, _)| k).unwrap();
    let he = body.get_vertex(v).unwrap().emanating.unwrap();
    let created = body
        .mev_null(
            topo::MevSite::Fan { he1: he, he2: he },
            topo::NewVertexSide::Below,
        )
        .unwrap();
    assert!(matches!(
        topo::mass_properties(&body, Tol::witness()).unwrap_err(),
        topo::MassPropsError::NullScaffoldEdge { .. }
    ));
    assert!(validate_closed(&body).is_err());
    body.kev(created.he_plus).unwrap();
    assert_eq!(validate_geometric(&body, Tol::witness()), Ok(()));
}
