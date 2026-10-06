//! FILLET-RIM review probes (r1), `sweep` half — the PR's Phase 1 table
//! re-read from the tree, the rotation claim swept over EVERY circle
//! edge of the corpus (not a chosen radius list), and the seam-vertex
//! recourse followed literally from the refusal's own text through the
//! door it names.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use sweep::ExtrudeSide;

use geom::Curve3;
use geom_core::{Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane, test_support::bulge_loop};
use sweep::blend::build::fillet_edges;
use sweep::test_support::{
    arcs_at, ball_poled_z, cube, dome, lantern, realized, sphere_zone, waisted,
};
use sweep::{Extrusion, Revolution, extrude};
use topo::boolean::BooleanOp;
use topo::query::rim_of;
use topo::{Body, EdgeKey, RimError, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn is_rotation(a: &[EdgeKey], b: &[EdgeKey]) -> bool {
    a.len() == b.len() && (0..a.len()).any(|k| (0..a.len()).all(|i| a[(i + k) % a.len()] == b[i]))
}

/// A die pip's shape: a cube with a ball subtracted at one face's centre.
fn cube_minus_ball() -> Body<f64> {
    realized(
        BooleanOp::Subtract,
        &cube(1.0, tol()),
        &ball_poled_z(0.3, Vec3::new(0.5, 0.5, 1.0), tol()),
        tol(),
    )
}

/// A unit plate with a circular through-hole (two-vertex bulge loop).
fn plate_with_hole() -> Body<f64> {
    let outer = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(0.0, 1.0),
    ]);
    let hole = bulge_loop(vec![
        (Point2::new(0.4, 0.5), 1.0),
        (Point2::new(0.6, 0.5), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![outer, hole])
        .validate(tol())
        .unwrap();
    extrude(
        &profile,
        Extrusion::Distance {
            depth: 1.0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body
}

fn is_circle(body: &Body<f64>, k: EdgeKey) -> bool {
    matches!(
        body.get_edge(k)
            .and_then(|e| body.get_curve_geom(e.curve))
            .and_then(|g| g.certified())
            .map(|c| c.carrier()),
        Some(Curve3::Circle { .. })
    )
}

fn co_surface(body: &Body<f64>, k: EdgeKey) -> bool {
    let e = body.get_edge(k).unwrap();
    let s = |he| {
        let l = body.get_half_edge(he).unwrap().parent_loop;
        body.get_face(body.get_loop(l).unwrap().face)
            .unwrap()
            .surface
    };
    s(e.he_plus) == s(e.he_minus)
}

/// **Phase 1, re-read; and the rotation claim on every arc of every
/// rim.** One body per class the spec lists, plus the partial revolve.
/// For each: every circle edge with two distinct side surfaces is
/// either an arc of a rim the door answers (closed classes) or refuses
/// `NotOneRim` (the partial). Every member's answer starts at itself
/// and is a rotation of the first seed's.
#[test]
fn phase_one_reread_and_the_rotation_claim_on_every_arc() {
    let mut repaired = lantern(tol());
    repaired.merge_coplanar_faces(tol()).unwrap();
    let bodies: Vec<(&str, Body<f64>, bool)> = vec![
        ("seam-split revolve (lantern)", lantern(tol()), true),
        ("repaired (lantern, merged caps)", repaired, true),
        ("one-edge rims (dome)", dome(1.0, tol()), true),
        ("boolean-made (cube minus ball)", cube_minus_ball(), true),
        (
            "extrude hole rims (plate with hole)",
            plate_with_hole(),
            true,
        ),
        (
            "partial revolve (zone, pi/2)",
            sphere_zone(
                0.5,
                Revolution::Partial(core::f64::consts::FRAC_PI_2),
                tol(),
            ),
            false,
        ),
    ];
    let mut total_rims = 0;
    for (name, body, closed) in &bodies {
        let mut rims: BTreeMap<Vec<EdgeKey>, Vec<EdgeKey>> = BTreeMap::new();
        let mut seams = 0;
        let mut refused = 0;
        for (k, _) in body.edges() {
            if !is_circle(body, k) {
                continue;
            }
            if co_surface(body, k) {
                seams += 1;
                assert!(
                    matches!(rim_of(body, k), Err(RimError::CoSurface { .. })),
                    "{name}: a seam refuses CoSurface"
                );
                continue;
            }
            match rim_of(body, k) {
                Ok(rim) => {
                    assert!(*closed, "{name}: an open rim must not be answered");
                    assert_eq!(rim[0], k, "{name}: the seed comes first");
                    let mut key = rim.clone();
                    key.sort();
                    if let Some(prev) = rims.get(&key) {
                        assert!(is_rotation(prev, &rim), "{name}: {rim:?} vs {prev:?}");
                    } else {
                        rims.insert(key, rim);
                    }
                }
                Err(RimError::NotOneRim { walked, .. }) => {
                    assert!(
                        !*closed,
                        "{name}: a closed class refused NotOneRim on {k:?}"
                    );
                    assert_eq!(walked[0], k, "{name}: the walk starts at the seed");
                    refused += 1;
                }
                Err(e) => panic!("{name}: unexpected refusal on {k:?}: {e}"),
            }
        }
        let arcs: Vec<usize> = rims.values().map(Vec::len).collect();
        println!(
            "PHASE1 {name}: rims={} arcs_per_rim={arcs:?} seams={seams} refused_open={refused}",
            rims.len()
        );
        if *closed {
            assert!(!rims.is_empty(), "{name}: not vacuous");
        } else {
            assert!(refused > 0 && rims.is_empty(), "{name}: every arc refuses");
        }
        total_rims += rims.len();
    }
    println!("PHASE1 total rims answered: {total_rims}");
    assert!(
        total_rims >= 12,
        "the sweep covers more rims than the PR's ten"
    );
}

/// **The recourse, followed literally from the refusal.** Fillet one
/// arc of a convex seam-split rim; the refusal's text names
/// `topo::query::rim_of`; call exactly that on the arc that refused;
/// hand the answer back to `fillet_edges`; it carves.
#[test]
fn the_refusals_own_text_names_the_door_and_following_it_carves() {
    let source = waisted(tol());
    let before = mass_properties(&source, tol()).unwrap().volume;
    let seed = arcs_at(&source, 1.0, 0.0)[0];
    let Err(refusal) = fillet_edges(&source, &[seed], 0.05, tol()) else {
        panic!("one arc stops at the seam vertex")
    };
    let text = refusal.error.to_string();
    assert!(
        text.contains("request the rim whole, every arc the seam split it into"),
        "the refusal names the request that carves: {text}"
    );
    assert!(
        text.contains("the rim query, `rim_of`"),
        "the refusal names the door that answers the request: {text}"
    );
    let rim = topo::query::rim_of(&source, seed).expect("the refusing arc names its rim");
    assert_eq!(rim.len(), 2);
    let out = fillet_edges(&source, &rim, 0.05, tol()).expect("the door's answer carves");
    assert_eq!(out.band_faces.len(), 1);
    validate_geometric(&out.body, tol()).unwrap();
    let after = mass_properties(&out.body, tol()).unwrap().volume;
    assert!(after < before, "convex: material removed");
}
