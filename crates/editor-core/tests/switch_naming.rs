//! **LIB-SWITCH §6: a parameter edit cannot renumber, pinned.**
//!
//! Profile-entity names index canonical positions, and the canonical
//! form keeps each loop's authored start (§V3): nothing geometric
//! enters the index, so a parameter edit CANNOT renumber — including
//! the edit class that renumbered under a geometric start (a rectangle
//! whose lexicographic-minimum corner moves). The freeze doctrine
//! remains the structural-edit backstop (stale refs refuse Vanished).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::fixture;

use editor_core::{
    CancelToken, Dimension, DocEdit, DocParam, EvalOptions, Expr, LoopProgram, Node, ParamName,
    ProfileDoc, ProfileProgram, ProgramStep, ProgramTarget, RecipeNodeId, StableName, ValuePayload,
    evaluate,
};
use geom_core::Tol;

/// A quad whose LAST authored corner x is a document parameter: at
/// x0 = 0.5 that corner (0.5, 1) is the lexicographic minimum; at
/// x0 = 1.5 the ENTRY corner (1, 0) is — a geometric start would move
/// under this pure parameter edit, the §6 renumbering class on the
/// nose.
/// Every document here is a sketch frame, the profile drawn on it, and
/// the extrude over that: node 0 is the frame, so these two are what
/// the rows address.
const PROFILE: RecipeNodeId = RecipeNodeId(1);
const BODY: RecipeNodeId = RecipeNodeId(2);

fn param_rect_doc(x0: f64) -> ProfileDoc {
    let lit = |v: f64| Expr::literal(v, Dimension::Length).unwrap();
    let x0e = || Expr::param(ParamName::new("x0"), Dimension::Length);
    let doc = ProfileDoc::empty_derived("switch_naming", Tol::witness())
        .apply(
            &DocEdit::SetDocParam {
                name: ParamName::new("x0"),
                value: DocParam::continuous(Dimension::Length, x0),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .unwrap()
        .doc;
    let loop_ = LoopProgram::Chain(vec![
        ProgramStep::At([lit(1.0), lit(0.0)]),
        ProgramStep::LineTo(ProgramTarget::Point([lit(2.0), lit(0.0)])),
        ProgramStep::LineTo(ProgramTarget::Point([lit(2.0), lit(1.0)])),
        ProgramStep::LineTo(ProgramTarget::Point([x0e(), lit(1.0)])),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let (doc, xy) = fixture::insert(doc, fixture::xy_frame());
    let doc = doc
        .apply(
            &DocEdit::InsertNode {
                node: Node::Profile(ProfileProgram {
                    plane: xy,
                    loops: vec![loop_],
                }),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .unwrap()
        .doc;
    doc.apply(
        &DocEdit::InsertNode {
            node: Node::Extrude {
                profile: PROFILE,
                distance: lit(1.0),
            },
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .unwrap()
    .doc
}

fn names_of(doc: &ProfileDoc, id: RecipeNodeId) -> BTreeSet<StableName> {
    let ev = evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    ev.value(id)
        .expect("node evaluates")
        .name_table
        .iter()
        .map(|(n, _)| n.clone())
        .collect()
}

fn anchor_reversed(doc: &ProfileDoc) -> bool {
    let ev = evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let ValuePayload::Profile(pv) = &ev.value(PROFILE).expect("profile").payload else {
        panic!("profile payload");
    };
    pv.naming.loops[0].reversed
}

/// THE demonstration row (§6c): the param edit moves the loop's
/// lexicographic minimum, and neither the canonical start nor the
/// emitted name set moves — the start is the authored one.
#[test]
fn a_parameter_edit_that_moves_the_lex_min_corner_renumbers_nothing() {
    let before = param_rect_doc(0.5);
    let after = param_rect_doc(1.5);
    // The edit genuinely moves the lexicographic minimum…
    let lex_min = |doc: &ProfileDoc| {
        let ev = evaluate::<f64>(
            doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        let ValuePayload::Profile(pv) = &ev.value(PROFILE).expect("profile").payload else {
            panic!("profile payload");
        };
        let vs = pv.validated.loops()[0].vertices();
        (0..vs.len())
            .min_by(|&i, &j| {
                let (p, q) = (vs[i], vs[j]);
                p.x.total_cmp(&q.x).then(p.y.total_cmp(&q.y))
            })
            .unwrap()
    };
    assert_ne!(
        lex_min(&before),
        lex_min(&after),
        "the edit must actually move the lexicographic minimum for this row to \
         demonstrate anything"
    );
    // …while the anchor stays the identity: CCW-authored rectangles
    // never reverse, and the start is the authored one by
    // construction (the denotation check below reads it)…
    assert!(
        !anchor_reversed(&before) && !anchor_reversed(&after),
        "CCW-authored rectangles never reverse"
    );
    // …so the name set is identical…
    assert_eq!(names_of(&before, BODY), names_of(&after, BODY));
    // …AND (set equality alone is renumbering-blind for a full table)
    // the DENOTATION held: in both documents, canonical segment 0 —
    // Lateral(0)'s referent — is the leg leaving the authored entry
    // corner (1, 0).
    for doc in [&before, &after] {
        let ev = evaluate::<f64>(
            doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        let ValuePayload::Profile(pv) = &ev.value(PROFILE).expect("profile").payload else {
            panic!("profile payload");
        };
        let start = pv.validated.loops()[0].vertices()[0];
        assert_eq!(start.x.to_bits(), 1.0_f64.to_bits());
        assert_eq!(start.y.to_bits(), 0.0_f64.to_bits());
    }
}

/// The stable case the spec names (§6b): a circle's radius edit — the
/// carrier form has no authored rotation at all, and names hold.
#[test]
fn circle_radius_edit_keeps_names() {
    let mk = |r: f64| {
        let (doc, xy) = fixture::insert(
            ProfileDoc::empty_derived("switch_naming", Tol::witness()),
            fixture::xy_frame(),
        );
        let doc = doc
            .apply(
                &DocEdit::InsertNode {
                    node: Node::Profile(ProfileProgram {
                        plane: xy,
                        loops: vec![LoopProgram::circle(0.0, 0.0, r).unwrap()],
                    }),
                },
                Tol::witness(),
                &editor_core::RefusingReach,
            )
            .unwrap()
            .doc;
        doc.apply(
            &DocEdit::InsertNode {
                node: Node::Extrude {
                    profile: PROFILE,
                    distance: Expr::literal(1.0, Dimension::Length).unwrap(),
                },
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .unwrap()
        .doc
    };
    assert_eq!(names_of(&mk(0.5), BODY), names_of(&mk(0.75), BODY));
}

/// The freeze-doctrine backstop (§6, unchanged by the resolution): a
/// STRUCTURAL reference beyond the program's step-anchored entities
/// refuses Vanished at resolution — stale selections die loudly, never
/// silently repoint.
#[test]
fn stale_program_refs_refuse_vanished() {
    use editor_core::{CapEnd, EntityKind, ProfileEdgeRef, RoleSeg};
    let doc = param_rect_doc(0.5);
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let ghost = StableName {
        kind: EntityKind::Edge,
        node: BODY,
        path: vec![RoleSeg::RimEdge(
            CapEnd::End,
            ProfileEdgeRef {
                loop_index: 0,
                segment: 9, // the program has 4 segments
            },
        )],
    };
    let table = &ev.value(BODY).expect("extrude").name_table;
    assert!(
        table.lookup(&ghost).is_none(),
        "a ref beyond the program's segments resolves to NOTHING — the \
         Vanished class at every consumer (fillet selections, Declare, \
         appearance), the M6-5 freeze doctrine"
    );
    // And a REAL canonical ref resolves.
    let real = StableName {
        kind: EntityKind::Edge,
        node: BODY,
        path: vec![RoleSeg::RimEdge(
            CapEnd::End,
            ProfileEdgeRef {
                loop_index: 0,
                segment: 0,
            },
        )],
    };
    assert!(table.lookup(&real).is_some());
}

/// Non-identity anchors REMAP structurally: whatever the canonical
/// rotation did, the anchor maps canonical indices back to the
/// PROGRAM's authored order — program vertex 0 is the entry corner
/// (x0, 0) in every binding.
#[test]
fn program_vertex_zero_is_the_authored_entry() {
    for x0 in [0.5, 1.5] {
        let doc = param_rect_doc(x0);
        let ev = evaluate::<f64>(
            &doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        let ValuePayload::Profile(pv) = &ev.value(PROFILE).expect("profile").payload else {
            panic!("profile payload");
        };
        let anchor = pv.naming.loops[0];
        let verts = pv.validated.loops()[0].vertices();
        let canonical_of_program_zero = (0..anchor.len)
            .find(|&k| anchor.vertex(k) == 0)
            .expect("program vertex 0 exists");
        let v = verts[canonical_of_program_zero as usize];
        assert_eq!(
            v.x.to_bits(),
            1.0_f64.to_bits(),
            "entry corner x (x0 = {x0})"
        );
        assert_eq!(v.y.to_bits(), 0.0_f64.to_bits(), "entry corner y");
    }
}

/// **The reversed-loop (hole circle) row — PR #291 review MAJOR-1,
/// both reviewers' probes adopted.** `circle()` lowers CCW;
/// canonicalization orients holes CW, i.e. REVERSES the program loop —
/// and at n = 2 vertex positions cannot decide the parity (forward and
/// reversed vertex maps agree mod 2), so the anchor must read the
/// BULGE bits. Pins: the anchor recovers `reversed: true`, and the
/// §V3 semantics — program segment 0 IS the authored upper (CCW)
/// semicircle — hold under the reversal, checked against the
/// primitive's own lowering.
#[test]
fn hole_circle_anchor_recovers_reversal() {
    let lit = |v: f64| Expr::literal(v, Dimension::Length).unwrap();
    let outer = LoopProgram::polygon([(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]).unwrap();
    let hole = LoopProgram::circle(2.0, 2.0, 0.5).unwrap();
    let (doc, xy) = fixture::insert(
        ProfileDoc::empty_derived("switch_naming", Tol::witness()),
        fixture::xy_frame(),
    );
    let doc = doc
        .apply(
            &DocEdit::InsertNode {
                node: Node::Profile(ProfileProgram {
                    plane: xy,
                    loops: vec![outer, hole],
                }),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .unwrap()
        .doc;
    let doc = doc
        .apply(
            &DocEdit::InsertNode {
                node: Node::Extrude {
                    profile: PROFILE,
                    distance: lit(1.0),
                },
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .unwrap()
        .doc;
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let ValuePayload::Profile(pv) = &ev.value(PROFILE).expect("profile").payload else {
        panic!("profile payload");
    };
    // Canonical loop 1 is the hole; its anchor must say REVERSED
    // (R1's P1 executed the pre-fix mis-recovery {offset: 1,
    // reversed: false} here).
    let a = pv.naming.loops[1];
    assert_eq!(a.program_loop, 1);
    assert!(a.reversed, "a CW-canonicalized CCW hole is REVERSED");
    // The program's own lowering (the primitive, replayed directly):
    // program segment 0 is the arc leaving (cx+r, cy) with bulge +1.
    let program = profile::circle(geom_core::Point2::new(2.0, 2.0), 0.5, Tol::witness())
        .expect("circle lowers")
        .loop_;
    let verts = pv.validated.loops()[1].vertices();
    let n = a.len;
    for c in 0..n {
        let p_seg = a.segment(c) as usize;
        // Canonical segment c is program segment p_seg traversed
        // BACKWARD: it starts at the program segment's END vertex and
        // carries the NEGATED bulge — bit-exact both.
        let p_end = (p_seg + 1) % n as usize;
        assert_eq!(
            verts[c as usize].x.to_bits(),
            program.vertices()[p_end].x.to_bits(),
            "canonical seg {c} starts at program vertex {p_end}"
        );
        let canonical = pv.validated.loops()[1].segments()[c as usize];
        assert_eq!(
            canonical.bulge.to_bits(),
            (-program.bulges()[p_seg]).to_bits(),
            "canonical seg {c} carries program seg {p_seg}'s negated bulge"
        );
        // … and its canonical sweep is the program segment's, negated.
        let (
            profile::SegmentKind::Arc { sweep, .. },
            profile::Segment::Arc {
                sweep: program_sweep,
                ..
            },
        ) = (canonical.kind, program.segments()[p_seg])
        else {
            panic!("a circle's segments are arcs");
        };
        assert_eq!(
            sweep.to_bits(),
            (-program_sweep).to_bits(),
            "canonical seg {c} carries program seg {p_seg}'s negated sweep"
        );
    }
    // Denotation at the name layer: both semicircle walls exist under
    // the hole's CANONICAL indices — its two program segments,
    // reflected.
    use editor_core::{EntityKind, ProfileEdgeRef, RoleSeg};
    let table = &ev.value(BODY).expect("extrude").name_table;
    for seg in 0..2u32 {
        let name = StableName {
            kind: EntityKind::Face,
            node: BODY,
            path: vec![RoleSeg::Lateral(ProfileEdgeRef {
                loop_index: 1,
                segment: seg,
            })],
        };
        assert!(
            table.lookup(&name).is_some(),
            "hole wall Lateral(loop 1, seg {seg}) resolves under program indices"
        );
    }
}
