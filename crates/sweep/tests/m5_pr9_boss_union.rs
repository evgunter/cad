//! M5 PR 9 acceptance shape (ii): cylinder boss ∪ plate — the first
//! transverse curved boolean end to end. The boss pierces the plate's
//! top face; the seam is the rim circle where the wall crosses the
//! top plane, minted as THREE conic arcs (the boss's three walls),
//! carried by exact `Circle` carriers on BOTH operands' sides,
//! described intrinsically, certified pcurves at rest, tier-3 valid,
//! volume backstopped.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tolerance, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::Body;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The plate: a 4×4×1 block, z ∈ [0, 1].
fn plate() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex {
            pos: p2(0.0, 0.0),
            bulge: 0.0,
        },
        ProfileVertex {
            pos: p2(4.0, 0.0),
            bulge: 0.0,
        },
        ProfileVertex {
            pos: p2(4.0, 4.0),
            bulge: 0.0,
        },
        ProfileVertex {
            pos: p2(0.0, 4.0),
            bulge: 0.0,
        },
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0)).unwrap().body
}

/// The boss: a radius-0.5 disc centered at (2, 2) authored as THREE
/// 120° arcs (three wall faces on ONE cylinder surface, meridian
/// struts every 120° — a 120° seam arc's endpoint tangents face its
/// chord definitely, which the join's mutual-facing germ test
/// requires; the two-arc disc's semicircles leave the germ exactly
/// perpendicular to the chord). Sketched at z = 0.4 (strictly inside
/// the plate), extruded 1.2: it pierces the top face transversally
/// and pokes out to z = 1.6.
fn boss() -> Body<f64> {
    let b120 = (core::f64::consts::PI / 6.0).tan(); // bulge of a 120° arc
    let at = |deg: f64| {
        let th = deg.to_radians();
        p2(2.0 + 0.5 * th.cos(), 2.0 + 0.5 * th.sin())
    };
    let lp = ProfileLoop::new(vec![
        ProfileVertex {
            pos: at(0.0),
            bulge: b120,
        },
        ProfileVertex {
            pos: at(120.0),
            bulge: b120,
        },
        ProfileVertex {
            pos: at(240.0),
            bulge: b120,
        },
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.4)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.2)).unwrap().body
}

#[test]
fn the_boss_union_lands_end_to_end() {
    let a = plate();
    let b = boss();
    let out = topo::union(&a, &b).expect("the first transverse curved boolean");
    let body = &out.body().expect("a seamed result").body;

    // Tier 3 in full: dihedrals, descriptions, pcurve caches, +V.
    if let Err(errs) = topo::validate_geometric(body) {
        panic!("shape (ii) must be tier-3 valid: {errs:?}");
    }

    // The seam: circle-carrier edges at z = 1, radius 0.5 about
    // (2, 2), described intrinsically as Intersection.
    let mut seam_arcs = 0;
    for (_, e) in body.edges() {
        let Some(c) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
            continue;
        };
        if let geom_curves::Curve3::Circle { center, radius, .. } = *c.carrier()
            && (center.z - 1.0).abs() < 1e-9
            && (radius - 0.5).abs() < 1e-9
        {
            seam_arcs += 1;
            assert!(
                matches!(
                    c.description(),
                    geom_brep::EdgeGeometry::Intersection { .. }
                ),
                "a transverse curved seam edge is intrinsic (D6)"
            );
        }
    }
    assert_eq!(seam_arcs, 3, "the rim seam is three arcs (three walls)");

    // Volume: plate (16) + protruding boss (π·0.25·0.6), within the
    // exact-B-rep props' honesty.
    let vol = topo::mass_properties(body).unwrap().volume;
    let expect = 16.0 + core::f64::consts::PI * 0.25 * 0.6;
    assert!(
        (vol - expect).abs() < 1e-6,
        "vol(A∪B) = {vol}, expected {expect}"
    );
}

#[test]
fn a_touching_curved_result_refuses_typed_at_the_three_prime_gate() {
    // The M5 envelope's frontier (C12.4/OQ5, decided): the census
    // stays exact-on-planar, so a curved body carrying declared
    // contacts (the touching class) refuses CensusUnsupported at the
    // 3′ gate — with the deferral named, never a sampled census.
    let b = boss();
    let mut contacts = topo::ContactRecords::default();
    // A declared v-v self-contact record (the minimal touching
    // currency; which record it is does not matter — the census
    // walks the body's inventory first and meets the wall).
    let v = b.vertices().next().unwrap().0;
    contacts.vv.push(topo::VvContact { a: v, b: v });
    let errs =
        topo::validate_pseudomanifold(&b, &contacts).expect_err("curved inventory refuses at 3′");
    let hit = errs.iter().find_map(|e| match e {
        topo::ValidationError::CensusUnsupported { .. } => Some(format!("{e}")),
        _ => None,
    });
    let msg = hit.expect("a CensusUnsupported refusal");
    assert!(msg.contains("exact-on-planar"), "{msg}");
    assert!(msg.contains("touching curved result refuses HERE"), "{msg}");
}
