//! **R1 review probes for FRAME-WITNESS (PR 2675)** — lane
//! `scalar-frame-r1`, at the frozen head `c1d8a7ffe`.
//!
//! Each row is either a falsification of a claim in the PR body or an
//! end-to-end exercise a user would write. The rows that document a
//! defect are written to be GREEN on the head: they assert the defect
//! exists, so a fix pass sees them go red and knows what it closed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, DatumValue, EvalOptions, Node, ProfileDoc, ValuePayload, evaluate,
};
use geom_core::linalg::frame::{path_start_frame, point_at};
use geom_core::{
    Affine3, Band, Mat3, OrthoAxis, OrthoFrame, OrthoFrameError, Point3, Tol, UnitVec3,
    UnitVec3Error, Vec3,
};
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
// 1. The type's claim vs the public `from_aim` door
// ---------------------------------------------------------------

/// **`from_aim` is a public "trust me" door for perpendicularity.**
/// The type's doc says a frame "cannot be non-orthonormal" and that
/// `v` is orthogonal to `u` "never by a caller asserting that two
/// vectors it held happened to be perpendicular" — but `from_aim`'s
/// only guard on `perp_raw ⊥ aim` is its doc comment. Hand it an
/// offset that is NOT perpendicular and the witness carries a `v`
/// that is not unit and a `u` that is not orthogonal to `w`.
#[test]
fn from_aim_mints_a_non_orthonormal_frame_from_a_non_perpendicular_offset() {
    let aim = UnitVec3::new(Vec3::unit_z(), SITE, band()).unwrap();
    // 45° off the aim line — perfectly decidable, just not perpendicular.
    let f = OrthoFrame::from_aim(Point3::origin(), aim, Vec3::new(1.0, 0.0, 1.0), SITE, band())
        .expect("the offset's length decides positive, so the door builds");
    let (u, v, w) = (f.u().get(), f.v().get(), f.w().get());
    let v_norm = v.norm();
    let u_dot_w = u.dot(w);
    assert!(
        (v_norm - 1.0).abs() > 0.25,
        "|v| = {v_norm}: the frame's second axis is NOT unit"
    );
    assert!(
        u_dot_w.abs() > 0.5,
        "u·w = {u_dot_w}: the frame's first and third axes are NOT orthogonal"
    );
    // And it converts into a placement whose linear part is not rigid.
    let det = f.to_affine().linear.determinant();
    assert!((det - 1.0).abs() > 0.25, "det = {det}: the placement is a skew map");
    // Whereas the residual door repairs the same input.
    let g = OrthoFrame::from_aim_and_reference(
        Point3::origin(),
        aim,
        Vec3::new(1.0, 0.0, 1.0),
        SITE,
        band(),
    )
    .unwrap();
    assert!((g.v().get().norm() - 1.0).abs() < 1e-15);
    assert!(g.u().get().dot(g.w().get()).abs() < 1e-15);
}

// ---------------------------------------------------------------
// 2. Bit identity of the aiming ladders against the retired recipe
// ---------------------------------------------------------------

/// The retired `frame_from_unit_aim` arithmetic, spelled out:
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

#[test]
fn point_at_and_path_start_frame_are_the_retired_recipe_bit_for_bit() {
    let mut n = 0;
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
        n += 1;
    }
    assert_eq!(n, 200);
}

// ---------------------------------------------------------------
// 3. The tube door, as a user writes it
// ---------------------------------------------------------------

/// **A tube and a hollow tube from a hand-minted frame.** What the
/// user pays: one `Band`, one `UnitVec3::new` (with a K funnel name
/// they have to invent), one `from_aim_and_reference` (the same name
/// again), then the door. The stored torus axis and reference radial
/// are the frame's `w` and `u` verbatim.
#[test]
fn a_tube_from_a_hand_minted_frame_stores_the_frames_axes_verbatim() {
    let band = band();
    let axis_raw = Vec3::new(0.0, 2.0, 0.0);
    let axis = UnitVec3::new(axis_raw, SITE, band).unwrap();
    // A reference that is neither unit nor perpendicular: the old door
    // refused both (`NonUnitURef` / `FrameNotOrthogonal`); the mint
    // projects and normalizes.
    let frame =
        OrthoFrame::from_aim_and_reference(Point3::origin(), axis, Vec3::new(3.0, 1.0, 0.0), SITE, band)
            .unwrap();
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
                assert_eq!(bits3(*a), bits3(frame.w().get()), "{what}: the torus axis is w");
                // `u_ref` for the full ring is the frame's `u` verbatim.
                if what == "solid" {
                    assert_eq!(bits3(*u_ref), bits3(frame.u().get()), "{what}: u_ref is u");
                }
            }
        }
        assert!(tori > 0, "{what}: no torus face found");
    }
    assert_eq!(bits3(frame.w().get()), bits3(Vec3::unit_y()));
    assert_eq!(bits3(frame.u().get()), bits3(Vec3::unit_x()));
    let vol = topo::mass_properties(&solid.body, Tol::witness()).unwrap().volume;
    let pappus = 2.0 * core::f64::consts::PI.powi(2) * 2.0 * 0.25;
    assert!(((vol - pappus) / pappus).abs() < 1e-12, "{vol} vs {pappus}");
}

// ---------------------------------------------------------------
// 4. Sketch planes: exact, skewed, parallel
// ---------------------------------------------------------------

/// A skewed pair does NOT refuse — it is silently orthonormalized, so
/// the plane a user gets is not the plane they wrote. Only a pair
/// spanning no plane refuses.
#[test]
fn a_skewed_pair_is_repaired_and_a_parallel_pair_refuses() {
    let exact = SketchPlane::from_frame(OrthoFrame::axes_zx(Point3::new(1.0, 2.0, 3.0)));
    assert_eq!(bits3(exact.normal()), bits3(Vec3::unit_y()));
    let skewed = OrthoFrame::gram_schmidt(
        Point3::origin(),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 0.0),
        SITE,
        SITE,
        band(),
    )
    .expect("a skewed pair spans a plane and builds");
    assert_eq!(bits3(skewed.u().get()), bits3(Vec3::unit_x()));
    assert_eq!(bits3(skewed.v().get()), bits3(Vec3::unit_y()));
    let parallel = OrthoFrame::gram_schmidt(
        Point3::origin(),
        Vec3::unit_x(),
        Vec3::new(-3.0, 0.0, 0.0),
        SITE,
        SITE,
        band(),
    )
    .unwrap_err();
    assert_eq!(
        parallel,
        OrthoFrameError {
            axis: OrthoAxis::V,
            error: UnitVec3Error::Degenerate
        }
    );
}

/// **The mint re-normalizes what a caller already normalized**, and a
/// second `normalize` is not the identity: over 10 000 directions
/// normalized once by hand, count how many move under the mint.
#[test]
fn a_second_normalize_moves_the_last_bit_on_a_fraction_of_directions() {
    let mut moved = 0;
    let n = 10_000;
    for k in 0..n {
        let t = f64::from(k) * 0.001_37 + 0.05;
        let u_hand = Vec3::new(t.cos() * 1.7, t.sin() * 0.3, 0.9 * (2.0 * t).sin()).normalize();
        let f = OrthoFrame::gram_schmidt(Point3::origin(), u_hand, Vec3::unit_z(), SITE, SITE, band())
            .unwrap();
        if bits3(f.u().get()) != bits3(u_hand) {
            moved += 1;
        }
    }
    eprintln!("second normalize moved {moved} of {n} hand-normalized directions");
    assert!(moved > 0, "a second normalize must move at least one last bit");
}

// ---------------------------------------------------------------
// 5. A datum frame through the document
// ---------------------------------------------------------------

#[test]
fn a_datum_frame_node_evaluates_to_the_witness_and_the_fixture_plane_agrees() {
    let doc = ProfileDoc::empty_derived("scalar_frame_r1", Tol::witness());
    // Deliberately non-unit, non-perpendicular literals.
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
    let _ = Node::<editor_core::ProfileProgram>::Datum;
}
