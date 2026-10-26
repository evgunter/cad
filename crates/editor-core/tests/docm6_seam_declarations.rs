//! **What crosses the instantiation seam, and how a part's rows are
//! said.**
//!
//! A part's contact RECORDS cross the document seam with its geometry,
//! and its own at-rest refusal crosses whole, raised by the outermost
//! gate before anything else this document has to say. A product is no
//! boolean operand, however it is reached. And a row or fault of a part
//! is said as the pinned part holds its nodes, never as the outer
//! document holds the same ids.
//!
//! The store and the two part-face spellings come from
//! `fixture::resolver`, which every assembly suite instantiates
//! through; what stays here is this suite's own geometry and its
//! `wrap`, the one rung a name climbs per seam.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::Formula;

use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    AssemblyError, CapEnd, Coord, DocEdit, DocRef, DocumentId, EntityKind, Frame, Mate, MateRef,
    Node, Operand, ProfileDoc, RecipeNodeId, RoleSeg, StableName, TargetAxis, assemble,
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
fn seat(on: StableName, to: StableName, standoff: f64) -> Mate<Formula> {
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
    fixture::mate(fixture::plane(fixture::head(on)), fixture::flip(target))
}

/// **`mate` added with a value on each freedom it leaves**: a slide of
/// `shift` along its plane's `x`, none along its `y`, and no spin — so
/// its copy is pinned against the target, slid `shift` along the
/// target space's `x`.
fn seated(doc: ProfileDoc, mate: Mate<Formula>, shift: f64) -> (ProfileDoc, MateRef) {
    let (doc, mate) = crate::fixture::constrain_with(
        doc,
        mate,
        &[
            (Coord::Slide(TargetAxis::X), len(shift)),
            (Coord::Slide(TargetAxis::Y), len(0.0)),
            (Coord::SPIN, fixture::ang(0.0)),
        ],
    );
    (doc, mate)
}

/// **An instance of `part` pinned to the world at the identity**, the
/// store resolving it.
fn pinned_instance(doc: ProfileDoc, store: &PartStore, part: DocRef) -> (ProfileDoc, RecipeNodeId) {
    fixture::insert_pinned(
        doc,
        crate::fixture::mated_instance(part),
        &with_resolver(store.clone()),
    )
}

/// **An instance of `part`, whose part never evaluates, in the world**:
/// by the identity bundle, which reads nothing of the copy, so the copy
/// is the world's by its recipe.
fn nominally_pinned_instance(doc: ProfileDoc, part: DocRef) -> (ProfileDoc, RecipeNodeId) {
    insert(doc, crate::fixture::mated_instance(part))
}

/// `node`'s bundle replaced by one world mate at the translation `at`,
/// the store resolving its part. The values go first, since a value
/// names its mate.
fn place(doc: ProfileDoc, store: &PartStore, node: RecipeNodeId, at: [f64; 3]) -> ProfileDoc {
    let doc = fixture::stand_again(doc, node);
    fixture::pin(
        doc,
        node,
        &Frame::translation(at),
        &with_resolver(store.clone()),
    )
}

/// Two instances of `part`, the first pinned to the world and the
/// second seated on its top by ONE rest, `standoff` off it and slid
/// `shift` along it. A standoff of `0` rests the cubes; `-0.5`
/// penetrates and `0.5` gaps.
fn stand(
    store: &mut PartStore,
    label: &str,
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

/// `count` instances of `part` in a row, pinned `spacing` apart along
/// +x.
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
        let (next, id) = insert(doc, crate::fixture::mated_instance(part));
        #[allow(clippy::cast_precision_loss)]
        let dx = spacing * i as f64;
        doc = place(next, store, id, [dx, 0.0, 0.0]);
        ids.push(id);
    }
    (doc, ids)
}

// ---- An outer mate and a part's own pair ----

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
    let (inner_ref, inner_id, _, _, cube_body) =
        stand(&mut store, "docm6-precedence-stand", -0.5, 0.0);
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

// ---- A product is no boolean operand ----

/// **A product is no boolean operand.** An instance of a document with
/// two mated parts is a product, an instance's output is its copies,
/// and a seat that takes one body refuses a whole read of them at the
/// edit door (`EditError::SlotVarKind`, `Bodies` where a `Body` goes),
/// before anything evaluates.
///
/// Every seat, so what refuses is the rule and not this geometry:
/// penetrating, resting, gapped.
#[test]
fn a_mated_product_is_no_boolean_operand() {
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

/// A stand (two mated cubes: a product of two parts) and a lone cube,
/// both in one store.
fn product_and_cube(label: &str, standoff: f64) -> (PartStore, DocRef, DocRef) {
    let mut store = PartStore::default();
    let (inner_ref, ..) = stand(&mut store, &format!("docm6-bool-{label}"), standoff, 0.0);
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
    let (doc, far) = insert(doc.clone(), crate::fixture::mated_instance(cube));
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

/// **A placer carries the copies through.** A placed copy of an
/// instance is still its copies, so the union of it refuses at the door.
#[test]
fn a_placed_product_still_refuses() {
    let (store, inner_ref, cube) = product_and_cube("moved", 0.0);
    let doc = ProfileDoc::empty(DocumentId::derive("docm6-bool-moved-outer"), Tol::witness());
    let (doc, instance) = pinned_instance(doc, &store, inner_ref);
    let (doc, moved) = insert(
        doc,
        crate::fixture::stood(vec![editor_core::BodyRead::plain(
            editor_core::Operand::from(instance),
        )]),
    );
    let doc = fixture::pinned(doc, moved, &with_resolver(store.clone()));
    refuses_as_product(&store, &doc, moved, cube, "the placed instance");
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

/// **A measure reads a product.** A measure consumes no material — it
/// reads faces — so the product refusal is not its: the distance across
/// the stand's upper cube, cap to cap, read through an instance of the
/// two-part stand, evaluates.
#[test]
fn a_measure_on_a_product_evaluates() {
    let mut store = PartStore::default();
    let (inner_ref, _, _, _, cube_body) = stand(&mut store, "docm6-datum-stand", 0.0, 0.0);
    let doc = ProfileDoc::empty(DocumentId::derive("docm6-datum-outer"), Tol::witness());
    let (doc, instance) = pinned_instance(doc, &store, inner_ref);
    let cap = |end| {
        editor_core::Operand::select(
            instance,
            vec![wrap(
                instance,
                in_stand(&store, inner_ref, 1, cube_body, end),
            )],
        )
    };
    let (doc, datum) = fixture::measure_node(
        &doc,
        editor_core::MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![cap(CapEnd::End), cap(CapEnd::Start)],
    );
    let ev = run(&doc, &with_resolver(store));
    assert_eq!(
        ev.value(instance).map(|v| v.parts),
        Some(2),
        "the stand is a product"
    );
    assert!(
        ev.node_error(datum).is_none(),
        "the measure reads the product: {:?}",
        ev.node_error(datum).map(|e| &e.kind)
    );
    assert!(ev.value(datum).is_some(), "the measure has a value");
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
             each of constraints 1, 5 fixes an equation {}",
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
/// a stand and a block of its own three ε off the stand's lower cube, in
/// the sliver band, where its census escalates. Its own gate refuses. A
/// top document instantiating it refuses `CarriedAtRest` with that
/// refusal whole, verbatim, by the instance
/// (`work/intent/a-part-drops-its-census-findings-at-the-seam.md`).
///
/// *Breaks if* the carried refusal drops or re-says a finding.
#[test]
fn a_parts_own_refusal_crosses_whole() {
    let mut store = PartStore::default();
    let (gapped, ..) = stand(&mut store, "docm6-whole-stand", 0.0, 3.0);
    let mid = ProfileDoc::empty(DocumentId::derive("docm6-whole-mid"), Tol::witness());
    let (mid, _) = pinned_instance(mid, &store, gapped);
    let (mid, profile) = on_frame(
        mid,
        [0.0, 0.0, 0.0],
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
        },
    );
    let (mid, _) = fixture::in_world(
        mid,
        block,
        &editor_core::Frame::translation([0.0, 0.0, 0.25]),
    );
    let Err(AssemblyError::AtRest { findings: own }) = assemble(
        &mid,
        &run(&mid, &with_resolver(store.clone())),
        Tol::witness(),
    ) else {
        panic!("the premise: the sub-assembly refuses at rest")
    };
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
    // ONE recourse for the list, in the header: the repair is to open
    // the document below.
    let line = result.expect_err("refused").spoken(&top);
    assert_eq!(
        line.matches("open those documents and repair them there")
            .count(),
        1,
        "{line}"
    );
}
