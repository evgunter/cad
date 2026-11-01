//! **Instantiation nests to its bound and refuses one past it, on the
//! smallest stack a door runs on.**
//!
//! A chain of documents, each instantiating the one below over a leaf
//! part, is evaluated on a thread whose stack is the wasm32 build's
//! (one mebibyte, the smallest any evaluating door runs on). How deep
//! the chain nests costs that stack nothing, so the chain at the bound
//! evaluates and the chain one past it refuses with
//! [`PartFault::DepthExceeded`] rather than killing the process.
//!
//! The bound is written here as the number the refusal's own sentence
//! states, and the one-past row reads it back out of that sentence, so
//! the two cannot drift apart silently.
//!
//! The descent below the top runs bottom-up, entering every part a
//! document instantiates before evaluating it. The rows after the
//! bound's pin what that must not change: a loop is still named as a
//! loop at the bound, every row a nested document reads is the one its
//! own evaluation produces, both askers below the top find the rows
//! the descent entered, and a part no instance asks for is evaluated
//! without its failure reaching anything.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::{Dimension, Formula};

use std::collections::BTreeMap;

use editor_core::{
    CapEnd, ContentPin, Coord, DocRef, DocumentId, EvalOptions, Node, NodeErrorKind, NodeResult,
    PartFault, PartResolver, ProfileDoc, RecipeNodeId, ResolveFailure, ResolveFault, TargetAxis,
    product,
};
use fixture::resolver::{PartStore, with_resolver};
use fixture::{insert, len, run, square};
use geom_core::Tol;
use std::sync::Arc;
use test_utils::own_thread::on_the_smallest_stack;

/// The deepest nesting that evaluates: `PartFault::DepthExceeded`'s
/// sentence names it.
const BOUND: usize = 1024;

/// The leaf part: a block with a boss unioned onto it, so the leaf's
/// own evaluation runs a boolean on top of whatever the chain costs.
fn leaf() -> ProfileDoc {
    leaf_labelled("part-depth-leaf").0
}

/// [`leaf`] under its own identity, so a store holds it as a distinct
/// part, its union placed in its world; and the block extrude whose
/// caps the leaf's product names, as that placement's copy.
fn leaf_labelled(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = insert(
        doc,
        Node::Profile(fixture::desc(vec![square(0.0, 0.0, 1.0)])),
    );
    let (doc, block) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let (doc, boss_profile) = insert(
        doc,
        Node::Profile(fixture::desc(vec![square(0.2, 0.2, 0.25)])),
    );
    let (doc, boss) = insert(
        doc,
        Node::Extrude {
            profile: boss_profile.into(),
            distance: len(1.0),
        },
    );
    // The boss stands halfway up the block.
    let (doc, boss) = fixture::slid(doc, block, boss, [0.0, 0.0, 0.5]);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![block.into(), boss.into()]),
        },
    );
    (fixture::place(doc, union).0, block)
}

/// The cap `end` of a [`leaf_labelled`] part's `block`, as the part's
/// product names it: the union carries it from its member read of the
/// block, under its one placement's copy (A10).
fn leaf_cap(leaf: &ProfileDoc, block: RecipeNodeId, end: CapEnd) -> editor_core::FaceName {
    // The world copy: the placement whose shape is the union (the
    // boss's copy against the block is the other).
    let worlds: Vec<(RecipeNodeId, RecipeNodeId)> = leaf
        .placements()
        .into_iter()
        .filter_map(|copy| match leaf.node(copy) {
            Some(Node::Place { source, .. }) => leaf
                .operation_of(source.read)
                .filter(|&read| matches!(leaf.node(read), Some(Node::Union { .. })))
                .map(|union| (copy, union)),
            _ => None,
        })
        .collect();
    let [(_copy, union)] = worlds[..] else {
        panic!("a leaf places its union once");
    };
    let read = leaf.output(block, 0).expect("the block defines its output");
    let cap = editor_core::StableName {
        kind: editor_core::EntityKind::Face,
        node: union,
        path: vec![editor_core::RoleSeg::From {
            read,
            of: editor_core::StableName {
                kind: editor_core::EntityKind::Face,
                node: block,
                path: vec![editor_core::RoleSeg::Cap(end)],
            }
            .into(),
        }],
    };
    editor_core::FaceName::new(cap).expect("a cap is a face")
}

/// A document whose one instance, of `below`, is loose: what the
/// descent refuses is asked of it, and nothing of its place.
fn instantiating(label: &str, below: DocRef) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    insert(doc, crate::fixture::mated_instance(below))
}

/// `instance` standing in the world where its body stands
/// ([`editor_core::stand`]), which reads nothing of its part.
fn world_pinned(doc: ProfileDoc, instance: RecipeNodeId) -> ProfileDoc {
    fixture::written(doc, instance, editor_core::stand())
}

/// [`instantiating`], its instance [`world_pinned`].
fn instantiating_pinned(label: &str, below: DocRef) -> (ProfileDoc, RecipeNodeId) {
    let (doc, instance) = instantiating(label, below);
    (world_pinned(doc, instance), instance)
}

/// A store holding the leaf and `levels` documents above it, each
/// pinning an instance of the one below to its world, and the
/// references it answers — the leaf's first, the topmost document's
/// last.
///
/// Each level stands its instance where its body does by the identity
/// bundle ([`editor_core::stand`]), which reads nothing of its part, so
/// building the chain evaluates no part.
fn chain(levels: usize) -> (PartStore, Vec<DocRef>) {
    let mut store = PartStore::new();
    let mut refs = vec![store.insert(leaf(), Tol::witness())];
    for level in 1..=levels {
        let below = *refs.last().expect("the leaf is stored");
        let (doc, instance) = instantiating(&format!("part-depth-level-{level}"), below);
        refs.push(store.insert(world_pinned(doc, instance), Tol::witness()));
    }
    (store, refs)
}

fn resolving(store: PartStore) -> EvalOptions {
    EvalOptions {
        resolver: Some(Arc::new(store) as Arc<dyn PartResolver>),
        ..EvalOptions::default()
    }
}

/// The refusal a failed node raised. A value is not printed: its
/// names nest as deep as the chain, and this runs on a small stack.
fn failed(result: Option<&NodeResult<f64>>) -> &NodeErrorKind {
    match result {
        Some(NodeResult::Failed(e)) => &e.kind,
        Some(NodeResult::Ok(_)) => panic!("expected a failed node, got a value"),
        _ => panic!("expected a failed node, got none"),
    }
}

/// The deepest level a part refusal carries: how many documents down
/// it was raised, its refusal, and its line as a surface draws it.
fn deepest(kind: &NodeErrorKind) -> (usize, &NodeErrorKind, String) {
    let levels: Vec<_> = kind.carried_chain().collect();
    let last = levels.last().expect("a part refusal carries its chain");
    (levels.len(), last.refusal.kind(), last.line())
}

/// **One past the bound refuses typed** where the evaluation descends,
/// on the wasm32 stack. The refusal is raised `BOUND` documents down, is
/// `DepthExceeded`, and its sentence states the bound and meets the
/// refusal standard.
#[test]
fn a_chain_one_past_the_bound_refuses_depth_exceeded_on_the_smallest_stack() {
    on_the_smallest_stack(|| {
        // The top document sits above BOUND + 1 nested documents: the
        // leaf and BOUND levels.
        let (store, refs) = chain(BOUND);
        let top_ref = *refs.last().expect("the top level is stored");
        let (top, instance) = instantiating("part-depth-past-top", top_ref);
        let opts = resolving(store);

        let ev = run(&top, &opts);
        let (levels, last, line) = deepest(failed(ev.result(instance)));
        assert_eq!(
            levels, BOUND,
            "the refusal is raised by the document at the bound, carried up through every \
             document above it"
        );
        assert!(
            matches!(
                last,
                NodeErrorKind::Part {
                    fault: PartFault::DepthExceeded,
                    ..
                }
            ),
            "the chain ends in DepthExceeded: {line}"
        );
        assert!(
            line.contains(&format!("deeper than {BOUND} documents")),
            "the sentence states the bound this suite tests: {line}"
        );
        let problems = test_utils::refusal::problems("DepthExceeded", &line, &[], false);
        assert!(problems.is_empty(), "{problems:#?}");
    });
}

/// **The recourse holds, and the bound is the bound**: the refused
/// assembly flattened by one level — its top instantiating what its
/// child instantiated — nests exactly `BOUND` documents deep, and
/// evaluates on the wasm32 stack to the leaf's own volume.
#[test]
fn the_refused_chain_flattened_by_one_level_evaluates_at_the_bound() {
    on_the_smallest_stack(|| {
        let (store, refs) = chain(BOUND);
        // One level flatter: the top skips the topmost stored level.
        let flattened_ref = refs[refs.len() - 2];
        let opts = resolving(store);
        let (top, instance) = instantiating_pinned("part-depth-flattened-top", flattened_ref);
        let ev = run(&top, &opts);
        assert!(
            matches!(ev.result(instance), Some(NodeResult::Ok(_))),
            "a chain at the bound evaluates"
        );

        let body = product(&top, &ev, Tol::witness()).expect("the chain's product gathers");
        let leaf = leaf();
        let leaf_body = product(&leaf, &run(&leaf, &EvalOptions::default()), Tol::witness())
            .expect("the leaf's product gathers");
        let volume = |b: &topo::Body<f64>| {
            topo::test_support::pinned_volume(
                "mass properties",
                &topo::test_support::finished(
                    "the measured body",
                    topo::Body::clone(b),
                    Tol::witness(),
                ),
                Tol::witness(),
            )
            .volume
        };
        assert_eq!(
            volume(&body).to_bits(),
            volume(&leaf_body).to_bits(),
            "BOUND instances pinned at the identity carry the leaf through bit for bit"
        );
    });
}

/// A resolver that checks no pins: it answers whatever is stored under
/// the reference, so a store can hold a loop or swap a part's content
/// under the reference an instance was authored against.
#[derive(Default, Debug)]
struct Unpinned(BTreeMap<DocRef, ProfileDoc>);

impl PartResolver for Unpinned {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        self.0.get(doc_ref).cloned().ok_or(ResolveFailure {
            fault: ResolveFault::Unresolved,
            message: "the store holds no such document".into(),
        })
    }
}

impl Unpinned {
    fn options(self) -> EvalOptions {
        EvalOptions {
            resolver: Some(Arc::new(self) as Arc<dyn PartResolver>),
            ..EvalOptions::default()
        }
    }
}

/// A reference by label alone, for a store that checks no pins.
fn unpinned_ref(label: &str) -> DocRef {
    DocRef {
        id: DocumentId::derive(label),
        pin: ContentPin([7; 32]),
    }
}

/// A document whose nodes instantiate `below`, in order, each loose.
fn instantiating_all(label: &str, below: &[DocRef]) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for b in below {
        let (d, id) = insert(doc, crate::fixture::mated_instance(*b));
        doc = d;
        ids.push(id);
    }
    (crate::fixture::assemble_world_copies(doc), ids)
}

/// [`instantiating_all`], each instance [`world_pinned`].
fn instantiating_all_pinned(label: &str, below: &[DocRef]) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, ids) = instantiating_all(label, below);
    let doc = ids.iter().fold(doc, |doc, &id| world_pinned(doc, id));
    (doc, ids)
}

/// A refusal as a surface draws it: its own sentence, then each
/// carried level's line.
fn drawn(kind: &NodeErrorKind) -> Vec<String> {
    std::iter::once(kind.to_string())
        .chain(kind.carried_chain().map(|level| level.line()))
        .collect()
}

/// **The loop check runs before the depth check.** A chain of
/// documents, each pinning its instance of the next to its world, whose
/// last document, `BOUND` documents down, instantiates the first again is
/// refused as the loop it is; the same loop closing one document lower
/// is past the bound first, and refuses `DepthExceeded`.
#[test]
fn a_loop_closing_at_the_bound_is_named_as_a_loop_and_one_past_it_is_too_deep() {
    on_the_smallest_stack(|| {
        for (documents, expect_loop) in [(BOUND, true), (BOUND + 1, false)] {
            let refs: Vec<DocRef> = (1..=documents)
                .map(|i| unpinned_ref(&format!("part-depth-loop-{i}")))
                .collect();
            let mut store = Unpinned::default();
            for (i, r) in refs.iter().enumerate() {
                let next = refs.get(i + 1).copied().unwrap_or(refs[0]);
                let (doc, _) =
                    instantiating_all_pinned(&format!("part-depth-loop-{}", i + 1), &[next]);
                store.0.insert(*r, doc);
            }
            let (top, ids) = instantiating_all("part-depth-loop-top", &[refs[0]]);
            let ev = run(&top, &store.options());
            let (levels, last, line) = deepest(failed(ev.result(ids[0])));
            if expect_loop {
                let NodeErrorKind::Part {
                    fault: PartFault::ReferenceCycle { cycle },
                    ..
                } = last
                else {
                    panic!("a loop closing at the bound is named as a loop: {line}");
                };
                assert_eq!(
                    (levels, cycle.len()),
                    (BOUND, BOUND + 1),
                    "raised by the document at the bound, naming every document of the loop \
                     and its repeat"
                );
            } else {
                assert!(
                    matches!(
                        last,
                        NodeErrorKind::Part {
                            fault: PartFault::DepthExceeded,
                            ..
                        }
                    ),
                    "a loop closing past the bound is past the bound first: {line}"
                );
                assert_eq!(levels, BOUND, "raised by the document at the bound");
            }
        }
    });
}

/// **Every row a nested document reads is the one its own evaluation
/// produces.** Below the top, `mixed` pins to its world a healthy
/// part twice, a part whose own part is missing, a missing part, and
/// itself; each is decided against `mixed`'s chain before `mixed`
/// evaluates. The refusal the top carries up is drawn exactly as
/// `mixed`'s own evaluation draws the node it names, and the healthy
/// part is evaluated once for its two instances.
#[test]
fn below_the_top_a_documents_rows_are_the_ones_its_own_evaluation_produces() {
    let mut store = Unpinned::default();
    let healthy = unpinned_ref("part-descent-healthy");
    store.0.insert(healthy, leaf());
    let hollow = unpinned_ref("part-descent-hollow");
    let (hollow_doc, _) = instantiating_all_pinned(
        "part-descent-hollow",
        &[unpinned_ref("part-descent-hollow-missing")],
    );
    store.0.insert(hollow, hollow_doc);
    let mixed = unpinned_ref("part-descent-mixed");
    let (mixed_doc, mixed_ids) = instantiating_all(
        "part-descent-mixed",
        &[
            healthy,
            healthy,
            hollow,
            unpinned_ref("part-descent-missing"),
            mixed,
        ],
    );
    // Each copy in `mixed`'s world: the healthy ones pinned over their
    // geometry at the identity, the rest nominally.
    let healthy_only = {
        let mut only = Unpinned::default();
        only.0.insert(healthy, leaf());
        only.options()
    };
    let mixed_doc = mixed_ids[..2].iter().fold(mixed_doc, |doc, &id| {
        fixture::pinned(doc, id, &healthy_only)
    });
    let mixed_doc = mixed_ids[2..]
        .iter()
        .fold(mixed_doc, |doc, &id| world_pinned(doc, id));
    store.0.insert(mixed, mixed_doc.clone());
    let (top, ids) = instantiating_all("part-descent-mixed-top", &[mixed]);
    let opts = store.options();

    let ev = run(&top, &opts);
    let refused = failed(ev.result(ids[0]));
    let level = refused
        .carried_chain()
        .next()
        .expect("the instance carries the part's failed piece");
    let own = run(&mixed_doc, &opts);
    let at_the_top = failed(own.result(level.node));
    assert_eq!(
        drawn(level.refusal.kind()),
        drawn(at_the_top),
        "the failed piece the top carries is drawn as the part's own evaluation draws it"
    );
    assert_eq!(
        ev.part_evaluations, 3,
        "`mixed`, the healthy part once for both its instances, and the hollow part; the \
         missing parts and the loop refuse before any evaluation"
    );
}

/// `leaf` stored, with its reference.
fn stored(store: &mut PartStore, leaf: (ProfileDoc, RecipeNodeId)) -> Stored {
    (store.insert(leaf.0.clone(), Tol::witness()), leaf)
}

/// A stored leaf: its reference, and its document with its block.
type Stored = (DocRef, (ProfileDoc, RecipeNodeId));

/// `label`'s document with an instance of `first` pinned to its world
/// at the identity (`authoring` resolving it), and an instance of
/// `second`, loose until a constraint is added to it.
fn first_pinned(
    label: &str,
    first: DocRef,
    second: DocRef,
    authoring: &EvalOptions,
) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, mut ids) = instantiating_all(label, &[first]);
    let doc = fixture::pinned(doc, ids[0], authoring);
    let (doc, placed) = insert(doc, fixture::mated_instance(second));
    ids.push(placed);
    (crate::fixture::assemble_world_copies(doc), ids)
}

/// A document instantiating `first` and `second` — each a reference
/// with the block its caps are named on — `first` pinned to the world
/// and `second` seated on its top cap, square, by a rest and a zero
/// value on each freedom it leaves.
fn mated(
    label: &str,
    (first, (first_doc, first_block)): Stored,
    (second, (second_doc, second_block)): Stored,
    authoring: &EvalOptions,
) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, ids) = first_pinned(label, first, second, authoring);
    let mate = fixture::rest(
        fixture::head_at(
            ids[1],
            leaf_cap(&second_doc, second_block, CapEnd::Start).into_name(),
        ),
        fixture::head_at(
            ids[0],
            leaf_cap(&first_doc, first_block, CapEnd::End).into_name(),
        ),
    );
    let (doc, _mate) = crate::fixture::mated(
        doc,
        ids[1],
        mate,
        &[
            (
                Coord::Slide(TargetAxis::X),
                Formula::zero(Dimension::Length),
            ),
            (
                Coord::Slide(TargetAxis::Y),
                Formula::zero(Dimension::Length),
            ),
            (Coord::SPIN, Formula::zero(Dimension::Angle)),
        ],
    );
    (doc, ids)
}

/// A document instantiating `first` — a reference with the block its
/// caps are named on — pinned to the world, and `second` placed by one
/// rest of its own copy's bottom cap, as `second`'s leaf names it, on
/// `first`'s top cap.
fn rested_on_first(
    label: &str,
    (first, (first_doc, first_block)): Stored,
    (second, (second_doc, second_block)): Stored,
    authoring: &EvalOptions,
) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, ids) = first_pinned(label, first, second, authoring);
    let mate = fixture::rest(
        fixture::head_at(
            ids[1],
            leaf_cap(&second_doc, second_block, CapEnd::Start).into_name(),
        ),
        fixture::head_at(
            ids[0],
            leaf_cap(&first_doc, first_block, CapEnd::End).into_name(),
        ),
    );
    let (doc, _) = fixture::constrain_at(doc, ids[1], mate);
    (doc, ids)
}

/// **Below the top, both askers find the rows the descent entered.** A
/// nested document with two mated instances asks its cache twice per
/// part — the mate solve's reach, then each instance's own op — and
/// every ask is one the descent entered, so the nested document
/// evaluates exactly as it does at the top. An ask that escaped the
/// census would refuse `NotEntered` here.
#[test]
fn below_the_top_the_mate_solve_and_the_instances_find_the_rows_the_descent_entered() {
    let mut store = PartStore::new();
    let first = stored(&mut store, leaf_labelled("part-descent-first"));
    let second = stored(&mut store, leaf_labelled("part-descent-second"));
    let (assembly, ids) = mated(
        "part-descent-mated",
        first,
        second,
        &with_resolver(store.clone()),
    );
    let assembly = crate::fixture::assemble_world_copies(assembly);
    let assembly_ref = store.insert(assembly.clone(), Tol::witness());
    let opts = with_resolver(store);
    let (top, top_ids) = instantiating_all("part-descent-mated-top", &[assembly_ref]);
    let top = fixture::pinned(top, top_ids[0], &opts);

    let own = run(&assembly, &opts);
    for id in &ids {
        assert!(
            matches!(own.result(*id), Some(NodeResult::Ok(_))),
            "the mated instances place at the top: {:?}",
            own.node_error(*id)
        );
    }
    let ev = run(&top, &opts);
    assert!(
        matches!(ev.result(top_ids[0]), Some(NodeResult::Ok(_))),
        "the mated assembly evaluates below the top"
    );
    let volume = |doc: &ProfileDoc, ev| {
        topo::mass_properties(
            &topo::test_support::finished(
                "the measured body",
                topo::Body::clone(&product(doc, ev, Tol::witness()).expect("the product gathers")),
                Tol::witness(),
            ),
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .expect("mass properties")
        .met()
        .expect("the pinned resolution is met")
        .volume
    };
    assert_eq!(
        volume(&top, &ev).to_bits(),
        volume(&assembly, &own).to_bits(),
        "below the top the assembly is the assembly it is at the top"
    );
}

/// **A part no instance asks for is evaluated, and its failure reaches
/// nothing.** `holder` pins an instance of a part the store cannot
/// resolve to the world, and places a second instance, of `asked`, by
/// a rest on a face of the first. The first refuses, so the second,
/// which reads it, is poisoned before it evaluates, and nothing asks
/// for `asked`. Its census evaluates it anyway, with the two documents
/// under it, whether `holder` is evaluated at the top or below it.
/// Whether that subtree is healthy or ends in a missing document, the
/// top's refusal is drawn the same, and it is the one `holder`'s own
/// evaluation draws.
#[test]
fn a_part_no_instance_asks_for_is_evaluated_and_its_failure_reaches_nothing() {
    let mut authoring = PartStore::new();
    let asked = stored(&mut authoring, leaf_labelled("part-descent-asked"));
    let lost = stored(&mut authoring, leaf_labelled("part-descent-lost"));
    let asked_ref = asked.0;
    // `lost` is pinned to the world, `asked` the copy the constraint
    // places.
    let (holder, ids) = rested_on_first(
        "part-descent-holder",
        lost,
        asked,
        &with_resolver(authoring),
    );
    let holder_ref = unpinned_ref("part-descent-holder");
    let inner = unpinned_ref("part-descent-unasked-inner");
    let mut seen = Vec::new();
    for (bottom, evaluated) in [(Some(leaf()), 4), (None, 3)] {
        // `lost` is absent; `asked` is a two-level assembly over the
        // bottom, which is a leaf or missing.
        let mut store = Unpinned::default();
        store.0.insert(holder_ref, holder.clone());
        let bottom_ref = unpinned_ref("part-descent-unasked-bottom");
        store.0.insert(
            asked_ref,
            instantiating_all("part-descent-unasked", &[inner]).0,
        );
        store.0.insert(
            inner,
            instantiating_all("part-descent-unasked-inner", &[bottom_ref]).0,
        );
        if let Some(bottom) = bottom {
            store.0.insert(bottom_ref, bottom);
        }
        let (top, top_ids) = instantiating_all("part-descent-holder-top", &[holder_ref]);
        let opts = store.options();

        let own = run(&holder, &opts);
        assert_eq!(
            own.part_evaluations,
            evaluated - 1,
            "at the top the holder's census evaluates the whole unasked subtree"
        );
        let ev = run(&top, &opts);
        assert_eq!(
            ev.part_evaluations, evaluated,
            "below the top the holder and the whole unasked subtree are evaluated"
        );
        let refused = failed(ev.result(top_ids[0]));
        let level = refused
            .carried_chain()
            .next()
            .expect("the instance carries the holder's failed piece");
        assert!(
            ids.contains(&level.node),
            "the holder's failed piece is one of its instances"
        );
        assert_eq!(
            drawn(level.refusal.kind()),
            drawn(failed(own.result(level.node))),
            "the top carries the refusal the holder's own evaluation raises"
        );
        seen.push(drawn(refused));
    }
    assert_eq!(
        seen[0], seen[1],
        "the unasked subtree's failure changes nothing the top draws"
    );
}
