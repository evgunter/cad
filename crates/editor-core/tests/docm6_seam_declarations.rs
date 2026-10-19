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
//! The store and the two part-face spellings come from
//! `fixture::resolver`, which every assembly suite instantiates
//! through; what stays here is this suite's own geometry and its
//! `wrap`, the one rung a name climbs per seam.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;
use editor_core::Formula;

use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    Assembly, AssemblyError, Attribution, CapEnd, CarriedRefusal, ContactClass, Coord, DocEdit,
    DocRef, DocumentId, EntityKey, EntityKind, Entry, Frame, Mate, MateRef, Node, Operand,
    ProfileDoc, RecipeNodeId, Relation, RoleSeg, SlideAxis, StableName, Value, assemble,
    product_recorded,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{insert, len, on_frame, run, step};
use geom_core::Tol;

// ---- This suite's own documents, over the shared store ----

/// A block part: `w` × `d` footprint, extruded `h`, and its body.
fn block_part(label: &str, w: f64, d: f64, h: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        ProfileDoc::empty(DocumentId::derive(label), Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (w, 0.0), (w, d), (0.0, d)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(h),
            side: ExtrudeSide::Along,
        },
    )
}

fn cube_part(label: &str) -> (ProfileDoc, RecipeNodeId) {
    block_part(label, 1.0, 1.0, 1.0)
}

/// `inner` seen through the instance `node` placed it at — one rung of
/// the route a name climbs at each seam.
fn wrap(node: RecipeNodeId, inner: StableName) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::InPart { of: inner.into() }],
    }
}

/// Cube `k`'s `cap` as the stand `stand` names it in its product: the
/// cube instance's cap, the instance being its own placement (A10).
/// [`stand`] inserts its two cubes first.
fn in_stand(
    store: &PartStore,
    stand: DocRef,
    k: usize,
    body: RecipeNodeId,
    cap: CapEnd,
) -> StableName {
    let doc = store.doc(stand.id);
    in_part(doc.placements()[k], body, cap)
}

/// **A rest of face `on` on face `to`, `standoff` off it**: `on`'s
/// plane equated with `to`'s slid `standoff` along its own outward
/// normal and flipped, so the faces face each other `standoff` apart (a
/// negative standoff passes `on` into `to`).
fn seat(on: StableName, to: StableName, class: ContactClass, standoff: f64) -> Mate<Formula> {
    let target = fixture::plane(fixture::head(to));
    let target = if standoff == 0.0 {
        target
    } else {
        Operand::Pose(Box::new(PoseDef::Slide {
            pose: RoleRead::plain(target.clone()),
            along: RoleRead::through(target, PoseRole::Normal),
            by: len(standoff),
        }))
    };
    fixture::mate(
        fixture::plane(fixture::head(on)),
        fixture::flip(target),
        class,
    )
}

/// **`mate` added with a value on each freedom it leaves**: a slide of
/// `shift` along its plane's `x`, none along its `y`, and no spin — so
/// its copy is pinned against the target, its own coordinates agreeing
/// with the target's but for the shift.
fn seated(doc: ProfileDoc, mate: Mate<Formula>, shift: f64) -> (ProfileDoc, MateRef) {
    let (doc, mate) = fixture::constrain(doc, mate);
    let doc = [
        (Coord::Slide(SlideAxis::X), len(shift)),
        (Coord::Slide(SlideAxis::Y), len(0.0)),
        (Coord::Spin, fixture::ang(0.0)),
    ]
    .into_iter()
    .fold(doc, |doc, (coord, by)| {
        step(
            doc,
            DocEdit::AddValue {
                placement: mate.placement,
                value: Box::new(Value {
                    mate: mate.id,
                    coord,
                    by,
                }),
                fresh: Vec::new(),
            },
        )
        .0
    });
    (doc, mate)
}

/// **An instance of `part` pinned to the world at the identity**, the
/// store resolving it.
fn pinned_instance(doc: ProfileDoc, store: &PartStore, part: DocRef) -> (ProfileDoc, RecipeNodeId) {
    fixture::insert_pinned(
        doc,
        Node::instantiate_part(part),
        &with_resolver(store.clone()),
    )
}

/// **An instance of `part`, whose part never evaluates, pinned to the
/// world**: no frame can be read off a copy that has no geometry, and
/// none is asked for, so the pin is a frame through an edge and a
/// vertex the copy would name, equated with the world's own. The names
/// are never resolved; the mate is the shape of a pin, so the copy is
/// the world's by its recipe.
fn nominally_pinned_instance(doc: ProfileDoc, part: DocRef) -> (ProfileDoc, RecipeNodeId) {
    let (doc, instance) = insert(doc, Node::instantiate_part(part));
    let copy = Operand::Var(
        doc.output(instance, 0)
            .expect("an instance defines its copies"),
    );
    let named = |kind| StableName {
        kind,
        node: instance,
        path: vec![RoleSeg::Cap(CapEnd::Start)],
    };
    let read = |def: PoseDef<Operand, Formula>| RoleRead::plain(Operand::Pose(Box::new(def)));
    let mate = Mate {
        on: Operand::Pose(Box::new(PoseDef::Through {
            axis: read(PoseDef::Axis {
                of: Operand::select(copy.clone(), vec![named(EntityKind::Edge)]),
            }),
            point: read(PoseDef::Point {
                of: Operand::select(copy, vec![named(EntityKind::Vertex)]),
            }),
        })),
        to: editor_core::Target::World(editor_core::WorldRead::Frame),
        class: ContactClass::Rest,
    };
    (fixture::constrain_at(doc, instance, mate).0, instance)
}

/// `node`'s bundle replaced by one world pin at the translation `at`,
/// the store resolving its part.
fn place(doc: ProfileDoc, store: &PartStore, node: RecipeNodeId, at: [f64; 3]) -> ProfileDoc {
    let doc = fixture::bundle(&doc, node).into_iter().fold(doc, |doc, c| {
        step(
            doc,
            DocEdit::RemoveConstraint {
                placement: node,
                constraint: c.id,
            },
        )
        .0
    });
    fixture::pin(
        doc,
        node,
        &Frame::translation(at),
        &with_resolver(store.clone()),
    )
}

/// Two instances of `part`, the first pinned to the world and the
/// second seated on its top by ONE rest, `standoff` off it and slid
/// `shift` along it. A standoff of `0` is the true declaration; `-0.5`
/// penetrates and `0.5` gaps, both counter-evidence; a shift of `1` is
/// the GRAZING seat — one carrier, no shared area, decidable in neither
/// direction, so the census DECLINES the pair.
fn stand(
    store: &mut PartStore,
    label: &str,
    class: ContactClass,
    standoff: f64,
    shift: f64,
) -> (DocRef, DocumentId, Vec<RecipeNodeId>, MateRef, RecipeNodeId) {
    let (cube, cube_body) = store.insert_part(cube_part(&format!("{label}-cube")), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, c0) = pinned_instance(doc, store, cube);
    let (doc, c1) = insert(doc, fixture::mated_instance(cube));
    let (doc, mate) = seated(
        doc,
        seat(
            in_part(c1, cube_body, CapEnd::Start),
            in_part(c0, cube_body, CapEnd::End),
            class,
            standoff,
        ),
        shift,
    );
    let id = doc.id();
    (
        store.insert(doc, Tol::witness()),
        id,
        vec![c0, c1],
        mate,
        cube_body,
    )
}

/// A resting stand: the ordinary carried declaration.
fn resting(
    store: &mut PartStore,
    label: &str,
) -> (DocRef, DocumentId, Vec<RecipeNodeId>, MateRef, RecipeNodeId) {
    stand(store, label, ContactClass::Rest, 0.0, 0.0)
}

/// A part whose only mate is a `Tangent` — a class that solves and
/// mints NO record at rest, so the part refuses its own gate.
///
/// Returns the reference, the document id, the mate, and the stand's
/// cube instances with the cube's body.
fn broken_part(
    store: &mut PartStore,
    label: &str,
) -> (DocRef, DocumentId, MateRef, Vec<RecipeNodeId>, RecipeNodeId) {
    let (r, id, cubes, mate, cube_body) = stand(store, label, ContactClass::Tangent, 0.0, 0.0);
    (r, id, mate, cubes, cube_body)
}

/// `count` instances of `part` in a row, pinned `spacing` apart along +x
/// and declaring nothing about each other.
fn row_of(
    store: &PartStore,
    label: &str,
    part: DocRef,
    count: usize,
    spacing: f64,
) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for i in 0..count {
        let (next, id) = insert(doc, Node::instantiate_part(part));
        #[allow(clippy::cast_precision_loss)]
        let dx = spacing * i as f64;
        doc = place(next, store, id, [dx, 0.0, 0.0]);
        ids.push(id);
    }
    (doc, ids)
}

fn findings_of(result: &Result<Assembly<f64>, AssemblyError>) -> Vec<editor_core::AtRestFinding> {
    match result {
        Err(AssemblyError::AtRest { findings } | AssemblyError::Uncertified { findings, .. }) => {
            findings.clone()
        }
        // A part's own refusal, carried whole: its findings, each
        // attributed (where it is) to the part's own declaration.
        Err(AssemblyError::CarriedAtRest { findings }) => {
            findings.iter().map(|row| row.finding.clone()).collect()
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

/// **`inner` as the product names it through `hops`** (innermost
/// first): at each seam the name is worn inside the instance's
/// `InPart`, the instance being the placement that copies it (A10).
/// The product's own table answers which name that is — exactly one of
/// its names unwraps to `inner` along `hops`.
fn routed(
    gathered: &editor_core::Product<f64>,
    inner: &StableName,
    hops: &[RecipeNodeId],
) -> StableName {
    fn unwraps(name: &StableName, inner: &StableName, hops: &[RecipeNodeId]) -> bool {
        match hops.split_last() {
            None => name == inner,
            Some((&hop, below)) => match &name.path[..] {
                [RoleSeg::InPart { of }] if name.node == hop => unwraps(of, inner, below),
                _ => false,
            },
        }
    }
    let found: Vec<StableName> = gathered
        .names
        .iter()
        .map(|(n, _)| n.clone())
        .filter(|n| unwraps(n, inner, hops))
        .collect();
    let [name] = found.as_slice() else {
        panic!("{inner:?} through {hops:?} is one product name, not {found:?}");
    };
    name.clone()
}

/// **The A1 oracle**: each carried row's faces are EXACTLY the faces
/// its own route-wrapped references name in the product's table.
///
/// The row's route says how to wrap its inner reference — one
/// `InPart` rung per instance, outermost last ([`routed`]) — and the
/// name table is
/// the product's own independent answer to "which face is that". So a
/// row keyed to some other recorded pair, or to the right pair in the
/// wrong order, or carrying a route that does not lead to its own
/// faces, fails here. A weaker oracle ("the pair is SOME carried
/// record's pair") passes all three.
fn assert_rows_match_names(gathered: &editor_core::Product<f64>) {
    for row in &gathered.carried {
        let hops: Vec<RecipeNodeId> = row
            .route
            .via
            .iter()
            .rev()
            .map(editor_core::SpokenNode::id)
            .chain([row.route.through])
            .collect();
        let want = (
            face_named(gathered, &routed(gathered, &row.declaration.a, &hops)),
            face_named(gathered, &routed(gathered, &row.declaration.b, &hops)),
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
) -> Vec<(RecipeNodeId, DocumentId, Vec<RecipeNodeId>, MateRef)> {
    gathered
        .carried
        .iter()
        .map(|c| {
            (
                c.route.through,
                c.route.of,
                c.route
                    .via
                    .iter()
                    .map(editor_core::SpokenNode::id)
                    .collect(),
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
    let mut store = PartStore::default();
    let (inner_ref, inner_id, _, inner_mate, _) = resting(&mut store, "docm6-a1-stand");
    let (outer, instances) = row_of(&store, "docm6-a1-row", inner_ref, 3, 4.0);

    let ev = run(&outer, &with_resolver(store));
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
    let mut store = PartStore::default();

    // S1: a cube resting on a slab.
    let (slab, slab_body) =
        store.insert_part(block_part("docm6-deep-slab", 2.0, 2.0, 0.5), Tol::witness());
    let (cube, cube_body) =
        store.insert_part(block_part("docm6-deep-cube", 1.0, 1.0, 1.0), Tol::witness());
    let s1 = ProfileDoc::empty(DocumentId::derive("docm6-deep-s1"), Tol::witness());
    let (s1, s1_slab) = pinned_instance(s1, &store, slab);
    let (s1, s1_cube) = insert(s1, crate::fixture::mated_instance(cube));
    let (s1, s1_mate) = seated(
        s1,
        seat(
            in_part(s1_cube, cube_body, CapEnd::Start),
            in_part(s1_slab, slab_body, CapEnd::End),
            ContactClass::Rest,
            0.0,
        ),
        0.0,
    );
    let s1_id = s1.id();
    let s1_ref = store.insert(s1, Tol::witness());

    // The MID: S1 twice, a bare cube, and the mid's OWN mate resting a
    // second cube on the first.
    let mid = ProfileDoc::empty(DocumentId::derive("docm6-deep-mid"), Tol::witness());
    let (mid, m_s1a) = pinned_instance(mid, &store, s1_ref);
    let (mid, m_s1b) = insert(mid, Node::instantiate_part(s1_ref));
    let mid = place(mid, &store, m_s1b, [10.0, 0.0, 0.0]);
    let (mid, m_c0) = insert(mid, Node::instantiate_part(cube));
    let mid = place(mid, &store, m_c0, [30.0, 0.0, 0.0]);
    let (mid, m_c1) = insert(mid, crate::fixture::mated_instance(cube));
    let (mid, mid_mate) = seated(
        mid,
        seat(
            in_part(m_c1, cube_body, CapEnd::Start),
            in_part(m_c0, cube_body, CapEnd::End),
            ContactClass::Rest,
            0.0,
        ),
        0.0,
    );
    let mid_id = mid.id();
    let mid_ref = store.insert(mid, Tol::witness());

    // The OUTER: a bare cube first, so the mid instances' ids do not
    // coincide with the mid's own sub-instance ids, then the mid twice.
    let outer = ProfileDoc::empty(DocumentId::derive("docm6-deep-outer"), Tol::witness());
    let (outer, o_cube) = insert(outer, Node::instantiate_part(cube));
    let outer = place(outer, &store, o_cube, [0.0, -50.0, 0.0]);
    let (outer, o_m1) = pinned_instance(outer, &store, mid_ref);
    let (outer, o_m2) = insert(outer, Node::instantiate_part(mid_ref));
    let outer = place(outer, &store, o_m2, [0.0, 100.0, 0.0]);
    let outer_id = outer.id();

    let ev = run(&outer, &with_resolver(store.clone()));
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
    let (top, t) = pinned_instance(top, &store, outer_ref);
    let ev = run(&top, &with_resolver(store));
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
    let mut store = PartStore::default();
    // A shift of 3: the two caps lie in one plane, apart, so the
    // declared rest is definite counter-evidence.
    let (inner_ref, inner_id, _, inner_mate, _) =
        stand(&mut store, "docm6-a2-stand", ContactClass::Rest, 0.0, 3.0);
    let (outer, instances) = row_of(&store, "docm6-a2-row", inner_ref, 1, 4.0);

    let ev = run(&outer, &with_resolver(store));
    let result = assemble(&outer, &ev, Tol::witness());
    assert!(
        matches!(&result, Err(AssemblyError::AtRest { .. })),
        "a refuted declaration is a finding against the document: {result:?}"
    );
    let findings = findings_of(&result);
    assert!(
        findings.iter().any(|f| matches!(
            &f.attribution,
            Some(Attribution::Carried { route, declaration, relation: Relation::Refuted, .. })
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
        .map(|f| {
            f.attribution
                .as_ref()
                .map_or_else(String::new, ToString::to_string)
        })
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
    let mut store = PartStore::default();
    let (inner_ref, inner_id, _, inner_mate, _) = stand(
        &mut store,
        "docm6-decline-stand",
        ContactClass::Rest,
        0.0,
        1.0,
    );
    let inner = store.doc(inner_id);
    let inner_result = assemble(
        &inner,
        &run(&inner, &with_resolver(store.clone())),
        Tol::witness(),
    );
    assert!(
        matches!(&inner_result, Err(AssemblyError::Uncertified { .. })),
        "the inner document alone is the frontier: {inner_result:?}"
    );

    let (outer, instances) = row_of(&store, "docm6-decline-row", inner_ref, 1, 4.0);
    let ev = run(&outer, &with_resolver(store));
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
            Some(Attribution::Carried { route, declaration, relation: Relation::Declined, .. })
                if route.through == instances[0]
                    && route.of == inner_id
                    && route.via.is_empty()
                    && declaration.mate == inner_mate
        )),
        "every finding names the inner mate, its document and the instance: {findings:?}"
    );
}

/// A3: a finding with no attribution means what it says. Over every
/// fixture whose outer gate refuses, such a finding is one NO
/// declaration of the tree answers for — never a carried declaration's
/// refutation or decline.
///
/// The listed arms are `attribute`'s own unattributable vocabulary, and
/// the row names them rather than accepting whatever turns up. Each
/// fixture is held to its own floor: a fixture that stopped refusing,
/// or stopped reaching an unrecorded contact, is a moved baseline and
/// says so here rather than hiding behind a sibling.
#[test]
fn no_attribution_is_only_a_finding_no_declaration_answers_for() {
    let mut store = PartStore::default();
    let (penetrating, ..) = stand(&mut store, "docm6-a3-pen", ContactClass::Rest, -0.5, 0.0);
    let (gapped, ..) = stand(&mut store, "docm6-a3-gap", ContactClass::Rest, 0.0, 3.0);

    // (fixture, does it reach an unrecorded contact?)
    let fixtures = [
        // The penetrating seat: the declared pair is refuted AND the
        // interpenetration itself is a crowd of unrecorded contacts.
        (
            row_of(&store, "docm6-a3-pen-row", penetrating, 1, 4.0),
            true,
        ),
        (
            row_of(&store, "docm6-a3-pen-pair", penetrating, 2, 4.0),
            true,
        ),
        // The gapped seat touches nothing: its ONE finding is the
        // refuted carried declaration, and nothing is unattributed.
        (row_of(&store, "docm6-a3-gap-row", gapped, 1, 4.0), false),
        // Two gapped stands one unit apart: their cubes meet on a face
        // no mate relates, which the gather records as rows of the two
        // copies rather than a finding of the gate (D1 (iii)), so only
        // the refuted carried declarations are left.
        (row_of(&store, "docm6-a3-touching", gapped, 2, 1.0), false),
    ];
    for ((doc, _), undeclared_expected) in &fixtures {
        let ev = run(doc, &with_resolver(store.clone()));
        let result = assemble(doc, &ev, Tol::witness());
        let findings = findings_of(&result);
        assert!(!findings.is_empty(), "{doc:?} refuses: {result:?}");
        let mut undeclared = 0;
        for finding in &findings {
            if finding.attribution.is_some() {
                continue;
            }
            undeclared += 1;
            let rendered = format!("{:?}", finding.error);
            // The penetrating seat's two cubes pass into each other
            // where they touch, and the census's instance arm refuses
            // that pair as a mixed touch: no declaration answers for a
            // placement either.
            assert!(
                ["UnrecordedContact", "CensusEscalated"]
                    .iter()
                    .any(|arm| rendered.contains(arm))
                    || matches!(
                        finding.error,
                        topo::ValidationError::CensusUndecidable {
                            a: topo::EntityId::Solid(_),
                            b: topo::EntityId::Solid(_),
                            what,
                        } if what.starts_with("one passes into the other where they touch")
                    ),
                "a finding with no attribution is one no declaration answers for: {rendered}"
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
/// fact about the edit door, not about the lookup.
///
/// A carried declaration's two faces are both inside ONE instance (a
/// mate is between two members of the document that authored it, and
/// instantiating that document places all of it through one node). So
/// an outer mate naming those same two faces reads, on its target side,
/// the very copy it places: the placement would read its own output,
/// and the edit door refuses the cycle before the mate enters the
/// document, let alone before any record is minted. Two different
/// instances are two different aggregate face pairs, so no other
/// spelling reaches it either.
///
/// The day a mate's target may read its own copy — or a second
/// declaring node kind mints into `Product::minted` — this row goes red
/// and the precedence order becomes observable; until then it is a
/// rule written down, not a behaviour a fixture can distinguish.
#[test]
fn an_outer_mate_cannot_name_a_pair_inside_one_instance() {
    let mut store = PartStore::default();
    let (inner_ref, inner_id, _, _, cube_body) = stand(
        &mut store,
        "docm6-precedence-stand",
        ContactClass::Rest,
        -0.5,
        0.0,
    );
    let doc = ProfileDoc::empty(DocumentId::derive("docm6-precedence"), Tol::witness());
    let (doc, instance) = pinned_instance(doc, &store, inner_ref);
    // The SAME two faces the inner mate declared, named from out here:
    // the stand's own two cube caps, each seen through this instance.
    let err = doc
        .apply(
            &DocEdit::AddConstraint {
                placement: instance,
                mate: Box::new(seat(
                    wrap(
                        instance,
                        in_stand(&store, inner_ref, 1, cube_body, CapEnd::Start),
                    ),
                    wrap(
                        instance,
                        in_stand(&store, inner_ref, 0, cube_body, CapEnd::End),
                    ),
                    ContactClass::Rest,
                    -0.5,
                )),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect_err("the outer mate never enters the document");
    assert!(
        matches!(
            &err,
            editor_core::EditError::WouldCycle { at } if at.id() == instance
        ),
        "both references are members of one instance: {err:?}"
    );
    // So the product holds exactly the carried rows, and no second
    // declaration for the pair: the document gathers as it did
    // without the attempt.
    let ev = run(&doc, &with_resolver(store));
    assert!(product_recorded(&doc, &ev, Tol::witness()).is_ok());
    assert_ne!(inner_id, doc.id());
}

// ---- Inner mint health at the outermost gate ----

/// A4: an inner document with an unmintable mate refuses the OUTER
/// gate, naming the document, the mate and the instance — and the inner
/// document alone refuses exactly as it did before.
#[test]
fn an_inner_mint_refusal_refuses_the_outer_gate_naming_document_and_mate() {
    let mut store = PartStore::default();
    let (inner_ref, inner_id, inner_mate, ..) = broken_part(&mut store, "docm6-a4-stand");
    let (outer, instances) = row_of(&store, "docm6-a4-row", inner_ref, 1, 4.0);

    // The inner document alone: unchanged, its own class refusal.
    let inner = store.doc(inner_id);
    let inner_ev = run(&inner, &with_resolver(store.clone()));
    assert!(matches!(
        assemble(&inner, &inner_ev, Tol::witness()),
        Err(AssemblyError::Mint { refusals })
            if matches!(refusals.as_slice(), [r] if r.mate() == inner_mate)
    ));

    let ev = run(&outer, &with_resolver(store));
    let result = assemble(&outer, &ev, Tol::witness());
    let Err(AssemblyError::CarriedMintRefusal { refusals }) = &result else {
        panic!("an inner part with an unverified contact is not at rest: {result:?}");
    };
    let [CarriedRefusal { route, refusal, .. }] = refusals.as_slice() else {
        panic!("one broken part, so one carried row: {refusals:?}");
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
            && rendered.contains("constraint 1 of")
            && rendered.contains(&test_utils::refusal::tag(inner_mate.placement.0.digest())),
        "the badge names the document and the mate, as that document holds it: {rendered}"
    );
}

/// A4's ORDER, pinned by three documents that differ in one thing
/// each. The carried refusal is raised BEFORE this document's own
/// unminted head and before the at-rest gate — and the row below it
/// shows the unminted head is live rather than absent.
#[test]
fn the_carried_refusal_precedes_the_own_unminted_head_and_the_at_rest_gate() {
    let mut store = PartStore::default();
    let (broken, broken_id, _, _, broken_body) = broken_part(&mut store, "docm6-order-broken");
    let (good, _, _, _, cube_body) = resting(&mut store, "docm6-order-good");

    // The three documents share a shape: two instances of a stand. In
    // the first, both are pinned a unit apart, so their cubes MEET on a
    // face no mate relates (rows of the two copies, which the gate does
    // not refuse). In the other two, the second instance sits where one
    // `Tangent` mate of this document's own puts it (its unminted head).
    let build = |label: &str, part: DocRef| row_of(&store, label, part, 2, 1.0);
    let build_mated = |label: &str, part: DocRef| {
        let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
        let (doc, first) = pinned_instance(doc, &store, part);
        let (doc, second) = insert(doc, fixture::mated_instance(part));
        (doc, vec![first, second])
    };
    // The instances here are of a STAND, so a face of one is named one
    // wrapper deeper: through the stand's own instance of the cube,
    // `cubes` and `body` being the instantiated stand's.
    let with_own_mate =
        |doc: ProfileDoc, ids: &[RecipeNodeId], (stand, body): (DocRef, RecipeNodeId)| {
            seated(
                doc,
                seat(
                    wrap(ids[1], in_stand(&store, stand, 0, body, CapEnd::Start)),
                    wrap(ids[0], in_stand(&store, stand, 1, body, CapEnd::End)),
                    ContactClass::Tangent,
                    0.0,
                ),
                0.0,
            )
        };

    // (a) The at-rest gate alone: good parts, no mate of this
    // document's own — the contact between the copies is a row, and
    // the gate passes.
    let (doc, _) = build("docm6-order-atrest", good);
    let ev = run(&doc, &with_resolver(store.clone()));
    assert!(assemble(&doc, &ev, Tol::witness()).is_ok());

    // (b) Add this document's own unminted mate: it preempts the gate.
    let (doc, ids) = build_mated("docm6-order-own", good);
    let (doc, own_mate) = with_own_mate(doc, &ids, (good, cube_body));
    let ev = run(&doc, &with_resolver(store.clone()));
    assert!(matches!(
        assemble(&doc, &ev, Tol::witness()),
        Err(AssemblyError::Mint { refusals })
            if matches!(refusals.as_slice(), [r] if r.mate() == own_mate)
    ));

    // (c) The same document over a BROKEN part: the carried refusal
    // preempts both, because the file to open is the inner one.
    let (doc, ids) = build_mated("docm6-order-carried", broken);
    let (doc, _) = with_own_mate(doc, &ids, (broken, broken_body));
    let ev = run(&doc, &with_resolver(store));
    let result = assemble(&doc, &ev, Tol::witness());
    assert!(
        matches!(&result, Err(AssemblyError::CarriedMintRefusal { refusals })
            if !refusals.is_empty() && refusals.iter().all(|r| r.route.of == broken_id)),
        "inner mint health is read first: {result:?}"
    );
}

/// A4: two levels down, the refusal names the whole route.
#[test]
fn a_refusal_two_levels_down_names_its_route() {
    let mut store = PartStore::default();
    let (broken, broken_id, broken_mate, ..) = broken_part(&mut store, "docm6-route-stand");
    let (mid, mid_instances) = row_of(&store, "docm6-route-mid", broken, 1, 4.0);
    let mid_ref = store.insert(mid, Tol::witness());
    let (outer, outer_instances) = row_of(&store, "docm6-route-outer", mid_ref, 1, 12.0);

    let ev = run(&outer, &with_resolver(store));
    let result = assemble(&outer, &ev, Tol::witness());
    let Err(AssemblyError::CarriedMintRefusal { refusals }) = &result else {
        panic!("the refusal reaches the outermost gate: {result:?}");
    };
    let [CarriedRefusal { route, refusal, .. }] = refusals.as_slice() else {
        panic!("one broken part, so one carried row: {refusals:?}");
    };
    assert_eq!(
        (
            route.through,
            route.of,
            route
                .via
                .iter()
                .map(editor_core::SpokenNode::id)
                .collect::<Vec<_>>(),
            refusal.mate()
        ),
        (
            outer_instances[0],
            broken_id,
            mid_instances.clone(),
            broken_mate
        )
    );
}

/// `doc` with `node` labelled `label`.
fn labelled(doc: ProfileDoc, node: RecipeNodeId, label: &str) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetLabel {
            node,
            label: Some(editor_core::Label::new(label).expect("a valid label")),
        },
    )
    .0
}

/// **A deeper hop is said as its own document holds it.** A broken
/// stand, a middle document instancing it (the instance labelled "mid
/// seat"), and an outer document whose first node is the middle
/// document's twin — an instance of the stand, labelled "spare seat" —
/// and whose second instances the middle document. The second row's
/// hop is the middle document's instance, whose id the outer document
/// holds as that twin: the hop says the middle document's label, and
/// the twin's label is said only where the outer document's own
/// instance is meant.
#[test]
fn a_carried_refusals_deeper_hop_says_its_documents_label_where_the_outer_holds_its_id() {
    let mut store = PartStore::default();
    let (broken, broken_id, broken_mate, ..) = broken_part(&mut store, "speak-hop-stand");
    let (mid, mid_instances) = row_of(&store, "speak-hop-mid", broken, 1, 4.0);
    let mid = labelled(mid, mid_instances[0], "mid seat");
    let mid_ref = store.insert(mid.clone(), Tol::witness());
    let (outer, twin) = row_of(&store, "speak-hop-outer", broken, 1, 4.0);
    assert_eq!(
        twin, mid_instances,
        "both documents mint from the zero chain"
    );
    let outer = labelled(outer, twin[0], "spare seat");
    let (outer, instance) = pinned_instance(outer, &store, mid_ref);
    let outer = place(outer, &store, instance, [12.0, 0.0, 0.0]);
    let outer = labelled(outer, instance, "left bracket");

    let ev = run(&outer, &with_resolver(store));
    let result = assemble(&outer, &ev, Tol::witness());
    let Err(AssemblyError::CarriedMintRefusal { refusals }) = &result else {
        panic!("the refusal reaches the outermost gate: {result:?}");
    };
    let [direct, deep] = refusals.as_slice() else {
        panic!("one row by each instance of the stand: {refusals:?}");
    };
    assert_eq!(
        (
            direct.route.through,
            deep.route.through,
            direct.route.of,
            deep.route.of
        ),
        (twin[0], instance, broken_id, broken_id)
    );
    assert_eq!(
        deep.route.via,
        vec![mid.spoken(mid_instances[0])],
        "the hop holds the middle document's node"
    );
    assert_eq!(
        (direct.refusal.mate(), deep.refusal.mate()),
        (broken_mate, broken_mate)
    );
    let spoken = result.as_ref().expect_err("refused").spoken(&outer);
    let hops = format!(
        "{} → InstantiatePart \"mid seat\" ({})",
        left_bracket(instance),
        test_utils::refusal::tag(mid_instances[0].0.digest())
    );
    assert!(
        spoken.contains(&hops),
        "the first hop is the outer document's, the second the middle document's: {spoken}"
    );
    assert_eq!(
        spoken.matches("spare seat").count(),
        1,
        "only the outer document's own instance says the twin's label: {spoken}"
    );
    let said = deep.to_string();
    assert!(
        said.contains(&format!(
            "through instance {} → InstantiatePart \"mid seat\"",
            test_utils::refusal::tag(instance.0.digest())
        )),
        "with no document at hand the first hop is said by its tag, the deeper one as its \
         document holds it: {said}"
    );
}

/// **A carried row holds the labels its pin fixes, and compares them.**
/// One middle document in two versions labelled apart, so two pins:
/// an instance of each carries the stand's refusal by rows whose ids
/// below the instance agree and whose hop labels do not, so the rows
/// differ, and each says its own pin's label.
#[test]
fn a_carried_row_says_the_labels_its_pin_fixes() {
    let mut store = PartStore::default();
    let (broken, ..) = broken_part(&mut store, "speak-pin-stand");
    let (mid, mid_instances) = row_of(&store, "speak-pin-mid", broken, 1, 4.0);
    let rows_of = |mid: ProfileDoc| {
        // One store per version: a store holds one version of a
        // document.
        let mut store = store.clone();
        let mid_ref = store.insert(mid, Tol::witness());
        let outer = ProfileDoc::empty(DocumentId::derive("speak-pin-outer"), Tol::witness());
        let (outer, _) = pinned_instance(outer, &store, mid_ref);
        let ev = run(&outer, &with_resolver(store));
        match assemble(&outer, &ev, Tol::witness()) {
            Err(AssemblyError::CarriedMintRefusal { refusals }) => (mid_ref, refusals),
            other => panic!("the stand's refusal is carried: {other:?}"),
        }
    };
    let (seat_ref, seat) = rows_of(labelled(mid.clone(), mid_instances[0], "mid seat"));
    let (moved_ref, moved) = rows_of(labelled(mid, mid_instances[0], "mid moved"));
    assert_ne!(seat_ref.pin, moved_ref.pin, "a label is in the pin");
    let ids = |rows: &[CarriedRefusal]| {
        rows.iter()
            .map(|r| {
                (
                    r.route
                        .via
                        .iter()
                        .map(editor_core::SpokenNode::id)
                        .collect::<Vec<_>>(),
                    r.refusal.clone(),
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(
        ids(&seat),
        ids(&moved),
        "the two versions' rows name the same ids below the instance"
    );
    // An instance's id is minted from its reference, so the two
    // instances differ; the rows are compared through one of them.
    let moved_through_seat: Vec<CarriedRefusal> = moved
        .iter()
        .zip(&seat)
        .map(|(m, s)| CarriedRefusal {
            route: editor_core::Route {
                through: s.route.through,
                ..m.route.clone()
            },
            ..m.clone()
        })
        .collect();
    assert_ne!(
        seat, moved_through_seat,
        "the rows hold different labels under different pins, so they differ"
    );
    let line = |rows: &[CarriedRefusal]| rows.iter().map(ToString::to_string).collect::<String>();
    assert!(
        line(&seat).contains("\"mid seat\"") && !line(&seat).contains("mid moved"),
        "{}",
        line(&seat)
    );
    assert!(
        line(&moved).contains("\"mid moved\"") && !line(&moved).contains("mid seat"),
        "{}",
        line(&moved)
    );
}

/// **Which carried refusals are raised**: EVERY one, in gather order
/// — the same rule this document's own `unminted` list follows. An
/// author whose assembly holds two broken parts has two files to open,
/// and learning about the second only after repairing the first makes
/// the gate's answer a function of how many evaluations they ran.
#[test]
fn every_carried_refusal_is_raised_in_gather_order() {
    let mut store = PartStore::default();
    let (first, first_id, first_mate, ..) = broken_part(&mut store, "docm6-head-first");
    let (second, second_id, second_mate, ..) = broken_part(&mut store, "docm6-head-second");
    assert_ne!(first_id, second_id);

    let doc = ProfileDoc::empty(DocumentId::derive("docm6-head-row"), Tol::witness());
    let (doc, a) = pinned_instance(doc, &store, first);
    let (doc, b) = pinned_instance(doc, &store, second);
    let doc = place(doc, &store, b, [20.0, 0.0, 0.0]);

    let ev = run(&doc, &with_resolver(store));
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
    let Err(AssemblyError::CarriedMintRefusal { refusals }) = &result else {
        panic!("the gate refuses: {result:?}");
    };
    assert_eq!(
        refusals
            .iter()
            .map(|r| (r.route.through, r.route.of, r.refusal.mate()))
            .collect::<Vec<_>>(),
        vec![(a, first_id, first_mate), (b, second_id, second_mate)],
        "both rows, in gather order — not the head alone"
    );
    let rendered = result.unwrap_err().to_string();
    assert!(
        rendered.contains(&first_id.to_string()) && rendered.contains(&second_id.to_string()),
        "and both documents are named in the one message: {rendered:?}"
    );
}

/// The gate is the ONE reader of `carried_unminted`: the gather
/// carries the rows and every path out of the gate that has seen one
/// is a refusal, so no `Ok(Assembly)` can stand over an inner mint
/// refusal. ("No advisory channel exists" is the grep, named in the PR;
/// this row is what a reader can run.)
#[test]
fn the_gate_has_no_success_arm_over_a_carried_mint_refusal() {
    let mut store = PartStore::default();
    let (broken, ..) = broken_part(&mut store, "docm6-advisory-stand");
    let (good, ..) = resting(&mut store, "docm6-advisory-good");

    // A document that would otherwise certify, plus one broken part.
    let doc = ProfileDoc::empty(DocumentId::derive("docm6-advisory-row"), Tol::witness());
    let (doc, _) = pinned_instance(doc, &store, good);
    let (doc, second) = pinned_instance(doc, &store, broken);
    let doc = place(doc, &store, second, [20.0, 0.0, 0.0]);

    let ev = run(&doc, &with_resolver(store));
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

/// **What the channel does NOT reach, measured.** A boolean's value
/// carries records (its own declared pairs, remapped) and no
/// declaration rows, so a finding against one of them is
/// `Unattributed` — which is correct there, because no mate authored
/// them. What could make it wrong is a declaration row reaching a
/// boolean operand, and nothing can: an instance that CARRIES a
/// declaration is a product of at least two parts (a mate is between
/// two members of the document that authored it), an instance's output
/// is its copies, and a seat that takes one body refuses a whole read
/// of them at the edit door (`EditError::SlotVarKind`, `Bodies` where a
/// `Body` goes), before anything evaluates.
///
/// Every seat, so what refuses is the rule and not this geometry:
/// penetrating, resting, gapped.
#[test]
fn no_carried_declaration_can_reach_a_boolean_operand() {
    for (label, standoff) in [("pen", -0.5), ("rest", 0.0), ("gap", 0.5)] {
        let (store, inner_ref, cube) = product_and_cube(label, standoff);
        let doc = ProfileDoc::empty(
            DocumentId::derive(&format!("docm6-bool-{label}-outer")),
            Tol::witness(),
        );
        let (doc, instance) = pinned_instance(doc, &store, inner_ref);
        refuses_as_product(
            &store,
            &doc,
            instance,
            cube,
            &format!("{label}: the instance"),
        );
    }
}

/// A stand (two mated cubes: a product of two parts, carrying the mate's
/// declaration) and a lone cube, both in one store.
fn product_and_cube(label: &str, standoff: f64) -> (PartStore, DocRef, DocRef) {
    let mut store = PartStore::default();
    let (inner_ref, ..) = stand(
        &mut store,
        &format!("docm6-bool-{label}"),
        ContactClass::Rest,
        standoff,
        0.0,
    );
    let (cube, _) = store.insert_part(
        cube_part(&format!("docm6-bool-{label}-cube")),
        Tol::witness(),
    );
    (store, inner_ref, cube)
}

/// `Boolean(Union, a, cube placed far off)` refused at the edit door:
/// `a` is a whole read of copies, where a member seat takes one body.
fn refuses_as_product(
    store: &PartStore,
    doc: &ProfileDoc,
    a: RecipeNodeId,
    cube: DocRef,
    what: &str,
) {
    let (doc, far) = insert(doc.clone(), Node::instantiate_part(cube));
    let doc = place(doc, store, far, [50.0, 0.0, 0.0]);
    let refused = fixture::insert_refused(
        &doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), far.into()]),
        },
    );
    assert!(
        matches!(
            refused,
            editor_core::EditError::SlotVarKind {
                found: editor_core::VarKind::Bodies,
                ..
            }
        ),
        "{what}: a whole read of copies is no body: {refused:?}"
    );
}

/// **A sub-assembly carries the copies through.** A document whose ONE
/// placement instantiates the stand delivers copies when it is itself
/// instantiated, so the union of that instance refuses too.
#[test]
fn a_product_inside_a_sub_assembly_still_refuses() {
    let (mut store, inner_ref, cube) = product_and_cube("nested", 0.0);
    let wrap = ProfileDoc::empty(DocumentId::derive("docm6-bool-nested-wrap"), Tol::witness());
    let (wrap, _) = pinned_instance(wrap, &store, inner_ref);
    let wrap_ref = store.insert(wrap, Tol::witness());
    let doc = ProfileDoc::empty(
        DocumentId::derive("docm6-bool-nested-outer"),
        Tol::witness(),
    );
    let (doc, instance) = pinned_instance(doc, &store, wrap_ref);
    refuses_as_product(&store, &doc, instance, cube, "the sub-assembly's instance");
}

/// **A placer carries the copies through.** A transformed instance is
/// still its copies, so the union of it refuses at the door.
#[test]
fn a_transformed_product_still_refuses() {
    let (store, inner_ref, cube) = product_and_cube("moved", 0.0);
    let doc = ProfileDoc::empty(DocumentId::derive("docm6-bool-moved-outer"), Tol::witness());
    let (doc, instance) = pinned_instance(doc, &store, inner_ref);
    let (doc, moved) = insert(
        doc,
        Node::Transform {
            input: instance.into(),
            placement: editor_core::Placement::literal(&Frame::translation([0.0, 10.0, 0.0])),
        },
    );
    refuses_as_product(&store, &doc, moved, cube, "the transformed instance");
}

/// **A placed union of a product refuses too**: its prototype takes the
/// copies, and its evaluation refuses them as a product of two parts
/// (`NodeErrorKind::ProductOperand`), naming the explicit union as the
/// recourse.
#[test]
fn a_placed_union_of_a_product_refuses() {
    let (store, inner_ref, _) = product_and_cube("placed", 0.0);
    let doc = ProfileDoc::empty(
        DocumentId::derive("docm6-bool-placed-outer"),
        Tol::witness(),
    );
    let (doc, instance) = pinned_instance(doc, &store, inner_ref);
    let (doc, group) = insert(
        doc,
        Node::PlacedUnion {
            input: instance.into(),
            count: None,
            kind: editor_core::PatternKind::Explicit(vec![
                Frame::IDENTITY,
                Frame::translation([20.0, 0.0, 0.0]),
            ]),
        },
    );
    let ev = run(&doc, &with_resolver(store));
    let failure = ev
        .node_error(group)
        .expect("the placed union does not evaluate over a product");
    assert!(
        matches!(
            failure.kind,
            editor_core::NodeErrorKind::ProductOperand { input: i, parts: 2 } if i == instance
        ),
        "the placed union's prototype: refused as a product: {:?}",
        failure.kind
    );
}

/// **A datum reads a product.** A face frame consumes no material — it
/// reads a face — so the product refusal is not its: a frame on the
/// top cap of the stand's upper cube, read through an instance of the
/// two-part stand, evaluates.
#[test]
fn a_face_frame_on_a_product_evaluates() {
    let mut store = PartStore::default();
    let (inner_ref, _, _, _, cube_body) = resting(&mut store, "docm6-datum-stand");
    let doc = ProfileDoc::empty(DocumentId::derive("docm6-datum-outer"), Tol::witness());
    let (doc, instance) = pinned_instance(doc, &store, inner_ref);
    let (doc, datum) = insert(
        doc,
        Node::Datum(editor_core::Datum::FaceFrame {
            face: editor_core::Operand::select(
                instance,
                vec![wrap(
                    instance,
                    in_stand(&store, inner_ref, 1, cube_body, CapEnd::End),
                )],
            ),
            spin: fixture::ang(0.0),
        }),
    );
    let ev = run(&doc, &with_resolver(store));
    assert_eq!(
        ev.value(instance).map(|v| v.parts),
        Some(2),
        "the stand is a product"
    );
    assert!(
        ev.node_error(datum).is_none(),
        "the frame reads the product: {:?}",
        ev.node_error(datum).map(|e| &e.kind)
    );
    assert!(ev.value(datum).is_some(), "the frame has a value");
}

/// R-I: a CERTIFIED assembly keeps the carried rows, so it can say
/// which inner mates its verdict answered for.
#[test]
fn a_certified_assembly_names_the_carried_mates_it_certified_over() {
    let mut store = PartStore::default();
    let (inner_ref, inner_id, _, inner_mate, _) = resting(&mut store, "docm6-ok-stand");
    let (outer, instances) = row_of(&store, "docm6-ok-row", inner_ref, 2, 4.0);
    let ev = run(&outer, &with_resolver(store));
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

// ---- A carried row says a part's ids as the part holds them ----

/// A stand over a shared cube part: two instances of it, the first
/// pinned, and one rest between their caps, slid `shift` along x, in the
/// second instance's bundle, that instance labelled `label`, in a
/// document named `id`.
fn stand_over(
    store: &PartStore,
    (cube, cube_body): (DocRef, RecipeNodeId),
    id: &str,
    class: ContactClass,
    shift: f64,
    label: &str,
) -> (ProfileDoc, MateRef) {
    let doc = ProfileDoc::empty(DocumentId::derive(id), Tol::witness());
    let (doc, c0) = pinned_instance(doc, store, cube);
    let (doc, c1) = insert(doc, fixture::mated_instance(cube));
    let (doc, mate) = seated(
        doc,
        seat(
            in_part(c1, cube_body, CapEnd::Start),
            in_part(c0, cube_body, CapEnd::End),
            class,
            0.0,
        ),
        shift,
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetLabel {
            node: mate.placement,
            label: Some(editor_core::Label::new(label).expect("a valid label")),
        },
    );
    (doc, mate)
}

/// **A part whose mate id the outer document also holds**: the outer
/// document is the same stand (labelled "outer seat") plus an instance
/// of the part (labelled "inner seat"), placed clear of it, the
/// instance labelled "left bracket". Every document's mint starts at the
/// zero chain, so the two stands mint the same ids. A carried row says
/// the part's ids as the part holds them; one spoken from the outer
/// document would name the outer mate. Its route's first instance is
/// the outer document's, so that document speaks it.
///
/// Returns the store, the part, the outer document, the shared id and
/// the instance.
fn outer_over_twin(
    class: ContactClass,
    shift: f64,
) -> (PartStore, ProfileDoc, ProfileDoc, MateRef, RecipeNodeId) {
    let mut store = PartStore::default();
    let cube = store.insert_part(cube_part("speak-cube"), Tol::witness());
    let (inner, mate) = stand_over(&store, cube, "speak-inner", class, shift, "inner seat");
    let inner_ref = store.insert(inner.clone(), Tol::witness());
    let (outer, outer_mate) = stand_over(&store, cube, "speak-outer", class, shift, "outer seat");
    assert_eq!(outer_mate, mate, "both stands mint from the zero chain");
    let (outer, instance) = insert(outer, Node::instantiate_part(inner_ref));
    let outer = place(outer, &store, instance, [10.0, 0.0, 0.0]);
    let (outer, _) = step(
        outer,
        DocEdit::SetLabel {
            node: instance,
            label: Some(editor_core::Label::new("left bracket").expect("a valid label")),
        },
    );
    (store, inner, outer, mate, instance)
}

/// The route's first instance as the outer document says it.
fn left_bracket(instance: RecipeNodeId) -> String {
    format!(
        "through InstantiatePart \"left bracket\" ({})",
        test_utils::refusal::tag(instance.0.digest())
    )
}

/// The part's own mint refusal speaks its labelled mate from the part;
/// the same row carried to the outer gate says it as the part does,
/// though the outer document holds that id as its own labelled mate, and
/// its route's first instance is spoken from the outer document.
#[test]
fn a_carried_mint_refusal_says_the_parts_label_where_the_outer_document_holds_the_id() {
    let (store, inner, outer, mate, instance) = outer_over_twin(ContactClass::Tangent, 0.0);
    let t = test_utils::refusal::tag(mate.placement.0.digest());
    let c = mate.id;

    let inner_ev = run(&inner, &with_resolver(store.clone()));
    let own = assemble(&inner, &inner_ev, Tol::witness()).expect_err("a tangent mints nothing");
    let AssemblyError::Mint { refusals } = &own else {
        panic!("the part refuses its own mate: {own:?}");
    };
    let spoken_row = refusals[0].spoken(&inner);
    assert!(
        spoken_row.starts_with(&format!(
            "constraint 1 of InstantiatePart \"inner seat\" ({t})'s class"
        )),
        "{spoken_row}"
    );
    assert!(
        own.spoken(&inner).contains(&spoken_row),
        "{}",
        own.spoken(&inner)
    );
    assert!(
        own.to_string()
            .contains(&format!("constraint {c} of node {t}'s class")),
        "the tag form keeps the placement's noun: {own}"
    );
    let empty = ProfileDoc::empty(DocumentId::derive("speak-empty"), Tol::witness());
    assert!(
        refusals[0]
            .spoken(&empty)
            .starts_with(&format!("constraint {c} of node {t}'s class")),
        "a document that does not hold the mate keeps its noun: {}",
        refusals[0].spoken(&empty)
    );

    let ev = run(&outer, &with_resolver(store));
    let carried = assemble(&outer, &ev, Tol::witness()).expect_err("the part is not at rest");
    assert!(
        matches!(&carried, AssemblyError::CarriedMintRefusal { refusals }
            if matches!(refusals.as_slice(), [r] if r.refusal.mate() == mate)),
        "the carried row is raised first: {carried:?}"
    );
    let spoken = carried.spoken(&outer);
    assert!(
        spoken.contains(&format!(": {spoken_row}")),
        "the carried row says the part's refusal as the part does: {spoken}"
    );
    assert!(
        !spoken.contains("outer seat"),
        "a carried row is never spoken from the outer document: {spoken}"
    );
    assert!(
        spoken.contains(&left_bracket(instance)),
        "the route's first instance is spoken from the outer document: {spoken}"
    );
    let said = carried.to_string();
    assert!(
        said.contains(&format!(
            "through instance {}",
            test_utils::refusal::tag(instance.0.digest())
        )) && said.contains(&format!(": {spoken_row}")),
        "with no document at hand the instance is said by its tag, and the part's mate as \
         the part holds it: {said}"
    );
}

/// Both stands' declared rests are refuted (seat 0.5: the cubes
/// interpenetrate). The outer document's own finding speaks its
/// labelled mate; the part's carried finding says the part's, though
/// the outer document holds that id.
#[test]
fn a_carried_attribution_says_the_parts_label_and_this_documents_own_is_spoken() {
    let (store, _, outer, mate, instance) = outer_over_twin(ContactClass::Rest, 3.0);
    let t = test_utils::refusal::tag(mate.placement.0.digest());
    let ev = run(&outer, &with_resolver(store));
    let result = assemble(&outer, &ev, Tol::witness());
    let findings = findings_of(&result);
    let own = findings
        .iter()
        .find(|f| matches!(f.attribution, Some(Attribution::Refuted(_))))
        .expect("the outer document's own rest is refuted");
    let carried = findings
        .iter()
        .find(|f| matches!(f.attribution, Some(Attribution::Carried { .. })))
        .expect("the part's rest is refuted, carried");
    assert!(
        own.spoken(&outer).starts_with(&format!(
            "constraint 1 of InstantiatePart \"outer seat\" ({t})'s Rest contact"
        )),
        "{}",
        own.spoken(&outer)
    );
    let carried_line = carried.spoken(&outer);
    assert!(
        carried_line.starts_with(&format!(
            "constraint 1 of InstantiatePart \"inner seat\" ({t})'s Rest contact"
        )) && !carried_line.contains("outer seat"),
        "{carried_line}"
    );
    assert!(
        carried.to_string().starts_with(&format!(
            "constraint 1 of InstantiatePart \"inner seat\" ({t})'s Rest contact"
        )),
        "with no document at hand the part's mate is said as the part holds it: {carried}"
    );
    assert!(
        carried_line.contains(&left_bracket(instance)),
        "the route's first instance is spoken from the outer document: {carried_line}"
    );
    let whole = result.expect_err("refuted").spoken(&outer);
    assert_eq!(
        whole.matches("outer seat").count(),
        1,
        "only the outer document's own row speaks its label: {whole}"
    );
}

// ---- A carried level is spoken from the part only where the part is held ----

/// A stand whose second constraint fixes what its first does: the top
/// cube rests on the base's top cap, and a second rest beside it, past
/// the door (the state a doctored save leaves), seats it half a unit
/// higher. Both are the top cube's bundle, in a document named `id`,
/// the top cube labelled `label`; the solve refuses the pair
/// overconstrained. Every document's mint starts at the zero chain, so
/// two such stands mint the same ids. Answers the document and its two
/// constraints.
fn overconstrained(
    store: &PartStore,
    cube: DocRef,
    cube_body: RecipeNodeId,
    id: &str,
    label: &str,
) -> (ProfileDoc, [MateRef; 2]) {
    let doc = ProfileDoc::empty(DocumentId::derive(id), Tol::witness());
    let (doc, c0) = pinned_instance(doc, store, cube);
    let (doc, c1) = insert(doc, fixture::mated_instance(cube));
    let rest = |standoff| {
        seat(
            in_part(c1, cube_body, CapEnd::Start),
            in_part(c0, cube_body, CapEnd::End),
            ContactClass::Rest,
            standoff,
        )
    };
    let (doc, held) = fixture::constrain_at(doc, c1, rest(0.0));
    let (doc, added) =
        editor_core::test_support::beside_past_the_bundle_rule(&doc, c1, rest(0.5), Tol::witness());
    let added = MateRef::new(c1, added);
    let (doc, _) = step(
        doc,
        DocEdit::SetLabel {
            node: c1,
            label: Some(editor_core::Label::new(label).expect("a valid label")),
        },
    );
    (doc, [held, added])
}

/// **A part's nodes are said as the pinned part holds them, never as
/// the outer document holds the same ids.** The part and the outer
/// document both hold an overconstrained pair of mates under the same ids,
/// labelled apart. The outer document's own failure speaks both of its
/// mates from the outer document and names its failing node once; the
/// instance's fault and its carried level say the part's labels in a
/// frame holding only the outer document, exactly as the part's own
/// tree draws them.
#[test]
fn a_carried_level_says_the_parts_labels_where_the_outer_document_holds_its_ids() {
    let mut store = PartStore::default();
    let (cube, cube_body) = store.insert_part(cube_part("speak-c-cube"), Tol::witness());
    let (inner, _) = overconstrained(&store, cube, cube_body, "speak-c-inner", "inner top");
    let inner_ref = store.insert(inner.clone(), Tol::witness());
    let (outer, [held, _added]) =
        overconstrained(&store, cube, cube_body, "speak-c-outer", "outer top");
    assert_eq!(
        inner.ids(),
        outer.ids(),
        "both stands mint from the zero chain"
    );
    let top = held.placement;
    let (outer, instance) = nominally_pinned_instance(outer, inner_ref);
    let tt = test_utils::refusal::tag(top.0.digest());

    let ev = run(&outer, &with_resolver(store.clone()));
    let own = ev
        .node_error(top)
        .expect("the top cube's bundle is refused");
    assert_eq!(
        own.spoken(&outer, &ev),
        format!(
            "InstantiatePart \"outer top\" ({tt}) failed: the mate solve refused: this node's \
             each of constraints 1, 2 fixes an equation {}",
            own.to_string()
                .split_once(" fixes an equation ")
                .expect("the pair's sentence")
                .1
        ),
        "the outer placement's own failure speaks the constraints it holds, and names itself \
         once"
    );
    let (renamed, _) = step(
        outer.clone(),
        DocEdit::SetLabel {
            node: top,
            label: Some(editor_core::Label::new("outer renamed").expect("a valid label")),
        },
    );
    assert!(
        own.spoken(&renamed, &ev)
            .starts_with(&format!("InstantiatePart \"outer renamed\" ({tt}) failed")),
        "the memoized refusal holds the id, so a rename shows with nothing re-evaluated: {}",
        own.spoken(&renamed, &ev)
    );

    let error = ev
        .node_error(instance)
        .expect("the instance's part has no body");
    let levels: Vec<_> = error.kind.carried_chain().collect();
    let [level] = levels.as_slice() else {
        panic!("one carried level, in the part: {levels:?}");
    };
    assert_eq!(level.document.doc_ref(), Some(&inner_ref), "{levels:?}");
    let inner_ev = run(&inner, &with_resolver(store));
    let inner_own = inner_ev
        .node_error(level.node)
        .expect("the part's own tree draws the level's node failed")
        .spoken(&inner, &inner_ev);
    let in_outer = level.line_in(&outer);
    assert_eq!(
        in_outer, inner_own,
        "a frame holding only the outer document draws the level as the part's own tree does"
    );
    assert!(
        in_outer.contains("\"inner top\"") && !in_outer.contains("outer"),
        "the level says the part's labels and never the outer document's for the same ids: \
         {in_outer}"
    );
    assert_eq!(in_outer, level.line(), "the level holds its own nodes");
    let in_frame = error.spoken(&outer, &ev);
    assert!(
        in_frame.contains(&format!("the part's {} failed", inner.spoken(level.node)))
            && !in_frame.contains("outer"),
        "the instance's own line says the part's node as the part holds it: {in_frame}"
    );
    let (_, fault_said) = in_frame
        .split_once(" failed: ")
        .expect("the instance's line opens with its node");
    assert!(
        error.to_string().ends_with(fault_said),
        "a frame without the outer document says the part's nodes the same: {error}"
    );
}

/// **A part's labels are its pin's.** A relabelled part is another
/// version with another pin, and an instance of each says the labels its
/// own pin fixes: the snapshot on a fault can never be another
/// version's, and two faults over different pins are unequal.
#[test]
fn a_part_fault_says_the_labels_its_pin_fixes() {
    let mut store = PartStore::default();
    let (cube, cube_body) = store.insert_part(cube_part("speak-d-cube"), Tol::witness());
    let (inner, [held, _]) = overconstrained(&store, cube, cube_body, "speak-d-inner", "inner top");
    let (relabelled, _) = step(
        inner.clone(),
        DocEdit::SetLabel {
            node: held.placement,
            label: Some(editor_core::Label::new("inner moved").expect("a valid label")),
        },
    );
    // One store per version: a store holds one version of a document.
    let instance_of = |part: ProfileDoc| {
        let mut store = store.clone();
        let part_ref = store.insert(part, Tol::witness());
        let outer = ProfileDoc::empty(DocumentId::derive("speak-d-outer"), Tol::witness());
        let (outer, instance) = nominally_pinned_instance(outer, part_ref);
        let ev = run(&outer, &with_resolver(store));
        let error = ev.node_error(instance).expect("the part has no body");
        let level = error
            .kind
            .carried_chain()
            .next()
            .unwrap_or_else(|| panic!("the part's level: {error:?}"))
            .line();
        (part_ref, part_fault(error).0.clone(), level)
    };
    let (inner_ref, at_inner, inner_line) = instance_of(inner);
    let (moved_ref, at_moved, moved_line) = instance_of(relabelled);
    assert_ne!(inner_ref.pin, moved_ref.pin, "a label is in the pin");
    assert!(
        inner_line.contains("\"inner top\"") && !inner_line.contains("inner moved"),
        "{inner_line}"
    );
    assert!(
        moved_line.contains("\"inner moved\"") && !moved_line.contains("inner top"),
        "{moved_line}"
    );
    assert_ne!(
        at_inner, at_moved,
        "the faults hold different labels under different pins, so they differ"
    );
}

/// The part fault of a failed part instance, with its reference.
fn part_fault(error: &editor_core::NodeError) -> (&editor_core::PartFault, &DocRef) {
    let editor_core::NodeErrorKind::Part { fault, doc_ref } = &error.kind else {
        panic!("the instance refuses on its part: {error:?}");
    };
    (fault, doc_ref)
}

// ---- A part's product refusal says the part's labels ----

/// A block placed in the world and then deleted, its placement
/// labelled `label` and stranded, in a document named `id`. Every
/// document's mint starts at the zero chain, so two such documents
/// hold the placement under one id.
fn stranded(id: &str, label: &str) -> (ProfileDoc, RecipeNodeId) {
    let (doc, block) = cube_part(id);
    let (doc, placement) = fixture::place(doc, block);
    let (doc, _) = step(
        doc,
        DocEdit::SetLabel {
            node: placement,
            label: Some(editor_core::Label::new(label).expect("a valid label")),
        },
    );
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: block });
    (doc, placement)
}

/// **A part's product refusal says the part's labels in a frame holding
/// only the outer document**, which holds the same id as its own
/// labelled extrude, and says it as the part's own gather does.
#[test]
fn a_parts_product_refusal_says_the_parts_label_where_the_outer_document_holds_the_id() {
    let mut store = PartStore::default();
    let (inner, block) = stranded("speak-p-inner", "inner block");
    let inner_ref = store.insert(inner.clone(), Tol::witness());
    let (outer, outer_block) = stranded("speak-p-outer", "outer block");
    assert_eq!(
        outer_block, block,
        "both documents mint from the zero chain"
    );
    let (outer, instance) = nominally_pinned_instance(outer, inner_ref);
    let opts = with_resolver(store);
    let t = test_utils::refusal::tag(block.0.digest());

    let ev = run(&outer, &opts);
    let error = ev.node_error(instance).expect("the part has no product");
    let (fault, _) = part_fault(error);
    let editor_core::PartFault::PartProduct { refusal, .. } = fault else {
        panic!("the part's gather refuses: {fault:?}");
    };
    assert_eq!(
        refusal.kind(),
        editor_core::ProductErrorKind::StrandedPlacement
    );
    let in_outer = error.spoken(&outer, &ev);
    assert!(
        in_outer.contains(&format!(
            "Place \"inner block\" ({t}) reads a body that is gone"
        )) && !in_outer.contains("outer block"),
        "a part's refusal says the part's label, never the outer document's for its id: \
         {in_outer}"
    );
    let own = product_recorded(&inner, &run(&inner, &opts), Tol::witness())
        .expect_err("the part's own gather refuses")
        .spoken(&inner);
    let in_part = fault.to_string();
    assert!(
        in_part.contains(
            own.strip_prefix("product: ")
                .expect("the gather's stage word")
        ),
        "as the part's own gather says it: {in_part} / {own}"
    );
}

/// **A mate the edit door refuses speaks the nodes its fault names** as
/// the document it would stand in holds them, and names itself once:
/// a mate on one instance whose `on` reads another's face.
#[test]
fn a_mate_refused_at_the_door_speaks_the_nodes_its_fault_names() {
    let mut store = PartStore::default();
    let (cube, cube_body) = store.insert_part(cube_part("speak-e-cube"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("speak-e"), Tol::witness());
    let (doc, leg) = pinned_instance(doc, &store, cube);
    let (doc, foot) = pinned_instance(doc, &store, cube);
    let (doc, _) = step(
        doc,
        DocEdit::SetLabel {
            node: leg,
            label: Some(editor_core::Label::new("leg").expect("a valid label")),
        },
    );
    let err = editor_core::apply(
        &doc,
        &DocEdit::AddConstraint {
            placement: leg,
            mate: Box::new(seat(
                in_part(foot, cube_body, CapEnd::Start),
                in_part(leg, cube_body, CapEnd::End),
                ContactClass::Rest,
                0.0,
            )),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .expect_err("the mate's `on` reads another copy");
    let editor_core::EditError::MateRefused { node, fault, .. } = &err else {
        panic!("{err:?}");
    };
    assert!(
        matches!(**fault, editor_core::MateFault::NotOnTheCopy { mate } if mate.placement == leg),
        "{fault:?}"
    );
    let t = test_utils::refusal::tag(leg.0.digest());
    let said = err.to_string();
    assert!(
        said.starts_with(&format!(
            "InstantiatePart \"leg\" ({t}) is refused by the solve on its own datum: \
             constraint "
        )) && said.contains(" of this node's `on` pose reads geometry other than the copy")
            && node.id() == leg,
        "the placement is spoken once, as the document holds it, and the fault says it as \
         this node: {err}"
    );
    assert!(
        fault.to_string().contains(&t),
        "the fault's own words say the placement by tag: {fault}"
    );
}

/// **A part's own at-rest refusal crosses whole.** A sub-assembly holds
/// the gapped stand, whose carried declaration its census refutes, and
/// a block of its own three ε off the stand's lower cube, in the
/// sliver band, where its census escalates. Its own gate refuses both. A top
/// document instantiating it refuses `CarriedAtRest` with that refusal
/// whole: the escalation no declaration answers for, and the refutation
/// of the stand's mate, attributed as the sub-assembly attributed it,
/// rather than dropping the refutation because the carried refusal is
/// raised before the top's own verdict
/// (`work/intent/a-part-drops-its-census-findings-at-the-seam.md`).
///
/// *Breaks if* the carried refusal keeps only the findings no
/// declaration answers for.
#[test]
fn a_parts_own_refusal_crosses_whole_with_its_refuted_mate() {
    let mut store = PartStore::default();
    let (gapped, stand_id, _, stand_mate, _) = stand(
        &mut store,
        "docm6-whole-stand",
        ContactClass::Rest,
        0.0,
        3.0,
    );
    let mid = ProfileDoc::empty(DocumentId::derive("docm6-whole-mid"), Tol::witness());
    let (mid, _) = pinned_instance(mid, &store, gapped);
    let (mid, profile) = on_frame(
        mid,
        [0.0, 0.0, 0.25],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (1.0 + 3.0 * Tol::witness().eps(), 0.25),
            (1.5, 0.25),
            (1.5, 0.75),
            (1.0 + 3.0 * Tol::witness().eps(), 0.75),
        ]],
    );
    let (mid, block) = insert(
        mid,
        Node::Extrude {
            profile: profile.into(),
            distance: len(0.5),
            side: ExtrudeSide::Along,
        },
    );
    let (mid, _) = fixture::place(mid, block);
    let Err(AssemblyError::AtRest { findings: own }) = assemble(
        &mid,
        &run(&mid, &with_resolver(store.clone())),
        Tol::witness(),
    ) else {
        panic!("the premise: the sub-assembly refuses at rest")
    };
    assert!(
        own.iter().any(|f| f.attribution.is_none())
            && own.iter().any(|f| matches!(
                &f.attribution,
                Some(Attribution::Carried { declaration, relation: Relation::Refuted, .. })
                    if declaration.mate == stand_mate
            )),
        "the premise: a refutation beside a finding no declaration answers for: {own:?}"
    );
    let mid_id = mid.id();
    let mid_ref = store.insert(mid, Tol::witness());
    let (top, instances) = row_of(&store, "docm6-whole-top", mid_ref, 1, 4.0);
    let result = assemble(&top, &run(&top, &with_resolver(store)), Tol::witness());
    let Err(AssemblyError::CarriedAtRest { findings }) = &result else {
        panic!("the sub-assembly's refusal, carried: {result:?}")
    };
    assert_eq!(
        findings.iter().map(|row| &row.finding).collect::<Vec<_>>(),
        own.iter().collect::<Vec<_>>(),
        "the refusal whole, verbatim"
    );
    assert!(
        findings
            .iter()
            .all(|row| (row.route.through, row.route.of) == (instances[0], mid_id)),
        "every row is the sub-assembly's, by the instance"
    );
    assert!(
        findings.iter().any(|row| matches!(
            &row.finding.attribution,
            Some(Attribution::Carried { route, .. }) if route.of == stand_id
        )),
        "the refutation names the stand, as the sub-assembly said it"
    );
    // ONE recourse for the list, in the header, as the carried mint
    // refusal's: the repair is to open the document below.
    let line = result.expect_err("refused").spoken(&top);
    assert_eq!(
        line.matches("open those documents and repair them there")
            .count(),
        1,
        "{line}"
    );
}
