//! R2's review probes for D′3 (PR 5636): each row states the claim it
//! tries to falsify and prints what it measured.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::{
    Bodies, BodyRead, DocEdit, DocumentId, EvalOptions, Evaluation, Node, Operand, OperandSlot,
    ProfileDoc, RecipeNodeId, SlotId, VarId, apply,
};
use fixture::{len, on_frame, run, square};
use geom_core::Tol;

fn put(doc: ProfileDoc, node: editor_core::AuthoredNode) -> (ProfileDoc, RecipeNodeId) {
    let (doc, minted) = fixture::step(
        doc,
        DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
    );
    (doc, minted.expect("the insert mints the node"))
}

fn block(doc: ProfileDoc, [x, y]: [f64; 2], half: f64, height: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(x, y, half)],
    );
    put(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
        },
    )
}

fn beside(
    doc: ProfileDoc,
    of: RecipeNodeId,
    [x, y, z]: [f64; 3],
    half: f64,
    height: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, body) = block(doc, [x, y], half, height);
    fixture::slid(doc, of, body, [0.0, 0.0, z])
}

fn spelled(reads: &[RecipeNodeId]) -> Bodies<BodyRead<editor_core::Formula>> {
    Bodies::Spelled(reads.iter().map(|&r| r.into()).collect())
}

fn assemble(doc: ProfileDoc, members: &[RecipeNodeId]) -> (ProfileDoc, RecipeNodeId) {
    put(
        doc,
        Node::Assemble {
            members: spelled(members),
        },
    )
}

fn set_product(doc: &ProfileDoc, product: Option<Operand>) -> ProfileDoc {
    apply(doc, &DocEdit::SetProduct { product }, Tol::witness())
        .expect("the slot takes a body in the world")
        .doc
}

fn doc(label: &str) -> ProfileDoc {
    ProfileDoc::empty(DocumentId::derive(label), Tol::witness())
}

fn in_world(doc: ProfileDoc, node: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    put(doc, fixture::stood(node))
}

fn unproven(doc: &ProfileDoc, ev: &Evaluation<f64>) -> Vec<String> {
    editor_core::run_checks(
        doc,
        ev,
        &editor_core::ChecksConfig::default(),
        Tol::witness(),
    )
    .expect("the checks run")
    .findings
    .iter()
    .map(|f| format!("{:?}", f.check))
    .collect()
}

/// A plate and a bolt resting on it, both copied into the world.
fn seat_in_world(label: &str) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = doc(label);
    let (doc, p) = block(doc, [0.0; 2], 0.5, 0.2);
    let (doc, bolt) = beside(doc, p, [0.0, 0.0, 0.2], 0.2, 1.0);
    let (doc, p) = in_world(doc, p);
    let (doc, bolt) = in_world(doc, bolt);
    (doc, p, bolt)
}

fn summary(doc: &ProfileDoc, label: &str) -> (usize, usize, usize, Vec<String>, String) {
    let ev = run(doc, &EvalOptions::default());
    let product = editor_core::product_recorded(doc, &ev, Tol::witness()).expect("it gathers");
    let at_rest = match editor_core::assemble(doc, &ev, Tol::witness()) {
        Ok(_) => "assembles".to_string(),
        Err(e) => format!("{e:?}").chars().take(300).collect(),
    };
    let findings = unproven(doc, &ev);
    eprintln!(
        "R2 {label}: coincidences {} contacts {} census {} findings {findings:?} at-rest {at_rest}",
        product.coincidences.len(),
        product.contacts.len(),
        product.census.len()
    );
    (
        product.coincidences.len(),
        product.contacts.len(),
        product.census.len(),
        findings,
        at_rest,
    )
}

/// Claim 1: a meeting inside an operand is never re-decided by a later
/// `⊔`. `(p ⊔ bolt) ⊔ far` against `p ⊔ bolt` read directly.
#[test]
fn r2_nested_assemble_decides_the_inner_touch_once() {
    let (doc, p, bolt) = seat_in_world("r2-nested");
    let (doc, inner) = assemble(doc, &[p, bolt]);
    let (doc, far) = block(doc, [9.0, 0.0], 0.5, 0.5);
    let (doc, far) = in_world(doc, far);
    let (doc, outer) = assemble(doc, &[inner, far]);
    let direct = set_product(&doc, Some(Operand::Node(inner)));
    let nested = set_product(&doc, Some(Operand::Node(outer)));
    let d = summary(&direct, "product = p ⊔ bolt");
    let n = summary(&nested, "product = (p ⊔ bolt) ⊔ far");
    assert_eq!(
        d.3.len(),
        n.3.len(),
        "the inner touch is reported the same number of times"
    );
    assert_eq!(
        n.0, 0,
        "the outer `⊔` records no coincidence of the inner touch"
    );
}

/// Claim 1, through a placement: the product reads a copy of
/// `p ⊔ bolt`.
#[test]
fn r2_placed_assemble_product() {
    let doc = doc("r2-placed");
    let (doc, p) = block(doc, [0.0; 2], 0.5, 0.2);
    let (doc, bolt) = beside(doc, p, [0.0, 0.0, 0.2], 0.2, 1.0);
    let (doc, inner) = assemble(doc, &[p, bolt]);
    let (doc, copy) = in_world(doc, inner);
    let placed = set_product(&doc, Some(Operand::Node(copy)));
    let s = summary(&placed, "product = copy of (p ⊔ bolt)");
    assert!(
        !s.3.is_empty() || s.0 > 0,
        "the touch inside the copy is decided somewhere"
    );
}

/// Claim 2: a `⊔` of two blocks stacked face to face, cut by a tool
/// touching neither piece's shared face: two pieces, no refusal.
#[test]
fn r2_cut_of_a_face_touching_assemble_keeps_two_pieces() {
    let doc = doc("r2-stack");
    let (doc, a) = block(doc, [0.0; 2], 0.5, 1.0);
    let (doc, b) = beside(doc, a, [0.0, 0.0, 1.0], 0.5, 1.0);
    let (doc, both) = assemble(doc, &[a, b]);
    let (doc, t) = beside(doc, a, [0.5, 0.5, -0.5], 0.2, 3.0);
    let (doc, cut) = put(
        doc,
        Node::Subtract {
            from: both.into(),
            tool: t.into(),
        },
    );
    let (doc, far) = beside(doc, a, [9.0, 0.0, 0.0], 0.2, 0.2);
    let (doc, fused) = put(
        doc,
        Node::Union {
            members: spelled(&[both, far]),
        },
    );
    let ev = run(&doc, &EvalOptions::default());
    for (label, at) in [("cut", cut), ("union", fused)] {
        let solids = ev.value(at).map(|v| match &v.payload {
            editor_core::ValuePayload::Body(b) => b.solids().count(),
            editor_core::ValuePayload::Boolean(editor_core::BooleanValue::Body {
                body, ..
            }) => body.solids().count(),
            other => panic!("{other:?}"),
        });
        eprintln!(
            "R2 stacked {label}: solids {solids:?} error {:?}",
            ev.node_error(at).map(|e| format!("{:?}", e.kind))
        );
    }
    assert!(ev.node_error(cut).is_none(), "the cut does not refuse");
    assert!(ev.node_error(fused).is_none(), "the union does not refuse");
}

/// Claim 3: a placement the slot reads, re-targeted out of the world by
/// the operand-write door.
#[test]
fn r2_product_retargeted_out_of_the_world() {
    let doc = doc("r2-retarget");
    let (doc, a) = block(doc, [0.0; 2], 0.5, 1.0);
    let (doc, copy) = in_world(doc, a);
    let doc = set_product(&doc, Some(Operand::Node(copy)));
    let (doc, space) = put(doc, Node::Space);
    let space_var = doc.output(space, 0).unwrap();
    let moved = apply(
        &doc,
        &DocEdit::SetParam {
            node: copy,
            slot: SlotId::Operand(OperandSlot::Space),
            value: Operand::Var(space_var).into(),
            fresh: Vec::new(),
        },
        Tol::witness(),
    );
    let moved = match moved {
        Ok(applied) => {
            eprintln!("R2 retarget admitted: {:?}", applied.maintenance);
            applied.doc
        }
        Err(e) => {
            eprintln!("R2 retarget refused: {e:?}");
            return;
        }
    };
    eprintln!(
        "R2 product slot {:?}, space_of {:?}",
        moved.product(),
        moved.product().map(|v| moved.space_of(v))
    );
    let ev = run(&moved, &EvalOptions::default());
    let product = editor_core::product(&moved, &ev, Tol::witness());
    eprintln!(
        "R2 product after retarget: {:?}",
        product.as_ref().map(|b| b.solids().count())
    );
    let saved = editor_core::persist::save(&moved, &[], Tol::witness());
    eprintln!(
        "R2 save after retarget: {:?}",
        saved
            .as_ref()
            .map(|_| "saved")
            .map_err(|e| format!("{e:?}"))
    );
    if let Ok(text) = saved {
        let loaded = editor_core::persist::load(&text, Tol::witness());
        eprintln!(
            "R2 load after retarget: {:?}",
            loaded
                .as_ref()
                .map(|_| "loaded")
                .map_err(|e| format!("{e:?}"))
        );
    }
    assert!(
        product.is_err(),
        "a product read outside the world is not a product"
    );
}

/// Claim 5: picks refuse through `−`, and a step naming no operand.
#[test]
fn r2_pick_paths() {
    let doc = doc("r2-picks");
    let (doc, bolt) = block(doc, [0.0; 2], 0.2, 1.0);
    let (doc, plate) = beside(doc, bolt, [2.0, 0.0, 0.0], 0.5, 0.2);
    let (doc, seat) = assemble(doc, &[bolt, plate]);
    let (doc, place) = in_world(doc, seat);
    let bolt_var = doc.output(bolt, 0).unwrap();
    let plate_var = doc.output(plate, 0).unwrap();
    let (doc, cut) = put(
        doc,
        Node::Subtract {
            from: seat.into(),
            tool: plate.into(),
        },
    );
    let (doc, through) = in_world(doc, cut);
    let pick = |at: RecipeNodeId, of: Vec<VarId>| {
        fixture::stood(BodyRead::plain(Operand::pick(Operand::Node(at), of)))
    };
    let attempt = |doc: &ProfileDoc, node| {
        apply(
            doc,
            &DocEdit::InsertNode {
                node: Box::new(node),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
    };
    let through_cut = attempt(&doc, pick(through, vec![bolt_var]));
    eprintln!("R2 pick through −: {:?}", through_cut.as_ref().err());
    assert!(through_cut.is_err(), "a pick through − refuses");
    let bogus = attempt(&doc, pick(place, vec![bolt_var, plate_var]));
    eprintln!("R2 pick past a leaf: {:?}", bogus.as_ref().err());
    assert!(bogus.is_err(), "a step past a leaf refuses");
    let empty = attempt(&doc, pick(place, vec![]));
    eprintln!("R2 empty pick: {:?}", empty.as_ref().err());
    assert!(empty.is_err(), "an empty path refuses");
    let of_seat = attempt(&doc, pick(seat, vec![bolt_var]));
    eprintln!("R2 pick of a ⊔ itself: {:?}", of_seat.as_ref().err());
    // A pick of a pick: the bolt's copy re-placed, picked again.
    let (doc2, again) = put(doc.clone(), pick(place, vec![bolt_var]));
    let (doc2, outer) = in_world(doc2, again);
    let deeper = attempt(&doc2, pick(outer, vec![bolt_var]));
    eprintln!("R2 pick through a pick: {:?}", deeper.as_ref().err());
}

/// Claim 3: the part's pin moves with its product slot (an instance of
/// it must not be served a memo of the other product).
#[test]
fn r2_pin_moves_with_the_slot() {
    let (doc, p, bolt) = seat_in_world("r2-pin");
    let a = set_product(&doc, Some(Operand::Node(p)));
    let b = set_product(&doc, Some(Operand::Node(bolt)));
    let pa = editor_core::persist::content_pin(&a, Tol::witness()).unwrap();
    let pb = editor_core::persist::content_pin(&b, Tol::witness()).unwrap();
    assert_ne!(pa, pb, "the pin keys the product slot");
}

/// Claim 7: main's golden document, saved before D′3, refuses typed.
#[test]
fn r2_old_wire_refuses_typed() {
    let Ok(path) = std::env::var("R2_OLD_DOC") else {
        eprintln!("R2_OLD_DOC unset");
        return;
    };
    let text = std::fs::read_to_string(path).unwrap();
    let loaded = editor_core::persist::load(&text, Tol::witness());
    eprintln!(
        "R2 old wire: {:?}",
        loaded
            .as_ref()
            .map(|_| "loaded")
            .map_err(|e| format!("{e}"))
    );
    assert!(loaded.is_err());
}

/// Controls for the stacked cut: the same tool through `a` alone, through
/// `a ∪ b`, and a tool that cuts `a` without reaching the shared face.
#[test]
fn r2_stacked_cut_controls() {
    let doc = doc("r2-stack-controls");
    let (doc, a) = block(doc, [0.0; 2], 0.5, 1.0);
    let (doc, b) = beside(doc, a, [0.0, 0.0, 1.0], 0.5, 1.0);
    let (doc, both) = assemble(doc, &[a, b]);
    let (doc, fused) = put(
        doc,
        Node::Union {
            members: spelled(&[a, b]),
        },
    );
    let (doc, through) = beside(doc, a, [0.5, 0.5, -0.5], 0.2, 3.0);
    let (doc, low) = beside(doc, a, [0.5, 0.5, -0.5], 0.2, 1.0);
    let (doc, hole) = beside(doc, a, [0.0, 0.0, -0.5], 0.1, 3.0);
    let mut rows = Vec::new();
    let mut doc = doc;
    for (label, from, tool) in [
        ("a − through", a, through),
        ("(a ∪ b) − through", fused, through),
        ("(a ⊔ b) − through", both, through),
        ("(a ⊔ b) − low", both, low),
        ("(a ⊔ b) − hole", both, hole),
        ("(a ∪ b) − hole", fused, hole),
    ] {
        let (d, cut) = put(
            doc,
            Node::Subtract {
                from: from.into(),
                tool: tool.into(),
            },
        );
        doc = d;
        rows.push((label, cut));
    }
    let ev = run(&doc, &EvalOptions::default());
    for (label, at) in rows {
        let solids = ev.value(at).map(|v| match &v.payload {
            editor_core::ValuePayload::Body(b) => b.solids().count(),
            editor_core::ValuePayload::Boolean(editor_core::BooleanValue::Body {
                body, ..
            }) => body.solids().count(),
            _ => usize::MAX,
        });
        eprintln!(
            "R2 control {label}: solids {solids:?} error {:?}",
            ev.node_error(at).map(|e| format!("{:?}", e.kind))
        );
    }
}

/// Claim 1, isolated: a document whose only `⊔`s are `p ⊔ bolt` and
/// `(p ⊔ bolt) ⊔ far` against one with `p ⊔ bolt` alone.
#[test]
fn r2_nested_assemble_isolated() {
    let (doc, p, bolt) = seat_in_world("r2-nested-iso");
    let (doc, inner) = assemble(doc, &[p, bolt]);
    let alone = set_product(&doc, Some(Operand::Node(inner)));
    let a = summary(&alone, "only p ⊔ bolt");
    let (doc, far) = block(doc, [9.0, 0.0], 0.5, 0.5);
    let (doc, far) = in_world(doc, far);
    let (doc, outer) = assemble(doc, &[inner, far]);
    let nested = set_product(&doc, Some(Operand::Node(outer)));
    let n = summary(&nested, "(p ⊔ bolt) ⊔ far");
    assert_eq!(a.4, "assembles", "p ⊔ bolt alone assembles");
    assert_eq!(
        n.4, "assembles",
        "the outer ⊔ does not re-decide the inner seat"
    );
}

/// Claim 5: a pick through a copy whose source is already a pick.
#[test]
fn r2_pick_of_a_pick_evaluates() {
    let doc = doc("r2-pick-pick");
    let (doc, bolt) = block(doc, [0.0; 2], 0.2, 1.0);
    let (doc, plate) = beside(doc, bolt, [2.0, 0.0, 0.0], 0.5, 0.2);
    let (doc, seat) = assemble(doc, &[bolt, plate]);
    let (doc, place) = in_world(doc, seat);
    let bolt_var = doc.output(bolt, 0).unwrap();
    let plate_var = doc.output(plate, 0).unwrap();
    let pick = |at: RecipeNodeId, of: Vec<VarId>| {
        fixture::stood(BodyRead::plain(Operand::pick(Operand::Node(at), of)))
    };
    let (doc, again) = put(doc, pick(place, vec![bolt_var]));
    let (doc, outer) = in_world(doc, again);
    let (doc, deeper) = put(doc, pick(outer, vec![bolt_var]));
    let wrong = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(pick(outer, vec![plate_var])),
            fresh: Vec::new(),
        },
        Tol::witness(),
    );
    eprintln!(
        "R2 pick of the plate through the bolt's pick: {:?}",
        wrong.as_ref().err().map(|e| format!("{e:?}"))
    );
    let (doc, wrong) = match wrong {
        Ok(a) => {
            let id = a.record.minted.unwrap();
            (a.doc, Some(id))
        }
        Err(_) => (doc, None),
    };
    let ev = run(&doc, &EvalOptions::default());
    eprintln!(
        "R2 pick of a pick: value {:?} error {:?}",
        ev.value(deeper).is_some(),
        ev.node_error(deeper).map(|e| format!("{:?}", e.kind))
    );
    if let Some(w) = wrong {
        eprintln!(
            "R2 plate through bolt pick: value {:?} error {:?}",
            ev.value(w).is_some(),
            ev.node_error(w).map(|e| format!("{:?}", e.kind))
        );
    }
}

/// Claim 6: each corpus document's node count, `⊔` count and placement
/// count, and what the slot reads, for the rebuild-latency re-pins.
#[test]
fn r2_corpus_assemble_census() {
    for d in crate::corpus::documents() {
        let doc = &d.doc;
        let ids = doc.ids();
        let assembles: Vec<_> = ids
            .iter()
            .filter_map(|&id| match doc.node(id) {
                Some(Node::Assemble { members }) => Some(members.rows().len()),
                _ => None,
            })
            .collect();
        let places = ids
            .iter()
            .filter(|&&id| matches!(doc.node(id), Some(Node::Place { .. })))
            .count();
        let slot = doc
            .product()
            .and_then(|v| doc.operation_of(v))
            .and_then(|at| doc.node(at))
            .map(|n| format!("{n:?}").chars().take(12).collect::<String>());
        eprintln!(
            "R2 corpus {}: nodes {} assembles {assembles:?} places {places} slot {slot:?}",
            d.name,
            d.len()
        );
    }
}
