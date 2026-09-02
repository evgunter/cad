//! ASM-R2a acceptance — the mate solve.
//!
//! Rows 1–7 of the unit's acceptance list; row 8 is CI's clippy lanes.
//! Everything here runs on a STUB resolver, the substrate ASM-2A D-3
//! established for this layer.
//!
//! Undo note, as in `asm_roots` and `asm2a_instantiate`: this layer has
//! no undo STACK. `apply` is pure and undo is keeping prior values, so
//! "undo restores exactly" is checked the way the layer offers it — the
//! pre-edit document, held across the edit, is unchanged, bit for bit.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Alignment, AxisSense, CancelToken, ClusterMaintenance, ContactClass, DocEdit, DocRef,
    DocumentId, EditError, EntityKind, EvalOptions, Evaluation, Frame, MateFrame, MatePrimitive,
    MateRole, Node, NodeErrorKind, NodeResult, PartResolver, ProfileDoc, RecipeNodeId,
    ResolveFailure, ResolveFault, RoleSeg, StableName, apply, clusters, content_pin, evaluate,
    load, product, relative_freedom_components, save, solve_document,
};
use fixture::{desc, insert, len, on_frame, square, step};
use geom_core::Tol;

/// `step`, with the minted id unwrapped — every insert in this suite
/// mints one.
fn mint(doc: ProfileDoc, edit: DocEdit<editor_core::ProfileProgram>) -> (ProfileDoc, RecipeNodeId) {
    let (doc, id) = step(doc, edit);
    (doc, id.expect("the insert minted an id"))
}

// ---- Substrate ----

#[derive(Debug, Default)]
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

impl PartResolver for StubStore {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        let fail = |fault, message: &str| ResolveFailure {
            fault,
            message: message.to_string(),
        };
        let doc = self
            .docs
            .get(&doc_ref.id)
            .ok_or_else(|| fail(ResolveFault::Unresolved, "no such document"))?;
        let found = content_pin(doc, Tol::witness()).expect("the pin computes");
        if found != doc_ref.pin {
            return Err(fail(ResolveFault::PinMismatch, "the pin does not hold"));
        }
        Ok(doc.clone())
    }
}

/// A one-solid part: a unit square extruded 1 tall.
fn part(label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
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

/// An assembly of `n` instances of one part, plus the store that
/// resolves them.
fn assembly(label: &str, n: usize) -> (ProfileDoc, Vec<RecipeNodeId>, StubStore) {
    let mut store = StubStore::default();
    let doc_ref = store.insert(part(&format!("{label}-part")), Tol::witness());
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..n {
        let (next, id) = insert(doc, Node::instantiate_part(doc_ref));
        doc = next;
        ids.push(id);
    }
    (doc, ids, store)
}

/// An instance-qualified name: a face of `instance`'s part product.
/// The HEAD is the instantiate node, which is exactly what A12's
/// reading edge is recomputed from.
fn in_part(instance: RecipeNodeId, part_node: RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: part_node,
                path: vec![RoleSeg::Cap(editor_core::CapEnd::Bottom)],
            }),
        }],
    }
}

fn frame(origin: [f64; 3], axis: [f64; 3], reference: [f64; 3]) -> MateFrame {
    MateFrame {
        origin,
        axis,
        reference,
    }
}

fn mate(
    a: RecipeNodeId,
    b: RecipeNodeId,
    primitive: MatePrimitive,
    sense: AxisSense,
    fa: MateFrame,
    fb: MateFrame,
    clocking: Option<f64>,
) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a: in_part(a, RecipeNodeId(1)),
        b: in_part(b, RecipeNodeId(1)),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: fa,
            b: fb,
            primitive,
            sense,
            clocking,
        },
    }
}

/// Whether two frames agree to `tol` in every stored coordinate — for
/// poses that come out of a closed form through the transcendentals,
/// where the claim is the VALUE and bit-equality is not on offer.
fn near(a: Frame, b: Frame, tol: f64) -> bool {
    a.columns
        .iter()
        .flatten()
        .chain(a.translation.iter())
        .zip(b.columns.iter().flatten().chain(b.translation.iter()))
        .all(|(x, y)| (x - y).abs() <= tol)
}

fn z_up() -> MateFrame {
    frame([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0])
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

fn mate_fault(ev: &Evaluation<f64>, node: RecipeNodeId) -> editor_core::MateFault {
    match ev.result(node) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::Mate(fault) => (**fault).clone(),
            other => panic!("expected a Mate refusal, got {other:?}"),
        },
        other => panic!("expected a failure, got {other:?}"),
    }
}

// ---- Row 1: a determined pair, evaluated, deterministic ----

/// The A11 rule-1 exemplar, verbatim: **coaxial plus flush plus
/// clocking** is the DETERMINED case. The spec row names it
/// "coaxial+planar(⊥)", but the binding table has no trivial entry for
/// coaxial ∩ planar at any angle — parallel gives revolute,
/// perpendicular gives prismatic — so the clocking rider is what
/// closes it, which is exactly the combination A11 rule 1 lists as its
/// own DETERMINED example.
fn determined_pair() -> (ProfileDoc, Vec<RecipeNodeId>, StubStore, RecipeNodeId) {
    let (doc, ids, store) = assembly("asm-r2a-row1", 2);
    // Coaxial about the shared +z, clocked at zero: prismatic along z.
    let (doc, _) = step(
        doc,
        DocEdit::InsertNode {
            node: mate(
                ids[0],
                ids[1],
                MatePrimitive::Coaxial,
                AxisSense::Aligned,
                z_up(),
                z_up(),
                Some(0.0),
            ),
        },
    );
    // The seating face: a's outward normal is +z at z = 1; b's own
    // outward normal is −z at its origin. Opposed, so b lands facing
    // it, at the authored standoff (the offset runs along the TARGET
    // frame's own axis, which the opposed sense has already flipped —
    // so −1 lifts b clear rather than driving it in).
    let (doc, rest) = mint(
        doc,
        DocEdit::InsertNode {
            node: mate(
                ids[0],
                ids[1],
                MatePrimitive::PlanarRest { offset: -1.0 },
                AxisSense::Opposed,
                frame([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
                frame([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [1.0, 0.0, 0.0]),
                None,
            ),
        },
    );
    (doc, ids, store, rest)
}

/// A pair determined by ONE mate: frame coincidence, offset a unit up
/// the shared axis. The maintenance rows want this shape because
/// deleting the single mate is the split, and the document it splits
/// FROM is one that solved.
fn stacked_pair(label: &str) -> (ProfileDoc, Vec<RecipeNodeId>, RecipeNodeId) {
    let (doc, ids, _) = assembly(label, 2);
    let (doc, joint) = mint(
        doc,
        DocEdit::InsertNode {
            node: mate(
                ids[0],
                ids[1],
                MatePrimitive::FrameCoincidence,
                AxisSense::Aligned,
                frame([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
                z_up(),
                None,
            ),
        },
    );
    (doc, ids, joint)
}

#[test]
fn row1_a_coaxial_clocked_rest_pair_is_determined_and_evaluates() {
    let (doc, ids, store, _) = determined_pair();
    let poses = solve_document(&doc, Tol::witness());
    assert_eq!(poses.fault(ids[1]), None, "the pair solves");
    assert_eq!(poses.gauge(ids[1]), Some(ids[0]), "the gauge is instance 0");
    let relative = poses.relative(ids[1]).expect("a solved pose");
    assert!(
        relative.bit_eq(&Frame::translation([0.0, 0.0, 2.0])),
        "the child sits at the seating height plus the standoff: {relative:?}"
    );
    assert!(
        poses
            .relative(ids[0])
            .expect("the gauge")
            .is_identity_bits(),
        "the gauge's own relative pose is the bit-exact identity"
    );

    let o = opts(store);
    let ev = run(&doc, &o);
    assert!(matches!(ev.result(ids[1]), Some(NodeResult::Ok(_))));
    let body = product(&doc, &ev, Tol::witness()).expect("the product gathers");
    let volume = topo::mass_properties(&body, Tol::witness())
        .expect("mass properties")
        .volume;
    assert!(
        (volume - 2.0).abs() < 1e-9,
        "two unit blocks, stacked, share only a face: {volume}"
    );
}

/// D9 at this layer: two independent evaluations of the same document
/// agree BIT for bit, and two saves produce identical bytes. (The
/// two-fresh-PROCESS variant is the document layer's row, where ASM-2A
/// put it — a process hosts one ε, so the cross-process claim needs
/// the store.)
#[test]
fn row1_evaluation_and_save_bytes_are_bit_identical_across_runs() {
    let (doc, ids, store, _) = determined_pair();
    let o = opts(store);
    let first = run(&doc, &o);
    let second = run(&doc, &o);
    let volume = |ev: &Evaluation<f64>| {
        let body = product(&doc, ev, Tol::witness()).expect("gathers");
        topo::mass_properties(&body, Tol::witness())
            .expect("props")
            .volume
    };
    assert_eq!(
        volume(&first).to_bits(),
        volume(&second).to_bits(),
        "two evaluations of one document agree bit for bit"
    );
    let _ = ids;
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    assert_eq!(
        text,
        save(&doc, &[], Tol::witness()).expect("saves"),
        "byte-stable"
    );
    let back = load(&text, Tol::witness()).expect("loads").doc;
    assert!(back.bit_eq(&doc), "the mates round-trip bit for bit");
}

// ---- Row 2: UNDER, naming the residual and its parameters ----

#[test]
fn row2_a_v_block_refuses_under_naming_prismatic_and_its_direction() {
    let (doc, ids, store) = assembly("asm-r2a-row2", 2);
    let mut doc = doc;
    for axis in [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] {
        let (next, _) = step(
            doc,
            DocEdit::InsertNode {
                node: mate(
                    ids[0],
                    ids[1],
                    MatePrimitive::PlanarRest { offset: 0.0 },
                    AxisSense::Opposed,
                    frame([0.0, 0.0, 0.0], axis, [0.0, 0.0, 1.0]),
                    frame([0.0, 0.0, 0.0], axis, [0.0, 0.0, 1.0]),
                    None,
                ),
            },
        );
        doc = next;
    }
    let poses = solve_document(&doc, Tol::witness());
    let fault = poses.fault(ids[1]).expect("the pair refuses").clone();
    let editor_core::MateFault::Under { residual, .. } = &fault else {
        panic!("expected UNDER, got {fault:?}");
    };
    assert_eq!(residual.name(), "prismatic");
    assert_eq!(residual.dimension(), Some(1));
    let message = fault.to_string();
    assert!(
        message.contains("prismatic freedom of translation along [0, 0, 1]"),
        "the refusal names the residual AND its parameters: {message}"
    );
    assert!(
        message.contains(editor_core::UNDER_RECOURSE),
        "A11 rule 4's recourse text rides the refusal: {message}"
    );

    // The instances refuse too — they have no pose — and the evaluation
    // says so in the mate's own vocabulary.
    let ev = run(&doc, &opts(store));
    assert!(matches!(
        mate_fault(&ev, ids[1]),
        editor_core::MateFault::Under { .. }
    ));
}

// ---- Row 3: CONTRADICTORY, naming both mates and the clash ----

#[test]
fn row3_a_gap_mismatched_planar_pair_refuses_contradictory() {
    let (doc, ids, _) = assembly("asm-r2a-row3", 2);
    let mut doc = doc;
    let mut mates = Vec::new();
    for height in [0.0, 1.0] {
        let (next, id) = mint(
            doc,
            DocEdit::InsertNode {
                node: mate(
                    ids[0],
                    ids[1],
                    MatePrimitive::PlanarRest { offset: 0.0 },
                    AxisSense::Opposed,
                    frame([0.0, 0.0, height], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
                    frame([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [1.0, 0.0, 0.0]),
                    None,
                ),
            },
        );
        doc = next;
        mates.push(id);
    }
    let poses = solve_document(&doc, Tol::witness());
    let fault = poses.fault(mates[1]).expect("the pair refuses").clone();
    let editor_core::MateFault::Contradictory {
        held,
        added,
        predicate,
        clash,
    } = &fault
    else {
        panic!("expected CONTRADICTORY, got {fault:?}");
    };
    assert_eq!(*held, mates[0], "the mate already folded is named");
    assert_eq!(*added, mates[1], "the mate that died against it is named");
    assert_eq!(*predicate, "mate_member_translation_in_plane");
    assert!(
        (clash.abs() - 1.0).abs() < 1e-9,
        "the measured clash IS the authored gap mismatch: {clash}"
    );
    let message = fault.to_string();
    assert!(message.contains(predicate), "{message}");
    assert!(message.contains("clash"), "{message}");
}

// ---- Row 4: the cluster-record keying migration ----

#[test]
fn row4a_a_mate_insert_joins_two_clusters_consuming_the_absorbed_frame() {
    let (doc, ids, _) = assembly("asm-r2a-row4a", 2);
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[0],
            frame: Frame::translation([1.0, 0.0, 0.0]),
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[1],
            frame: Frame::translation([0.0, 5.0, 0.0]),
        },
    );
    assert_eq!(clusters(&doc).len(), 2, "two singleton clusters");
    let before = doc.clone();

    let applied = apply(
        &doc,
        &DocEdit::InsertNode {
            node: mate(
                ids[0],
                ids[1],
                MatePrimitive::Coaxial,
                AxisSense::Aligned,
                z_up(),
                z_up(),
                Some(0.0),
            ),
        },
        Tol::witness(),
    )
    .expect("the mate inserts");
    assert_eq!(clusters(&applied.doc).len(), 1, "one cluster now");
    assert_eq!(
        applied.maintenance,
        vec![ClusterMaintenance::Join {
            survived: ids[0],
            absorbed: ids[1],
            absorbed_frame: Some(Frame::translation([0.0, 5.0, 0.0])),
        }],
        "the join names the survivor and CONSUMES the absorbed frame"
    );
    assert_eq!(
        applied.doc.placements().len(),
        1,
        "one cluster, one recorded frame"
    );
    assert!(
        applied.doc.placement(ids[1]).bit_eq(&doc.placement(ids[0])),
        "the surviving gauge's frame is unchanged, bit for bit"
    );
    assert!(before.bit_eq(&doc), "undo is the prior value, held exactly");
}

#[test]
fn row4b_a_mate_delete_splits_and_re_mints_from_the_solved_pose() {
    let (doc, ids, joint) = stacked_pair("asm-r2a-row4b");
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[0],
            frame: Frame::translation([0.0, 0.0, 4.0]),
        },
    );
    let before = doc.clone();
    let applied =
        apply(&doc, &DocEdit::DeleteNode { id: joint }, Tol::witness()).expect("the mate deletes");
    assert_eq!(clusters(&applied.doc).len(), 2, "the cluster split");
    assert_eq!(
        applied.maintenance,
        vec![ClusterMaintenance::Split {
            from: ids[0],
            to: ids[1],
            frame: Some(Frame::translation([0.0, 0.0, 5.0])),
        }],
        "the orphan's frame is RE-MINTED from its solved pose, so its \
         world pose is unchanged"
    );
    assert!(
        applied
            .doc
            .placement(ids[0])
            .bit_eq(&Frame::translation([0.0, 0.0, 4.0])),
        "the surviving cluster keeps its frame verbatim"
    );
    assert!(before.bit_eq(&doc), "undo is the prior value, held exactly");
}

#[test]
fn row4c_deleting_the_gauge_rewrites_the_key_and_holds_world_poses() {
    let (doc, ids, _) = stacked_pair("asm-r2a-row4c");
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[0],
            frame: Frame::translation([0.0, 0.0, 4.0]),
        },
    );
    let before = doc.clone();
    let applied = apply(&doc, &DocEdit::DeleteNode { id: ids[0] }, Tol::witness())
        .expect("the gauge deletes");
    assert_eq!(
        applied.maintenance,
        vec![ClusterMaintenance::GaugeRewrite {
            from: ids[0],
            to: ids[1],
            frame: Some(Frame::translation([0.0, 0.0, 5.0])),
        }],
        "the key moves to the next representative, composed with the \
         already-solved relative pose, so the survivor's world pose \
         does not move"
    );
    assert_eq!(
        applied.doc.placements().keys().copied().collect::<Vec<_>>(),
        vec![ids[1]]
    );
    assert!(before.bit_eq(&doc), "undo is the prior value, held exactly");
}

#[test]
fn row4d_a_no_mates_document_round_trips_identically_below_the_header() {
    let (doc, ids, _) = assembly("asm-r2a-row4d", 3);
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[2],
            frame: Frame::rotate_then_translate([0.0, 0.0, 1.0], 0.25, [2.0, 3.0, 0.0]),
        },
    );
    assert_eq!(
        doc.placements().keys().copied().collect::<Vec<_>>(),
        vec![ids[2]],
        "a singleton cluster's gauge IS its instance, so the key is \
         exactly the pre-mate one"
    );
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let back = load(&text, Tol::witness()).expect("loads").doc;
    assert!(back.bit_eq(&doc), "the round trip is bit-exact");
    // The header necessarily moved (the `Node::Mate` arm took a schema
    // version), so byte-identity is asserted BELOW it — everything the
    // cluster keying could have touched.
    let body = |t: &str| t.split_once('\n').expect("a header").1.to_string();
    assert_eq!(
        body(&text),
        body(&save(&back, &[], Tol::witness()).expect("saves")),
        "every byte below the header survives the keying migration"
    );
}

// ---- Row 4e/4f: the cut is a union of WHOLE clusters (ASM-4 rider ii) ----

/// Two clusters of two: `{i0,i1}` and `{i2,i3}`, each joined by one
/// mate, each carrying a recorded frame on its gauge.
fn two_clusters() -> (ProfileDoc, Vec<RecipeNodeId>, Vec<RecipeNodeId>) {
    let (doc, ids, _) = assembly("asm-r2a-clusters", 4);
    let mut doc = doc;
    let mut mates = Vec::new();
    for (a, b) in [(0, 1), (2, 3)] {
        let (next, m) = mint(
            doc,
            DocEdit::InsertNode {
                node: mate(
                    ids[a],
                    ids[b],
                    MatePrimitive::FrameCoincidence,
                    AxisSense::Aligned,
                    frame([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
                    z_up(),
                    None,
                ),
            },
        );
        doc = next;
        mates.push(m);
    }
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[0],
            frame: Frame::translation([3.0, 0.0, 0.0]),
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[2],
            frame: Frame::translation([7.0, 0.0, 0.0]),
        },
    );
    (doc, ids, mates)
}

/// Row 4e — a cut of ONE WHOLE cluster hoists its frame onto the
/// remainder instance and leaves the part unplaced. This is ASM-4's
/// D-2 rider (ii) doing its job at a cluster the pre-mate predicate
/// could not even spell: two instantiate nodes, one cluster.
#[test]
fn row4e_a_whole_cluster_cut_hoists_the_cluster_frame() {
    use std::collections::BTreeSet;
    let (doc, ids, mates) = two_clusters();
    let cut = BTreeSet::from([ids[0], ids[1], mates[0]]);
    let out = editor_core::split(
        &doc,
        &cut,
        editor_core::DocumentId::derive("asm-r2a-4e"),
        Tol::witness(),
    )
    .expect("a whole-cluster cut splits");
    assert!(
        out.part.placements().is_empty(),
        "the part holds the cluster UNPLACED — its world pose belongs \
         to the assembly"
    );
    let instance = *out
        .remainder
        .order()
        .iter()
        .find(|id| doc.node(**id).is_none())
        .expect("the remainder gained an instance");
    assert!(
        out.remainder
            .placement(instance)
            .bit_eq(&Frame::translation([3.0, 0.0, 0.0])),
        "the cluster's frame HOISTED onto the remainder instance"
    );
    assert!(
        out.remainder
            .placement(ids[2])
            .bit_eq(&Frame::translation([7.0, 0.0, 0.0])),
        "the untouched cluster keeps its own frame"
    );
}

/// Row 4f — a cut that TEARS a cluster refuses typed, naming the
/// cluster and the instance on the far side (review MAJOR-2).
///
/// A11 puts the frame on the cluster, so a torn cluster has one frame
/// and two homes. Before this refusal landed, such a cut passed the
/// hoist's cluster count (the torn cluster was FILTERED OUT of it, so
/// a torn cluster looked like an absent one) and the torn frame —
/// `[7,0,0]` here — silently vanished.
#[test]
fn row4f_a_torn_cluster_cut_refuses_typed_naming_both_sides() {
    use std::collections::BTreeSet;
    let (doc, ids, mates) = two_clusters();
    // One whole cluster PLUS one instance torn out of the other.
    let cut = BTreeSet::from([ids[0], ids[1], mates[0], ids[2]]);
    match editor_core::split(
        &doc,
        &cut,
        editor_core::DocumentId::derive("asm-r2a-4f"),
        Tol::witness(),
    ) {
        Err(editor_core::SplitError::TornCluster {
            gauge,
            instance,
            gauge_is_cut,
        }) => {
            assert_eq!(gauge, ids[2], "the torn cluster's gauge is named");
            assert_eq!(instance, ids[3], "so is the member left behind");
            assert!(gauge_is_cut, "and which side each is on");
        }
        other => panic!("expected TornCluster, got {other:?}"),
    }
    // The message names the cluster, both sides, and the repair.
    let message = editor_core::split(
        &doc,
        &cut,
        editor_core::DocumentId::derive("asm-r2a-4f2"),
        Tol::witness(),
    )
    .expect_err("refuses")
    .to_string();
    assert!(message.contains("tears the placement cluster"), "{message}");
    assert!(message.contains("widen the cut"), "{message}");
    // The tear is refused in the OTHER direction too: keeping the
    // gauge and cutting the member is the same fault.
    let other_way = BTreeSet::from([ids[0], ids[1], mates[0], ids[3]]);
    match editor_core::split(
        &doc,
        &other_way,
        editor_core::DocumentId::derive("asm-r2a-4f3"),
        Tol::witness(),
    ) {
        Err(editor_core::SplitError::TornCluster {
            gauge,
            instance,
            gauge_is_cut,
        }) => {
            assert_eq!((gauge, instance), (ids[2], ids[3]));
            assert!(!gauge_is_cut);
        }
        other => panic!("expected TornCluster, got {other:?}"),
    }
}

// ---- Row 5: the closure enumeration, executed ----

#[test]
fn row5_the_closure_set_is_closed_under_intersection() {
    use editor_core::Subgroup;
    use geom_core::linalg::{Point3, Vec3};
    use geom_core::predicate::Band;
    let band = Band::linear(Tol::witness()).expect("a band");
    let x = Vec3::new(1.0, 0.0, 0.0);
    let y = Vec3::new(0.0, 1.0, 0.0);
    let z = Vec3::new(0.0, 0.0, 1.0);
    let o = Point3::origin();
    let off = Point3::new(0.0, 1.0, 0.0);
    let planar = |n| Subgroup::Planar { normal: n };
    let cyl = |p, d| Subgroup::Cylindrical {
        point: p,
        direction: d,
    };
    let prism = |d| Subgroup::Prismatic { direction: d };
    let rev = |p, d| Subgroup::Revolute {
        point: p,
        direction: d,
    };
    let meet =
        |a, b| editor_core::mate::coset::intersect_subgroups(a, b, band, 1.0).expect("decided");
    // The universal rows: SE(3) is the identity, empty absorbs,
    // trivial is the zero.
    for g in [
        Subgroup::Se3,
        planar(z),
        cyl(o, z),
        prism(z),
        rev(o, z),
        Subgroup::Trivial,
        Subgroup::Empty,
    ] {
        assert_eq!(meet(Subgroup::Se3, g), g, "X ∩ SE(3) = X");
        assert_eq!(meet(g, Subgroup::Se3), g, "unordered");
        assert_eq!(
            meet(Subgroup::Empty, g),
            Subgroup::Empty,
            "X ∩ empty = empty"
        );
        if !matches!(g, Subgroup::Empty) {
            assert_eq!(meet(Subgroup::Trivial, g), Subgroup::Trivial);
        }
    }
    // Table row 1 — planar ∩ planar.
    assert_eq!(meet(planar(z), planar(z)), planar(z));
    assert_eq!(meet(planar(x), planar(y)), prism(z), "the V-block");
    // Table row 2 — planar ∩ cylindrical.
    assert_eq!(meet(planar(z), cyl(o, z)), rev(o, z), "the pin in the hole");
    assert_eq!(meet(planar(z), cyl(o, x)), prism(x), "the slot");
    assert_eq!(
        meet(planar(z), cyl(o, Vec3::new(1.0, 0.0, 1.0).normalize())),
        Subgroup::Trivial,
        "a generic angle kills both freedoms"
    );
    // Table row 3 — cylindrical ∩ cylindrical.
    assert_eq!(meet(cyl(o, z), cyl(o, z)), cyl(o, z), "the same line");
    assert_eq!(meet(cyl(o, z), cyl(off, z)), prism(z), "parallel, distinct");
    assert_eq!(meet(cyl(o, z), cyl(o, x)), Subgroup::Trivial, "concurrent");
    assert_eq!(meet(cyl(o, z), cyl(off, x)), Subgroup::Trivial, "skew");
    // Table row 4 — prismatic against everything.
    assert_eq!(meet(prism(x), planar(z)), prism(x), "in-plane");
    assert_eq!(meet(prism(z), planar(z)), Subgroup::Trivial);
    assert_eq!(meet(prism(z), cyl(o, z)), prism(z));
    assert_eq!(meet(prism(x), cyl(o, z)), Subgroup::Trivial);
    assert_eq!(meet(prism(z), prism(z)), prism(z));
    assert_eq!(meet(prism(x), prism(z)), Subgroup::Trivial);
    assert_eq!(meet(prism(z), rev(o, z)), Subgroup::Trivial);
    // Table row 5 — revolute against everything.
    assert_eq!(meet(rev(o, z), planar(z)), rev(o, z));
    assert_eq!(meet(rev(o, z), planar(x)), Subgroup::Trivial);
    assert_eq!(meet(rev(o, z), cyl(o, z)), rev(o, z));
    assert_eq!(meet(rev(o, z), cyl(off, z)), Subgroup::Trivial);
    assert_eq!(meet(rev(o, z), rev(o, z)), rev(o, z));
    assert_eq!(meet(rev(o, z), rev(off, z)), Subgroup::Trivial);
    // Closedness, stated as the obligation it is: every result above is
    // itself a member of the closure set.
    for a in [planar(z), cyl(o, z), prism(z), rev(o, z), Subgroup::Trivial] {
        for b in [planar(x), cyl(off, x), prism(y), rev(o, y), Subgroup::Se3] {
            let out = meet(a, b);
            assert!(
                out.dimension().is_some() || matches!(out, Subgroup::Empty),
                "the closure is closed: {} ∩ {} = {}",
                a.name(),
                b.name(),
                out.name()
            );
        }
    }
}

// ---- Row 5b: the COSET level — invariant matches, not just types ----
//
// Row 5 enumerates SUBGROUP types. The table's other half is the
// representative: entry 3's "inter-axis displacement invariants match
// A vs B (decided) → prismatic, else empty". Those matches had no
// direct rows, and that gap is exactly where review MAJOR-1 hid — the
// candidate pinned the held representative's clocking instead of
// solving the free angle about the shared axis, so an assemblable
// two-pin pattern refused CONTRADICTORY. These rows are at the coset
// level: same subgroup types, different representatives, different
// verdicts.

/// A pin: a coaxial mate whose two axes are authored at `a_at` on the
/// A side and `b_at` on the B side, both along +z.
fn pin(
    a: RecipeNodeId,
    b: RecipeNodeId,
    a_at: [f64; 3],
    b_at: [f64; 3],
) -> Node<editor_core::ProfileProgram> {
    mate(
        a,
        b,
        MatePrimitive::Coaxial,
        AxisSense::Aligned,
        frame(a_at, [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
        frame(b_at, [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
        None,
    )
}

/// Two pins, inter-axis invariants MATCHING (both distances 1) but the
/// B-side pattern clocked 90° round: the table's entry 3
/// parallel-distinct row, which requires **prismatic**. Before the
/// MAJOR-1 fix this refused `Contradictory { mate_member_point_on_axis,
/// clash 1.414… }` — the candidate was in the held coset but was not a
/// witness of the intersection.
#[test]
fn row5b_two_pins_clocked_apart_but_invariant_matched_fold_to_prismatic() {
    let (doc, ids, _) = assembly("asm-r2a-row5b", 2);
    let (doc, _) = mint(
        doc,
        DocEdit::InsertNode {
            node: pin(ids[0], ids[1], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
        },
    );
    // A's second pin sits at +x; B's at +y — same radius, quarter turn.
    let (doc, second) = mint(
        doc,
        DocEdit::InsertNode {
            node: pin(ids[0], ids[1], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        },
    );
    let poses = solve_document(&doc, Tol::witness());
    let fault = poses
        .fault(second)
        .expect("a two-pin pair is UNDER")
        .clone();
    let editor_core::MateFault::Under { residual, .. } = &fault else {
        panic!("expected UNDER (prismatic along the shared axis), got {fault:?}");
    };
    assert_eq!(residual.name(), "prismatic");
    assert!(
        fault.to_string().contains("translation along [0, 0, 1]"),
        "the residual slides along the pins' shared axis: {fault}"
    );
}

/// The same shape at the COSET door, so the witness itself is pinned:
/// the folded representative must be the quarter turn that carries B's
/// pattern onto A's, not the identity the held mate happened to state.
#[test]
fn row5b_the_folded_representative_is_the_solved_clocking() {
    use editor_core::mate::coset::{Coset, Subgroup, intersect};
    use geom_core::linalg::{Affine3, Mat3, Point3, Vec3};
    use geom_core::predicate::Band;
    let band = Band::linear(Tol::witness()).expect("a band");
    let z = Vec3::new(0.0, 0.0, 1.0);
    // Pin 1 pins the shared axis through the origin; pin 2's A-side
    // axis is at +x while its representative carries B's at +y.
    let held = Coset {
        subgroup: Subgroup::Cylindrical {
            point: Point3::origin(),
            direction: z,
        },
        representative: Affine3::identity(),
    };
    let added = Coset {
        subgroup: Subgroup::Cylindrical {
            point: Point3::new(1.0, 0.0, 0.0),
            direction: z,
        },
        representative: Affine3::from_parts(Mat3::identity(), Vec3::new(1.0, -1.0, 0.0)),
    };
    let out = intersect(held, added, band, 1.0).expect("the pair is assemblable");
    assert_eq!(out.subgroup.name(), "prismatic");
    let turned = out
        .representative
        .transform_point(Point3::new(0.0, 1.0, 0.0));
    assert!(
        (turned - Point3::new(1.0, 0.0, 0.0)).norm() < 1e-12,
        "the witness carries B's second pin onto A's: {turned:?}"
    );
}

/// Invariants that do NOT match: A's pins are 1 apart, B's are 2. The
/// table says empty, and the refusal must quote the MEASURED clash —
/// the radius difference, 1.
#[test]
fn row5b_mismatched_inter_axis_invariants_refuse_contradictory() {
    let (doc, ids, _) = assembly("asm-r2a-row5b-mismatch", 2);
    let (doc, first) = mint(
        doc,
        DocEdit::InsertNode {
            node: pin(ids[0], ids[1], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
        },
    );
    let (doc, second) = mint(
        doc,
        DocEdit::InsertNode {
            node: pin(ids[0], ids[1], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]),
        },
    );
    let poses = solve_document(&doc, Tol::witness());
    let fault = poses.fault(second).expect("the pair refuses").clone();
    let editor_core::MateFault::Contradictory {
        held,
        added,
        predicate,
        clash,
    } = &fault
    else {
        panic!("expected CONTRADICTORY, got {fault:?}");
    };
    assert_eq!((*held, *added), (first, second), "both mates are named");
    assert_eq!(*predicate, "mate_member_point_on_axis");
    assert!(
        (clash.abs() - 1.0).abs() < 1e-9,
        "the clash IS the inter-axis invariant difference (2 − 1): {clash}"
    );
}

/// A rest plus two pins — A11 rule 1's own fully-determined territory,
/// and the shape MAJOR-1 falsely refused. The plate seats at the rest
/// and both pins land, so the fold is DETERMINED and the pose is the
/// seating height.
#[test]
fn row5b_a_rest_and_two_pins_determine_the_plate() {
    let (doc, ids, store) = assembly("asm-r2a-row5b-plate", 2);
    let (doc, _) = mint(
        doc,
        DocEdit::InsertNode {
            node: mate(
                ids[0],
                ids[1],
                MatePrimitive::PlanarRest { offset: -1.0 },
                AxisSense::Opposed,
                frame([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
                frame([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [1.0, 0.0, 0.0]),
                None,
            ),
        },
    );
    let (doc, _) = mint(
        doc,
        DocEdit::InsertNode {
            node: pin(ids[0], ids[1], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
        },
    );
    let (doc, _) = mint(
        doc,
        DocEdit::InsertNode {
            node: pin(ids[0], ids[1], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        },
    );
    let poses = solve_document(&doc, Tol::witness());
    assert_eq!(poses.fault(ids[1]), None, "rest + two pins determine");
    let relative = poses.relative(ids[1]).expect("a solved pose");
    // VALUE-exact, not bit-exact, and the difference is the point: this
    // pose comes out of the clocking solve (`atan2` then
    // `rotation_about`), where row 1's came out of the forced-rotation
    // arm with no trigonometry in it. A closed form through the
    // transcendentals leaves a rounding step — here the residue of
    // turning the rest's half-turn back, sin(π) ≈ 1.2e-16 off the
    // identity. D9 is untouched: the same expression yields the same
    // bits on every run, which is what determinism asks for, and the
    // pncad cross-process row proves it across processes.
    assert!(
        near(relative, Frame::translation([0.0, 0.0, 2.0]), 1e-12),
        "the plate seats at the rest, unturned (both patterns agree): {relative:?}"
    );
    let ev = run(&doc, &opts(store));
    assert!(matches!(ev.result(ids[1]), Some(NodeResult::Ok(_))));
}

// ---- Row 6: the A12 partition consequences ----

#[test]
fn row6a_mated_instances_share_an_a9_component() {
    let (doc, ids, _) = assembly("asm-r2a-row6a", 2);
    let apart = relative_freedom_components(&doc);
    assert_eq!(apart.len(), 2, "unmated instances are relatively free");
    let (doc, mate_id) = mint(
        doc,
        DocEdit::InsertNode {
            node: mate(
                ids[0],
                ids[1],
                MatePrimitive::Coaxial,
                AxisSense::Aligned,
                z_up(),
                z_up(),
                Some(0.0),
            ),
        },
    );
    let together = relative_freedom_components(&doc);
    assert_eq!(together.len(), 1, "the mate COUPLES the components");
    assert!(together[0].contains(&mate_id));
    assert_eq!(
        editor_core::reading_edges(&doc),
        vec![(mate_id, ids[0]), (mate_id, ids[1])],
        "the reading edges are recomputed from the name heads"
    );
}

#[test]
fn row6b_instances_keep_their_roots_across_mate_insert_and_delete() {
    let (doc, ids, _) = assembly("asm-r2a-row6b", 2);
    assert_eq!(doc.roots(), &ids[..], "both instances are roots");
    let (doc, mate_id) = mint(
        doc,
        DocEdit::InsertNode {
            node: mate(
                ids[0],
                ids[1],
                MatePrimitive::Coaxial,
                AxisSense::Aligned,
                z_up(),
                z_up(),
                Some(0.0),
            ),
        },
    );
    assert_eq!(
        doc.roots(),
        &[ids[0], ids[1], mate_id][..],
        "no tip transfer: the mate APPENDS as an ordinary non-body root"
    );
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: mate_id });
    assert_eq!(doc.roots(), &ids[..], "and leaves them as it found them");
}

#[test]
fn row6c_the_gather_ignores_the_mate_root() {
    let (doc, ids, store, _) = determined_pair();
    let ev = run(&doc, &opts(store));
    let mates: Vec<RecipeNodeId> = doc
        .order()
        .iter()
        .copied()
        .filter(|&id| matches!(doc.node(id), Some(Node::Mate { .. })))
        .collect();
    for &m in &mates {
        assert!(doc.roots().contains(&m), "a mate IS listed as a root");
        assert!(matches!(
            ev.result(m),
            Some(NodeResult::Ok(v)) if matches!(v.payload, editor_core::ValuePayload::Mate(_))
        ));
    }
    let body = product(&doc, &ev, Tol::witness()).expect("the product gathers");
    let volume = topo::mass_properties(&body, Tol::witness())
        .expect("props")
        .volume;
    assert!(
        (volume - 2.0).abs() < 1e-9,
        "the gather took the two instances and ignored the mates: {volume}"
    );
    let _ = ids;
}

#[test]
fn row6d_a_dangling_head_contributes_no_edge_and_the_solve_refuses_typed() {
    let (doc, ids, store) = assembly("asm-r2a-row6d", 2);
    let (doc, mate_id) = mint(
        doc,
        DocEdit::InsertNode {
            node: mate(
                ids[0],
                ids[1],
                MatePrimitive::Coaxial,
                AxisSense::Aligned,
                z_up(),
                z_up(),
                Some(0.0),
            ),
        },
    );
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: ids[1] });
    assert_eq!(
        editor_core::reading_edges(&doc),
        vec![(mate_id, ids[0])],
        "the stranded head contributes NO edge (N5)"
    );
    let fault = solve_document(&doc, Tol::witness())
        .fault(mate_id)
        .expect("the solve refuses")
        .clone();
    let editor_core::MateFault::DanglingHead { head, .. } = &fault else {
        panic!("expected DanglingHead, got {fault:?}");
    };
    assert_eq!(*head, ids[1]);
    assert!(fault.to_string().contains("rebind"), "{fault}");
    let ev = run(&doc, &opts(store));
    assert!(matches!(
        mate_fault(&ev, mate_id),
        editor_core::MateFault::DanglingHead { .. }
    ));
}

#[test]
fn row6e_a_non_tree_mate_declares_rather_than_determining() {
    let (doc, ids, _, rest) = determined_pair();
    let roles = solve_document(&doc, Tol::witness());
    let mates: Vec<RecipeNodeId> = doc
        .order()
        .iter()
        .copied()
        .filter(|&id| matches!(doc.node(id), Some(Node::Mate { .. })))
        .collect();
    for &m in &mates {
        assert_eq!(
            roles.role(m),
            Some(MateRole::Determining),
            "both mates ride the pair's ONE tree edge"
        );
    }
    let _ = (ids, rest);

    // A third instance mated to both makes a loop: the loop-closing
    // pair is not in the spanning tree, so its mate DECLARES.
    let (doc, ids3, _) = assembly("asm-r2a-row6e", 3);
    let mut doc = doc;
    let mut made = Vec::new();
    for (a, b) in [(0, 1), (1, 2), (0, 2)] {
        let (next, id) = mint(
            doc,
            DocEdit::InsertNode {
                node: mate(
                    ids3[a],
                    ids3[b],
                    MatePrimitive::FrameCoincidence,
                    AxisSense::Aligned,
                    z_up(),
                    z_up(),
                    None,
                ),
            },
        );
        doc = next;
        made.push(id);
    }
    let roles = solve_document(&doc, Tol::witness());
    // The spanning tree leaves the gauge for each neighbour in
    // document order — (0,1) then (0,2) — so the pair that closes the
    // loop is (1,2), whatever order the mates were authored in.
    assert_eq!(roles.role(made[0]), Some(MateRole::Determining));
    assert_eq!(roles.role(made[2]), Some(MateRole::Determining));
    assert_eq!(
        roles.role(made[1]),
        Some(MateRole::Declaring),
        "the loop closer solved nothing — R2-b verifies it"
    );
}

// ---- Row 6f–6j: A12's repair path, and the three doors ----
// ---- that keep a head honest                              ----

/// A12: *"A dangling head (N5) contributes no edge until `Rebind`"* —
/// the rebind of a stranded head IS the repair, and the reading edge
/// comes back with it. Here the head is the document's ONLY reference
/// to the stranded name, so the edit's own reference count is what
/// decides whether the repair runs at all.
#[test]
fn row6f_rebind_repairs_a_mate_head_that_is_the_only_reference() {
    let (doc, ids, _) = assembly("asm-r2a-rebind-only-ref", 3);
    let (doc, mate_id) = mint(
        doc,
        DocEdit::InsertNode {
            node: mate(
                ids[0],
                ids[1],
                MatePrimitive::Coaxial,
                AxisSense::Aligned,
                z_up(),
                z_up(),
                Some(0.0),
            ),
        },
    );
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: ids[1] });
    assert_eq!(
        editor_core::reading_edges(&doc),
        vec![(mate_id, ids[0])],
        "the stranded head contributes NO edge (N5)"
    );
    let applied = doc
        .apply(
            &DocEdit::Rebind {
                from: in_part(ids[1], RecipeNodeId(1)),
                to: in_part(ids[2], RecipeNodeId(1)),
            },
            Tol::witness(),
        )
        .expect("a mate head is a rebind site");
    assert_eq!(
        editor_core::reading_edges(&applied.doc),
        vec![(mate_id, ids[0]), (mate_id, ids[2])],
        "the repaired head reads through the instance it now names"
    );
    assert!(
        applied.record.structural,
        "a mate payload moved, so its content key moved with it"
    );
    assert!(
        !matches!(
            solve_document(&applied.doc, Tol::witness()).fault(mate_id),
            Some(editor_core::MateFault::DanglingHead { .. })
        ),
        "and the solve no longer refuses the head"
    );
}

/// The same repair with a `Declare` referencing the stranded name too:
/// the declaration's rewrite is what makes the edit acceptable, so a
/// mate head skipped here is skipped SILENTLY — the loud arm never
/// fires.
#[test]
fn row6g_rebind_repairs_a_mate_head_beside_a_declare_reference() {
    let (doc, ids, _) = assembly("asm-r2a-rebind-with-declare", 3);
    let (doc, mate_id) = mint(
        doc,
        DocEdit::InsertNode {
            node: mate(
                ids[0],
                ids[1],
                MatePrimitive::Coaxial,
                AxisSense::Aligned,
                z_up(),
                z_up(),
                Some(0.0),
            ),
        },
    );
    let (doc, declare_id) = mint(
        doc,
        DocEdit::InsertNode {
            node: Node::Declare {
                pairs: vec![(
                    (
                        in_part(ids[1], RecipeNodeId(1)),
                        in_part(ids[0], RecipeNodeId(1)),
                    ),
                    ContactClass::Rest,
                )],
            },
        },
    );
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: ids[1] });
    let applied = doc
        .apply(
            &DocEdit::Rebind {
                from: in_part(ids[1], RecipeNodeId(1)),
                to: in_part(ids[2], RecipeNodeId(1)),
            },
            Tol::witness(),
        )
        .expect("the declare pair alone makes this a rebind site");
    let Some(Node::Declare { pairs }) = applied.doc.node(declare_id) else {
        panic!("the declare is still there");
    };
    assert_eq!(
        pairs[0].0.0,
        in_part(ids[2], RecipeNodeId(1)),
        "the declaration was rewritten"
    );
    assert_eq!(
        editor_core::reading_edges(&applied.doc),
        vec![(mate_id, ids[0]), (mate_id, ids[2])],
        "and so was the mate head — one rebind repairs every site, or none of them"
    );
}

/// The insert door's own claim (`Node::named_nodes`): a mate's two
/// heads carry the `Declare` carve-out, so a head naming a node that
/// never existed is a typo and is refused THERE — the only door that
/// checks.
#[test]
fn row6h_the_insert_door_refuses_a_mate_head_naming_no_node() {
    let (doc, ids, _) = assembly("asm-r2a-mate-insert-door", 1);
    let ghost = RecipeNodeId(9_999);
    let err = doc
        .apply(
            &DocEdit::InsertNode {
                node: mate(
                    ids[0],
                    ghost,
                    MatePrimitive::Coaxial,
                    AxisSense::Aligned,
                    z_up(),
                    z_up(),
                    Some(0.0),
                ),
            },
            Tol::witness(),
        )
        .expect_err("the head names no node");
    assert!(
        matches!(&err, EditError::DeclareNamesMissingNode { name } if name.node == ghost),
        "{err:?}"
    );
}

/// A saved file is DATA: a mate head naming an id past the mint counter
/// is as corrupt as a `Declare` pair naming one, and worse to let in —
/// `Rebind`'s source door refuses a never-minted id, so the document
/// would load unrepairable.
#[test]
fn row6i_the_load_check_refuses_a_mate_head_past_the_mint_counter() {
    let (doc, ids, _) = assembly("asm-r2a-mate-wire-id", 3);
    let (doc, mate_id) = mint(
        doc,
        DocEdit::InsertNode {
            node: mate(
                ids[0],
                ids[2],
                MatePrimitive::Coaxial,
                AxisSense::Aligned,
                z_up(),
                z_up(),
                Some(0.0),
            ),
        },
    );
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    // Doctored BY PATH, not by position: the `b` head of this mate,
    // reached through the wire's own structure, so a fixture or
    // field-order change breaks the probe instead of silently moving it
    // onto an unrelated id.
    let split = text.find('{').expect("the JSON body follows the header");
    let (header, body) = text.split_at(split);
    let mut wire: serde_json::Value = serde_json::from_str(body).expect("the body parses");
    let head = &mut wire["snapshot"]["nodes"][mate_id.0.to_string()]["Mate"]["b"];
    assert_eq!(
        head["node"],
        serde_json::json!(ids[2].0),
        "the probe is aimed at the `b` head"
    );
    head["node"] = serde_json::json!(99);
    let doctored = format!("{header}{wire}");
    match load(&doctored, Tol::witness()) {
        Err(editor_core::PersistError::Snapshot(editor_core::SnapshotError::IdBeyondCounter {
            id,
            ..
        })) => assert_eq!(id, RecipeNodeId(99)),
        other => panic!("expected IdBeyondCounter, got {other:?}"),
    }
}

/// The name-level edit door (`apply_with_names`, PR 3's R6 obligation)
/// reads a mate's heads under the rule it has always applied to a
/// `Declare` pair: checkable exactly when the minting node evaluated
/// `Ok`, deferred otherwise. An instance-qualified head the tables
/// carry passes; a role the part's product does not have is refused
/// there rather than at the solve.
#[test]
fn row6j_the_name_door_reads_a_mates_heads_like_a_declare_pair() {
    let (doc, ids, store) = assembly("asm-r2a-mate-name-door", 2);
    let ev = run(&doc, &opts(store));
    assert!(
        editor_core::apply_with_names(
            &doc,
            &DocEdit::InsertNode {
                node: mate(
                    ids[0],
                    ids[1],
                    MatePrimitive::Coaxial,
                    AxisSense::Aligned,
                    z_up(),
                    z_up(),
                    Some(0.0),
                ),
            },
            &ev,
            Tol::witness(),
        )
        .is_ok(),
        "the instance-qualified heads resolve in the instance's own table"
    );
    // The same head, wearing a role the part product has no entity for.
    let bogus = StableName {
        kind: EntityKind::Face,
        node: ids[1],
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId(1),
                path: vec![RoleSeg::Lateral(editor_core::ProfileEdgeRef {
                    loop_index: 7,
                    segment: 7,
                })],
            }),
        }],
    };
    let Node::Mate {
        a,
        class,
        alignment,
        ..
    } = mate(
        ids[0],
        ids[1],
        MatePrimitive::Coaxial,
        AxisSense::Aligned,
        z_up(),
        z_up(),
        Some(0.0),
    )
    else {
        panic!("the fixture builds a mate");
    };
    let err = editor_core::apply_with_names(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Mate {
                a,
                b: bogus.clone(),
                class,
                alignment,
            },
        },
        &ev,
        Tol::witness(),
    )
    .unwrap_err();
    assert_eq!(
        err,
        EditError::NameUnresolvedInEvaluation { name: bogus },
        "the mate head is checkable, so it is checked"
    );
}

// ---- Row 7: every refusal arm ----

#[test]
fn row7a_a_standalone_clocking_refuses_typed() {
    let (doc, ids, _) = assembly("asm-r2a-row7a", 2);
    let (doc, mate_id) = mint(
        doc,
        DocEdit::InsertNode {
            node: mate(
                ids[0],
                ids[1],
                MatePrimitive::Clocking,
                AxisSense::Aligned,
                z_up(),
                z_up(),
                Some(0.5),
            ),
        },
    );
    let fault = solve_document(&doc, Tol::witness())
        .fault(mate_id)
        .expect("refuses")
        .clone();
    let editor_core::MateFault::TableLacks { what, .. } = &fault else {
        panic!("expected TableLacks, got {fault:?}");
    };
    assert!(what.contains("standalone clocking"), "{what}");
    assert!(fault.to_string().contains("coset table"), "{fault}");
}

#[test]
fn row7b_a_clocking_rider_on_a_planar_rest_refuses_typed() {
    let (doc, ids, _) = assembly("asm-r2a-row7b", 2);
    let (doc, mate_id) = mint(
        doc,
        DocEdit::InsertNode {
            node: mate(
                ids[0],
                ids[1],
                MatePrimitive::PlanarRest { offset: 0.0 },
                AxisSense::Opposed,
                z_up(),
                z_up(),
                Some(0.5),
            ),
        },
    );
    let fault = solve_document(&doc, Tol::witness())
        .fault(mate_id)
        .expect("refuses")
        .clone();
    assert!(
        matches!(&fault, editor_core::MateFault::TableLacks { what, .. }
                 if what.contains("planar rest")),
        "{fault:?}"
    );
}

/// The `Fit` class is not representable in this kernel — the variant is
/// specified and not built — so the refusal is demonstrated at the WIRE
/// door, where an unknown spelling arrives. The sentence it says is
/// [`topo::FIT_DEFERRAL`], VERBATIM, and it is the same sentence the
/// solve door says when a future class reaches it.
#[test]
fn row7c_an_unadmitted_class_refuses_naming_the_fit_deferral() {
    let (doc, ids, _) = assembly("asm-r2a-row7c", 2);
    let (doc, _) = step(
        doc,
        DocEdit::InsertNode {
            node: mate(
                ids[0],
                ids[1],
                MatePrimitive::Coaxial,
                AxisSense::Aligned,
                z_up(),
                z_up(),
                Some(0.0),
            ),
        },
    );
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let doctored = text.replacen("\"rest\"", "\"fit\"", 1);
    let message = match load(&doctored, Tol::witness()) {
        Err(e) => e.to_string(),
        Ok(_) => panic!("an unknown class spelling must refuse"),
    };
    assert!(
        message.contains(topo::FIT_DEFERRAL),
        "the refusal quotes the kernel's deferral sentence verbatim: {message}"
    );
    assert!(message.contains("fit"), "{message}");
}

/// The typed escalation: a case split constructed to land INSIDE the
/// ambiguity band at the run's own ε, so the row holds at every ε CI
/// sweeps.
#[test]
fn row7d_an_in_band_case_split_escalates_typed() {
    let eps = geom_core::Tol::witness().get().eps;
    // Two planar rests whose normals differ by an angle whose induced
    // displacement at the mate's lever arm is 3ε — definitely inside
    // (ε, Kε) for the default K = 10.
    let tilt = 3.0 * eps;
    let (doc, ids, _) = assembly("asm-r2a-row7d", 2);
    let mut doc = doc;
    let mut mates = Vec::new();
    for axis in [[0.0, 0.0, 1.0], [tilt, 0.0, 1.0]] {
        let (next, id) = mint(
            doc,
            DocEdit::InsertNode {
                node: mate(
                    ids[0],
                    ids[1],
                    MatePrimitive::PlanarRest { offset: 0.0 },
                    AxisSense::Opposed,
                    frame([0.0, 0.0, 0.0], axis, [0.0, 1.0, 0.0]),
                    frame([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 1.0, 0.0]),
                    None,
                ),
            },
        );
        doc = next;
        mates.push(id);
    }
    let fault = solve_document(&doc, Tol::witness())
        .fault(mates[1])
        .expect("the band escalates")
        .clone();
    let editor_core::MateFault::Indeterminate { diag, .. } = &fault else {
        panic!("expected the typed escalation, got {fault:?}");
    };
    assert_eq!(
        diag.predicate,
        Some("mate_axes_parallel"),
        "the escalation NAMES the predicate that could not decide"
    );
    assert!(
        fault.to_string().contains("could not be decided"),
        "{fault}"
    );
}

#[test]
fn row7e_a_self_mate_refuses_naming_the_instance_it_names_twice() {
    let (doc, ids, _) = assembly("asm-r2a-row7e", 2);
    let (doc, mate_id) = mint(
        doc,
        DocEdit::InsertNode {
            node: mate(
                ids[0],
                ids[0],
                MatePrimitive::Coaxial,
                AxisSense::Aligned,
                z_up(),
                z_up(),
                None,
            ),
        },
    );
    let fault = solve_document(&doc, Tol::witness())
        .fault(mate_id)
        .expect("refuses")
        .clone();
    assert!(
        matches!(&fault, editor_core::MateFault::SelfMate { instance, .. } if *instance == ids[0]),
        "{fault:?}"
    );
    assert!(fault.to_string().contains("a PAIR"), "{fault}");
}

#[test]
fn row7f_a_non_finite_alignment_refuses_at_the_edit_door() {
    let (doc, ids, _) = assembly("asm-r2a-row7f", 2);
    let refusal = apply(
        &doc,
        &DocEdit::InsertNode {
            node: mate(
                ids[0],
                ids[1],
                MatePrimitive::Coaxial,
                AxisSense::Aligned,
                frame([f64::NAN, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
                z_up(),
                None,
            ),
        },
        Tol::witness(),
    )
    .expect_err("a non-finite alignment never enters the document");
    assert!(
        matches!(refusal, editor_core::EditError::NonFiniteAlignment { .. }),
        "{refusal:?}"
    );
}
