//! **LIB-SWITCH §6: program-anchored naming, pinned.**
//!
//! Profile-entity naming for program loops anchors to PROGRAM-
//! STRUCTURAL positions (the ratified §V3 round-2 resolution): nothing
//! geometric enters the index, so a parameter edit CANNOT renumber —
//! including the very edit class that renumbers the CANONICAL indices
//! (a lex-min-swapping rectangle). The freeze doctrine remains the
//! structural-edit backstop (stale program refs refuse Vanished).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

mod fixture;

use editor_core::{
    CancelToken, Dimension, DocEdit, DocParam, EvalOptions, Expr, LoopProgram, Node, ParamName,
    ProfileDoc, ProfileProgram, ProgramStep, ProgramTarget, RecipeNodeId, StableName, ValuePayload,
    evaluate,
};
use geom_core::Tol;

/// A quad whose LAST authored corner x is a document parameter: at
/// x0 = 0.5 that corner (0.5, 1) is lex-min (canonical offset 3); at
/// x0 = 1.5 the ENTRY corner (1, 0) is — the canonical rotation
/// changes under a pure parameter edit, the §6 renumbering class on
/// the nose.
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
        )
        .unwrap()
        .doc;
    doc.apply(
        &DocEdit::InsertNode {
            node: Node::Extrude {
                profile: RecipeNodeId(0),
                distance: lit(1.0),
            },
        },
        Tol::witness(),
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

fn anchor_offset(doc: &ProfileDoc) -> (u32, bool) {
    let ev = evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let ValuePayload::Profile(pv) = &ev.value(RecipeNodeId(0)).expect("profile").payload else {
        panic!("profile payload");
    };
    let a = pv.naming.loops[0];
    (a.offset, a.reversed)
}

/// THE demonstration row (§6c, inverted by the resolution): the param
/// edit MOVES lex-min — the canonical rotation demonstrably changes —
/// and the emitted name set does NOT move: program anchoring ate the
/// renumbering class.
#[test]
fn lex_min_swap_cannot_renumber_program_names() {
    let before = param_rect_doc(0.5);
    let after = param_rect_doc(1.5);
    // The canonicalization genuinely rotated (the §6 measurement's
    // renumbering class is REAL on the canonical substrate)…
    let (off_before, rev_b) = anchor_offset(&before);
    let (off_after, rev_a) = anchor_offset(&after);
    assert!(!rev_b && !rev_a, "CCW-authored rectangles never reverse");
    assert_ne!(
        off_before, off_after,
        "the edit must actually swap the lex-min vertex for this row to demonstrate anything"
    );
    // …and the NAME substrate did not: identical name sets…
    assert_eq!(
        names_of(&before, RecipeNodeId(1)),
        names_of(&after, RecipeNodeId(1))
    );
    // …AND (review MINOR-1: set equality alone is renumbering-blind
    // for a full table) the DENOTATION held: in both documents,
    // program segment 0 — Lateral(0)'s referent — is the leg leaving
    // the authored entry corner (1, 0), read through the anchor.
    for doc in [&before, &after] {
        let ev = evaluate::<f64>(
            doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        let ValuePayload::Profile(pv) = &ev.value(RecipeNodeId(0)).expect("profile").payload else {
            panic!("profile payload");
        };
        let a = pv.naming.loops[0];
        let c = (0..a.len)
            .find(|&k| a.segment(k) == 0)
            .expect("program segment 0 exists");
        let start = pv.validated.loops()[0].vertices()[c as usize].pos();
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
            )
            .unwrap()
            .doc;
        doc.apply(
            &DocEdit::InsertNode {
                node: Node::Extrude {
                    profile: RecipeNodeId(0),
                    distance: Expr::literal(1.0, Dimension::Length).unwrap(),
                },
            },
            Tol::witness(),
        )
        .unwrap()
        .doc
    };
    assert_eq!(
        names_of(&mk(0.5), RecipeNodeId(1)),
        names_of(&mk(0.75), RecipeNodeId(1))
    );
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
        node: RecipeNodeId(1),
        path: vec![RoleSeg::RimEdge(
            CapEnd::Top,
            ProfileEdgeRef {
                loop_index: 0,
                segment: 9, // the program has 4 segments
            },
        )],
    };
    let table = &ev.value(RecipeNodeId(1)).expect("extrude").name_table;
    assert!(
        table.lookup(&ghost).is_none(),
        "a ref beyond the program's segments resolves to NOTHING — the \
         Vanished class at every consumer (fillet selections, Declare, \
         appearance), the M6-5 freeze doctrine"
    );
    // And a REAL program-anchored ref resolves.
    let real = StableName {
        kind: EntityKind::Edge,
        node: RecipeNodeId(1),
        path: vec![RoleSeg::RimEdge(
            CapEnd::Top,
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
        let ValuePayload::Profile(pv) = &ev.value(RecipeNodeId(0)).expect("profile").payload else {
            panic!("profile payload");
        };
        let anchor = pv.naming.loops[0];
        let verts = pv.validated.loops()[0].vertices();
        let canonical_of_program_zero = (0..anchor.len)
            .find(|&k| anchor.vertex(k) == 0)
            .expect("program vertex 0 exists");
        let v = verts[canonical_of_program_zero as usize].pos();
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
        )
        .unwrap()
        .doc;
    let doc = doc
        .apply(
            &DocEdit::InsertNode {
                node: Node::Extrude {
                    profile: RecipeNodeId(0),
                    distance: lit(1.0),
                },
            },
            Tol::witness(),
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
    let ValuePayload::Profile(pv) = &ev.value(RecipeNodeId(0)).expect("profile").payload else {
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
            verts[c as usize].pos().x.to_bits(),
            program.vertices()[p_end].pos().x.to_bits(),
            "canonical seg {c} starts at program vertex {p_end}"
        );
        assert_eq!(
            verts[c as usize].bulge().to_bits(),
            (-program.vertices()[p_seg].bulge()).to_bits(),
            "canonical seg {c} carries program seg {p_seg}'s negated bulge"
        );
    }
    // Denotation at the name layer: both program-anchored semicircle
    // walls exist under the hole's PROGRAM indices.
    use editor_core::{EntityKind, ProfileEdgeRef, RoleSeg};
    let table = &ev.value(RecipeNodeId(1)).expect("extrude").name_table;
    for seg in 0..2u32 {
        let name = StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(1),
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
