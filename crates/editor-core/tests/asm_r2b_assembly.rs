//! **ASM-R2b acceptance** — declaration minting and the assembly
//! at-rest gate (acceptance rows 1–5 and 7; ASSEMBLY-DESIGN
//! A3/A4/A5/A13 clause 4).
//!
//! One assertion per acceptance row, each comment stating the
//! INVARIANT the row pins rather than the mechanics it exercises.
//!
//! # The F1 declared direction does NOT go green — and the door says so
//!
//! **Stated as executed, not as hoped.** The spec's F1 sentence says a
//! declared planar Rest between two touching instances VALIDATES. It
//! does not, in this tree: the census's patch certifier DECLINES every
//! cross-instance pair, for the structural reason
//! `AssemblyError::Uncertified` gives at its own definition.
//!
//! What matters for these rows is that the boundary is a variant, not
//! a paragraph. [`declared_frontier`] is where they read it,
//! `a_mixed_verdict_is_the_at_rest_arm_not_the_frontier` pins the case
//! that separates the two arms, and `row4_a` pins that a REFUTED
//! declaration is never dressed as a decline. The declaration still
//! does its job: it is what suppresses the F1 `UndeclaredContact`
//! refusal (row 3).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Alignment, AssemblyError, AxisSense, CancelToken, CapEnd, ContactClass, DocEdit, DocRef,
    DocumentId, EntityKey, EntityKind, EntityRef, Entry, EvalOptions, Evaluation,
    InterfaceCrossing, MateFrame, MatePrimitive, Node, NodeErrorKind, ProfileDoc, RecipeNodeId,
    ResolveFailure, ResolveFault, RoleSeg, StableName, assemble, content_pin, evaluate, inline,
    product_recorded, split,
};
use fixture::{desc, insert, len, step};
use geom_core::Tol;

// ---- The stub store (the ASM-2A/R2a shape, verbatim in spirit) ----

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
        ProfileDoc::empty(DocumentId::derive(label), Tol::witness()),
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
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
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

/// A `Rest` mate declaring instance `a`'s TOP cap against instance
/// `b`'s BOTTOM cap, seating `b` at height `seat` by frame
/// coincidence.
///
/// Frame coincidence rather than a bare planar rest because ONE
/// planar rest leaves a positive-dimensional residual and A11 rule 4
/// refuses an UNDER pair — the pair must be DETERMINED for the
/// instances to have poses at all, and this unit is about what a
/// solved mate DECLARES, not about re-testing the coset fold.
/// `seat = 1.0` puts `b`'s bottom exactly on `a`'s top (the unit cube
/// is z ∈ [0,1]); anything larger leaves a definite gap.
fn rest_mate(a: RecipeNodeId, b: RecipeNodeId, seat: f64) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a: in_part(a, CapEnd::Top),
        b: in_part(b, CapEnd::Bottom),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: frame([0.0, 0.0, seat], [0.0, 0.0, 1.0]),
            b: frame([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Aligned,
            clocking: None,
        },
    }
}

/// Two instances of the unit cube, plus the seating mate at `seat`.
/// Returns (document, instance ids, mate id, store).
fn stacked(label: &str, seat: f64) -> (ProfileDoc, Vec<RecipeNodeId>, RecipeNodeId, StubStore) {
    let mut store = StubStore::default();
    let doc_ref = store.insert(cube_part(&format!("{label}-part")), Tol::witness());
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..2 {
        let (next, id) = insert(doc, Node::instantiate_part(doc_ref));
        doc = next;
        ids.push(id);
    }
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: rest_mate(ids[0], ids[1], seat),
        },
    );
    (doc, ids, mate.expect("the mate mints"), store)
}

/// The kernel findings the gate raised — from EITHER refusing arm, so
/// a row asserting about the finding SET does not have to know which
/// side of the split its fixture lands on. The empty vector for a
/// gate that passed.
fn findings(result: &Result<editor_core::Assembly<f64>, AssemblyError>) -> Vec<String> {
    match result {
        Ok(_) => Vec::new(),
        Err(AssemblyError::AtRest { findings } | AssemblyError::Uncertified { findings, .. }) => {
            findings.iter().map(|f| format!("{:?}", f.error)).collect()
        }
        Err(other) => panic!("expected an at-rest verdict, got {other}"),
    }
}

/// **The declared direction's frontier, read in ONE place.**
///
/// `assemble` does not reach its success arm for a document whose
/// mates declare a cross-instance contact: the certifier DECLINES
/// every declared pair rather than certifying or refuting it, which
/// the door states as its own [`AssemblyError::Uncertified`] arm.
/// Every row that needs such a document's minting reads it through
/// here, so the day the census grows its cross-instance chart rung,
/// `assemble` returns `Ok`, this function goes RED, and every row
/// using it moves with the prose.
///
/// Returns the record set the gate was handed and its findings.
fn declared_frontier(
    result: &Result<editor_core::Assembly<f64>, AssemblyError>,
) -> (&topo::ContactRecords, &[editor_core::AtRestFinding]) {
    match result.as_ref().expect_err(
        "a declared cross-instance contact does not reach the success arm \
         in this tree — if it now does, the frontier moved and the door's \
         own statement of it must move with the census",
    ) {
        AssemblyError::Uncertified { contacts, findings } => (contacts, findings),
        other => panic!(
            "the frontier is the UNCERTIFIED arm — declines and nothing \
             else, never a finding against the geometry: {other}"
        ),
    }
}

/// Every declaration the findings name, and in what relation.
fn relations(findings: &[editor_core::AtRestFinding]) -> Vec<(RecipeNodeId, &'static str)> {
    findings
        .iter()
        .map(|f| match &f.attribution {
            editor_core::Attribution::Refuted(m) => (m.mate, "refuted"),
            editor_core::Attribution::Declined(m) => (m.mate, "declined"),
            editor_core::Attribution::Unattributed => (RecipeNodeId(u64::MAX), "unattributed"),
        })
        .collect()
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
    let doc_ref = store.insert(part.clone(), Tol::witness());

    // What the part's own product records, on its own keys.
    let part_ev = run(&part, &EvalOptions::default());
    let part_product = product_recorded(&part, &part_ev, Tol::witness()).expect("the part gathers");
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
        ProfileDoc::empty(DocumentId::derive("asm-r2b-row1"), Tol::witness()),
        Node::instantiate_part(doc_ref),
    );
    let ev = run(&doc, &opts(store));
    let product = product_recorded(&doc, &ev, Tol::witness()).expect("the assembly gathers");

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
///
/// Asserted POSITIVELY, on the record set itself: the uncertified arm
/// hands back what the gate was given, so the `PatchContact` claim is
/// pinned by the type of the thing in `patches`, not inferred from a
/// refusal's shape.
#[test]
fn row2_a_solved_rest_mate_mints_its_declaration() {
    let (doc, ids, mate, store) = stacked("asm-r2b-row2", 1.0);
    let ev = run(&doc, &opts(store));

    // The gather itself mints nothing: minting is the assembly door's
    // act, not the product's.
    let product = product_recorded(&doc, &ev, Tol::witness()).expect("gathers");
    assert_eq!(product.contacts.patches.len(), 0);

    let result = assemble(&doc, &ev, Tol::witness());
    let (contacts, findings) = declared_frontier(&result);

    // The record: one, a `PatchContact`, keyed to the faces the
    // references resolved to.
    assert_eq!(
        contacts.patches.len(),
        1,
        "one mate, one minted record: {contacts:?}"
    );
    let record: topo::PatchContact = contacts.patches[0];

    assert_eq!(findings.len(), 1, "and one examined declaration");
    let editor_core::Attribution::Declined(declared) = &findings[0].attribution else {
        panic!("the frontier declines the declaration: {:?}", findings[0])
    };
    assert_eq!(declared.mate, mate, "the declaration names its mate");
    assert_eq!(declared.class, ContactClass::Rest, "with the mate's class");
    assert_eq!(
        (declared.a.clone(), declared.b.clone()),
        (
            in_part(ids[0], CapEnd::Top),
            in_part(ids[1], CapEnd::Bottom)
        ),
        "and both of its references"
    );
    assert_eq!(
        (record.face_a, record.face_b),
        declared.faces,
        "the record is keyed to the faces the references resolved to"
    );
}

/// INVARIANT: minting is DECLARATION, not verification — a NON-TREE
/// (`Declaring`) mate mints identically to the tree mate that placed
/// its child. Roles are assigned per PAIR (a second mate on a tree
/// pair is a co-determiner, not a declarer), so the declaring case is
/// a CYCLE: three instances stacked in a column, mated 0-1, 0-2 and
/// 1-2. The spanning tree from the gauge takes the first two; the
/// third solved nothing, and it still says what touches what.
#[test]
fn row2_b_a_declaring_mate_mints_identically() {
    let mut store = StubStore::default();
    let doc_ref = store.insert(cube_part("asm-r2b-row2b-part"), Tol::witness());
    let mut doc = ProfileDoc::empty(DocumentId::derive("asm-r2b-row2b"), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..3 {
        let (next, id) = insert(doc, Node::instantiate_part(doc_ref));
        doc = next;
        ids.push(id);
    }
    // The column: instance 1 seats on 0 (z ∈ [1,2]), instance 2 on
    // that (z ∈ [2,3]). Both are edges from the gauge, so both are
    // TREE edges.
    let (doc, _) = step(
        doc,
        DocEdit::InsertNode {
            node: rest_mate(ids[0], ids[1], 1.0),
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::InsertNode {
            node: rest_mate(ids[0], ids[2], 2.0),
        },
    );
    // The cycle-closing mate: instance 1's top against instance 2's
    // bottom. Consistent with the poses already solved, and non-tree.
    let (doc, second) = step(
        doc,
        DocEdit::InsertNode {
            node: rest_mate(ids[1], ids[2], 1.0),
        },
    );
    let second = second.expect("the third mate mints");
    let poses = editor_core::solve_document(&doc, Tol::witness());
    assert_eq!(
        poses.role(second),
        Some(editor_core::MateRole::Declaring),
        "the cycle-closing mate solved nothing"
    );

    // The column's two tree mates seat their instances a unit apart
    // from the gauge's top face, so this document's verdict is NOT the
    // frontier — one declaration is genuinely contradicted, and the
    // gate refuses either way. What the row reads out of the refusal
    // is who was ATTRIBUTED, which is the set that got minted.
    let ev = run(&doc, &opts(store));
    let err =
        assemble(&doc, &ev, Tol::witness()).expect_err("the column's declarations do not all hold");
    let AssemblyError::AtRest { findings } = &err else {
        panic!("expected the at-rest verdict, got {err}");
    };
    let mut declared: Vec<RecipeNodeId> = relations(findings).into_iter().map(|(m, _)| m).collect();
    declared.sort();
    declared.dedup();
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
    let (doc, ids, mate, store) = stacked("asm-r2b-row3a", 1.0);
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: mate });
    let ev = run(&doc, &opts(store));
    let result = assemble(&doc, &ev, Tol::witness());
    let errs = findings(&result);
    assert!(
        errs.iter().any(|e| e.contains("UndeclaredContact")),
        "an undeclared cross-instance contact refuses, naming the \
         finding: {errs:?} (instances {ids:?})"
    );
}

/// INVARIANT (the scan-to-bless ban, DECLARED direction): the same
/// geometry WITH the mate's declaration minted is no longer an
/// UNDECLARED contact — the declaration is what suppresses the F1
/// refusal, and nothing else does.
///
/// **And the residual verdict is PINNED, exactly** (review MAJOR-1):
/// by ARM, COUNT and RELATION, rather than by a `.all()` an empty
/// vector would satisfy vacuously. If the census grows a
/// cross-instance chart rung this row goes RED and must be re-blessed
/// deliberately — which is the only way a boundary claim stays honest
/// as the kernel moves.
#[test]
fn row3_b_the_declared_touching_pair_is_not_an_undeclared_contact() {
    let (doc, _, mate, store) = stacked("asm-r2b-row3b", 1.0);
    let ev = run(&doc, &opts(store));
    let result = assemble(&doc, &ev, Tol::witness());
    let errs = findings(&result);
    assert!(
        !errs.iter().any(|e| e.contains("UndeclaredContact")),
        "the minted declaration backs the touching pair: {errs:?}"
    );
    // The pin: exactly one residual finding, of exactly this kind,
    // attributed to exactly this mate. Not "all of them are X" — a
    // vacuous truth over an empty vector is how a weakened row hides.
    // The KIND is asserted through the door's own predicate, so the
    // claim this row makes is the claim the door makes.
    let (_, findings) = declared_frontier(&result);
    assert_eq!(findings.len(), 1, "exactly one residual finding: {errs:?}");
    assert_eq!(
        relations(findings),
        vec![(mate, "declined")],
        "attributed to the mate whose declaration was examined, and \
         DECLINED rather than refuted"
    );
    // And the whole refusal RENDERS as the frontier, so a caller who
    // only has the Display can tell it from a defect of their own.
    let msg = result.as_ref().expect_err("refused").to_string();
    assert!(
        msg.contains("frontier"),
        "the rendering names the frontier: {msg}"
    );
}

/// INVARIANT (spec row 4's in-band clause; C4's escalation rail): a
/// declared Rest whose gap is AUTHORED INTO THE BAND is neither
/// certified nor contradicted — it escalates TYPED, and the escalation
/// carries the kernel's predicate name.
///
/// **This row carries more than its own invariant.** It is the suite's
/// only fixture whose findings are attributed to NO declaration, so it
/// is the sole guard against two structurally distinct widenings of
/// the frontier arm: an `Unattributed` finding being swept into
/// `Uncertified`, and an undeclared contact being swept there with it.
/// Delete it and both open silently.
///
/// The gap is DERIVED FROM THE COMMITTED BAND, never spelled as a
/// literal: the hosted matrix runs ε = 1e-12 and 1e-6 as well as the
/// default, and a hard-coded 3e-9 is definitely-separated in one lane
/// and definitely-zero in another — the row would pin an ε, not an
/// invariant. `Band::linear(Tol::witness())` is the same band the at-rest door
/// builds, and the geometric mean of its two thresholds is strictly
/// between them in every lane, which is exactly what "in band" means.
/// (Row adopted from the review's probe; the band-relative derivation
/// is the repo's own precedent for literal-ε rows.)
#[test]
fn row4_b_an_in_band_authored_gap_escalates_typed_and_predicate_named() {
    let band =
        geom_core::predicate::Band::linear(Tol::witness()).expect("the committed band builds");
    let gap = (band.zero() * band.escalate()).sqrt();
    assert!(
        gap > band.zero() && gap < band.escalate(),
        "the derived gap is strictly inside the band in THIS lane: \
         {gap} vs [{}, {}]",
        band.zero(),
        band.escalate()
    );
    let (doc, _, mate, store) = stacked("asm-r2b-row4b", 1.0 + gap);
    let ev = run(&doc, &opts(store));
    let result = assemble(&doc, &ev, Tol::witness());
    let errs = findings(&result);
    assert!(
        errs.iter().any(|e| e.contains("CensusEscalated")),
        "an in-band declared gap is the Err(Indeterminate) rail, not a \
         verdict either way: {errs:?}"
    );
    assert!(
        errs.iter().any(|e| e.contains("predicate")),
        "and the escalation names the PREDICATE that could not decide: \
         {errs:?}"
    );
    // It is still the assembly's typed refusal, and this row is about
    // the ESCALATION, so the mate need not be attributable: an
    // escalation names a predicate, not a declaration.
    assert!(
        matches!(result, Err(AssemblyError::AtRest { .. })),
        "the escalation surfaces through the assembly gate"
    );
    let _ = mate;
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
    let (doc, ids, mate, store) = stacked("asm-r2b-row4", 2.0);
    let ev = run(&doc, &opts(store));
    let err = assemble(&doc, &ev, Tol::witness()).expect_err("a gapped Rest refuses");
    let AssemblyError::AtRest { findings } = &err else {
        panic!("expected the at-rest verdict, got {err}");
    };
    let named = findings
        .iter()
        .find_map(|f| f.attribution.declaration().cloned())
        .expect("the refusal names its mate");
    assert_eq!(named.mate, mate, "the MATE NODE is named");
    assert_eq!(
        (named.a, named.b),
        (
            in_part(ids[0], CapEnd::Top),
            in_part(ids[1], CapEnd::Bottom)
        ),
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
    // The other side of the split: a REFUTED declaration is a finding
    // against this document, so it lands in the at-rest arm (which
    // this row already destructured) and its relation says refuted —
    // never dressed as a decline.
    assert!(
        relations(findings)
            .iter()
            .any(|&(m, r)| m == mate && r == "refuted"),
        "the gapped declaration is REFUTED, not declined: {msg}"
    );
}

// ---- Rows 5 and 6: the crossing record ----
//
// **A finding, pinned rather than papered over — SCOPED to mate
// EDGES** (review MAJOR-2 corrected an earlier over-claim here).
//
// A4 speaks of "every mate EDGE crossing the cut", and an edge exists
// only when BOTH of a mate's heads are live instances (A12: `head_of`
// requires `Node::InstantiatePart`). For such a PROPER mate edge the
// crossing is unreachable, and that is verified in both directions
// below: the mate joins its two instances into ONE placement cluster
// (A11 rule 2, role-blind), and ASM-R2a's ratified precondition
// refuses any cut that is not a union of WHOLE clusters
// (`SplitError::TornCluster`). Opposite sides of a cut and the same
// cluster are mutually exclusive. A4's sentence and A11's cut rule are
// in tension for proper edges; that gap is now recorded as **AQ8** in
// docs/ASSEMBLY-DESIGN.md, whose proposed resolution is a conversion
// door (ASM-XSPLIT) rather than a change to either rule.
//
// **And ONLY an edge can cross** (AQ8, ruled — option (b), SKIP). The
// review found the collector's predicate (name-derivation sides) was
// wider than A4's edge: a mate with a DANGLING head — one reference
// naming non-instance geometry, or a node id not in the document —
// contributes no cluster edge, leaves its instance a singleton, and
// so slipped a populated record through a cut the precondition
// accepts. The ruling closes that: such a mate never solved, so a
// record minted from it would be trusted-at-rest state, which AQ8's
// ratification condition forbids. The collector now gates on both
// heads being live instances, and `row5_d` asserts the SKIP as an
// invariant.

/// INVARIANT (the A4-vs-A11 tension for a PROPER MATE EDGE,
/// executable): cutting ONE instance of a mated pair refuses
/// `TornCluster` — which is exactly why a mate EDGE cannot cross a cut
/// — and the whole-cluster cut that IS accepted carries both of the
/// mate's ends, so its record is empty. Scoped to edges: the
/// non-edge case is `row5_d`'s subject.
#[test]
fn row5_a_a_proper_mate_edge_cannot_cross_a_cut_and_split_says_so_both_ways() {
    let (doc, ids, _, store) = stacked("asm-r2b-row5", 1.0);

    // One instance alone: the cut tears the cluster.
    let torn = split(
        &doc,
        &[ids[1]].into_iter().collect(),
        DocumentId::derive("asm-r2b-row5-torn"),
        Tol::witness(),
    )
    .expect_err("a torn cluster refuses");
    assert!(
        matches!(torn, editor_core::SplitError::TornCluster { .. }),
        "the ratified whole-cluster precondition is what makes a \
         mate EDGE's crossing unreachable: {torn:?}"
    );

    // The whole cluster: accepted, and nothing crosses.
    let out = split(
        &doc,
        &ids.iter().copied().collect(),
        DocumentId::derive("asm-r2b-row5-whole"),
        Tol::witness(),
    )
    .expect("a whole-cluster cut splits");
    let Some(Node::InstantiatePart { interface, .. }) = out.remainder.node(out.instance) else {
        panic!("the split minted an instance");
    };
    assert!(
        interface.is_empty(),
        "both ends of the EDGE moved together, so no declaration \
         crossed: {:?}",
        interface.crossings
    );
    let _ = store;
}

/// INVARIANT (A4's "does it actually fit" + A13 clause 4): an
/// instance carrying a crossing declaration RE-VERIFIES it against
/// the pinned part at every evaluation. Pre-move it resolves and the
/// instance evaluates; a pin move to a part that no longer names the
/// declared entity refuses TYPED, naming the crossing — the swap is
/// never accepted with the declaration quietly dropped.
#[test]
fn row5_b_a_pin_move_that_breaks_a_crossing_refuses_at_evaluation() {
    let part_id = DocumentId::derive("asm-r2b-row5b-part");
    let mut store = StubStore::default();
    let doc_ref = store.insert(
        {
            let (d, _) = block(
                ProfileDoc::empty(part_id, Tol::witness()),
                (0.0, 1.0),
                (0.0, 1.0),
                0.0,
                1.0,
            );
            d
        },
        Tol::witness(),
    );
    // The part-local name of the cube's top cap: profile is node 0,
    // extrude node 1.
    let inner = StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId(1),
        path: vec![RoleSeg::Cap(CapEnd::Top)],
    };
    let record = editor_core::InterfaceRecord {
        crossings: vec![InterfaceCrossing::Mate {
            mate: RecipeNodeId(7),
            class: ContactClass::Rest,
            outer: inner.clone(),
            inner: inner.clone(),
        }],
    };
    let (doc, instance) = insert(
        ProfileDoc::empty(DocumentId::derive("asm-r2b-row5b"), Tol::witness()),
        Node::instantiate_part_with(doc_ref, record),
    );

    let ev = run(&doc, &opts(store.clone()));
    assert!(
        ev.node_error(instance).is_none(),
        "the crossing re-verifies against the pinned part: {:?}",
        ev.node_error(instance).map(ToString::to_string)
    );

    // The move: the SAME document id, re-modelled so the declared
    // entity's minting node is no longer the one the crossing names.
    // A leading datum shifts the extrude off node 1 — the part still
    // has a product, and the crossing's reference is simply gone.
    let (shifted, _) = insert(
        ProfileDoc::empty(part_id, Tol::witness()),
        Node::Datum(editor_core::Datum::Point {
            position: [len(0.0), len(0.0), len(0.0)],
        }),
    );
    let (shifted, _) = block(shifted, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let new_pin = content_pin(&shifted, Tol::witness()).expect("the pin computes");
    store.docs.insert(part_id, shifted);
    let moved = editor_core::apply(
        &doc,
        &DocEdit::UpdateReference {
            node: instance,
            new_pin,
        },
        Tol::witness(),
    )
    .expect("the pin moves")
    .doc;

    let err = run(&moved, &opts(store))
        .node_error(instance)
        .expect("the moved pin refuses")
        .to_string();
    assert!(
        err.contains("does not re-verify"),
        "the refusal names the crossing that no longer fits: {err}"
    );
}

/// INVARIANT: inline CONSUMES the record — every crossing's part-side
/// reference must re-anchor onto a spliced local name — and then the
/// record dissolves with the instance it rode on.
#[test]
fn row5_c_inline_dissolves_the_crossing_record() {
    let part_id = DocumentId::derive("asm-r2b-row5c-part");
    let mut store = StubStore::default();
    let doc_ref = store.insert(
        {
            let (d, _) = block(
                ProfileDoc::empty(part_id, Tol::witness()),
                (0.0, 1.0),
                (0.0, 1.0),
                0.0,
                1.0,
            );
            d
        },
        Tol::witness(),
    );
    let inner = StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId(1),
        path: vec![RoleSeg::Cap(CapEnd::Top)],
    };
    let record = editor_core::InterfaceRecord {
        crossings: vec![InterfaceCrossing::Mate {
            mate: RecipeNodeId(9),
            class: ContactClass::Rest,
            outer: inner.clone(),
            inner,
        }],
    };
    let (doc, instance) = insert(
        ProfileDoc::empty(DocumentId::derive("asm-r2b-row5c"), Tol::witness()),
        Node::instantiate_part_with(doc_ref, record),
    );
    let back = inline(&doc, instance, &store, Tol::witness()).expect("inline succeeds");
    assert!(
        back.doc.node(instance).is_none(),
        "the instance is gone, and its record with it"
    );
    assert!(
        !back.doc.order().is_empty(),
        "the part's recipe is spliced in"
    );
}

/// INVARIANT (**AQ8, ruled — option (b), SKIP**): only a mate EDGE can
/// cross a cut, so a mate with a DANGLING head contributes NO crossing
/// record however its names fall across the cut.
///
/// A12's reading edge exists only when BOTH heads are live instances
/// (`head_of` requires an instantiate node). A mate naming
/// non-instance geometry is therefore not an edge: it joins no
/// placement cluster, its instance stays a singleton, and a cut of
/// that instance alone IS a whole-cluster cut the precondition
/// accepts — but the record it produces is empty. The ruling's reason
/// is the load-bearing one: such a mate never solved, so a record
/// minted from it would be trusted-at-rest state, which AQ8's
/// ratification condition forbids. A4's letter says the same thing
/// from the other side — no edge, no crossing.
///
/// The mate itself survives in the document (N5) and its names rebind
/// like any other reference to cut material; it simply says nothing
/// about the seam.
#[test]
fn row5_d_a_dangling_head_mate_contributes_no_crossing() {
    let mut store = StubStore::default();
    let doc_ref = store.insert(cube_part("asm-r2b-row5d-part"), Tol::witness());
    let (doc, instance) = insert(
        ProfileDoc::empty(DocumentId::derive("asm-r2b-row5d"), Tol::witness()),
        Node::instantiate_part(doc_ref),
    );
    // Local geometry in the SAME document — not an instance, so the
    // mate's `b` head is dangling in A12's sense.
    let (doc, local) = block(doc, (0.0, 1.0), (0.0, 1.0), 5.0, 1.0);
    let mut node = rest_mate(instance, instance, 1.0);
    if let Node::Mate { b, .. } = &mut node {
        *b = StableName {
            kind: EntityKind::Face,
            node: local,
            path: vec![RoleSeg::Cap(CapEnd::Bottom)],
        };
    }
    let (doc, mate) = step(doc, DocEdit::InsertNode { node });
    let mate = mate.expect("the mate mints");

    // The instance is a singleton cluster (no reading edge), so a cut
    // of it alone is whole-cluster and the precondition accepts.
    let out = split(
        &doc,
        &[instance].into_iter().collect(),
        DocumentId::derive("asm-r2b-row5d-split"),
        Tol::witness(),
    )
    .expect("a singleton-cluster cut splits");
    let Some(Node::InstantiatePart { interface, .. }) = out.remainder.node(out.instance) else {
        panic!("the split minted an instance");
    };
    assert!(
        interface.is_empty(),
        "AQ8 (b): a dangling-head mate is not an EDGE, so it crosses \
         nothing — a never-solved declaration's record would be \
         trusted-at-rest state: {:?}",
        interface.crossings
    );
    // And the mate is still there, saying what it said — the skip is
    // about the RECORD, not about deleting the declaration (N5).
    assert!(
        matches!(out.remainder.node(mate), Some(Node::Mate { .. })),
        "the mate survives the split; only its crossing does not exist"
    );
}

/// INVARIANT (spec row 5's "a pin update that CHANGES the part's
/// contact face", the geometric half; review MINOR-2): a pin move that
/// leaves the recipe's node layout alone but MOVES the contact face is
/// invisible to the instance's own structural re-verification — the
/// name still resolves — and is caught by the AT-REST DOOR, which is
/// where a geometric question belongs.
///
/// Both halves stated together so the boundary is not a gap: `row5_b`
/// pins the structural half at the wire, and this row pins the
/// geometric half at `assemble`. An in-document mate is what makes the
/// second reachable; a crossing declaration on a WIRE record has no
/// geometric gate today, which is the honest limit of D-5 and is
/// stated in `wire_instantiate_part`'s own docs.
#[test]
fn row5_e_a_pin_move_that_changes_the_contact_geometry_is_caught_at_rest() {
    let part_id = DocumentId::derive("asm-r2b-row5e-part");
    let mut store = StubStore::default();
    let doc_ref = store.insert(
        {
            let (d, _) = block(
                ProfileDoc::empty(part_id, Tol::witness()),
                (0.0, 1.0),
                (0.0, 1.0),
                0.0,
                1.0,
            );
            d
        },
        Tol::witness(),
    );
    let mut doc = ProfileDoc::empty(DocumentId::derive("asm-r2b-row5e"), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..2 {
        let (next, id) = insert(doc, Node::instantiate_part(doc_ref));
        doc = next;
        ids.push(id);
    }
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: rest_mate(ids[0], ids[1], 1.0),
        },
    );
    let mate = mate.expect("the mate mints");

    // Pre-move: the declared pair touches, so the ONLY finding is the
    // chart-identity boundary (row3_b's pin) — no contradiction.
    let ev = run(&doc, &opts(store.clone()));
    let before = findings(&assemble(&doc, &ev, Tol::witness()));
    assert!(
        !before.iter().any(|e| e.contains("ContactContradicted")),
        "pre-move the declaration is not contradicted: {before:?}"
    );

    // The move: SAME node layout, different geometry — the cube is
    // half as tall, so its top cap is at z = 0.5 while the mate still
    // seats the second instance's bottom at z = 1.
    let (shorter, _) = block(
        ProfileDoc::empty(part_id, Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        0.0,
        0.5,
    );
    let new_pin = content_pin(&shorter, Tol::witness()).expect("the pin computes");
    store.docs.insert(part_id, shorter);
    let moved = editor_core::apply(
        &doc,
        &DocEdit::UpdateReference {
            node: ids[0],
            new_pin,
        },
        Tol::witness(),
    )
    .expect("the pin moves")
    .doc;
    let moved = editor_core::apply(
        &moved,
        &DocEdit::UpdateReference {
            node: ids[1],
            new_pin,
        },
        Tol::witness(),
    )
    .expect("the pin moves")
    .doc;

    let ev2 = run(&moved, &opts(store));
    let err =
        assemble(&moved, &ev2, Tol::witness()).expect_err("the moved geometry no longer fits");
    let AssemblyError::AtRest { findings } = &err else {
        panic!("expected the at-rest verdict, got {err}");
    };
    assert!(
        findings
            .iter()
            .any(|f| f.attribution.declaration().is_some_and(|m| m.mate == mate)),
        "the at-rest door catches the geometry change and NAMES the \
         mate whose declaration it broke: {findings:?}"
    );
}

// ---- Row 6: the content key feeds on the inhabited record ----

/// INVARIANT (the content-key half of ASM-4's hook obligation): a
/// crossing-record edit MOVES the instantiate node's content key. Two
/// documents differing ONLY in the record must not share a memo
/// entry — otherwise a crossing edit would be served the pre-edit
/// answer, re-verification and all.
#[test]
fn row6_a_crossing_record_edit_moves_the_content_key() {
    let part_id = DocumentId::derive("asm-r2b-row6-part");
    let mut store = StubStore::default();
    let doc_ref = store.insert(
        {
            let (d, _) = block(
                ProfileDoc::empty(part_id, Tol::witness()),
                (0.0, 1.0),
                (0.0, 1.0),
                0.0,
                1.0,
            );
            d
        },
        Tol::witness(),
    );
    let inner = StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId(1),
        path: vec![RoleSeg::Cap(CapEnd::Top)],
    };
    let record = editor_core::InterfaceRecord {
        crossings: vec![InterfaceCrossing::Mate {
            mate: RecipeNodeId(4),
            class: ContactClass::Rest,
            outer: inner.clone(),
            inner,
        }],
    };
    let (with, id_with) = insert(
        ProfileDoc::empty(DocumentId::derive("asm-r2b-row6"), Tol::witness()),
        Node::instantiate_part_with(doc_ref, record),
    );
    let (without, id_without) = insert(
        ProfileDoc::empty(DocumentId::derive("asm-r2b-row6"), Tol::witness()),
        Node::instantiate_part(doc_ref),
    );
    assert_eq!(id_with, id_without, "same id, same reference, same pin");

    let key = |d: &ProfileDoc, id| {
        run(d, &opts(store.clone()))
            .value(id)
            .expect("the instance evaluates")
            .content_key
    };
    assert_ne!(
        key(&with, id_with),
        key(&without, id_without),
        "the inhabited record feeds the key"
    );
}

// ---- Row 7: D9 determinism ----

/// INVARIANT (D9): a mated, minted document's evaluation is a pure
/// function of the document — two independent evaluations produce the
/// bit-identical record set and the same at-rest verdict.
#[test]
fn row7_the_minted_record_set_is_deterministic() {
    let (doc, _, _, store) = stacked("asm-r2b-row7", 1.0);
    let a = run(&doc, &opts(store.clone()));
    let b = run(&doc, &opts(store));
    let pa = product_recorded(&doc, &a, Tol::witness()).expect("gathers");
    let pb = product_recorded(&doc, &b, Tol::witness()).expect("gathers");
    assert_eq!(pa.contacts, pb.contacts, "the carried records replay");
    let ra = assemble(&doc, &a, Tol::witness());
    let rb = assemble(&doc, &b, Tol::witness());
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

/// INVARIANT: the two doors admit DIFFERENT class sets, and
/// `class_admission` is the one statement of both. `Tangent` solves —
/// it folds a coset and places its instance — and then refuses TYPED
/// at the mint door, because no kernel record carries a tangency at
/// rest and one is never minted with an invented witness.
///
/// The row asserts the table and the two doors TOGETHER: teach either
/// door a class the table does not give it and this goes red, which
/// is what stops a door advertising what it cannot execute.
#[test]
fn a_tangent_mate_solves_and_then_refuses_at_the_mint_door() {
    assert_eq!(
        editor_core::class_admission(ContactClass::Rest),
        editor_core::ClassAdmission::Mints,
        "Rest clears both doors"
    );
    let tangent_admission = editor_core::class_admission(ContactClass::Tangent);
    let editor_core::ClassAdmission::NoAtRestRecord { why } = tangent_admission else {
        panic!("Tangent clears the solve door only: {tangent_admission:?}")
    };

    let (doc, ids, _, store) = stacked("asm-r2b-tangent", 1.0);
    let mut node = rest_mate(ids[0], ids[1], 1.0);
    if let Node::Mate { class, .. } = &mut node {
        *class = ContactClass::Tangent;
    }
    let (doc, tangent) = step(doc, DocEdit::InsertNode { node });
    let tangent = tangent.expect("the tangent mate mints");

    // Door one: the solve admits it — no fault, and it took a role in
    // the pair. (A class the table DEFERS refuses here instead.)
    let poses = editor_core::solve_document(&doc, Tol::witness());
    assert!(
        poses.fault(tangent).is_none(),
        "the solve door admits Tangent: {:?}",
        poses.fault(tangent)
    );
    assert!(
        poses.role(tangent).is_some(),
        "and folds it into the pair like any admitted class"
    );

    // Door two: the mint refuses it, naming the class.
    let ev = run(&doc, &opts(store));
    match assemble(&doc, &ev, Tol::witness()) {
        Err(AssemblyError::NoAtRestRecord {
            class,
            mate,
            why: rendered,
        }) => {
            assert_eq!(class, ContactClass::Tangent);
            assert_eq!(mate, tangent, "naming the mate that declared it");
            assert_eq!(
                rendered, why,
                "and giving the TABLE's reason for this class, never one \
                 borrowed from another"
            );
        }
        other => panic!("a Tangent mate must refuse at the mint door: {other:?}"),
    }
}

/// INVARIANT (**the split, in the case that distinguishes the two
/// arms**): ONE refuted declaration makes the whole refusal a finding
/// against the document, however many declines ride with it.
///
/// The document mixes both in one gate run: instance 1 seats ON
/// instance 0 (a touching pair, which the certifier DECLINES), and
/// instance 2 is declared against instance 0's top from a unit away
/// (a pair the kernel REFUTES). Nothing else in the suite mixes them,
/// and without this row `Uncertified`'s "and nothing else" is
/// unfalsifiable — an `any` in place of the `all` would pass every
/// other row.
#[test]
fn a_mixed_verdict_is_the_at_rest_arm_not_the_frontier() {
    let mut store = StubStore::default();
    let doc_ref = store.insert(cube_part("asm-r2b-mixed-part"), Tol::witness());
    let mut doc = ProfileDoc::empty(DocumentId::derive("asm-r2b-mixed"), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..3 {
        let (next, id) = insert(doc, Node::instantiate_part(doc_ref));
        doc = next;
        ids.push(id);
    }
    // Touching: instance 1's bottom on instance 0's top (z = 1).
    let (doc, touching) = step(
        doc,
        DocEdit::InsertNode {
            node: rest_mate(ids[0], ids[1], 1.0),
        },
    );
    // Gapped: instance 2 seats at z = 3, and the mate declares its
    // bottom against instance 0's top at z = 1 — two units apart.
    let (doc, gapped) = step(
        doc,
        DocEdit::InsertNode {
            node: rest_mate(ids[0], ids[2], 3.0),
        },
    );
    let touching = touching.expect("the touching mate mints");
    let gapped = gapped.expect("the gapped mate mints");

    let ev = run(&doc, &opts(store));
    let err =
        assemble(&doc, &ev, Tol::witness()).expect_err("the gapped declaration does not hold");
    let AssemblyError::AtRest { findings } = &err else {
        panic!(
            "a refuted declaration is a finding against the DOCUMENT — it \
             is never the frontier, whatever else the run declined: {err}"
        );
    };
    let rows = relations(findings);
    assert!(
        rows.contains(&(gapped, "refuted")),
        "the gapped declaration is refuted: {rows:?}"
    );
    assert!(
        rows.contains(&(touching, "declined")),
        "and the touching one is declined in the SAME run, which is \
         what makes this the mixed case: {rows:?}"
    );
}

/// INVARIANT: the mint door renders the reason the TABLE gives for the
/// class in front of it — never a sentence of its own, and never
/// another class's.
///
/// **Self-arming, deliberately.** With one non-minting class in the
/// tree this row cannot distinguish "sourced from the table" from "a
/// literal copy of that class's sentence", and no row can: the two are
/// observationally identical until a SECOND class enters the state.
/// The row is therefore written as a loop over the roster's
/// non-minting classes, each asserted against its own reason and the
/// reasons asserted distinct — so the day `Fit` lands with a reason of
/// its own, a hard-coded copy fails here without anyone remembering to
/// come back.
#[test]
fn the_mint_door_renders_each_class_its_own_reason() {
    let mut seen: Vec<&'static str> = Vec::new();
    for class in [ContactClass::Rest, ContactClass::Tangent] {
        let editor_core::ClassAdmission::NoAtRestRecord { why } =
            editor_core::class_admission(class)
        else {
            continue;
        };
        let (doc, ids, _, store) = stacked("asm-r2b-reason", 1.0);
        let mut node = rest_mate(ids[0], ids[1], 1.0);
        if let Node::Mate { class: c, .. } = &mut node {
            *c = class;
        }
        let (doc, mate) = step(doc, DocEdit::InsertNode { node });
        let mate = mate.expect("the mate mints");
        let ev = run(&doc, &opts(store));
        match assemble(&doc, &ev, Tol::witness()) {
            Err(AssemblyError::NoAtRestRecord {
                class: refused,
                mate: named,
                why: rendered,
            }) => {
                assert_eq!(refused, class);
                assert_eq!(named, mate);
                assert_eq!(
                    rendered, why,
                    "the door renders THIS class's reason: {class:?}"
                );
            }
            other => panic!("{class:?} must refuse at the mint door: {other:?}"),
        }
        assert!(
            !seen.contains(&why),
            "each non-minting class states its own reason, or the door \
             cannot be telling them apart: {class:?}"
        );
        seen.push(why);
    }
    assert!(!seen.is_empty(), "the roster has a non-minting class");
}

/// INVARIANT (**the two tables agree, where it is observable**): a
/// class [`editor_core::class_admission`] does not defer must have a
/// wire spelling, or a document carrying such a mate would build and
/// then be unsavable.
///
/// The two tables answer different questions — how far a class gets,
/// and how it is spelled on the wire — and live in different layers,
/// so they are not merged. What binds them is this row: authoring a
/// mate of each admitted class and round-tripping the document.
///
/// Blind spot, stated: the roster below is written out, because
/// `ContactClass` is `#[non_exhaustive]` and offers no iterator. A
/// class the kernel grows is invisible to this row until it is added
/// here — the same edit that admits it in the table.
#[test]
fn every_admitted_class_has_a_wire_spelling() {
    for class in [ContactClass::Rest, ContactClass::Tangent] {
        assert_ne!(
            editor_core::class_admission(class),
            editor_core::ClassAdmission::NotAdmitted,
            "the roster is the admitted set: {class:?}"
        );
        let mut store = StubStore::default();
        let doc_ref = store.insert(cube_part("asm-r2b-wire-part"), Tol::witness());
        let mut doc = ProfileDoc::empty(DocumentId::derive("asm-r2b-wire"), Tol::witness());
        let mut ids = Vec::new();
        for _ in 0..2 {
            let (next, id) = insert(doc, Node::instantiate_part(doc_ref));
            doc = next;
            ids.push(id);
        }
        let mut node = rest_mate(ids[0], ids[1], 1.0);
        if let Node::Mate { class: c, .. } = &mut node {
            *c = class;
        }
        let (doc, _) = step(doc, DocEdit::InsertNode { node });
        let text =
            editor_core::save(&doc, &[], Tol::witness()).expect("an admitted class is savable");
        let back = editor_core::load(&text, Tol::witness()).expect("and loads back");
        assert_eq!(
            back.doc.order().len(),
            doc.order().len(),
            "the round trip keeps the mate: {class:?}"
        );
    }
}

/// INVARIANT: a mate reference that names no face of the product
/// refuses typed — never resolved by picking, never widened.
#[test]
fn a_mate_reference_that_names_nothing_refuses_typed() {
    let (doc, ids, _, store) = stacked("asm-r2b-vanish", 1.0);
    let mut node = rest_mate(ids[0], ids[1], 1.0);
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
    match assemble(&doc, &ev, Tol::witness()) {
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
