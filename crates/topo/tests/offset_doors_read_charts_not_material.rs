//! **The offset doors move charts, not material.** `replace_face_offset`
//! and `offset_planes_together` take construction state (`&mut Body`)
//! and read `d` along each chart's stored normal; neither reads the
//! solid's sense. On the wedge prism wound either way the chart normals
//! are the loop's, so the top face's signed volume change is `+A·d` in
//! both windings, and the inside-out operand and its result alike are
//! refused `NegativeVolume` where a body becomes finished
//! (`AtRestBody::validate`). The axial door's row is
//! `crates/sweep/tests/offset_axial_door_reads_charts_not_material.rs`,
//! where a body of revolution can be built.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom::Surface;
use geom_core::{Band, Tol};
use topo::{AtRestBody, Body, ChartMove, FaceKey, ValidationError, mass_properties};

/// The triangle (0,0), 80°, 190° on the unit circle, counterclockwise
/// when `ccw` (`inside_out_operand.rs`'s wedge).
fn wedge_profile(ccw: bool) -> [(f64, f64); 3] {
    let at = |deg: f64| (deg.to_radians().cos(), deg.to_radians().sin());
    if ccw {
        [(0.0, 0.0), at(80.0), at(190.0)]
    } else {
        [(0.0, 0.0), at(190.0), at(80.0)]
    }
}

/// The wedge prism over z ∈ (0.5, 1).
fn wedge(ccw: bool) -> Body<f64> {
    common::prism_z::<f64>(&wedge_profile(ccw), 0.5, 1.0, Tol::witness()).body
}

/// The profile's area, `½·sin 110°`.
fn cap_area() -> f64 {
    0.5 * 110f64.to_radians().sin()
}

/// The wedge's face on the plane z = 1, whichever way its chart faces.
fn top(body: &Body<f64>) -> FaceKey {
    let mut hits = body.faces().filter(|(_, f)| {
        matches!(
            body.get_surface(f.surface),
            Some(Surface::Plane { origin, normal, .. })
                if normal.z.abs() == 1.0 && origin.z == 1.0
        )
    });
    let (k, _) = hits.next().expect("the wedge has a face on z = 1");
    assert!(hits.next().is_none(), "one face on z = 1");
    k
}

/// The top chart's stored normal's z.
fn top_normal_z(body: &Body<f64>) -> f64 {
    let f = body.get_face(top(body)).unwrap();
    match body.get_surface(f.surface) {
        Some(Surface::Plane { normal, .. }) => normal.z,
        other => panic!("the top chart is a plane, got {other:?}"),
    }
}

fn volume(body: &Body<f64>) -> f64 {
    mass_properties(body, Tol::witness()).unwrap().volume
}

/// `body` finishes when `ccw`, and refuses on check 7 alone when not.
fn finishes_iff(what: &str, body: &Body<f64>, ccw: bool) {
    match AtRestBody::validate(body.clone(), Tol::witness()) {
        Ok(_) => assert!(ccw, "{what}: the inside-out body finished"),
        Err(errors) => assert!(
            !ccw && errors.len() == 1
                && matches!(errors[0], ValidationError::NegativeVolume { .. }),
            "{what}: want NegativeVolume alone on the inside-out body only, got {errors:?}"
        ),
    }
}

/// Every chart of `body`, the top moved by `d` and the rest by zero.
fn top_move(body: &Body<f64>, d: f64) -> Vec<ChartMove<f64>> {
    let top = top(body);
    let mut by_surface: std::collections::BTreeMap<_, Vec<FaceKey>> = Default::default();
    for (k, f) in body.faces() {
        by_surface.entry(f.surface).or_default().push(k);
    }
    by_surface
        .into_values()
        .map(|faces| ChartMove {
            distance: if faces.contains(&top) { d } else { 0.0 },
            faces,
        })
        .collect()
}

/// One door, both windings: the top chart moves by `d` along its
/// stored normal — up on the counterclockwise wedge, down on the
/// clockwise one, whose chart faces −z — so the signed volume changes
/// by `+A·d` in each, and the operand and the result finish exactly
/// when the winding is counterclockwise.
fn moves_by_chart(door: &str, offset: impl Fn(&mut Body<f64>, f64)) {
    let d = 0.1;
    for ccw in [true, false] {
        let what = format!("{door}, ccw = {ccw}");
        let mut body = wedge(ccw);
        assert_eq!(
            top_normal_z(&body),
            if ccw { 1.0 } else { -1.0 },
            "{what}: the top chart's normal is the loop's"
        );
        let before = volume(&body);
        let want_before = if ccw { 0.5 } else { -0.5 } * cap_area();
        assert!(
            (before - want_before).abs() < 1e-12,
            "{what}: signed volume {before}, want {want_before}"
        );
        finishes_iff(&format!("{what}, the operand"), &body, ccw);
        offset(&mut body, d);
        let delta = volume(&body) - before;
        assert!(
            (delta - cap_area() * d).abs() < 1e-12,
            "{what}: signed ΔV {delta}, want +A·d = {}",
            cap_area() * d
        );
        finishes_iff(&format!("{what}, the result"), &body, ccw);
    }
}

#[test]
fn replace_face_offset_moves_the_chart_in_either_winding() {
    moves_by_chart("replace_face_offset", |body, d| {
        let face = top(body);
        topo::replace_face_offset(body, face, d, Tol::witness()).expect("the top face offsets");
    });
}

#[test]
fn offset_planes_together_moves_the_chart_in_either_winding() {
    moves_by_chart("offset_planes_together", |body, d| {
        let moves = top_move(body, d);
        topo::offset_planes_together(
            body,
            &moves,
            Band::linear(Tol::witness()).unwrap(),
            Tol::witness(),
        )
        .expect("the top chart offsets");
    });
}
