//! R2 review probes for INTENT stage 2 F (PR #4566). Not for merging:
//! each row states what the spec or the PR claims, and goes red where
//! the branch disagrees.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::{
    Alignment, AssemblyError, AxisSense, CapEnd, ContactClass, DocEdit, DocumentId, ExtrudeSide,
    Formula, MateFrame, MatePrimitive, MintRefusal, Node, OperandSlot, PatternKind, ProfileDoc,
    RecipeNodeId, RefusedRef, StableName, assemble,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{in_copy, insert, len, on_frame, run, scl, step};
use geom_core::Tol;

fn cube_part(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    )
}

fn frame(origin: [f64; 3]) -> MateFrame<Formula> {
    MateFrame::authored(origin, [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], Tol::witness())
        .expect("a definite frame")
}

fn seat_names(a: StableName, b: StableName, face_based: bool) -> editor_core::AuthoredNode {
    let (fa, fb) = if face_based {
        (MateFrame::from_face(), MateFrame::from_face())
    } else {
        (frame([0.0, 0.0, 1.0]), frame([0.0, 0.0, 0.0]))
    };
    Node::Mate {
        a: fixture::head(a).into(),
        b: fixture::head(b).into(),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: fa,
            b: fb,
            primitive: MatePrimitive::FrameCoincidence,
            sense: if face_based {
                AxisSense::Opposed
            } else {
                AxisSense::Aligned
            },
            clocking: None,
        },
    }
}

fn pair(label: &str) -> (ProfileDoc, [RecipeNodeId; 2], PartStore, RecipeNodeId) {
    let mut store = PartStore::default();
    let (doc_ref, body) = store.insert_part(cube_part(&format!("{label}-part")), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, a) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, b) = insert(doc, fixture::mated_instance(doc_ref));
    (doc, [a, b], store, body)
}

fn ghost_of(b: RecipeNodeId) -> StableName {
    StableName {
        kind: editor_core::EntityKind::Face,
        node: b,
        path: vec![editor_core::RoleSeg::InPart {
            of: StableName {
                kind: editor_core::EntityKind::Face,
                node: RecipeNodeId::new(0, test_utils::refusal::tagged(77)),
                path: vec![editor_core::RoleSeg::Cap(CapEnd::Start)],
            }
            .into(),
        }],
    }
}

fn break_side_b(doc: ProfileDoc, mate: RecipeNodeId, b: RecipeNodeId) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetParam {
            node: mate,
            slot: editor_core::SlotId::Operand(OperandSlot::Side(editor_core::MateSide::B)),
            value: editor_core::SlotValue::Read(fixture::head_at(b, ghost_of(b)).into()),
            fresh: Vec::new(),
        },
    )
    .0
}

/// Spec §7 / test 22: "A mate whose members are not both placed mints
/// nothing ... that is the boolean's coincidence door's business, not
/// an at-rest fact of the product"; spec §1: "a Measure, Assertion or
/// Mate can never gate it". A mate on a boolean's unplaced operands
/// whose side stops resolving must therefore not gate the product.
#[test]
fn p1_a_broken_mate_on_unplaced_operands_does_not_gate_the_product() {
    let (doc, [a, b], store, body) = pair("r2-p1");
    let (doc, mate) = insert(
        doc,
        seat_names(
            in_part(a, body, CapEnd::End),
            in_part(b, body, CapEnd::Start),
            false,
        ),
    );
    let (doc, fused) = insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a: a.into(),
            b: b.into(),
            declare: Vec::new(),
        },
    );
    let doc = fixture::place_all(doc, &[fused]);
    let opts = with_resolver(store);
    let ev = run(&doc, &opts);
    assert!(
        assemble(&doc, &ev, Tol::witness()).is_ok(),
        "healthy: gate passes"
    );
    let doc = break_side_b(doc, mate, b);
    let ev = run(&doc, &opts);
    assert!(ev.node_error(mate).is_some(), "the mate fails");
    eprintln!(
        "product_recorded: {:?}",
        editor_core::product_recorded(&doc, &ev, Tol::witness()).err()
    );
    match assemble(&doc, &ev, Tol::witness()) {
        Ok(_) => {}
        Err(e) => panic!("an unplaced mate's failure gated the product: {e:?}"),
    }
}

/// Spec §7: "`FaceUnresolved` becomes the select's resolve error". A
/// face-based side whose name stops resolving should refuse in the
/// selection's words (as the PR's authored-frame row does).
#[test]
fn p5_a_face_based_side_that_vanishes_refuses_in_the_selections_words() {
    let (doc, [a, b], store, body) = pair("r2-p5");
    let doc = fixture::place_all(doc, &[a, b]);
    let opts = with_resolver(store);
    let reach = editor_core::mate_reach::<f64>(&opts, Tol::witness());
    let (doc, mate) = fixture::step_with(
        doc,
        DocEdit::InsertNode {
            node: Box::new(seat_names(
                in_part(a, body, CapEnd::End),
                in_part(b, body, CapEnd::Start),
                true,
            )),
            fresh: Vec::new(),
        },
        &reach,
    );
    let mate = mate.unwrap();
    let ev = run(&doc, &opts);
    assert!(ev.node_error(mate).is_none(), "{:?}", ev.node_error(mate));
    let doc = break_side_b(doc, mate, b);
    let ev = run(&doc, &opts);
    assert!(
        matches!(
            ev.node_error(mate).map(|e| &e.kind),
            Some(editor_core::NodeErrorKind::SelectResolve { .. })
        ),
        "{:?}",
        ev.node_error(mate).map(|e| &e.kind)
    );
}

// ---- the copy rule over a pattern's picks ----

const COPY: u32 = 2;

fn linear(spacing: f64) -> PatternKind<Formula> {
    PatternKind::Linear {
        direction: [scl(1.0), scl(0.0), scl(0.0)],
        spacing: len(spacing),
    }
}

/// A leg pattern (count 4, spacing 2) and a top seated on copy COPY,
/// with the given copies placed (as `Part` picks, in the given order).
fn legs(
    label: &str,
    picks: &[i64],
) -> (
    ProfileDoc,
    editor_core::EvalOptions,
    RecipeNodeId,
    RecipeNodeId,
) {
    let (doc, store, mate, top, _) = legs_at(label, picks, false);
    (doc, with_resolver(store), mate, top)
}

/// [`legs`], the leg side read at the pick of copy COPY when `at_pick`
/// (the first such pick), else at the pattern; also returns the store
/// and every node it inserted.
fn legs_at(
    label: &str,
    picks: &[i64],
    at_pick: bool,
) -> (
    ProfileDoc,
    PartStore,
    RecipeNodeId,
    RecipeNodeId,
    Vec<RecipeNodeId>,
) {
    let mut store = PartStore::new();
    let (leg_ref, leg_body) = store.insert_part(cube_part(&format!("{label}-leg")), Tol::witness());
    let (top_ref, top_body) = store.insert_part(cube_part(&format!("{label}-top")), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg.into(),
            count: Formula::count(4),
            kind: linear(2.0),
        },
    );
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let mut nodes = vec![leg, pattern, top];
    let mut pick_of_copy = None;
    let doc = picks.iter().fold(doc, |doc, &i| {
        let (doc, copy) = insert(
            doc,
            Node::Part {
                of: pattern.into(),
                select: editor_core::PartSelect::Instance(Formula::count(i)),
            },
        );
        if i == i64::from(COPY) && pick_of_copy.is_none() {
            pick_of_copy = Some(copy);
        }
        nodes.push(copy);
        let (doc, pl) = fixture::place(doc, copy);
        nodes.push(pl);
        doc
    });
    let (doc, pl) = fixture::place(doc, top);
    nodes.push(pl);
    let leg_name = in_copy(pattern, COPY, in_part(leg, leg_body, CapEnd::End));
    let a_side = if at_pick {
        fixture::head_at(pick_of_copy.expect("COPY is picked"), leg_name)
    } else {
        fixture::head(leg_name)
    };
    let (doc, mate) = insert(
        doc,
        Node::Mate {
            a: a_side.into(),
            b: fixture::head(in_part(top, top_body, CapEnd::Start)).into(),
            class: ContactClass::Rest,
            alignment: Alignment {
                a: frame([0.0, 0.0, 1.0]),
                b: frame([0.0, 0.0, 0.0]),
                primitive: MatePrimitive::FrameCoincidence,
                sense: AxisSense::Aligned,
                clocking: None,
            },
        },
    );
    nodes.push(mate);
    (doc, store, mate, top, nodes)
}

/// The mate mints on the pick of copy COPY whatever order the picks
/// are placed in, and the gate (which checks the top really rests on
/// THAT copy) certifies it.
#[test]
fn p2_a_pattern_side_mints_on_the_pick_of_its_copy_in_any_order() {
    for picks in [&[0, 1, 2, 3][..], &[3, 2, 1, 0], &[2], &[1, 3, 2]] {
        let (doc, opts, mate, _) = legs("r2-p2", picks);
        let ev = run(&doc, &opts);
        assert!(
            ev.node_error(mate).is_none(),
            "{picks:?}: {:?}",
            ev.node_error(mate)
        );
        let (minted, unminted) = fixture::mate_mints(&doc, &ev);
        assert!(unminted.is_empty(), "{picks:?}: {unminted:?}");
        assert_eq!(minted.len(), 1, "{picks:?}");
        let r = assemble(&doc, &ev, Tol::witness());
        assert!(r.is_ok(), "{picks:?}: {:?}", r.err());
    }
}

/// With copy COPY unplaced the mate mints nothing.
#[test]
fn p3_a_pattern_side_whose_copy_is_not_picked_mints_nothing() {
    let (doc, opts, mate, _) = legs("r2-p3", &[0, 1, 3]);
    let ev = run(&doc, &opts);
    fixture::assert_mints_nothing(&doc, &ev, mate);
}

/// Copy COPY picked twice (two `Part` nodes) is two world copies.
#[test]
fn p4_a_copy_picked_twice_refuses_ambiguous() {
    let (doc, opts, mate, _) = legs("r2-p4", &[2, 2]);
    let ev = run(&doc, &opts);
    let (_, unminted) = fixture::mate_mints(&doc, &ev);
    assert!(
        matches!(
            unminted.as_slice(),
            [MintRefusal::Reference { mate: m, why: RefusedRef::Ambiguous { width: 2 }, .. }] if *m == mate
        ),
        "{unminted:?}"
    );
    let _ = AssemblyError::Mint { refusals: vec![] };
}

// ---- claim 9: the edit and persist doors agree ----

fn side_vars(doc: &ProfileDoc, mate: RecipeNodeId) -> (editor_core::VarId, editor_core::VarId) {
    match doc.node(mate) {
        Some(Node::Mate { a, b, .. }) => (*a, *b),
        _ => panic!("a mate"),
    }
}

/// A pattern-side mate saves and loads.
#[test]
fn p9a_a_pattern_side_mate_round_trips() {
    let (doc, _, _, _) = legs("r2-p9a", &[2]);
    let bytes = editor_core::save(&doc, &[], Tol::witness()).expect("saves");
    let back = editor_core::load(&bytes, Tol::witness());
    assert!(back.is_ok(), "{:?}", back.err());
}

/// The selection a mate side minted over a pattern's `Bodies`, read
/// again by a measure: whatever the edit door decides, the load door
/// must decide the same (a document the edit door builds must load).
#[test]
fn p9b_a_bodies_selection_shared_with_a_measure_agrees_at_both_doors() {
    let (doc, _, mate, _) = legs("r2-p9b", &[2]);
    let (sa, sb) = side_vars(&doc, mate);
    let doc = step(
        doc,
        DocEdit::RenameVar {
            var: sa.into(),
            name: Some(editor_core::VarName::new("leg_top".to_owned()).unwrap()),
        },
    )
    .0;
    let doc = step(
        doc,
        DocEdit::RenameVar {
            var: sb.into(),
            name: Some(editor_core::VarName::new("top_bottom".to_owned()).unwrap()),
        },
    )
    .0;
    let refs = [editor_core::Operand::Var(sa), editor_core::Operand::Var(sb)];
    let prim = fixture::sited(
        editor_core::MeasurePrimitive::Distance { a: 0, b: 1 },
        &refs,
    );
    let edit = editor_core::measure(&doc, &[prim], Tol::witness(), &editor_core::RefusingReach);
    match edit {
        Err(e) => panic!("edit door refuses (then it agrees with load; check the message): {e:?}"),
        Ok(out) => {
            let doc = out.doc;
            let bytes = editor_core::save(&doc, &[], Tol::witness()).expect("saves");
            let back = editor_core::load(&bytes, Tol::witness());
            assert!(
                back.is_ok(),
                "the edit door built a document the load door refuses: {:?}",
                back.err()
            );
        }
    }
}

// ---- claim 5: inline(split(d)) == d ----

fn round_trip(label: &str, at_pick: bool) {
    let (doc, store, mate, _, nodes) = legs_at(label, &[2], at_pick);
    let opts = with_resolver(store.clone());
    let ev = run(&doc, &opts);
    assert!(ev.node_error(mate).is_none(), "{:?}", ev.node_error(mate));
    assert!(assemble(&doc, &ev, Tol::witness()).is_ok(), "gates");
    let cut = fixture::with_placements(&doc, &nodes.iter().copied().collect());
    let out = editor_core::split(
        &doc,
        &cut,
        DocumentId::derive(&format!("{label}-part")),
        Tol::witness(),
        opts.resolver.as_ref(),
    )
    .unwrap_or_else(|e| panic!("the whole group cuts: {e}"));
    let mut store = store;
    store.insert(out.part.clone(), Tol::witness());
    let back = editor_core::inline(
        &out.remainder,
        out.instance,
        &crate::p2_gauges::resolver(store.clone()),
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("inlines back: {e}"));
    let (map, steps) = fixture::round_trip::composed(&doc, &out, &back);
    fixture::round_trip::same_up_to_ids(&doc, &back.doc, &map, &steps)
        .unwrap_or_else(|e| panic!("inline(split(d)) is d up to node ids:\n{e}"));
}

#[test]
fn p6a_round_trip_a_mate_read_at_the_pattern() {
    round_trip("r2-p6a", false);
}

#[test]
fn p6b_round_trip_a_mate_read_at_a_pick() {
    round_trip("r2-p6b", true);
}

// ---- claim 5: a selection crossing a POSED cut placement ----

fn cap(node: RecipeNodeId, end: CapEnd) -> StableName {
    StableName {
        kind: editor_core::EntityKind::Face,
        node,
        path: vec![editor_core::RoleSeg::Cap(end)],
    }
}

fn block(doc: ProfileDoc, height: f64) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let before = doc.ids();
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(height),
            side: ExtrudeSide::Along,
        },
    );
    let nodes = doc
        .ids()
        .into_iter()
        .filter(|i| !before.contains(i))
        .collect();
    (doc, nodes)
}

/// A remainder measure reads a face of a cut body whose world placement
/// is POSED. Pre-split it measures the body where it is built; the
/// split must either refuse (as main's `world_heir` did for every
/// crossing read) or leave the measured value unchanged.
#[test]
fn p7_a_selection_crossing_a_posed_cut_placement_keeps_its_value() {
    let doc = ProfileDoc::empty(DocumentId::derive("r2-p7"), Tol::witness());
    let (doc, b_nodes) = block(doc, 1.0);
    let b = *b_nodes.last().unwrap();
    let (doc, c_nodes) = block(doc, 2.0);
    let c = *c_nodes.last().unwrap();
    let (doc, pl_b) = step(
        doc,
        DocEdit::place(
            editor_core::Operand::Node(b),
            Some(editor_core::Placement::literal(
                &editor_core::Frame::translation([0.0, 0.0, 10.0]),
            )),
        ),
    );
    let pl_b = pl_b.unwrap();
    let (doc, _) = fixture::place(doc, c);
    let (doc, m) = fixture::measure_node(
        &doc,
        editor_core::MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            editor_core::SitedRef::at_mint(cap(b, CapEnd::End)),
            editor_core::SitedRef::at_mint(cap(c, CapEnd::End)),
        ],
    );
    let var = doc.output(m, 0).unwrap();
    let ev = run(&doc, &editor_core::EvalOptions::default());
    let before = fixture::reading(&doc, &ev, var);
    eprintln!("before split: {before:?}");
    let mut cut: std::collections::BTreeSet<_> = b_nodes.iter().copied().collect();
    cut.insert(pl_b);
    match editor_core::split(
        &doc,
        &cut,
        DocumentId::derive("r2-p7-part"),
        Tol::witness(),
        None,
    ) {
        Err(e) => eprintln!("split refuses: {e:?}"),
        Ok(out) => {
            let mut store = PartStore::new();
            store.insert(out.part.clone(), Tol::witness());
            let ev2 = run(&out.remainder, &with_resolver(store.clone()));
            let after = fixture::reading(&out.remainder, &ev2, var);
            eprintln!("after split: {after:?}");
            let back = editor_core::inline(
                &out.remainder,
                out.instance,
                &crate::p2_gauges::resolver(store.clone()),
                Tol::witness(),
            );
            eprintln!("inline back: {:?}", back.as_ref().err());
            assert_eq!(
                before, after,
                "the split moved what a remainder measure measures"
            );
        }
    }
}
