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
//! - **The channel is total, and keyed.** Every declaration an inner
//!   document minted has exactly one row in the outer product, and
//!   that row's faces are EXACTLY what its own route-wrapped
//!   references resolve to in the product's name table. A row from
//!   deeper keeps its route.
//! - **The outermost gate reads both.** A refuted carried declaration
//!   names its mate, its document and the instances it arrived
//!   through; a declined one reaches the frontier arm under the same
//!   name; an inner mate that could not be minted refuses the outer
//!   gate before anything else this document has to say.
//!
//! **The store/eval scaffolding below (`StubStore`, `opts`, `run`,
//! `block_part`, `in_part`, `mate_node`) is a fifth copy** of the
//! shape the four mate suites carry (`mate6_gather_mints.rs`,
//! `mate6r1_shared.rs`, `mate6r2_probes.rs`, `mate1_member_vocab.rs`),
//! disclosed rather than shared: each copy pins its own fixture
//! geometry, and the suites' `in_part` spellings have already diverged
//! once by a node index. Sharing them is a test-support unit of its
//! own, not this one's.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Alignment, Assembly, AssemblyError, Attribution, AxisSense, CancelToken, CapEnd, ContactClass,
    DocEdit, DocRef, DocumentId, EntityKey, EntityKind, Entry, EvalOptions, Evaluation, Frame,
    MateFrame, MatePrimitive, Node, ProfileDoc, RecipeNodeId, Relation, ResolveFailure,
    ResolveFault, RoleSeg, SitedRef, StableName, assemble, content_pin, evaluate, product_recorded,
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

/// A block part: `w` × `d` footprint, extruded `h`. Three nodes — the
/// sketch frame, the profile, the extrude — so the body's names carry
/// node 2.
fn block_part(label: &str, w: f64, d: f64, h: f64) -> ProfileDoc {
    let (doc, profile) = on_frame(
        ProfileDoc::empty(DocumentId::derive(label), Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (w, 0.0), (w, d), (0.0, d)]],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(h),
        },
    );
    doc
}

fn cube_part(label: &str) -> ProfileDoc {
    block_part(label, 1.0, 1.0, 1.0)
}

const PART_BODY: RecipeNodeId = RecipeNodeId(2);

/// `inner` seen through the instance `node` placed it at — one rung of
/// the route a name climbs at each seam.
fn wrap(node: RecipeNodeId, inner: StableName) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::InPart {
            of: Box::new(inner),
        }],
    }
}

/// A face of `instance`'s part product, named through the instance
/// qualifier (A12's reading-edge head).
fn in_part(instance: RecipeNodeId, cap: CapEnd) -> StableName {
    wrap(
        instance,
        StableName {
            kind: EntityKind::Face,
            node: PART_BODY,
            path: vec![RoleSeg::Cap(cap)],
        },
    )
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
    a_frame: MateFrame,
) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a: SitedRef::at_mint(a),
        b: SitedRef::at_mint(b),
        class,
        alignment: Alignment {
            a: a_frame,
            b: frame([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Aligned,
            clocking: None,
        },
    }
}

fn place(doc: ProfileDoc, node: RecipeNodeId, at: [f64; 3]) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetPlacement {
            node,
            frame: Frame::translation(at),
        },
    )
    .0
}

/// Two instances of `part`, stacked by ONE mate whose a-frame is
/// `(seat, axis)`. `seat` `[0,0,1]` is the true declaration; `[0,0,0.5]`
/// penetrates and `[0,0,1.5]` gaps, both counter-evidence; `[1,0,1]` is
/// the GRAZING seat — one carrier, no shared area, decidable in neither
/// direction, so the census DECLINES the pair. A tilted `axis` is an
/// ANGULAR contradiction.
fn stand(
    store: &mut StubStore,
    label: &str,
    class: ContactClass,
    seat: [f64; 3],
    axis: [f64; 3],
) -> (DocRef, DocumentId, Vec<RecipeNodeId>, RecipeNodeId) {
    let cube = store.insert(cube_part(&format!("{label}-cube")), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, c0) = insert(doc, Node::instantiate_part(cube));
    let (doc, c1) = insert(doc, Node::instantiate_part(cube));
    let (doc, mate) = insert(
        doc,
        mate_node(
            in_part(c0, CapEnd::End),
            in_part(c1, CapEnd::Start),
            class,
            frame(seat, axis),
        ),
    );
    let id = doc.id();
    (store.insert(doc, Tol::witness()), id, vec![c0, c1], mate)
}

/// A resting stand: the ordinary carried declaration.
fn resting(
    store: &mut StubStore,
    label: &str,
) -> (DocRef, DocumentId, Vec<RecipeNodeId>, RecipeNodeId) {
    stand(
        store,
        label,
        ContactClass::Rest,
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    )
}

/// A part whose only mate is a `Tangent` — a class that solves and
/// mints NO record at rest, so the part refuses its own gate.
fn broken_part(store: &mut StubStore, label: &str) -> (DocRef, DocumentId, RecipeNodeId) {
    let (r, id, _, mate) = stand(
        store,
        label,
        ContactClass::Tangent,
        [0.0, 0.0, 5.0],
        [0.0, 0.0, 1.0],
    );
    (r, id, mate)
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
            doc = place(doc, id, [dx, 0.0, 0.0]);
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

/// The face a name resolves to in `gathered`'s table, or a panic
/// naming the miss.
fn face_named(gathered: &editor_core::Product<f64>, name: &StableName) -> topo::FaceKey {
    match gathered.names.lookup(name) {
        Some(Entry::Unique(ent)) => match ent.key {
            EntityKey::Face(f) => f,
            other => panic!("{name:?} names {other:?}, not a face"),
        },
        other => panic!("{name:?} resolves to {other:?} in the product"),
    }
}

/// **The A1 oracle**: each carried row's faces are EXACTLY the faces
/// its own route-wrapped references name in the product's table.
///
/// The row's route says how to wrap its inner reference — one
/// `InPart` rung per instance, outermost last — and the name table is
/// the product's own independent answer to "which face is that". So a
/// row keyed to some other recorded pair, or to the right pair in the
/// wrong order, or carrying a route that does not lead to its own
/// faces, fails here. A weaker oracle ("the pair is SOME carried
/// record's pair") passes all three.
fn assert_rows_match_names(gathered: &editor_core::Product<f64>) {
    for row in &gathered.carried {
        let route = |inner: &StableName| {
            let mut name = inner.clone();
            for &node in row.route.via.iter().rev() {
                name = wrap(node, name);
            }
            wrap(row.route.through, name)
        };
        let want = (
            face_named(gathered, &route(&row.declaration.a)),
            face_named(gathered, &route(&row.declaration.b)),
        );
        assert_eq!(
            row.declaration.faces, want,
            "row {row:?} is not keyed to the faces its own references name"
        );
    }
}

/// One row's identity as the acceptance rows compare it.
fn rows(
    gathered: &editor_core::Product<f64>,
) -> Vec<(RecipeNodeId, DocumentId, Vec<RecipeNodeId>, RecipeNodeId)> {
    gathered
        .carried
        .iter()
        .map(|c| {
            (
                c.route.through,
                c.route.of,
                c.route.via.clone(),
                c.declaration.mate,
            )
        })
        .collect()
}

// ---- The channel is total, and keyed ----

/// A1: one carried row per inner declaration, keyed onto the outer
/// product by the graft's own map, and the name table is the oracle.
#[test]
fn every_carried_row_is_keyed_to_what_its_own_names_resolve_to() {
    let mut store = StubStore::default();
    let (inner_ref, inner_id, _, inner_mate) = resting(&mut store, "docm6-a1-stand");
    let (outer, instances) = row_of("docm6-a1-row", inner_ref, 3, 4.0);

    let ev = run(&outer, &opts(store));
    let gathered = product_recorded(&outer, &ev, Tol::witness()).expect("the row gathers");

    assert_eq!(
        rows(&gathered),
        instances
            .iter()
            .map(|&i| (i, inner_id, Vec::new(), inner_mate))
            .collect::<Vec<_>>(),
    );
    // This document minted nothing of its own, and carried no refusal.
    assert!(gathered.minted.is_empty());
    assert!(gathered.unminted.is_empty());
    assert!(gathered.carried_unminted.is_empty());
    assert_rows_match_names(&gathered);
    // Three instances of ONE part re-key onto three DIFFERENT face
    // pairs: the rows share an `Arc` at the seam and must not share a
    // key after the graft.
    let pairs: Vec<_> = gathered
        .carried
        .iter()
        .map(|c| c.declaration.faces)
        .collect();
    for (i, a) in pairs.iter().enumerate() {
        for b in &pairs[i + 1..] {
            assert_ne!(a, b, "two instances re-key onto DIFFERENT aggregate faces");
        }
    }
}

/// A1, four instantiation levels and a tree the row did not simplify:
/// two different parts, a sub-assembly instantiated twice, an instance
/// repeated, and a mid-level mate of the mid's own.
///
/// `of` names the MINTING document — never one the row merely passed
/// through — and `via` is the path below `through`, nearest first.
#[test]
fn a_four_level_assembly_carries_every_row_with_its_route() {
    let mut store = StubStore::default();

    // S1: a cube resting on a slab.
    let slab = store.insert(block_part("docm6-deep-slab", 2.0, 2.0, 0.5), Tol::witness());
    let cube = store.insert(block_part("docm6-deep-cube", 1.0, 1.0, 1.0), Tol::witness());
    let s1 = ProfileDoc::empty(DocumentId::derive("docm6-deep-s1"), Tol::witness());
    let (s1, s1_slab) = insert(s1, Node::instantiate_part(slab));
    let (s1, s1_cube) = insert(s1, Node::instantiate_part(cube));
    let (s1, s1_mate) = insert(
        s1,
        mate_node(
            in_part(s1_slab, CapEnd::End),
            in_part(s1_cube, CapEnd::Start),
            ContactClass::Rest,
            frame([0.0, 0.0, 0.5], [0.0, 0.0, 1.0]),
        ),
    );
    let s1_id = s1.id();
    let s1_ref = store.insert(s1, Tol::witness());

    // The MID: S1 twice, a bare cube, and the mid's OWN mate resting a
    // second cube on the first.
    let mid = ProfileDoc::empty(DocumentId::derive("docm6-deep-mid"), Tol::witness());
    let (mid, m_s1a) = insert(mid, Node::instantiate_part(s1_ref));
    let (mid, m_s1b) = insert(mid, Node::instantiate_part(s1_ref));
    let mid = place(mid, m_s1b, [10.0, 0.0, 0.0]);
    let (mid, m_c0) = insert(mid, Node::instantiate_part(cube));
    let mid = place(mid, m_c0, [30.0, 0.0, 0.0]);
    let (mid, m_c1) = insert(mid, Node::instantiate_part(cube));
    let (mid, mid_mate) = insert(
        mid,
        mate_node(
            in_part(m_c0, CapEnd::End),
            in_part(m_c1, CapEnd::Start),
            ContactClass::Rest,
            frame([0.0, 0.0, 1.0], [0.0, 0.0, 1.0]),
        ),
    );
    let mid_id = mid.id();
    let mid_ref = store.insert(mid, Tol::witness());

    // The OUTER: a bare cube first, so the mid instances' ids do not
    // coincide with the mid's own sub-instance ids, then the mid twice.
    let outer = ProfileDoc::empty(DocumentId::derive("docm6-deep-outer"), Tol::witness());
    let (outer, o_cube) = insert(outer, Node::instantiate_part(cube));
    let outer = place(outer, o_cube, [0.0, -50.0, 0.0]);
    let (outer, o_m1) = insert(outer, Node::instantiate_part(mid_ref));
    let (outer, o_m2) = insert(outer, Node::instantiate_part(mid_ref));
    let outer = place(outer, o_m2, [0.0, 100.0, 0.0]);
    let outer_id = outer.id();

    let ev = run(&outer, &opts(store.clone()));
    let gathered = product_recorded(&outer, &ev, Tol::witness()).expect("the outer gathers");
    let per_mid = |om: RecipeNodeId| {
        vec![
            (om, mid_id, vec![], mid_mate),
            (om, s1_id, vec![m_s1a], s1_mate),
            (om, s1_id, vec![m_s1b], s1_mate),
        ]
    };
    let mut expected = per_mid(o_m1);
    expected.extend(per_mid(o_m2));
    assert_eq!(rows(&gathered), expected);
    assert!(
        gathered.carried.iter().all(|c| c.route.of != outer_id),
        "no row is `of` the document that merely passed it"
    );
    assert_rows_match_names(&gathered);
    assert!(assemble(&outer, &ev, Tol::witness()).is_ok());

    // Level four: the outer instantiated once more. Every `via` gains
    // a second entry, nearest first, and `of` does not move.
    let outer_ref = store.insert(outer, Tol::witness());
    let top = ProfileDoc::empty(DocumentId::derive("docm6-deep-top"), Tol::witness());
    let (top, t) = insert(top, Node::instantiate_part(outer_ref));
    let ev = run(&top, &opts(store));
    let gathered = product_recorded(&top, &ev, Tol::witness()).expect("the top gathers");
    let mut expected = Vec::new();
    for om in [o_m1, o_m2] {
        expected.push((t, mid_id, vec![om], mid_mate));
        expected.push((t, s1_id, vec![om, m_s1a], s1_mate));
        expected.push((t, s1_id, vec![om, m_s1b], s1_mate));
    }
    assert_eq!(rows(&gathered), expected);
    assert_rows_match_names(&gathered);
    assert!(assemble(&top, &ev, Tol::witness()).is_ok());
}

// ---- The outermost gate reads what was carried ----

/// A2: a part whose declared contact its placement contradicts. The
/// outer census refutes the carried declaration — and the finding names
/// the mate, the document and the instance, where before it named
/// nobody.
#[test]
fn a_refuted_carried_declaration_names_its_mate_and_route() {
    let mut store = StubStore::default();
    // Seat 0.5: the two cubes interpenetrate, so the declared rest is
    // definite counter-evidence.
    let (inner_ref, inner_id, _, inner_mate) = stand(
        &mut store,
        "docm6-a2-stand",
        ContactClass::Rest,
        [0.0, 0.0, 0.5],
        [0.0, 0.0, 1.0],
    );
    let (outer, instances) = row_of("docm6-a2-row", inner_ref, 1, 4.0);

    let ev = run(&outer, &opts(store));
    let result = assemble(&outer, &ev, Tol::witness());
    assert!(
        matches!(&result, Err(AssemblyError::AtRest { .. })),
        "a refuted declaration is a finding against the document: {result:?}"
    );
    let findings = findings_of(&result);
    assert!(
        findings.iter().any(|f| matches!(
            &f.attribution,
            Attribution::Carried { route, declaration, relation: Relation::Refuted }
                if route.through == instances[0]
                    && route.of == inner_id
                    && route.via.is_empty()
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
        rendered.contains(&inner_id.to_string()) && rendered.contains("carried from document"),
        "and it renders as one: {rendered}"
    );
}

/// **The verdict this unit MOVES.** A carried declaration the census
/// merely DECLINES — the grazing seat, one carrier and no shared area,
/// decidable in neither direction — is exactly what
/// [`AssemblyError::Uncertified`] means: nothing refuted, nothing
/// undeclared, nothing decided. Before the seam carried the rows the
/// same finding was `Unattributed` and the whole refusal fell through
/// to `AtRest`, a verdict against the document over geometry nothing
/// had decided anything about.
///
/// The inner document alone answered `Uncertified` all along; this row
/// is the outer document catching up to it.
#[test]
fn a_carried_decline_reaches_the_frontier_arm_under_its_own_name() {
    let mut store = StubStore::default();
    let (inner_ref, inner_id, _, inner_mate) = stand(
        &mut store,
        "docm6-decline-stand",
        ContactClass::Rest,
        [1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    );
    let inner = store.docs[&inner_id].clone();
    let inner_result = assemble(&inner, &run(&inner, &opts(store.clone())), Tol::witness());
    assert!(
        matches!(&inner_result, Err(AssemblyError::Uncertified { .. })),
        "the inner document alone is the frontier: {inner_result:?}"
    );

    let (outer, instances) = row_of("docm6-decline-row", inner_ref, 1, 4.0);
    let ev = run(&outer, &opts(store));
    let result = assemble(&outer, &ev, Tol::witness());
    assert!(
        matches!(&result, Err(AssemblyError::Uncertified { .. })),
        "and so is the document that instantiates it: {result:?}"
    );
    let findings = findings_of(&result);
    assert!(!findings.is_empty());
    assert!(
        findings.iter().all(|f| matches!(
            &f.attribution,
            Attribution::Carried { route, declaration, relation: Relation::Declined }
                if route.through == instances[0]
                    && route.of == inner_id
                    && route.via.is_empty()
                    && declaration.mate == inner_mate
        )),
        "every finding names the inner mate, its document and the instance: {findings:?}"
    );
}

/// A3: `Unattributed` means what it says. Over every fixture whose
/// outer gate refuses, an unattributed finding is a contact NO
/// declaration of the tree answers for — never a carried declaration's
/// refutation or decline.
///
/// The listed arms are `attribute`'s own unattributable vocabulary, and
/// the row names them rather than accepting whatever turns up. Each
/// fixture is held to its own floor: a fixture that stopped refusing,
/// or stopped reaching an undeclared contact, is a moved baseline and
/// says so here rather than hiding behind a sibling.
#[test]
fn unattributed_is_only_a_finding_no_declaration_answers_for() {
    let mut store = StubStore::default();
    let (penetrating, ..) = stand(
        &mut store,
        "docm6-a3-pen",
        ContactClass::Rest,
        [0.0, 0.0, 0.5],
        [0.0, 0.0, 1.0],
    );
    let (gapped, ..) = stand(
        &mut store,
        "docm6-a3-gap",
        ContactClass::Rest,
        [0.0, 0.0, 1.5],
        [0.0, 0.0, 1.0],
    );

    // (fixture, does it reach an undeclared contact?)
    let fixtures = [
        // The penetrating seat: the declared pair is refuted AND the
        // interpenetration itself is a crowd of undeclared contacts.
        (row_of("docm6-a3-pen-row", penetrating, 1, 4.0), true),
        (row_of("docm6-a3-pen-pair", penetrating, 2, 4.0), true),
        // The gapped seat touches nothing: its ONE finding is the
        // refuted carried declaration, and nothing is unattributed.
        (row_of("docm6-a3-gap-row", gapped, 1, 4.0), false),
        // Two gapped stands one unit apart: their cubes meet on a face
        // NO mate declared, the undeclared-contact hard error.
        (row_of("docm6-a3-touching", gapped, 2, 1.0), true),
    ];
    for ((doc, _), undeclared_expected) in &fixtures {
        let ev = run(doc, &opts(store.clone()));
        let result = assemble(doc, &ev, Tol::witness());
        let findings = findings_of(&result);
        assert!(!findings.is_empty(), "{doc:?} refuses: {result:?}");
        let mut undeclared = 0;
        for finding in &findings {
            if !matches!(finding.attribution, Attribution::Unattributed) {
                continue;
            }
            undeclared += 1;
            let rendered = format!("{:?}", finding.error);
            assert!(
                ["UndeclaredContact", "UndeclaredCusp", "CensusEscalated"]
                    .iter()
                    .any(|arm| rendered.contains(arm)),
                "an unattributed finding is one no declaration answers for: {rendered}"
            );
        }
        assert_eq!(
            undeclared > 0,
            *undeclared_expected,
            "this fixture's own undeclared floor moved: {findings:?}"
        );
    }
}

// ---- Precedence, when both homes name one pair ----

/// **The precedence rule, and why it is not observable today.**
///
/// `attribute` looks a pair up in this document's own minted rows
/// first and in the carried rows second, and the rule is stated as a
/// rule: where both name one pair, the mate THIS document's author can
/// edit is the one the finding reports. What this row measures is that
/// the overlap is currently UNREACHABLE, and the reason — which is a
/// fact about the solve door, not about the lookup.
///
/// A carried declaration's two faces are both inside ONE instance (a
/// mate is between two members of the document that authored it, and
/// instantiating that document places all of it through one node). So
/// an outer mate naming those same two faces names two faces of one
/// instance, and A11's solve door refuses it `SelfMate` before any
/// record is minted. Two different instances are two different
/// aggregate face pairs, so no other spelling reaches it either.
///
/// The day `SelfMate` admits something — or a second declaring node
/// kind mints into `Product::minted` — this row goes red and the
/// precedence order becomes observable; until then it is a rule
/// written down, not a behaviour a fixture can distinguish.
#[test]
fn an_outer_mate_cannot_name_a_pair_inside_one_instance() {
    let mut store = StubStore::default();
    let (inner_ref, inner_id, inner_instances, _) = stand(
        &mut store,
        "docm6-precedence-stand",
        ContactClass::Rest,
        [0.0, 0.0, 0.5],
        [0.0, 0.0, 1.0],
    );
    let doc = ProfileDoc::empty(DocumentId::derive("docm6-precedence"), Tol::witness());
    let (doc, instance) = insert(doc, Node::instantiate_part(inner_ref));
    // The SAME two faces the inner mate declared, named from out here:
    // the stand's own two cube caps, each seen through this instance.
    let (doc, outer_mate) = insert(
        doc,
        mate_node(
            wrap(instance, in_part(inner_instances[0], CapEnd::End)),
            wrap(instance, in_part(inner_instances[1], CapEnd::Start)),
            ContactClass::Rest,
            frame([0.0, 0.0, 0.5], [0.0, 0.0, 1.0]),
        ),
    );

    let ev = run(&doc, &opts(store));
    let failure = ev
        .node_error(outer_mate)
        .expect("the outer mate does not evaluate");
    assert!(
        format!("{:?}", failure.kind).contains("SelfMate"),
        "both references are members of one instance: {:?}",
        failure.kind
    );
    // And the document does not gather at all, so no product holds two
    // rows for one pair.
    assert!(product_recorded(&doc, &ev, Tol::witness()).is_err());
    assert_ne!(inner_id, doc.id());
}

// ---- Inner mint health at the outermost gate ----

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
    let Err(AssemblyError::CarriedMintRefusal { route, refusal }) = &result else {
        panic!("an inner part with an unverified contact is not at rest: {result:?}");
    };
    assert_eq!(
        (route.through, route.of, route.via.len()),
        (instances[0], inner_id, 0)
    );
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
    let (good, ..) = resting(&mut store, "docm6-order-good");

    // The three documents share a shape: two instances a unit apart,
    // so their cubes MEET on a face no mate declared (the at-rest
    // gate's hard error), plus one `Tangent` mate of this document's
    // own (its unminted head).
    let build = |label: &str, part: DocRef| row_of(label, part, 2, 1.0);
    // The instances here are of a STAND, so a face of one is named one
    // wrapper deeper: through the stand's own instance of the cube.
    let with_own_mate = |doc: ProfileDoc, ids: &[RecipeNodeId]| {
        insert(
            doc,
            mate_node(
                wrap(ids[0], in_part(RecipeNodeId(1), CapEnd::End)),
                wrap(ids[1], in_part(RecipeNodeId(0), CapEnd::Start)),
                ContactClass::Tangent,
                frame([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]),
            ),
        )
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
        matches!(&result, Err(AssemblyError::CarriedMintRefusal { route, .. })
            if route.of == broken_id),
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
    let Err(AssemblyError::CarriedMintRefusal { route, refusal }) = &result else {
        panic!("the refusal reaches the outermost gate: {result:?}");
    };
    assert_eq!(
        (
            route.through,
            route.of,
            route.via.as_slice(),
            refusal.mate()
        ),
        (
            outer_instances[0],
            broken_id,
            &mid_instances[..],
            broken_mate
        )
    );
}

/// **Which carried refusal is raised**: the HEAD in gather order, and
/// only the head — the same rule the own `unminted` head follows, and
/// the same follow-up would widen both.
#[test]
fn the_head_carried_refusal_in_gather_order_is_the_one_raised() {
    let mut store = StubStore::default();
    let (first, first_id, first_mate) = broken_part(&mut store, "docm6-head-first");
    let (second, second_id, _) = broken_part(&mut store, "docm6-head-second");
    assert_ne!(first_id, second_id);

    let doc = ProfileDoc::empty(DocumentId::derive("docm6-head-row"), Tol::witness());
    let (doc, a) = insert(doc, Node::instantiate_part(first));
    let (doc, b) = insert(doc, Node::instantiate_part(second));
    let doc = place(doc, b, [20.0, 0.0, 0.0]);

    let ev = run(&doc, &opts(store));
    let gathered = product_recorded(&doc, &ev, Tol::witness()).expect("gathers");
    assert_eq!(
        gathered
            .carried_unminted
            .iter()
            .map(|r| (r.route.through, r.route.of))
            .collect::<Vec<_>>(),
        vec![(a, first_id), (b, second_id)],
        "both rows are carried, in gather order"
    );
    let result = assemble(&doc, &ev, Tol::witness());
    let Err(AssemblyError::CarriedMintRefusal { route, refusal }) = &result else {
        panic!("the gate refuses: {result:?}");
    };
    assert_eq!(
        (route.through, route.of, refusal.mate()),
        (a, first_id, first_mate),
        "the head, not the second"
    );
}

/// The gate is the ONE reader of `carried_unminted`: the gather
/// carries the rows and every path out of the gate that has seen one
/// is a refusal, so no `Ok(Assembly)` can stand over an inner mint
/// refusal. ("No advisory channel exists" is the grep, named in the PR;
/// this row is what a reader can run.)
#[test]
fn the_gate_has_no_success_arm_over_a_carried_mint_refusal() {
    let mut store = StubStore::default();
    let (broken, ..) = broken_part(&mut store, "docm6-advisory-stand");
    let (good, ..) = resting(&mut store, "docm6-advisory-good");

    // A document that would otherwise certify, plus one broken part.
    let doc = ProfileDoc::empty(DocumentId::derive("docm6-advisory-row"), Tol::witness());
    let (doc, _) = insert(doc, Node::instantiate_part(good));
    let (doc, second) = insert(doc, Node::instantiate_part(broken));
    let doc = place(doc, second, [20.0, 0.0, 0.0]);

    let ev = run(&doc, &opts(store));
    let gathered = product_recorded(&doc, &ev, Tol::witness()).expect("the row gathers");
    assert_eq!(gathered.carried.len(), 1, "the good stand's declaration");
    assert_eq!(
        gathered.carried_unminted.len(),
        1,
        "the broken stand's mate"
    );
    // The gather still carries both — the channel is the gather's —
    // and the gate turns the refusal into a refusal with no success
    // arm that could carry it past.
    assert!(matches!(
        assemble(&doc, &ev, Tol::witness()),
        Err(AssemblyError::CarriedMintRefusal { .. })
    ));
}

/// R-I: a CERTIFIED assembly keeps the carried rows, so it can say
/// which inner mates its verdict answered for.
#[test]
fn a_certified_assembly_names_the_carried_mates_it_certified_over() {
    let mut store = StubStore::default();
    let (inner_ref, inner_id, _, inner_mate) = resting(&mut store, "docm6-ok-stand");
    let (outer, instances) = row_of("docm6-ok-row", inner_ref, 2, 4.0);
    let ev = run(&outer, &opts(store));
    let assembly = assemble(&outer, &ev, Tol::witness()).expect("the row certifies");
    assert!(assembly.minted.is_empty(), "this document has no mates");
    assert_eq!(
        assembly
            .carried
            .iter()
            .map(|c| (c.route.through, c.route.of, c.declaration.mate))
            .collect::<Vec<_>>(),
        instances
            .iter()
            .map(|&i| (i, inner_id, inner_mate))
            .collect::<Vec<_>>(),
    );
}
