//! Reviewer rows (cleave-rings-review-r2, PR 3658): plane splits through
//! bored and islanded bodies, through the public doors (`subtract`,
//! `union`, `split`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use sweep::test_support::prism_at;
use topo::Body;
use topo::splitting::{SplitPart, SplitPlane, split};

fn tol() -> Tol {
    Tol::witness()
}

/// A unit-bulge disc prism of radius `r` about `(cx, cy)`, `z ∈ [z0, z0 + h]`.
fn rod(cx: f64, cy: f64, r: f64, z0: f64, h: f64) -> Body<f64> {
    prism_at(
        vec![
            (Point2::new(cx - r, cy), 1.0),
            (Point2::new(cx + r, cy), 1.0),
        ],
        z0,
        h,
        tol(),
    )
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    sweep::test_support::brick(x, y, z, tol())
}

fn sub(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    match topo::subtract(a, b, tol()) {
        Ok(topo::BooleanResult::Body(b)) => b.body,
        other => panic!("subtracts: {:?}", other.err()),
    }
}

fn union(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    match topo::union(a, b, tol()) {
        Ok(topo::BooleanResult::Body(b)) => b.body,
        other => panic!("unites: {:?}", other.err()),
    }
}

/// Through `(0, 0, z)`, tilted `t` rad about `y`, flipped when `flip`.
fn tilt(z: f64, t: f64, flip: bool) -> SplitPlane<f64> {
    let s = if flip { -1.0 } else { 1.0 };
    SplitPlane {
        origin: Point3::new(0.0, 0.0, z),
        normal: Vec3::new(s * t.sin(), 0.0, s * t.cos()),
    }
}

/// The two halves, each asserted to pass tiers 1 and 3, with their
/// section faces' `(sense, ring count)` and volumes.
fn halves(what: &str, body: &Body<f64>, plane: &SplitPlane<f64>) -> [(Vec<(bool, usize)>, f64); 2] {
    let r = split(body, plane, tol()).unwrap_or_else(|e| panic!("{what}: splits: {e:?}"));
    let mut out = Vec::new();
    for (side, part) in [("below", r.below), ("above", r.above)] {
        let SplitPart::Body(h) = part else {
            panic!("{what} {side}: material on both sides")
        };
        assert_eq!(topo::validate(&h), Ok(()), "{what} {side}: tier 1");
        assert_eq!(
            topo::validate_geometric(&h, tol()),
            Ok(()),
            "{what} {side}: tier 3"
        );
        let sections = h
            .faces()
            .filter(|(_, f)| {
                matches!(h.get_surface(f.surface), Some(geom::Surface::Plane { origin, normal, .. })
                    if normal.cross(plane.normal).norm() < 1e-12
                        && (*origin - plane.origin).dot(plane.normal).abs() < 1e-12)
            })
            .map(|(_, f)| (f.sense, f.rings.len()))
            .collect();
        let v = topo::mass_properties(&h, tol())
            .unwrap_or_else(|e| panic!("{what} {side}: volume: {e:?}"))
            .volume;
        out.push((sections, v));
    }
    [out.remove(0), out.remove(0)]
}

/// **A bored brick splits validly at every tilt and rod offset, and
/// the halves' volumes add to the whole.** The steep rows here refused
/// (`RingHomingAmbiguous`, `TornComponent`) or came back invalid
/// (`LoopRoleInverted`) on main at `c19126837`.
#[test]
fn a_bored_brick_splits_at_every_tilt_and_offset() {
    let whole = 40.0 - 2.5 * core::f64::consts::PI;
    let block = brick((-2.0, 2.0), (-2.0, 2.0), (0.0, 2.5));
    for (cx, cy) in [(0.0, 0.0), (0.4, -0.3), (0.5, 0.0), (0.95, 0.0)] {
        let body = sub(&block, &rod(cx, cy, 1.0, -0.5, 3.5));
        for t in [0.3, 0.9, 1.1, 1.4, 1.45] {
            for flip in [false, true] {
                let what = format!("rod ({cx}, {cy}) at tilt {t}, flipped {flip}");
                let [(below, vb), (above, va)] = halves(&what, &body, &tilt(1.25, t, flip));
                assert!(
                    (vb + va - whole).abs() < 1e-6,
                    "{what}: {vb} + {va} against {whole}"
                );
                for (side, s) in [("below", below), ("above", above)] {
                    assert!(
                        s.iter().all(|&(sense, _)| sense),
                        "{what} {side}: every section face counter-clockwise: {s:?}"
                    );
                }
            }
        }
    }
}

/// **A hole inside an island inside a hole goes to the island.** The
/// block less an annular groove (`R ∈ [1, 2]`, from `z = 1` up) whose
/// island is bored at `R = 0.5`: on a plane through the groove each
/// half's section is two faces, the block's square holed by the groove
/// and the island's disc holed by the bore.
#[test]
fn a_hole_in_an_island_in_a_hole_goes_to_the_island() {
    let block = brick((-3.0, 3.0), (-3.0, 3.0), (0.0, 4.0));
    let grooved = sub(&block, &rod(0.0, 0.0, 2.0, 1.0, 4.0));
    let islanded = union(&grooved, &rod(0.0, 0.0, 1.0, 0.5, 4.0));
    let body = sub(&islanded, &rod(0.0, 0.0, 0.5, -1.0, 6.0));
    for (z, t, flip) in [(2.0, 0.0, false), (2.0, 0.2, false), (2.0, 0.4, true)] {
        let what = format!("plane through z = {z} at tilt {t}, flipped {flip}");
        for (side, (s, _)) in
            ["below", "above"]
                .into_iter()
                .zip(halves(&what, &body, &tilt(z, t, flip)))
        {
            assert_eq!(
                s,
                vec![(true, 1), (true, 1)],
                "{what} {side}: the square holed by the groove, the island holed by the bore"
            );
        }
    }
}

/// **M1 witness: a flat cut refuses where two unrelated crossings sit
/// within the column band's ambiguity window in `u` alone.** Two rods
/// two apart in `y`, their facing seams `5e-9` apart in `x`; the cut is
/// the axis-aligned plane `z = 1.25`, so `u = x` exactly. Main answers
/// (each half one face, two rings after nesting); the head refuses
/// `Join(OrderEscalated)` on `split_join_order_column`.
#[test]
#[ignore = "M1 witness: red on 6ceb56ffb"]
fn a_flat_cut_answers_whatever_the_u_gap_between_unrelated_crossings() {
    let block = brick((-2.5, 2.5), (-2.5, 2.5), (0.0, 2.5));
    let a = sub(&block, &rod(0.5, 1.0, 0.5, -0.5, 3.5));
    for g in [2e-9, 5e-9, 9e-9] {
        let body = sub(&a, &rod(1.5 + g, -1.0, 0.5, -0.5, 3.5));
        let what = format!("seams {g:e} apart");
        for (side, (s, _)) in
            ["below", "above"]
                .into_iter()
                .zip(halves(&what, &body, &tilt(1.25, 0.0, false)))
        {
            assert_eq!(s, vec![(true, 2)], "{what} {side}");
        }
    }
}

/// **M1 witness, one face: a hairline slot crossed along its length
/// by a plane `1e-5` rad off its axis refuses.** The slot is `0.1 mm`
/// wide; the cap line's `u` gap across it lands in `(ε, K·ε)`. Main
/// answers the same rows.
#[test]
#[ignore = "M1 witness: red on 6ceb56ffb"]
fn a_hairline_slot_cut_nearly_along_its_axis_answers() {
    let t: f64 = 1.4;
    let w = 1e-4;
    let block = brick((-3.0, 3.0), (-3.0, 3.0), (0.0, 2.5));
    let slot = prism_at(
        [
            (-0.8, -w / 2.0),
            (1.2, -w / 2.0),
            (1.2, w / 2.0),
            (-0.8, w / 2.0),
        ]
        .iter()
        .map(|&(x, y)| (Point2::new(x, y), 0.0))
        .collect(),
        -0.5,
        3.5,
        tol(),
    );
    let body = sub(&block, &slot);
    for delta in [1e-5, -1e-5] {
        let plane = SplitPlane {
            origin: Point3::new(0.0, 0.0, 1.25),
            normal: Vec3::new(-t.sin(), -delta, -t.cos()).normalize(),
        };
        halves(&format!("off-axis by {delta:e}"), &body, &plane);
    }
}
