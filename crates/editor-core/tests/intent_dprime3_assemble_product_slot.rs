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
    Bodies, BodyRead, DocEdit, DocumentId, EditError, EvalOptions, Evaluation, Node, Operand,
    ProfileDoc, RecipeNodeId, RoleSeg, SplitHalf, ValuePayload, VarId, apply,
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
        Some(ValuePayload::Boolean(editor_core::BooleanValue::Body { body, .. })) => {
            (**body).clone()
        }
        other => panic!(
            "{node:?} reads one body, not {other:?}: {:?}",
            ev.result(node)
        ),
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
    a.0.iter()
        .chain(&a.1)
        .zip(b.0.iter().chain(&b.1))
        .all(|(x, y)| (x - y).abs() < 1e-12)
}

fn doc(label: &str) -> ProfileDoc {
    ProfileDoc::empty(DocumentId::derive(label), Tol::witness())
}

/// A copy of `node`'s value standing in the world.
fn in_world(
    doc: ProfileDoc,
    node: impl Into<BodyRead<editor_core::Formula>>,
) -> (ProfileDoc, RecipeNodeId) {
    put(doc, fixture::stood(node))
}

/// The `⊔` of `members`, each read in the world, made the product: the
/// slot reads the `⊔` itself, so the census at its door is the
/// product's. (A placement of a `⊔` carries no decision of its door
/// until `a-placement-carries-its-sources-at-rest-result`.)
fn produced(doc: ProfileDoc, members: &[RecipeNodeId]) -> (ProfileDoc, RecipeNodeId) {
    let (doc, assembled) = put(
        doc,
        Node::Assemble {
            members: spelled(members),
        },
    );
    (set_product(&doc, Some(Operand::Node(assembled))), assembled)
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
    let (doc, union) = put(
        doc,
        Node::Union {
            members: spelled(&[near, far]),
        },
    );
    let ev = run(&doc, &EvalOptions::default());
    let reads = [doc.output(near, 0).unwrap(), doc.output(far, 0).unwrap()];
    let table = ev
        .table(doc.output(union, 0).unwrap())
        .expect("the union's table");
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
    let (doc, seat) = put(
        doc,
        Node::Assemble {
            members: spelled(&[bolt, plate]),
        },
    );
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
            near(
                bounds(&body_of(&ev, at)),
                ([-0.2, -0.2, 0.0], [0.2, 0.2, 1.0])
            ),
            "the pick reads the bolt's copy alone"
        );
    }
    let (doc, fused) = put(
        doc,
        Node::Union {
            members: spelled(&[bolt, plate]),
        },
    );
    let (doc, through) = put(doc, fixture::stood(fused));
    let refused = fixture::insert_refused(
        &doc,
        fixture::stood(BodyRead::plain(Operand::pick(
            Operand::Node(through),
            vec![bolt_var],
        ))),
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
    let (doc, copy) = in_world(doc, a);
    let (doc, _) = produced(doc, &[copy, copy]);
    let ev = run(&doc, &EvalOptions::default());
    let product = editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("it gathers");
    assert!(
        !product.census.is_empty(),
        "the census reads the two pieces meeting"
    );
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
    let (doc, twice) = put(
        doc,
        Node::Union {
            members: spelled(&[a, a]),
        },
    );
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
    let (doc, both) = put(
        doc,
        Node::Assemble {
            members: spelled(&[p, q]),
        },
    );
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
    assert!(
        near(bounds(&body), ([-0.5, -0.5, 0.0], [2.5, 0.5, 0.5])),
        "{:?}",
        bounds(&body)
    );
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
    // Split in the world, so the halves are the product's to read.
    let (doc, a) = in_world(doc, a);
    let tool = fixture::plane_on(&doc, a, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (doc, split) = put(
        doc,
        Node::Split {
            target: a.into(),
            tool,
        },
    );
    let halves = Bodies::Spelled(
        [SplitHalf::Above, SplitHalf::Below]
            .map(|h| BodyRead::plain(Operand::Var(doc.output(split, h.port()).unwrap())))
            .to_vec(),
    );
    let (doc, glued) = put(
        doc,
        Node::Union {
            members: halves.clone(),
        },
    );
    let (doc, assembled) = put(doc, Node::Assemble { members: halves });
    let doc = set_product(&doc, Some(Operand::Node(assembled)));
    let ev = run(&doc, &EvalOptions::default());
    let body = body_of(&ev, glued);
    assert_eq!(solids(&body), 1, "the union glues the cut");
    assert!(near(bounds(&body), ([-0.5, -0.5, 0.0], [0.5, 0.5, 1.0])));
    let product = editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("it gathers");
    assert!(
        product.census.is_empty(),
        "the census leaves nothing: {:?}",
        product.census
    );
    assert!(
        !product.coincidences.is_empty() && !product.contacts.is_empty(),
        "the meeting is recorded: rows and the record citing them"
    );
    editor_core::assemble(&doc, &ev, Tol::witness()).expect("the halves assemble at rest");
    let report = editor_core::run_checks(
        &doc,
        &ev,
        &editor_core::ChecksConfig::default(),
        Tol::witness(),
    )
    .expect("the checks run");
    assert!(report.findings.is_empty(), "the cut is proved: {report}");
}

/// **`(bolt ⊔ plate) ∪ bead` keeps the `⊔`'s pieces**: a bead against
/// the bolt alone fuses with it and leaves the plate a piece of its
/// own, the bolt–plate meeting not decided again. A bead bridging bolt
/// and plate makes one solid touching itself where they met, which the
/// union refuses, typed by the kernel. The `⊔`'s record cited through
/// the union, and a refusal naming the bridged pair, are
/// `work/intent/a-boolean-over-an-assemble-cites-none-of-its-meetings.md`.
#[test]
fn a_seat_unioned_with_a_bead_keeps_its_pieces() {
    let doc = doc("dprime3-bead");
    let (doc, plate) = block(doc, [0.0; 2], 1.0, 0.2);
    let (doc, bolt) = beside(doc, plate, [0.0, 0.0, 0.2], 0.2, 1.0);
    let (doc, seat) = put(
        doc,
        Node::Assemble {
            members: spelled(&[bolt, plate]),
        },
    );
    let (doc, side) = beside(doc, plate, [0.25, 0.0, 0.6], 0.1, 0.2);
    let (doc, bridge) = beside(doc, plate, [0.25, 0.0, 0.1], 0.1, 0.3);
    let (doc, kept) = put(
        doc,
        Node::Union {
            members: spelled(&[seat, side]),
        },
    );
    let (doc, fused) = put(
        doc,
        Node::Union {
            members: spelled(&[seat, bridge]),
        },
    );
    let ev = run(&doc, &EvalOptions::default());
    assert_eq!(
        solids(&body_of(&ev, kept)),
        2,
        "the bolt and its bead, and the plate"
    );
    assert!(
        matches!(
            ev.node_error(fused).map(|e| &e.kind),
            Some(editor_core::NodeErrorKind::Boolean(_))
        ),
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
    let (doc, three) = put(
        doc,
        Node::Assemble {
            members: spelled(&[p, bolt, nut]),
        },
    );
    let (doc, copy) = in_world(doc, three);
    let doc = set_product(&doc, Some(Operand::Node(copy)));
    let (doc, spare) = block(doc, [9.0, 0.0], 0.5, 0.5);
    let (doc, unread) = in_world(doc, spare);
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
    assert!(
        ev.value(unread).is_some(),
        "the unread placement evaluates all the same"
    );
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
        Some(Node::Assemble {
            members: Bodies::Spelled(reads),
        }) => reads[1].read,
        other => panic!("the `⊔`, not {other:?}"),
    };
    let mut store = fixture::resolver::PartStore::default();
    let part_ref = store.insert(part, Tol::witness());
    let doc = doc("dprime3-instance");
    let (doc, instance) = put(
        doc,
        Node::instantiate_part(
            part_ref,
            Operand::Var(VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
    assert_eq!(doc.outputs(instance).len(), 1, "one output, one body");
    let (doc, picked) = put(
        doc,
        fixture::stood(BodyRead::plain(Operand::pick(
            Operand::Node(instance),
            vec![bolt_var],
        ))),
    );
    let opts = fixture::resolver::with_resolver(store);
    let ev = run(&doc, &opts);
    assert_eq!(
        solids(&body_of(&ev, instance)),
        3,
        "the part's three pieces"
    );
    assert!(
        near(
            bounds(&body_of(&ev, picked)),
            ([1.8, -0.2, 0.0], [2.2, 0.2, 1.0])
        ),
        "the pick reads the bolt: {:?}",
        bounds(&body_of(&ev, picked))
    );
}

/// **An instance's census decisions are its part's, reported once**:
/// a bolt resting on a plate, `p ⊔ bolt`, is the part's product, and
/// the census at that `⊔` records the touch, which the part's
/// `unproven-coincidence` check reports. An instance of the part
/// carries that decision: the host's census decides the touch inside
/// the instance's copy no second time, and the host reports nothing of
/// it.
///
/// *Breaks if* an instance's census rows are decided again in the host.
#[test]
fn an_instances_census_decisions_are_its_parts() {
    let doc = doc("dprime3-carried-part");
    let (doc, p) = block(doc, [0.0; 2], 0.5, 0.2);
    let (doc, bolt) = beside(doc, p, [0.0, 0.0, 0.2], 0.2, 1.0);
    let (doc, p) = in_world(doc, p);
    let (doc, bolt) = in_world(doc, bolt);
    let (part, _) = produced(doc, &[p, bolt]);
    let unproven = |doc: &ProfileDoc, ev: &Evaluation<f64>| {
        editor_core::run_checks(
            doc,
            ev,
            &editor_core::ChecksConfig::default(),
            Tol::witness(),
        )
        .expect("the checks run")
        .findings
        .iter()
        .filter(|f| f.check == editor_core::CheckId::UnprovenCoincidence)
        .count()
    };
    let ev = run(&part, &EvalOptions::default());
    let decided = editor_core::product_recorded(&part, &ev, Tol::witness()).expect("it gathers");
    assert!(
        !decided.coincidences.is_empty(),
        "the part's `⊔` records the touch"
    );
    assert!(unproven(&part, &ev) > 0, "and the part reports it");

    let mut store = fixture::resolver::PartStore::default();
    let part_ref = store.insert(part, Tol::witness());
    let doc = self::doc("dprime3-carried-host");
    let (doc, instance) = put(
        doc,
        Node::instantiate_part(
            part_ref,
            Operand::Var(VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
    let doc = set_product(&doc, Some(Operand::Node(instance)));
    let ev = run(&doc, &fixture::resolver::with_resolver(store));
    let host = editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("it gathers");
    assert!(
        host.coincidences.is_empty() && host.census.is_empty(),
        "the host decides nothing inside the instance's copy"
    );
    assert_eq!(
        unproven(&doc, &ev),
        0,
        "and reports nothing of the part's touch"
    );
    editor_core::assemble(&doc, &ev, Tol::witness()).expect("the host assembles");
}

// ---- Every door holds the slot and the picks ----

/// **The slot reads a body in the world after every edit**: a re-target
/// of the copy the slot reads, out of the world, refuses as `SetProduct`
/// would, and the document keeps its product.
#[test]
fn a_retarget_of_the_products_copy_out_of_the_world_refuses() {
    let d = doc("dprime3-retarget");
    let (d, a) = block(d, [0.0; 2], 0.5, 1.0);
    let (d, copy) = in_world(d, a);
    let d = set_product(&d, Some(Operand::Node(copy)));
    let (d, space) = put(d, Node::Space);
    let space = d.output(space, 0).unwrap();
    let moved = apply(
        &d,
        &DocEdit::SetParam {
            node: copy,
            slot: editor_core::SlotId::Operand(editor_core::OperandSlot::Space),
            value: Operand::Var(space).into(),
            fresh: Vec::new(),
        },
        Tol::witness(),
    );
    assert!(
        matches!(
            moved,
            Err(EditError::ProductRead {
                fault: editor_core::ProductReadFault::Space { .. },
                ..
            })
        ),
        "the slot's copy moved out of the world: {moved:?}"
    );
}

/// **The load door holds the slot as the edit door does**: a stored
/// document whose slot reads a body outside the world, or a variable it
/// never minted, refuses typed.
#[test]
fn the_load_door_refuses_a_slot_reading_no_body_in_the_world() {
    let d = doc("dprime3-load-slot");
    let (d, p) = block(d, [0.0; 2], 0.5, 0.5);
    let (d, copy) = in_world(d, p);
    let d = set_product(&d, Some(Operand::Node(copy)));
    let text = editor_core::persist::save(&d, &[], Tol::witness()).unwrap();
    let ser = |v: VarId| serde_json::to_string(&v).unwrap();
    let product = ser(d.product().unwrap());
    let needles = [
        format!("\"product\":{product}"),
        format!("\"product\": {product}"),
    ];
    assert!(
        needles.iter().any(|n| text.contains(n.as_str())),
        "the slot is on the wire"
    );
    let inner = ser(d.output(p, 0).unwrap());
    for (what, to) in [
        ("a construction's body", inner),
        ("no variable", ser(VarId::new(999, 5))),
    ] {
        let swapped = needles.iter().fold(text.clone(), |t, n| {
            t.replace(n.as_str(), &n.replace(&product, &to))
        });
        let loaded = editor_core::persist::load(&swapped, Tol::witness());
        assert!(
            matches!(
                loaded,
                Err(editor_core::PersistError::Snapshot(
                    editor_core::SnapshotError::ProductRead { .. }
                ))
            ),
            "a slot reading {what} loads: {:?}",
            loaded.map(|l| l.doc.product())
        );
    }
}

/// **A pick of a pick walks the piece the inner pick kept**: through a
/// copy of the bolt's pick, the bolt is a piece and the plate is not.
#[test]
fn a_pick_through_a_pick_walks_the_piece_it_kept() {
    let d = doc("dprime3-pick-pick");
    let (d, bolt) = block(d, [0.0; 2], 0.2, 1.0);
    let (d, plate) = beside(d, bolt, [2.0, 0.0, 0.0], 0.5, 0.2);
    let (d, seat) = put(
        d,
        Node::Assemble {
            members: spelled(&[bolt, plate]),
        },
    );
    let (d, place) = in_world(d, seat);
    let [bolt_var, plate_var] = [bolt, plate].map(|n| d.output(n, 0).unwrap());
    let pick = |at: RecipeNodeId, of: Vec<VarId>| {
        fixture::stood(BodyRead::plain(Operand::pick(Operand::Node(at), of)))
    };
    let (d, kept) = put(d, pick(place, vec![bolt_var]));
    let (d, again) = in_world(d, kept);
    let insert = |node| {
        apply(
            &d,
            &DocEdit::InsertNode {
                node: Box::new(node),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
    };
    let plate_through_bolt = insert(pick(again, vec![plate_var]));
    assert!(
        matches!(
            plate_through_bolt,
            Err(EditError::PickPath { at: Some(0), .. })
        ),
        "the plate is no piece of the bolt's copy: {:?}",
        plate_through_bolt.map(|a| a.record.minted)
    );
    // The bolt's own copy holds no `⊔`, so no step names a piece of it.
    let bolt_through_bolt = insert(pick(again, vec![bolt_var]));
    assert!(
        matches!(
            bolt_through_bolt,
            Err(EditError::PickPath { at: Some(0), .. })
        ),
        "the bolt's copy has no pieces: {:?}",
        bolt_through_bolt.map(|a| a.record.minted)
    );
}

/// **A pick of a `⊔` reading one body twice, by that body, is
/// ambiguous and refuses**: two pieces answer to the read.
#[test]
fn a_pick_by_a_read_the_assemble_reads_twice_refuses() {
    let d = doc("dprime3-pick-twice");
    let (d, a) = block(d, [0.0; 2], 0.5, 1.0);
    let (d, twice) = put(
        d,
        Node::Assemble {
            members: spelled(&[a, a]),
        },
    );
    let (d, place) = in_world(d, twice);
    let a_var = d.output(a, 0).unwrap();
    let picked = apply(
        &d,
        &DocEdit::InsertNode {
            node: Box::new(fixture::stood(BodyRead::plain(Operand::pick(
                Operand::Node(place),
                vec![a_var],
            )))),
            fresh: Vec::new(),
        },
        Tol::witness(),
    );
    assert!(
        matches!(picked, Err(EditError::PickPath { at: Some(0), .. })),
        "two pieces answer to the read: {:?}",
        picked.map(|a| a.record.minted)
    );
}

/// **A pick into an instance's part is the evaluation's to answer**: the
/// edit door reads nothing across the seam (`DocEdit::UpdateReference`),
/// so a path naming no piece of the part is admitted and refuses at
/// evaluation, typed.
#[test]
fn a_pick_into_a_part_naming_no_piece_refuses_at_evaluation() {
    let part = doc("dprime3-pick-part");
    let (part, bolt) = block(part, [0.0; 2], 0.2, 1.0);
    let (part, bolt) = in_world(part, bolt);
    let part = set_product(&part, Some(Operand::Node(bolt)));
    let mut store = fixture::resolver::PartStore::default();
    let part_ref = store.insert(part, Tol::witness());
    let host = doc("dprime3-pick-part-host");
    let (host, inst) = put(
        host,
        Node::instantiate_part(
            part_ref,
            Operand::Var(VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
    let (host, read) = put(
        host,
        fixture::stood(BodyRead::plain(Operand::pick(
            Operand::Node(inst),
            vec![VarId::WORLD],
        ))),
    );
    let ev = run(&host, &fixture::resolver::with_resolver(store));
    assert!(
        matches!(
            ev.node_error(read).map(|e| &e.kind),
            Some(editor_core::NodeErrorKind::NoSuchPiece { .. })
        ),
        "a pick naming no piece of the part: {:?}",
        ev.node_error(read)
    );
}
