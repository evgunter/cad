//! **A boolean records each coincident face pair whose region it holds
//! through one face only** (`BooleanNaming::covered`).
//!
//! `b` = x 0.5..1.5 lies inside `a` = x 0..2, both over y 0..1 and z 0..1,
//! flush on both caps and both y-walls. With `a` as operand A the union
//! keeps `a`'s copies, cuts nothing and is `a`, by the containment
//! fallback; with `b` as operand A it keeps `b`'s copies and takes the
//! section path. Raised to z 0..2, `b` pokes out of `a`'s top and the
//! union takes the section path in either order; its bottom and y-walls
//! stay flush.
//! Every flush pair is declared. On either path, and with either block as
//! operand A, each flush pair is recorded once, as `(A face, B face)`,
//! whichever copy the classification kept.
//!
//! A discarded face that a kept face holds part of records the kept
//! face's edges that run into it (`DiscardRow::held`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::common;

use common::{brick, finished};
use geom_core::Tol;
use topo::flush::{declare_all, find_flush_candidates};
use topo::{
    AtRestBody, Body, BooleanResult, BooleanResultKind, CarrierDesc, FaceKey, face_carrier,
    union_with,
};

/// A face's outward normal, rounded to a unit axis.
type Axis = [i8; 3];

/// Two faces' axes, operand A's then operand B's.
type AxisPair = (Axis, Axis);

/// An edge's two ends in thousandths, sorted.
type Ends = [[i64; 3]; 2];

/// The outward normal of `body`'s face `f`, rounded to a unit axis.
fn normal(body: &Body<f64>, f: FaceKey) -> Axis {
    let Some(CarrierDesc::Plane { normal, .. }) = face_carrier(body, f) else {
        panic!("face {f:?} is not a plane of its operand");
    };
    [normal.x, normal.y, normal.z].map(|c| c.round() as i8)
}

/// The union of `x` and `y` with every flush pair declared: its kind and
/// the normals of each covered pair, operand A's then operand B's.
fn covered(x: &AtRestBody<f64>, y: &AtRestBody<f64>) -> (BooleanResultKind, BTreeSet<AxisPair>) {
    let tol = Tol::witness();
    let decls = declare_all(&find_flush_candidates(x, y, tol).expect("the flush detector decides"));
    let BooleanResult::Body(out) = union_with(x, y, &decls, tol).expect("the union fuses") else {
        panic!("a union of non-empty blocks cannot be empty");
    };
    let pairs: BTreeSet<_> = out
        .naming
        .covered
        .iter()
        .map(|&(fa, fb)| (normal(x, fa), normal(y, fb)))
        .collect();
    assert_eq!(
        pairs.len(),
        out.naming.covered.len(),
        "one row per coincident pair"
    );
    (out.kind, pairs)
}

/// Same-normal pairs on each of `normals`.
fn flush_on(normals: &[Axis]) -> BTreeSet<AxisPair> {
    normals.iter().map(|&n| (n, n)).collect()
}

#[test]
fn a_union_records_each_flush_pair_it_holds_through_one_face_in_either_operand_order() {
    let tol = Tol::witness();
    let unit = (0.0, 1.0);
    let a = finished("a", brick((0.0, 2.0), unit, unit, tol), tol);
    let inside = finished("inside", brick((0.5, 1.5), unit, unit, tol), tol);
    let poking = finished("poking", brick((0.5, 1.5), unit, (0.0, 2.0), tol), tol);
    let walls_and_caps = flush_on(&[[0, -1, 0], [0, 1, 0], [0, 0, -1], [0, 0, 1]]);
    let walls_and_bottom = flush_on(&[[0, -1, 0], [0, 1, 0], [0, 0, -1]]);
    for (label, x, y, kind, want) in [
        (
            "a ∪ inside",
            &a,
            &inside,
            BooleanResultKind::OperandA,
            &walls_and_caps,
        ),
        (
            "inside ∪ a",
            &inside,
            &a,
            BooleanResultKind::Seamed,
            &walls_and_caps,
        ),
        (
            "a ∪ poking",
            &a,
            &poking,
            BooleanResultKind::Seamed,
            &walls_and_bottom,
        ),
        (
            "poking ∪ a",
            &poking,
            &a,
            BooleanResultKind::Seamed,
            &walls_and_bottom,
        ),
    ] {
        let (got_kind, got) = covered(x, y);
        assert_eq!(got_kind, kind, "{label}: the path taken");
        assert_eq!(&got, want, "{label}: the covered pairs");
    }
}

/// `b` = x 0.5..1.5 cut by a slab over x 0.499..0.501 from z 0.5 up,
/// then united with `a` = x 0..1 flush on both caps and y-walls: `a`'s
/// top is discarded from x 0.499 on, and from x 0.501 `b`'s cut top
/// holds it. The edge of `b`'s top along the slab at x 0.501 runs into
/// it, and on each y-wall so do the two edges of the notch the slab cut
/// in `b`'s wall.
#[test]
fn a_discarded_face_holds_the_edges_of_the_kept_face_that_runs_into_it() {
    let tol = Tol::witness();
    let unit = (0.0, 1.0);
    let a = finished("a", brick::<f64>((0.0, 1.0), unit, unit, tol), tol);
    let b = finished("b", brick::<f64>((0.5, 1.5), unit, unit, tol), tol);
    let slab = finished(
        "slab",
        brick((0.499, 0.501), (-1.0, 2.0), (0.5, 2.0), tol),
        tol,
    );
    let BooleanResult::Body(cut) =
        union_with(&b, &slab, &topo::BooleanDeclarations::default(), tol).expect("b ∪ slab")
    else {
        panic!("a union of non-empty blocks cannot be empty");
    };
    let decls = declare_all(
        &find_flush_candidates(&cut.body, &a, tol).expect("the flush detector decides"),
    );
    let BooleanResult::Body(out) = union_with(&cut.body, &a, &decls, tol).expect("the union fuses")
    else {
        panic!("a union of non-empty blocks cannot be empty");
    };
    // Each holding row: `a`'s face's normal, and each held edge's ends,
    // sorted.
    let mut got: Vec<(Axis, Vec<Ends>)> = out
        .naming
        .discards
        .iter()
        .filter(|r| !r.held.is_empty())
        .map(|r| (normal(&a, a_face(&out, r.face)), stretches(&out, &r.held)))
        .collect();
    got.sort();
    assert_eq!(
        got,
        vec![
            (
                [0, -1, 0],
                vec![
                    [[500, 0, 0], [500, 0, 500]],
                    [[501, 0, 500], [501, 0, 1000]]
                ]
            ),
            ([0, 0, -1], vec![[[500, 0, 0], [500, 1000, 0]]]),
            ([0, 0, 1], vec![[[501, 0, 1000], [501, 1000, 1000]]]),
            (
                [0, 1, 0],
                vec![
                    [[500, 1000, 0], [500, 1000, 500]],
                    [[501, 1000, 500], [501, 1000, 1000]]
                ]
            ),
        ],
        "each face of `a` that `b` covers holds `b`'s edges entering it at a shared vertex"
    );
}

/// Each stretch's two ends in thousandths, sorted, read through the
/// zip's fusions.
fn stretches(
    out: &topo::BooleanBody<f64>,
    rows: &[(topo::VertexKey, topo::VertexKey)],
) -> Vec<Ends> {
    let fused = out.naming.fused_into();
    let at = |v| {
        let v = fused.get(&v).copied().unwrap_or(v);
        let p = topo::readback::vertex_point(&out.body, v).expect("a held end is live");
        [p.x, p.y, p.z].map(|c| (c * 1000.0).round() as i64)
    };
    let mut out: Vec<Ends> = rows
        .iter()
        .map(|&(u, w)| {
            let mut e = [at(u), at(w)];
            e.sort();
            e
        })
        .collect();
    out.sort();
    out
}

/// The operand face of `a` (operand B) that discarded clone face `f` is
/// a fragment of.
fn a_face(out: &topo::BooleanBody<f64>, f: FaceKey) -> FaceKey {
    let rows = &out.naming.face_fragments_b;
    topo::lineage_root(f, rows.len(), |k| {
        rows.iter().find(|(new, _)| *new == k).map(|&(_, up)| up)
    })
    .expect("the fragment rows do not cycle")
}

/// **A held edge goes only to the fragment it enters.** As above, with
/// a pillar over x 0.1..0.2, y 0.4..0.6 standing on `a`'s top, joined
/// to the slab by a bridge above it. `a`'s top is discarded in two
/// fragments: the strip from the slab on, and the pillar's footprint.
/// The edge of `b`'s top at x 0.501 enters the strip at its corners and
/// is held there; the footprint, a fragment of the same face that the
/// edge never reaches, holds nothing.
#[test]
fn a_held_edge_goes_only_to_the_fragment_it_enters() {
    let tol = Tol::witness();
    let unit = (0.0, 1.0);
    let a = finished("a", brick::<f64>((0.0, 1.0), unit, unit, tol), tol);
    let b = finished("b", brick::<f64>((0.5, 1.5), unit, unit, tol), tol);
    let slab = finished(
        "slab",
        brick((0.499, 0.501), (-1.0, 2.0), (0.5, 2.0), tol),
        tol,
    );
    let pillar = finished(
        "pillar",
        brick((0.1, 0.2), (0.4, 0.6), (0.5, 2.0), tol),
        tol,
    );
    let bridge = finished(
        "bridge",
        brick((0.15, 0.55), (0.45, 0.55), (1.5, 1.8), tol),
        tol,
    );
    let none = topo::BooleanDeclarations::default();
    let union = |x: &AtRestBody<f64>, y: &AtRestBody<f64>| -> AtRestBody<f64> {
        let BooleanResult::Body(o) =
            union_with(x, y, &none, tol).expect("an undeclared union fuses")
        else {
            panic!("a union of non-empty blocks cannot be empty");
        };
        o.body
    };
    let x = union(&union(&union(&b, &slab), &bridge), &pillar);
    let decls =
        declare_all(&find_flush_candidates(&x, &a, tol).expect("the flush detector decides"));
    let BooleanResult::Body(out) = union_with(&x, &a, &decls, tol).expect("the union fuses") else {
        panic!("a union of non-empty blocks cannot be empty");
    };
    // `a`'s top's discarded fragments: how many stretches each borders
    // a kept face along, and what it holds.
    let mut tops: Vec<(usize, Vec<Ends>)> = out
        .naming
        .discards
        .iter()
        .filter(|r| r.operand == topo::Operand::B)
        .filter(|r| normal(&a, a_face(&out, r.face)) == [0, 0, 1])
        .map(|r| (r.bordered.len(), stretches(&out, &r.held)))
        .collect();
    tops.sort();
    assert_eq!(
        tops,
        vec![(1, vec![[[501, 0, 1000], [501, 1000, 1000]]]), (4, vec![]),],
        "the strip holds the edge at x 0.501; the pillar's footprint holds nothing"
    );
}

/// **Faces on one plane that share no region are not covered.** `a` =
/// [0,1]³ and a block on the same caps meeting it along a vertical edge,
/// or standing apart beside it: the caps are coplanar and
/// same-oriented, but neither holds any of the other's region, so the
/// union records no covered pair.
#[test]
fn coplanar_faces_that_share_no_region_are_not_covered() {
    let tol = Tol::witness();
    let unit = (0.0, 1.0);
    let a = finished("a", brick::<f64>(unit, unit, unit, tol), tol);
    for (label, other) in [
        ("edge", brick((1.0, 2.0), (1.0, 2.0), unit, tol)),
        ("apart", brick((2.0, 3.0), unit, unit, tol)),
    ] {
        let other = finished(label, other, tol);
        let decls = declare_all(
            &find_flush_candidates(&a, &other, tol).expect("the flush detector decides"),
        );
        let BooleanResult::Body(out) =
            union_with(&a, &other, &decls, tol).expect("the union fuses")
        else {
            panic!("{label}: a union of non-empty blocks cannot be empty");
        };
        assert_eq!(out.naming.covered, vec![], "{label}: covered pairs");
    }
}
