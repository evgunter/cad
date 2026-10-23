//! INTENT stage 3 D′3 acceptance — `⊔` is an operator, the product is
//! a slot, a placement copies one source and a reader of one piece of a
//! copy holds a pick (`docs/INTENT-STAGE3-SPEC.md` §5′.3, tests 19, 25
//! and 26).
//!
//! A copy carries its source's names as they are, and an operator keys
//! each operand's names by its read, so no name spells a placement. The
//! census runs at each `⊔`, between its operands, whether or not the
//! product slot reads it. A body of several parts is an ordinary
//! operand: an operator reading one keeps the meetings inside it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::{
    Bodies, BodyRead, DocEdit, DocumentId, EditError, EvalOptions, Evaluation, Node, Operand, ProfileDoc, RecipeNodeId, RoleSeg, SplitHalf, ValuePayload, VarId, apply,
};
use fixture::{len, on_frame, run, square};
use geom_core::Tol;

/// A block of half-width `half` centred at `(x, y)`, from `0` to
/// `height`, a construction in a space of its own.
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

/// A block brought into `of`'s space, raised to stand on `z`.
fn beside(doc: ProfileDoc, of: RecipeNodeId, [x, y, z]: [f64; 3], half: f64, height: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, body) = block(doc, [x, y], half, height);
    fixture::slid(doc, of, body, [0.0, 0.0, z])
}

/// Inserts `node` as written: the fixture's `insert` makes a world
/// placement join the product, and these rows say what the product is.
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

fn spelled(reads: &[RecipeNodeId]) -> Bodies<BodyRead<editor_core::Formula>> {
    Bodies::Spelled(reads.iter().map(|&r| r.into()).collect())
}

fn set_product(doc: &ProfileDoc, product: Option<Operand>) -> ProfileDoc {
    apply(doc, &DocEdit::SetProduct { product }, Tol::witness())
        .expect("the slot takes a body in the world")
        .doc
}

fn body_of(ev: &Evaluation<f64>, node: RecipeNodeId) -> topo::Body<f64> {
    match ev.value(node).map(|v| &v.payload) {
        Some(ValuePayload::Body(b)) => (**b).clone(),
        Some(ValuePayload::Boolean(editor_core::BooleanValue::Body { body, .. })) => (**body).clone(),
        other => panic!("{node:?} reads one body, not {other:?}: {:?}", ev.result(node)),
    }
}

fn bounds(body: &topo::Body<f64>) -> ([f64; 3], [f64; 3]) {
    let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    for (_, p) in body.points() {
        for (k, c) in p.to_array().into_iter().enumerate() {
            lo[k] = lo[k].min(c);
            hi[k] = hi[k].max(c);
        }
    }
    (lo, hi)
}

fn solids(body: &topo::Body<f64>) -> usize {
    body.solids().count()
}

fn near(a: ([f64; 3], [f64; 3]), b: ([f64; 3], [f64; 3])) -> bool {
    a.0.iter().chain(&a.1).zip(b.0.iter().chain(&b.1)).all(|(x, y)| (x - y).abs() < 1e-12)
}

fn doc(label: &str) -> ProfileDoc {
    ProfileDoc::empty(DocumentId::derive(label), Tol::witness())
}

/// `node`'s value placed in the world and made the product.
fn produced(doc: ProfileDoc, node: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    let (doc, place) = put(doc, fixture::stood(node));
    (set_product(&doc, Some(Operand::Node(place))), place)
}

// ---- Test 19: picks ----

/// **Two placements of one body into one space, unioned, name each
/// operand by its read.** The union's names carried from an operand are
/// `From { read }` of that copy's read, and what each wraps is the
/// block's own name: a copy carries its source's names verbatim, so no
/// name spells the placement.
///
/// *Breaks if* a name spells the placement.
#[test]
fn two_copies_unioned_name_each_operand_by_its_read() {
    let doc = doc("dprime3-two-copies");
    let (doc, a) = block(doc, [0.0; 2], 0.5, 1.0);
    let (doc, near) = fixture::stood_in(doc, a, a);
    let (doc, far) = fixture::slid(doc, a, a, [0.5, 0.0, 0.0]);
    let (doc, union) = put(doc, Node::Union { members: spelled(&[near, far]) });
    let ev = run(&doc, &EvalOptions::default());
    let reads = [doc.output(near, 0).unwrap(), doc.output(far, 0).unwrap()];
    let table = ev.table(doc.output(union, 0).unwrap()).expect("the union's table");
    let mut seen = std::collections::BTreeSet::new();
    for (name, _) in table.iter() {
        let Some(RoleSeg::From { read, of }) = name.path.first() else {
            continue;
        };
        assert!(reads.contains(read), "keyed by a member's read: {name:?}");
        assert_eq!(of.node, a, "a copy carries the block's names: {name:?}");
        seen.insert(*read);
    }
    assert_eq!(seen.len(), 2, "both copies' names reach the union");
}

/// **A pick of one piece of a copy of `bolt ⊔ plate` reads the bolt's
/// copy**; picking it twice gives two variables; a pick through a `∪`
/// refuses at the door.
///
/// *Breaks if* a reader reads one piece of a copy without a pick.
#[test]
fn a_pick_reads_one_piece_and_two_picks_are_two_variables() {
    let doc = doc("dprime3-pick");
    let (doc, bolt) = block(doc, [0.0; 2], 0.2, 1.0);
    let (doc, plate) = beside(doc, bolt, [2.0, 0.0, 0.0], 0.5, 0.2);
    let (doc, seat) = put(doc, Node::Assemble { members: spelled(&[bolt, plate]) });
    let (doc, place) = put(doc, fixture::stood(seat));
    let bolt_var = doc.output(bolt, 0).unwrap();
    let pick = || BodyRead::plain(Operand::pick(Operand::Node(place), vec![bolt_var]));
    let (doc, first) = put(doc, fixture::stood(pick()));
    let (doc, second) = put(doc, fixture::stood(pick()));
    let source = |at: RecipeNodeId| match doc.node(at) {
        Some(Node::Place { source, .. }) => source.read,
        other => panic!("a placement, not {other:?}"),
    };
    let (one, two) = (source(first), source(second));
    assert_ne!(one, two, "a pick authored twice is two variables");
    for var in [one, two] {
        let pick = doc.pick(var).expect("the source is a pick");
        assert_eq!(pick.of, vec![bolt_var], "the pick's path is the bolt");
    }
    let ev = run(&doc, &EvalOptions::default());
    for at in [first, second] {
        assert!(
            near(bounds(&body_of(&ev, at)), ([-0.2, -0.2, 0.0], [0.2, 0.2, 1.0])),
            "the pick reads the bolt's copy alone"
        );
    }
    let (doc, fused) = put(doc, Node::Union { members: spelled(&[bolt, plate]) });
    let (doc, through) = put(doc, fixture::stood(fused));
    let refused = fixture::insert_refused(
        &doc,
        fixture::stood(BodyRead::plain(Operand::pick(Operand::Node(through), vec![bolt_var]))),
    );
    assert!(
        matches!(refused, EditError::PickPath { .. }),
        "a pick through a union refuses: {refused:?}"
    );
}

// ---- Test 25: each operator decides its operands' meetings ----

/// **`A ⊔ A` is an interference finding**: the census at the `⊔`
/// reads two pieces in one place, and the assembly gate refuses the
/// product at rest.
#[test]
fn one_body_assembled_twice_is_an_interference_finding() {
    let doc = doc("dprime3-twice");
    let (doc, a) = block(doc, [0.0; 2], 0.5, 1.0);
    let (doc, twice) = put(doc, Node::Assemble { members: spelled(&[a, a]) });
    let (doc, _) = produced(doc, twice);
    let ev = run(&doc, &EvalOptions::default());
    let product = editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("it gathers");
    assert!(!product.census.is_empty(), "the census reads the two pieces meeting");
    match editor_core::assemble(&doc, &ev, Tol::witness()) {
        Err(editor_core::AssemblyError::AtRest { findings }) => {
            assert!(!findings.is_empty(), "the census's findings travel");
        }
        other => panic!("A ⊔ A is a finding at rest, got {:?}", other.err()),
    }
}

/// **`A ∪ A` is `A`.**
#[test]
fn one_body_unioned_twice_is_that_body() {
    let doc = doc("dprime3-union-twice");
    let (doc, a) = block(doc, [0.0; 2], 0.5, 1.0);
    let (doc, twice) = put(doc, Node::Union { members: spelled(&[a, a]) });
    let ev = run(&doc, &EvalOptions::default());
    let body = body_of(&ev, twice);
    assert_eq!(solids(&body), 1);
    assert!(near(bounds(&body), ([-0.5, -0.5, 0.0], [0.5, 0.5, 1.0])));
}

/// **`(p ⊔ q) − t` keeps two pieces**: a `⊔`'s result is an ordinary
/// operand, and a tool through both pieces cuts each.
#[test]
fn a_cut_through_an_assemble_keeps_its_pieces() {
    let doc = doc("dprime3-cut");
    let (doc, p) = block(doc, [0.0; 2], 0.5, 1.0);
    let (doc, q) = beside(doc, p, [2.0, 0.0, 0.0], 0.5, 1.0);
    let (doc, both) = put(doc, Node::Assemble { members: spelled(&[p, q]) });
    let (doc, t) = beside(doc, p, [1.0, 0.0, 0.5], 2.0, 1.0);
    let (doc, cut) = put(
        doc,
        Node::Subtract {
            from: both.into(),
            tool: t.into(),
        },
    );
    let ev = run(&doc, &EvalOptions::default());
    let body = body_of(&ev, cut);
    assert_eq!(solids(&body), 2, "both pieces survive the cut");
    assert!(near(bounds(&body), ([-0.5, -0.5, 0.0], [2.5, 0.5, 0.5])), "{:?}", bounds(&body));
}

/// **`h1 ⊔ h2` of a split's halves records the cut, and `h1 ∪ h2`
/// glues it.** The halves rest on the one tool plane, so the census at
/// the `⊔` records the meeting and the coincidence door proves it: no
/// `unproven-coincidence` finding. The union of the halves is the block
/// again.
#[test]
fn a_splits_halves_assembled_record_the_cut_and_unioned_glue_it() {
    let doc = doc("dprime3-halves");
    let (doc, a) = block(doc, [0.0; 2], 0.5, 1.0);
    let tool = fixture::plane_on(&doc, a, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (doc, split) = put(doc, Node::Split { target: a.into(), tool });
    let half = |h: SplitHalf| BodyRead::plain(Operand::Var(doc.output(split, h.port()).unwrap()));
    let halves = Bodies::Spelled(vec![half(SplitHalf::Above), half(SplitHalf::Below)]);
    let (doc, assembled) = put(doc, Node::Assemble { members: halves.clone() });
    let (doc, glued) = put(doc, Node::Union { members: halves });
    let (doc, _) = produced(doc, assembled);
    let ev = run(&doc, &EvalOptions::default());
    let body = body_of(&ev, glued);
    assert_eq!(solids(&body), 1, "the union glues the cut");
    assert!(near(bounds(&body), ([-0.5, -0.5, 0.0], [0.5, 0.5, 1.0])));
    let product = editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("it gathers");
    eprintln!("PROBE halves coinc {} contacts {} census {:.400}", product.coincidences.len(), product.contacts.len(), format!("{:?}", product.census));
    let report = editor_core::run_checks(&doc, &ev, &editor_core::ChecksConfig::default(), Tol::witness())
        .expect("the checks run");
    eprintln!("PROBE halves report {:.1200}", format!("{report:?}"));
    eprintln!("PROBE halves assemble {:.600}", format!("{:?}", editor_core::assemble(&doc, &ev, Tol::witness()).err()));
    assert!(
        report.findings.iter().all(|f| f.check != editor_core::CheckId::UnprovenCoincidence),
        "the cut is proved: {report}"
    );
    panic!("probe");
}

/// **`(bolt ⊔ plate) ∪ bead` keeps the `⊔`'s pieces**: a bead against
/// the bolt alone fuses with it and leaves the plate a piece of its
/// own, the bolt–plate meeting not decided again; a bead bridging bolt
/// and plate makes one solid touching itself where they met, which the
/// union refuses, typed.
#[test]
fn a_seat_unioned_with_a_bead_keeps_its_pieces() {
    let doc = doc("dprime3-bead");
    let (doc, plate) = block(doc, [0.0; 2], 1.0, 0.2);
    let (doc, bolt) = beside(doc, plate, [0.0, 0.0, 0.2], 0.2, 1.0);
    let (doc, seat) = put(doc, Node::Assemble { members: spelled(&[bolt, plate]) });
    let (doc, side) = beside(doc, plate, [0.25, 0.0, 0.6], 0.1, 0.2);
    let (doc, bridge) = beside(doc, plate, [0.25, 0.0, 0.1], 0.1, 0.3);
    let (doc, kept) = put(doc, Node::Union { members: spelled(&[seat, side]) });
    let (doc, fused) = put(doc, Node::Union { members: spelled(&[seat, bridge]) });
    let ev = run(&doc, &EvalOptions::default());
    assert_eq!(solids(&body_of(&ev, kept)), 2, "the bolt and its bead, and the plate");
    assert!(
        matches!(ev.node_error(fused).map(|e| &e.kind), Some(editor_core::NodeErrorKind::Boolean(_))),
        "a bridged seat refuses, typed: {:?}",
        ev.node_error(fused)
    );
}

// ---- Test 26: the product is a slot ----

/// Three disjoint pieces in one space, `p ⊔ bolt ⊔ nut`, the product,
/// and a fourth body placed in the world that the slot does not read.
fn three_pieces(label: &str) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = doc(label);
    let (doc, p) = block(doc, [0.0; 2], 0.5, 0.2);
    let (doc, bolt) = beside(doc, p, [2.0, 0.0, 0.0], 0.2, 1.0);
    let (doc, nut) = beside(doc, p, [4.0, 0.0, 0.0], 0.3, 0.3);
    let (doc, three) = put(doc, Node::Assemble { members: spelled(&[p, bolt, nut]) });
    let (doc, product) = produced(doc, three);
    let (doc, spare) = block(doc, [9.0, 0.0], 0.5, 0.5);
    let (doc, unread) = put(doc, fixture::stood(spare));
    let _ = product;
    (doc, three, unread)
}

/// **The product is what the slot reads**: `p ⊔ bolt ⊔ nut` gathers
/// those three pieces and nothing else, however many other bodies are
/// placed in the world; a slot reading nothing is the empty product.
///
/// *Breaks if* the product is derived from which bodies go unread.
#[test]
fn the_product_is_what_the_slot_reads() {
    let (doc, _, unread) = three_pieces("dprime3-slot");
    let ev = run(&doc, &EvalOptions::default());
    let product = editor_core::product(&doc, &ev, Tol::witness()).expect("it gathers");
    assert_eq!(solids(&product), 3, "the three pieces the slot reads");
    assert!(
        bounds(&product).1[0] < 9.0,
        "the body placed at x = 9 is not read, so not in the product"
    );
    assert!(ev.value(unread).is_some(), "the unread placement evaluates all the same");
    let empty = set_product(&doc, None);
    let ev = run(&empty, &EvalOptions::default());
    assert!(matches!(
        editor_core::product(&empty, &ev, Tol::witness()),
        Err(editor_core::ProductError::EmptyProduct)
    ));
}

/// **An instance of a document defines one body**: the copy of its
/// product, all three pieces, and a pick of one piece reads it.
#[test]
fn an_instance_defines_one_body_of_its_parts_product() {
    let (part, three, _) = three_pieces("dprime3-instance-part");
    let bolt_var = match part.node(three) {
        Some(Node::Assemble { members: Bodies::Spelled(reads) }) => reads[1].read,
        other => panic!("the `⊔`, not {other:?}"),
    };
    let mut store = fixture::resolver::PartStore::default();
    let part_ref = store.insert(part, Tol::witness());
    let doc = doc("dprime3-instance");
    let (doc, instance) = put(
        doc,
        Node::instantiate_part(part_ref, Operand::Var(VarId::WORLD), editor_core::stand().bundle()),
    );
    assert_eq!(doc.outputs(instance).len(), 1, "one output, one body");
    let (doc, picked) = put(
        doc,
        fixture::stood(BodyRead::plain(Operand::pick(Operand::Node(instance), vec![bolt_var]))),
    );
    let opts = fixture::resolver::with_resolver(store);
    let ev = run(&doc, &opts);
    assert_eq!(solids(&body_of(&ev, instance)), 3, "the part's three pieces");
    assert!(
        near(bounds(&body_of(&ev, picked)), ([1.8, -0.2, 0.0], [2.2, 0.2, 1.0])),
        "the pick reads the bolt: {:?}",
        bounds(&body_of(&ev, picked))
    );
}
