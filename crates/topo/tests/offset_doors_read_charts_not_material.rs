//! **The offset doors move charts, not material.** `replace_face_offset`
//! and `offset_planes_together` take construction state (`&mut Body`)
//! and read `d` along each chart's stored normal; no face's sense
//! decides the move. The wedge prism's top face moves by `d` along its
//! chart in either winding, on the reverted wedge, and where the face
//! wears its plane reversed, so the signed volume changes by
//! `sense·A·d`. The inside-out operand and its result alike are refused
//! `NegativeVolume` where a body becomes finished
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

/// [`wedge`] with its top face wearing its plane reversed: the chart
/// normal negated and the face's sense `false`, so the face bounds the
/// same material against the opposite chart. Charted before the edges
/// are described, so every description reads the chart it ends on.
fn wedge_top_reversed(ccw: bool) -> Body<f64> {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let ops = common::prism_ops(
        &mut body,
        &wedge_profile(ccw),
        (0.5, 1.0),
        common::identity_map,
        common::FaceGeometry::Certified,
        tol,
    );
    let top = ops.seed.face;
    let outer = body.get_face(top).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("the top face's outer loop is a cycle");
    };
    let mut corners: Vec<_> = body
        .loop_cycle(first)
        .unwrap()
        .iter()
        .map(|&he| {
            let v = body.get_half_edge(he).unwrap().start;
            *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
        })
        .collect();
    assert!(
        corners.iter().all(|p| p.z == 1.0),
        "the seed face is the top"
    );
    corners.reverse();
    body.set_face_surface(
        top,
        topo::FaceSurface::New {
            surface: common::plane(&corners, tol),
            sense: false,
        },
    )
    .unwrap();
    common::describe_as_intersections(&mut body, tol);
    body
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

/// `body` finishes when `right_way`, and refuses on check 7 alone when
/// not.
fn finishes_iff(what: &str, body: &Body<f64>, right_way: bool) {
    match AtRestBody::validate(body.clone(), Tol::witness()) {
        Ok(_) => assert!(right_way, "{what}: the inside-out body finished"),
        Err(errors) => assert!(
            !right_way
                && errors.len() == 1
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

/// One door, five wedges. The top chart moves by `d` along its stored
/// normal whatever the solid's winding and whatever the face's sense,
/// so the signed volume changes by `sense·A·d`: `+A·d` on either
/// winding, and on the reverted wedge, whose planes carry the reversal
/// in their normals; `−A·d` where the top face wears its plane reversed
/// (`sense` false). A door that read the sense would move that face
/// the other way. Each operand and result finishes exactly when its
/// winding is the right way round.
fn moves_by_chart(door: &str, offset: impl Fn(&mut Body<f64>, f64)) {
    let d = 0.1;
    let a_d = cap_area() * d;
    let rows = [
        ("ccw", wedge(true), true, 1.0, true, a_d),
        ("cw", wedge(false), false, -1.0, true, a_d),
        ("ccw reverted", wedge(true).revert(), false, -1.0, true, a_d),
        (
            "ccw, top reversed",
            wedge_top_reversed(true),
            true,
            -1.0,
            false,
            -a_d,
        ),
        (
            "cw, top reversed",
            wedge_top_reversed(false),
            false,
            1.0,
            false,
            -a_d,
        ),
    ];
    for (row, mut body, right_way, normal_z, sense, want) in rows {
        let what = format!("{door}, {row}");
        let f = body.get_face(top(&body)).unwrap();
        assert_eq!(
            (top_normal_z(&body), f.sense),
            (normal_z, sense),
            "{what}: the top chart's normal and the face's sense"
        );
        let before = volume(&body);
        let want_before = if right_way { 0.5 } else { -0.5 } * cap_area();
        assert!(
            (before - want_before).abs() < 1e-12,
            "{what}: signed volume {before}, want {want_before}"
        );
        finishes_iff(&format!("{what}, the operand"), &body, right_way);
        offset(&mut body, d);
        let delta = volume(&body) - before;
        assert!(
            (delta - want).abs() < 1e-12,
            "{what}: signed ΔV {delta}, want {want}"
        );
        finishes_iff(&format!("{what}, the result"), &body, right_way);
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
