//! Confirming-review probes for PR 4797 (predicate 2 reads three ways):
//! the request's one verdict across several chains, both convexities in
//! one request, edge order, fuse order, pose, and a mitre.
//!
//! Every void is a box `x × y × z` subtracted from the block `[0, 4]³`
//! (translated by `off` where a row moves the pose). At size `S = 0.25`
//! a convex band on the block's edge `y = z = 0` removes the material
//! `y, z ≥ 0` outside the ball's section about `(S, S)`: the section
//! `[0.015, 0.04]²` lies wholly inside it (its far corner `0.297` from
//! the axis), `[0.05, 0.15]²` crosses it (`0.283` and `0.141`), and
//! `[0.2, 0.22]²` lies in the kept material.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol};
use sweep::blend::{BlendError, fillet_edges};
use topo::{AtRestBody, Body, EdgeKey, EntityId};

use crate::common::cavity::{brick, cut, edges_with_corners};

fn tol() -> Tol {
    Tol::witness()
}

const S: f64 = 0.25;

type Span = (f64, f64);
type Void = (Span, Span, Span);

const INSIDE: Span = (0.015, 0.04);
const ACROSS: Span = (0.05, 0.15);
const KEPT: Span = (0.2, 0.22);

/// `4 − span`, reflected: the same section about the block's far edge.
fn far((a, b): Span) -> Span {
    (4.0 - b, 4.0 - a)
}

/// The block `[0, 4]³ + off` less each void, subtracted in order.
fn block_less(off: f64, voids: &[Void]) -> AtRestBody<f64> {
    let p = |x: f64, y: f64, z: f64| Point3::new(x + off, y + off, z + off);
    let mut body = brick(p(0.0, 0.0, 0.0), p(4.0, 4.0, 4.0));
    for (i, &(x, y, z)) in voids.iter().enumerate() {
        body = cut(
            &format!("void {i}"),
            &body,
            &brick(p(x.0, y.0, z.0), p(x.1, y.1, z.1)),
        );
    }
    sweep::test_support::finished("voided", body, tol())
}

/// The block's edge along `x` at `(y, z) = (w, w)` (+ `off`).
fn x_edge<T: geom_core::Bounds>(body: &Body<T>, off: f64, w: f64) -> EdgeKey {
    let at = |x: T, v: f64| x.lo() <= v + off + 1e-12 && v + off - 1e-12 <= x.hi();
    let e = edges_with_corners(body, |q| at(q.y, w) && at(q.z, w));
    assert_eq!(e.len(), 1, "the edge y = z = {w}");
    e[0]
}

fn kind(e: &Result<(), BlendError>) -> String {
    match e {
        Ok(()) => "built".into(),
        Err(BlendError::WhollyInside { at, margin, .. }) => {
            format!("WhollyInside({at:?}, {margin})")
        }
        Err(BlendError::FaceClearance {
            bounded, margin, ..
        }) => format!("FaceClearance(bounded={bounded}, {margin})"),
        Err(BlendError::RingClearance {
            bounded, margin, ..
        }) => format!("RingClearance(bounded={bounded}, {margin})"),
        Err(other) => format!("{other:?}"),
    }
}

fn fillet(body: &AtRestBody<f64>, edges: &[EdgeKey]) -> Result<(), BlendError> {
    fillet_edges(body, edges, S, tol())
        .map(|_| ())
        .map_err(|e| e.error)
}

fn is_crossing(e: &Result<(), BlendError>) -> bool {
    matches!(e, Err(BlendError::FaceClearance { bounded: false, .. }))
}

fn is_inside(e: &Result<(), BlendError>) -> bool {
    matches!(e, Err(BlendError::WhollyInside { .. }))
}

/// **Two chains that do not meet, one verdict.** The near edge
/// `y = z = 0` and the far edge `y = z = 4`, each with its own void at
/// `x ∈ [1.9, 2.1]` (near) and `[1.4, 1.6]` (far). Every pairing of
/// readings is run in both request orders, both fuse orders and two
/// poses: an inside void beside a crossing one is a crossing; beside a
/// kept one, or another inside one, `WhollyInside`; a kept pair builds.
#[test]
fn two_chains_make_one_verdict_in_every_order_and_pose() {
    let near = |s: Span| ((1.9, 2.1), s, s);
    let far_void = |s: Span| ((1.4, 1.6), far(s), far(s));
    let rows: [(&str, Span, Span, &str); 6] = [
        ("inside near, across far", INSIDE, ACROSS, "crossing"),
        ("across near, inside far", ACROSS, INSIDE, "crossing"),
        ("inside near, kept far", INSIDE, KEPT, "inside"),
        ("kept near, inside far", KEPT, INSIDE, "inside"),
        ("inside both", INSIDE, INSIDE, "inside"),
        ("kept both", KEPT, KEPT, "built"),
    ];
    let mut bad = Vec::new();
    for (what, a, b, want) in rows {
        for off in [0.0, 7.25] {
            for fuse_far_first in [false, true] {
                let voids = if fuse_far_first {
                    [far_void(b), near(a)]
                } else {
                    [near(a), far_void(b)]
                };
                let body = block_less(off, &voids);
                let (e0, e4) = (x_edge(&body, off, 0.0), x_edge(&body, off, 4.0));
                for edges in [[e0, e4], [e4, e0]] {
                    let got = fillet(&body, &edges);
                    let ok = match want {
                        "crossing" => is_crossing(&got),
                        "inside" => is_inside(&got),
                        _ => got.is_ok(),
                    };
                    eprintln!(
                        "{what} off={off} far_first={fuse_far_first} {:?}: {}",
                        edges == [e4, e0],
                        kind(&got)
                    );
                    if !ok {
                        bad.push(format!(
                            "{what} off={off} far_first={fuse_far_first}: want {want}, got {}",
                            kind(&got)
                        ));
                    }
                }
            }
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// **Both convexities in one request.** A sealed cavity `[1, 3]³` in
/// the block, its floor edge `y = z = 1` (concave) requested with the
/// block's edge `y = z = 0` (convex). An island in the cavity's corner
/// is inside the fill (`[1.015, 1.04]²`) or crosses it
/// (`[1.05, 1.15]²`); a void at the convex edge is inside or crosses
/// the removed material. An inside reading on either chain beside a
/// crossing on the other is a crossing, in either request order; two
/// inside readings are `WhollyInside`.
#[test]
fn both_convexities_in_one_request_make_one_verdict() {
    let p = Point3::new;
    let shift = |(a, b): Span| (a + 1.0, b + 1.0);
    let rows: [(&str, Span, Span, &str); 3] = [
        ("island inside, void across", INSIDE, ACROSS, "crossing"),
        ("island across, void inside", ACROSS, INSIDE, "crossing"),
        ("island inside, void inside", INSIDE, INSIDE, "inside"),
    ];
    let mut bad = Vec::new();
    for (what, island, void, want) in rows {
        let block = brick(p(0.0, 0.0, 0.0), p(4.0, 4.0, 4.0));
        let block = cut(
            "void",
            &block,
            &brick(p(1.9, void.0, void.0), p(2.1, void.1, void.1)),
        );
        let sealed = sweep::test_support::finished(
            "sealed",
            cut("cavity", &block, &brick(p(1.0, 1.0, 1.0), p(3.0, 3.0, 3.0))),
            tol(),
        );
        let (iy, iz) = (shift(island), shift(island));
        let island = sweep::test_support::finished(
            "island",
            brick(p(1.4, iy.0, iz.0), p(1.6, iy.1, iz.1)),
            tol(),
        );
        let body = topo::union(&sealed, &island, tol())
            .expect("the union succeeds")
            .body()
            .expect("material")
            .body
            .clone();
        let concave = edges_with_corners(&body, |q| {
            (q.y - 1.0).abs() < 1e-12 && (q.z - 1.0).abs() < 1e-12 && (0.5..3.5).contains(&q.x)
        });
        assert_eq!(concave.len(), 1, "{what}: the floor edge");
        let convex = x_edge(&body, 0.0, 0.0);
        for edges in [[convex, concave[0]], [concave[0], convex]] {
            let got = fillet(&body, &edges);
            eprintln!("{what} convex_first={}: {}", edges[0] == convex, kind(&got));
            let ok = match want {
                "crossing" => is_crossing(&got),
                _ => is_inside(&got),
            };
            if !ok {
                bad.push(format!(
                    "{what} convex_first={}: want {want}, got {}",
                    edges[0] == convex,
                    kind(&got)
                ));
            }
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// **A mitre: a void wholly inside one band's material that crosses the
/// other's.** The block's edges `y = z = 0` (along `x`) and
/// `x = y = 0` (along `z`) meet at the origin with the third edge
/// unnamed. The void `[0.05, 0.12] × INSIDE × INSIDE` lies wholly inside
/// the `x` band's removed material, and crosses the `z` band's (its
/// corner `(0.05, 0.015)` is in it, `(0.12, 0.04)` is `0.247` from that
/// band's axis, in the kept ball). Under the ruling the convex union removes it, so
/// `WhollyInside` or a bound are both sound; it must neither build nor
/// fail untyped.
#[test]
fn a_mitre_void_inside_one_band_and_across_the_other_refuses_typed() {
    let void = ((0.05, 0.12), INSIDE, INSIDE);
    let body = block_less(0.0, &[void]);
    let ex = x_edge(&body, 0.0, 0.0);
    let ez = edges_with_corners(&body, |q| q.x.abs() < 1e-12 && q.y.abs() < 1e-12);
    assert_eq!(ez.len(), 1, "the edge x = y = 0");
    for edges in [[ex, ez[0]], [ez[0], ex]] {
        let got = fillet(&body, &edges);
        eprintln!("mitre x_first={}: {}", edges[0] == ex, kind(&got));
        assert!(
            matches!(
                got,
                Err(BlendError::WhollyInside { .. } | BlendError::FaceClearance { .. })
            ),
            "mitre x_first={}: {}",
            edges[0] == ex,
            kind(&got)
        );
        if let Err(BlendError::WhollyInside { at, .. }) = &got {
            assert!(matches!(at, EntityId::Face(_)), "names a face: {at:?}");
        }
    }
}
