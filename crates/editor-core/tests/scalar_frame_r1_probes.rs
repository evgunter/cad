//! **The frame witness, exercised from outside geom-core** — the
//! aiming ladders against the arithmetic they replaced, a tube built
//! the way a user builds one, the sketch-plane door's repair of a
//! skewed pair, and a frame datum carried whole through the document.
//!
//! These rows came out of a review of the type and were kept as
//! ordinary suite rows, trimmed to what nothing else asserts:
//! `geom-core`'s own module tests pin the mints' columns and refusals,
//! so what is here is the layers above them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{CancelToken, DatumValue, EvalOptions, ProfileDoc, ValuePayload, evaluate};
use geom_core::linalg::frame::{path_start_frame, point_at};
use geom_core::{Affine3, Band, Mat3, OrthoFrame, Point3, Tol, Vec3};
use profile::SketchPlane;
use sweep::{TubeWindow, tube_along_arc, tube_along_arc_hollow};
use topo::Surface;

const SITE: &str = "scalar_frame_r1_probe";

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn bits3(v: Vec3<f64>) -> [u64; 3] {
    [v.x, v.y, v.z].map(f64::to_bits)
}

fn bits12(a: &Affine3<f64>) -> [u64; 12] {
    let l = a.linear;
    [
        l.c0.x,
        l.c0.y,
        l.c0.z,
        l.c1.x,
        l.c1.y,
        l.c1.z,
        l.c2.x,
        l.c2.y,
        l.c2.z,
        a.translation.x,
        a.translation.y,
        a.translation.z,
    ]
    .map(f64::to_bits)
}

// ---------------------------------------------------------------
// The aiming ladders against the arithmetic they replaced
// ---------------------------------------------------------------

/// The retired aiming frame constructor's arithmetic, spelled out:
/// `x = perp / |perp|`, `y = aim × x`, columns `(x, y, aim)`,
/// translation `origin − O`.
fn old_recipe(origin: Point3<f64>, aim: Vec3<f64>, reference: Vec3<f64>) -> Affine3<f64> {
    let unit = aim / aim.norm();
    let perp = reference.cross(unit);
    let len = perp.norm();
    let x = perp / len;
    let y = unit.cross(x);
    Affine3::from_parts(Mat3::from_cols(x, y, unit), origin - Point3::origin())
}

/// **The two aiming doors return the retired recipe bit for bit.**
/// Both are now `OrthoFrame::from_aim` plus `to_affine`, and the claim
/// that costs nothing is that the twelve stored components did not
/// move: the spelling changed, the arithmetic did not. Asserted from
/// OUTSIDE `geom-core`, over a sweep of eyes, targets, references and
/// tangents, so it is the public doors' bits and not an internal one's.
#[test]
fn point_at_and_path_start_frame_are_the_retired_recipe_bit_for_bit() {
    for k in 0..200 {
        let t = f64::from(k) * 0.37 + 0.11;
        let eye = Point3::new(t.sin() * 3.0, -t, t * t * 0.01);
        let target = Point3::new(t.cos(), t.sin() * 2.0, 1.0 + 0.5 * t.cos());
        let reference = Vec3::new(0.3 * t.cos(), 1.0, 0.2 * t.sin());
        let got = point_at(eye, target, reference, Tol::witness()).unwrap();
        let want = old_recipe(eye, target - eye, reference);
        assert_eq!(bits12(&got), bits12(&want), "point_at at k={k}");
        let tangent = Vec3::new(t.cos(), t.sin(), 0.3 * t);
        let got = path_start_frame(eye, tangent, Tol::witness()).unwrap();
        // The ladder's first rung is world +Z; for these tangents it
        // is always off the tangent line.
        let want = old_recipe(eye, tangent, Vec3::unit_z());
        assert_eq!(bits12(&got), bits12(&want), "path_start_frame at k={k}");
    }
}

// ---------------------------------------------------------------
// The tube door, as a user writes it
// ---------------------------------------------------------------

/// **A tube and a hollow tube from a frame a user minted**, with the
/// stored torus axis and reference radial the frame's `w` and `u`
/// VERBATIM — the door's "it stores what it was given" posture, read
/// off the built body rather than off the door's prose.
///
/// The reference here is neither unit nor perpendicular. The retired
/// door refused both of those (`NonUnitURef`, `FrameNotOrthogonal`);
/// the mint projects and normalizes instead, and the frame that comes
/// out is the exact `(x̂, ŷ)` pair the geometry deserves.
#[test]
fn a_tube_from_a_minted_frame_stores_the_frames_axes_verbatim() {
    let frame = OrthoFrame::from_axis_and_reference(
        Point3::origin(),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(3.0, 1.0, 0.0),
        SITE,
        band(),
    )
    .unwrap();
    assert_eq!(bits3(frame.w().get()), bits3(Vec3::unit_y()));
    assert_eq!(bits3(frame.u().get()), bits3(Vec3::unit_x()));
    let solid = tube_along_arc::<f64>(frame, 2.0, TubeWindow::Full, 0.5, Tol::witness())
        .expect("the solid ring builds");
    let hollow = tube_along_arc_hollow::<f64>(
        frame,
        2.0,
        TubeWindow::Arc { t0: 0.25, t1: 1.75 },
        0.5,
        0.125,
        Tol::witness(),
    )
    .expect("the hollow elbow builds");
    for (what, body) in [("solid", &solid.body), ("hollow", &hollow.body)] {
        let mut tori = 0;
        for (_, f) in body.faces() {
            if let Some(Surface::Torus { axis: a, u_ref, .. }) = body.get_surface(f.surface) {
                tori += 1;
                assert_eq!(
                    bits3(*a),
                    bits3(frame.w().get()),
                    "{what}: the torus axis is w"
                );
                // `u_ref` for the full ring is the frame's `u`
                // verbatim; a window rotates it by `t0` first.
                if what == "solid" {
                    assert_eq!(bits3(*u_ref), bits3(frame.u().get()), "{what}: u_ref is u");
                }
            }
        }
        assert!(tori > 0, "{what}: no torus face found");
    }
    let vol = topo::mass_properties(&solid.body, Tol::witness())
        .unwrap()
        .volume;
    let pappus = 2.0 * core::f64::consts::PI.powi(2) * 2.0 * 0.25;
    assert!(((vol - pappus) / pappus).abs() < 1e-12, "{vol} vs {pappus}");
}

// ---------------------------------------------------------------
// Sketch planes: exact, and skewed
// ---------------------------------------------------------------

/// **A skewed pair does NOT refuse at the plane door — it is
/// orthonormalized**, so the plane a user gets back is not the pair
/// they wrote. The guide and the Python docstring say so, and this is
/// where the behaviour is pinned: `u` normalized and kept, `v` yielding
/// its component along it, over a pair leaning 45°. An exact world
/// frame is the case where the two coincide.
///
/// (A pair spanning NO plane refuses; that refusal is pinned at the
/// mint in `geom-core` and is not restated here.)
#[test]
fn a_skewed_pair_is_repaired_at_the_sketch_plane_door() {
    let exact = SketchPlane::from_frame(OrthoFrame::axes_zx(Point3::new(1.0, 2.0, 3.0)));
    assert_eq!(bits3(exact.normal()), bits3(Vec3::unit_y()));
    let skewed = OrthoFrame::gram_schmidt(
        Point3::origin(),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 0.0),
        SITE,
        band(),
    )
    .expect("a skewed pair spans a plane and builds");
    assert_eq!(bits3(skewed.u().get()), bits3(Vec3::unit_x()));
    assert_eq!(bits3(skewed.v().get()), bits3(Vec3::unit_y()));
    let plane = SketchPlane::from_frame(skewed);
    assert_eq!(bits3(plane.normal()), bits3(Vec3::unit_z()));
}

// ---------------------------------------------------------------
// A datum frame through the document
// ---------------------------------------------------------------

/// **A frame datum evaluates to the witness, and the fixture's plane
/// is the evaluator's** — the two roads to a sketch plane off a frame
/// node agree bit for bit, which is what carrying the witness through
/// `DatumValue::Frame` buys. The node's literals are deliberately
/// neither unit nor perpendicular.
#[test]
fn a_datum_frame_node_evaluates_to_the_witness_and_the_fixture_plane_agrees() {
    let doc = ProfileDoc::empty_derived("scalar_frame_r1", Tol::witness());
    let (doc, plane) = fixture::insert(
        doc,
        fixture::frame([1.0, 2.0, 3.0], [2.0, 0.0, 0.0], [1.0, 1.0, 0.0]),
    );
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let ValuePayload::Datum(DatumValue::Frame(f)) = &ev.value(plane).expect("evaluated").payload
    else {
        panic!("not a frame")
    };
    assert_eq!(bits3(f.u().get()), bits3(Vec3::unit_x()));
    assert_eq!(bits3(f.v().get()), bits3(Vec3::unit_y()));
    assert_eq!(bits3(f.w().get()), bits3(Vec3::unit_z()));
    let fixture_plane = fixture::plane_of(&doc, plane);
    assert_eq!(
        bits12(&fixture_plane.placement),
        bits12(&SketchPlane::from_frame(*f).placement)
    );
}
