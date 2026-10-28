//! INTENT stage 3 E acceptance — an operation computes in a frame of
//! its reads (`docs/INTENT-STAGE3-SPEC.md` §6; D10 `[placement.frame]`,
//! tests 13 and 27).
//!
//! A body value is its body in its own coordinates, shared, and one map
//! into its space's (`Framed`). A copy keeps its source's `Arc` under the
//! composed map and builds nothing. An operation over operands of one
//! space computes in a frame chosen for conditioning
//! (`[placement.frame]`): its first operand in its space's coordinates,
//! else its first (`Framed::computing_among`). It reads the rest through
//! `relative`, so the frame is no part of meaning: computed in a second
//! frame, every corpus document is the same bodies where they stand, the
//! same names and the same verdicts.

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

/// **The counter counts** (the positive control of
/// [`two_copies_share_one_arc_and_a_third_builds_nothing`]): a union of
/// two copies of a block standing under different frames reads the
/// second into the first's frame through `read_in`, and that read is a
/// rigid map the counter sees. A counter that never counted would pass
/// the row above by counting nothing.
#[test]
fn a_union_over_a_moved_copy_counts_its_rigid_map() {
    let doc = ProfileDoc::empty(DocumentId::derive("e-counter"), Tol::witness());
    let (doc, source) = block(doc);
    let (doc, first) = in_world(
        doc,
        source,
        &editor_core::Frame::translation([0.0, 0.0, 0.5]),
    );
    let (doc, copy) = in_world(
        doc,
        source,
        &editor_core::Frame::translation([0.5, 0.25, 0.0]),
    );
    let (doc, union) = fixture::union_over(doc, &[first, copy]);
    let before = EvalCounters::on_this_thread();
    let ev = run(&doc, None);
    let counted = EvalCounters::on_this_thread().since(before);
    assert!(
        ev.value(union).is_some(),
        "the union builds: {:?}",
        ev.node_error(union)
    );
    assert!(
        counted.rigid_maps > 0,
        "the union reads its moved member through one rigid map: {counted:?}"
    );
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

/// **Text with every float masked**: a refusal, a verdict or a census
/// finding compared whole, its words, keys, counts and variants, while
/// the numbers a frame's rounding moves (margins, witnesses) read `#`.
/// An integer (a key's index, a count) is kept: only a token with a
/// decimal point or an exponent is masked.
fn masked(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < chars.len() {
        let starts = chars[i].is_ascii_digit()
            || (chars[i] == '-' && chars.get(i + 1).is_some_and(char::is_ascii_digit));
        let after_word = i > 0 && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_');
        if starts && !after_word {
            let mut j = i + 1;
            while j < chars.len()
                && (chars[j].is_ascii_digit()
                    || chars[j] == '.'
                    || ((chars[j] == 'e' || chars[j] == 'E')
                        && chars
                            .get(j + 1)
                            .is_some_and(|c| c.is_ascii_digit() || *c == '-'))
                    || ((chars[j] == '-' || chars[j] == '+') && matches!(chars[j - 1], 'e' | 'E')))
            {
                j += 1;
            }
            let token: String = chars[i..j].iter().collect();
            if token.contains('.') || token.contains('e') || token.contains('E') {
                out.push('#');
            } else {
                out.push_str(&token);
            }
            i = j;
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out.replace("NaN", "#").replace("inf", "#")
}

/// A named entity where it stands in its space: its points, and for a
/// face its carrier's kind, distinguished point, principal direction and
/// sense (`topo::readback::face_pose`), where the carrier has one.
#[derive(Debug)]
struct Named {
    points: Vec<[f64; 3]>,
    surface: Option<(String, [f64; 3], [f64; 3], bool)>,
}

/// What one node is, read where it stands in its space: its standing
/// (a refusal whole), and for a body each name's points and each face's
/// carrier; for a measure its value; for an assertion its verdict whole.
#[derive(Debug)]
enum Reading {
    Refused { whole: String },
    Bodies(Vec<BTreeMap<String, Named>>),
    Measure(f64),
    Verdict(String),
    Other(&'static str),
}

fn reading(ev: &Evaluation<f64>, node: RecipeNodeId) -> Option<Reading> {
    let value = match ev.result(node)? {
        NodeResult::Ok(value) => value,
        other => {
            return Some(match ev.node_error(node) {
                Some(e) => Reading::Refused {
                    whole: masked(&format!("{:?}", e.kind)),
                },
                None => Reading::Refused {
                    whole: masked(&format!("{other:?}")),
                },
            });
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
            return Some(Reading::Verdict(masked(&format!("{verdict:?}"))));
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
            let mut by_name: BTreeMap<String, Named> = BTreeMap::new();
            for (name, entry) in ev.table(var).expect("a table").iter() {
                let Entry::Unique(e) = entry else {
                    by_name.insert(
                        format!("{name:?}"),
                        Named {
                            points: Vec::new(),
                            surface: None,
                        },
                    );
                    continue;
                };
                if e.body != index {
                    continue;
                }
                let at = |v| {
                    let p = fixture::point(&placed, v);
                    [p.x, p.y, p.z]
                };
                let (points, surface) = match e.key {
                    EntityKey::Vertex(v) => (vec![at(v)], None),
                    EntityKey::Edge(k) => (fixture::ends(&placed, k).map(at).to_vec(), None),
                    EntityKey::Face(f) => {
                        let kind = topo::readback::face_carrier_kind(&placed, f).ok();
                        let pose = topo::readback::face_pose(&placed, f).ok();
                        let surface = kind.zip(pose).map(|(kind, pose)| {
                            (
                                format!("{kind:?}"),
                                [pose.origin.x, pose.origin.y, pose.origin.z],
                                [pose.axis.x, pose.axis.y, pose.axis.z],
                                pose.sense,
                            )
                        });
                        (Vec::new(), surface)
                    }
                    EntityKey::Body => (Vec::new(), None),
                };
                by_name.insert(format!("{name:?}"), Named { points, surface });
            }
            by_name
        })
        .collect();
    Some(Reading::Bodies(read))
}

/// Whether two readings agree: the same standing and the same refusal
/// whole, the same names, every named point within `tol` once sorted (an
/// edge's two ends are a set), every face's carrier the same kind and
/// sense with its point and direction within `tol`, measures within
/// `tol` and the same verdicts whole.
fn agree(a: &Reading, b: &Reading, tol: f64) -> Result<(), String> {
    let close = |p: &[f64; 3], q: &[f64; 3]| p.iter().zip(q).all(|(x, y)| (x - y).abs() <= tol);
    match (a, b) {
        (Reading::Refused { whole: x, .. }, Reading::Refused { whole: y, .. })
        | (Reading::Verdict(x), Reading::Verdict(y)) => {
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
                for (name, nx) in bx {
                    let ny = &by[name];
                    let mut px = nx.points.clone();
                    let mut py = ny.points.clone();
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
                    let same = match (&nx.surface, &ny.surface) {
                        (None, None) => true,
                        (Some(x), Some(y)) => same_carrier(x, y, tol),
                        _ => false,
                    };
                    if !same {
                        return Err(format!(
                            "{name}'s carrier reads {:?} vs {:?}",
                            nx.surface, ny.surface
                        ));
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

/// **Whether two face carriers are one surface where they stand**, by
/// what the surface is rather than how its chart was minted (a chart's
/// origin and pole follow the frame it was built in): a plane by its
/// outward normal and offset, a sphere by its centre, a carrier about an
/// axis (cylinder, cone, torus) by its axis line, direction up to sign;
/// any other by its kind.
fn same_carrier(
    (kx, ox, ax, sx): &(String, [f64; 3], [f64; 3], bool),
    (ky, oy, ay, sy): &(String, [f64; 3], [f64; 3], bool),
    tol: f64,
) -> bool {
    let dot = |a: &[f64; 3], b: &[f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let sub = |a: &[f64; 3], b: &[f64; 3]| [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    let near = |a: f64, b: f64| (a - b).abs() <= tol;
    if kx != ky {
        return false;
    }
    match kx.as_str() {
        "Plane" => {
            let side = |axis: &[f64; 3], sense: bool| {
                let s = if sense { 1.0 } else { -1.0 };
                axis.map(|c| s * c)
            };
            let (nx, ny) = (side(ax, *sx), side(ay, *sy));
            nx.iter().zip(&ny).all(|(a, b)| near(*a, *b)) && near(dot(&nx, ox), dot(&ny, oy))
        }
        "Sphere" => ox.iter().zip(oy).all(|(a, b)| near(*a, *b)),
        "Cylinder" | "Cone" | "Torus" => {
            let parallel = near(dot(ax, ay).abs(), 1.0);
            let off = sub(oy, ox);
            let along = dot(&off, ax);
            let across = sub(&off, &ax.map(|c| c * along));
            parallel && near(dot(&across, &across).sqrt(), 0.0)
        }
        _ => true,
    }
}

/// A document's product census as both runs gather it: every finding
/// whole, its numbers masked, or the gather's refusal whole.
fn census(doc: &ProfileDoc, ev: &Evaluation<f64>) -> Vec<String> {
    match editor_core::product_recorded(doc, ev, Tol::witness()) {
        Ok(product) => product
            .census
            .iter()
            .map(|finding| masked(&format!("{finding:?}")))
            .collect(),
        Err(refusal) => vec![masked(&format!("gather refused: {refusal:?}"))],
    }
}

/// **The computing frame is no part of meaning** (test 13). Every
/// corpus document evaluated as built and again with every construction
/// computed in a second frame of its own — each its node's variation of
/// a turn by an irrational angle about an irrational axis and a
/// translation (`ComputingTwist::for_node`), a part's constructions
/// too, so every operation downstream computes over operands in frames
/// turned apart — reads the same:
/// - each node refuses alike (the refusal compared whole) or builds
///   alike, and a node with a result in one run has one in the other;
/// - every body stands where it stood in its space to the rounding of
///   the frames, under the same names, every face on the same carrier;
/// - every measure is the same number and every assertion the same
///   verdict, whole;
/// - every document's product census finds the same findings.
///
/// A union over a family's rigid images certifies its pairs apart by
/// axis-aligned boxes, which a turned frame fattens; a pair it does not
/// certify is judged, so what it builds reads the same.
///
/// *Breaks if* an operation reads an operand's coordinates as its own
/// computing frame's without `relative` (a measure, a union member, a
/// gathered source lands elsewhere), a minted reference follows the
/// computing axes (a name changes), or a census reads a frame's axes.
/// Both reviewers' mutants of E — `wire_measure` reading an operand
/// raw, and `Member::in_frame_of` returning a member unmapped — go red
/// here.
#[test]
fn every_corpus_document_reads_alike_in_a_second_computing_frame() {
    let twisted = EvalOptions {
        computing_twist: Some(twist()),
        ..EvalOptions::default()
    };
    let mut compared = 0usize;
    let mut apart: Vec<String> = Vec::new();
    let mut unmoved: Vec<&str> = Vec::new();
    for d in crate::corpus::documents() {
        let plain = run(&d.doc, None);
        let turned = evaluate::<f64>(&d.doc, None, &CancelToken::new(), &twisted, Tol::witness());
        let mut bodies = 0usize;
        let mut moved = 0usize;
        for node in d.doc.ids() {
            let (a, b) = match (reading(&plain, node), reading(&turned, node)) {
                (None, None) => continue,
                (Some(a), Some(b)) => (a, b),
                (a, b) => {
                    apart.push(format!(
                        "{}: node {node:?} has a result in one run alone: {:?} vs {:?}",
                        d.name,
                        a.is_some(),
                        b.is_some()
                    ));
                    continue;
                }
            };
            if let Err(why) = agree(&a, &b, 1e-9) {
                apart.push(format!(
                    "{}: node {node:?} ({}) reads apart: {}",
                    d.name,
                    d.doc.node(node).map_or("?", crate::corpus::node_kind),
                    why.chars().take(600).collect::<String>()
                ));
            }
            compared += 1;
            if let (Some(p), Some(t)) = (plain.value(node), turned.value(node))
                && let (ValuePayload::Body(p), ValuePayload::Body(t)) = (&p.payload, &t.payload)
            {
                bodies += 1;
                if !Arc::ptr_eq(p.shared(), t.shared()) && p.relative(t).is_ok_and(|m| m.is_some())
                {
                    moved += 1;
                }
            }
        }
        // The twist reached this document's operations: a document
        // whose second run computed nothing in another frame compared
        // itself with itself.
        if bodies > 0 && moved == 0 {
            unmoved.push(d.name);
        }
        let (cx, cy) = (census(&d.doc, &plain), census(&d.doc, &turned));
        if cx != cy {
            apart.push(format!(
                "{}: the product census reads apart: {cx:?} vs {cy:?}",
                d.name
            ));
        }
    }
    assert!(
        apart.is_empty(),
        "{} readings apart:\n{}",
        apart.len(),
        apart.join("\n")
    );
    assert!(
        unmoved.is_empty(),
        "documents with bodies none of which computed in another frame: {unmoved:?}"
    );
    assert!(compared > 100, "{compared} nodes compared");
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
