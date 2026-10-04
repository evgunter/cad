//! **A plane lying along an edge of a solid classifies the edge by its
//! convexity**: a convex edge, all of whose material is on one side,
//! stays an ordinary edge of that side's piece and adds nothing to the
//! section; a reflex edge, with material on both sides, is cut through.
//!
//! The first rows are the block y, z ∈ (0, 1) touched by the plane
//! y + z = 2 along its top/far edge, alone and with a slab through it at
//! x = 1.2..1.3. The rest are planar edges along which a plane lies,
//! convex and reflex, most with a real section elsewhere in the same
//! body. Every expected volume is computed by hand from the profile, not
//! read off a run, and every row runs under `n` and `−n`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol, Vec3};
use topo::test_support::{
    brick, describe_as_intersections, finished, holed_block, prism, split_plane,
};
use topo::{
    AtRestBody, Body, SplitPart, SplitPlane, mass_properties, plane_section, split, union,
    validate_closed,
};

/// The plane through `o` with normal along `n` (normalized).
fn plane(o: (f64, f64, f64), n: (f64, f64, f64)) -> SplitPlane<f64> {
    split_plane(
        Point3::new(o.0, o.1, o.2),
        Vec3::new(n.0, n.1, n.2).normalize(),
        Tol::witness(),
    )
}

fn unite(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> Body<f64> {
    union(a, b, Tol::witness())
        .unwrap()
        .body()
        .expect("the operands overlap")
        .body
        .clone()
        .into_body()
}

/// A closed half's volume; `None` for an empty one.
fn volume(label: &str, part: &SplitPart<f64>) -> Option<f64> {
    part.body().map(|b| {
        assert_eq!(validate_closed(b), Ok(()), "{label}");
        mass_properties(b, Tol::witness()).unwrap().volume
    })
}

/// Splits `body` under `n` and under `−n`, expecting `(above, below)`
/// under `n` (`None` for an empty half) and the swap under `−n`. Returns
/// the two results, `n` first.
fn both_ways(
    label: &str,
    body: &Body<f64>,
    o: (f64, f64, f64),
    n: (f64, f64, f64),
    want: (Option<f64>, Option<f64>),
) -> [topo::SplitResult<f64>; 2] {
    [(1.0, want), (-1.0, (want.1, want.0))].map(|(s, (wa, wb))| {
        let label = format!("{label}, s = {s}");
        let r = split(body, &plane(o, (s * n.0, s * n.1, s * n.2)), Tol::witness())
            .unwrap_or_else(|e| panic!("{label}: {e:?}"));
        for (side, got, want) in [
            ("above", volume(&label, &r.above), wa),
            ("below", volume(&label, &r.below), wb),
        ] {
            let ok = match (got, want) {
                (None, None) => true,
                (Some(g), Some(w)) => (g - w).abs() <= 1e-9 * w.max(1.0),
                _ => false,
            };
            assert!(ok, "{label}: {side} {got:?}, want {want:?}");
        }
        r
    })
}

fn block() -> AtRestBody<f64> {
    let tol = Tol::witness();
    finished(
        "the block",
        brick::<f64>((0.0, 1.5), (0.0, 1.0), (0.0, 1.0), tol),
        tol,
    )
}

/// The edges of `body` lying along y = z = 1, as sorted (x₀, x₁) spans.
fn rim_spans(body: &Body<f64>) -> Vec<(f64, f64)> {
    let point = |he| {
        let v = body.get_half_edge(he).unwrap().start;
        *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
    };
    let mut spans: Vec<(f64, f64)> = body
        .edges()
        .filter_map(|(_, e)| {
            let (p, q) = (point(e.he_plus), point(e.he_minus));
            (p.y == 1.0 && p.z == 1.0 && q.y == 1.0 && q.z == 1.0)
                .then(|| (p.x.min(q.x), p.x.max(q.x)))
        })
        .collect();
    spans.sort_by(|a, b| a.partial_cmp(b).unwrap());
    spans
}

/// The block alone: the whole block on its material side, `Empty` on
/// the other, the rim one ordinary edge, and no section region.
#[test]
fn a_tangent_contact_standing_alone_lands_whole() {
    let body = block();
    let [r, m] = both_ways(
        "block",
        &body,
        (0.0, 1.0, 1.0),
        (0.0, 1.0, 1.0),
        (None, Some(1.5)),
    );
    assert_eq!(rim_spans(r.below.body().unwrap()), vec![(0.0, 1.5)]);
    assert_eq!(rim_spans(m.above.body().unwrap()), vec![(0.0, 1.5)]);
    for s in [1.0, -1.0] {
        let p = plane((0.0, 1.0, 1.0), (0.0, s, s));
        let section = plane_section(&body, &p, Tol::witness()).unwrap();
        assert!(section.regions.is_empty(), "s = {s}");
    }
}

/// With the slab the contact reaches, in both operand orders: the
/// section is the slab's alone. The slab's part above the plane is
/// 0.1 × 4.375; the rest of the 2.2 is on the block's side, where the
/// rim survives as two ordinary edges, one each side of the slab.
#[test]
fn a_tangent_contact_meeting_a_real_section_cuts_only_the_slab() {
    let slab = finished(
        "the slab",
        brick::<f64>((1.2, 1.3), (-1.0, 2.0), (0.5, 3.0), Tol::witness()),
        Tol::witness(),
    );
    for (label, body) in [
        ("block ∪ slab", unite(&block(), &slab)),
        ("slab ∪ block", unite(&slab, &block())),
    ] {
        let [r, m] = both_ways(
            label,
            &body,
            (0.0, 1.0, 1.0),
            (0.0, 1.0, 1.0),
            (Some(0.4375), Some(1.7625)),
        );
        for (half, rest) in [(&r.below, &r.above), (&m.above, &m.below)] {
            assert_eq!(
                rim_spans(half.body().unwrap()),
                vec![(0.0, 1.2), (1.3, 1.5)],
                "{label}: the rim either side of the slab"
            );
            assert!(rim_spans(rest.body().unwrap()).is_empty(), "{label}");
        }
        let p = plane((0.0, 1.0, 1.0), (0.0, 1.0, 1.0));
        let section = plane_section(&body, &p, Tol::witness()).unwrap();
        assert_eq!(section.regions.len(), 1, "{label}: the slab's section");
    }
}

/// A step, profile area 6: convex corners at (4, 1) and (2, 2), reflex
/// at (2, 1). The plane x + y = 4 touches (2, 2) from above while
/// cutting the corner (3..4, 0..1) off, area 0.5; x + y = 5 touches
/// (4, 1) alone; x + y = 3 passes through the reflex corner.
#[test]
fn a_step_cut_along_each_of_its_edges() {
    let step = prism::<f64>(
        &[
            (0.0, 0.0),
            (4.0, 0.0),
            (4.0, 1.0),
            (2.0, 1.0),
            (2.0, 2.0),
            (0.0, 2.0),
        ],
        1.0,
        Tol::witness(),
    )
    .body;
    let n = (1.0, 1.0, 0.0);
    both_ways(
        "convex (2,2)",
        &step,
        (2.0, 2.0, 0.0),
        n,
        (Some(0.5), Some(5.5)),
    );
    both_ways("convex (4,1)", &step, (4.0, 1.0, 0.0), n, (None, Some(6.0)));
    both_ways(
        "reflex (2,1)",
        &step,
        (2.0, 1.0, 0.0),
        n,
        (Some(2.0), Some(4.0)),
    );
}

/// A ridge on a bar, area 9, apex (4, 2) convex. y = 2 touches the apex
/// alone; 0.5x + y = 4 touches it while cutting the bar's far end,
/// area 1.
#[test]
fn a_ridge_cut_along_its_apex() {
    let ridge = prism::<f64>(
        &[
            (0.0, 0.0),
            (8.0, 0.0),
            (8.0, 1.0),
            (5.0, 1.0),
            (4.0, 2.0),
            (3.0, 1.0),
            (0.0, 1.0),
        ],
        1.0,
        Tol::witness(),
    )
    .body;
    let o = (4.0, 2.0, 0.0);
    both_ways("apex alone", &ridge, o, (0.0, 1.0, 0.0), (None, Some(9.0)));
    both_ways(
        "apex and the bar's end",
        &ridge,
        o,
        (0.5, 1.0, 0.0),
        (Some(1.0), Some(8.0)),
    );
}

/// One plane, y = 1, along a convex peak at (5, 1) (material below) and
/// a reflex valley at (8, 1) (material on both sides), and cutting four
/// walls. Area 15, of which 17/3 is above.
#[test]
fn a_convex_and_a_reflex_edge_in_one_plane() {
    let saw = prism::<f64>(
        &[
            (0.0, 0.0),
            (10.0, 0.0),
            (10.0, 2.0),
            (9.0, 2.0),
            (8.0, 1.0),
            (7.0, 2.0),
            (6.0, 0.5),
            (5.0, 1.0),
            (4.0, 0.5),
            (3.0, 2.0),
            (0.0, 2.0),
        ],
        1.0,
        Tol::witness(),
    )
    .body;
    let want = (Some(17.0 / 3.0), Some(15.0 - 17.0 / 3.0));
    both_ways("sawtooth", &saw, (0.0, 1.0, 0.0), (0.0, 1.0, 0.0), want);
}

/// A = [0,2]×[0,1]×[0,1] ∪ B = [0.2,1]×[0.2,2]×[−0.5,1.5] (4.24, no
/// shared planes). x + y = 3 touches A's convex (2, 1) and B's convex
/// (1, 2) and nothing else; x + y = 2 runs along the union's own
/// reflex seam at (1, 1), above it 0.5 of A and 0.48 × 2 of B.
#[test]
fn a_union_cut_along_its_edges() {
    let t = Tol::witness();
    let a = finished("A", brick::<f64>((0.0, 2.0), (0.0, 1.0), (0.0, 1.0), t), t);
    let b = finished("B", brick::<f64>((0.2, 1.0), (0.2, 2.0), (-0.5, 1.5), t), t);
    let u = unite(&a, &b);
    let n = (1.0, 1.0, 0.0);
    both_ways(
        "two convex edges",
        &u,
        (2.0, 1.0, 0.0),
        n,
        (None, Some(4.24)),
    );
    both_ways(
        "reflex seam",
        &u,
        (1.0, 1.0, 0.0),
        n,
        (Some(1.46), Some(2.78)),
    );
}

/// Genus 1: the 4 × 2 × 2 block with a unit hole at (1.5..2.5,
/// 0.5..1.5). x + y = 2 runs along the hole's reflex corner edge; its
/// whole hole lies above, so above is (6 − 1) × 2. x + y = 6 touches
/// the outer convex corner (4, 2) alone. A convexity read that took the
/// reflex corner for convex returns closed halves of 12 and 2 here,
/// not a refusal.
#[test]
fn a_holed_block_cut_along_its_corner_edges() {
    let t = Tol::witness();
    let mut hb = holed_block::<f64>(4.0, &[2.0], t);
    describe_as_intersections(&mut hb, t);
    let n = (1.0, 1.0, 0.0);
    both_ways(
        "hole corner",
        &hb,
        (1.5, 0.5, 0.0),
        n,
        (Some(10.0), Some(4.0)),
    );
    both_ways("outer corner", &hb, (4.0, 2.0, 0.0), n, (None, Some(14.0)));
}
