//! **A boolean divides a coincident face where its region's fate
//! changes** (`boolean::fate`): where the Eq. 15.3 lump keeps one
//! operand's copy of a coincident region, the dropped copy is cut along
//! the kept copy's edges where the held region ends. The fragment the
//! kept face holds is recorded as substituted by it
//! (`BooleanNaming::substituted`) and is no discard; the fragment the
//! result loses is a discard that borders the kept face there
//! (`DiscardRow::bordered`).
//!
//! `b` = x 0.5..1.5 lies inside `a` = x 0..2, both over y 0..1 and z 0..1,
//! flush on both caps and both y-walls. With `a` as operand A the union
//! keeps `a`'s copies, cuts nothing and is `a`, by the containment
//! fallback; with `b` as operand A it keeps `b`'s copies and takes the
//! section path. Raised to z 0..2, `b` pokes out of `a`'s top and the
//! union takes the section path in either order; its bottom and y-walls
//! stay flush. On either path, and with either block as operand A, each
//! flush pair is recorded, whichever copy the classification kept.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::common;

use common::{brick, finished};
use geom_core::Tol;
use topo::{
    AtRestBody, Body, BooleanBody, BooleanResult, BooleanResultKind, CarrierDesc, FaceKey, Operand,
    face_carrier, mass_properties, union,
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

/// The plane offset of `body`'s face `f` along its rounded normal, in
/// thousandths.
fn level(body: &Body<f64>, f: FaceKey) -> i64 {
    let Some(CarrierDesc::Plane { origin, normal }) = face_carrier(body, f) else {
        panic!("face {f:?} is not a plane of its operand");
    };
    let n = [normal.x, normal.y, normal.z].map(f64::round);
    ((origin.x * n[0] + origin.y * n[1] + origin.z * n[2]) * 1000.0).round() as i64
}

/// The operand face `f`, a clone face of `operand`, is a fragment of.
fn root(out: &BooleanBody<f64>, operand: Operand, f: FaceKey) -> FaceKey {
    match operand {
        Operand::A => out.naming.face_fragments_a.root(f),
        Operand::B => out.naming.face_fragments_b.root(f),
    }
}

/// The union of `x` and `y`: its kind and the normals of each
/// substituted fragment's operand face and of the face holding it,
/// operand A's then operand B's.
fn substituted(
    x: &AtRestBody<f64>,
    y: &AtRestBody<f64>,
) -> (BooleanResultKind, BTreeSet<AxisPair>) {
    let tol = Tol::witness();
    let BooleanResult::Body(out) = union(x, y, tol).expect("the union fuses") else {
        panic!("a union of non-empty blocks cannot be empty");
    };
    let pairs = out
        .naming
        .substituted
        .iter()
        .map(|s| {
            let (fa, fb) = match s.operand {
                Operand::A => (s.face, s.by),
                Operand::B => (s.by, s.face),
            };
            (
                normal(x, root(&out, Operand::A, fa)),
                normal(y, root(&out, Operand::B, fb)),
            )
        })
        .collect();
    (out.kind, pairs)
}

/// Same-normal pairs on each of `normals`.
fn flush_on(normals: &[Axis]) -> BTreeSet<AxisPair> {
    normals.iter().map(|&n| (n, n)).collect()
}

#[test]
fn a_union_records_each_flush_face_it_holds_through_another_in_either_operand_order() {
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
        let (got_kind, got) = substituted(x, y);
        assert_eq!(got_kind, kind, "{label}: the path taken");
        assert_eq!(&got, want, "{label}: the substituted faces");
    }
}

/// Each stretch's live edge, by its two ends in thousandths, sorted:
/// the edge between its ends once settled (`BooleanNaming::settler`),
/// or the edge a join made of it. A stretch no live edge holds merged
/// away with the faces beside it, and is left out.
fn stretches(out: &BooleanBody<f64>, rows: &[(topo::VertexKey, topo::VertexKey)]) -> Vec<Ends> {
    let body = &out.body;
    let settler = out.naming.settler(body);
    let settle = |v| settler.settle(v).expect("every end settles on a live cell");
    let at = |v| {
        let p = topo::readback::vertex_point(body, v).expect("an edge's end is live");
        [p.x, p.y, p.z].map(|c| (c * 1000.0).round() as i64)
    };
    let ends = |e: topo::EdgeKey| {
        let d = body.get_edge(e).expect("a live edge");
        [d.he_plus, d.he_minus].map(|h| body.get_half_edge(h).expect("a live half-edge").start)
    };
    let mut out: Vec<Ends> = rows
        .iter()
        .filter_map(|&(u, w)| {
            let (u, w) = (settle(u), settle(w));
            let between = match (u, w) {
                (topo::Cell::Vertex(u), topo::Cell::Vertex(w)) => {
                    body.edges().map(|(k, _)| k).find(|&k| {
                        let [s, t] = ends(k);
                        (s, t) == (u, w) || (s, t) == (w, u)
                    })
                }
                // An end inside a joined edge or a merged face ends no edge.
                _ => None,
            };
            let edge = between.or_else(|| out.naming.stretch_through_joins(body, (u, w)))?;
            let mut e = ends(edge).map(at);
            e.sort();
            Some(e)
        })
        .collect();
    out.sort();
    out
}

/// `b` = x 0.5..1.5 cut by a slab over x 0.499..0.501 from z 0.5 up,
/// then united with `a` = x 0..1 flush on both caps and y-walls. `a`'s
/// top is discarded from x 0.499 on: the slab removes it to x 0.501, and
/// from there `b`'s cut top holds it, so the top is cut along `b`'s edge
/// at x 0.501, and on each y-wall along the two edges of the notch the
/// slab cut in `b`'s wall. Each removed fragment borders the kept face
/// along those edges; the held fragments are no discards. Red before
/// the cut: the removed region and the held one were one discarded
/// fragment, and the edges were recorded on the side.
#[test]
fn a_dropped_face_is_cut_where_the_kept_face_holding_it_ends() {
    let tol = Tol::witness();
    let unit = (0.0, 1.0);
    let a = finished("a", brick::<f64>((0.0, 1.0), unit, unit, tol), tol);
    let b = finished("b", brick::<f64>((0.5, 1.5), unit, unit, tol), tol);
    let slab = finished(
        "slab",
        brick((0.499, 0.501), (-1.0, 2.0), (0.5, 2.0), tol),
        tol,
    );
    let BooleanResult::Body(cut) = union(&b, &slab, tol).expect("b ∪ slab") else {
        panic!("a union of non-empty blocks cannot be empty");
    };
    let BooleanResult::Body(out) = union(&cut.body, &a, tol).expect("the union fuses") else {
        panic!("a union of non-empty blocks cannot be empty");
    };
    // Each of `a`'s discarded fragments: its face's normal, and the edges
    // it borders the kept faces along.
    let mut got: Vec<(Axis, Vec<Ends>)> = out
        .naming
        .discards
        .iter()
        .filter(|r| r.operand == Operand::B)
        .map(|r| {
            (
                normal(&a, root(&out, Operand::B, r.face)),
                stretches(&out, &r.bordered),
            )
        })
        .collect();
    got.sort();
    let notch = |y| {
        vec![
            [[499, y, 500], [499, y, 1000]],
            [[499, y, 500], [501, y, 500]],
            [[499, y, 500], [501, y, 500]],
            [[501, y, 500], [501, y, 1000]],
        ]
    };
    assert_eq!(
        got,
        vec![
            ([0, -1, 0], notch(0)),
            (
                [0, 0, 1],
                vec![
                    [[499, 0, 1000], [499, 1000, 1000]],
                    [[501, 0, 1000], [501, 1000, 1000]]
                ]
            ),
            ([0, 1, 0], notch(1000)),
            ([1, 0, 0], vec![]),
        ],
        "each removed fragment of `a`'s faces borders the slab at x 0.499 and `b` at x 0.501"
    );
    for s in &out.naming.substituted {
        assert_eq!(s.operand, Operand::B, "the kept copies are `b`'s");
        assert!(
            out.naming
                .discards
                .iter()
                .all(|r| (r.operand, r.face) != (s.operand, s.face)),
            "a held fragment is no discard: {s:?}"
        );
    }
}

/// **A cut goes only into the fragment it lies in.** As above, with a
/// pillar over x 0.1..0.2, y 0.4..0.6 standing on `a`'s top, joined to
/// the slab by a bridge above it. `a`'s top is discarded in two places:
/// the strip from the slab on, and the pillar's footprint. The strip is
/// cut at x 0.501, where `b`'s top holds it; the footprint, which no
/// kept copy holds, is cut nowhere and borders only the pillar.
#[test]
fn a_cut_goes_only_into_the_fragment_it_lies_in() {
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
    let fused = |x: &AtRestBody<f64>, y: &AtRestBody<f64>| -> AtRestBody<f64> {
        let BooleanResult::Body(o) = union(x, y, tol).expect("the union fuses") else {
            panic!("a union of non-empty blocks cannot be empty");
        };
        o.body
    };
    let x = fused(&fused(&fused(&b, &slab), &bridge), &pillar);
    let BooleanResult::Body(out) = union(&x, &a, tol).expect("the union fuses") else {
        panic!("a union of non-empty blocks cannot be empty");
    };
    let mut tops: Vec<Vec<Ends>> = out
        .naming
        .discards
        .iter()
        .filter(|r| r.operand == Operand::B)
        .filter(|r| normal(&a, root(&out, Operand::B, r.face)) == [0, 0, 1])
        .map(|r| stretches(&out, &r.bordered))
        .collect();
    tops.sort();
    assert_eq!(
        tops,
        vec![
            vec![
                [[100, 400, 1000], [100, 600, 1000]],
                [[100, 400, 1000], [200, 400, 1000]],
                [[100, 600, 1000], [200, 600, 1000]],
                [[200, 400, 1000], [200, 600, 1000]],
            ],
            vec![
                [[499, 0, 1000], [499, 1000, 1000]],
                [[501, 0, 1000], [501, 1000, 1000]]
            ],
        ],
        "the footprint borders the pillar; the strip, the slab and `b`'s top"
    );
}

/// **Faces on one plane that share no region are not substituted.**
/// `a` = [0,1]³ and a block on the same caps meeting it along a vertical
/// edge, or standing apart beside it: the caps are coplanar and
/// same-oriented, but neither holds any of the other's region, so the
/// union records no substituted face.
#[test]
fn coplanar_faces_that_share_no_region_are_not_substituted() {
    let tol = Tol::witness();
    let unit = (0.0, 1.0);
    let a = finished("a", brick::<f64>(unit, unit, unit, tol), tol);
    for (label, other) in [
        ("edge", brick((1.0, 2.0), (1.0, 2.0), unit, tol)),
        ("apart", brick((2.0, 3.0), unit, unit, tol)),
    ] {
        let other = finished(label, other, tol);
        let BooleanResult::Body(out) = union(&a, &other, tol).expect("the union fuses") else {
            panic!("{label}: a union of non-empty blocks cannot be empty");
        };
        assert_eq!(out.naming.substituted, vec![], "{label}: substituted faces");
    }
}

/// What one union says about its coincident region, up to which
/// operand is A: each substituted fragment's operand face and the face
/// holding it, by normal and level, as an unordered pair; and each
/// discard's operand face with the edges it borders the kept faces
/// along, where it borders any. (The containment fallback records no
/// discard, so a discard that borders nothing would tell the paths
/// apart, not the operand orders.)
#[derive(Debug, PartialEq, Eq)]
struct Fates {
    volume: i64,
    held: BTreeSet<((Axis, i64), (Axis, i64))>,
    lost: BTreeSet<((Axis, i64), Vec<Ends>)>,
}

/// [`Fates`] of `x ∪ y`.
fn fates(x: &AtRestBody<f64>, y: &AtRestBody<f64>) -> Fates {
    let tol = Tol::witness();
    let BooleanResult::Body(out) = union(x, y, tol).expect("the union fuses") else {
        panic!("a union of non-empty blocks cannot be empty");
    };
    let face = |operand: Operand, f: FaceKey| {
        let (body, f) = match operand {
            Operand::A => (x, root(&out, Operand::A, f)),
            Operand::B => (y, root(&out, Operand::B, f)),
        };
        (normal(body, f), level(body, f))
    };
    Fates {
        volume: (mass_properties(&out.body, tol).expect("mass").volume * 1e6).round() as i64,
        held: out
            .naming
            .substituted
            .iter()
            .map(|s| {
                let pair = [face(s.operand, s.face), face(s.operand.other(), s.by)];
                (pair[0].min(pair[1]), pair[0].max(pair[1]))
            })
            .collect(),
        lost: out
            .naming
            .discards
            .iter()
            .map(|r| (face(r.operand, r.face), stretches(&out, &r.bordered)))
            .filter(|(_, edges)| !edges.is_empty())
            .collect(),
    }
}

/// **A pocket floor flush with a bar's top cuts the bar's top in either
/// order.** `a` is the plate [0,3]×[0,2]×[0,1] less the pocket
/// [1,2]×[0.8,1.2]×[0.5,2], whose floor at z 0.5 is flush with the top
/// of the bar `b` = [0.5,2.5]×[−1,3]×[0,0.5], which is wider than it.
/// The floor's corners touch the bar's top with no surgery when `a` is
/// operand A, and pierce it when `b` is. Either way the dropped copy is
/// cut along the floor's outline: the part under the floor is held
/// through the kept one, and the rest under the plate is lost and
/// borders the outline. Red before the cut: with `a` as A, the bar's top
/// under the plate was one discarded fragment, bordering no floor edge.
#[test]
fn a_pocket_floor_flush_with_a_wider_top_is_cut_alike_in_either_order() {
    let tol = Tol::witness();
    let plate = finished(
        "plate",
        brick::<f64>((0.0, 3.0), (0.0, 2.0), (0.0, 1.0), tol),
        tol,
    );
    let pocket = finished(
        "pocket",
        brick::<f64>((1.0, 2.0), (0.8, 1.2), (0.5, 1.5), tol),
        tol,
    );
    let BooleanResult::Body(a) = topo::subtract(&plate, &pocket, tol).expect("plate ∖ pocket")
    else {
        panic!("the plate less the pocket is not empty");
    };
    let b = finished(
        "b",
        brick::<f64>((0.5, 2.5), (-1.0, 3.0), (0.0, 0.5), tol),
        tol,
    );
    let (ab, ba) = (fates(&a.body, &b), fates(&b, &a.body));
    assert_eq!(ab, ba, "a ∪ b against b ∪ a");
    let floor = [
        [[1000, 800, 500], [1000, 1200, 500]],
        [[1000, 800, 500], [2000, 800, 500]],
        [[1000, 1200, 500], [2000, 1200, 500]],
        [[2000, 800, 500], [2000, 1200, 500]],
    ];
    assert!(
        ab.lost
            .iter()
            .any(|(f, edges)| *f == ([0, 0, 1], 500) && floor.iter().all(|e| edges.contains(e))),
        "the top lost under the plate borders the floor's outline: {:?}",
        ab.lost
    );
    assert!(
        ab.held.contains(&(([0, 0, 1], 500), ([0, 0, 1], 500))),
        "the part of one top under the floor is held through the other: {:?}",
        ab.held
    );
}

/// **A box sunk flush in a block, either order.** The box
/// [1,2]×[1,2]×[0.5,1] lies in the block [0,3]×[0,3]×[0,1], its top
/// flush with the block's: the union is the block, and its coincident
/// tops are held and lost alike in either order.
#[test]
fn a_box_sunk_flush_in_a_block_unites_alike_in_either_order() {
    let tol = Tol::witness();
    let block = finished(
        "block",
        brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 1.0), tol),
        tol,
    );
    let sunk = finished(
        "box",
        brick::<f64>((1.0, 2.0), (1.0, 2.0), (0.5, 1.0), tol),
        tol,
    );
    let (ab, ba) = (fates(&block, &sunk), fates(&sunk, &block));
    assert_eq!(ab.volume, 9_000_000, "the union is the block");
    assert_eq!(ab, ba, "block ∪ box against box ∪ block");
}

/// **Equal-height blocks overlapping in part, either order.**
/// [0,2]²×[0,1] and [1,3]²×[0,1]: their tops and bottoms coincide over
/// [1,2]², each held there through the other's copy whichever is A.
#[test]
fn equal_height_blocks_overlapping_in_part_unite_alike_in_either_order() {
    let tol = Tol::witness();
    let x = finished(
        "x",
        brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0), tol),
        tol,
    );
    let y = finished(
        "y",
        brick::<f64>((1.0, 3.0), (1.0, 3.0), (0.0, 1.0), tol),
        tol,
    );
    let (xy, yx) = (fates(&x, &y), fates(&y, &x));
    assert_eq!(xy.volume, 7_000_000, "the union's volume");
    assert_eq!(xy, yx, "x ∪ y against y ∪ x");
}
