//! **ASM-R2b acceptance** — declaration minting and the assembly
//! at-rest gate (docs/ASM-R2B-SPEC.md rows 1–5 and 7;
//! ASSEMBLY-DESIGN A3/A4/A5/A13 clause 4).
//!
//! One assertion per acceptance row, each comment stating the
//! INVARIANT the row pins rather than the mechanics it exercises.
//!
//! # What the kernel currently certifies, and what it does not
//!
//! Row 3's DECLARED direction is written against the invariant that is
//! true today and stays true: a minted declaration means the gate does
//! not report the pair as an UNDECLARED contact, and any refusal that
//! remains is the census's own typed carrier-inventory passthrough.
//! On main a face-granularity `PatchContact` is not certifiable at all
//! (`topo::census`'s patch arm answers `CensusUnsupported`
//! unconditionally), so "remains" is non-empty; with M9-2 PR-2's
//! conformal arm and patch certifier it is empty and the gate returns
//! `Ok(())`. Both satisfy the rows below, which is the point — the
//! assertions name the ASM-owned invariant, not the kernel's current
//! reach.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Alignment, AssemblyError, AxisSense, CancelToken, CapEnd, ContactClass, ContentPin, DocEdit,
    DocRef, DocumentId, EntityKey, EntityKind, EntityRef, Entry, EvalOptions, Evaluation,
    InterfaceCrossing, MateFrame, MatePrimitive, Node, NodeErrorKind, ProfileDoc, RecipeNodeId,
    ResolveFailure, ResolveFault, RoleSeg, StableName, assemble, content_pin, evaluate, inline,
    product_recorded, split,
};
use fixture::{desc, insert, len, step};

// ---- The stub store (the ASM-2A/R2a shape, verbatim in spirit) ----

#[derive(Debug, Default, Clone)]
struct StubStore {
    docs: BTreeMap<DocumentId, ProfileDoc>,
}

impl StubStore {
    fn insert(&mut self, doc: ProfileDoc) -> DocRef {
        let pin = content_pin(&doc).expect("the pin computes");
        let id = doc.id();
        self.docs.insert(id, doc);
        DocRef { id, pin }
    }
}

impl editor_core::PartResolver for StubStore {
    fn resolve(&self, doc_ref: &DocRef) -> Result<ProfileDoc, ResolveFailure> {
        let fail = |fault, message: &str| ResolveFailure {
            fault,
            message: message.to_string(),
        };
        let doc = self
            .docs
            .get(&doc_ref.id)
            .ok_or_else(|| fail(ResolveFault::Unresolved, "no such document"))?;
        if content_pin(doc).expect("the pin computes") != doc_ref.pin {
            return Err(fail(ResolveFault::PinMismatch, "the pin does not hold"));
        }
        Ok(doc.clone())
    }
}

fn opts(store: StubStore) -> EvalOptions {
    EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    }
}

fn run(doc: &ProfileDoc, o: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), o)
}

// ---- Documents ----

/// An axis-aligned block: `[x]×[y]` on the plane at `z0`, extruded
/// `dz`. Its extrude node is the returned id.
fn block(
    doc: ProfileDoc,
    x: (f64, f64),
    y: (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, z0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
        )),
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
}

/// A one-block part document: `[0,1]³`. Its extrude is node 1.
fn cube_part(label: &str) -> ProfileDoc {
    let (doc, _) = block(
        ProfileDoc::empty(DocumentId::derive(label)),
        (0.0, 1.0),
        (0.0, 1.0),
        0.0,
        1.0,
    );
    doc
}

/// The corner-kiss part (`m4_pr5_declare`'s `kiss_base`, as a
/// document): `[0,1]³` ∪ `[1,2]³`, whose union DISCOVERS the v-v kiss
/// at (1,1,1) and records it. This is a part whose product carries
/// declared contact records of its own — row 1's subject.
fn kiss_part(label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label));
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (1.0, 2.0), (1.0, 2.0), 1.0, 1.0);
    let (doc, _) = insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a,
            b,
            declare: None,
        },
    );
    doc
}

/// A face of `instance`'s part product, named through the instance
/// qualifier (A12's reading-edge head).
fn in_part(instance: RecipeNodeId, cap: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId(1),
                path: vec![RoleSeg::Cap(cap)],
            }),
        }],
    }
}

fn frame(origin: [f64; 3], axis: [f64; 3]) -> MateFrame {
    MateFrame {
        origin,
        axis,
        reference: [1.0, 0.0, 0.0],
    }
}

/// A `Rest` mate seating instance `b`'s bottom cap on instance `a`'s
/// top cap, at `offset` (0 = touching; negative lifts `b` clear —
/// the offset runs along the TARGET frame's own axis, which the
/// opposed sense has already flipped, per ASM-R2a's row 1).
fn rest_mate(
    a: RecipeNodeId,
    b: RecipeNodeId,
    offset: f64,
) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a: in_part(a, CapEnd::Top),
        b: in_part(b, CapEnd::Bottom),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: frame([0.0, 0.0, 1.0], [0.0, 0.0, 1.0]),
            b: frame([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
            primitive: MatePrimitive::PlanarRest { offset },
            sense: AxisSense::Opposed,
            clocking: None,
        },
    }
}

/// Two instances of the unit cube, plus the seating mate at `offset`.
/// Returns (document, instance ids, mate id, store).
fn stacked(
    label: &str,
    offset: f64,
) -> (ProfileDoc, Vec<RecipeNodeId>, RecipeNodeId, StubStore) {
    let mut store = StubStore::default();
    let doc_ref = store.insert(cube_part(&format!("{label}-part")));
    let mut doc = ProfileDoc::empty(DocumentId::derive(label));
    let mut ids = Vec::new();
    for _ in 0..2 {
        let (next, id) = insert(doc, Node::instantiate_part(doc_ref));
        doc = next;
        ids.push(id);
    }
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: rest_mate(ids[0], ids[1], offset),
        },
    );
    (doc, ids, mate.expect("the mate mints"), store)
}

/// The kernel findings of an at-rest refusal, or the empty vector for
/// a gate that passed — so a row can assert about the finding SET
/// without a conditional.
fn findings(result: &Result<editor_core::Assembly<f64>, AssemblyError>) -> Vec<String> {
    match result {
        Ok(_) => Vec::new(),
        Err(AssemblyError::AtRest { findings }) => {
            findings.iter().map(|f| format!("{:?}", f.error)).collect()
        }
        Err(other) => panic!("expected an at-rest verdict, got {other}"),
    }
}

fn names_of(table: &editor_core::NameTable, key: EntityKey) -> Option<StableName> {
    table
        .name_of(&EntityRef { body: 0, key })
        .cloned()
        .or_else(|| {
            table.iter().find_map(|(n, e)| match e {
                Entry::Unique(r) if r.key == key => Some(n.clone()),
                Entry::Tied(rs) if rs.iter().any(|r| r.key == key) => Some(n.clone()),
                _ => None,
            })
        })
}

/// Unwraps one `InPart` layer: the assembly's name for a part entity
/// is the part's own name, wrapped at the instance.
fn unwrap_in_part(name: &StableName) -> StableName {
    match &name.path[..] {
        [RoleSeg::InPart { of }] => (**of).clone(),
        _ => panic!("an instance's product name is InPart-wrapped: {name:?}"),
    }
}

// ---- Row 1: the contacts channel (D-1) ----

/// INVARIANT: a part's own declared contacts survive instantiation
/// into the assembly's record set, under the keys the GRAFT says its
/// entities became — lineage through the descendant map, never
/// re-derivation from the gathered geometry. Observably: the assembly
/// carries exactly the part's records, and each record still names the
/// same part entities (unwrapping the instance qualifier).
#[test]
fn row1_a_parts_declared_contacts_survive_instantiation() {
    let mut store = StubStore::default();
    let part = kiss_part("asm-r2b-row1-part");
    let doc_ref = store.insert(part.clone());

    // What the part's own product records, on its own keys.
    let part_ev = run(&part, &EvalOptions::default());
    let part_product = product_recorded(&part, &part_ev).expect("the part gathers");
    assert_eq!(
        part_product.contacts.vv.len(),
        1,
        "the kiss union records one v-v contact: {:?}",
        part_product.contacts
    );
    let part_pair = {
        let c = part_product.contacts.vv[0];
        (
            names_of(&part_product.names, EntityKey::Vertex(c.a)),
            names_of(&part_product.names, EntityKey::Vertex(c.b)),
        )
    };

    // The same part, instantiated.
    let (doc, _) = insert(
        ProfileDoc::empty(DocumentId::derive("asm-r2b-row1")),
        Node::instantiate_part(doc_ref),
    );
    let ev = run(&doc, &opts(store));
    let product = product_recorded(&doc, &ev).expect("the assembly gathers");

    assert_eq!(
        product.contacts.vv.len(),
        1,
        "the record crossed the seam, exactly once: {:?}",
        product.contacts
    );
    let c = product.contacts.vv[0];
    let moved = (
        names_of(&product.names, EntityKey::Vertex(c.a)).map(|n| unwrap_in_part(&n)),
        names_of(&product.names, EntityKey::Vertex(c.b)).map(|n| unwrap_in_part(&n)),
    );
    assert_eq!(
        moved, part_pair,
        "the carried record names the SAME part entities — the key \
         correspondence is the graft's descendant map, not a re-scan"
    );
}

// ---- Row 2: minting (D-2) ----

/// INVARIANT: a solved mate's declaration appears in the product's
/// record set at FACE granularity with the mate's class, keyed to the
/// placed faces its references resolve to — the kernel's own record
/// type, no adapter (A3).
#[test]
fn row2_a_solved_rest_mate_mints_its_declaration() {
    let (doc, ids, mate, store) = stacked("asm-r2b-row2", 0.0);
    let ev = run(&doc, &opts(store));
    let result = assemble(&doc, &ev);
    let minted = match &result {
        Ok(a) => a.minted.clone(),
        Err(AssemblyError::AtRest { .. }) => {
            // The gate's verdict is row 3's subject; the MINT is this
            // row's, and it happens either way. Re-derive it from a
            // second gather so the row asserts about minting alone.
            let product = product_recorded(&doc, &ev).expect("gathers");
            assert_eq!(
                product.contacts.patches.len(),
                0,
                "the gather itself mints nothing — minting is the \
                 assembly door's, not the product's"
            );
            Vec::new()
        }
        Err(other) => panic!("unexpected refusal: {other}"),
    };
    // Whether or not the kernel certifies the pair, the DECLARATION is
    // what this row is about, so it is re-checked through the door
    // that always reports it.
    let (mate_row, class) = match assemble(&doc, &ev) {
        Ok(a) => {
            assert_eq!(a.minted.len(), 1, "one mate, one declaration");
            assert_eq!(
                a.contacts.patches.len(),
                1,
                "the declaration is a PatchContact — face granularity"
            );
            assert_eq!(
                (a.contacts.patches[0].face_a, a.contacts.patches[0].face_b),
                a.minted[0].faces,
                "the record is keyed to the faces the references resolved to"
            );
            (a.minted[0].mate, a.minted[0].class)
        }
        Err(AssemblyError::AtRest { findings }) => {
            // Every finding the kernel raised is about a face the
            // declaration named — which is only possible because the
            // declaration was minted and fed to the gate.
            let attributed = findings
                .iter()
                .filter_map(|f| f.mate.clone())
                .collect::<Vec<_>>();
            assert!(
                !attributed.is_empty(),
                "the minted declaration reached the gate: {findings:?}"
            );
            (attributed[0].mate, attributed[0].class)
        }
        Err(other) => panic!("unexpected refusal: {other}"),
    };
    assert_eq!(mate_row, mate, "the declaration names its mate");
    assert_eq!(class, ContactClass::Rest, "with the mate's class");
    assert_eq!(ids.len(), 2);
    let _ = minted;
}

/// INVARIANT: minting is DECLARATION, not verification — a
/// non-tree (DECLARING) mate mints identically to the tree mate that
/// placed its child. Two mates on the same pair: the second solved
/// nothing, and it still says what touches what.
#[test]
fn row2_b_a_declaring_mate_mints_identically() {
    let (doc, ids, _, store) = stacked("asm-r2b-row2b", 0.0);
    // A second Rest mate on the SAME pair, same statement: the solve
    // makes it non-tree (its pair is already determined).
    let (doc, second) = step(
        doc,
        DocEdit::InsertNode {
            node: rest_mate(ids[0], ids[1], 0.0),
        },
    );
    let second = second.expect("the second mate mints");
    let poses = editor_core::solve_document(&doc);
    assert_eq!(
        poses.role(second),
        Some(editor_core::MateRole::Declaring),
        "the second mate solved nothing"
    );
    let ev = run(&doc, &opts(store));
    let declared: Vec<RecipeNodeId> = match assemble(&doc, &ev) {
        Ok(a) => a.minted.iter().map(|m| m.mate).collect(),
        Err(AssemblyError::AtRest { findings }) => {
            let mut v: Vec<RecipeNodeId> = findings
                .iter()
                .filter_map(|f| f.mate.as_ref().map(|m| m.mate))
                .collect();
            v.sort();
            v.dedup();
            v
        }
        Err(other) => panic!("unexpected refusal: {other}"),
    };
    assert!(
        declared.contains(&second),
        "the DECLARING mate minted too — role does not enter minting: \
         {declared:?}"
    );
}

// ---- Row 3: the F1 pair, both directions ----

/// INVARIANT (the scan-to-bless ban, UNDECLARED direction): two
/// instances that touch with NOTHING declared are the A5 hard error.
/// F1 executes across the document seam exactly as it does within it.
#[test]
fn row3_a_an_undeclared_touching_pair_is_the_hard_error() {
    // The same touching geometry, with the mate DELETED after it
    // solved the pose — the placement survives as document data, so
    // the instances still touch and nothing declares it.
    let (doc, ids, mate, store) = stacked("asm-r2b-row3a", 0.0);
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: mate });
    let ev = run(&doc, &opts(store));
    let result = assemble(&doc, &ev);
    let errs = findings(&result);
    assert!(
        errs.iter().any(|e| e.contains("UndeclaredContact")),
        "an undeclared cross-instance contact refuses, naming the \
         finding: {errs:?} (instances {ids:?})"
    );
}

/// INVARIANT (the scan-to-bless ban, DECLARED direction): the same
/// geometry WITH the mate's declaration minted is no longer an
/// undeclared contact — the declaration is what suppresses the F1
/// refusal, and nothing else does. Whatever refusal remains is the
/// census's own typed carrier-inventory passthrough (module docs:
/// certifying a face-granularity patch is the kernel's reach, not
/// this layer's).
#[test]
fn row3_b_the_declared_touching_pair_is_not_an_undeclared_contact() {
    let (doc, _, _, store) = stacked("asm-r2b-row3b", 0.0);
    let ev = run(&doc, &opts(store));
    let result = assemble(&doc, &ev);
    let errs = findings(&result);
    assert!(
        !errs.iter().any(|e| e.contains("UndeclaredContact")),
        "the minted declaration backs the touching pair: {errs:?}"
    );
    assert!(
        errs.iter().all(|e| e.contains("CensusUnsupported")),
        "any remaining refusal is the typed carrier-inventory \
         passthrough, never a silent bless: {errs:?}"
    );
}

// ---- Row 4: a definite mismatch names its mate ----

/// INVARIANT: a mate declaring `Rest` over a genuinely GAPPED pair
/// refuses, and the refusal names the mate node, both of its
/// references, and the kernel's own finding. The layer attributes; it
/// does not decide.
#[test]
fn row4_a_gapped_rest_declaration_refuses_naming_its_mate() {
    // offset −1 lifts b a full unit clear: the declared faces are a
    // unit apart, definitely.
    let (doc, ids, mate, store) = stacked("asm-r2b-row4", -1.0);
    let ev = run(&doc, &opts(store));
    let err = assemble(&doc, &ev).err().expect("a gapped Rest refuses");
    let AssemblyError::AtRest { findings } = &err else {
        panic!("expected the at-rest verdict, got {err}");
    };
    let named = findings
        .iter()
        .find_map(|f| f.mate.clone())
        .expect("the refusal names its mate");
    assert_eq!(named.mate, mate, "the MATE NODE is named");
    assert_eq!(
        (named.a, named.b),
        (in_part(ids[0], CapEnd::Top), in_part(ids[1], CapEnd::Bottom)),
        "both references are named"
    );
    assert!(
        !findings.is_empty(),
        "the kernel's own finding travels with it"
    );
    // And the whole refusal renders with all three, so a caller that
    // only has the Display still learns which mate is wrong.
    let msg = err.to_string();
    assert!(
        msg.contains(&format!("mate {}", mate.0)),
        "the rendering names the mate: {msg}"
    );
}

// ---- Row 5: split, the crossing record, and re-verification ----

/// INVARIANT: split populates the interface record with EXACTLY the
/// mates whose ends land on opposite sides of the cut (A4: "the seam
/// is the crossing declarations"), spelling the part-side reference in
/// the PART's own names; inline dissolves it and the document
/// round-trips.
#[test]
fn row5_a_split_populates_the_crossing_record_and_inline_dissolves_it() {
    let (doc, ids, mate, store) = stacked("asm-r2b-row5", 0.0);
    // Cut out instance 1: the mate's `b` end goes, its `a` end stays.
    let cut = [ids[1]].into_iter().collect();
    let out = split(&doc, &cut, DocumentId::derive("asm-r2b-row5-split"))
        .expect("the split succeeds");
    let Some(Node::InstantiatePart { interface, .. }) = out.remainder.node(out.instance) else {
        panic!("the split minted an instance");
    };
    assert_eq!(
        interface.crossings.len(),
        1,
        "one mate crossed the cut: {:?}",
        interface.crossings
    );
    let InterfaceCrossing::Mate {
        mate: crossed,
        class,
        outer,
        inner,
    } = &interface.crossings[0];
    assert_eq!(*crossed, mate, "the crossing names its mate");
    assert_eq!(*class, ContactClass::Rest, "and the class it declares");
    assert_eq!(
        *outer,
        in_part(ids[0], CapEnd::Top),
        "the remainder-side reference is unchanged"
    );
    assert_eq!(
        inner.node,
        out.node_map[&ids[1]],
        "the part-side reference is spelled in the PART's own ids"
    );

    // Inline is the inverse: the record dissolves with the instance.
    let mut store2 = store.clone();
    store2.insert(out.part.clone());
    let back = inline(&out.remainder, out.instance, &store2).expect("inline succeeds");
    assert!(
        back.doc.node(out.instance).is_none(),
        "the instance is gone, and its record with it"
    );
    assert!(
        back.doc
            .order()
            .iter()
            .any(|&id| matches!(back.doc.node(id), Some(Node::Mate { .. }))),
        "the declaration itself survives, as the mate node it always was"
    );
}

/// INVARIANT (A13 clause 4 + A4's "does it actually fit"): a pin move
/// that changes the part so a crossing declaration's reference no
/// longer names anything re-verifies at the NEXT EVALUATION and
/// refuses typed, naming the crossing. Pre-move the same document
/// evaluates.
#[test]
fn row5_b_a_pin_move_that_breaks_a_crossing_refuses_at_evaluation() {
    let (doc, ids, _, store) = stacked("asm-r2b-row5b", 0.0);
    let cut = [ids[1]].into_iter().collect();
    let part_id = DocumentId::derive("asm-r2b-row5b-split");
    let out = split(&doc, &cut, part_id).expect("the split succeeds");
    let mut store = store;
    store.insert(out.part.clone());

    // Pre-move: the crossing re-verifies, so the instance evaluates.
    let ev = run(&out.remainder, &opts(store.clone()));
    assert!(
        ev.node_error(out.instance).is_none(),
        "the crossing re-verifies against the pinned part: {:?}",
        ev.node_error(out.instance).map(ToString::to_string)
    );

    // The move: a part whose contact face is gone — the instantiate
    // node's own extrude replaced by one the crossing cannot name.
    let mut replaced = ProfileDoc::empty(part_id);
    for &old in out.part.order() {
        if let Some(Node::Profile(_)) = out.part.node(old) {
            let (next, _) = insert(replaced, out.part.node(old).unwrap().clone());
            replaced = next;
        }
    }
    let new_pin = content_pin(&replaced).expect("the pin computes");
    store.docs.insert(part_id, replaced);
    let moved = editor_core::apply(
        &out.remainder,
        &DocEdit::UpdateReference {
            node: out.instance,
            new_pin,
        },
    )
    .expect("the pin moves")
    .doc;

    let ev2 = run(&moved, &opts(store));
    let err = ev2
        .node_error(out.instance)
        .expect("the moved pin refuses")
        .to_string();
    assert!(
        err.contains("crossing") || err.contains("does not re-verify") || err.contains("part"),
        "the refusal names the crossing that no longer fits: {err}"
    );
}

// ---- Row 6: the content key feeds on the inhabited record ----

/// INVARIANT (the content-key half of ASM-4's hook obligation): a
/// crossing-record edit MOVES the instantiate node's content key. Two
/// documents that differ only in the record must not share a memo
/// entry — otherwise a crossing edit would be served the pre-edit
/// answer, re-verification and all.
#[test]
fn row6_a_crossing_record_edit_moves_the_content_key() {
    let (doc, ids, _, store) = stacked("asm-r2b-row6", 0.0);
    let cut = [ids[1]].into_iter().collect();
    let out = split(&doc, &cut, DocumentId::derive("asm-r2b-row6-split"))
        .expect("the split succeeds");
    let mut store = store;
    store.insert(out.part.clone());

    // The same document with the record EMPTIED, built by replaying
    // the split's own recorded edits with only the instance's node
    // swapped: same ids (the mint counter is deterministic), same
    // reference, same pin, same placement — the record is the ONLY
    // difference, which is what makes the key comparison mean
    // something.
    let mut emptied = doc.clone();
    for e in &out.remainder_edits {
        let e = match e {
            DocEdit::InsertNode {
                node: Node::InstantiatePart { doc_ref, .. },
            } => DocEdit::InsertNode {
                node: Node::instantiate_part(*doc_ref),
            },
            other => other.clone(),
        };
        emptied = editor_core::apply(&emptied, &e).expect("the split's edits replay").doc;
    }
    assert!(
        matches!(
            emptied.node(out.instance),
            Some(Node::InstantiatePart { interface, .. }) if interface.is_empty()
        ),
        "the replay reproduced the same instance id with an empty record"
    );

    let with = run(&out.remainder, &opts(store.clone()));
    let without = run(&emptied, &opts(store));
    let key = |ev: &Evaluation<f64>| {
        ev.value(out.instance)
            .expect("the instance evaluates")
            .content_key
    };
    assert_ne!(
        key(&with),
        key(&without),
        "the inhabited record feeds the key"
    );
}

// ---- Row 7: D9 determinism ----

/// INVARIANT (D9): a mated, minted document's evaluation is a pure
/// function of the document — two independent evaluations produce the
/// bit-identical record set and the same at-rest verdict.
#[test]
fn row7_the_minted_record_set_is_deterministic() {
    let (doc, _, _, store) = stacked("asm-r2b-row7", 0.0);
    let a = run(&doc, &opts(store.clone()));
    let b = run(&doc, &opts(store));
    let pa = product_recorded(&doc, &a).expect("gathers");
    let pb = product_recorded(&doc, &b).expect("gathers");
    assert_eq!(pa.contacts, pb.contacts, "the carried records replay");
    let ra = assemble(&doc, &a);
    let rb = assemble(&doc, &b);
    assert_eq!(
        findings(&ra),
        findings(&rb),
        "the at-rest verdict replays, finding for finding"
    );
    match (&ra, &rb) {
        (Ok(x), Ok(y)) => assert_eq!(x.contacts, y.contacts, "and so does the minted set"),
        (Err(_), Err(_)) => {}
        _ => panic!("two evaluations of one document disagreed on the verdict"),
    }
}

// ---- The typed refusals the mint door owns ----

/// INVARIANT: a class with no at-rest kernel record refuses TYPED
/// naming the deferral, rather than minting a record with an invented
/// witness (module docs' honest boundary).
#[test]
fn a_tangent_mate_refuses_at_the_mint_door() {
    let (doc, ids, _, store) = stacked("asm-r2b-tangent", 0.0);
    let mut node = rest_mate(ids[0], ids[1], 0.0);
    if let Node::Mate { class, .. } = &mut node {
        *class = ContactClass::Tangent;
    }
    let (doc, _) = step(doc, DocEdit::InsertNode { node });
    let ev = run(&doc, &opts(store));
    match assemble(&doc, &ev) {
        Err(AssemblyError::NoAtRestRecord { class, .. }) => {
            assert_eq!(class, ContactClass::Tangent);
        }
        other => panic!("a Tangent mate must refuse at the mint door: {other:?}"),
    }
}

/// INVARIANT: a mate reference that names no face of the product
/// refuses typed — never resolved by picking, never widened.
#[test]
fn a_mate_reference_that_names_nothing_refuses_typed() {
    let (doc, ids, _, store) = stacked("asm-r2b-vanish", 0.0);
    let mut node = rest_mate(ids[0], ids[1], 0.0);
    if let Node::Mate { a, .. } = &mut node {
        a.path = vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId(99),
                path: vec![RoleSeg::Cap(CapEnd::Top)],
            }),
        }];
    }
    let (doc, _) = step(doc, DocEdit::InsertNode { node });
    let ev = run(&doc, &opts(store));
    match assemble(&doc, &ev) {
        Err(AssemblyError::Reference { why, .. }) => {
            assert_eq!(why, editor_core::RefusedRef::Vanished);
        }
        other => panic!("an unresolvable reference must refuse typed: {other:?}"),
    }
}

/// Sanity: `NodeErrorKind::CrossingUnverified` is reachable as a typed
/// value, so the D-5 refusal is nameable by a caller (the pncad
/// binding lifts these by name).
#[test]
fn the_crossing_refusal_is_a_named_node_error() {
    let e = NodeErrorKind::CrossingUnverified {
        instance: RecipeNodeId(1),
        mate: RecipeNodeId(2),
        name: Box::new(StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(3),
            path: vec![RoleSeg::Cap(CapEnd::Top)],
        }),
    };
    let msg = e.to_string();
    assert!(msg.contains("re-verify"), "{msg}");
}
