//! **The planar cut-off across the shapes of its end and its request**,
//! each built at its closed form through [`carve`] (tier 3, one genus-0
//! shell, naming totality, `ΔV`): end faces leaning along the support
//! and tilted out of the vertical, a dihedral that is not right, and
//! requests that mix patches, cut-offs and shared end faces on one box.
//! The round band at an oblique end face refuses typed throughout.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Point2, Point3, Vec3};
use sweep::blend::BlendError;
use sweep::blend::battery::END_FACE_OBLIQUE;
use sweep::test_support::{block, prism, prism_on, sketch_from_axes};
use topo::{Body, EdgeKey};

use crate::band_planar_cut_off::{D, Verb, carve, edge, the_box, tol};

/// The round band at an end face oblique to its edge: the section there
/// is an ellipse, refused as the run-out it is.
fn fillet_refuses_oblique(body: &Body<f64>, edges: &[EdgeKey], what: &str) {
    match Verb::Fillet.run(body, edges) {
        Err(BlendError::UnsupportedRunOut { detail, .. }) => {
            assert_eq!(detail, END_FACE_OBLIQUE, "{what}");
        }
        other => panic!("{what}: an oblique round end refuses, got {other:?}"),
    }
}

/// **End faces leaning along the support**: a trapezoid prism
/// `(0,0), (2,0), (2−s,1), (s,1)` at slopes on both sides of upright.
/// The band's region is a prism of section `d²/2` cut by two planes, so
/// `ΔV = (d²/2)·L(ȳ)` with `L(y) = 2 − 2sy` at the section's centroid:
/// `ȳ = d/3` for the front edges, `1 − d/3` for the back one. A
/// trapezoid in `xz`, extruded along `y`, leans the end faces the other
/// transverse way.
#[test]
fn end_faces_leaning_along_the_support_cut_the_chamfer_at_the_centroid_length() {
    let half = D * D / 2.0;
    for s in [0.05, 0.3, 0.5, 0.9, -0.3, -1.0, -3.0] {
        let body = prism(
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(2.0, 0.0), 0.0),
                (Point2::new(2.0 - s, 1.0), 0.0),
                (Point2::new(s, 1.0), 0.0),
            ],
            1.0,
            tol(),
        );
        let front = half * (2.0 - 2.0 * s * D / 3.0);
        for z in [1.0, 0.0] {
            let e = edge(&body, [0.0, 0.0, z], [2.0, 0.0, z]);
            carve(
                &body,
                &[e],
                Verb::Chamfer,
                front,
                &format!("trapezoid s = {s}, z = {z}"),
            );
        }
        let top = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
        fillet_refuses_oblique(&body, &[top], &format!("trapezoid s = {s}"));
        let back = edge(&body, [s, 1.0, 1.0], [2.0 - s, 1.0, 1.0]);
        let along_back = half * (2.0 - 2.0 * s * (1.0 - D / 3.0));
        carve(
            &body,
            &[back],
            Verb::Chamfer,
            along_back,
            &format!("trapezoid back s = {s}"),
        );
    }
    for s in [0.3, -0.5] {
        let plane = sketch_from_axes(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            tol(),
        );
        let body = prism_on(
            plane,
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(2.0, 0.0), 0.0),
                (Point2::new(2.0 - s, 1.0), 0.0),
                (Point2::new(s, 1.0), 0.0),
            ],
            1.5,
            tol(),
        );
        let e = edge(&body, [0.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
        let removed = half * (2.0 - 2.0 * s * D / 3.0);
        carve(
            &body,
            &[e],
            Verb::Chamfer,
            removed,
            &format!("xz trapezoid s = {s}"),
        );
    }
}

/// **End faces tilted out of the vertical**: a profile in `(z, x)`
/// extruded along `y`, its end walls leaning inward (`L(z) = 2 − 0.6z`)
/// or outward (`L(z) = 1.4 + 0.6z`). The top front edge's section has
/// its centroid at `z̄ = 1 − d/3`, so `ΔV = (d²/2)·L(z̄)`.
#[test]
fn end_faces_tilted_out_of_the_vertical_cut_the_chamfer_at_the_centroid_length() {
    let plane = sketch_from_axes(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
        tol(),
    );
    for (what, outline, (x0, x1), (l0, l1)) in [
        (
            "tilted in",
            [(0.0, 0.0), (1.0, 0.3), (1.0, 1.7), (0.0, 2.0)],
            (0.3, 1.7),
            (2.0, -0.6),
        ),
        (
            "tilted out",
            [(0.0, 0.3), (1.0, 0.0), (1.0, 2.0), (0.0, 1.7)],
            (0.0, 2.0),
            (1.4, 0.6),
        ),
    ] {
        // Counter-clockwise in the sketch's `(z, x)`.
        let mut ccw: Vec<(Point2<f64>, f64)> = outline
            .iter()
            .map(|&(z, x)| (Point2::new(z, x), 0.0))
            .collect();
        let twice_area: f64 = (0..4)
            .map(|i| {
                let (a, b) = (ccw[i].0, ccw[(i + 1) % 4].0);
                a.x * b.y - b.x * a.y
            })
            .sum();
        if twice_area < 0.0 {
            ccw.reverse();
        }
        let body = prism_on(plane, ccw, 1.0, tol());
        let e = edge(&body, [x0, 0.0, 1.0], [x1, 0.0, 1.0]);
        let zbar = 1.0 - D / 3.0;
        carve(
            &body,
            &[e],
            Verb::Chamfer,
            D * D / 2.0 * (l0 + l1 * zbar),
            what,
        );
        fillet_refuses_oblique(&body, &[e], what);
    }
}

/// **A dihedral that is not right**, its end faces perpendicular: a
/// triangular prism's vertical edge at the angle `α` between its two
/// walls. The chamfer's section is the isosceles triangle of legs `d`,
/// `d²·sin α / 2`; the fillet's the region between the walls and the
/// ball, `r²·(cot(α/2) − (π − α)/2)`.
#[test]
fn a_dihedral_that_is_not_right_cuts_off_at_both_verbs_closed_forms() {
    let body = prism(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(0.7, 1.3), 0.0),
        ],
        1.0,
        tol(),
    );
    let e = edge(&body, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let alpha = 1.3_f64.atan2(0.7);
    carve(
        &body,
        &[e],
        Verb::Chamfer,
        D * D * alpha.sin() / 2.0,
        "a 61.7° chamfer",
    );
    let fillet = D * D * (1.0 / (alpha / 2.0).tan() - (PI - alpha) / 2.0);
    carve(&body, &[e], Verb::Fillet, fillet, "a 61.7° fillet");
}

/// **Patches and cut-offs on one box**, both verbs: two opposite
/// corners' three edges each (two patches, six cut-offs); the four
/// vertical edges, the top and bottom each the end face of four
/// cut-offs; a corner's three edges and the edge parallel to one of
/// them, sharing its far end face; and three edges whose end faces are
/// each other's supports. `ΔV` is the section times the requested
/// length, less a corner term per patch.
#[test]
fn patches_and_cut_offs_mix_on_one_box() {
    let body = the_box();
    let corner_a = [
        edge(&body, [0.0, 0.0, 0.0], [2.0, 0.0, 0.0]),
        edge(&body, [0.0, 0.0, 0.0], [0.0, 1.5, 0.0]),
        edge(&body, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
    ];
    let corner_b = [
        edge(&body, [2.0, 1.5, 1.0], [0.0, 1.5, 1.0]),
        edge(&body, [2.0, 1.5, 1.0], [2.0, 0.0, 1.0]),
        edge(&body, [2.0, 1.5, 1.0], [2.0, 1.5, 0.0]),
    ];
    let verticals: Vec<EdgeKey> = [[0.0, 0.0], [2.0, 0.0], [2.0, 1.5], [0.0, 1.5]]
        .iter()
        .map(|[x, y]| edge(&body, [*x, *y, 0.0], [*x, *y, 1.0]))
        .collect();
    let mut corner_and_parallel = corner_a.to_vec();
    corner_and_parallel.push(edge(&body, [0.0, 1.5, 1.0], [2.0, 1.5, 1.0]));
    let shared_end_faces = [
        edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]),
        edge(&body, [2.0, 0.0, 0.0], [2.0, 1.5, 0.0]),
        edge(&body, [0.0, 0.0, 0.0], [0.0, 1.5, 0.0]),
    ];
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let (sec, corner) = (verb.section(), verb.corner());
        let both: Vec<EdgeKey> = corner_a.iter().chain(&corner_b).copied().collect();
        carve(
            &body,
            &both,
            verb,
            sec * 9.0 - 2.0 * corner,
            "two opposite corners",
        );
        carve(&body, &verticals, verb, sec * 4.0, "four verticals");
        carve(
            &body,
            &corner_and_parallel,
            verb,
            sec * 6.5 - corner,
            "a corner and a parallel",
        );
        carve(
            &body,
            &shared_end_faces,
            verb,
            sec * 5.0,
            "end faces that are supports",
        );
    }
}

/// **A thin plate's two top edges**, both verbs: at `w = 0.25` the two
/// strips stay apart on the top and both bands build; at `w ≤ 2d` the
/// strips meet, and predicate 2 refuses before anything is planned.
#[test]
fn a_thin_plate_builds_its_two_top_edges_until_the_strips_meet() {
    for w in [0.25, 0.2, 0.15] {
        let body = block(2.0, w, 1.0, tol());
        let pair = [
            edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]),
            edge(&body, [0.0, w, 1.0], [2.0, w, 1.0]),
        ];
        for verb in [Verb::Chamfer, Verb::Fillet] {
            let what = format!("a {w} plate");
            if w > 2.0 * D {
                carve(&body, &pair, verb, verb.section() * 4.0, &what);
                continue;
            }
            match verb.run(&body, &pair) {
                Err(BlendError::FaceClearanceUncertified { .. }) => {}
                other => panic!("{what} ({verb:?}): the strips meet, got {other:?}"),
            }
        }
    }
}
