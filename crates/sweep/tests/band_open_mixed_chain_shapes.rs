//! **Open mixed chains of every shape the reviews measured**, each at
//! its closed form (Pappus by pieces, `band_open_mixed_chain`'s
//! oracle), and the refusals that stay.
//!
//! - Links of one open chain joined along it by links turning less than
//!   a half turn are no clearance pair of the support screen: arcs
//!   flanking a short flat, a compound corner, a short link to an
//!   oblique cut-off.
//! - An arc of a half turn or more is metered over its own pieces'
//!   wedges, not its whole circle: keyholes of 270°, 220° and 200°, a
//!   180° tombstone; a hole over the band still refuses.
//! - Two arcs, an S-run of a convex and a reflex arc, a run ending at a
//!   turn or at a corner patch, two runs on one curved mate, the body's
//!   membership, and naming independent of the request's order.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use crate::band_closed_mixed_chain_shapes::assert_touch_escalation;
use crate::band_open_mixed_chain::{
    R, assert_shared_row_unadmitted, edges_at, l_plate, outline, run_row, section, tol, volume,
};
use crate::common::interval::iv;
use geom_core::{Bounds, CertifiedBounds, Decide, Interval, Point3};
use sweep::blend::{BlendError, fillet_edges};
use sweep::test_support::{at_rest, extruded};
use topo::{Body, EdgeKey};

type V = (f64, f64, f64);

fn q(theta: f64) -> f64 {
    (theta / 4.0).tan()
}

/// The edges of `body` both of whose ends `keep` admits.
fn edges_by<T: Decide + Bounds>(
    body: &Body<T>,
    keep: impl Fn(Point3<f64>, Point3<f64>) -> bool,
) -> Vec<EdgeKey> {
    let mid = |x: T| 0.5 * (x.lo() + x.hi());
    let mut all = topo::query::all_edges(body);
    all.sort_unstable();
    all.into_iter()
        .filter(|&e| {
            let (s, t) = body.edge_vertices(e).unwrap();
            let p = |v| {
                let p = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
                Point3::new(mid(p.x), mid(p.y), mid(p.z))
            };
            keep(p(s), p(t))
        })
        .collect()
}

/// The L-plate's run on its top: the line along `y = 0`, the arc, the
/// line along `x = 2`.
fn l_run<T: Decide + Bounds>(body: &Body<T>, z: f64) -> Vec<EdgeKey> {
    edges_at(body, z, |x, y| y.abs() < 1e-9 || (x > 1.5 && y < 0.9))
}

fn l_truth(r: f64) -> f64 {
    let (a, d) = section(r);
    a * ((2.0 - R) + (1.0 - R) + FRAC_PI_2 * (R - d))
}

/// A built row at `f64` (the `Interval` rows say what they do at the
/// tightest ε themselves).
fn builds(what: &str, body: &Body<f64>, edges: &[EdgeKey], r: f64, links: usize, truth: f64) {
    run_row(
        what,
        body,
        edges,
        r,
        r,
        (links, links - 1),
        (-1.0, truth),
        assert_touch_escalation,
    );
}

/// **Arcs flanking a short flat**: line, arc 0.3, a 0.1 flat, arc 0.3,
/// line. The two lines and the two arcs face across the flat, closer
/// than two setbacks at every radius here; their windows lie on either
/// side of the run between them, which turns a quarter turn.
#[test]
fn arcs_flanking_a_short_flat_in_an_open_run_build() {
    let f = 0.1;
    let top = 2.0 * R + f;
    let o: Vec<V> = vec![
        (0.0, 0.0, 0.0),
        (3.0 - R, 0.0, q(FRAC_PI_2)),
        (3.0, R, 0.0),
        (3.0, R + f, q(FRAC_PI_2)),
        (3.0 - R, top, 0.0),
        (1.0, top, 0.0),
        (1.0, 0.35, 0.0),
        (0.0, 0.35, 0.0),
    ];
    let body = outline::<f64>(&o, 0.0, 0.5);
    let run = edges_at(&body, 0.5, |x, y| {
        y.abs() < 1e-9 || x > 2.5 || (y - top).abs() < 1e-9
    });
    assert_eq!(run.len(), 5, "line, arc, flat, arc, line");
    for r in [0.06, 0.1, 0.14] {
        let (a, d) = section(r);
        let truth = a * ((3.0 - R) + f + (2.0 - R) + 2.0 * FRAC_PI_2 * (R - d));
        builds(&format!("short flat, r = {r}"), &body, &run, r, 5, truth);
    }
}

/// **A compound corner**: line, arc 0.3 over 45°, arc 0.6 over 45°,
/// line, two arcs on different cylinders meeting at a junction of their
/// own.
#[test]
fn a_compound_corner_of_two_arcs_builds() {
    for (r1, r2) in [(0.3, 0.6), (0.6, 0.3)] {
        let (s, c) = (PI / 4.0).sin_cos();
        let p1 = (1.0 + r1 * s, r1 - r1 * c);
        let c2 = (p1.0 - r2 * s, p1.1 + r2 * c);
        let p2 = (c2.0 + r2, c2.1);
        let y_top = p2.1 + 1.0;
        let o: Vec<V> = vec![
            (0.0, 0.0, 0.0),
            (1.0, 0.0, q(PI / 4.0)),
            (p1.0, p1.1, q(PI / 4.0)),
            (p2.0, p2.1, 0.0),
            (p2.0, y_top, 0.0),
            (0.0, y_top, 0.0),
        ];
        let body = outline::<f64>(&o, 0.0, 0.5);
        let run = edges_at(&body, 0.5, |x, y| x > 1e-9 && y < y_top - 1e-9);
        assert_eq!(run.len(), 4, "line, arc, arc, line");
        for r in [0.1, 0.12, 0.14] {
            let (a, d) = section(r);
            let truth = a * (2.0 + PI / 4.0 * (r1 - d) + PI / 4.0 * (r2 - d));
            builds(
                &format!("compound {r1}/{r2}, r = {r}"),
                &body,
                &run,
                r,
                4,
                truth,
            );
        }
    }
}

/// **A short link to an oblique cut-off**: the L-plate's run whose arm
/// along `x = 2` is `s = 0.1` long and ends at a wall leaning back at
/// 45°; the arm's prism to the oblique plane holds `s − d/2` of
/// centroid line.
#[test]
fn a_short_link_to_an_oblique_cut_off_builds() {
    let s = 0.1;
    let o: Vec<V> = vec![
        (0.0, 0.0, 0.0),
        (2.0 - R, 0.0, q(FRAC_PI_2)),
        (2.0, R, 0.0),
        (2.0, R + s, 0.0),
        (1.8, R + s - 0.1, 0.0),
        (1.8, 1.0, 0.0),
        (0.0, 1.0, 0.0),
    ];
    let body = outline::<f64>(&o, 0.0, 0.5);
    let on = |p: Point3<f64>, ps: &[(f64, f64)]| {
        (p.z - 0.5).abs() < 1e-9
            && ps
                .iter()
                .any(|q| (q.0 - p.x).abs() < 1e-9 && (q.1 - p.y).abs() < 1e-9)
    };
    let ends = [(0.0, 0.0), (2.0 - R, 0.0), (2.0, R), (2.0, R + s)];
    let run = edges_by(&body, |a, b| on(a, &ends) && on(b, &ends));
    assert_eq!(run.len(), 3, "line, arc, short line");
    let r = 0.1;
    let (a, d) = section(r);
    let truth = a * ((2.0 - R) + FRAC_PI_2 * (R - d) + (s - 0.5 * d));
    builds("short oblique link", &body, &run, r, 3, truth);
}

/// A keyhole: an arc of `sweep` about `(0, 1)` of radius 1, its middle
/// at the top, its two tangent lines run down to `y = yb`, `extra`
/// below the arc's lower end, closed by the bottom line. The run is
/// line, arc, line, each line cut off obliquely at the bottom.
fn keyhole(sweep: f64, extra: f64) -> (Body<f64>, f64, Vec<EdgeKey>) {
    let a0 = FRAC_PI_2 - sweep / 2.0;
    let a1 = a0 + sweep;
    let p0 = (a0.cos(), 1.0 + a0.sin());
    let p1 = (a1.cos(), 1.0 + a1.sin());
    let t0 = (-a0.sin(), a0.cos());
    let t1 = (-a1.sin(), a1.cos());
    let yb = p0.1.min(p1.1) - 0.3 - extra;
    let s0 = (p0.1 - yb) / t0.1;
    let q0 = (p0.0 - s0 * t0.0, yb);
    let s1 = (yb - p1.1) / t1.1;
    let q1 = (p1.0 + s1 * t1.0, yb);
    let o = [
        (q1.0, q1.1, 0.0),
        (q0.0, q0.1, 0.0),
        (p0.0, p0.1, q(sweep)),
        (p1.0, p1.1, 0.0),
    ];
    let body = outline::<f64>(&o, 0.0, 0.5);
    let run = edges_at(&body, 0.5, |_, y| y > yb + 1e-6);
    let r = 0.1;
    let (a, d) = section(r);
    // Each line's centroid line runs from its junction's section plane,
    // `d` inside, down to the bottom plane.
    let n0 = (-t0.1, t0.0);
    let len0 = (p0.1 + d * n0.1 - yb) / t0.1;
    let n1 = (-t1.1, t1.0);
    let len1 = (yb - (p1.1 + d * n1.1)) / t1.1;
    (body, a * (len0 + len1 + sweep * (1.0 - d)), run)
}

/// **An arc of a half turn or more is metered over its own extent**:
/// keyholes of 270° and 220° on long stems, 200° on short ones, and a
/// 180° tombstone build at Pappus, where reading the arc's whole circle
/// refused them.
#[test]
fn arcs_of_a_half_turn_or_more_build() {
    for (deg, extra) in [(270.0f64, 0.0), (220.0, 1.0), (200.0, 0.0), (180.0, 0.5)] {
        let (body, truth, run) = keyhole(deg.to_radians(), extra);
        assert_eq!(run.len(), 3, "{deg}°: line, arc, line");
        builds(&format!("keyhole {deg}°"), &body, &run, 0.1, 3, truth);
    }
}

/// **A hole over a wide arc's band still refuses, and one clear of it
/// builds**: a vertical bore about the 270° keyhole's centre, of radius
/// 0.95 into the band on the top (which spans radii `[0.9, 1]` at
/// `r = 0.1`), refused by predicate 2's support screen, and of 0.85 clear
/// of it, which the windowed meters pass.
#[test]
fn a_hole_over_a_wide_arcs_band_refuses_and_one_clear_of_it_builds() {
    use topo::boolean::{BooleanInputs, BooleanOp, SweepStrategy, boolean_op_with};
    for (hr, fits) in [(0.85, true), (0.95, false)] {
        let (body, truth, _) = keyhole(1.5 * PI, 0.0);
        let bore = outline::<f64>(&[(-hr, 1.0, 1.0), (hr, 1.0, 1.0)], -0.5, 1.5);
        let holed = boolean_op_with(
            BooleanOp::Subtract,
            &sweep::test_support::finished("the keyhole", body, tol()),
            &sweep::test_support::finished("the bore", bore, tol()),
            &BooleanInputs::default(),
            SweepStrategy::Realized,
            tol(),
        )
        .unwrap_or_else(|e| panic!("the bore builds: {e:?}"))
        .body()
        .expect("a body")
        .body
        .clone()
        .into_body();
        // The bottom sits 0.3 below the arc's lower ends, at `1 − sin 45° − 0.3`.
        let yb = 0.7 - FRAC_PI_2.sin() * 0.5f64.sqrt();
        let on_bore = |p: Point3<f64>| ((p.x * p.x + (p.y - 1.0).powi(2)).sqrt() - hr).abs() < 1e-9;
        let run = edges_by(&holed, |a, b| {
            (a.z - 0.5).abs() < 1e-9
                && (b.z - 0.5).abs() < 1e-9
                && a.y.max(b.y) > yb + 1e-6
                && !(on_bore(a) && on_bore(b))
        });
        assert_eq!(run.len(), 3, "bore {hr}: line, arc, line");
        if fits {
            builds(&format!("bore {hr}"), &holed, &run, 0.1, 3, truth);
        } else {
            let got = fillet_edges(&at_rest(&holed, tol()), &run, 0.1, tol()).map_err(|r| r.error);
            assert!(
                matches!(got, Err(BlendError::FaceClearanceUncertified { .. })),
                "bore {hr}: a ring over the wide arc's band refuses, got {got:?}"
            );
        }
    }
}

/// **Two arcs in one run**, equal and unequal: line, arc, line, arc,
/// line around a rectangle's two right corners.
fn two_arcs_row<T: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy>(
    what: &str,
    (r1, r2): (f64, f64),
    r: f64,
    size: T,
) {
    let o: Vec<V> = vec![
        (0.0, 0.0, 0.0),
        (3.0 - r1, 0.0, q(FRAC_PI_2)),
        (3.0, r1, 0.0),
        (3.0, 2.0 - r2, q(FRAC_PI_2)),
        (3.0 - r2, 2.0, 0.0),
        (0.0, 2.0, 0.0),
    ];
    let body = outline::<T>(&o, 0.0, 0.5);
    let run = edges_at(&body, 0.5, |x, _| x > 1e-9);
    assert_eq!(run.len(), 5, "{what}: line, arc, line, arc, line");
    let (a, d) = section(r);
    let truth = a * ((3.0 - r1) + (2.0 - r1 - r2) + (3.0 - r2) + FRAC_PI_2 * (r1 - d + r2 - d));
    run_row(
        what,
        &body,
        &run,
        r,
        size,
        (5, 4),
        (-1.0, truth),
        assert_touch_escalation,
    );
}

#[test]
fn two_arcs_in_one_run_build() {
    for radii in [(0.3, 0.3), (0.3, 0.5), (0.5, 0.3)] {
        for r in [0.1, 0.14] {
            two_arcs_row::<f64>(&format!("two arcs {radii:?}, r = {r}"), radii, r, r);
        }
    }
    two_arcs_row::<Interval>("two arcs, Interval", (0.3, 0.5), 0.1, iv(0.1));
}

/// **An S-run**: the L's convex corner `(2, 1)` rounded at `r1` and its
/// reflex corner `(1, 1)` at `r2`, the run from `y = 0` up `x = 2`,
/// round the convex arc, along `y = 1`, round the reflex arc, up
/// `x = 1` to `y = 2`. The reflex arc's centroid runs at `r2 + d`.
fn s_run<T: Decide + topo::AtRestPolicy>((r1, r2): (f64, f64), turn: f64) -> Body<T> {
    let o: Vec<V> = vec![
        (0.0, 0.0, 0.0),
        (2.0, 0.0, 0.0),
        (2.0, 1.0 - r1, q(FRAC_PI_2)),
        (2.0 - r1, 1.0, 0.0),
        (1.0 + r2, 1.0, -q(FRAC_PI_2)),
        (1.0, 1.0 + r2, 0.0),
        (1.0, 2.0, 0.0),
        (0.0, 2.0, 0.0),
    ];
    let (s, c) = turn.sin_cos();
    let o: Vec<V> = o
        .into_iter()
        .map(|(x, y, b)| (c * x - s * y, s * x + c * y, b))
        .collect();
    outline::<T>(&o, 0.0, 0.5)
}

fn s_run_row<T: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy>(
    what: &str,
    radii: (f64, f64),
    turn: f64,
    size: T,
) {
    let (r1, r2) = radii;
    let body = s_run::<T>(radii, turn);
    let (s, c) = (-turn).sin_cos();
    let run = edges_at(&body, 0.5, |x, y| {
        let (u, v) = (c * x - s * y, s * x + c * y);
        u > 0.9 && v > 0.01 && v < 1.99
    });
    assert_eq!(run.len(), 5, "{what}: line, arc, line, arc, line");
    let r = 0.1;
    let (a, d) = section(r);
    let truth = a
        * ((1.0 - r1) + (1.0 - r1 - r2) + (1.0 - r2) + FRAC_PI_2 * (r1 - d) + FRAC_PI_2 * (r2 + d));
    let tightest = if turn == 0.0 {
        assert_touch_escalation
    } else {
        assert_shared_row_unadmitted
    };
    run_row(what, &body, &run, r, size, (5, 4), (-1.0, truth), tightest);
}

#[test]
fn an_s_run_of_a_convex_and_a_reflex_arc_builds() {
    for radii in [(0.3, 0.3), (0.3, 0.2)] {
        for turn in [0.0, PI / 6.0] {
            s_run_row::<f64>(&format!("S-run {radii:?} turned {turn}"), radii, turn, 0.1);
        }
    }
    s_run_row::<Interval>("S-run, Interval", (0.3, 0.2), 0.0, iv(0.1));
}

/// **The run ending at a turn**: the L-plate's run with the top edge
/// along `x = 0` requested too, the line along `y = 0` meeting it in
/// the mitre at the origin, the other end cut off square at `y = 2`.
#[test]
fn an_open_mixed_run_ending_at_a_turn_builds() {
    let body = l_plate::<f64>();
    let run = edges_at(&body, 0.5, |x, y| {
        y.abs() < 1e-9 || (x > 1.5 && y < 0.9) || x.abs() < 1e-9
    });
    assert_eq!(run.len(), 4, "the run and the edge it turns into");
    for r in [0.1, 0.14] {
        let (a, d) = section(r);
        let truth = a * ((2.0 - R - d) + FRAC_PI_2 * (R - d) + (1.0 - R) + (2.0 - d));
        run_row(
            &format!("run ending at a turn, r = {r}"),
            &body,
            &run,
            r,
            r,
            (4, 2),
            (-1.0, truth),
            assert_touch_escalation,
        );
    }
}

/// **The run ending at a corner patch**, against the square L as a
/// differential: the run, the top edge along `x = 0` and the post at the
/// origin requested on both, the square L's run its line along `y = 0`
/// alone. Everything about the corner is the same on both, so the
/// difference is the rounded run's pieces less the square line's prism.
#[test]
fn an_open_mixed_run_ending_at_a_corner_patch_builds() {
    let r = 0.1;
    let (a, d) = section(r);
    let corner = |p: Point3<f64>, q: Point3<f64>| {
        let post = p.x.abs() < 1e-9 && p.y.abs() < 1e-9 && q.x.abs() < 1e-9 && q.y.abs() < 1e-9;
        let west = (p.z - 0.5).abs() < 1e-9
            && (q.z - 0.5).abs() < 1e-9
            && p.x.abs() < 1e-9
            && q.x.abs() < 1e-9;
        post || west
    };
    let top = |p: Point3<f64>, q: Point3<f64>| (p.z - 0.5).abs() < 1e-9 && (q.z - 0.5).abs() < 1e-9;
    let rounded = l_plate::<f64>();
    let e1 = edges_by(&rounded, |p, q| {
        let (x, y) = (0.5 * (p.x + q.x), 0.5 * (p.y + q.y));
        corner(p, q) || top(p, q) && (y.abs() < 1e-9 || (x > 1.5 && y < 0.9))
    });
    let sharp = outline::<f64>(
        &[
            (0.0, 0.0, 0.0),
            (2.0, 0.0, 0.0),
            (2.0, 1.0, 0.0),
            (1.0, 1.0, 0.0),
            (1.0, 2.0, 0.0),
            (0.0, 2.0, 0.0),
        ],
        0.0,
        0.5,
    );
    let e2 = edges_by(&sharp, |p, q| {
        corner(p, q) || top(p, q) && (0.5 * (p.y + q.y)).abs() < 1e-9
    });
    assert_eq!((e1.len(), e2.len()), (5, 3), "the requests");
    let run = |body: &Body<f64>, edges: &[EdgeKey]| {
        let out = fillet_edges(&at_rest(body, tol()), edges, r, tol())
            .unwrap_or_else(|e| panic!("the corner-ended run builds, got {:?}", e.error));
        topo::validate_geometric(&out.body, tol()).expect("tier 3");
        sweep::test_support::assert_naming_totality(body, &out, edges, "corner-ended run");
        let (v0, w0) = volume(body);
        let (v1, w1) = volume(&out.body.into_body());
        (v0 - v1, w0 + w1)
    };
    let ((dv1, w1), (dv2, w2)) = (run(&rounded, &e1), run(&sharp, &e2));
    // The square line runs 2 from the corner to the far wall; the
    // rounded run's line `2 − R`, its arc and its line `1 − R`.
    let want = a * ((2.0 - R) + FRAC_PI_2 * (R - d) + (1.0 - R) - 2.0);
    assert!(
        (dv1 - dv2 - want).abs() <= w1 + w2 + 1e-12,
        "the corner-ended run's ΔV less the square L's is {}, want {want}",
        dv1 - dv2
    );
}

/// **Two runs on one curved mate**: the L-plate's top and bottom runs in
/// one call, both arcs' bands on the one cylinder wall, at twice the
/// one run's Pappus.
#[test]
fn the_top_and_bottom_runs_on_one_curved_mate_build() {
    let body = l_plate::<f64>();
    let mut run = l_run(&body, 0.5);
    run.extend(l_run(&body, 0.0));
    assert_eq!(run.len(), 6, "two runs of three");
    for r in [0.1, 0.14] {
        run_row(
            &format!("top and bottom, r = {r}"),
            &body,
            &run,
            r,
            r,
            (6, 4),
            (-1.0, 2.0 * l_truth(r)),
            assert_touch_escalation,
        );
    }
}

/// **The filleted L-plate is the rolling ball's body point by point**:
/// a lattice of 4,000 points over the plate and its run's band at
/// `r = 0.14`, each read in or out against the closed form (the L less
/// the quarter disc outside the arc, less the material outside each
/// ball along the run); points within 1e-6 of a boundary are skipped.
#[test]
fn the_filleted_l_plate_is_the_rolling_balls_body() {
    let body = l_plate::<f64>();
    let run = l_run(&body, 0.5);
    let r = 0.14;
    let out = fillet_edges(&at_rest(&body, tol()), &run, r, tol())
        .unwrap_or_else(|e| panic!("the L-plate builds, got {:?}", e.error));
    let c = (2.0 - R, R);
    let truth = |p: Point3<f64>| -> Option<bool> {
        let near = |v: f64| v.abs() < 1e-6;
        let (x, y, z) = (p.x, p.y, p.z);
        if [z, z - 0.5, x, y, x - 2.0, y - 1.0, x - 1.0, y - 2.0]
            .into_iter()
            .any(near)
        {
            return None;
        }
        let in_l = (z > 0.0 && z < 0.5)
            && ((x > 0.0 && x < 2.0 && y > 0.0 && y < 1.0)
                || (x > 0.0 && x < 1.0 && y > 0.0 && y < 2.0));
        let rho = ((x - c.0).powi(2) + (y - c.1).powi(2)).sqrt();
        let past_arc = x > c.0 && y < c.1;
        if past_arc && near(rho - R) {
            return None;
        }
        if !in_l || past_arc && rho > R {
            return Some(false);
        }
        // `s` in from the run's wall, `t` down from the top.
        let t = 0.5 - z;
        let s = if x <= c.0 && y < 1.0 {
            Some(y)
        } else if past_arc {
            Some(R - rho)
        } else if y >= c.1 && y < 1.0 {
            Some(2.0 - x)
        } else {
            None
        };
        if let Some(s) = s.filter(|&s| s < r && t < r) {
            let m = ((r - s).powi(2) + (r - t).powi(2)).sqrt() - r;
            if near(m) {
                return None;
            }
            if m > 0.0 {
                return Some(false);
            }
        }
        Some(true)
    };
    let band = geom_core::Band::linear(tol()).unwrap();
    let (mut read, mut wrong) = (0, Vec::new());
    // Half the lattice over the plate, half in the band along the run.
    let lattice = |n: [usize; 3], lo: [f64; 3], hi: [f64; 3]| {
        let at = move |i: usize, k: usize| lo[k] + (hi[k] - lo[k]) * (i as f64 + 0.5) / n[k] as f64;
        (0..n[0]).flat_map(move |i| {
            (0..n[1]).flat_map(move |j| {
                (0..n[2]).map(move |k| Point3::new(at(i, 0), at(j, 1), at(k, 2)))
            })
        })
    };
    let mut points: Vec<Point3<f64>> =
        lattice([20, 20, 5], [-0.1, -0.1, -0.1], [2.1, 2.1, 0.6]).collect();
    points.extend(lattice([40, 10, 5], [0.0, -0.02, 0.32], [2.05, 1.03, 0.52]));
    assert_eq!(points.len(), 4000, "the lattice");
    for p in points {
        let Some(inside) = truth(p) else { continue };
        read += 1;
        match topo::point_in_solid(&out.body, p, band, tol()) {
            Ok(topo::SolidContainment::In) if inside => {}
            Ok(topo::SolidContainment::Out) if !inside => {}
            got => wrong.push((p, inside, format!("{got:?}"))),
        }
    }
    assert!(
        read > 3000,
        "the lattice reads most of its points, read {read}"
    );
    assert!(
        wrong.is_empty(),
        "membership disagrees at {:?}",
        &wrong[..wrong.len().min(5)]
    );
}

/// **The naming is the request's, not its order's**: two arcs of
/// different radii, requested forward, reversed and rotated, record the
/// same source-side rows.
#[test]
fn an_open_mixed_runs_naming_does_not_depend_on_the_request_order() {
    let o: Vec<V> = vec![
        (0.0, 0.0, 0.0),
        (3.0 - 0.3, 0.0, q(FRAC_PI_2)),
        (3.0, 0.3, 0.0),
        (3.0, 2.0 - 0.5, q(FRAC_PI_2)),
        (3.0 - 0.5, 2.0, 0.0),
        (0.0, 2.0, 0.0),
    ];
    let body = outline::<f64>(&o, 0.0, 0.5);
    let run = edges_at(&body, 0.5, |x, _| x > 1e-9);
    let rows = |edges: &[EdgeKey]| {
        let out = fillet_edges(&at_rest(&body, tol()), edges, 0.1, tol()).unwrap();
        let r = out.naming.unwrap();
        let sorted = |mut v: Vec<String>| {
            v.sort();
            v
        };
        [
            sorted(r.blends.iter().map(|(_, e)| format!("{e:?}")).collect()),
            sorted(
                r.sections
                    .iter()
                    .map(|(_, m, v)| format!("{m:?}{v:?}"))
                    .collect(),
            ),
            sorted(
                r.meridian_splits
                    .iter()
                    .map(|(_, m, b)| {
                        let mut b = b.clone();
                        b.sort();
                        format!("{m:?}{b:?}")
                    })
                    .collect(),
            ),
            sorted(r.rim_feet.iter().map(|(_, v)| format!("{v:?}")).collect()),
            sorted(
                r.trims
                    .iter()
                    .map(|(_, e, f)| format!("{e:?}{f:?}"))
                    .collect(),
            ),
            sorted(
                r.feet
                    .iter()
                    .map(|(_, v, f)| format!("{v:?}{f:?}"))
                    .collect(),
            ),
        ]
    };
    let forward = rows(&run);
    let mut reversed = run.clone();
    reversed.reverse();
    let mut rotated = run.clone();
    rotated.rotate_left(2);
    assert_eq!(forward, rows(&reversed), "reversed");
    assert_eq!(forward, rows(&rotated), "rotated");
}

/// **A curved mate carrying a ring refuses with its own sentence**: a
/// tunnel along `x` through the L-plate below the band, piercing the
/// arc's cylinder wall.
#[test]
fn a_curved_mate_with_a_ring_refuses_naming_it() {
    use topo::boolean::{BooleanInputs, BooleanOp, SweepStrategy, boolean_op_with};
    let plate = l_plate::<f64>();
    let (y0, z0, hr) = (0.15, 0.2, 0.04);
    let tunnel = extruded(
        profile::SketchPlane::yz(),
        vec![profile::test_support::bulge_loop(vec![
            (geom_core::Point2::new(y0 - hr, z0), 1.0),
            (geom_core::Point2::new(y0 + hr, z0), 1.0),
        ])],
        5.0,
        tol(),
    );
    let body = boolean_op_with(
        BooleanOp::Subtract,
        &sweep::test_support::finished("the plate", plate, tol()),
        &sweep::test_support::finished("the tunnel", tunnel, tol()),
        &BooleanInputs::default(),
        SweepStrategy::Realized,
        tol(),
    )
    .unwrap_or_else(|e| panic!("the tunnel builds: {e:?}"))
    .body()
    .expect("a body")
    .body
    .clone()
    .into_body();
    let run = l_run(&body, 0.5);
    assert_eq!(run.len(), 3, "the run");
    let got = fillet_edges(&at_rest(&body, tol()), &run, 0.1, tol()).map_err(|r| r.error);
    assert!(
        matches!(&got, Err(BlendError::UnsupportedGeometry { detail, .. })
            if detail.contains("curved support carries a ring")),
        "a ringed curved mate refuses with its own sentence, got {got:?}"
    );
}

/// **A kink at the junctions refuses `ChainNotG1`**: the L-plate's arc
/// turned `δ` past a quarter, so neither junction is tangent.
#[test]
fn a_kinked_junction_refuses_chain_not_g1() {
    for delta in [0.2, 1e-3] {
        let qd = q(FRAC_PI_2 + delta);
        let o: Vec<V> = vec![
            (0.0, 0.0, 0.0),
            (2.0 - R, 0.0, qd),
            (2.0, R, 0.0),
            (2.0, 1.0, 0.0),
            (1.0, 1.0, 0.0),
            (1.0, 2.0, 0.0),
            (0.0, 2.0, 0.0),
        ];
        let body = outline::<f64>(&o, 0.0, 0.5);
        let run = l_run(&body, 0.5);
        let got = fillet_edges(&at_rest(&body, tol()), &run, 0.1, tol()).map_err(|r| r.error);
        assert!(
            matches!(got, Err(BlendError::ChainNotG1 { .. })),
            "kink {delta}: refuses ChainNotG1, got {got:?}"
        );
    }
}

/// The certified scalar on the L-plate's run turned off the axes, where
/// its sections' shared rows do not admit at the tightest ε.
#[test]
fn a_turned_l_plates_run_builds_at_interval() {
    let turn = PI / 6.0;
    let (s, c) = turn.sin_cos();
    let qq = q(FRAC_PI_2);
    let o: Vec<V> = [
        (0.0, 0.0, 0.0),
        (2.0 - R, 0.0, qq),
        (2.0, R, 0.0),
        (2.0, 1.0, 0.0),
        (1.0, 1.0, 0.0),
        (1.0, 2.0, 0.0),
        (0.0, 2.0, 0.0),
    ]
    .into_iter()
    .map(|(x, y, b)| (c * x - s * y, s * x + c * y, b))
    .collect();
    let body = outline::<Interval>(&o, 0.0, 0.5);
    let run = edges_at(&body, 0.5, |x, y| {
        let (u, v) = (c * x + s * y, -s * x + c * y);
        v.abs() < 1e-9 || (u > 1.5 && v < 0.9)
    });
    assert_eq!(run.len(), 3, "the run");
    run_row(
        "turned L-plate, Interval",
        &body,
        &run,
        0.1,
        iv(0.1),
        (3, 2),
        (-1.0, l_truth(0.1)),
        assert_shared_row_unadmitted,
    );
}
