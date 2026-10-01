//! **A cut elsewhere on an edge leaves its other pieces' names as they
//! were** (N2's `Ends`, the locality the ruled rule buys). A piece is
//! named by its two ends, so a cut that does not touch it neither moves
//! its ends nor renames it; a rank over the pieces would have written
//! the group's new size into every one of them.
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
            declare: None,
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
            declare: None,
        },
    );
    let (doc, cut) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: joined,
            b: notches,
            declare: None,
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
