//! SHELL-1 R1 end-to-end exercise (probe branch only): a consumer of the
//! public doors that names the tour teapot's mouth rim, its ring, and
//! the inner twin of the belly wall THROUGH THE RECORD ALONE, then
//! rebuilds the pot at another wall thickness and checks the same rows
//! still resolve to the same roles.
//!
//! The pot's meridian is the tour's own (`demos/tour/src/teapot.rs`
//! `vessel_meridian`), restated here because the scene exposes no door
//! to its operand.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::authoring::validated;
use pncad::geom::Surface;
use pncad::geom_core::{Point2, Tol, Vec2};
use pncad::prelude::{Open, Start, query};
use pncad::profile::{ArcSweep, Center, ProfileLoop, SketchPlane};
use pncad::sweep::{Revolution, RevolveAxis, revolve};
use pncad::topo::{Body, FaceKey, LoopBoundary, LoopKey, ShellNaming, Shelled};

const R_FOOT: f64 = 4.0 / 64.0;
const R_BELLY: f64 = 5.0 / 64.0;
const R_NECK: f64 = 3.0 / 64.0;
const Y_FOOT: f64 = 1.0 / 64.0;
const Y_BELLY_C: f64 = 4.0 / 64.0;
const Y_MOUTH: f64 = 8.0 / 64.0;
const WALL: f64 = 1.0 / 128.0;
const FIT_TOL: f64 = 1e-6;

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
        &validated(SketchPlane::xy(), vec![vessel_meridian(tol)], tol).expect("validates"),
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .expect("revolves")
    .body
}

fn plane_chart_at(body: &Body<f64>, y: f64) -> Vec<FaceKey> {
    query::all_faces(body)
        .into_iter()
        .filter(|&f| {
            matches!(
                body.get_face(f).and_then(|face| body.get_surface(face.surface)),
                Some(Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-12
            )
        })
        .collect()
}

/// The operand's belly wall: the faces on a SPHERE. (Selecting the
/// operand face by description is the recipe layer's job — the revolve's
/// own naming record would hand it over; here it is a read of the
/// operand, which is the record's SOURCE key space.)
fn sphere_faces(body: &Body<f64>) -> Vec<FaceKey> {
    query::all_faces(body)
        .into_iter()
        .filter(|&f| {
            matches!(
                body.get_face(f)
                    .and_then(|face| body.get_surface(face.surface)),
                Some(Surface::Sphere { .. })
            )
        })
        .collect()
}

/// What the consumer wants: three roles, read from the record alone.
struct Roles {
    rim: FaceKey,
    ring: LoopKey,
    belly_twin: FaceKey,
}

fn roles(source: &Body<f64>, mouth: &[FaceKey], naming: &ShellNaming) -> Roles {
    // The rim of the mouth: the RimNaming whose sources are the mouth
    // designation. There is no lookup door; scan.
    let rim = naming
        .rims
        .iter()
        .find(|r| r.sources.iter().any(|s| mouth.contains(s)))
        .expect("the mouth's rim row");
    // The inner twin of ONE belly wall face: scan `inner` for the source.
    let belly = sphere_faces(source)[0];
    let (twin, _) = naming
        .inner
        .iter()
        .copied()
        .find(|&(_, s)| s == belly)
        .expect("the belly wall has an inner twin row");
    Roles {
        rim: rim.rim,
        ring: rim.ring,
        belly_twin: twin,
    }
}

fn circle_radius(body: &Body<f64>, lk: LoopKey) -> f64 {
    let LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
        panic!("empty loop")
    };
    let he = body.loop_cycle(first).unwrap()[0];
    let e = body.get_edge(body.get_half_edge(he).unwrap().edge).unwrap();
    match body
        .get_curve_geom(e.curve)
        .and_then(|g| g.certified())
        .unwrap()
        .carrier()
    {
        pncad::geom::Curve3::Circle { radius, .. } => *radius,
        other => panic!("ring bounded by {other:?}"),
    }
}

fn check_roles(what: &str, body: &Body<f64>, r: &Roles, wall: f64) {
    // The rim: planar at the mouth, annular, its one ring is the row's.
    let rim = body
        .get_face(r.rim)
        .unwrap_or_else(|| panic!("{what}: rim resolves"));
    assert!(
        matches!(body.get_surface(rim.surface), Some(Surface::Plane { origin, .. }) if (origin.y - Y_MOUTH).abs() < 1e-12)
    );
    assert_eq!(
        rim.rings,
        vec![r.ring],
        "{what}: the rim's one ring is the row's"
    );
    assert!(
        (circle_radius(body, rim.outer) - R_NECK).abs() < 1e-12,
        "{what}: outer rim circle"
    );
    // The ring is where the INNER sphere (R_BELLY - wall) meets the mouth
    // plane: the lift extends the cavity's belly up to the mouth.
    let want = ((R_BELLY - wall).powi(2) - (Y_MOUTH - Y_BELLY_C).powi(2)).sqrt();
    let got = circle_radius(body, r.ring);
    assert!(
        (got - want).abs() < 1e-12,
        "{what}: ring radius {got}, want inner-sphere-at-mouth {want}"
    );
    // The belly twin: a sphere of radius R_BELLY - wall.
    let twin = body
        .get_face(r.belly_twin)
        .unwrap_or_else(|| panic!("{what}: twin resolves"));
    match body.get_surface(twin.surface) {
        Some(Surface::Sphere { radius, .. }) => assert!(
            (radius - (R_BELLY - wall)).abs() < 1e-12,
            "{what}: twin radius {radius}"
        ),
        other => panic!("{what}: twin is {other:?}"),
    }
    assert_eq!(body.shells().count(), 1, "{what}: opened pot is one shell");
}

#[test]
fn the_teapots_mouth_rim_ring_and_belly_twin_from_the_record_alone() {
    let tol = Tol::witness();
    let pot = bellied(tol);
    let mouth = plane_chart_at(&pot, Y_MOUTH);
    assert_eq!(mouth.len(), 2, "the mouth is two half-discs");

    let Shelled { body, naming } =
        pncad::topo::shell_open(&pot, WALL, &mouth, FIT_TOL, tol).expect("opens");
    let a = roles(&pot, &mouth, &naming);
    check_roles("wall 1/128", &body, &a, WALL);
    println!(
        "[e2e] rim {:?} ring {:?} belly twin {:?}; rims {} dead ({},{},{}) inner {} inner_edges {} ring_edges {}",
        a.rim,
        a.ring,
        a.belly_twin,
        naming.rims.len(),
        naming.dead.faces.len(),
        naming.dead.edges.len(),
        naming.dead.vertices.len(),
        naming.inner.len(),
        naming.inner_edges.len(),
        naming.rims[0].ring_edges.len()
    );

    // Rebuild at another wall: same operand, same designation.
    let wall2 = 1.0 / 256.0;
    let Shelled {
        body: body2,
        naming: naming2,
    } = pncad::topo::shell_open(&pot, wall2, &mouth, FIT_TOL, tol).expect("opens thinner");
    let b = roles(&pot, &mouth, &naming2);
    check_roles("wall 1/256", &body2, &b, wall2);
    assert_eq!(
        (a.rim, a.ring, a.belly_twin),
        (b.rim, b.ring, b.belly_twin),
        "the same rows resolve to the same keys at the other wall"
    );
    // And the whole record is key-identical: thickness changes geometry, not the construction.
    assert_eq!(
        format!("{naming:?}"),
        format!("{naming2:?}"),
        "record identical across walls"
    );

    // The ring rows reach the OPERAND's mouth boundary: each source is an edge of the mouth chart.
    let rimrow = &naming.rims[0];
    for &(_, src) in &rimrow.ring_edges {
        let on_mouth = mouth.iter().any(|&f| {
            let d = pot.get_face(f).unwrap();
            let LoopBoundary::Cycle { first } = pot.get_loop(d.outer).unwrap().boundary else {
                return false;
            };
            pot.loop_cycle(first)
                .unwrap()
                .iter()
                .any(|&he| pot.get_half_edge(he).unwrap().edge == src)
        });
        assert!(
            on_mouth,
            "ring edge source {src:?} bounds the mouth in the operand"
        );
    }
}
