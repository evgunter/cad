//! INTENT stage 3 E acceptance — an operation computes in a frame of
//! its reads (`docs/INTENT-STAGE3-SPEC.md` §6; D10 `[placement.frame]`,
//! tests 13 and 27).
//!
//! A body value is its body in its own coordinates, shared, and one map
//! into its space's (`Framed`). A copy keeps its source's `Arc` under the
//! composed map and builds nothing. Every operation computes in its first
//! operand's frame and reads the rest through `relative`, so the frame is
//! no part of meaning: computed in a second frame, every corpus document
//! is the same bodies where they stand, the same names and the same
//! verdicts.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::fixture;
use editor_core::{
    BooleanValue, CancelToken, ComputingTwist, DocumentId, EntityKey, Entry, EvalCounters,
    EvalOptions, Evaluation, Framed, Node, NodeResult, ProfileDoc, RecipeNodeId, SplitSide,
    ValuePayload, evaluate,
};
use fixture::{len, on_frame, square};
use geom_core::Tol;

/// A unit block, a construction in a space of its own.
fn block(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.5, 0.5, 0.5)],
    );
    fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    )
}

/// The one body a node's value holds, as it holds it.
fn framed(ev: &Evaluation<f64>, node: RecipeNodeId) -> Framed<f64> {
    match ev.value(node).map(|v| &v.payload) {
        Some(ValuePayload::Body(b) | ValuePayload::Boolean(BooleanValue::Body { body: b, .. })) => {
            b.clone()
        }
        other => panic!("{node:?} holds one body, not {other:?}"),
    }
}

/// A copy of `body` in the world under `at`'s values, the product left
/// alone: no `⊔` reads the copies, so the copies are all that runs.
fn in_world(
    doc: ProfileDoc,
    body: RecipeNodeId,
    at: &editor_core::Frame,
) -> (ProfileDoc, RecipeNodeId) {
    editor_core::test_support::in_world(doc, body, at, Tol::witness())
}

fn run(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        prior,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// **A copy shares its source, and a third builds nothing** (test 27).
/// Three placements of one block, each moved by values: every copy's
/// body is the block's own `Arc`, the evaluation maps no body, and the
/// third placement, added after the first two evaluated, is the one
/// node that recomputes and maps nothing either.
///
/// *Breaks if* `wire_place` maps its source body into the target (each
/// copy a body of its own, and one rigid map per copy).
#[test]
fn two_copies_share_one_arc_and_a_third_builds_nothing() {
    let doc = ProfileDoc::empty(DocumentId::derive("e-copies"), Tol::witness());
    let (doc, source) = block(doc);
    let (doc, first) = in_world(
        doc,
        source,
        &editor_core::Frame::translation([2.0, 0.0, 0.0]),
    );
    let (doc, second) = in_world(
        doc,
        source,
        &editor_core::Frame::translation([0.0, 3.0, 0.0]),
    );
    let before = EvalCounters::on_this_thread();
    let ev = run(&doc, None);
    let built = EvalCounters::on_this_thread().since(before);
    let own = framed(&ev, source);
    for (copy, which) in [(first, "first"), (second, "second")] {
        assert!(
            Arc::ptr_eq(framed(&ev, copy).shared(), own.shared()),
            "the {which} copy is the block's own Arc"
        );
    }
    assert_eq!(built.rigid_maps, 0, "two copies map no body: {built:?}");

    let (doc, third) = in_world(
        doc,
        source,
        &editor_core::Frame::translation([0.0, 0.0, 4.0]),
    );
    let before = EvalCounters::on_this_thread();
    let again = run(&doc, Some(&ev));
    let built = EvalCounters::on_this_thread().since(before);
    assert_eq!(again.recomputed, 1, "the third placement alone runs");
    assert_eq!(built.rigid_maps, 0, "and it maps no body: {built:?}");
    assert!(
        Arc::ptr_eq(
            framed(&again, third).shared(),
            framed(&again, source).shared()
        ),
        "the third copy is the block's own Arc"
    );
    // Each copy stands where its values put it.
    for (copy, at) in [
        (first, [2.0, 0.0, 0.0]),
        (second, [0.0, 3.0, 0.0]),
        (third, [0.0, 0.0, 4.0]),
    ] {
        let lo = fixture::in_space(&framed(&again, copy))
            .points()
            .fold([f64::INFINITY; 3], |m, (_, p)| {
                [m[0].min(p.x), m[1].min(p.y), m[2].min(p.z)]
            });
        assert!(
            lo.iter().zip(at).all(|(a, b)| (a - b).abs() < 1e-12),
            "{copy:?} stands at {lo:?}, placed at {at:?}"
        );
    }
}

/// **The second computing frame**: a turn by an irrational angle about
/// an irrational axis, then a translation off every axis.
fn twist() -> ComputingTwist {
    let n = 6.0_f64.sqrt();
    ComputingTwist {
        axis: [1.0 / n, 2.0_f64.sqrt() / n, 3.0_f64.sqrt() / n],
        angle: std::f64::consts::FRAC_1_SQRT_2,
        by: [0.37, -1.25, 2.5],
    }
}

/// What one node is, read where it stands in its space: its standing,
/// and for a body each name's points; for a measure its value; for an
/// assertion its verdict's side.
#[derive(Debug)]
enum Reading {
    Refused(String),
    Bodies(Vec<BTreeMap<String, Vec<[f64; 3]>>>),
    Measure(f64),
    Verdict(String),
    Other(&'static str),
}

fn reading(ev: &Evaluation<f64>, node: RecipeNodeId) -> Option<Reading> {
    let value = match ev.result(node)? {
        NodeResult::Ok(value) => value,
        other => {
            return Some(Reading::Refused(match ev.node_error(node) {
                Some(e) => format!("{:?}", e.kind.class()),
                None => format!("{:?}", core::mem::discriminant(other)),
            }));
        }
    };
    let bodies: Vec<&Framed<f64>> = match &value.payload {
        ValuePayload::Body(b) | ValuePayload::Boolean(BooleanValue::Body { body: b, .. }) => {
            vec![b]
        }
        ValuePayload::Instances(v) => v.iter().collect(),
        ValuePayload::Split { above, below } => [above, below]
            .into_iter()
            .filter_map(|s| match s {
                SplitSide::Body(b) => Some(b),
                SplitSide::Empty => None,
            })
            .collect(),
        ValuePayload::Measure { value, .. } => return Some(Reading::Measure(*value)),
        ValuePayload::Assertion(verdict) => {
            let said = format!("{verdict:?}");
            let side = said.split([' ', '{', '(']).next().unwrap_or("").to_owned();
            return Some(Reading::Verdict(side));
        }
        other => return Some(Reading::Other(other.kind_name())),
    };
    let ports = u8::try_from(bodies.len()).expect("a handful of bodies");
    let read = (0..ports)
        .zip(&bodies)
        .map(|(i, body)| {
            let placed = fixture::in_space(body);
            // A split's half reads its own output's table; every other
            // body indexes the node's one table.
            let (var, index) = ev.body_read_of(node, u32::from(i)).expect("a body's read");
            let mut by_name: BTreeMap<String, Vec<[f64; 3]>> = BTreeMap::new();
            for (name, entry) in ev.table(var).expect("a table").iter() {
                let Entry::Unique(e) = entry else {
                    by_name.insert(format!("{name:?}"), Vec::new());
                    continue;
                };
                if e.body != index {
                    continue;
                }
                let at = |v| {
                    let p = fixture::point(&placed, v);
                    [p.x, p.y, p.z]
                };
                let points = match e.key {
                    EntityKey::Vertex(v) => vec![at(v)],
                    EntityKey::Edge(k) => fixture::ends(&placed, k).map(at).to_vec(),
                    EntityKey::Face(_) | EntityKey::Body => Vec::new(),
                };
                by_name.insert(format!("{name:?}"), points);
            }
            by_name
        })
        .collect();
    Some(Reading::Bodies(read))
}

/// Whether two readings agree: the same standing, the same names, every
/// named point within `tol` once sorted (an edge's two ends are a set),
/// measures within `tol` and the same verdicts.
fn agree(a: &Reading, b: &Reading, tol: f64) -> Result<(), String> {
    let close = |p: &[f64; 3], q: &[f64; 3]| p.iter().zip(q).all(|(x, y)| (x - y).abs() <= tol);
    match (a, b) {
        (Reading::Refused(x), Reading::Refused(y)) | (Reading::Verdict(x), Reading::Verdict(y)) => {
            (x == y).then_some(()).ok_or_else(|| format!("{x} vs {y}"))
        }
        (Reading::Other(x), Reading::Other(y)) => {
            (x == y).then_some(()).ok_or_else(|| format!("{x} vs {y}"))
        }
        (Reading::Measure(x), Reading::Measure(y)) => ((x - y).abs() <= tol)
            .then_some(())
            .ok_or_else(|| format!("measured {x} vs {y}")),
        (Reading::Bodies(x), Reading::Bodies(y)) => {
            if x.len() != y.len() {
                return Err(format!("{} bodies vs {}", x.len(), y.len()));
            }
            for (bx, by) in x.iter().zip(y) {
                if bx.keys().ne(by.keys()) {
                    let only: Vec<_> = bx.keys().filter(|k| !by.contains_key(*k)).collect();
                    let other: Vec<_> = by.keys().filter(|k| !bx.contains_key(*k)).collect();
                    return Err(format!("names differ: {only:?} vs {other:?}"));
                }
                for (name, px) in bx {
                    let mut px = px.clone();
                    let mut py = by[name].clone();
                    // Ordered at a grid far coarser than the rounding, so
                    // two ends a rounding apart sort alike.
                    let grid = |a: &[f64; 3]| a.map(|c| (c * 1e6).round());
                    let order =
                        |a: &[f64; 3], b: &[f64; 3]| grid(a).partial_cmp(&grid(b)).expect("finite");
                    px.sort_by(order);
                    py.sort_by(order);
                    if px.len() != py.len() || !px.iter().zip(&py).all(|(p, q)| close(p, q)) {
                        return Err(format!("{name} stands at {px:?} vs {py:?}"));
                    }
                }
            }
            Ok(())
        }
        _ => {
            let short = |r: &Reading| match r {
                Reading::Bodies(b) => format!("{} bodies", b.len()),
                other => format!("{other:?}"),
            };
            Err(format!("{} vs {}", short(a), short(b)))
        }
    }
}

/// **The computing frame is no part of meaning** (test 13). Every
/// corpus document evaluated as built and again with every construction
/// computed in a second frame — turned by an irrational angle about an
/// irrational axis and translated, so every operation downstream computes
/// in a frame that twist turned — reads the same: each node refuses or
/// builds alike, every body stands where it stood in its space to the
/// rounding of the frames, under the same names, every measure is the
/// same number and every assertion the same verdict.
///
/// The one exception is a placed union's separation certificate, an
/// axis-aligned box test, which refuses in the turned frame where it
/// certifies in the fins' own
/// (`work/orbit/a-placed-unions-separation-certificate-depends-on-the-computing-axes.md`);
/// it is named and counted.
///
/// *Breaks if* an operation reads a body's coordinates as its space's
/// without its frame (a copy or an operand lands elsewhere), or a minted
/// reference follows the computing axes (a name changes).
#[test]
fn every_corpus_document_reads_alike_in_a_second_computing_frame() {
    let twisted = EvalOptions {
        computing_twist: Some(twist()),
        ..EvalOptions::default()
    };
    let mut compared = 0usize;
    let mut moved = 0usize;
    let mut apart: Vec<String> = Vec::new();
    let mut uncertified: Vec<(&str, RecipeNodeId)> = Vec::new();
    for d in crate::corpus::documents() {
        let plain = run(&d.doc, None);
        let turned = evaluate::<f64>(&d.doc, None, &CancelToken::new(), &twisted, Tol::witness());
        for node in d.doc.ids() {
            let (Some(a), Some(b)) = (reading(&plain, node), reading(&turned, node)) else {
                continue;
            };
            // The one verdict outside the band a frame moves: a placed
            // union's separation certificate bounds each copy by an
            // axis-aligned box, which the turned frame fattens
            // (`work/orbit/a-placed-unions-separation-certificate-depends-on-the-computing-axes.md`).
            // Counted, so the exception cannot quietly grow.
            if let (Reading::Bodies(_), Reading::Refused(class)) = (&a, &b)
                && class == "PlacementsUncertified"
            {
                uncertified.push((d.name, node));
                continue;
            }
            if let Err(why) = agree(&a, &b, 1e-9) {
                apart.push(format!(
                    "{}: node {node:?} ({}) reads apart: {}",
                    d.name,
                    d.doc.node(node).map_or("?", crate::corpus::node_kind),
                    why.chars().take(300).collect::<String>()
                ));
            }
            compared += 1;
            if let (Some(p), Some(t)) = (plain.value(node), turned.value(node))
                && let (ValuePayload::Body(p), ValuePayload::Body(t)) = (&p.payload, &t.payload)
                && !Arc::ptr_eq(p.shared(), t.shared())
                && p.relative(t).is_ok_and(|m| m.is_some())
            {
                moved += 1;
            }
        }
    }
    assert!(
        apart.is_empty(),
        "{} nodes read apart:\n{}",
        apart.len(),
        apart.join("\n")
    );
    assert_eq!(
        uncertified
            .iter()
            .map(|(name, _)| *name)
            .collect::<Vec<_>>(),
        ["heat_sink_fins", "heat_sink_fins"],
        "the placed union and the copy reading it, alone: {uncertified:?}"
    );
    // The twist reached the operations: a row whose second frame was the
    // first would pass having compared a document with itself.
    assert!(compared > 100, "{compared} nodes compared");
    assert!(moved > 20, "{moved} bodies computed in another frame");
}

/// **A value edit moves a frame, not a mesh** (§6, the viewer's
/// drawing): the pick memo serves a copy whose values moved the
/// tessellation it built for the copy's body, which the copy still
/// shares, under the frame the copy stands in now.
///
/// *Breaks if* the memo keys a copy by its content and naming keys alone (the edited
/// copy misses and is tessellated again), or serves the old frame.
#[test]
fn a_value_edit_moves_a_copys_frame_and_not_its_mesh() {
    let doc = ProfileDoc::empty(DocumentId::derive("e-value-edit"), Tol::witness());
    let (doc, source) = block(doc);
    let (doc, copy) = in_world(
        doc,
        source,
        &editor_core::Frame::translation([2.0, 0.0, 0.0]),
    );
    let tol = Tol::witness();
    let first = run(&doc, None);
    let mut memo = editor_core::PickMemo::new();
    let before = editor_core::NodePick::build_with(&first, copy, 0, 0.1, tol, &mut memo)
        .expect("the copy tessellates");
    memo.end_picture();
    let slid = editor_core::Coord::Slide(editor_core::TargetAxis::X);
    let edit = fixture::value_bump(&doc, copy, slid, 1.5);
    let moved = editor_core::apply(&doc, &edit, tol)
        .expect("the value moves")
        .doc;
    let second = run(&moved, Some(&first));
    let after = editor_core::NodePick::build_with(&second, copy, 0, 0.1, tol, &mut memo)
        .expect("the moved copy tessellates");
    memo.end_picture();
    // The counts are the closed picture's: the second.
    assert_eq!(
        (memo.node_misses(), memo.node_hits()),
        (0, 1),
        "the moved copy is served the tessellation its body already has"
    );
    assert!(
        std::ptr::eq(before.mesh(), after.mesh()),
        "one mesh, under two frames"
    );
    let x = |pick: &editor_core::NodePick| {
        editor_core::in_space(pick).map_or(0.0, |map| map.origin().x)
    };
    assert_eq!(
        (x(&before), x(&after)),
        (2.0, 3.5),
        "the frame moved with the value"
    );
}
