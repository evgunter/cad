//! **A cut elsewhere on an edge leaves its other pieces' names as they
//! were** (N2's `Ends`, the locality the ruled rule buys). A piece is
//! named by its two ends, so a cut that does not touch it neither moves
//! its ends nor renames it; a rank over the pieces would have written
//! the group's new size into every one of them. A crossing is named by
//! its sense, so a second crossing by a face that already crosses the
//! edge leaves the first crossing's name as it was.
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

/// **A second crossing by a face that already crosses the edge renames
/// neither the first crossing nor the pieces ending at it.** A crossing
/// is named by its sense (N2), a fact of the one vertex, and a second
/// crossing of the other sense leaves it as it was.
///
/// One document, edited in place: a cylinder (radius 0.3, its rims the
/// profile's two semicircles, `Piece(0)` the `+y` one) split by a
/// vertical plane through the rim point P at 135°. Its other crossing is
/// at 300° (on `Piece(1)`) until the plane's normal is edited to put it
/// at 30° (on `Piece(0)`), so the plane crosses `Piece(0)` once and then
/// twice, at P both times. `Piece(0)` runs from 0° to 180°, and the
/// plane's normal puts 180° Below, so the rim enters the Below half at P
/// and, once crossed at 30°, leaves it there.
#[test]
fn a_second_crossing_by_the_same_face_keeps_the_first_crossings_name() {
    use editor_core::{Datum, Sense, SplitHalf};

    use crate::emit_union_borders::cylinder;

    let at = |deg: f64| {
        let t = deg.to_radians();
        (0.3 * t.cos(), 0.3 * t.sin())
    };
    let p = at(135.0);
    // The plane through P and the rim point at `other`, its normal's
    // sign putting the rim point at 180° Below.
    let normal = |other: f64| {
        let q = at(other);
        let (dx, dy) = (q.0 - p.0, q.1 - p.1);
        let k = (dx * dx + dy * dy).sqrt();
        let (nx, ny) = (dy / k, -dx / k);
        if nx * (-0.3 - p.0) + ny * (0.0 - p.1) > 0.0 {
            (-nx, -ny)
        } else {
            (nx, ny)
        }
    };
    let doc = ProfileDoc::empty_derived("edge-piece-locality-twice", Tol::witness());
    let (doc, rod) = cylinder(doc, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], 1.0);
    let (nx, ny) = normal(300.0);
    let (doc, tool) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(p.0), len(p.1), len(0.0)],
            normal: [scl(nx), scl(ny), scl(0.0)],
        }),
    );
    let (doc, cut) = insert(doc, Node::Split { target: rod, tool });
    let (nx, ny) = normal(30.0);
    let (doc2, _) = step(
        doc.clone(),
        DocEdit::SetParam {
            node: tool,
            slot: SlotId::Normal(Axis3::X),
            expr: scl(nx),
        },
    );
    let (doc2, _) = step(
        doc2,
        DocEdit::SetParam {
            node: tool,
            slot: SlotId::Normal(Axis3::Y),
            expr: scl(ny),
        },
    );
    let (ev, ev2) = (run(&doc), run(&doc2));
    for (label, ev) in [("crossed once", &ev), ("crossed twice", &ev2)] {
        assert!(
            failure(ev, cut).is_none(),
            "{label}: {:?}",
            failure(ev, cut)
        );
    }
    // The Below crossings of the start rim (z = 0) on `Piece(0)`, by
    // where they lie.
    let below_crossings = |ev: &editor_core::Evaluation<f64>| {
        let below = match &ev.value(cut).unwrap().payload {
            editor_core::ValuePayload::Split { below, .. } => match below {
                editor_core::SplitSide::Body(b) => std::sync::Arc::clone(b),
                editor_core::SplitSide::Empty => panic!("a Below half"),
            },
            _ => panic!("a split value"),
        };
        let mut out = Vec::new();
        for (n, e) in table(ev, cut).iter() {
            let (Entry::Unique(r), Some(RoleSeg::CrossingVertex { side, .. })) =
                (e, n.path.first())
            else {
                continue;
            };
            let EntityKey::Vertex(v) = r.key else {
                continue;
            };
            if *side != SplitHalf::Below || r.body != 1 {
                continue;
            }
            let q = point(&below, v);
            if q.z.abs() < 1e-9 && q.y > -1e-9 {
                out.push(((q.x, q.y), n.clone()));
            }
        }
        out
    };
    let near = |q: (f64, f64), r: (f64, f64)| (q.0 - r.0).abs() < 1e-9 && (q.1 - r.1).abs() < 1e-9;
    let (once, twice) = (below_crossings(&ev), below_crossings(&ev2));
    assert_eq!(once.len(), 1, "crossed once: {once:?}");
    assert_eq!(twice.len(), 2, "crossed twice: {twice:?}");
    let at_p = |crossings: &[((f64, f64), StableName)]| {
        crossings
            .iter()
            .find(|(q, _)| near(*q, p))
            .map(|(_, n)| n.clone())
            .expect("a Below crossing at P")
    };
    let (p_once, p_twice) = (at_p(&once), at_p(&twice));
    assert_eq!(p_once, p_twice, "the crossing at P keeps its name");
    assert!(
        matches!(
            p_once.path.as_slice(),
            [RoleSeg::CrossingVertex {
                sense: Sense::Enters,
                ..
            }]
        ),
        "the rim enters the Below half at P, and nothing ranks it: {p_once:?}"
    );
    let second = twice
        .iter()
        .find(|(q, _)| near(*q, at(30.0)))
        .map(|(_, n)| n)
        .expect("a Below crossing at 30°");
    assert!(
        matches!(
            second.path.as_slice(),
            [RoleSeg::CrossingVertex {
                sense: Sense::Leaves,
                ..
            }]
        ),
        "the rim leaves the Below half at 30°: {second:?}"
    );
}
