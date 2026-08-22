//! Boss ∪ plate — the milestone's first curved boolean, VISIBLE
//! (M5 PR 11 §3; the body is PR 9's acceptance shape (ii)).
//!
//! A radius-0.5 cylindrical boss, authored as three 120° arcs (three
//! wall faces on one cylinder surface), pierces a 4×4×1 plate's top
//! face transversally and pokes out. The union's seam is the rim
//! circle where the wall crosses the top plane, minted as THREE exact
//! `Circle` arcs — carried on BOTH operands' sides, described
//! intrinsically as `Intersection`, certified pcurves at rest.
//!
//! What this stop shows, at demo altitude: a curved boolean whose seam
//! is exact geometry, whose volume is the closed form
//! `16 + π·0.25·0.6` on the nose, and whose mesh is watertight ACROSS
//! the curved-planar seam arcs — pinned here by the shared-chord
//! assertion (both adjacent faces consume the SAME chord ids; the
//! compute-chords-once-per-edge contract made visible).
//!
//! Constructors are generic over [`Scalar`] so the K-probe sweep
//! rebuilds the same geometry at the recording scalar.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::HashMap;

use pncad::geom_core::{Affine3, Point2, Vec3};
use pncad::profile::{Profile, SketchPlane, circle_split};
use pncad::sweep::{Extrusion, extrude};
use pncad::topo::{Body, BooleanBody, BooleanResult, Curve3};

use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};
use pncad::geom_core::Tol;

/// The plate: a 4×4×1 block, z ∈ [0, 1].
fn plate<S: Scalar>(tol: Tol) -> Body<S> {
    // Algebra-authored (LIB-U2 PR-2).
    let lp = crate::paths::path_polygon(&[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)], tol);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .unwrap();
    extrude(&profile, Extrusion::Distance(S::from_f64(1.0)), tol)
        .unwrap()
        .body
}

/// The boss: radius 0.5 about (2, 2), three 120° arcs, sketched at
/// z = 0.4 (strictly inside the plate), extruded 1.2 → pokes out to
/// z = 1.6.
fn boss<S: Scalar>(tol: Tol) -> Body<S> {
    // The DECLARED-SUBDIVISION carrier (`circle_split`), not `circle`:
    // this boss is deliberately split into THREE 120-degree arcs of one
    // carrier, because the stop's whole point is a transverse curved
    // boolean crossing a three-face rim seam and the assertion below
    // pins the count. `circle` authors no seam at all (its private
    // lowering is the conventional two-semicircle split), so the split
    // count has to be said out loud; `circle_split` is the door that
    let rim = circle_split(
        Point2::new(S::from_f64(2.0), S::from_f64(2.0)),
        S::from_f64(0.5),
        3,
        S::from_f64(0.0),
        tol,
    )
    .expect("the three-arc rim authors");
    let lp = rim.into();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(
        S::from_f64(0.0),
        S::from_f64(0.0),
        S::from_f64(0.4),
    )));
    let profile = Profile::new(plane, vec![lp]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(S::from_f64(1.2)), tol)
        .unwrap()
        .body
}

/// The union (a seamed boolean body — 3′ validates with its own
/// declared contacts, like every boolean stop).
pub fn build<S: Scalar>(tol: Tol) -> BooleanBody<S> {
    match pncad::topo::union(&plate::<S>(tol), &boss::<S>(tol), tol)
        .expect("the first transverse curved boolean")
    {
        BooleanResult::Body(bb) => bb,
        other => panic!("the boss union yields a body, got {other:?}"),
    }
}

/// The shared-chord watertightness pin (module docs): every chord
/// segment of every seam arc (the z = 1, r = 0.5 circles) is an edge
/// of exactly TWO triangles — the curved wall's and the ringed top
/// face's — through the SAME mesh vertex ids.
fn assert_seam_chords_shared(body: &Body<f64>, delta: f64, tol: Tol) -> usize {
    let mesh = pncad::mesh::tessellate(body, delta, tol).expect("boss∪plate tessellates");
    pncad::mesh::validate::check_mesh(&mesh).expect("watertight");
    let mut uses: HashMap<(u32, u32), u32> = HashMap::new();
    for patch in &mesh.patches {
        for t in &patch.triangles {
            for k in 0..3 {
                let (a, b) = (t[k], t[(k + 1) % 3]);
                *uses.entry((a.min(b), a.max(b))).or_insert(0) += 1;
            }
        }
    }
    let mut seam_edges = 0;
    for poly in &mesh.boundaries {
        let Some(edge) = body.get_edge(poly.edge) else {
            continue;
        };
        let Some(curve) = body.get_curve_geom(edge.curve).and_then(|g| g.certified()) else {
            continue;
        };
        let Curve3::Circle { center, radius, .. } = *curve.carrier() else {
            continue;
        };
        if (center.z - 1.0).abs() > 1e-9 || (radius - 0.5).abs() > 1e-9 {
            continue;
        }
        seam_edges += 1;
        for w in poly.points.windows(2) {
            let key = (w[0].min(w[1]), w[0].max(w[1]));
            assert_eq!(
                uses.get(&key).copied().unwrap_or(0),
                2,
                "seam chord {key:?} must be consumed by BOTH adjacent faces \
                 (compute-chords-once-per-edge)"
            );
        }
    }
    assert_eq!(seam_edges, 3, "the rim seam is three arcs (three walls)");
    seam_edges
}

/// The showcase stop.
pub fn stops(tol: Tol) -> Vec<Stop> {
    let bb = build::<f64>(tol);
    let expect = 16.0 + core::f64::consts::PI * 0.25 * 0.6;
    let vol = pncad::topo::mass_properties(&bb.body, tol).unwrap().volume;
    assert!(
        (vol - expect).abs() < 1e-6,
        "vol(plate∪boss) = {vol}, closed form {expect}"
    );
    let seam_arcs = assert_seam_chords_shared(&bb.body, 1e-2, tol);
    vec![Stop {
        name: "bossplate",
        // Short — montage captions share the panel's width (review
        // NOTE: the long form truncated and collided with its
        // neighbour); the story/note carry the long narration.
        caption: "boss ∪ plate (first curved boolean)".to_string(),
        montage: true,
        story: "a cylindrical boss unioned into a plate — the milestone's first \
                transverse curved boolean, rendered",
        ops: "extrude(plate) ∪ extrude(three-arc boss); seam = 3 exact Circle arcs \
              (Intersection-described, certified pcurves at rest)",
        delta: 1e-2,
        note: Some(format!(
            "the seam where the boss meets the plate top is EXACT geometry — {seam_arcs} \
             circle arcs carried on both operands' sides — and the mesh consumes one \
             chord set per seam edge, so the curved wall and the planar top are \
             watertight against each other by construction; V = 16 + π·0.25·0.6 = \
             {expect:.6} on the nose"
        )),
        view: View {
            elev: 24.0,
            azim: -55.0,
            up: 'z',
        },
        // A TRANSVERSE curved boolean declares no contacts, and the 3′
        // census is exact-on-planar by ruling (C12.4/OQ5: a TOUCHING
        // curved result refuses there — pinned in
        // `sweep/tests/m5_pr9_boss_union.rs`); a contact-free body
        // takes plain tier 3, which this one passes in full.
        bodies: vec![{
            let contact_free = bb.contacts.vv.is_empty()
                && bb.contacts.a_on_b.is_empty()
                && bb.contacts.b_on_a.is_empty();
            if contact_free {
                SceneBody::plain("bossplate", [0.85, 0.55, 0.25], bb.body)
            } else {
                SceneBody::seamed("bossplate", [0.85, 0.55, 0.25], bb.body, bb.contacts)
            }
        }],
    }]
}
