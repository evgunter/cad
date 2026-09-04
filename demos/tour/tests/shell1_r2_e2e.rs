//! SHELL-1 R2 — the required end-to-end exercise (probe branch only).
//!
//! One consumer program against the public doors: `shell_open` the tour
//! teapot's own bellied vessel and, THROUGH THE RECORD ALONE, name the
//! rim of the mouth, its ring, and the inner twin of one wall. Then
//! rebuild at a different wall thickness and check the same source keys
//! still resolve to the same roles.
//!
//! "Through the record alone" is the whole point: nothing below reads a
//! surface, a plane, a normal or a loop of the RESULT to decide what a
//! result key is. The only body walked is the OPERAND, to pick the
//! designation and to name "one wall" — which is what a document layer
//! has (its stable names hang off the operand's recipe node).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::authoring::{p2, validated};
use pncad::geom::Surface;
use pncad::geom_core::{Point2, Tol, Vec2};
use pncad::prelude::{Open, Start};
use pncad::profile::{ArcSweep, Center, ProfileLoop, SketchPlane};
use pncad::sweep::{Revolution, RevolveAxis, revolve};
use pncad::topo::{Body, FaceKey, Shelled};

const R_FOOT: f64 = 4.0 / 64.0;
const R_NECK: f64 = 3.0 / 64.0;
const Y_FOOT: f64 = 1.0 / 64.0;
const Y_BELLY_C: f64 = 4.0 / 64.0;
const Y_MOUTH: f64 = 8.0 / 64.0;
const WALL: f64 = 1.0 / 128.0;
const FIT_TOL: f64 = 1e-6;

/// `teapot.rs`'s `vessel_meridian`, re-derived (a test crate cannot
/// import a binary's module).
fn vessel_meridian(tol: Tol) -> ProfileLoop<f64> {
    Open.at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(R_FOOT, 0.0), tol)
        .expect("the base disc")
        .line_to(Point2::new(R_FOOT, Y_FOOT), tol)
        .expect("the foot")
        .arc_to(
            Center {
                c: Point2::new(0.0, Y_BELLY_C),
                winding: ArcSweep::Ccw,
                p: Point2::new(R_NECK, Y_MOUTH),
            },
            tol,
        )
        .expect("the belly rides a sphere centred on the axis")
        .line_to(Point2::new(0.0, Y_MOUTH), tol)
        .expect("the mouth disc")
        .line_to(Start, tol)
        .expect("the axis closes the meridian")
        .into()
}

fn bellied(tol: Tol) -> Body<f64> {
    revolve(
        &validated(SketchPlane::xy(), vec![vessel_meridian(tol)], tol).expect("valid"),
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .expect("the meridian fully revolves")
    .body
}

fn plane_chart_at(body: &Body<f64>, y: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect()
}

/// A spherical wall face of the operand — "one wall", picked in the
/// OPERAND, which is the key space a document layer already names.
fn a_sphere_wall(body: &Body<f64>) -> FaceKey {
    body.faces()
        .find(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Sphere { .. })))
        .map(|(k, _)| k)
        .expect("the belly is a sphere zone")
}

/// What the emitter would mint, read out of the record only.
#[derive(Debug, PartialEq, Eq)]
struct Named {
    rim: FaceKey,
    ring_edges: usize,
    ring_vertices: usize,
    twin_of_wall: FaceKey,
    twin_is_live: bool,
    rim_is_a_designated_source: bool,
    holes: usize,
}

fn name_through_the_record(shelled: &Shelled<f64>, chart: &[FaceKey], wall: FaceKey) -> Named {
    let record = &shelled.naming;
    assert_eq!(record.rims.len(), 1, "one designated chart, one rim row");
    let rim = &record.rims[0];
    // The rim of the mouth, and its ring: both read straight off the row.
    let rim_face = rim.rim;
    // The inner twin of one wall: the `inner` channel, keyed by source.
    let twin = record
        .inner
        .iter()
        .find(|(_, src)| *src == wall)
        .map(|&(t, _)| t)
        .expect("every source face has an inner-twin row");
    Named {
        rim: rim_face,
        ring_edges: rim.ring_edges.len(),
        ring_vertices: rim.ring_vertices.len(),
        twin_of_wall: twin,
        twin_is_live: !record.dead.faces.contains(&twin),
        rim_is_a_designated_source: chart.contains(&rim_face),
        holes: rim.holes.len(),
    }
}

#[test]
fn the_teapots_mouth_is_nameable_through_the_record_alone() {
    let tol = Tol::witness();
    let source = bellied(tol);
    let mouth = plane_chart_at(&source, Y_MOUTH);
    assert_eq!(mouth.len(), 2, "a full revolve's mouth is two half-discs");
    let wall = a_sphere_wall(&source);

    let thin = pncad::topo::shell_open(&source, WALL, &mouth, FIT_TOL, tol)
        .expect("the pot opens at its mouth");
    let a = name_through_the_record(&thin, &mouth, wall);
    println!("[e2e] at WALL = {WALL}: {a:?}");
    println!(
        "[e2e]   record.ring = {:?}, dead f/e/v = {}/{}/{}",
        thin.naming.rims[0].ring,
        thin.naming.dead.faces.len(),
        thin.naming.dead.edges.len(),
        thin.naming.dead.vertices.len()
    );

    // The record's claims, checked against the body it came with —
    // this is the only place the RESULT is read, and it is the check,
    // not the naming.
    let rim_data = thin.body.get_face(a.rim).expect("the rim resolves");
    assert!(
        rim_data.rings.contains(&thin.naming.rims[0].ring),
        "the row's ring is a ring of the row's rim"
    );
    assert!(
        thin.body.get_face(a.twin_of_wall).is_some(),
        "the belly's inner twin is a live face of the result"
    );

    // ---- The same pot at a different wall. ----
    let thicker = WALL * 1.5;
    let fat = pncad::topo::shell_open(&source, thicker, &mouth, FIT_TOL, tol)
        .expect("the pot opens at a thicker wall");
    let b = name_through_the_record(&fat, &mouth, wall);
    println!("[e2e] at WALL = {thicker}: {b:?}");
    assert_eq!(a, b, "the same source keys resolve to the same roles");

    // And the SOURCE columns are stable across the two builds, which is
    // the property an emitter's stable names actually need.
    assert_eq!(
        thin.naming.outer, fat.naming.outer,
        "the outer channel is a function of the operand"
    );
    assert_eq!(
        thin.naming.inner, fat.naming.inner,
        "the inner channel is a function of the operand"
    );
    assert_eq!(
        thin.naming.rims[0].sources, fat.naming.rims[0].sources,
        "the designation row is a function of the operand"
    );
    assert_eq!(
        thin.naming.rims[0]
            .ring_edges
            .iter()
            .map(|&(_, s)| s)
            .collect::<Vec<_>>(),
        fat.naming.rims[0]
            .ring_edges
            .iter()
            .map(|&(_, s)| s)
            .collect::<Vec<_>>(),
        "the ring rows' SOURCE column is a function of the operand"
    );
}

/// **What the record does NOT carry**, measured rather than asserted:
/// can a consumer name the OUTER wall of the mouth's own rim band, the
/// rim's outer loop, or the pairing between a rim edge and the wall
/// face it separates, without walking the result body?
#[test]
fn what_the_record_leaves_the_emitter_to_walk() {
    let tol = Tol::witness();
    let source = bellied(tol);
    let mouth = plane_chart_at(&source, Y_MOUTH);
    let thin = pncad::topo::shell_open(&source, WALL, &mouth, FIT_TOL, tol).expect("opens");
    let rim = &thin.naming.rims[0];
    let rim_data = thin.body.get_face(rim.rim).expect("rim");
    println!(
        "[e2e-gap] rim outer loop {:?} appears in the record: {}",
        rim_data.outer,
        rim.holes.iter().any(|&(_, l)| l == rim_data.outer)
    );
    println!(
        "[e2e-gap] the record names {} loops in total ({} ring + {} holes); \
         the result carries {} loops",
        1 + rim.holes.len(),
        1,
        rim.holes.len(),
        thin.body.loops().count()
    );
    println!(
        "[e2e-gap] edges: record names {} inner twins + {} ring rows; \
         result carries {} edges, operand {}",
        thin.naming.inner_edges.len(),
        rim.ring_edges.len(),
        thin.body.edges().count(),
        source.edges().count()
    );
    println!(
        "[e2e-gap] no channel names: the rim's OUTER loop, a half-edge, \
         a shell, or the wall face on the far side of an inner twin edge"
    );
}
