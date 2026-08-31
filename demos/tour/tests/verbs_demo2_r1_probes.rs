//! VERBS-DEMO2 review probes (ordinal 83) — unique-signal attacks on
//! PR #1054's load-bearing claims. Each probe is designed to be
//! evidence the scenes' own asserts cannot already be: teeth checks
//! (would the pin actually red?) and generality checks (is the pinned
//! fact the door's contract or this fixture's coincidence?).
//!
//! Review-lane only; not part of the PR under review.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::authoring::{p2, validated};
use pncad::geom::Surface;
use pncad::geom_brep::SurfaceKind;
use pncad::geom_core::{Point2, Point3, Tol, Vec2, Vec3};
use pncad::prelude::{Open, Start, fillet_edges};
use pncad::profile::{ArcSweep, Center, ProfileLoop, SketchPlane};
use pncad::sweep::{
    Revolution, RevolveAxis, TubeWindow, revolve, tube_along_arc, tube_along_arc_hollow,
};
use pncad::topo::{Body, EdgeKey};

/// P1 — TEETH of the cross-scene mesh pin. The scene asserts the
/// hollow elbow's outer-wall triangle counts equal the solid tube's at
/// one shared δ. This probe shows the compared quantity is actually
/// SENSITIVE to a one-sided sizing change: mesh the solid under a
/// budget only 25% coarser and the counts separate. If this probe ever
/// finds the counts equal, the scene's pin has no teeth (the counts
/// would be constants of the surface, not of the schedule).
#[test]
fn p1_mesh_pin_reds_on_a_one_sided_sizing_change() {
    let tol = Tol::witness();
    let hollow = tube_along_arc_hollow::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        2.0,
        TubeWindow::Arc { t0: 0.25, t1: 1.75 },
        0.5,
        0.125,
        tol,
    )
    .expect("hollow elbow");
    let solid = tube_along_arc::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        2.0,
        TubeWindow::Arc { t0: 0.25, t1: 1.75 },
        0.5,
        tol,
    )
    .expect("solid tube");
    let outer_bits = 0.5f64.to_bits();
    let counts = |body: &Body<f64>, delta: f64| -> Vec<usize> {
        let mesh = pncad::mesh::tessellate(body, delta, tol).expect("tessellates");
        body.faces()
            .filter(|(_, f)| {
                matches!(
                    body.get_surface(f.surface),
                    Some(Surface::Torus { minor_radius, .. })
                        if minor_radius.to_bits() == outer_bits
                )
            })
            .map(|(k, _)| {
                mesh.patches
                    .iter()
                    .find(|p| p.face == k)
                    .expect("meshed")
                    .triangles
                    .len()
            })
            .collect()
    };
    // The scene's own equality, reproduced.
    assert_eq!(counts(&hollow.body, 1e-2), counts(&solid.body, 1e-2));
    assert_eq!(counts(&hollow.body, 1e-2), vec![17152, 17152]);
    // The fork: one door under a 25%-coarser budget. MUST differ, or
    // the pinned equality could never red on a sizing fork.
    assert_ne!(
        counts(&hollow.body, 1e-2),
        counts(&solid.body, 1.25e-2),
        "outer-wall triangle counts are insensitive to a 25% sizing change — \
         the scene's cross-scene mesh pin has no teeth"
    );
}

/// P2 — GENERALITY of the storage contract. The scene pins outer ==
/// 0.5 and inner == 0.5 - 0.125 bit for bit at its own fixture. This
/// probe asks the door the same question at unrelated constants (an
/// outer radius and wall with no finite binary alignment), so a pass
/// here says the bit-equality is the door's contract, not an artifact
/// of the demo's round numbers.
#[test]
fn p2_storage_contract_holds_at_unaligned_constants() {
    let tol = Tol::witness();
    let (outer, wall) = (0.61, 0.17);
    let hollow = tube_along_arc_hollow::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        1.7,
        TubeWindow::Arc { t0: 0.4, t1: 2.1 },
        outer,
        wall,
        tol,
    )
    .expect("hollow elbow at unaligned constants");
    let mut got: Vec<u64> = hollow
        .body
        .faces()
        .filter_map(|(_, f)| match hollow.body.get_surface(f.surface) {
            Some(Surface::Torus { minor_radius, .. }) => Some(minor_radius.to_bits()),
            _ => None,
        })
        .collect();
    got.sort_unstable();
    let inner = outer - wall; // the caller's own one IEEE subtraction
    let mut want = vec![
        inner.to_bits(),
        inner.to_bits(),
        outer.to_bits(),
        outer.to_bits(),
    ];
    want.sort_unstable();
    assert_eq!(
        got, want,
        "the inner wall's stored radius is not the caller's own subtraction \
         at unaligned constants — the scene's bit-equality was a fixture \
         coincidence"
    );
}

// ---- the bud, replicated from the scene's constants (the scene is a
// bin module, so an integration test restates them; any drift from
// demos/tour/src/bud.rs is a probe bug, not a finding). ----
const BORE: f64 = 0.2;
const MOUTH: (f64, f64) = (0.8, 0.6);
const LIP_R: f64 = 0.35;
const TOP: f64 = 0.75;
const ROLL: f64 = 0.05;

fn bud(tol: Tol) -> Body<f64> {
    let meridian: ProfileLoop<f64> = Open
        .at(Point2::new(BORE, 0.0))
        .line_to(Point2::new(1.0, 0.0), tol)
        .expect("base")
        .arc_to(
            Center {
                c: Point2::new(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: Point2::new(MOUTH.0, MOUTH.1),
            },
            tol,
        )
        .expect("belly")
        .line_to(Point2::new(LIP_R, TOP), tol)
        .expect("pucker")
        .line_to(Point2::new(BORE, TOP), tol)
        .expect("lip")
        .line_to(Start, tol)
        .expect("bore")
        .into();
    let profile =
        validated(SketchPlane::xy(), vec![meridian], tol).expect("the bud's meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .expect("revolves")
    .body
}

fn rims_between(body: &Body<f64>, a: SurfaceKind, b: SurfaceKind) -> Vec<EdgeKey> {
    let kind_at = |he| {
        let l = body.get_half_edge(he)?.parent_loop;
        let f = body.get_loop(l)?.face;
        body.get_surface(body.get_face(f)?.surface)
            .map(SurfaceKind::of)
    };
    body.edges()
        .filter(|(_, e)| {
            let (ka, kb) = (kind_at(e.he_plus), kind_at(e.he_minus));
            (ka, kb) == (Some(a), Some(b)) || (ka, kb) == (Some(b), Some(a))
        })
        .map(|(k, _)| k)
        .collect()
}

fn bore_base(body: &Body<f64>) -> EdgeKey {
    let rims = rims_between(body, SurfaceKind::Cylinder, SurfaceKind::Plane);
    assert_eq!(rims.len(), 2);
    *rims
        .iter()
        .min_by(|a, b| {
            let station = |e: &EdgeKey| match *body
                .get_curve_geom(body.get_edge(*e).unwrap().curve)
                .unwrap()
                .certified()
                .unwrap()
                .carrier()
            {
                pncad::geom::Curve3::Circle { center, .. } => center.y,
                ref other => panic!("latitude rim is a circle, got {other:?}"),
            };
            station(a).partial_cmp(&station(b)).unwrap()
        })
        .unwrap()
}

/// P3 — the grain of the one-call door, RE-CUT at #935 (BLEND-2). The
/// probe originally pinned the ATTRIBUTION of the shared-support
/// refusal ([mouth, lip] refused on the sharing, not the rim count);
/// the door now SERVES shared-wall annulus pairs by re-reading the
/// later rim's seam keys between carves, so the probe pins the two
/// facts that replace it: the shared pair builds in one call, and it
/// builds the SAME body the sequential composition builds (volume
/// equal to the bit). The disjoint-support fork is unchanged.
#[test]
fn p3_the_shared_pair_builds_and_matches_the_sequential_composition() {
    let tol = Tol::witness();
    let sharp = bud(tol);
    let mouth = {
        let hits = rims_between(&sharp, SurfaceKind::Sphere, SurfaceKind::Cone);
        assert_eq!(hits.len(), 1);
        hits[0]
    };
    let lip = {
        let hits = rims_between(&sharp, SurfaceKind::Cone, SurfaceKind::Plane);
        assert_eq!(hits.len(), 1);
        hits[0]
    };
    let base = bore_base(&sharp);

    // The shared pair BUILDS in one call — and is the sequential
    // composition to the bit.
    let one = fillet_edges(&sharp, &[mouth, lip], ROLL, tol)
        .expect("mouth + lip share the pucker cone; one call serves the pair (#935)");
    assert_eq!(one.band_faces.len(), 2, "two bands from the shared pair");
    let first = fillet_edges(&sharp, &[mouth], ROLL, tol).expect("the mouth alone");
    let lip2 = {
        let hits = rims_between(&first.body, SurfaceKind::Cone, SurfaceKind::Plane);
        assert_eq!(hits.len(), 1);
        hits[0]
    };
    let second = fillet_edges(&first.body, &[lip2], ROLL, tol).expect("the lip on the result");
    let volume = |b: &pncad::topo::Body<f64>| {
        pncad::topo::mass_properties(b, tol)
            .expect("mass properties")
            .volume
    };
    let (v1, v2) = (volume(&one.body), volume(&second.body));
    assert!(
        v1 == v2,
        "one call == sequential composition to the bit: {v1:.17e} vs {v2:.17e}"
    );

    // Two rims with disjoint supports roll together in ONE call.
    let rolled = fillet_edges(&sharp, &[mouth, base], ROLL, tol)
        .expect("mouth + bore base share no support face, so one call composes");
    assert_eq!(rolled.band_faces.len(), 2, "two bands from the one call");
}
