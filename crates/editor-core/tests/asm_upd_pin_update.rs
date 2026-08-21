//! ASM-UPD acceptance — the pin-update door (ASSEMBLY-DESIGN A13).
//!
//! The kernel-layer half of the eight spec rows: everything reachable
//! with a STUB store, which is the substrate ASM-2A D-3 established.
//! The document-layer half — `update_to_store` computing a pin from a
//! real `Workspace`, and D9 across two fresh processes — lives in
//! `pncad`'s own suite, because the store is that layer's.
//!
//! The stub here is keyed by (id, PIN), not by id: it can hand back
//! two versions of one document at once. That is deliberate and is
//! what makes the D-4 memo evidence sharp — when both versions are
//! available, the ONLY thing deciding which content an instance gets
//! is the pin in its reference, so "the memo re-keys" is observable
//! rather than inferred.
//!
//! Undo note, as in `asm2a_instantiate`: this layer has no undo STACK.
//! `apply` is pure and undo is keeping prior values, so "undo
//! restores" is checked the way the layer offers it — the pre-edit
//! document, held across the edit, is unchanged.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    CancelToken, ContentPin, DocEdit, DocRef, DocumentId, EditError, EvalOptions, Evaluation,
    Frame, Node, ProfileDoc, RecipeNodeId, ResolveFailure, ResolveFault, UpdateError, content_pin,
    evaluate, load, mixed_pins, product, save, update_references,
};
use fixture::{desc, insert, len, square, step};
use geom_core::Tol;

// ---- The versioned stub store ----

/// A resolver over an in-memory map keyed by the FULL reference —
/// (id, pin) — so one document id can be resolvable at several
/// versions simultaneously. A real store holds one file per id; this
/// one holds a version shelf, which is what isolates the pin as the
/// deciding input.
#[derive(Debug, Default)]
struct VersionShelf {
    docs: BTreeMap<(DocumentId, ContentPin), ProfileDoc>,
}

impl VersionShelf {
    /// Shelves `doc` under its own true pin and returns the reference.
    fn shelve(&mut self, doc: ProfileDoc, tol: Tol) -> DocRef {
        let pin = content_pin(&doc, tol).expect("the pin computes");
        let id = doc.id();
        self.docs.insert((id, pin), doc);
        DocRef { id, pin }
    }
}

impl editor_core::PartResolver for VersionShelf {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        self.docs
            .get(&(doc_ref.id, doc_ref.pin))
            .cloned()
            .ok_or_else(|| ResolveFailure {
                fault: ResolveFault::Unresolved,
                message: "no such version on the shelf".to_string(),
            })
    }
}

fn with_shelf(shelf: VersionShelf) -> EvalOptions {
    EvalOptions {
        resolver: Some(Arc::new(shelf)),
        ..EvalOptions::default()
    }
}

fn run(doc: &ProfileDoc, opts: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), opts, Tol::witness())
}

/// An evaluation carrying a WARM prior — the incremental editing
/// session, which is the only place a stale-serving bug can hide: a
/// fresh evaluation rebuilds the part cache from nothing, so it cannot
/// serve yesterday's geometry no matter how the node is keyed.
fn run_warm(doc: &ProfileDoc, prior: &Evaluation<f64>, opts: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, Some(prior), &CancelToken::new(), opts, Tol::witness())
}

// ---- Fixtures ----

/// A one-solid part under `id`: a `side`-wide square extruded 1 tall.
/// The id is a parameter, so two DIFFERENT contents can share one
/// identity — which is exactly what "two versions of a part" means.
fn part_version(id: DocumentId, side: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(id, Tol::witness());
    let (doc, profile) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![square(0.0, 0.0, side / 2.0)],
        )),
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    doc
}

/// An assembly instantiating `refs` in order, each a root, the i-th
/// displaced 10·i along +x so the solids stay disjoint.
fn assembly(label: &str, refs: &[DocRef]) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for (i, r) in refs.iter().enumerate() {
        let (next, id) = insert(doc, Node::instantiate_part(*r));
        doc = next;
        if i > 0 {
            #[allow(clippy::cast_precision_loss)]
            let dx = 10.0 * i as f64;
            let (next, _) = step(
                doc,
                DocEdit::SetPlacement {
                    node: id,
                    frame: Frame::translation([dx, 0.0, 0.0]),
                },
            );
            doc = next;
        }
        ids.push(id);
    }
    (doc, ids)
}

/// The reference a node carries — the value every row reads back.
fn pin_at(doc: &ProfileDoc, node: RecipeNodeId) -> DocRef {
    match doc.node(node) {
        Some(Node::InstantiatePart { doc_ref, .. }) => *doc_ref,
        other => panic!("expected an instantiate node, got {other:?}"),
    }
}

fn volume(body: &topo::Body<f64>) -> f64 {
    topo::mass_properties(body, Tol::witness()).expect("mass properties").volume
}

/// The assembly's whole-product volume, gathered through the shelf.
fn product_volume(doc: &ProfileDoc, opts: &EvalOptions) -> f64 {
    let ev = run(doc, opts);
    volume(&product(doc, &ev, Tol::witness()).expect("the product gathers"))
}

// ---- Row 1: the primitive ----

/// Row 1a — an accepted update moves the VERSION half of the
/// reference and nothing else: the id is untouched, the node is the
/// one named, and the edit reports itself STRUCTURAL (the pin decides
/// which content materializes).
#[test]
fn row1a_update_moves_the_pin_and_only_the_pin() {
    let id = DocumentId::derive("asm-upd-r1a-part");
    let v1 = content_pin(&part_version(id, 1.0), Tol::witness()).expect("pin");
    let v2 = content_pin(&part_version(id, 2.0), Tol::witness()).expect("pin");
    assert_ne!(v1, v2, "the two versions really are different content");

    let (doc, ids) = assembly("asm-upd-r1a-asm", &[DocRef { id, pin: v1 }]);
    let applied = doc
        .apply(&DocEdit::UpdateReference {
            node: ids[0],
            new_pin: v2,
        }, Tol::witness())
        .expect("the update is accepted");

    assert_eq!(pin_at(&applied.doc, ids[0]).pin, v2, "the version moved");
    assert_eq!(pin_at(&applied.doc, ids[0]).id, id, "the identity did not");
    assert!(applied.record.structural, "a pin move is recipe shape");
    assert_eq!(applied.record.minted, None, "an update mints no node");
    // Undo is keeping the prior value: the document held across the
    // edit still names the prior version.
    assert_eq!(pin_at(&doc, ids[0]).pin, v1, "the pre-edit value is intact");
}

/// Row 1b — the update replays: a log ending in one `UpdateReference`
/// reproduces the updated document bit-identically from empty.
#[test]
fn row1b_the_update_replays_bit_identically() {
    let id = DocumentId::derive("asm-upd-r1b-part");
    let v1 = content_pin(&part_version(id, 1.0), Tol::witness()).expect("pin");
    let v2 = content_pin(&part_version(id, 2.0), Tol::witness()).expect("pin");

    let mut rec = fixture::Recorder::new();
    let asm_id = rec.doc.id();
    let node = rec.insert(Node::instantiate_part(DocRef { id, pin: v1 }));
    rec.push(DocEdit::UpdateReference { node, new_pin: v2 });

    let replayed = ProfileDoc::replay(asm_id, &rec.edits, Tol::witness()).expect("the log replays");
    assert!(
        replayed.bit_eq(&rec.doc),
        "replay reproduces the updated document bit for bit"
    );
    assert_eq!(pin_at(&replayed, node).pin, v2);
}

/// Row 1c — the three refusals, each constructed, each typed, each
/// naming its own subject. An unknown id and a live-but-wrong-kind
/// node are different mistakes and do not collapse into one message.
#[test]
fn row1c_the_three_refusals_each_name_their_subject() {
    let id = DocumentId::derive("asm-upd-r1c-part");
    let v1 = content_pin(&part_version(id, 1.0), Tol::witness()).expect("pin");
    let v2 = content_pin(&part_version(id, 2.0), Tol::witness()).expect("pin");
    let (doc, ids) = assembly("asm-upd-r1c-asm", &[DocRef { id, pin: v1 }]);
    // A live NON-instance node to aim at.
    let (doc, profile) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![square(0.0, 0.0, 0.5)],
        )),
    );

    // Same pin: a semantically empty edit refuses rather than records.
    match doc.apply(&DocEdit::UpdateReference {
        node: ids[0],
        new_pin: v1,
    }, Tol::witness()) {
        Err(EditError::PinUnchanged { node, pin }) => {
            assert_eq!(node, ids[0]);
            assert_eq!(pin, v1);
            let msg = EditError::PinUnchanged { node, pin }.to_string();
            assert!(msg.contains(&format!("node {}", node.0)), "{msg}");
            assert!(msg.contains(&pin.hex()), "{msg}");
        }
        other => panic!("a same-pin update must refuse PinUnchanged, got {other:?}"),
    }

    // A live node that instantiates nothing.
    match doc.apply(&DocEdit::UpdateReference {
        node: profile,
        new_pin: v2,
    }, Tol::witness()) {
        Err(EditError::UpdateOnNonInstance { node }) => {
            assert_eq!(node, profile);
            let msg = EditError::UpdateOnNonInstance { node }.to_string();
            assert!(msg.contains(&format!("node {}", node.0)), "{msg}");
        }
        other => panic!("a non-instance target must refuse UpdateOnNonInstance, got {other:?}"),
    }

    // An id no node ever had.
    let ghost = RecipeNodeId(9_999);
    match doc.apply(&DocEdit::UpdateReference {
        node: ghost,
        new_pin: v2,
    }, Tol::witness()) {
        Err(EditError::UnknownNode { id }) => {
            assert_eq!(id, ghost);
            let msg = EditError::UnknownNode { id }.to_string();
            assert!(msg.contains(&format!("node {}", id.0)), "{msg}");
        }
        other => panic!("an unknown node must refuse UnknownNode, got {other:?}"),
    }
}

// ---- Row 2: the elaboration ----

/// Row 2 — "update id X everywhere" on a two-instance-plus-one-other
/// document returns ONE edit per matching site, in document order, and
/// touches no reference to any other id. The list is returned, not
/// applied: purity is what makes applying it atomic.
#[test]
fn row2_elaboration_updates_every_matching_site_and_only_those() {
    let x = DocumentId::derive("asm-upd-r2-x");
    let y = DocumentId::derive("asm-upd-r2-y");
    let x1 = content_pin(&part_version(x, 1.0), Tol::witness()).expect("pin");
    let x2 = content_pin(&part_version(x, 2.0), Tol::witness()).expect("pin");
    let y1 = content_pin(&part_version(y, 3.0), Tol::witness()).expect("pin");

    let (doc, ids) = assembly(
        "asm-upd-r2-asm",
        &[
            DocRef { id: x, pin: x1 },
            DocRef { id: y, pin: y1 },
            DocRef { id: x, pin: x1 },
        ],
    );

    let edits = update_references(&doc, x, x2).expect("both x sites elaborate");
    assert_eq!(
        edits,
        vec![
            DocEdit::UpdateReference {
                node: ids[0],
                new_pin: x2
            },
            DocEdit::UpdateReference {
                node: ids[2],
                new_pin: x2
            },
        ],
        "one edit per x site, in document order, none for y"
    );
    assert_eq!(pin_at(&doc, ids[0]).pin, x1, "the elaboration applied none");

    let mut moved = doc.clone();
    for e in &edits {
        moved = moved.apply(e, Tol::witness()).expect("the group applies").doc;
    }
    assert_eq!(pin_at(&moved, ids[0]).pin, x2);
    assert_eq!(pin_at(&moved, ids[2]).pin, x2);
    assert_eq!(
        pin_at(&moved, ids[1]),
        DocRef { id: y, pin: y1 },
        "y is untouched"
    );
}

/// Row 2b — the empty match refuses TYPED, naming the id, and the
/// already-complete update refuses separately: a whole-document update
/// never reports success while recording nothing.
#[test]
fn row2b_no_such_reference_and_already_pinned_refuse_separately() {
    let x = DocumentId::derive("asm-upd-r2b-x");
    let absent = DocumentId::derive("asm-upd-r2b-absent");
    let x1 = content_pin(&part_version(x, 1.0), Tol::witness()).expect("pin");
    let x2 = content_pin(&part_version(x, 2.0), Tol::witness()).expect("pin");
    let (doc, ids) = assembly("asm-upd-r2b-asm", &[DocRef { id: x, pin: x1 }; 2]);

    match update_references(&doc, absent, x2) {
        Err(UpdateError::NoSuchReference { id }) => {
            assert_eq!(id, absent);
            let msg = UpdateError::NoSuchReference { id }.to_string();
            assert!(msg.contains(&absent.to_string()), "{msg}");
        }
        other => panic!("an unreferenced id must refuse NoSuchReference, got {other:?}"),
    }

    // Staged: one site already moved, so the elaboration covers the
    // remaining one and stays appliable.
    let staged = doc
        .apply(&DocEdit::UpdateReference {
            node: ids[0],
            new_pin: x2,
        }, Tol::witness())
        .expect("the first site moves")
        .doc;
    assert_eq!(
        update_references(&staged, x, x2).expect("the second site elaborates"),
        vec![DocEdit::UpdateReference {
            node: ids[1],
            new_pin: x2
        }],
        "a staged state elaborates to exactly the sites that still move"
    );

    let done = staged
        .apply(&DocEdit::UpdateReference {
            node: ids[1],
            new_pin: x2,
        }, Tol::witness())
        .expect("the second site moves")
        .doc;
    match update_references(&done, x, x2) {
        Err(UpdateError::AlreadyPinned { id, pin }) => {
            assert_eq!(id, x);
            assert_eq!(pin, x2);
            let msg = UpdateError::AlreadyPinned { id, pin }.to_string();
            assert!(msg.contains(&x.to_string()), "{msg}");
            assert!(msg.contains(&x2.hex()), "{msg}");
        }
        other => panic!("a completed update must refuse AlreadyPinned, got {other:?}"),
    }
}

// ---- Row 4: the mixed-pin lint ----

/// Row 4 — the lint reports the multiplicity a staged update leaves,
/// listing both pins with their nodes, and reports EMPTY once the
/// state is uniform again. Nothing gates on it: both documents apply
/// edits, save, and evaluate exactly as before.
#[test]
fn row4_the_lint_reports_staged_multiplicity_and_nothing_else() {
    let x = DocumentId::derive("asm-upd-r4-x");
    let y = DocumentId::derive("asm-upd-r4-y");
    let x1 = content_pin(&part_version(x, 1.0), Tol::witness()).expect("pin");
    let x2 = content_pin(&part_version(x, 2.0), Tol::witness()).expect("pin");
    let y1 = content_pin(&part_version(y, 3.0), Tol::witness()).expect("pin");

    let (uniform, ids) = assembly(
        "asm-upd-r4-asm",
        &[
            DocRef { id: x, pin: x1 },
            DocRef { id: y, pin: y1 },
            DocRef { id: x, pin: x1 },
        ],
    );
    assert_eq!(
        mixed_pins(&uniform),
        Vec::new(),
        "one pin per id reports nothing — checked and clean, not unchecked"
    );

    let staged = uniform
        .apply(&DocEdit::UpdateReference {
            node: ids[2],
            new_pin: x2,
        }, Tol::witness())
        .expect("the staged half applies")
        .doc;
    let report = mixed_pins(&staged);
    assert_eq!(report.len(), 1, "only x carries two pins");
    assert_eq!(report[0].id, x, "the entry names the document id");
    let pins: Vec<_> = report[0].pins.iter().map(|s| s.pin).collect();
    let mut expected = vec![x1, x2];
    expected.sort_unstable();
    assert_eq!(pins, expected, "both pins are listed, ascending");
    let sites: Vec<_> = report[0]
        .pins
        .iter()
        .map(|s| (s.pin, s.nodes.clone()))
        .collect();
    assert!(
        sites.contains(&(x1, vec![ids[0]])) && sites.contains(&(x2, vec![ids[2]])),
        "each pin lists the nodes holding it: {sites:?}"
    );

    // Reports, never gates: the mixed-pin document is a first-class
    // value at every door.
    assert!(save(&staged, &[], Tol::witness()).is_ok(), "a mixed-pin document saves");
    assert_eq!(
        mixed_pins(
            &staged
                .apply(&DocEdit::UpdateReference {
                    node: ids[0],
                    new_pin: x2,
                }, Tol::witness())
                .expect("finishing the migration applies")
                .doc
        ),
        Vec::new(),
        "finishing the migration empties the report"
    );
}

// ---- Row 5: memo re-key evidence ----

/// Row 5a — within ONE evaluation, two instances of one document id at
/// DIFFERENT pins are two memo entries: the part evaluates twice and
/// each site gets its own version's geometry. The memo key is the
/// reference, so an updated site cannot be served the old content.
#[test]
fn row5a_a_moved_pin_is_a_different_memo_entry() {
    let id = DocumentId::derive("asm-upd-r5a-part");
    let mut shelf = VersionShelf::default();
    let r1 = shelf.shelve(part_version(id, 1.0), Tol::witness());
    let r2 = shelf.shelve(part_version(id, 2.0), Tol::witness());
    let opts = with_shelf(shelf);

    let (doc, ids) = assembly("asm-upd-r5a-asm", &[r1, r1]);
    let before = run(&doc, &opts);
    assert_eq!(before.part_evaluations, 1, "one version, one evaluation");
    let vol_before = volume(&product(&doc, &before, Tol::witness()).expect("gathers"));

    let staged = doc
        .apply(&DocEdit::UpdateReference {
            node: ids[1],
            new_pin: r2.pin,
        }, Tol::witness())
        .expect("the update applies")
        .doc;
    let after = run(&staged, &opts);
    assert_eq!(
        after.part_evaluations, 2,
        "two pins of one id are two memo entries, not one"
    );
    // Version 1 is a 1×1×1 unit; version 2 is 2×2×1. The staged
    // assembly is one of each; the un-updated one is unchanged.
    let vol_after = volume(&product(&staged, &after, Tol::witness()).expect("gathers"));
    assert!(
        (vol_before - 2.0).abs() < 1e-9,
        "two v1 solids: {vol_before}"
    );
    assert!((vol_after - 5.0).abs() < 1e-9, "v1 + v2: {vol_after}");

    // Fully updated: the OLD content is not served at all.
    let done = staged
        .apply(&DocEdit::UpdateReference {
            node: ids[0],
            new_pin: r2.pin,
        }, Tol::witness())
        .expect("the second site moves")
        .doc;
    let ev = run(&done, &opts);
    assert_eq!(ev.part_evaluations, 1, "one version again, one evaluation");
    let vol_done = volume(&product(&done, &ev, Tol::witness()).expect("gathers"));
    assert!((vol_done - 8.0).abs() < 1e-9, "two v2 solids: {vol_done}");
}

/// Row 5b — the nested case: an assembly instantiates a SUB-ASSEMBLY
/// which instantiates the part. Updating the top document's pin to the
/// sub-assembly's new version serves the new geometry through two
/// seams, and the top document never names the part's pin at all.
#[test]
fn row5b_the_nested_case_serves_the_new_content_through_two_seams() {
    let part_id = DocumentId::derive("asm-upd-r5b-part");
    let sub_id = DocumentId::derive("asm-upd-r5b-sub");
    let mut shelf = VersionShelf::default();
    let p1 = shelf.shelve(part_version(part_id, 1.0), Tol::witness());
    let p2 = shelf.shelve(part_version(part_id, 2.0), Tol::witness());

    // The sub-assembly at each of the part's versions — two versions of
    // ONE sub-assembly id, because the pin it carries is its content.
    let (sub_v1, _) = assembly_under(sub_id, &[p1]);
    let (sub_v2, _) = assembly_under(sub_id, &[p2]);
    let s1 = shelf.shelve(sub_v1, Tol::witness());
    let s2 = shelf.shelve(sub_v2, Tol::witness());
    assert_ne!(s1.pin, s2.pin, "the sub-assembly's own pin moved with it");
    let opts = with_shelf(shelf);

    let (top, ids) = assembly("asm-upd-r5b-top", &[s1]);
    let vol_before = product_volume(&top, &opts);
    assert!(
        (vol_before - 1.0).abs() < 1e-9,
        "one v1 solid: {vol_before}"
    );

    let updated = top
        .apply(&DocEdit::UpdateReference {
            node: ids[0],
            new_pin: s2.pin,
        }, Tol::witness())
        .expect("the update applies")
        .doc;
    let vol_after = product_volume(&updated, &opts);
    assert!((vol_after - 4.0).abs() < 1e-9, "one v2 solid: {vol_after}");
    assert_eq!(
        pin_at(&updated, ids[0]).id,
        sub_id,
        "the top document names the sub-assembly, never the part"
    );
}

/// An assembly under an EXPLICIT id (the nested row needs two contents
/// sharing one identity, which the label-derived helper cannot give).
fn assembly_under(id: DocumentId, refs: &[DocRef]) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let mut doc = ProfileDoc::empty(id, Tol::witness());
    let mut ids = Vec::new();
    for r in refs {
        let (next, node) = insert(doc, Node::instantiate_part(*r));
        doc = next;
        ids.push(node);
    }
    (doc, ids)
}

/// Row 5c — the WARM channel, which is where "the old content is not
/// served" can actually fail. Rows 5a/5b evaluate from cold, so the
/// per-evaluation part cache alone decides their content; the
/// cross-evaluation `prior` memo keys an instantiate node by a
/// ContentKey that must FEED THE PIN, and only an
/// evaluate → update → re-evaluate-with-the-prior session can observe
/// that. The control comes first and proves the channel is live: an
/// unchanged document re-evaluated over its own prior does no part
/// work at all, so anything the update serves afterwards had to be
/// re-keyed rather than replayed.
#[test]
fn row5c_a_warm_prior_never_serves_the_old_content_after_an_update() {
    let id = DocumentId::derive("asm-upd-r5c-part");
    let mut shelf = VersionShelf::default();
    let r1 = shelf.shelve(part_version(id, 1.0), Tol::witness());
    let r2 = shelf.shelve(part_version(id, 2.0), Tol::witness());
    let opts = with_shelf(shelf);

    let (doc, ids) = assembly("asm-upd-r5c-asm", &[r1, r1]);
    let cold = run(&doc, &opts);
    assert_eq!(cold.part_evaluations, 1, "the cold run warms the memo");

    // Control: re-evaluating the SAME document over that prior does no
    // part work. Without this the rows below would prove nothing — a
    // prior that never serves instantiate nodes cannot serve a stale
    // one either.
    let control = run_warm(&doc, &cold, &opts);
    assert_eq!(
        control.part_evaluations, 0,
        "the prior really does serve instantiate nodes"
    );
    let vol_control = volume(&product(&doc, &control, Tol::witness()).expect("gathers"));
    assert!((vol_control - 2.0).abs() < 1e-9, "two v1 solids");

    // One site updated, re-evaluated over the SAME warm prior: the
    // moved reference is re-keyed and re-resolved, the untouched one
    // is served from the prior, and the product is one solid of each.
    let staged = doc
        .apply(&DocEdit::UpdateReference {
            node: ids[1],
            new_pin: r2.pin,
        }, Tol::witness())
        .expect("the update applies")
        .doc;
    let warm_staged = run_warm(&staged, &cold, &opts);
    assert_eq!(
        warm_staged.part_evaluations, 1,
        "only the moved reference does part work"
    );
    let vol_staged = volume(&product(&staged, &warm_staged, Tol::witness()).expect("gathers"));
    assert!((vol_staged - 5.0).abs() < 1e-9, "v1 + v2: {vol_staged}");

    // The second site moves over the STAGED prior: the old version is
    // now named nowhere, and nothing serves it.
    let done = staged
        .apply(&DocEdit::UpdateReference {
            node: ids[0],
            new_pin: r2.pin,
        }, Tol::witness())
        .expect("the update applies")
        .doc;
    let warm_done = run_warm(&done, &warm_staged, &opts);
    assert_eq!(warm_done.part_evaluations, 1, "one reference moved");
    let vol_done = volume(&product(&done, &warm_done, Tol::witness()).expect("gathers"));
    assert!((vol_done - 8.0).abs() < 1e-9, "two v2 solids: {vol_done}");
}

/// Row 5d — the warm channel in the NESTED case: the top document's
/// reference moves to a sub-assembly version whose own inner reference
/// changed, re-evaluated over a warm prior. The new geometry arrives
/// through two seams without the top document ever naming the part.
#[test]
fn row5d_a_warm_prior_carries_a_nested_update_through_two_seams() {
    let part_id = DocumentId::derive("asm-upd-r5d-part");
    let sub_id = DocumentId::derive("asm-upd-r5d-sub");
    let mut shelf = VersionShelf::default();
    let p1 = shelf.shelve(part_version(part_id, 1.0), Tol::witness());
    let p2 = shelf.shelve(part_version(part_id, 2.0), Tol::witness());
    let (sub_v1, _) = assembly_under(sub_id, &[p1]);
    let (sub_v2, _) = assembly_under(sub_id, &[p2]);
    let s1 = shelf.shelve(sub_v1, Tol::witness());
    let s2 = shelf.shelve(sub_v2, Tol::witness());
    let opts = with_shelf(shelf);

    let (top, ids) = assembly("asm-upd-r5d-top", &[s1]);
    let cold = run(&top, &opts);
    let control = run_warm(&top, &cold, &opts);
    assert_eq!(
        control.part_evaluations, 0,
        "the nested prior serves the instantiate node too"
    );
    let vol_before = volume(&product(&top, &control, Tol::witness()).expect("gathers"));
    assert!((vol_before - 1.0).abs() < 1e-9, "one v1 solid");

    let updated = top
        .apply(&DocEdit::UpdateReference {
            node: ids[0],
            new_pin: s2.pin,
        }, Tol::witness())
        .expect("the update applies")
        .doc;
    let warm = run_warm(&updated, &cold, &opts);
    let vol_after = volume(&product(&updated, &warm, Tol::witness()).expect("gathers"));
    assert!((vol_after - 4.0).abs() < 1e-9, "one v2 solid: {vol_after}");
}

// ---- Row 6: the assembly's own pin ----

/// Row 6 — the updated ASSEMBLY's own A4 pin moves, because canonical
/// bytes include node data. Both directions: the update moves it, and
/// it is a function of the resulting STATE rather than of the path —
/// an assembly reached by update pins identically to one authored at
/// the new version directly, and an unrelated edit composes either way
/// round to the same pin.
#[test]
fn row6_the_assembly_pin_moves_on_update_and_states_history() {
    let id = DocumentId::derive("asm-upd-r6-part");
    let v1 = content_pin(&part_version(id, 1.0), Tol::witness()).expect("pin");
    let v2 = content_pin(&part_version(id, 2.0), Tol::witness()).expect("pin");

    let (before, ids) = assembly("asm-upd-r6-asm", &[DocRef { id, pin: v1 }]);
    let after = before
        .apply(&DocEdit::UpdateReference {
            node: ids[0],
            new_pin: v2,
        }, Tol::witness())
        .expect("applies")
        .doc;
    let pin_before = content_pin(&before, Tol::witness()).expect("pin");
    let pin_after = content_pin(&after, Tol::witness()).expect("pin");
    assert_ne!(pin_before, pin_after, "a pin move moves the assembly's pin");

    // Reached-by-update == authored-directly: identity is state.
    let (direct, _) = assembly("asm-upd-r6-asm", &[DocRef { id, pin: v2 }]);
    assert_eq!(
        content_pin(&direct, Tol::witness()).expect("pin"),
        pin_after,
        "the log is not part of the content"
    );

    // An unrelated edit keeps moving the pin exactly as it did — the
    // update arm smuggles no path into the canonical bytes.
    let unrelated = DocEdit::SetPlacement {
        node: ids[0],
        frame: Frame::translation([1.0, 2.0, 3.0]),
    };
    let update = DocEdit::UpdateReference {
        node: ids[0],
        new_pin: v2,
    };
    let a = before
        .apply(&unrelated, Tol::witness())
        .expect("applies")
        .doc
        .apply(&update, Tol::witness())
        .expect("applies")
        .doc;
    let b = after.apply(&unrelated, Tol::witness()).expect("applies").doc;
    assert_eq!(
        content_pin(&a, Tol::witness()).expect("pin"),
        content_pin(&b, Tol::witness()).expect("pin"),
        "the two edits commute into one content"
    );
    assert_ne!(
        content_pin(&a, Tol::witness()).expect("pin"),
        pin_after,
        "and the unrelated edit still moves the pin on its own"
    );
}

// ---- Row 7: persistence ----

/// Row 7 — a document whose LOG carries an update round-trips: save
/// writes the log, load replays it, and the loaded document names the
/// new version. Undo across the load is dropping the last entry and
/// replaying — which restores the prior pin exactly.
#[test]
fn row7_a_logged_update_round_trips_and_undoes_across_the_load() {
    let id = DocumentId::derive("asm-upd-r7-part");
    let v1 = content_pin(&part_version(id, 1.0), Tol::witness()).expect("pin");
    let v2 = content_pin(&part_version(id, 2.0), Tol::witness()).expect("pin");

    let mut rec = fixture::Recorder::new();
    let asm_id = rec.doc.id();
    let node = rec.insert(Node::instantiate_part(DocRef { id, pin: v1 }));
    rec.push(DocEdit::UpdateReference { node, new_pin: v2 });

    // Snapshot = the EMPTY document, log = everything: the load
    // replays the update through `apply`'s own doors.
    let text = save(&ProfileDoc::empty(asm_id, Tol::witness()), &rec.edits, Tol::witness()).expect("the document saves");
    let loaded = load(&text, Tol::witness()).expect("the document loads");
    assert_eq!(loaded.edits.len(), rec.edits.len(), "the log persisted");
    assert_eq!(pin_at(&loaded.doc, node).pin, v2, "the replay lands on v2");
    assert!(
        loaded.doc.bit_eq(&rec.doc),
        "the loaded document is the recorded one, bit for bit"
    );

    // Undo across the load: replay every edit but the last.
    let undone = ProfileDoc::replay(asm_id, &loaded.edits[..loaded.edits.len() - 1], Tol::witness())
        .expect("the shortened log replays");
    assert_eq!(
        pin_at(&undone, node).pin,
        v1,
        "dropping the update restores the prior version"
    );
}
