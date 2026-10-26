//! **A plane face across a spline edge, through the boolean's sweep.**
//! The wavy loft of `editor-core`'s naming rows: 2 × 2 squares at
//! `z = 0, 1, 2, 3` whose x offsets alternate `−a, a, −a, a`
//! (`a = 0.3`), skinned at v-degree 2. Its corner edges are NURBS
//! curves, and the two at `x = dx + 1` wave about `x = 1`, where a box
//! over `x ∈ [1, 3]` stands its face. That face crosses each of them
//! three times, the first and third with one sense.
//!
//! The sweep reads each corner against the face through the curve ×
//! face table's ring-composite arm and splits it at all three
//! crossings, in either operand order. The union itself stops one step
//! on, at the sector walk around a crossing vertex between two spline
//! walls (`work/splice/the-sector-walk-reads-no-nurbs-face-at-a-crossing-vertex.md`),
//! which owes the row that names the crossings through the boolean.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol};
use profile::{ProfileLoop, RawLoop};
use sweep::test_support::{brick, stacked_at};
use topo::{Body, EdgeKey};

/// The wavy loft at offset `a`.
fn wavy(a: f64, tol: Tol) -> Body<f64> {
    let sections: Vec<Vec<ProfileLoop<f64>>> = [-a, a, -a, a]
        .into_iter()
        .map(|dx| {
            vec![ProfileLoop::polygon(
                [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
                    .into_iter()
                    .map(|(x, y)| Point2::new(dx + x, y)),
            )]
        })
        .collect();
    sweep::loft_body::<f64>(&sections, &stacked_at(&[0.0, 1.0, 2.0, 3.0]), 2, tol)
        .expect("the wavy loft skins")
        .body
}

fn carrier(body: &Body<f64>, e: EdgeKey) -> (&geom::Curve3<f64>, (f64, f64)) {
    let edge = body.get_edge(e).unwrap();
    let curve = body
        .get_curve_geom(edge.curve)
        .unwrap()
        .certified()
        .unwrap();
    (curve.carrier(), curve.params())
}

/// Where the corner's carrier crosses `x = 1`, with the sign of
/// `x − 1` after each: test-side root finding on a fine grid, bisected.
fn crossings_of(body: &Body<f64>, e: EdgeKey) -> Vec<(Point3<f64>, bool)> {
    let (c, (t0, t1)) = carrier(body, e);
    let f = |t: f64| c.eval(t).x - 1.0;
    let n = 4096;
    #[allow(clippy::cast_precision_loss)]
    let at = |k: usize| t0 + (t1 - t0) * k as f64 / n as f64;
    let mut roots = Vec::new();
    for k in 0..n {
        let (mut lo, mut hi) = (at(k), at(k + 1));
        if (f(lo) < 0.0) == (f(hi) < 0.0) {
            continue;
        }
        let rising = f(hi) > 0.0;
        for _ in 0..80 {
            let mid = (lo + hi) / 2.0;
            if (f(mid) > 0.0) == rising {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        roots.push((c.eval((lo + hi) / 2.0), rising));
    }
    roots
}

#[test]
fn a_box_face_splits_each_wavy_corner_at_its_three_crossings() {
    let tol = Tol::witness();
    let loft = wavy(0.3, tol);
    let block: Body<f64> = brick((1.0, 3.0), (-3.0, 3.0), (-0.5, 3.5), tol);
    let corners: Vec<EdgeKey> = loft
        .edges()
        .map(|(e, _)| e)
        .filter(|&e| {
            let (c, (t0, t1)) = carrier(&loft, e);
            matches!(c, geom::Curve3::Nurbs(_))
                && ((c.eval(t0).x + c.eval(t1).x) / 2.0 - 1.0).abs() < 1e-9
        })
        .collect();
    assert_eq!(
        corners.len(),
        2,
        "two corners wave about x = 1: {corners:?}"
    );
    let mut want = Vec::new();
    for &e in &corners {
        let roots = crossings_of(&loft, e);
        let senses: Vec<bool> = roots.iter().map(|&(_, s)| s).collect();
        assert!(
            senses.len() == 3 && senses[0] == senses[2] && senses[0] != senses[1],
            "x = 1 crosses corner {e:?} three times, the first and third with one sense: \
             {senses:?}"
        );
        want.extend(roots.into_iter().map(|(p, _)| p));
    }
    for (order, loft_first) in [("loft first", true), ("box first", false)] {
        let (a, b) = if loft_first {
            (&loft, &block)
        } else {
            (&block, &loft)
        };
        let (sa, sb, _, _) = topo::sweep_split(a, b, tol)
            .unwrap_or_else(|e| panic!("{order}: the sweep refused {e:?}"));
        let split = if loft_first { sa } else { sb };
        for p in &want {
            assert!(
                split.vertex_points().any(|(_, q)| (q - *p).norm() < 1e-9),
                "{order}: the loft is split at the crossing {p:?}"
            );
        }
    }
}
