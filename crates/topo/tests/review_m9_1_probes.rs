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
use topo::boolean::plane_eq::PlaneIdentity;
use topo::{
    CarrierDesc, CarrierEqError, CarrierRelation, ContactRefusal, ContactVerdict, ValidationError,
    carrier_eq, validate_pseudomanifold,
};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A ball of radius `arm` about the origin, no point of either face
/// known: the extent a bare arm names.
fn at(arm: f64) -> topo::ConsumedExtent<'static, f64> {
    topo::ConsumedExtent::new(geom_brep::ExtentBall::new(Point3::origin(), arm))
}

fn declared() -> PlaneIdentity {
    PlaneIdentity::DECLARED
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
            carrier_eq(&a, &off, PlaneIdentity::NONE, &at(arm), band()).unwrap(),
            CarrierRelation::Distinct,
            "definite center offset must be Distinct at arm {arm}"
        );
        // The point of `a` on the offset's far side stands the whole
        // offset off it.
        let on = [Point3::new(-2.0, 0.0, 0.0)];
        let extent = topo::ConsumedExtent {
            on: [&on, &[]],
            ..at(arm)
        };
        assert!(
            matches!(
                carrier_eq(&a, &off, declared(), &extent, band()),
                Err(CarrierEqError::Contradicted { .. })
            ),
            "declared, arm {arm}: contradicted"
        );
    }
    // In-band center offset: also arm-independent.
    let near = sphere([1e-13, 0.0, 0.0], 2.0, false);
    for arm in [1e-3, 1e3] {
        assert_eq!(
            carrier_eq(&a, &near, declared(), &at(arm), band()).unwrap(),
            CarrierRelation::SameOpposite,
            "in-band center offset bridges at arm {arm}"
        );
    }
}

/// CLAIM 1 — the ANGULAR margin is levered at the arm: a tilt that is
/// in-band at 1 m must become a definite distinction at a large arm.
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
    // Unit arm: in-band tilt bridges under the declaration.
    assert_eq!(
        carrier_eq(&a, &b, declared(), &at(1.0), band()).unwrap(),
        CarrierRelation::SameOpposite
    );
    // Large arm: the same tilt is a definite misalignment there, at a
    // point of the face half the arm along the axis.
    let on = [Point3::new(3.0, 0.0, 5e5)];
    let extent = topo::ConsumedExtent {
        on: [&on, &[]],
        ..at(1e6)
    };
    match carrier_eq(&a, &b, declared(), &extent, band()).unwrap_err() {
        CarrierEqError::Contradicted { diag: d, .. } => {
            assert_eq!(d.predicate, Some("carrier_cyl_axis_parallel"));
        }
        other => panic!("expected Contradicted at 1e6 arm, got {other:?}"),
    }
}

/// CLAIM 1 — mm-vs-metre twin: scaling the MODEL by 1000 must scale
/// every length margin with it (no dimensionless comparand hides in
/// the ladder), and the first definite margin in walk order decides.
#[test]
fn probe_mm_vs_metre_twin_and_margin_naming() {
    // metre model: radii 3.0 vs 3.001 (definite delta 1e-3 m).
    let m1 = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, true);
    let m2 = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.001, false);
    // "mm" model: same physical part authored 1000x (radii 3000/3001).
    let k1 = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3000.0, true);
    let k2 = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3001.0, false);
    for (a, b) in [(&m1, &m2), (&k1, &k2)] {
        match carrier_eq(a, b, declared(), &at(1.0), band()).unwrap_err() {
            CarrierEqError::Contradicted { diag: d, .. } => {
                assert_eq!(d.predicate, Some("carrier_cyl_radius"));
            }
            other => panic!("expected radius contradiction, got {other:?}"),
        }
    }
    // Near-tie walk order: axis-offset in band, radius definite — the
    // ladder must walk PAST the in-band margin and name the radius.
    let near = cyl([1e-13, 0.0, 0.0], [0.0, 0.0, 1.0], 3.001, false);
    match carrier_eq(&m1, &near, declared(), &at(1.0), band()).unwrap_err() {
        CarrierEqError::Contradicted { diag: d, .. } => {
            assert_eq!(d.predicate, Some("carrier_cyl_radius"));
        }
        other => panic!("expected radius contradiction, got {other:?}"),
    }
}

/// CLAIM 2 — a DEFINITE separation must beat the declaration: two
/// parallel planes a definite gap apart, declared tangent along a
/// line lying on the first. The pair's separation is read before the
/// witness, so the refusal is the gap (positive: apart), not the
/// witness's miss on the second plane.
#[test]
fn probe_tangent_definite_separation_contradicts() {
    let up = plane_s([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let down = plane_s([0.0, 0.0, 1e-3], [0.0, 0.0, 1.0]);
    let err = tangent_locus_relation(&up, true, &down, false, &xline(), -1.0, 1.0, true, band())
        .unwrap_err();
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

/// CLAIM 2 — a DEFINITE crossing (two planes at a definite angle,
/// declared tangent along their intersection line) must beat the
/// declaration: the bridge must not wave a transverse crossing
/// through.
#[test]
fn probe_tangent_definite_crossing_contradicts() {
    let up = plane_s([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let tilted = plane_s([0.0, 0.0, 0.0], [0.0, 1e-3, 1.0]);
    let out = tangent_locus_relation(&up, true, &tilted, false, &xline(), -1.0, 1.0, true, band());
    assert!(
        matches!(out, Err(topo::ContactRefusal::Contradicted { .. })),
        "a definite crossing must contradict the declaration, got {out:?}"
    );
}

/// FINDING (claim 2 / doc honesty) — the module docs and PR body say
/// normal opposition and on-surface residual "stay must-verify-
/// DEFINITE at every sample, so the weakening is confined to the
/// second-order margin". The code BRIDGES an in-band residual and an
/// in-band opposition margin under a declaration: this probe pins the
/// actual behaviour the docs deny.
#[test]
fn probe_tangent_bridges_inband_residual_beyond_the_stated_residue() {
    let eps = geom_core::Tol::witness().get().eps;
    let up = plane_s([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    // A genuine (authored) gap of 3 eps: in the sliver band, so not
    // definite separation — but also NOT the #175 kappa_rel residue.
    let gapped = plane_s([0.0, 0.0, 3.0 * eps], [0.0, 0.0, 1.0]);
    // FIXED (M9-1 fix pass, F1). This probe found the defect and now
    // pins the fix: an in-band on-surface residual is on C4's
    // must-verify-DEFINITE list, so it is neither verified, nor
    // bridged, nor contradicted — it ESCALATES, declaration or not.
    // Red-then-green: this assertion read `Bridged` when the probe was
    // written.
    let refusal =
        tangent_locus_relation(&up, true, &gapped, false, &xline(), -1.0, 1.0, true, band())
            .expect_err("an in-band residual is not something a declaration may bridge");
    assert!(
        matches!(refusal, ContactRefusal::Escalated { .. }),
        "in-band residual must escalate, not bridge: {refusal:?}"
    );
    // The declaration changes nothing here — that is the point.
    let undeclared = tangent_locus_relation(
        &up,
        true,
        &gapped,
        false,
        &xline(),
        -1.0,
        1.0,
        false,
        band(),
    )
    .expect_err("undeclared, the same in-band residual escalates");
    assert!(matches!(undeclared, ContactRefusal::Escalated { .. }));
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

/// CLAIM 7 / AQ6 — the definite verdict beats the declaration in BOTH
/// directions: exactly-equal declared carriers verify DEFINITE (the
/// declaration added nothing — it is not credited with a bridge), and
/// a definite difference contradicts.
#[test]
fn probe_aq6_definite_beats_declaration_both_directions() {
    let a = sphere([0.0, 0.0, 0.0], 2.0, true);
    let b = sphere([0.0, 0.0, 0.0], 2.0, false);
    let (rel, verdict) =
        topo::boolean::carrier_eq::carrier_eq_verdict(&a, &b, declared(), &at(1.0), band())
            .unwrap();
    assert_eq!(rel, CarrierRelation::SameOpposite);
    assert_eq!(
        verdict,
        ContactVerdict::Definite,
        "bit-equal data is the geometry's own evidence, not a bridge"
    );
    let off = sphere([0.0, 0.0, 0.0], 2.5, false);
    assert!(matches!(
        carrier_eq(&a, &off, declared(), &at(1.0), band()),
        Err(CarrierEqError::Contradicted { .. })
    ));
}
