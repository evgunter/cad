//! **The planar cut-off across the shapes of its end and its request**,
//! each built at its closed form through [`carve`] (tier 3, one genus-0
//! shell, naming totality, `ΔV`): end faces leaning along the support
//! and tilted out of the vertical, a dihedral that is not right, and
//! requests that mix patches, cut-offs and shared end faces on one box.
//! At an oblique end face the chamfer ends in a chord and the fillet in
//! an arc of an ellipse, the end plane's section of its cylinder.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::Curve3;
use geom_core::{Point2, Point3, Vec3};
use sweep::blend::{BlendError, Blended};
use sweep::test_support::{block, prism, prism_on, sketch_from_axes};
use topo::EdgeKey;

use crate::band_planar_cut_off::{D, Verb, carve, edge, the_box, tol};

/// **A fillet's oblique end curves are ellipses**: each `EndArc` is a
/// certified ellipse of minor semi-axis `r`, its major `r / cos θ` for
/// the tilt `θ` its plane makes with the edge's normal plane, with
/// `cos θ = |n̂ · τ̂|` read off the stored carrier's own frame; and at
/// least `oblique` of them are not circles.
fn ends_are_sections(out: &Blended<f64>, oblique: usize, what: &str) {
    let rec = out.naming.as_ref().expect("births");
    let mut ellipses = 0;
    for (arc, _, _) in &rec.arcs {
        let c = out
            .body
            .get_curve_geom(out.body.get_edge(*arc).unwrap().curve)
            .and_then(|g| g.certified())
            .expect("a certified end curve");
        match *c.carrier() {
            Curve3::Circle { radius, .. } => assert_eq!(radius, D, "{what}: a circle end"),
            Curve3::Ellipse {
                major, minor, axis, ..
            } => {
                ellipses += 1;
                assert!((minor - D).abs() < 1e-15, "{what}: minor = r, got {minor}");
                let band = out
                    .blend_faces
                    .iter()
                    .find_map(|f| {
                        let s = out.body.get_surface(out.body.get_face(*f)?.surface)?;
                        match *s {
                            geom::Surface::Cylinder { axis, .. } => Some(axis),
                            _ => None,
                        }
                    })
                    .expect("a cylinder band");
                let cos = axis.dot(band).abs();
                assert!(
                    (major - D / cos).abs() < 1e-12,
                    "{what}: major = r / cos θ, got {major} vs {}",
                    D / cos
                );
                let (t0, t1) = c.params();
                assert!(
                    t1 > t0 && t1 - t0 < PI,
                    "{what}: the end arc is short and runs forward"
                );
            }
            ref other => panic!("{what}: an end curve is a circle or an ellipse, got {other:?}"),
        }
    }
    assert!(
        ellipses >= oblique,
        "{what}: {ellipses} ellipse ends, expected at least {oblique}"
    );
}

/// **End faces leaning along the support**, both verbs: a trapezoid
/// prism `(0,0), (2,0), (2−s,1), (s,1)` at slopes on both sides of
/// upright. The band's region is a prism of the verb's section cut by
/// two planes, so `ΔV = A·L(ȳ)` with `L(y) = 2 − 2sy` at the section's
/// centroid ([`Verb::centroid`]): `ȳ = c` for the front edges, `1 − c`
/// for the back one. A trapezoid in `xz`, extruded along `y`, leans the
/// end faces the other transverse way.
#[test]
fn end_faces_leaning_along_the_support_cut_both_verbs_at_the_centroid_length() {
    for (verb, s) in [Verb::Chamfer, Verb::Fillet]
        .into_iter()
        .flat_map(|v| [0.05, 0.3, 0.5, 0.9, -0.3, -1.0, -3.0].map(|s| (v, s)))
    {
        let (half, c) = (verb.section(), verb.centroid());
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
        let front = half * (2.0 - 2.0 * s * c);
        for z in [1.0, 0.0] {
            let e = edge(&body, [0.0, 0.0, z], [2.0, 0.0, z]);
            let what = format!("trapezoid s = {s}, z = {z}");
            let out = carve(&body, &[e], verb, front, &what);
            if let Verb::Fillet = verb {
                ends_are_sections(&out, 2, &what);
            }
        }
        let back = edge(&body, [s, 1.0, 1.0], [2.0 - s, 1.0, 1.0]);
        let along_back = half * (2.0 - 2.0 * s * (1.0 - c));
        carve(
            &body,
            &[back],
            verb,
            along_back,
            &format!("trapezoid back s = {s}"),
        );
    }
    for (verb, s) in [Verb::Chamfer, Verb::Fillet]
        .into_iter()
        .flat_map(|v| [0.3, -0.5].map(|s| (v, s)))
    {
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
        let removed = verb.section() * (2.0 - 2.0 * s * verb.centroid());
        carve(&body, &[e], verb, removed, &format!("xz trapezoid s = {s}"));
    }
}

/// **End faces tilted out of the vertical**, both verbs: a profile in
/// `(z, x)` extruded along `y`, its end walls leaning inward
/// (`L(z) = 2 − 0.6z`) or outward (`L(z) = 1.4 + 0.6z`). The top front
/// edge's section has its centroid at `z̄ = 1 − c`, so `ΔV = A·L(z̄)`.
#[test]
fn end_faces_tilted_out_of_the_vertical_cut_both_verbs_at_the_centroid_length() {
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
        for verb in [Verb::Chamfer, Verb::Fillet] {
            let zbar = 1.0 - verb.centroid();
            let out = carve(&body, &[e], verb, verb.section() * (l0 + l1 * zbar), what);
            if let Verb::Fillet = verb {
                ends_are_sections(&out, 2, what);
            }
        }
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
