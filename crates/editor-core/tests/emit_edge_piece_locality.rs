//! **A cut elsewhere on an edge leaves its other pieces' names as they
//! were** (N2's `Ends`, the locality the ruled rule buys). A piece is
//! named by its two ends, so a cut that does not touch it neither moves
//! its ends nor renames it; a rank over the pieces would have written
//! the group's new size into every one of them. The limit of that is a
//! cut by a face that already crosses the edge: crossings keep an
//! ordinal, so there the first crossing and the pieces ending at it are
//! renamed.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, failure, run};
use crate::fixture::{ang, insert, len, point, scl, step, table};
use editor_core::{
    Axis3, BooleanOp, DocEdit, EntityKey, Entry, Node, ProfileDoc, Qualifier, RoleSeg, SlotId,
    StableName,
};
use geom_core::Tol;

/// The pieces of the seam the cut leaves along y = 1, z = 1, by their
/// x-span (to 1e-6).
fn seam_pieces(
    ev: &editor_core::Evaluation<f64>,
    cut: editor_core::RecipeNodeId,
) -> BTreeMap<(i64, i64), StableName> {
    let body = body_of(ev, cut);
    let micro = |x: f64| (x * 1e6).round() as i64;
    let mut out = BTreeMap::new();
    for (name, entry) in table(ev, cut).iter() {
        let (Entry::Unique(r), Some(RoleSeg::Fragment(Qualifier::Ends(_)))) =
            (entry, name.path.last())
        else {
            continue;
        };
        let EntityKey::Edge(e) = r.key else { continue };
        let edge = body.get_edge(e).unwrap();
        let ps = [edge.he_plus, edge.he_minus]
            .map(|he| point(body, body.get_half_edge(he).unwrap().start));
        if ps
            .iter()
            .all(|p| (p.y - 1.0).abs() < 1e-9 && (p.z - 1.0).abs() < 1e-9)
        {
            let (x0, x1) = (ps[0].x.min(ps[1].x), ps[0].x.max(ps[1].x));
            out.insert((micro(x0), micro(x1)), name.clone());
        }
    }
    out
}

#[test]
fn a_second_cut_on_a_seam_leaves_the_other_pieces_names() {
    let doc = ProfileDoc::empty_derived("edge-piece-locality", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, rib) = block(doc, (-1.0, 4.0), (1.0, 2.0), 0.5, 1.5);
    let (doc, joined) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: plate,
            b: rib,
            declare: Vec::new(),
        },
    );
    // Two notches across the rib's y = 1 seam: the first in place, the
    // second parked away from everything until the edit slides it on.
    let (doc, n1) = block(doc, (0.5, 0.7), (0.5, 1.5), 0.8, 0.7);
    let (doc, n2) = block(doc, (2.0, 2.2), (4.5, 5.5), 0.8, 0.7);
    let (doc, tr) = insert(
        doc,
        Node::transform(
            n2,
            editor_core::Step::Rigid {
                translation: [len(0.0), len(0.0), len(0.0)],
                axis: [scl(0.0), scl(0.0), scl(1.0)],
                angle: ang(0.0),
            },
        ),
    );
    let (doc, notches) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: n1,
            b: tr,
            declare: Vec::new(),
        },
    );
    let (doc, cut) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: joined,
            b: notches,
            declare: Vec::new(),
        },
    );
    let (doc2, _) = step(
        doc.clone(),
        DocEdit::SetParam {
            node: tr,
            slot: SlotId::Translation(Axis3::Y),
            expr: len(-4.0),
        },
    );
    let (ev, ev2) = (run(&doc), run(&doc2));
    for (label, ev) in [("one notch", &ev), ("two notches", &ev2)] {
        assert!(
            failure(ev, cut).is_none(),
            "{label}: {:?}",
            failure(ev, cut)
        );
    }
    let x = |a: f64, b: f64| ((a * 1e6).round() as i64, (b * 1e6).round() as i64);
    let (one, two) = (seam_pieces(&ev, cut), seam_pieces(&ev2, cut));
    assert_eq!(
        one.keys().copied().collect::<Vec<_>>(),
        vec![x(0.0, 0.5), x(0.7, 3.0)],
        "one notch cuts the seam in two"
    );
    assert_eq!(
        two.keys().copied().collect::<Vec<_>>(),
        vec![x(0.0, 0.5), x(0.7, 2.0), x(2.2, 3.0)],
        "the second cuts it in three"
    );
    assert_eq!(
        one[&x(0.0, 0.5)],
        two[&x(0.0, 0.5)],
        "the piece the second notch does not touch keeps its name"
    );
    assert_ne!(
        one[&x(0.7, 3.0)],
        two[&x(0.7, 2.0)],
        "the piece it cuts is renamed: one of its ends moved"
    );
}

/// **What `Ends` does not make local: a second crossing by a face that
/// already crosses the parent.** A crossing keeps an ordinal along the
/// edge it crosses (N2), and a lone crossing has none, so when one face
/// comes to cross an edge twice its first crossing is renamed though it
/// did not move, and so is every piece whose `Ends` cite it.
///
/// A cylinder (radius 0.3, its rims the profile's two semicircles,
/// `Piece(0)` the `+y` one) split by a vertical plane through the rim
/// point P at 135°. Its other crossing is at 300° (on `Piece(1)`) in one
/// document and at 30° (on `Piece(0)`) in the other, so the plane
/// crosses `Piece(0)` once and then twice, at P both times.
#[test]
fn a_second_crossing_by_the_same_face_renames_the_crossing_and_the_pieces_ending_at_it() {
    use editor_core::{Datum, SplitHalf};

    use crate::emit_union_borders::cylinder;

    let at = |deg: f64| {
        let t = deg.to_radians();
        (0.3 * t.cos(), 0.3 * t.sin())
    };
    let p = at(135.0);
    let split = |other: f64| {
        let doc = ProfileDoc::empty_derived("edge-piece-locality-twice", Tol::witness());
        let (doc, rod) = cylinder(doc, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], 1.0);
        let q = at(other);
        let (dx, dy) = (q.0 - p.0, q.1 - p.1);
        let k = (dx * dx + dy * dy).sqrt();
        // The normal's sign puts the rim point at 180° Below.
        let (mut nx, mut ny) = (dy / k, -dx / k);
        if nx * (-0.3 - p.0) + ny * (0.0 - p.1) > 0.0 {
            (nx, ny) = (-nx, -ny);
        }
        let (doc, tool) = insert(
            doc,
            Node::Datum(Datum::Plane {
                origin: [len(p.0), len(p.1), len(0.0)],
                normal: [scl(nx), scl(ny), scl(0.0)],
            }),
        );
        let (doc, cut) = insert(doc, Node::Split { target: rod, tool });
        let ev = run(&doc);
        assert!(
            failure(&ev, cut).is_none(),
            "{other}°: {:?}",
            failure(&ev, cut)
        );
        (ev, cut)
    };
    // The Below crossing vertex at P on the start rim (z = 0), and every
    // Below edge name whose `Ends` cite it.
    let at_p = |(ev, cut): &(editor_core::Evaluation<f64>, editor_core::RecipeNodeId)| {
        let below = match &ev.value(*cut).unwrap().payload {
            editor_core::ValuePayload::Split { below, .. } => match below {
                editor_core::SplitSide::Body(b) => std::sync::Arc::clone(b),
                editor_core::SplitSide::Empty => panic!("a Below half"),
            },
            _ => panic!("a split value"),
        };
        let t = table(ev, *cut);
        let vertex = t
            .iter()
            .find_map(|(n, e)| {
                let (Entry::Unique(r), Some(RoleSeg::CrossingVertex { side, .. })) =
                    (e, n.path.first())
                else {
                    return None;
                };
                let EntityKey::Vertex(v) = r.key else {
                    return None;
                };
                if *side != SplitHalf::Below || r.body != 1 {
                    return None;
                }
                let q = point(&below, v);
                (*side == SplitHalf::Below
                    && r.body == 1
                    && (q.x - p.0).abs() < 1e-9
                    && (q.y - p.1).abs() < 1e-9
                    && q.z.abs() < 1e-9)
                    .then(|| n.clone())
            })
            .expect("a Below crossing at P on the start rim");
        let ending: Vec<StableName> = t
            .iter()
            .filter(|(n, _)| {
                matches!(n.path.last(), Some(RoleSeg::Fragment(Qualifier::Ends(ends))) if ends.contains(&vertex))
            })
            .map(|(n, _)| n.clone())
            .collect();
        (vertex, ending)
    };
    let (once, twice) = (at_p(&split(300.0)), at_p(&split(30.0)));
    assert_eq!(
        once.0.path.len(),
        1,
        "crossed once, the crossing takes no rank: {:?}",
        once.0
    );
    assert!(
        matches!(
            twice.0.path.last(),
            Some(RoleSeg::Fragment(Qualifier::OrderAlong { of: 2, .. }))
        ),
        "crossed twice, the same crossing is ranked one of two: {:?}",
        twice.0
    );
    assert!(
        !twice.1.is_empty() && twice.1.iter().all(|n| !once.1.contains(n)),
        "every piece ending at it is renamed: {:?} -> {:?}",
        once.1,
        twice.1
    );
}
