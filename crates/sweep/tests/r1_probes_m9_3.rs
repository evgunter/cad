//! M9-3 unit review R1 — adversarial probes (blinded lane).
//!
//! Attacks: the C8 boundary under novel undeclared/mis-declared
//! configurations, the two-peg oracle re-derived and perturbed, the
//! zip's ring bookkeeping under nested/mismatched rings, partial
//! engagement, and the tube-chain additivity bound measured.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{
    Body, BooleanDeclarations, BooleanError, BooleanResult, ContactClass, FacePairDeclaration,
    mass_properties,
};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn plate6(z0: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(6.0, 0.0), p2(6.0, 4.0), p2(0.0, 4.0)]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body
}

fn cyl_at(cx: f64, z0: f64, h: f64, r: f64) -> Body<f64> {
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th = deg.to_radians();
        p2(cx + r * th.cos(), 2.0 + r * th.sin())
    };
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(at(0.0), b120),
        ProfileVertex::new(at(120.0), b120),
        ProfileVertex::new(at(240.0), b120),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .unwrap()
        .body
}

fn body_of(r: BooleanResult<f64>) -> Body<f64> {
    match r {
        BooleanResult::Body(b) => b.body,
        BooleanResult::Empty => panic!("operand cannot be empty"),
    }
}

fn plate_with_pegs(cx1: f64, cx2: f64, r: f64) -> Body<f64> {
    let p0 = plate6(0.0);
    let p1 = body_of(topo::union(&p0, &cyl_at(cx1, 0.4, 1.6, r), Tol::witness()).unwrap());
    body_of(topo::union(&p1, &cyl_at(cx2, 0.4, 1.6, r), Tol::witness()).unwrap())
}

fn plate_with_bores() -> Body<f64> {
    let q0 = plate6(1.0);
    let q1 = body_of(topo::subtract(&q0, &cyl_at(2.0, 0.8, 1.4, 0.5), Tol::witness()).unwrap());
    body_of(topo::subtract(&q1, &cyl_at(4.0, 0.8, 1.4, 0.5), Tol::witness()).unwrap())
}

fn walls_at(body: &Body<f64>, cx: f64) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { origin, .. }) if (origin.x - cx).abs() < 0.5
            )
        })
        .map(|(k, _)| k)
        .collect()
}

fn plane_face(body: &Body<f64>, z: f64, up: bool) -> topo::FaceKey {
    let hits: Vec<_> = body
        .faces()
        .filter(|(_, f)| match body.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, normal, .. }) => {
                (origin.z - z).abs() < 1e-9 && (normal.z > 0.5) == up
            }
            _ => false,
        })
        .map(|(k, _)| k)
        .collect();
    let [f] = hits[..] else {
        panic!(
            "expected exactly one z = {z} face (up = {up}), got {}",
            hits.len()
        );
    };
    f
}

/// Full two-peg declarations, optionally dropping peg 2's wall group
/// or redirecting peg 1's walls to bore 2 (the wrong-pair probe).
fn declarations(
    p: &Body<f64>,
    q: &Body<f64>,
    skip_second: bool,
    cross_wire: bool,
) -> BooleanDeclarations {
    let mut decls = BooleanDeclarations::none();
    decls.coincident_faces.push(FacePairDeclaration::new(
        plane_face(p, 1.0, true),
        plane_face(q, 1.0, false),
        ContactClass::Rest,
    ));
    let groups: Vec<(f64, f64)> = if cross_wire {
        vec![(2.0, 4.0), (4.0, 2.0)] // peg-1 walls declared against bore-2
    } else if skip_second {
        vec![(2.0, 2.0)]
    } else {
        vec![(2.0, 2.0), (4.0, 4.0)]
    };
    for (pcx, qcx) in groups {
        for &fa in &walls_at(p, pcx) {
            for &fb in &walls_at(q, qcx) {
                decls
                    .coincident_faces
                    .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
            }
        }
    }
    decls
}

/// Oracle re-derivation: the plate pair is 6·4·1 each = 24+24 = 48 and
/// the peg/bore π terms cancel IN REAL ARITHMETIC. The PR claims the
/// closed form is exactly 48 — check both the closed form and the
/// bitwise chain vp + vq == v == 48.
#[test]
fn probe_two_peg_oracle_rederivation() {
    let p = plate_with_pegs(2.0, 4.0, 0.5);
    let q = plate_with_bores();
    let vp = mass_properties(&p, Tol::witness()).unwrap().volume;
    let vq = mass_properties(&q, Tol::witness()).unwrap().volume;
    let decls = declarations(&p, &q, false, false);
    let body = body_of(topo::union_with(&p, &q, &decls, Tol::witness()).unwrap());
    let v = mass_properties(&body, Tol::witness()).unwrap().volume;
    eprintln!(
        "vp = {vp:.17e}, vq = {vq:.17e}, vp+vq = {:.17e}, v = {v:.17e}",
        vp + vq
    );
    assert_eq!(v, vp + vq, "bitwise additivity");
    assert_eq!(v, 48.0, "the closed form is exactly 48 (PR claim)");
}

/// Perturbation: peg 2 offset so the carriers differ by a DEFINITE
/// margin (1e-9 ≫ the witness band) — the declaration must be
/// contradicted, never silently zipped.
#[test]
fn probe_peg_offset_definite_contradicts() {
    let p = plate_with_pegs(2.0, 4.0 + 1e-9, 0.5);
    let q = plate_with_bores();
    let decls = declarations(&p, &q, false, false);
    let err = topo::union_with(&p, &q, &decls, Tol::witness())
        .expect_err("a definitely-offset peg declared Rest must not union");
    assert!(
        matches!(
            err,
            BooleanError::ContactContradicted { .. } | BooleanError::Escalated { .. }
        ),
        "typed, at the door: {err:?}"
    );
}

/// Perturbation: peg 2 offset by ~2 ulp of 4.0 (in the zero band).
/// C4 bridges in-band residues under a verified declaration — the
/// probe records which arm actually fires and that nothing SILENTLY
/// wrong emerges (either a typed refusal, or a union whose volume is
/// still additive to dust).
#[test]
fn probe_peg_offset_one_ulp_characterized() {
    let ulp = 4.0_f64 * f64::EPSILON; // ~8.9e-16
    let p = plate_with_pegs(2.0, 4.0 + ulp, 0.5);
    let q = plate_with_bores();
    let vp = mass_properties(&p, Tol::witness()).unwrap().volume;
    let vq = mass_properties(&q, Tol::witness()).unwrap().volume;
    let decls = declarations(&p, &q, false, false);
    match topo::union_with(&p, &q, &decls, Tol::witness()) {
        Ok(out) => {
            let body = body_of(out);
            let v = mass_properties(&body, Tol::witness()).unwrap().volume;
            eprintln!(
                "1-ulp offset unioned: v = {v:.17e}, vp+vq = {:.17e}",
                vp + vq
            );
            assert!(
                (v - (vp + vq)).abs() < 1e-12,
                "bridged union must stay additive to dust: {v} vs {}",
                vp + vq
            );
            if let Err(errs) = topo::validate_geometric(&body, Tol::witness()) {
                panic!("bridged union must stay tier-3 valid: {errs:?}");
            }
        }
        Err(err) => {
            eprintln!("1-ulp offset refused: {err:?}");
            assert!(
                matches!(
                    err,
                    BooleanError::Escalated { .. }
                        | BooleanError::ContactContradicted { .. }
                        | BooleanError::RestZipUnsupported { .. }
                        | BooleanError::JoinDesync { .. }
                        | BooleanError::CurvedPierceUnsupported { .. }
                ),
                "typed only: {err:?}"
            );
        }
    }
}

/// Missing declaration group: peg 2's walls undeclared — the second
/// incidence must keep the typed frontier refusal (C8), never ride
/// peg 1's declarations.
#[test]
fn probe_missing_declared_group_refuses_typed() {
    let p = plate_with_pegs(2.0, 4.0, 0.5);
    let q = plate_with_bores();
    let decls = declarations(&p, &q, true, false);
    let err = topo::union_with(&p, &q, &decls, Tol::witness())
        .expect_err("an undeclared second peg must refuse");
    assert!(
        matches!(err, BooleanError::CurvedPierceUnsupported { .. }),
        "the undeclared incidence keeps the frontier door: {err:?}"
    );
}

/// Wrong-pair declaration: peg 1's walls declared against BORE 2's
/// walls (distinct carriers two units apart). The declaration must be
/// contradicted at the door — it must not act as cover for either
/// real incidence.
#[test]
fn probe_cross_wired_declaration_contradicts() {
    let p = plate_with_pegs(2.0, 4.0, 0.5);
    let q = plate_with_bores();
    let decls = declarations(&p, &q, false, true);
    let err = topo::union_with(&p, &q, &decls, Tol::witness())
        .expect_err("cross-wired wall declarations must not union");
    assert!(
        matches!(err, BooleanError::ContactContradicted { .. }),
        "typed contradiction at the door: {err:?}"
    );
}

/// Wrong class on a conformal pair: the two-peg walls declared
/// Tangent must contradict (the conformal screen), not act as cover.
#[test]
fn probe_tangent_on_conformal_walls_contradicts() {
    let p = plate_with_pegs(2.0, 4.0, 0.5);
    let q = plate_with_bores();
    let mut decls = BooleanDeclarations::none();
    for cx in [2.0, 4.0] {
        for &fa in &walls_at(&p, cx) {
            for &fb in &walls_at(&q, cx) {
                decls.coincident_faces.push(FacePairDeclaration::new(
                    fa,
                    fb,
                    ContactClass::Tangent,
                ));
            }
        }
    }
    let err = topo::union_with(&p, &q, &decls, Tol::witness())
        .expect_err("a conformal pair declared Tangent must contradict");
    assert!(
        matches!(err, BooleanError::ContactContradicted { .. }),
        "{err:?}"
    );
}

/// Partial engagement: a peg reaching only halfway up its through-bore
/// (the bore is NOT fully engaged). Declared Rest on the walls + the
/// mate: whatever happens must be a typed refusal or an exactly-
/// additive union — never a silently wrong body.
#[test]
fn probe_partial_engagement_never_silent() {
    let p = {
        let p0 = plate6(0.0);
        body_of(topo::union(&p0, &cyl_at(2.0, 0.4, 1.1, 0.5), Tol::witness()).unwrap())
    };
    let q = {
        let q0 = plate6(1.0);
        body_of(topo::subtract(&q0, &cyl_at(2.0, 0.8, 1.4, 0.5), Tol::witness()).unwrap())
    };
    let vp = mass_properties(&p, Tol::witness()).unwrap().volume;
    let vq = mass_properties(&q, Tol::witness()).unwrap().volume;
    let mut decls = BooleanDeclarations::none();
    decls.coincident_faces.push(FacePairDeclaration::new(
        plane_face(&p, 1.0, true),
        plane_face(&q, 1.0, false),
        ContactClass::Rest,
    ));
    for &fa in &walls_at(&p, 2.0) {
        for &fb in &walls_at(&q, 2.0) {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    match topo::union_with(&p, &q, &decls, Tol::witness()) {
        Ok(out) => {
            let body = body_of(out);
            let v = mass_properties(&body, Tol::witness()).unwrap().volume;
            eprintln!(
                "partial engagement unioned: v = {v:.17e} vs {:.17e}",
                vp + vq
            );
            assert_eq!(v, vp + vq, "if it unions it must be exactly additive");
            if let Err(errs) = topo::validate_geometric(&body, Tol::witness()) {
                panic!("must be tier-3 valid: {errs:?}");
            }
        }
        Err(err) => {
            eprintln!("partial engagement refused: {err:?}");
            assert!(
                matches!(
                    err,
                    BooleanError::RestZipUnsupported { .. }
                        | BooleanError::JoinDesync { .. }
                        | BooleanError::Join(_)
                        | BooleanError::CurvedPierceUnsupported { .. }
                        | BooleanError::CurvedBooleanUnsupported { .. }
                ),
                "typed only: {err:?}"
            );
        }
    }
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(x.0, y.0), p2(x.1, y.0), p2(x.1, y.1), p2(x.0, y.1)]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z.0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(z.1 - z.0), Tol::witness())
        .unwrap()
        .body
}

/// A plate with `holes` square through-holes (two-ring patches when
/// stacked): the ring-capable glue must handle TWO rings per patch
/// face, and the volume must be exactly additive.
fn holed_plate(z0: f64, z1: f64, holes: &[(f64, f64)]) -> Body<f64> {
    let mut b = brick((0.0, 6.0), (0.0, 3.0), (z0, z1));
    for &(hx, hy) in holes {
        b = body_of(
            topo::subtract(
                &b,
                &brick((hx, hx + 1.0), (hy, hy + 1.0), (z0 - 0.5, z1 + 0.5)),
                Tol::witness(),
            )
            .unwrap(),
        );
    }
    b
}

fn flush_rest_decls(bot: &Body<f64>, top: &Body<f64>, z: f64) -> BooleanDeclarations {
    let mut decls = BooleanDeclarations::none();
    decls.coincident_faces.push(FacePairDeclaration::new(
        plane_face(bot, z, true),
        plane_face(top, z, false),
        ContactClass::Rest,
    ));
    decls
}

/// Two stacked plates each with TWO square through-holes: the contact
/// patch face carries two rings on each side. Exact additivity or bust.
#[test]
fn probe_two_ring_patch_unions_exactly() {
    let holes = [(1.0, 1.0), (4.0, 1.0)];
    let bot = holed_plate(0.0, 1.0, &holes);
    let top = holed_plate(1.0, 2.0, &holes);
    let vb = mass_properties(&bot, Tol::witness()).unwrap().volume;
    let vt = mass_properties(&top, Tol::witness()).unwrap().volume;
    let out = topo::union_with(
        &bot,
        &top,
        &flush_rest_decls(&bot, &top, 1.0),
        Tol::witness(),
    )
    .expect("the two-ring patch union runs");
    let body = body_of(out);
    let v = mass_properties(&body, Tol::witness()).unwrap().volume;
    assert_eq!(v, vb + vt, "exactly additive with two rings per patch");
    if let Err(errs) = topo::validate_geometric(&body, Tol::witness()) {
        panic!("tier-3 valid: {errs:?}");
    }
}

/// Ring-count mismatch: the bottom plate has two holes, the top ONE.
/// The pair's patch faces are not congruent — must refuse typed or
/// decline to the original join refusal, never a silent wrong body.
#[test]
fn probe_ring_count_mismatch_never_silent() {
    let bot = holed_plate(0.0, 1.0, &[(1.0, 1.0), (4.0, 1.0)]);
    let top = holed_plate(1.0, 2.0, &[(1.0, 1.0)]);
    let vb = mass_properties(&bot, Tol::witness()).unwrap().volume;
    let vt = mass_properties(&top, Tol::witness()).unwrap().volume;
    match topo::union_with(
        &bot,
        &top,
        &flush_rest_decls(&bot, &top, 1.0),
        Tol::witness(),
    ) {
        Ok(out) => {
            // A legitimate union of this mate exists (the top caps one
            // hole); if the kernel produces it, it must be exact.
            let body = body_of(out);
            let v = mass_properties(&body, Tol::witness()).unwrap().volume;
            eprintln!("ring mismatch unioned: v = {v:.17e}");
            assert_eq!(v, vb + vt, "if it unions it must be exactly additive");
            if let Err(errs) = topo::validate_geometric(&body, Tol::witness()) {
                panic!("tier-3 valid: {errs:?}");
            }
        }
        Err(err) => {
            eprintln!("ring mismatch refused: {err:?}");
            assert!(
                matches!(
                    err,
                    BooleanError::RestZipUnsupported { .. }
                        | BooleanError::Join(_)
                        | BooleanError::JoinDesync { .. }
                        | BooleanError::ZipCorrespondence { .. }
                ),
                "typed only: {err:?}"
            );
        }
    }
}

// ---- tube-chain rim (acceptance ii) attacks ----

fn lying_plane() -> SketchPlane<f64> {
    SketchPlane::new(Affine3::from_parts(
        geom_core::Mat3::from_cols(Vec3::unit_z(), Vec3::unit_x(), Vec3::unit_y()),
        Vec3::new(0.0, 0.0, 0.0),
    ))
}

fn lying_extrude(vertices: Vec<ProfileVertex<f64>>, tangent_joints: Vec<usize>) -> Body<f64> {
    let profile = Profile::new(
        lying_plane(),
        vec![ProfileLoop::new(vertices).with_tangent_joints(tangent_joints)],
    )
    .validate(Tol::witness())
    .unwrap();
    extrude(&profile, Extrusion::Distance(4.0), Tol::witness())
        .unwrap()
        .body
}

fn quarter_round_below() -> Body<f64> {
    let b90 = (core::f64::consts::PI / 8.0).tan();
    lying_extrude(
        vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(1.0, 0.0), 0.0),
            ProfileVertex::new(p2(1.0, 2.0), b90),
            ProfileVertex::new(p2(0.0, 3.0), 0.0),
        ],
        vec![2],
    )
}

fn quarter_round_above() -> Body<f64> {
    let b90 = (core::f64::consts::PI / 8.0).tan();
    lying_extrude(
        vec![
            ProfileVertex::new(p2(1.0, 0.5), 0.0),
            ProfileVertex::new(p2(1.0, 2.0), -b90),
            ProfileVertex::new(p2(2.0, 3.0), 0.0),
            ProfileVertex::new(p2(3.0, 3.0), 0.0),
            ProfileVertex::new(p2(3.0, 0.5), 0.0),
        ],
        vec![1, 2],
    )
}

fn one_cyl_face(body: &Body<f64>) -> topo::FaceKey {
    let hits: Vec<_> = body
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .map(|(k, _)| k)
        .collect();
    let [f] = hits[..] else {
        panic!("expected exactly one wall face");
    };
    f
}

/// Measure the actual additivity error on the rim fixture — is the
/// 1e-12 bound honest headroom or a fitted constant?
#[test]
fn probe_tube_chain_additivity_error_measured() {
    let a = quarter_round_below();
    let b = quarter_round_above();
    let va = mass_properties(&a, Tol::witness()).unwrap().volume;
    let vb = mass_properties(&b, Tol::witness()).unwrap().volume;
    let mut decls = BooleanDeclarations::none();
    decls.coincident_faces.push(FacePairDeclaration::new(
        plane_face(&a, 1.0, true),
        plane_face(&b, 1.0, false),
        ContactClass::Rest,
    ));
    decls.coincident_faces.push(FacePairDeclaration::new(
        one_cyl_face(&a),
        one_cyl_face(&b),
        ContactClass::Tangent,
    ));
    decls.coincident_faces.push(FacePairDeclaration::new(
        plane_face(&a, 1.0, true),
        one_cyl_face(&b),
        ContactClass::Tangent,
    ));
    decls.coincident_faces.push(FacePairDeclaration::new(
        one_cyl_face(&a),
        plane_face(&b, 1.0, false),
        ContactClass::Tangent,
    ));
    let body = body_of(topo::union_with(&a, &b, &decls, Tol::witness()).unwrap());
    let v = mass_properties(&body, Tol::witness()).unwrap().volume;
    eprintln!(
        "tube-chain: va = {va:.17e}, vb = {vb:.17e}, v = {v:.17e}, err = {:.3e}",
        (v - (va + vb)).abs()
    );
    assert!((v - (va + vb)).abs() < 1e-12);
}

/// Declared on one side only: drop the wall×wall Tangent declaration
/// (keep the plane Rest + the two plane×wall Tangents). The wall pair
/// incidence is then UNDECLARED and must keep a typed refusal.
#[test]
fn probe_rim_wall_pair_undeclared_refuses_typed() {
    let a = quarter_round_below();
    let b = quarter_round_above();
    let mut decls = BooleanDeclarations::none();
    decls.coincident_faces.push(FacePairDeclaration::new(
        plane_face(&a, 1.0, true),
        plane_face(&b, 1.0, false),
        ContactClass::Rest,
    ));
    decls.coincident_faces.push(FacePairDeclaration::new(
        plane_face(&a, 1.0, true),
        one_cyl_face(&b),
        ContactClass::Tangent,
    ));
    decls.coincident_faces.push(FacePairDeclaration::new(
        one_cyl_face(&a),
        plane_face(&b, 1.0, false),
        ContactClass::Tangent,
    ));
    match topo::union_with(&a, &b, &decls, Tol::witness()) {
        Ok(out) => {
            // If it unions anyway the result must still be exact and
            // valid — but record it: the wall-pair incidence rode
            // other declarations.
            let body = body_of(out);
            let v = mass_properties(&body, Tol::witness()).unwrap().volume;
            let va = mass_properties(&a, Tol::witness()).unwrap().volume;
            let vb = mass_properties(&b, Tol::witness()).unwrap().volume;
            eprintln!(
                "UNDECLARED WALL PAIR UNIONED: v = {v:.17e} vs {:.17e}",
                va + vb
            );
            panic!("the undeclared wall-pair incidence must keep a typed refusal (C8)");
        }
        Err(err) => {
            eprintln!("undeclared wall pair refused: {err:?}");
        }
    }
}
