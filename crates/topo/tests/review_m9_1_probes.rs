//! Blinded-review probes for PR #524 (M9-1 PR-1, head a557da3b).
//!
//! Each probe EXECUTES one claim from the review brief; assertions pin
//! the observed behaviour of the head, including behaviour the review
//! reports as findings (marked FINDING below).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{flush_declarations, prism_z};
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};
use topo::boolean::contact_verify::tangent_locus_relation;
use topo::boolean::plane_eq::PlaneIdentity;
use topo::{
    Body, CarrierDesc, CarrierEqError, CarrierRelation, ContactClass, ContactRecords,
    ContactRefusal, ContactVerdict, CurveContact, FacePairDeclaration, PatchContact,
    ValidationError, carrier_eq, validate_pseudomanifold,
};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    prism_z::<f64>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

fn declared() -> PlaneIdentity<'static> {
    PlaneIdentity {
        s1: None,
        s2: None,
        declared: true,
    }
}

fn sphere(c: [f64; 3], r: f64, outward: bool) -> CarrierDesc<f64> {
    CarrierDesc::Sphere {
        center: Point3::new(c[0], c[1], c[2]),
        radius: r,
        outward,
    }
}

fn cyl(o: [f64; 3], a: [f64; 3], r: f64, outward: bool) -> CarrierDesc<f64> {
    CarrierDesc::Cylinder {
        origin: Point3::new(o[0], o[1], o[2]),
        axis: Vec3::new(a[0], a[1], a[2]),
        radius: r,
        outward,
    }
}

fn plane_s(o: [f64; 3], n: [f64; 3]) -> geom::Surface<f64> {
    geom::Surface::Plane {
        origin: Point3::new(o[0], o[1], o[2]),
        normal: Vec3::new(n[0], n[1], n[2]),
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
            carrier_eq(&a, &off, PlaneIdentity::NONE, arm, band()).unwrap(),
            CarrierRelation::Distinct,
            "definite center offset must be Distinct at arm {arm}"
        );
        assert!(
            matches!(
                carrier_eq(&a, &off, declared(), arm, band()),
                Err(CarrierEqError::Contradicted(_))
            ),
            "declared, arm {arm}: contradicted"
        );
    }
    // In-band center offset: also arm-independent.
    let near = sphere([1e-13, 0.0, 0.0], 2.0, false);
    for arm in [1e-3, 1e3] {
        assert_eq!(
            carrier_eq(&a, &near, declared(), arm, band()).unwrap(),
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
        carrier_eq(&a, &b, declared(), 1.0, band()).unwrap(),
        CarrierRelation::SameOpposite
    );
    // Large arm: the same tilt is a definite misalignment there.
    match carrier_eq(&a, &b, declared(), 1e6, band()).unwrap_err() {
        CarrierEqError::Contradicted(d) => {
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
        match carrier_eq(a, b, declared(), 1.0, band()).unwrap_err() {
            CarrierEqError::Contradicted(d) => {
                assert_eq!(d.predicate, Some("carrier_cyl_radius"));
            }
            other => panic!("expected radius contradiction, got {other:?}"),
        }
    }
    // Near-tie walk order: axis-offset in band, radius definite — the
    // ladder must walk PAST the in-band margin and name the radius.
    let near = cyl([1e-13, 0.0, 0.0], [0.0, 0.0, 1.0], 3.001, false);
    match carrier_eq(&m1, &near, declared(), 1.0, band()).unwrap_err() {
        CarrierEqError::Contradicted(d) => {
            assert_eq!(d.predicate, Some("carrier_cyl_radius"));
        }
        other => panic!("expected radius contradiction, got {other:?}"),
    }
}

/// CLAIM 2 — a DEFINITE separation must beat the declaration: two
/// parallel planes a definite gap apart, declared tangent along a
/// line lying on the first.
#[test]
fn probe_tangent_definite_separation_contradicts() {
    let up = plane_s([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let down = plane_s([0.0, 0.0, 1e-3], [0.0, 0.0, 1.0]);
    let err = tangent_locus_relation(&up, true, &down, false, &xline(), -1.0, 1.0, true, band())
        .unwrap_err();
    match err {
        topo::ContactRefusal::Contradicted { diag, steer } => {
            assert_eq!(diag.predicate, Some("contact_tangent_on_2"));
            assert_eq!(steer, Some(topo::FIT_DEFERRAL));
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

/// CLAIM 4 — the at-rest gate fires `ValidationError::ContactContradicted`:
/// a CurveContact record whose faces are definitely separated must be
/// contradicted BY THE CENSUS, not just at use. (No test in the PR
/// executes this arm; this probe does.)
#[test]
fn probe_at_rest_gate_contradicts_a_false_curve_record() {
    let body = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
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
        .find(|_| true)
        .expect("brick has edges");
    let contacts = ContactRecords {
        curves: vec![CurveContact {
            face_a: top,
            face_b: bottom,
            witness,
        }],
        ..ContactRecords::default()
    };
    let errs = validate_pseudomanifold(&body, &contacts, Tol::witness())
        .expect_err("false record must fail");
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::ContactContradicted { .. })),
        "the at-rest gate must fire ContactContradicted specifically: {errs:?}"
    );
}

/// CLAIM 6 — nothing certifies a PatchContact: a body carrying a patch
/// record is refused typed at rest (CensusUnsupported), never blessed.
#[test]
fn probe_patch_contact_is_never_certified_at_rest() {
    let body = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let mut it = body.faces().map(|(k, _)| k);
    let (fa, fb) = (it.next().unwrap(), it.next().unwrap());
    let contacts = ContactRecords {
        patches: vec![PatchContact {
            face_a: fa,
            face_b: fb,
        }],
        ..ContactRecords::default()
    };
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
    let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick((0.5, 1.5), (0.25, 1.25), (1.0, 2.0));
    let decls = flush_declarations(&a, &b);
    let topo::BooleanResult::Body(out) = topo::union_with(&a, &b, &decls, Tol::witness()).unwrap()
    else {
        panic!("union is a body");
    };
    let mut mutated = out.contacts.clone();
    let fa = a.faces().next().unwrap().0;
    mutated.patches.push(PatchContact {
        face_a: fa,
        face_b: fa,
    });
    assert_ne!(out.contacts, mutated, "PartialEq must see the new fields");
    let mut curve_mutated = out.contacts.clone();
    if let Some((e, _)) = a.edges().next() {
        curve_mutated.curves.push(CurveContact {
            face_a: fa,
            face_b: fa,
            witness: e,
        });
        assert_ne!(out.contacts, curve_mutated);
    }
}

/// DEVIATION 8 — the C4 "silent no-op at the op" gap, CLOSED (M9-1 fix
/// pass, F2; issue #539).
///
/// The fixture is the `m4_pr3_names_interval` corpus's shape (block top
/// z = 1, slot top z = 1.5) declared as `Rest`. When this probe was
/// written the subtract SUCCEEDED — the lie never met a verifier,
/// because the only verify-at-use site was the REST lane, which is
/// Union-only — while the carrier ladder, asked directly, contradicted
/// it at `bool_plane_offset`. Red-then-green: the assertion below read
/// `out.is_ok()` then; the op-door pass now refuses the same
/// declaration before any classification runs, and the corpus that
/// carried this shape no longer declares it.
#[test]
fn probe_dev8_false_declaration_is_a_silent_noop_at_the_op() {
    let c = brick((0.0, 3.0), (0.0, 3.0), (0.0, 1.0));
    let slot = brick((1.0, 2.0), (-1.0, 4.0), (0.5, 1.5));
    let z_face = |body: &Body<f64>, z: f64| {
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
    let c_top = z_face(&c, 1.0);
    let slot_top = z_face(&slot, 1.5);
    let mut decls = topo::BooleanDeclarations::default();
    decls
        .coincident_faces
        .push(FacePairDeclaration::rest(c_top, slot_top));
    // The lie is definite: the ladder, asked directly, contradicts.
    let verdict =
        topo::boolean::carrier_pair_relation(&c, c_top, &slot, slot_top, true, band()).unwrap();
    assert!(
        matches!(verdict, Err(CarrierEqError::Contradicted(_))),
        "the declared caps are definitely offset: {verdict:?}"
    );
    // And the op refuses it too, at the door, naming the same margin —
    // the gap is closed, and the two sites agree because they share a
    // ladder rather than mirroring one.
    let err = topo::subtract_with(&c, &slot, &decls, Tol::witness())
        .expect_err("a false declaration is no longer a silent no-op at the op");
    match &err {
        topo::BooleanError::ContactContradicted {
            declaration,
            margin,
            ..
        } => {
            assert_eq!(declaration.class, ContactClass::Rest);
            assert_eq!(margin.predicate, Some("bool_plane_offset"));
        }
        other => panic!("expected ContactContradicted, got {other:?}"),
    }
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
        topo::boolean::carrier_eq::carrier_eq_verdict(&a, &b, declared(), 1.0, band()).unwrap();
    assert_eq!(rel, CarrierRelation::SameOpposite);
    assert_eq!(
        verdict,
        ContactVerdict::Definite,
        "bit-equal data is the geometry's own evidence, not a bridge"
    );
    let off = sphere([0.0, 0.0, 0.0], 2.5, false);
    assert!(matches!(
        carrier_eq(&a, &off, declared(), 1.0, band()),
        Err(CarrierEqError::Contradicted(_))
    ));
}

/// CLAIM 3 — DeclaredPairs keyed-map semantics: the old `contains`
/// behaviour (symmetric across operands, same-operand always false)
/// is preserved through the op door: a same-operand "declaration"
/// cannot be expressed at all (the field is cross-operand by type),
/// and the A/B orientation of a declared pair still verifies.
#[test]
fn probe_declared_pair_direction_still_normalized() {
    let a = brick((0.0, 2.0), (0.0, 2.0), (0.0, 1.0));
    let b = brick((0.0, 2.0), (0.0, 2.0), (1.0, 2.0));
    let decls = flush_declarations(&a, &b);
    assert!(!decls.coincident_faces.is_empty());
    assert!(
        topo::union_with(&a, &b, &decls, Tol::witness()).is_ok(),
        "declared flush pair verifies through the map exactly as through the set"
    );
    // Class mint honesty: every declaration built by the old helpers
    // is Rest, spelled out — no silent default hides in the map.
    assert!(
        decls
            .coincident_faces
            .iter()
            .all(|d| d.class == ContactClass::Rest)
    );
}
