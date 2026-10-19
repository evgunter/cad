//! Review probes for PR 5338 (a cup on a support's strip goes with the
//! band). Each row states the claim it attacks.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Point2, Point3, Vec3};
use sweep::test_support::{prism_on, sketch_from_axes};
use topo::{Body, ContactRecords, validate_geometric, validate_pseudomanifold};

use crate::band_concave_bar::{blend, boolean};
use crate::band_planar_cut_off::{Verb, edge, tol, volume};
use crate::band_strip_mouth_cups::{post, post_faces};

/// A wedge prism along `x ∈ [0, 1]`: its top `z = 1` for `y ∈ [0, 1]`,
/// and its edge `(·, 1, 1)` a convex edge of interior angle `phi` with
/// the face falling from it at `phi` below the top.
fn wedge(phi: f64) -> Body<f64> {
    wedge_along(phi, (0.0, 1.0))
}

/// [`wedge`] along `x ∈ [x0, x1]`.
fn wedge_along(phi: f64, (x0, x1): (f64, f64)) -> Body<f64> {
    let (c, s) = (phi.cos(), phi.sin());
    let q = (1.0 - 1.0 * c, 1.0 - 1.0 * s);
    let p = |y: f64, z: f64| (Point2::new(y, z), 0.0);
    prism_on(
        sketch_from_axes(
            Point3::new(x0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            tol(),
        ),
        vec![p(0.0, 1.0), p(q.0, q.1), p(1.0, 1.0)],
        x1 - x0,
        tol(),
    )
}

fn crease(body: &Body<f64>) -> topo::EdgeKey {
    edge(body, [0.0, 1.0, 1.0], [1.0, 1.0, 1.0])
}

/// **Claim 1, the `S⁺` ν-convexity.** At an acute dihedral the fillet's
/// arc runs past its rightmost point, so a line along the top's normal
/// can leave `S` through the arc and re-enter it in the cusp between the
/// arc and the other support. A dimple square to the top whose mouth is
/// on the strip and whose floor sits in that cusp has every boundary
/// edge certified in the core (the mouth in `M'`), and its vertical
/// walls pay no stray — yet its wall crosses the material the fillet
/// keeps (the disk). The correct body keeps a hole through the disk; a
/// build that takes the whole cup is a wrong body.
#[test]
fn an_acute_wedge_dimple_through_the_fillets_kept_disk_does_not_go_whole() {
    let phi = PI / 6.0;
    let r = 0.1;
    // In units of r: 2.743 from the edge, floor 1.488 under the top
    // (the disk spans 0.852 .. 1.148 there; the far support is at 1.584).
    let (t0, depth, a) = (2.743 * r, 1.488 * r, 0.004);
    let block = wedge(phi);
    let o = Point3::new(0.5, 1.0 - t0, 1.0);
    let dir = Vec3::new(0.0, 0.0, -1.0);
    let body = boolean(
        false,
        block.clone(),
        post(o - dir * 0.05, dir, 0.05 + depth, a),
    );
    let cups = post_faces(&body, (o, dir), a, (0.0, depth));
    assert_eq!(cups.len(), 3, "the dimple's three faces");
    let v0 = volume(&body);
    let e = crease(&body);
    let sec = r * r * (1.0 / (phi / 2.0).tan() - (PI - phi) / 2.0);
    let goes = -(sec - PI * a * a * depth);
    // The disk's chord along the axis: centre (t = r·cot(φ/2), −r).
    let ct = r / (phi / 2.0).tan();
    let chord = 2.0 * (r * r - (ct - t0).powi(2)).sqrt();
    let right = -(sec - PI * a * a * (depth - chord));
    for verb in [Verb::Fillet] {
        match blend(verb, &body, &[e], r) {
            Ok(out) => {
                let dv = volume(&out.body) - v0;
                let gone = cups.iter().all(|f| out.body.get_face(*f).is_none());
                eprintln!(
                    "{verb:?}: built; dv {dv:.12e}, goes-form {goes:.12e}, right ≈ {right:.12e}, all cup faces gone {gone}"
                );
                assert!(
                    !gone && (dv - goes).abs() > 1e-9,
                    "{verb:?}: the cup went whole (dv {dv:e} = the goes-form {goes:e}); \
                     the right body keeps a hole through the disk (dv ≈ {right:e})"
                );
            }
            Err(e) => eprintln!("{verb:?}: refused {e}"),
        }
    }
}

/// Control for the acute-wedge row: a shallow dimple clear of the disk
/// on the same wedge.
#[test]
fn an_acute_wedge_shallow_dimple_control() {
    let phi = PI / 6.0;
    let r = 0.1;
    for (t0, depth, a) in [
        (2.0 * r, 0.2 * r, 0.004),
        (3.0 * r, 0.3 * r, 0.004),
        (2.743 * r, 0.5 * r, 0.004),
    ] {
        let o = Point3::new(0.5, 1.0 - t0, 1.0);
        let dir = Vec3::new(0.0, 0.0, -1.0);
        let body = boolean(
            false,
            wedge(phi),
            post(o - dir * 0.05, dir, 0.05 + depth, a),
        );
        let e = crease(&body);
        let v0 = volume(&body);
        match blend(Verb::Fillet, &body, &[e], r) {
            Ok(out) => eprintln!(
                "t0 {t0} depth {depth}: built dv {:e}",
                volume(&out.body) - v0
            ),
            Err(e) => eprintln!("t0 {t0} depth {depth}: refused {e}"),
        }
    }
}

/// A round post of radius `a` from `from` along `-z` for `len`, its
/// wall split into two faces at section angles `240°` and `300°`: its
/// seams lie on the `+y` side (towards the wedge's edge).
fn seamed_post(from: Point3<f64>, len: f64, a: f64) -> Body<f64> {
    let at = |deg: f64| {
        let t = deg.to_radians();
        Point2::new(a * t.cos(), a * t.sin())
    };
    prism_on(
        sketch_from_axes(
            from,
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            tol(),
        ),
        vec![
            (at(240.0), (15.0f64).to_radians().tan()),
            (at(300.0), (75.0f64).to_radians().tan()),
        ],
        len,
        tol(),
    )
}

/// **Claim 1, the `S⁺` ν-convexity, with every boundary edge in `S`.**
/// As the row above, but the dimple's seams stand on the edge's side of
/// the disk and its floor sits in the cusp where the slice of `S` at that
/// height is one interval: every boundary edge of every cup face lies in
/// the core (the mouth in `M'`), yet the wall's far side cuts through the
/// disk the fillet keeps. Its vertical rulings pay no stray. The correct
/// body keeps the dimple's part inside the disk; taking the whole cup is
/// a wrong body.
#[test]
fn an_acute_wedge_dimple_with_its_boundary_in_s_through_the_kept_disk_does_not_go_whole() {
    let phi = PI / 6.0;
    let r = 0.1;
    let (t0, depth, a) = (2.62 * r, 1.4 * r, 0.15 * r);
    let o = Point3::new(0.5, 1.0 - t0, 1.0);
    let dir = Vec3::new(0.0, 0.0, -1.0);
    let body = boolean(
        false,
        wedge(phi),
        seamed_post(o - dir * 0.05, 0.05 + depth, a),
    );
    let cups = post_faces(&body, (o, dir), a, (0.0, depth));
    assert_eq!(cups.len(), 3, "the dimple's three faces");
    let v0 = volume(&body);
    let e = crease(&body);
    let sec = r * r * (1.0 / (phi / 2.0).tan() - (PI - phi) / 2.0);
    let goes = -(sec - PI * a * a * depth);
    // The post's part inside the disk the fillet keeps (centre at
    // `t = r·cot(φ/2)`, `r` under the top): its column chords, which
    // lie whole above the floor.
    let ct = r / (phi / 2.0).tan();
    let n = 20_000;
    let kept: f64 = (0..n)
        .map(|i| {
            let dt = 2.0 * a / f64::from(n);
            let t = t0 - a + (f64::from(i) + 0.5) * dt;
            let along = 2.0 * (a * a - (t - t0).powi(2)).max(0.0).sqrt();
            let off = (t - ct).abs();
            if off < r {
                along * 2.0 * (r * r - off * off).sqrt() * dt
            } else {
                0.0
            }
        })
        .sum();
    assert!(kept > 1e-6, "the wall cuts the disk: {kept:e}");
    let right = goes - kept;
    let out = blend(Verb::Fillet, &body, &[e], r);
    let Ok(out) = out else {
        eprintln!("refused {:?}", out.err());
        return;
    };
    validate_geometric(&out.body, tol()).expect("tier 3 holds on what was built");
    validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
        .expect("tier 3′ holds on what was built");
    let dv = volume(&out.body) - v0;
    let gone = cups.iter().all(|f| out.body.get_face(*f).is_none());
    let (c0, c1) = (
        topo::readback::euler_counts(&body),
        topo::readback::euler_counts(&out.body),
    );
    eprintln!(
        "built; tiers 3/3′ pass; dv {dv:.12e}, goes-form {goes:.12e}, right {right:.12e} \
         (kept {kept:e}); cup faces gone {gone}; genus {:?} -> {:?}",
        c0.genus(),
        c1.genus()
    );
    assert!(
        (dv - right).abs() < 1e-9,
        "the cup went whole (dv {dv:e} = the goes-form {goes:e}) though its wall cuts the \
         disk the fillet keeps: the right body keeps {kept:e} of the post as a slot (dv {right:e})"
    );
}

/// **Claim 1, the screen's stand-down, on a ring that is itself
/// requested.** A dimple's mouth on the block's top strip, its mouth
/// edge requested with the block's top edge: the ring carries a
/// requested edge, so arm (a) does not meter it and arm (d) leaves
/// requested edges to predicate 2's pairwise screen — which now stands
/// down on that ring beside the one-link band. The two bands overlap,
/// so the request must refuse, typed.
#[test]
fn a_requested_mouth_on_the_strip_still_refuses() {
    use crate::band_strip_mouth_cups::post;
    let k = 1.0;
    let (u, a, h) = (0.04, 0.01, 0.03);
    let o = Point3::new(0.5, k - u, k);
    let dir = Vec3::new(0.0, 0.0, -1.0);
    let body = boolean(
        false,
        sweep::test_support::brick((0.0, k), (0.0, k), (0.0, k), tol()),
        post::<f64>(o - dir * 0.2, dir, 0.2 + h, a),
    );
    let top = edge(&body, [0.0, k, k], [k, k, k]);
    let point = |v| {
        *body
            .get_point(body.get_vertex(v).expect("v").point)
            .expect("p")
    };
    let mouth: Vec<topo::EdgeKey> = topo::query::all_edges(&body)
        .into_iter()
        .filter(|&e| {
            let he = body.get_edge(e).expect("an edge").he_plus;
            let p: Point3<f64> = point(body.get_half_edge(he).expect("a half").start);
            let q: Point3<f64> = point(body.half_edge_end(he).expect("an end"));
            (p.z - k).abs() < 1e-12 && (q.z - k).abs() < 1e-12 && ((p - o).norm() - a).abs() < 1e-9
        })
        .collect();
    assert!(!mouth.is_empty(), "the mouth's edges");
    for verb in [Verb::Fillet, Verb::Chamfer] {
        let mut edges = vec![top];
        edges.extend(&mouth);
        match blend(verb, &body, &edges, 0.1) {
            Ok(out) => panic!(
                "{verb:?}: built over two overlapping bands, dv {:e}",
                volume(&out.body) - volume(&body)
            ),
            Err(e) => eprintln!("{verb:?}: refused {e:?}"),
        }
    }
}

/// **Claim 1, the concave mirror.** A slot of the [`wedge`]'s section
/// through a brick: its crease `(·, 1, 1)` is concave, its void `30°`.
/// A boss on the slot's ceiling `z = 1`, square to it, its mouth on the
/// ceiling's strip, its seams and its end in the fill, its wall through
/// the disk the fill leaves void. The right body keeps the boss's part
/// in that disk; taking the whole boss is a wrong body.
#[test]
fn an_acute_slot_boss_through_the_fills_void_disk_does_not_go_whole() {
    let phi = PI / 6.0;
    let r = 0.1;
    let (t0, depth, a) = (2.62 * r, 1.4 * r, 0.15 * r);
    let o = Point3::new(0.5, 1.0 - t0, 1.0);
    let dir = Vec3::new(0.0, 0.0, -1.0);
    // The slot runs between pocket walls `x = 0.2` and `x = 0.8` and
    // opens through the brick's face `y = 0.3`, so its crease ends at
    // all-concave corners.
    let slotted = boolean(
        false,
        sweep::test_support::brick((0.0, 1.0), (0.3, 1.5), (0.0, 1.5), tol()),
        wedge_along(phi, (0.2, 0.8)),
    );
    let body = boolean(true, slotted, seamed_post(o - dir * 0.05, 0.05 + depth, a));
    let cups = post_faces(&body, (o, dir), a, (0.0, depth));
    assert_eq!(cups.len(), 3, "the boss's three faces");
    let v0 = volume(&body);
    let e = edge(&body, [0.2, 1.0, 1.0], [0.8, 1.0, 1.0]);
    let sec = 0.6 * r * r * (1.0 / (phi / 2.0).tan() - (PI - phi) / 2.0);
    let goes = sec - PI * a * a * depth;
    let ct = r / (phi / 2.0).tan();
    let n = 20_000;
    let kept: f64 = (0..n)
        .map(|i| {
            let dt = 2.0 * a / f64::from(n);
            let t = t0 - a + (f64::from(i) + 0.5) * dt;
            let along = 2.0 * (a * a - (t - t0).powi(2)).max(0.0).sqrt();
            let off = (t - ct).abs();
            if off < r {
                along * 2.0 * (r * r - off * off).sqrt() * dt
            } else {
                0.0
            }
        })
        .sum();
    let right = goes + kept;
    let out = blend(Verb::Fillet, &body, &[e], r);
    let Ok(out) = out else {
        eprintln!("refused {:?}", out.err());
        return;
    };
    validate_geometric(&out.body, tol()).expect("tier 3 holds on what was built");
    validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
        .expect("tier 3′ holds on what was built");
    let dv = volume(&out.body) - v0;
    let gone = cups.iter().all(|f| out.body.get_face(*f).is_none());
    eprintln!(
        "built; tiers 3/3′ pass; dv {dv:.12e}, goes-form {goes:.12e}, right {right:.12e}; \
         boss faces gone {gone}"
    );
    assert!(
        (dv - right).abs() < 1e-9,
        "the boss went whole (dv {dv:e}, goes-form {goes:e}) though its wall cuts the disk the \
         fill leaves void: the right body keeps {kept:e} of it (dv {right:e})"
    );
}
