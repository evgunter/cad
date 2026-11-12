//! **A loop's lift runs on through a pole as its face's boundary.**
//!
//! A joint on a chart's singular set stores the turn there on the
//! face's side (`topo::joint`), so the chain a loop's stored rows draw
//! is the face's region boundary in the chart, a pole a side of it. The
//! fixture is the unit sphere about `z` cut by two meridians `WIDTH`
//! apart, joined at both poles: a lune and its complement, the same two
//! edges bounding both.
//!
//! - **The lune and its complement get different elements**: their
//!   windows are `WIDTH` and `τ − WIDTH` wide, and the joints the two
//!   loops make at the north pole store different turns.
//! - **A kill's sum at a pole equals a fresh decision**: a strut from
//!   the pole splits a face's turn there in two, `kev` sums them back to
//!   the turn it split, and tier 3, which decides every stored element
//!   again, reads the sum clean; `kef` of a meridian sums the two faces'
//!   turns into the whole turn of a sphere slit pole to pole.
//! - **The stored lift bounds the face's own region**: on every sphere
//!   face of the fixtures whose loop passes a pole, the chain's
//!   equal-area shoelace, turned by the face's sense, lies in `(0, 4π)`:
//!   no run between two poles sits a whole turn off.
//! - **A move to the opposite side decides again**: a face's rows go
//!   with its bit, the re-minted loop reads the complement, and
//!   `kfmrh_minting` back onto a face of the first sense decides the
//!   turns the loop first had.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{ball_poled_y, revolved_about_y_at};
use topo::construction::{Construct, FaceSurface, MefSite, MevSite};
use topo::{Body, FaceKey, HalfEdgeKey, JointElement};

/// The lune's width in azimuth.
const WIDTH: f64 = 0.8;

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

fn z() -> Vec3<f64> {
    Vec3::new(0.0, 0.0, 1.0)
}

/// The equator direction at azimuth `phi`.
fn out(phi: f64) -> Vec3<f64> {
    Vec3::new(phi.cos(), phi.sin(), 0.0)
}

/// The meridian at azimuth `phi` from the pole `from` (`±ẑ`):
/// `eval(t) = cos t · from + sin t · out(phi)`, the other pole at `π`.
fn meridian(from: Vec3<f64>, phi: f64) -> Curve3<f64> {
    Curve3::circle(Point3::origin(), from.cross(out(phi)), 1.0, from)
}

fn arc(curve: Curve3<f64>, to: f64) -> EdgeCurveSpec<f64> {
    EdgeCurveSpec::arc_of_circle(curve, 0.0, to).unwrap()
}

/// The unit sphere about `z` cut by the meridians at azimuth 0 and
/// `WIDTH`: its two faces, both outward, each with the half-edge its
/// loop leaves the north pole on.
struct Lune {
    body: Body<f64>,
    faces: [(FaceKey, HalfEdgeKey); 2],
}

fn lune() -> Lune {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(Point3::origin() + z(), true).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: Surface::Sphere {
                center: Point3::origin(),
                radius: 1.0,
                axis: z(),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            sense: true,
        },
    )
    .unwrap();
    let down = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            Point3::origin() - z(),
            arc(meridian(z(), 0.0), PI),
            tol(),
        )
        .unwrap();
    // The chord from the south pole back to the north.
    let up = body
        .mef(
            MefSite::Chords {
                he1: down.he_minus,
                he2: down.he_plus,
            },
            arc(meridian(z() * -1.0, WIDTH), PI),
            FaceSurface::Inherit,
            tol(),
        )
        .unwrap();
    topo::mint_pcurves(&mut body, tol()).unwrap();
    let from_north = |body: &Body<f64>, face: FaceKey| {
        let outer = body.get_face(face).unwrap().outer;
        body.loop_lift(outer)
            .unwrap()
            .rows()
            .iter()
            .map(|row| row.half_edge)
            .find(|&he| {
                let start = body.get_half_edge(he).unwrap().start;
                body.vertex_points()
                    .any(|(v, p)| v == start && (p.z - 1.0).abs() < 1e-12)
            })
            .expect("each loop leaves the north pole once")
    };
    let faces = [seed.face, up.face].map(|face| (face, from_north(&body, face)));
    Lune { body, faces }
}

/// The width of `face`'s azimuth window, off its outer loop's stored
/// lift.
fn width(body: &Body<f64>, face: FaceKey) -> f64 {
    let outer = body.get_face(face).unwrap().outer;
    let (lo, hi) = body
        .loop_lift(outer)
        .unwrap()
        .azimuth_window()
        .expect("meridians have closed-form images");
    hi - lo
}

fn clean(body: &Body<f64>, what: &str) {
    assert_eq!(
        topo::pcurves::validate_pcurves(body, band()),
        vec![],
        "{what}: tier 3 decides every stored element again and reads it clean"
    );
}

#[test]
fn the_lune_and_its_complement_get_different_elements() {
    let Lune { body, faces } = lune();
    clean(&body, "the lune and its complement");
    let mut widths = faces.map(|(face, _)| width(&body, face));
    widths.sort_by(f64::total_cmp);
    assert!(
        (widths[0] - WIDTH).abs() < 1e-12 && (widths[1] - (TAU - WIDTH)).abs() < 1e-12,
        "the two windows are the lune's and its complement's: {widths:?}"
    );
    let turns = faces.map(|(_, he)| body.joint(he).unwrap());
    assert_ne!(
        turns[0], turns[1],
        "the two loops turn at the north pole on different sides"
    );
    for (face, _) in faces {
        let outer = body.get_face(face).unwrap().outer;
        assert!(
            body.loop_lift(outer).unwrap().closes_in_chart(),
            "each loop closes in the chart, its poles two sides of its region"
        );
    }
}

#[test]
fn a_kills_sum_at_a_pole_equals_a_fresh_decision() {
    let Lune { mut body, faces } = lune();
    // The wider face: a strut from the north pole down its middle.
    let (wide, leaving) = faces
        .into_iter()
        .max_by(|a, b| width(&body, a.0).total_cmp(&width(&body, b.0)))
        .unwrap();
    assert!(width(&body, wide) > PI, "the complement is the wider face");
    let before = body.joint(leaving).unwrap();
    let strut = body
        .mev(
            MevSite::Fan {
                he1: leaving,
                he2: leaving,
            },
            meridian(z(), 3.5).eval(1.0),
            arc(meridian(z(), 3.5), 1.0),
            tol(),
        )
        .unwrap();
    clean(&body, "the strut from the pole");
    let (into_strut, onward) = (
        body.joint(strut.he_plus).unwrap(),
        body.joint(leaving).unwrap(),
    );
    assert!(
        into_strut != before && onward != before,
        "the strut takes part of the pole's turn: {into_strut:?} and {onward:?} of {before:?}"
    );
    assert_eq!(
        into_strut
            .then(body.joint(strut.he_minus).unwrap())
            .then(onward),
        before,
        "the turns either side of the strut sum to the turn it split"
    );
    body.kev(strut.he_plus).unwrap();
    assert_eq!(
        body.joint(leaving),
        Some(before),
        "the kill writes the sum, the turn the pole joint held before the strut"
    );
    clean(&body, "the strut killed");

    // The meridian between the two faces killed: one face, the sphere
    // slit pole to pole, whose loop leaves each pole along the edge it
    // arrived on.
    let Lune { mut body, faces } = lune();
    let shared = body.get_half_edge(faces[1].1).unwrap().edge;
    let half = body.get_edge(shared).unwrap().he_plus;
    body.kef(half).unwrap();
    clean(&body, "the sphere slit pole to pole");
    let (face, _) = body.faces().next().unwrap();
    assert_eq!(body.faces().count(), 1);
    let outer = body.get_face(face).unwrap().outer;
    let lift = body.loop_lift(outer).unwrap();
    let whole_turns: Vec<i32> = lift
        .rows()
        .iter()
        .map(|row| body.joint(row.half_edge).unwrap().deck().u)
        .collect();
    assert!(
        whole_turns.len() == 2 && whole_turns[0] == -whole_turns[1] && whole_turns[0].abs() == 1,
        "the two faces' turns at each pole sum to one whole turn, opposite at the two poles: \
         {whole_turns:?}"
    );
    assert!(
        lift.closes_in_chart() && (width(&body, face) - TAU).abs() < 1e-12,
        "the slit sphere's chain is the chart's whole rectangle"
    );
}

/// The equal-area shoelace of `face`'s outer loop's stored lift, turned
/// by the face's sense, and whether the loop passes a pole. `None` for a
/// loop that does not close in the chart (it winds, and bounds no
/// polygon).
fn region_area(body: &Body<f64>, face: FaceKey) -> Option<(f64, bool)> {
    let record = body.get_face(face).unwrap();
    let lift = body.loop_lift(record.outer).unwrap();
    if !lift.closes_in_chart() {
        return None;
    }
    let polygon: Vec<Point2<f64>> = lift
        .rows()
        .iter()
        .flat_map(|row| {
            let (entry, exit) = row.entry_exit();
            (0..=64).map(move |k| row.pcurve.eval(entry + (exit - entry) * f64::from(k) / 64.0))
        })
        .collect();
    let n = polygon.len();
    let area = 0.5
        * (0..n)
            .map(|i| {
                let (a, b) = (polygon[i], polygon[(i + 1) % n]);
                a.x * b.y.sin() - b.x * a.y.sin()
            })
            .sum::<f64>();
    let polar = polygon.iter().any(|p| (PI / 2.0 - p.y.abs()) < 1e-12);
    Some((if record.sense { area } else { -area }, polar))
}

#[test]
fn the_stored_lift_bounds_the_faces_own_region_through_its_poles() {
    let wedge = |theta: f64| {
        revolved_about_y_at(
            vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
            Revolution::Partial(theta),
            tol(),
        )
    };
    let fixtures: Vec<(&str, Body<f64>)> = vec![
        ("the lune", lune().body),
        ("the ball", ball_poled_y(1.0, Vec3::new(0.0, 0.0, 0.0), tol())),
        ("the narrow wedge", wedge(0.7)),
        ("the wide wedge", wedge(4.0)),
    ];
    let mut polar = 0;
    for (name, body) in &fixtures {
        for (face, _) in body.faces() {
            if !matches!(body.face_surface(face), Some(Surface::Sphere { .. })) {
                continue;
            }
            let Some((area, at_pole)) = region_area(body, face) else {
                continue;
            };
            assert!(
                area > 0.0 && area < 4.0 * PI,
                "{name}, face {face:?}: the chain bounds an area of {area}, not a region of \
                 the sphere on the face's side"
            );
            polar += usize::from(at_pole);
        }
    }
    assert!(polar >= 8, "the fixtures' faces pass their poles: {polar}");
}

#[test]
fn a_move_to_the_opposite_side_decides_the_turns_again() {
    let Lune { mut body, faces } = lune();
    let [(narrow, from_north), (other, _)] = {
        let mut by_width = faces;
        by_width.sort_by(|a, b| width(&body, a.0).total_cmp(&width(&body, b.0)));
        by_width
    };
    let first: JointElement = body.joint(from_north).unwrap();

    // The bit flipped in place: the rows are about the other side of
    // the loop, and go with it.
    body.set_face_sense(narrow, false).unwrap();
    assert!(
        body.pcurve(from_north).is_none() && body.joint(from_north).is_none(),
        "a flipped face on a chart with poles keeps no row of its old side"
    );
    topo::mint_pcurves_of(&mut body, &[narrow], tol()).unwrap();
    let flipped = body.joint(from_north).unwrap();
    assert_ne!(flipped, first, "the turn at the pole is the other side's");
    assert!(
        (width(&body, narrow) - (TAU - WIDTH)).abs() < 1e-12,
        "read from its other side, the lune's loop bounds the complement"
    );
    clean(&body, "the flipped lune, minted again");

    // The loop moved onto a face of the first sense, with a band: the
    // door decides the turns the loop first had.
    let mut moved = body.clone();
    moved.kfmrh_minting(other, narrow, tol()).unwrap();
    assert_eq!(
        moved.joint(from_north),
        Some(first),
        "moved back to a face of the first sense, the pole joint holds its first turn"
    );

    // With no band the door decides nothing: it refuses, or carries no
    // row of the old side across.
    let mut keyed = body.clone();
    if keyed.kfmrh(other, narrow).is_ok() {
        assert_ne!(
            keyed.joint(from_north),
            Some(flipped),
            "the keys-only door carries no turn of the old side onto the new face"
        );
    }
}
