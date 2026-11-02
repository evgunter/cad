//! Blinded-review probes for PR #524 (M9-1 PR-1, head a557da3b).
//!
//! Each probe EXECUTES one claim from the review brief; assertions pin
//! the observed behaviour of the head, including behaviour the review
//! reports as findings (marked FINDING below).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished};
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};
use topo::boolean::contact_verify::tangent_locus_relation;
use topo::{
    CarrierDesc, CarrierRelation, ContactRefusal, ValidationError, carrier_eq,
    validate_pseudomanifold,
};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A ball of radius `arm` about the origin: the extent a bare arm
/// names.
fn at(arm: f64) -> topo::ConsumedExtent<f64> {
    topo::ConsumedExtent::new(geom_brep::ExtentBall::new(Point3::origin(), arm))
}

fn sphere(c: [f64; 3], r: f64, outward: bool) -> CarrierDesc<f64> {
    CarrierDesc::Sphere {
        center: Point3::from_array(c),
        radius: r,
        outward,
    }
}

fn cyl(o: [f64; 3], a: [f64; 3], r: f64, outward: bool) -> CarrierDesc<f64> {
    CarrierDesc::Cylinder {
        origin: Point3::from_array(o),
        axis: Vec3::from_array(a),
        radius: r,
        outward,
    }
}

fn plane_s(o: [f64; 3], n: [f64; 3]) -> geom::Surface<f64> {
    geom::Surface::Plane {
        origin: Point3::from_array(o),
        normal: Vec3::from_array(n),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn xline() -> geom::Curve3<f64> {
    geom::Curve3::Line {
        origin: Point3::new(0.0, 0.0, 0.0),
        dir: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// CLAIM 1 — length margins are metered at UNIT arm: the arm must not
/// change a sphere-center or radius verdict (levering a length by the
/// arm would price the same error twice).
#[test]
fn probe_sphere_length_margins_ignore_the_arm() {
    let a = sphere([0.0, 0.0, 0.0], 2.0, true);
    let off = sphere([1e-3, 0.0, 0.0], 2.0, false);
    for arm in [1e-3, 1.0, 1e3] {
        assert_eq!(
            carrier_eq(&a, &off, &at(arm), band()).unwrap(),
            CarrierRelation::Distinct,
            "definite center offset must be Distinct at arm {arm}"
        );
    }
    // A zero-band center offset: also arm-independent.
    let near = sphere([1e-13, 0.0, 0.0], 2.0, false);
    for arm in [1e-3, 1e3] {
        assert_eq!(
            carrier_eq(&a, &near, &at(arm), band()).unwrap(),
            CarrierRelation::SameOpposite,
            "a zero-band center offset is one carrier at arm {arm}"
        );
    }
}

/// CLAIM 1 — the ANGULAR margin is levered at the arm: a tilt that is
/// in the zero band at 1 m must become a definite distinction at a
/// large arm.
#[test]
fn probe_cylinder_angular_margin_is_levered_at_the_arm() {
    let eps = geom_core::Tol::witness().get().eps;
    let tilt = eps / 10.0; // sin ~ tilt: sub-band at unit arm
    let a = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, true);
    let axis = Vec3::new(tilt, 0.0, 1.0).normalize();
    let b = CarrierDesc::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis,
        radius: 3.0,
        outward: false,
    };
    // Unit arm: the tilt decides zero.
    assert_eq!(
        carrier_eq(&a, &b, &at(1.0), band()).unwrap(),
        CarrierRelation::SameOpposite
    );
    // Large arm: the same tilt is a definite misalignment there.
    assert_eq!(
        carrier_eq(&a, &b, &at(1e6), band()).unwrap(),
        CarrierRelation::Distinct
    );
}

/// CLAIM 1 — mm-vs-metre twin: scaling the MODEL by 1000 must scale
/// every length margin with it (no dimensionless comparand hides in
/// the ladder), and a definite margin past an in-band one decides.
#[test]
fn probe_mm_vs_metre_twin_and_margin_naming() {
    // metre model: radii 3.0 vs 3.001 (definite delta 1e-3 m).
    let m1 = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, true);
    let m2 = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.001, false);
    // "mm" model: same physical part authored 1000x (radii 3000/3001).
    let k1 = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3000.0, true);
    let k2 = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3001.0, false);
    for (a, b) in [(&m1, &m2), (&k1, &k2)] {
        assert_eq!(
            carrier_eq(a, b, &at(1.0), band()).unwrap(),
            CarrierRelation::Distinct
        );
    }
    // Near-tie walk order: axis offset in the zero band, radius definite
    // — the ladder walks PAST the offset to the radius.
    let near = cyl([1e-13, 0.0, 0.0], [0.0, 0.0, 1.0], 3.001, false);
    assert_eq!(
        carrier_eq(&m1, &near, &at(1.0), band()).unwrap(),
        CarrierRelation::Distinct
    );
}

/// CLAIM 2 — a DEFINITE separation contradicts: two parallel planes a
/// definite gap apart, read tangent along a
/// line lying on the first. The pair's separation is read before the
/// witness, so the refusal is the gap (positive: apart), not the
/// witness's miss on the second plane.
#[test]
fn probe_tangent_definite_separation_contradicts() {
    let up = plane_s([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let down = plane_s([0.0, 0.0, 1e-3], [0.0, 0.0, 1.0]);
    let err =
        tangent_locus_relation(&up, true, &down, false, &xline(), -1.0, 1.0, band()).unwrap_err();
    match err {
        topo::ContactRefusal::Contradicted { diag } => {
            assert_eq!(diag.predicate, Some("contact_tangent_gap"));
            let gap = diag.margin.diagnostic_f64_for_error_text().value();
            assert!(
                gap.is_some_and(|g| g > 0.0),
                "apart, not crossing: {diag:?}"
            );
        }
        other => panic!("expected Contradicted, got {other:?}"),
    }
}

/// CLAIM 2 — a DEFINITE crossing (two planes at a definite angle, read
/// tangent along their intersection line) contradicts.
#[test]
fn probe_tangent_definite_crossing_contradicts() {
    let up = plane_s([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let tilted = plane_s([0.0, 0.0, 0.0], [0.0, 1e-3, 1.0]);
    let out = tangent_locus_relation(&up, true, &tilted, false, &xline(), -1.0, 1.0, band());
    assert!(
        matches!(out, Err(topo::ContactRefusal::Contradicted { .. })),
        "a definite crossing must contradict, got {out:?}"
    );
}

/// An in-band on-surface residual escalates: it is on C4's
/// must-verify-DEFINITE list, so it is neither verified nor
/// contradicted.
#[test]
fn probe_tangent_escalates_an_inband_residual() {
    let eps = geom_core::Tol::witness().get().eps;
    let up = plane_s([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    // A genuine (authored) gap of 3 eps: in the sliver band.
    let gapped = plane_s([0.0, 0.0, 3.0 * eps], [0.0, 0.0, 1.0]);
    let refusal = tangent_locus_relation(&up, true, &gapped, false, &xline(), -1.0, 1.0, band())
        .expect_err("an in-band residual refuses");
    assert!(
        matches!(refusal, ContactRefusal::Escalated { .. }),
        "in-band residual must escalate: {refusal:?}"
    );
}

/// CLAIM 4 — the at-rest gate refuses a false edge–face record: a top
/// edge of a brick recorded as touching its bottom face, a metre off
/// its plane, is stale.
#[test]
fn probe_at_rest_gate_refuses_a_false_edge_face_record() {
    let body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let z_face = |z: f64| {
        body.faces()
            .find(|(_, f)| match body.get_surface(f.surface) {
                Some(&topo::Surface::Plane { origin, normal, .. }) => {
                    normal.z.abs() > 0.5 && (origin.z - z).abs() < 1e-12
                }
                _ => false,
            })
            .map(|(k, _)| k)
            .unwrap()
    };
    let (top, bottom) = (z_face(1.0), z_face(0.0));
    // A top edge: a Line carrier lying ON the top plane, 1 m from the
    // bottom plane — definite separation from face_b.
    let witness = body
        .edges()
        .map(|(k, _)| k)
        .find(|&e| {
            topo::Contact::new(&body, topo::Cell::Edge(e), topo::Cell::Face(top)).is_err()
                && topo::Contact::new(&body, topo::Cell::Edge(e), topo::Cell::Face(bottom)).is_ok()
        })
        .expect("an edge of the top");
    let contacts = topo::test_support::records(
        &body,
        [(topo::Cell::Edge(witness), topo::Cell::Face(bottom))],
    );
    let errs = validate_pseudomanifold(&body, &contacts, Tol::witness())
        .expect_err("false record must fail");
    assert!(
        errs.contains(&ValidationError::StaleContactDeclaration {
            declaration: topo::StaleDeclaration::EdgeFace {
                edge: witness,
                face: bottom,
            },
        }),
        "the at-rest gate reads the record stale: {errs:?}"
    );
}

/// CLAIM 6 — nothing certifies a false patch: a body carrying a patch
/// record is refused typed at rest (CensusUnsupported), never blessed.
#[test]
fn probe_patch_contact_is_never_certified_at_rest() {
    let body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let mut it = body.faces().map(|(k, _)| k);
    let (fa, fb) = (it.next().unwrap(), it.next().unwrap());
    let contacts = topo::ContactRecords::from_iter(vec![topo::Cited::new(
        topo::Contact::new(&body, topo::Cell::Face(fa), topo::Cell::Face(fb)).unwrap(),
        topo::Cites::decided(0),
    )]);
    let errs = validate_pseudomanifold(&body, &contacts, Tol::witness())
        .expect_err("a patch record cannot be confirmed");
    // Since M9-2 the certifier EXISTS: two arbitrary brick faces are
    // DISTINCT carriers, so the fabricated record is CONTRADICTED —
    // the probe's real claim (a fabricated patch never blesses)
    // survives with the honest refusal upgraded from
    // not-yet-certifiable to contradicted.
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::ContactContradicted { .. })),
        "a fabricated patch record must refuse typed: {errs:?}"
    );
}

/// CLAIM 5 — the replay row's PartialEq BITES: a mutated record set
/// compares unequal (guards against a derive that ignores the new
/// fields).
#[test]
fn probe_records_partialeq_bites_on_mutation() {
    let a = finished(
        "a",
        brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness()),
        Tol::witness(),
    );
    let b = finished(
        "b",
        brick::<f64>((0.5, 1.5), (0.25, 1.25), (1.0, 2.0), Tol::witness()),
        Tol::witness(),
    );
    let topo::BooleanResult::Body(out) = topo::union(&a, &b, Tol::witness()).unwrap() else {
        panic!("union is a body");
    };
    let mut mutated = out.contacts.clone();
    let mut faces = out.body.faces().map(|(k, _)| k);
    let (fa, fb) = (faces.next().unwrap(), faces.next().unwrap());
    mutated.extend([topo::Cited::new(
        topo::Contact::new(&out.body, topo::Cell::Face(fa), topo::Cell::Face(fb)).unwrap(),
        topo::Cites::decided(0),
    )]);
    assert_ne!(out.contacts, mutated, "PartialEq must see the new fields");
}

/// CLAIM 7 — exactly-equal carriers are one carrier, and a definite
/// difference is two.
#[test]
fn probe_equal_carriers_are_one_and_a_definite_difference_two() {
    let a = sphere([0.0, 0.0, 0.0], 2.0, true);
    let b = sphere([0.0, 0.0, 0.0], 2.0, false);
    assert_eq!(
        carrier_eq(&a, &b, &at(1.0), band()).unwrap(),
        CarrierRelation::SameOpposite
    );
    let off = sphere([0.0, 0.0, 0.0], 2.5, false);
    assert_eq!(
        carrier_eq(&a, &off, &at(1.0), band()).unwrap(),
        CarrierRelation::Distinct
    );
}
