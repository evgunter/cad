//! FILLET-RIM review probes (r1), `sweep` half — the PR's Phase 1 table
//! re-read from the tree, the rotation claim swept over EVERY circle
//! edge of the corpus (not a chosen radius list), and the seam-vertex
//! recourse followed literally from the refusal's own text through the
//! door it names.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use geom::Curve3;
use geom_core::{Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::blend::build::fillet_edges;
use sweep::test_support::{arcs_at, ball_poled_z, cube, dome, lantern, sphere_zone, waisted};
use sweep::{Extrusion, Revolution, extrude};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::query::rim_of;
use topo::{
    Body, BooleanDeclarations, EdgeKey, RimError, VertexKey, mass_properties, validate_geometric,
};

fn tol() -> Tol {
    Tol::witness()
}

fn is_rotation(a: &[EdgeKey], b: &[EdgeKey]) -> bool {
    a.len() == b.len() && (0..a.len()).any(|k| (0..a.len()).all(|i| a[(i + k) % a.len()] == b[i]))
}

fn ends(body: &Body<f64>, k: EdgeKey) -> (VertexKey, VertexKey) {
    let e = body.get_edge(k).unwrap();
    (
        body.get_half_edge(e.he_plus).unwrap().start,
        body.half_edge_end(e.he_plus).unwrap(),
    )
}

/// A die pip's shape: a cube with a ball subtracted at one face's centre.
fn cube_minus_ball() -> Body<f64> {
    let out = boolean_op_with(
        BooleanOp::Subtract,
        &cube(1.0, tol()),
        &ball_poled_z(0.3, Vec3::new(0.5, 0.5, 1.0), tol()),
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol(),
    )
    .unwrap_or_else(|e| panic!("cube minus ball: {e}"));
    out.body().expect("a body").body.clone()
}

/// A unit plate with a circular through-hole (two-vertex bulge loop).
fn plate_with_hole() -> Body<f64> {
    let p2 = |x: f64, y: f64| Point2::new(x, y);
    let outer = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    let hole = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.4, 0.5), 1.0),
        ProfileVertex::new(p2(0.6, 0.5), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![outer, hole])
        .validate(tol())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0), tol())
        .unwrap()
        .body
}

struct Circle {
    center: [u64; 3],
    axis: [u64; 3],
    neg_axis: [u64; 3],
    radius: u64,
    u_ref: [u64; 3],
}

fn circle_bits(body: &Body<f64>, k: EdgeKey) -> Option<Circle> {
    let e = body.get_edge(k)?;
    let c = body.get_curve_geom(e.curve)?.certified()?;
    let Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    } = *c.carrier()
    else {
        return None;
    };
    let b3 = |v: Vec3<f64>| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()];
    Some(Circle {
        center: [center.x.to_bits(), center.y.to_bits(), center.z.to_bits()],
        axis: b3(axis),
        neg_axis: b3(-axis),
        radius: radius.to_bits(),
        u_ref: b3(u_ref),
    })
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
/// `NotOneRim` (the partial); within each answered rim every arc's
/// stored `center`, `radius` and `axis` bit-equal the seed's and the
/// axis is never the negation; `u_ref` agreement is recorded, not
/// asserted. Every member's answer starts at itself and is a rotation
/// of the seed's, and the second arc continues from the seed's
/// `he_plus` end.
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
            if circle_bits(body, k).is_none() {
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
                    if rim.len() > 1 {
                        let (_, seed_end) = ends(body, k);
                        let (n0, n1) = ends(body, rim[1]);
                        assert!(
                            n0 == seed_end || n1 == seed_end,
                            "{name}: the second arc continues from the seed's he_plus end"
                        );
                    }
                    let mut key = rim.clone();
                    key.sort();
                    if let Some(prev) = rims.get(&key) {
                        assert!(is_rotation(prev, &rim), "{name}: {rim:?} vs {prev:?}");
                    } else {
                        rims.insert(key, rim);
                    }
                }
                Err(RimError::NotOneRim { arcs, .. }) => {
                    assert!(
                        !*closed,
                        "{name}: a closed class refused NotOneRim on {k:?}"
                    );
                    assert!(arcs.contains(&k));
                    refused += 1;
                }
                Err(e) => panic!("{name}: unexpected refusal on {k:?}: {e}"),
            }
        }
        let mut u_ref_agree = 0;
        let mut u_ref_differ = 0;
        for rim in rims.values() {
            let seed = circle_bits(body, rim[0]).unwrap();
            for &a in &rim[1..] {
                let c = circle_bits(body, a).unwrap();
                assert_eq!(c.center, seed.center, "{name}: center bit-equal");
                assert_eq!(c.radius, seed.radius, "{name}: radius bit-equal");
                assert_eq!(c.axis, seed.axis, "{name}: axis bit-equal");
                assert_ne!(c.axis, seed.neg_axis, "{name}: axis never negated");
                if c.u_ref == seed.u_ref {
                    u_ref_agree += 1;
                } else {
                    u_ref_differ += 1;
                }
            }
        }
        let arcs: Vec<usize> = rims.values().map(Vec::len).collect();
        println!(
            "PHASE1 {name}: rims={} arcs_per_rim={arcs:?} seams={seams} refused_open={refused} \
             u_ref_pairs(agree={u_ref_agree}, differ={u_ref_differ})",
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
        text.contains("`topo::query::rim_of` on any one of its arcs"),
        "the refusal names the door: {text}"
    );
    let rim = topo::query::rim_of(&source, seed).expect("the refusing arc names its rim");
    assert_eq!(rim.len(), 2);
    let out = fillet_edges(&source, &rim, 0.05, tol()).expect("the door's answer carves");
    assert_eq!(out.band_faces.len(), 1);
    validate_geometric(&out.body, tol()).unwrap();
    let after = mass_properties(&out.body, tol()).unwrap().volume;
    assert!(after < before, "convex: material removed");
}
