//! M9-3 PR-A — the wall and the door (CONTACT-DESIGN C8 at the
//! boolean): the front door admits declarations by carrier inventory
//! and the DEV-1 Tangent witness lane; a VERIFIED declared `Rest`
//! pair opens the declared-cosurface reduction rung and the carrier
//! lump at both wall sites; declared `Tangent` pairs descend to the
//! second-order sector trilean. Undeclared touching refuses forever,
//! typed — the door only widens what a verified declaration unlocks.
//!
//! The canonical reachability fixture is the two-peg kernel shape's
//! core: a bored plate (through-hole subtract) and an exactly-filling
//! peg, both walls authored as three-arc cylinders on one carrier
//! (`demos/README.md` two-peg; CONTACT-DESIGN C7).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::k_stats::{start_verdict_log, take_verdict_log};
use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{
    Body, BooleanDeclarations, BooleanError, BooleanResult, ContactClass, FacePairDeclaration,
};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The plate: 4×4×1, z ∈ [0, 1].
fn plate() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(4.0, 0.0), p2(4.0, 4.0), p2(0.0, 4.0)]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body
}

/// A radius-`r` three-arc cylinder at (2, 2), z ∈ [z0, z0 + h] (the
/// boss_union authorship: three 120° arcs on ONE cylinder surface).
fn cyl(z0: f64, h: f64, r: f64) -> Body<f64> {
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th = deg.to_radians();
        p2(2.0 + r * th.cos(), 2.0 + r * th.sin())
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

/// The bored plate: a through-hole subtract (the shipped transverse
/// lane) leaving three bore-wall faces on one cylinder carrier.
fn bored_plate() -> Body<f64> {
    match topo::subtract(&plate(), &cyl(-0.2, 1.4, 0.5), Tol::witness())
        .expect("the through-hole subtract is the shipped transverse lane")
    {
        BooleanResult::Body(b) => b.body,
        BooleanResult::Empty => panic!("a bored plate cannot be empty"),
    }
}

/// The cylinder-surface faces of a body.
fn cyl_faces(body: &Body<f64>) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// Every bore-wall × peg-wall pair declared under `class`.
fn wall_declarations(a: &Body<f64>, b: &Body<f64>, class: ContactClass) -> BooleanDeclarations {
    let mut decls = BooleanDeclarations::none();
    for &fa in &cyl_faces(a) {
        for &fb in &cyl_faces(b) {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(fa, fb, class));
        }
    }
    assert!(
        !decls.coincident_faces.is_empty(),
        "the fixture must have wall faces on both operands"
    );
    decls
}

/// C8, the invariant half: the same touching geometry WITHOUT a
/// declaration keeps its typed refusal — value equality never glues,
/// and the reduction rung's frontier doors are verbatim on undeclared
/// incidences.
#[test]
fn undeclared_touching_curved_pair_still_refuses_typed() {
    let bored = bored_plate();
    let peg = cyl(0.0, 1.0, 0.5);
    let err = topo::union(&bored, &peg, Tol::witness())
        .expect_err("an undeclared exactly-filling peg must refuse");
    // The SAME typed refusal, at the SAME site, as before this unit
    // opened the declared rung: the sweep's curved frontier door on
    // the on-carrier rim circle (the spike's run-1 measurement of the
    // undeclared posture — bool_circle_curved_clearance decides Zero,
    // the frontier door fires).
    assert!(
        matches!(err, BooleanError::CurvedPierceUnsupported { .. }),
        "the undeclared refusal stays at the classification frontier, typed: {err:?}"
    );
}

/// The opened wall: with the nine bore×peg wall pairs declared `Rest`,
/// classification RUNS — the front door admits the cylinder carriers,
/// the reduction rung produces the v-v record family, and the v-v
/// lane's carrier lump consumes the declared pairs (the recl wall,
/// previously an untyped `ClassificationInvariant`, is gone). The op
/// then either succeeds with the closed-form volume or refuses typed
/// strictly DOWNSTREAM of classification.
#[test]
fn declared_rest_two_peg_reaches_downstream_of_classification() {
    let bored = bored_plate();
    let peg = cyl(0.0, 1.0, 0.5);
    let decls = wall_declarations(&bored, &peg, ContactClass::Rest);
    start_verdict_log();
    let out = topo::union_with(&bored, &peg, &decls, Tol::witness());
    let v = take_verdict_log();
    // The carrier ladder's cylinder rungs ran — the declared descent
    // executed rather than being skipped past (telemetry from birth).
    for name in ["carrier_cyl_axis_parallel", "carrier_cyl_radius"] {
        assert!(
            v.iter().any(|x| x.predicate == name),
            "{name} never reached the funnel — the declared descent did not run"
        );
    }
    match out {
        Ok(BooleanResult::Body(b)) => {
            // Exactly additive against the closed-form oracle: the peg
            // fills the bore exactly, so the union is the unbored
            // plate's volume, exactly (the C7-lane statement).
            let vol = topo::mass_properties(&b.body, Tol::witness())
                .unwrap()
                .volume;
            assert_eq!(vol, 16.0, "exactly-additive volume (closed form)");
            // The surviving rim arcs sit between two COPLANAR planar
            // faces (plate cap × peg cap): the surfaces under-
            // determine the locus, so the D6 pass re-describes the
            // stale intersection citations CONVENTIONALLY on the
            // unchanged circle carriers — the arrival the curved
            // smooth-seam `JoinDesync` door demands (the red half of
            // this row was the measured refusal before the
            // conventional-arc lane existed).
            let mut rims = 0;
            for (_, e) in b.body.edges() {
                let Some(c) = b.body.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
                    continue;
                };
                if matches!(c.carrier(), geom::Curve3::Circle { .. }) {
                    rims += 1;
                    assert!(
                        matches!(c.description(), geom_brep::EdgeDescription::Scaffold(_)),
                        "a coplanar-adjacent rim is conventionally described: {:?}",
                        c.description()
                    );
                }
            }
            assert_eq!(rims, 6, "two rim circles of three arcs each survive");
        }
        Ok(BooleanResult::Empty) => panic!("a filled plate cannot be empty"),
        // PR-B: the zip's band closure landed — the union SUCCEEDS
        // (the Ok arm above carries the closed-form volume oracle);
        // any refusal is a regression of the opened lane.
        Err(err) => panic!("the declared exactly-filling union must succeed: {err:?}"),
    }
}

/// C4 verify-at-use on the curved door: a declared `Rest` pair whose
/// radii definitely differ is CONTRADICTED at the door — definite
/// counter-evidence beats every declaration.
#[test]
fn declared_rest_with_wrong_radius_contradicts() {
    let bored = bored_plate();
    let thin_peg = cyl(0.0, 1.0, 0.4);
    let decls = wall_declarations(&bored, &thin_peg, ContactClass::Rest);
    let err = topo::union_with(&bored, &thin_peg, &decls, Tol::witness())
        .expect_err("a 0.4 peg declared Rest against a 0.5 bore must contradict");
    assert!(
        matches!(err, BooleanError::ContactContradicted { .. }),
        "{err:?}"
    );
}

/// A horizontal three-arc cylinder (axis +y at height `zc`, radius
/// 0.5) with a meridian SEAM on its lowest ruling (profile vertices
/// at 60°/180°/300° in sketch coordinates), spanning y ∈ [0.5, 3.5].
fn lying_cyl(zc: f64) -> Body<f64> {
    let b120 = (core::f64::consts::PI / 6.0).tan();
    // Sketch frame: sketch x → world z, sketch y → world x, normal
    // (extrusion) +y. Disc centre at world (x = 2, z = zc).
    let at = |deg: f64| {
        let th = deg.to_radians();
        p2(zc + 0.5 * th.cos(), 2.0 + 0.5 * th.sin())
    };
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(at(60.0), b120),
        ProfileVertex::new(at(180.0), b120),
        ProfileVertex::new(at(300.0), b120),
    ]);
    let plane = SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_z(), Vec3::unit_x(), Vec3::unit_y()),
        Point3::new(0.0, 0.5, 0.0) - Point3::origin(),
    ));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(3.0), Tol::witness())
        .unwrap()
        .body
}

/// The plate-top × cylinder-wall pairs declared under `class`.
fn top_wall_declarations(
    plate: &Body<f64>,
    cylinder: &Body<f64>,
    class: ContactClass,
) -> BooleanDeclarations {
    let top: Vec<_> = plate
        .faces()
        .filter(|(_, f)| match plate.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, normal, .. }) => {
                (origin.z - 1.0).abs() < 1e-12 && normal.z > 0.5
            }
            _ => false,
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(top.len(), 1, "one top face");
    let mut decls = BooleanDeclarations::none();
    for &fb in &cyl_faces(cylinder) {
        decls
            .coincident_faces
            .push(FacePairDeclaration::new(top[0], fb, class));
    }
    decls
}

/// The Tangent door, three-outcome honest on the witness lane's own
/// rows: definite counter-evidence CONTRADICTS (apart and crossing
/// both), an in-band gap ESCALATES, and the touching pair is ADMITTED
/// past the door (whatever the classification then answers, it is
/// never the door's refusal).
#[test]
fn tangent_door_contradicts_escalates_and_admits() {
    let a = plate();
    // Definitely apart (gap 0.5): contradicted.
    let apart = lying_cyl(2.0);
    let err = topo::union_with(
        &a,
        &apart,
        &top_wall_declarations(&a, &apart, ContactClass::Tangent),
        Tol::witness(),
    )
    .expect_err("a definite gap contradicts a Tangent declaration");
    assert!(
        matches!(err, BooleanError::ContactContradicted { .. }),
        "{err:?}"
    );
    // Definitely crossing (overlap 0.2): contradicted.
    let crossing = lying_cyl(1.3);
    let err = topo::union_with(
        &a,
        &crossing,
        &top_wall_declarations(&a, &crossing, ContactClass::Tangent),
        Tol::witness(),
    )
    .expect_err("a definite crossing contradicts a Tangent declaration");
    assert!(
        matches!(err, BooleanError::ContactContradicted { .. }),
        "{err:?}"
    );
    // An in-band gap (the geometric mean of the run's band, so the
    // row holds at every sampled ε): ESCALATES — the sliver is a
    // sliver whether or not intent is declared (C4's
    // must-verify-DEFINITE list is never bridged).
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let grazing = lying_cyl(1.5 + (band.zero() * band.escalate()).sqrt());
    let err = topo::union_with(
        &a,
        &grazing,
        &top_wall_declarations(&a, &grazing, ContactClass::Tangent),
        Tol::witness(),
    )
    .expect_err("an in-band tangency gap escalates");
    assert!(matches!(err, BooleanError::Escalated { .. }), "{err:?}");
    // The genuinely-touching pair is ADMITTED: whatever the outcome,
    // it is not a door refusal, and the second-order sector rows run.
    let resting = lying_cyl(1.5);
    start_verdict_log();
    let out = topo::union_with(
        &a,
        &resting,
        &top_wall_declarations(&a, &resting, ContactClass::Tangent),
        Tol::witness(),
    );
    let v = take_verdict_log();
    assert!(
        v.iter().any(|x| x.predicate == "tangent_locus_gap"),
        "the witness lane must have derived the ruling"
    );
    // Admitted past the door AND carried through: a `Tangent` pair's
    // carriers are distinct by its own verification, so the pair never
    // reaches the planar coplanar-merge door, and the join lane unions
    // the line-contact pair into ONE solid. The contact is a tangent
    // ruling — measure zero — so the volume is the operands' sum
    // BITWISE, which is the oracle this row pins.
    let Ok(BooleanResult::Body(b)) = out else {
        panic!("the admitted tangent pair must union: {out:?}");
    };
    assert_eq!(
        topo::validate_geometric(&b.body, Tol::witness()),
        Ok(()),
        "the tangent union is tier-3 valid"
    );
    assert_eq!(b.body.solids().count(), 1, "one solid, not a graft");
    let vol = topo::mass_properties(&b.body, Tol::witness())
        .unwrap()
        .volume;
    assert_eq!(
        vol,
        16.0 + 0.75 * core::f64::consts::PI,
        "plate + lying cylinder, exactly additive across the tangent ruling"
    );
}

/// Outside the DEV-1 witness lane the class refusal stands, naming
/// the class: two parallel DISTINCT planes declared Tangent have no
/// closed-form locus (plane×plane is not in the lane), and the
/// refusal is the typed class door, not a silent carry.
#[test]
fn tangent_outside_the_witness_lane_refuses_by_class() {
    let a = plate();
    // A second plate floating above (planar faces only, gap 1).
    let b = {
        let lp = ProfileLoop::polygon([p2(1.0, 1.0), p2(3.0, 1.0), p2(3.0, 3.0), p2(1.0, 3.0)]);
        let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 2.0)));
        let profile = Profile::new(plane, vec![lp])
            .validate(Tol::witness())
            .unwrap();
        extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
            .unwrap()
            .body
    };
    let top: Vec<_> = a
        .faces()
        .filter(|(_, f)| match a.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, normal, .. }) => {
                (origin.z - 1.0).abs() < 1e-12 && normal.z > 0.5
            }
            _ => false,
        })
        .map(|(k, _)| k)
        .collect();
    let bottom: Vec<_> = b
        .faces()
        .filter(|(_, f)| match b.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, normal, .. }) => {
                (origin.z - 2.0).abs() < 1e-12 && normal.z < -0.5
            }
            _ => false,
        })
        .map(|(k, _)| k)
        .collect();
    let mut decls = BooleanDeclarations::none();
    decls.coincident_faces.push(FacePairDeclaration::new(
        top[0],
        bottom[0],
        ContactClass::Tangent,
    ));
    let err = topo::union_with(&a, &b, &decls, Tol::witness())
        .expect_err("plane×plane Tangent is outside the witness lane");
    assert!(
        matches!(
            err,
            BooleanError::UnsupportedDeclarationClass {
                class: ContactClass::Tangent
            }
        ),
        "{err:?}"
    );
    let msg = err.to_string();
    assert!(
        msg.contains("Tangent") && msg.contains("witness"),
        "the refusal names the class and the lane: {msg}"
    );
}
