//! **The instantiation seam carries mate identity and mint health.**
//!
//! A part's declared contact RECORDS cross the document seam with its
//! geometry, and always did. What crosses here is the bookkeeping that
//! goes with them — which mate of which document authored each record,
//! and which of that document's mates could not be minted at all — so
//! that the outermost gate can name a carried declaration's mate
//! instead of reporting it anonymously, and can refuse over a part
//! whose own contacts nothing verified.
//!
//! Two claims, and the rows are grouped by them:
//!
//! - **The channel is total.** Every declaration an inner document
//!   minted has exactly one row in the outer product, its faces the
//!   image of the inner pair under the graft's descendant map — the
//!   same map, the same call, as the records themselves ride. A row
//!   from deeper keeps its route.
//! - **The outermost gate reads both.** A refuted carried declaration
//!   names its mate, its document and the instances it arrived
//!   through; an inner mate that could not be minted refuses the outer
//!   gate before anything else this document has to say.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Alignment, Assembly, AssemblyError, Attribution, AxisSense, CancelToken, CapEnd,
    CarriedRelation, ContactClass, DocEdit, DocRef, DocumentId, EntityKind, EvalOptions, Evaluation,
    Frame, MateFrame, MatePrimitive, Node, ProfileDoc, RecipeNodeId, ResolveFailure, ResolveFault,
    RoleSeg, SitedRef, StableName, assemble, content_pin, evaluate, product_recorded,
};
use fixture::{insert, len, on_frame, step};
use geom_core::Tol;

// ---- store / eval plumbing (the mate suites' shape) ----

#[derive(Debug, Default, Clone)]
struct StubStore {
    docs: BTreeMap<DocumentId, ProfileDoc>,
}

impl StubStore {
    fn insert(&mut self, doc: ProfileDoc, tol: Tol) -> DocRef {
        let pin = content_pin(&doc, tol).expect("the pin computes");
        let id = doc.id();
        self.docs.insert(id, doc);
        DocRef { id, pin }
    }
}

impl editor_core::PartResolver for StubStore {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        let fail = |fault, message: &str| ResolveFailure {
            fault,
            message: message.to_string(),
        };
        let doc = self
            .docs
            .get(&doc_ref.id)
            .ok_or_else(|| fail(ResolveFault::Unresolved, "no such document"))?;
        if content_pin(doc, Tol::witness()).expect("the pin computes") != doc_ref.pin {
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
    evaluate::<f64>(doc, None, &CancelToken::new(), o, Tol::witness())
}

/// The unit cube part every fixture below is built out of.
fn cube_part(label: &str) -> ProfileDoc {
    let (doc, profile) = on_frame(
        ProfileDoc::empty(DocumentId::derive(label), Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
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

/// The node a one-block part's own faces are minted by: a block is
/// three nodes — the sketch frame, the profile drawn on it, then the
/// extrude — so the body's names carry the third.
const PART_BODY: RecipeNodeId = RecipeNodeId(2);

/// A face of `instance`'s part product, named through the instance
/// qualifier (A12's reading-edge head).
fn in_part(instance: RecipeNodeId, cap: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: PART_BODY,
                path: vec![RoleSeg::Cap(cap)],
            }),
        }],
    }
}

/// The same reading one level deeper: `instance`'s part is ITSELF an
/// assembly, and the face wanted is a cap of the cube inside that
/// assembly's sub-instance `sub`.
fn in_part_in_part(instance: RecipeNodeId, sub: RecipeNodeId, cap: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: Box::new(in_part(sub, cap)),
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

fn mate_node(
    a: StableName,
    b: StableName,
    class: ContactClass,
    seat: f64,
) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a: SitedRef::at_mint(a),
        b: SitedRef::at_mint(b),
        class,
        alignment: Alignment {
            a: frame([0.0, 0.0, seat], [0.0, 0.0, 1.0]),
            b: frame([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Aligned,
            clocking: None,
        },
    }
}

/// Two instances of `part`, stacked by ONE mate at `seat` — the mate
/// suites' stand. `Rest` at seat 1.0 is a true declaration; any other
/// seat is a false one; `Tangent` is a class that mints no record at
/// rest, so the stand refuses its OWN gate.
fn stand(
    label: &str,
    part: DocRef,
    class: ContactClass,
    seat: f64,
) -> (ProfileDoc, Vec<RecipeNodeId>, RecipeNodeId) {
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..2 {
        let (next, id) = insert(doc, Node::instantiate_part(part));
        doc = next;
        ids.push(id);
    }
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: mate_node(
                in_part(ids[0], CapEnd::End),
                in_part(ids[1], CapEnd::Start),
                class,
                seat,
            ),
        },
    );
    (doc, ids, mate.expect("the mate inserts"))
}

/// `count` instances of `part` in a row, `spacing` apart along +x and
/// declaring nothing about each other.
fn row_of(
    label: &str,
    part: DocRef,
    count: usize,
    spacing: f64,
) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for i in 0..count {
        let (next, id) = insert(doc, Node::instantiate_part(part));
        doc = next;
        if i > 0 {
            #[allow(clippy::cast_precision_loss)]
            let dx = spacing * i as f64;
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

fn findings_of(result: &Result<Assembly<f64>, AssemblyError>) -> Vec<editor_core::AtRestFinding> {
    match result {
        Err(AssemblyError::AtRest { findings } | AssemblyError::Uncertified { findings, .. }) => {
            findings.clone()
        }
        _ => Vec::new(),
    }
}

// ---- The channel is total ----

/// A1: one carried row per inner declaration, keyed onto the outer
/// product by the SAME map that carried the record.
///
/// The oracle for "the image under the graft's map" is the record
/// carry itself: `carry_contacts` and the declaration carry read the
/// one `GraftKeys` the graft returned, so a declaration whose faces are
/// not a `PatchContact` of the gathered set is a declaration that took
/// some other route — a positional lookup or a re-measurement — and the
/// row says so without reaching into the bridge.
#[test]
fn every_carried_row_is_an_inner_declaration_re_keyed_by_the_graft() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("docm6-a1-cube"), Tol::witness());
    let (inner, _, inner_mate) = stand("docm6-a1-stand", part, ContactClass::Rest, 1.0);
    let inner_id = inner.id();
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, instances) = row_of("docm6-a1-row", inner_ref, 3, 4.0);

    let ev = run(&outer, &opts(store));
    let gathered = product_recorded(&outer, &ev, Tol::witness()).expect("the row gathers");

    // One row per instance of the stand, in gather order, each naming
    // the instance it came through and the document that minted it.
    assert_eq!(
        gathered
            .carried
            .iter()
            .map(|c| (c.through, c.of, c.via.clone(), c.declaration.mate))
            .collect::<Vec<_>>(),
        instances
            .iter()
            .map(|&i| (i, inner_id, Vec::new(), inner_mate))
            .collect::<Vec<_>>(),
    );
    // This document minted nothing of its own, and carried no refusal.
    assert!(gathered.minted.is_empty());
    assert!(gathered.unminted.is_empty());
    assert!(gathered.carried_unminted.is_empty());
    // Every carried pair is a pair of the gathered RECORD set: the two
    // carries agree because they read one map.
    for row in &gathered.carried {
        let (a, b) = row.declaration.faces;
        assert!(
            gathered
                .contacts
                .patches
                .iter()
                .any(|p| (p.face_a, p.face_b) == (a, b) || (p.face_a, p.face_b) == (b, a)),
            "a carried declaration names a pair no carried record does: {row:?}"
        );
        assert!(
            gathered.body.faces().any(|(k, _)| k == a)
                && gathered.body.faces().any(|(k, _)| k == b),
            "and both faces are live in the aggregate: {row:?}"
        );
    }
}

/// A1: three instantiation levels — a part in a stand, the stand in a
/// mid document, the mid in the outer row. The stand's mate reaches the
/// outer gate naming its whole route: the outer instance it came
/// through, then the mid's instance of the stand.
#[test]
fn a_three_level_assembly_carries_with_via_naming_the_route() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("docm6-deep-cube"), Tol::witness());
    let (inner, _, inner_mate) = stand("docm6-deep-stand", part, ContactClass::Rest, 1.0);
    let inner_id = inner.id();
    let inner_ref = store.insert(inner, Tol::witness());
    let (mid, mid_instances) = row_of("docm6-deep-mid", inner_ref, 2, 4.0);
    let mid_id = mid.id();
    let mid_ref = store.insert(mid, Tol::witness());
    let (outer, outer_instances) = row_of("docm6-deep-outer", mid_ref, 2, 12.0);

    let ev = run(&outer, &opts(store));
    let gathered = product_recorded(&outer, &ev, Tol::witness()).expect("the deep row gathers");

    // Four stands, four rows, each naming the document whose mate it
    // is — the STAND, never the mid the row passed through — with the
    // mid's own instance kept as the route below `through`.
    let expected: Vec<_> = outer_instances
        .iter()
        .flat_map(|&outer_i| {
            mid_instances
                .iter()
                .map(move |&mid_i| (outer_i, inner_id, vec![mid_i], inner_mate))
        })
        .collect();
    assert_eq!(
        gathered
            .carried
            .iter()
            .map(|c| (c.through, c.of, c.via.clone(), c.declaration.mate))
            .collect::<Vec<_>>(),
        expected
    );
    assert_ne!(inner_id, mid_id, "the two documents are distinct");
    // And the whole tree still certifies: naming a mate decides
    // nothing about the geometry.
    assert!(assemble(&outer, &ev, Tol::witness()).is_ok());
}

// ---- The outermost gate reads what was carried ----

/// A2: a part whose declared contact its placement contradicts. The
/// outer census refutes the carried declaration — and the finding names
/// the mate, the document and the instance, where before it named
/// nobody.
#[test]
fn a_refuted_carried_declaration_names_its_mate_and_route() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("docm6-a2-cube"), Tol::witness());
    // Seat 0.5: the two cubes interpenetrate, so the declared rest is
    // definite counter-evidence.
    let (inner, _, inner_mate) = stand("docm6-a2-stand", part, ContactClass::Rest, 0.5);
    let inner_id = inner.id();
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, instances) = row_of("docm6-a2-row", inner_ref, 1, 4.0);

    let ev = run(&outer, &opts(store));
    let result = assemble(&outer, &ev, Tol::witness());
    let findings = findings_of(&result);
    assert!(!findings.is_empty(), "the outer gate refuses: {result:?}");
    assert!(
        findings.iter().any(|f| matches!(
            &f.attribution,
            Attribution::Carried { through, of, via, declaration, .. }
                if *through == instances[0]
                    && *of == inner_id
                    && via.is_empty()
                    && declaration.mate == inner_mate
        )),
        "the carried declaration names its mate and route: {findings:?}"
    );
    // The subject a reader sees says which file to open.
    let rendered = findings
        .iter()
        .map(|f| f.attribution.to_string())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        rendered.contains(&inner_id.to_string()) && rendered.contains("carried through instance"),
        "and it renders as one: {rendered}"
    );
}

/// A3: `Unattributed` means what it says. Over every fixture whose
/// outer gate refuses, an unattributed finding is a contact NO
/// declaration of the tree answers for — never a carried declaration's
/// refutation or decline.
///
/// The listed arms are `attribute`'s own unattributable vocabulary, and
/// the row names them rather than accepting whatever turns up: a new
/// arm reaching here is a classification decision, not a pass.
#[test]
fn unattributed_is_only_a_finding_no_declaration_answers_for() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("docm6-a3-cube"), Tol::witness());
    let (penetrating, _, _) = stand("docm6-a3-pen", part, ContactClass::Rest, 0.5);
    let penetrating = store.insert(penetrating, Tol::witness());
    let (gapped, _, _) = stand("docm6-a3-gap", part, ContactClass::Rest, 1.5);
    let gapped = store.insert(gapped, Tol::witness());

    let fixtures = [
        row_of("docm6-a3-pen-row", penetrating, 1, 4.0),
        row_of("docm6-a3-pen-pair", penetrating, 2, 4.0),
        row_of("docm6-a3-gap-row", gapped, 1, 4.0),
        // Two stands one unit apart: their cubes meet on a face NO
        // mate declared, which is the undeclared-contact hard error.
        row_of("docm6-a3-touching", gapped, 2, 1.0),
    ];
    let mut seen_unattributed = 0;
    let mut refused = 0;
    for (doc, _) in &fixtures {
        let ev = run(doc, &opts(store.clone()));
        let result = assemble(doc, &ev, Tol::witness());
        let findings = findings_of(&result);
        if !findings.is_empty() {
            refused += 1;
        }
        for finding in &findings {
            if !matches!(finding.attribution, Attribution::Unattributed) {
                continue;
            }
            seen_unattributed += 1;
            let rendered = format!("{:?}", finding.error);
            assert!(
                ["UndeclaredContact", "UndeclaredCusp", "CensusEscalated"]
                    .iter()
                    .any(|arm| rendered.contains(arm)),
                "an unattributed finding is one no declaration answers for: {rendered}"
            );
        }
    }
    assert!(refused > 0, "the fixtures refuse");
    assert!(
        seen_unattributed > 0,
        "and at least one undeclared contact was actually reached"
    );
}

// ---- Inner mint health at the outermost gate ----

/// A part whose only mate is a `Tangent` — a class that solves and
/// mints NO record at rest, so the part refuses its own gate.
fn broken_part(store: &mut StubStore, label: &str) -> (DocRef, DocumentId, RecipeNodeId) {
    let cube = store.insert(cube_part(&format!("{label}-cube")), Tol::witness());
    let (doc, _, mate) = stand(label, cube, ContactClass::Tangent, 5.0);
    let id = doc.id();
    (store.insert(doc, Tol::witness()), id, mate)
}

/// A4: an inner document with an unmintable mate refuses the OUTER
/// gate, naming the document, the mate and the instance — and the inner
/// document alone refuses exactly as it did before.
#[test]
fn an_inner_mint_refusal_refuses_the_outer_gate_naming_document_and_mate() {
    let mut store = StubStore::default();
    let (inner_ref, inner_id, inner_mate) = broken_part(&mut store, "docm6-a4-stand");
    let (outer, instances) = row_of("docm6-a4-row", inner_ref, 1, 4.0);

    // The inner document alone: unchanged, its own class refusal.
    let inner = store.docs[&inner_id].clone();
    let inner_ev = run(&inner, &opts(store.clone()));
    assert!(matches!(
        assemble(&inner, &inner_ev, Tol::witness()),
        Err(AssemblyError::NoAtRestRecord { mate, .. }) if mate == inner_mate
    ));

    let ev = run(&outer, &opts(store));
    let result = assemble(&outer, &ev, Tol::witness());
    let Err(AssemblyError::CarriedMintRefusal {
        through,
        of,
        via,
        refusal,
    }) = &result
    else {
        panic!("an inner part with an unverified contact is not at rest: {result:?}");
    };
    assert_eq!((*through, *of, via.len()), (instances[0], inner_id, 0));
    assert_eq!(refusal.mate(), inner_mate);
    // The landing's badge renders the refusal through its `Display`,
    // and what it must say is which file to open.
    let rendered = result.unwrap_err().to_string();
    assert!(
        rendered.contains(&inner_id.to_string())
            && rendered.contains(&format!("mate {}", inner_mate.0)),
        "the badge names the document and the mate: {rendered}"
    );
}

/// A4's ORDER, pinned by three documents that differ in one thing
/// each. The carried refusal is raised BEFORE this document's own
/// unminted head and before the at-rest gate — and the two rows below
/// it show those arms are live rather than absent.
#[test]
fn the_carried_refusal_precedes_the_own_unminted_head_and_the_at_rest_gate() {
    let mut store = StubStore::default();
    let (broken, broken_id, _) = broken_part(&mut store, "docm6-order-broken");
    let cube = store.insert(cube_part("docm6-order-cube"), Tol::witness());
    let (good, _, _) = stand("docm6-order-good", cube, ContactClass::Rest, 1.0);
    let good = store.insert(good, Tol::witness());

    // The three documents share a shape: two instances a unit apart,
    // so their cubes MEET on a face no mate declared (the at-rest
    // gate's hard error), plus one `Tangent` mate of this document's
    // own (its unminted head).
    let build = |label: &str, part: DocRef| {
        let (doc, ids) = row_of(label, part, 2, 1.0);
        (doc, ids)
    };
    // The instances here are of a STAND, so a face of one is named
    // one wrapper deeper: through the stand's own instance of the cube.
    let with_own_mate = |doc: ProfileDoc, ids: &[RecipeNodeId]| {
        let (doc, mate) = step(
            doc,
            DocEdit::InsertNode {
                node: mate_node(
                    in_part_in_part(ids[0], RecipeNodeId(1), CapEnd::End),
                    in_part_in_part(ids[1], RecipeNodeId(0), CapEnd::Start),
                    ContactClass::Tangent,
                    5.0,
                ),
            },
        );
        (doc, mate.expect("the mate inserts"))
    };

    // (a) The at-rest gate alone: good parts, no mate of this
    // document's own — the undeclared contact is what refuses.
    let (doc, _) = build("docm6-order-atrest", good);
    let ev = run(&doc, &opts(store.clone()));
    assert!(matches!(
        assemble(&doc, &ev, Tol::witness()),
        Err(AssemblyError::AtRest { .. })
    ));

    // (b) Add this document's own unminted mate: it preempts the gate.
    let (doc, ids) = build("docm6-order-own", good);
    let (doc, own_mate) = with_own_mate(doc, &ids);
    let ev = run(&doc, &opts(store.clone()));
    assert!(matches!(
        assemble(&doc, &ev, Tol::witness()),
        Err(AssemblyError::NoAtRestRecord { mate, .. }) if mate == own_mate
    ));

    // (c) The same document over a BROKEN part: the carried refusal
    // preempts both, because the file to open is the inner one.
    let (doc, ids) = build("docm6-order-carried", broken);
    let (doc, _) = with_own_mate(doc, &ids);
    let ev = run(&doc, &opts(store));
    let result = assemble(&doc, &ev, Tol::witness());
    assert!(
        matches!(&result, Err(AssemblyError::CarriedMintRefusal { of, .. }) if *of == broken_id),
        "inner mint health is read first: {result:?}"
    );
}

/// A4: two levels down, the refusal names the whole route.
#[test]
fn a_refusal_two_levels_down_names_its_route() {
    let mut store = StubStore::default();
    let (broken, broken_id, broken_mate) = broken_part(&mut store, "docm6-route-stand");
    let (mid, mid_instances) = row_of("docm6-route-mid", broken, 1, 4.0);
    let mid_ref = store.insert(mid, Tol::witness());
    let (outer, outer_instances) = row_of("docm6-route-outer", mid_ref, 1, 12.0);

    let ev = run(&outer, &opts(store));
    let result = assemble(&outer, &ev, Tol::witness());
    let Err(AssemblyError::CarriedMintRefusal {
        through,
        of,
        via,
        refusal,
    }) = &result
    else {
        panic!("the refusal reaches the outermost gate: {result:?}");
    };
    assert_eq!(
        (*through, *of, via.as_slice(), refusal.mate()),
        (
            outer_instances[0],
            broken_id,
            &mid_instances[..],
            broken_mate
        )
    );
}

/// The advisory alternative was not built, and the row says where to
/// look: `carried_unminted` is READ at exactly one place — the gate's
/// refusal — so there is no channel by which a partial assembly could
/// be certified over an inner refusal and the refusals reported as
/// findings beside it.
#[test]
fn no_advisory_channel_reports_inner_refusals_beside_a_certification() {
    let mut store = StubStore::default();
    let (broken, _, _) = broken_part(&mut store, "docm6-advisory-stand");
    let (outer, _) = row_of("docm6-advisory-row", broken, 1, 4.0);
    let ev = run(&outer, &opts(store));
    // The gather still carries the row — the channel is the gather's —
    // and the gate is where it becomes a refusal, with no success arm
    // that could carry it past.
    let gathered = product_recorded(&outer, &ev, Tol::witness()).expect("the row gathers");
    assert_eq!(gathered.carried_unminted.len(), 1);
    assert!(matches!(
        assemble(&outer, &ev, Tol::witness()),
        Err(AssemblyError::CarriedMintRefusal { .. })
    ));
}

/// A carried `Tangent` refusal and a carried DECLARATION are different
/// facts about the same seam, and a document that carries both refuses
/// on the mint health: the record set the at-rest gate would judge is
/// already known to be short.
#[test]
fn a_carried_declaration_does_not_soften_a_carried_refusal() {
    let mut store = StubStore::default();
    let cube = store.insert(cube_part("docm6-mixed-cube"), Tol::witness());
    let (good, _, _) = stand("docm6-mixed-good", cube, ContactClass::Rest, 1.0);
    let good = store.insert(good, Tol::witness());
    let (broken, broken_id, _) = broken_part(&mut store, "docm6-mixed-broken");

    let mut doc = ProfileDoc::empty(DocumentId::derive("docm6-mixed-row"), Tol::witness());
    let (next, _) = insert(doc, Node::instantiate_part(good));
    doc = next;
    let (next, second) = insert(doc, Node::instantiate_part(broken));
    doc = next;
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: second,
            frame: Frame::translation([8.0, 0.0, 0.0]),
        },
    );

    let ev = run(&doc, &opts(store));
    let gathered = product_recorded(&doc, &ev, Tol::witness()).expect("the mixed row gathers");
    assert_eq!(gathered.carried.len(), 1, "the good stand's declaration");
    assert_eq!(gathered.carried_unminted.len(), 1, "the broken stand's mate");
    assert!(
        matches!(
            assemble(&doc, &ev, Tol::witness()),
            Err(AssemblyError::CarriedMintRefusal { of, .. }) if of == broken_id
        ),
        "mint health is read first"
    );
}

/// The relation a carried finding bears is the kernel's, decided by
/// the same dispatch as an own-minted one: a refuted carried
/// declaration is `Refuted` and reaches the verdict arm, never the
/// unrefuted frontier.
#[test]
fn a_carried_refutation_is_a_verdict_against_the_document() {
    let mut store = StubStore::default();
    let part = store.insert(cube_part("docm6-relation-cube"), Tol::witness());
    let (inner, _, _) = stand("docm6-relation-stand", part, ContactClass::Rest, 1.5);
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("docm6-relation-row", inner_ref, 1, 4.0);

    let ev = run(&outer, &opts(store));
    let result = assemble(&outer, &ev, Tol::witness());
    assert!(
        matches!(&result, Err(AssemblyError::AtRest { .. })),
        "a refuted declaration is a finding against the document: {result:?}"
    );
    assert!(
        findings_of(&result).iter().any(|f| matches!(
            f.attribution,
            Attribution::Carried {
                relation: CarriedRelation::Refuted,
                ..
            }
        )),
        "and its relation is the refuting one"
    );
}
