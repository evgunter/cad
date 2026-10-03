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
use editor_core::ExtrudeSide;

use std::collections::BTreeMap;

use editor_core::{
    Alignment, AxisSense, BooleanOp, CapEnd, ContactClass, ContentPin, DocRef, DocumentId,
    EvalOptions, MateFrame, MatePrimitive, MateReach, Node, NodeErrorKind, NodeResult, PartFault,
    PartResolver, ProfileDoc, ReachRefusal, RecipeNodeId, ResolveFailure, ResolveFault, mate_reach,
    product,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{at_the_door, insert, len, on_frame, run, square};
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
/// part; and the block extrude whose caps the leaf's product names.
fn leaf_labelled(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 1.0)],
    );
    let (doc, block) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, boss_profile) = on_frame(
        doc,
        [0.0, 0.0, 0.5],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.2, 0.2, 0.25)],
    );
    let (doc, boss) = insert(
        doc,
        Node::Extrude {
            profile: boss_profile,
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, _) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: block,
            b: boss,
            declare: Vec::new(),
        },
    );
    (doc, block)
}

/// A document whose one node instantiates `below`.
fn instantiating(label: &str, below: DocRef) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    insert(doc, Node::instantiate_part(below))
}

/// A store holding the leaf and `levels` documents above it, each
/// instantiating the one below, and the references it answers: the
/// leaf's first, the topmost document's last.
fn chain(levels: usize) -> (PartStore, Vec<DocRef>) {
    let mut store = PartStore::new();
    let mut refs = vec![store.insert(leaf(), Tol::witness())];
    for level in 1..=levels {
        let below = *refs.last().expect("the leaf is stored");
        let (doc, _) = instantiating(&format!("part-depth-level-{level}"), below);
        refs.push(store.insert(doc, Tol::witness()));
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

/// **One past the bound refuses typed, at every door that descends**:
/// the evaluation and the mate solve's reach, each on the wasm32
/// stack. The refusal is raised `BOUND` documents down, is
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

        // The mate solve's reach descends through the same cache from
        // outside any evaluation, and refuses the same way.
        let reach = mate_reach::<f64>(&opts, Tol::witness());
        let Err(ReachRefusal::PartUnresolved { fault }) = reach.reach(&top_ref) else {
            panic!("the reach refuses the part that nests too deep");
        };
        let refused = NodeErrorKind::Part {
            doc_ref: top_ref,
            fault,
        };
        let (levels, last, line) = deepest(&refused);
        assert_eq!(
            levels, BOUND,
            "the reach's refusal is raised at the bound too"
        );
        assert!(
            matches!(
                last,
                NodeErrorKind::Part {
                    fault: PartFault::DepthExceeded,
                    ..
                }
            ),
            "the reach's chain ends in DepthExceeded: {line}"
        );
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
        let (top, instance) = instantiating("part-depth-flattened-top", flattened_ref);
        let ev = run(&top, &resolving(store));
        assert!(
            matches!(ev.result(instance), Some(NodeResult::Ok(_))),
            "a chain at the bound evaluates"
        );

        let body = product(&top, &ev, Tol::witness()).expect("the chain's product gathers");
        let leaf = leaf();
        let leaf_body = product(&leaf, &run(&leaf, &EvalOptions::default()), Tol::witness())
            .expect("the leaf's product gathers");
        let volume = |b: &topo::Body<f64>| {
            topo::mass_properties(b, Tol::witness())
                .expect("mass properties")
                .volume
        };
        assert_eq!(
            volume(&body).to_bits(),
            volume(&leaf_body).to_bits(),
            "BOUND unplaced instances carry the leaf through bit for bit"
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

/// A document whose nodes instantiate `below`, in order.
fn instantiating_all(label: &str, below: &[DocRef]) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for b in below {
        let (d, id) = insert(doc, Node::instantiate_part(*b));
        doc = d;
        ids.push(id);
    }
    (doc, ids)
}

/// A refusal as a surface draws it: its own sentence, then each
/// carried level's line.
fn drawn(kind: &NodeErrorKind) -> Vec<String> {
    std::iter::once(kind.to_string())
        .chain(kind.carried_chain().map(|level| level.line()))
        .collect()
}

/// **The loop check runs before the depth check.** A chain whose last
/// document, `BOUND` documents down, instantiates the first again is
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
                let (doc, _) = instantiating_all(&format!("part-depth-loop-{}", i + 1), &[next]);
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
/// produces.** Below the top, `mixed` instantiates a healthy part
/// twice, a part whose own part is missing, a missing part, and
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
    let (hollow_doc, _) = instantiating_all(
        "part-descent-hollow",
        &[unpinned_ref("part-descent-hollow-missing")],
    );
    store.0.insert(hollow, hollow_doc);
    let mixed = unpinned_ref("part-descent-mixed");
    let (mixed_doc, _) = instantiating_all(
        "part-descent-mixed",
        &[
            healthy,
            healthy,
            hollow,
            unpinned_ref("part-descent-missing"),
            mixed,
        ],
    );
    store.0.insert(mixed, mixed_doc.clone());
    let (top, ids) = instantiating_all("part-descent-mixed-top", &[mixed]);
    let opts = store.options();

    let ev = run(&top, &opts);
    let refused = failed(ev.result(ids[0]));
    let level = refused
        .carried_chain()
        .next()
        .expect("the instance carries the part's failed root");
    let own = run(&mixed_doc, &opts);
    let at_the_top = failed(own.result(level.node));
    assert_eq!(
        drawn(level.refusal.kind()),
        drawn(at_the_top),
        "the failed root the top carries is drawn as the part's own evaluation draws it"
    );
    assert_eq!(
        ev.part_evaluations, 3,
        "`mixed`, the healthy part once for both its instances, and the hollow part; the \
         missing parts and the loop refuse before any evaluation"
    );
}

fn frame(origin: [f64; 3]) -> MateFrame {
    MateFrame::authored(
        origin,
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        geom_core::Tol::witness(),
    )
    .expect("a definite frame")
}

/// A document instantiating `first` and `second` — each a reference
/// with the block its caps are named on — with a mate between them,
/// admitted at the insert door over `authoring`'s reach.
fn mated(
    label: &str,
    (first, first_block): (DocRef, RecipeNodeId),
    (second, second_block): (DocRef, RecipeNodeId),
    authoring: &EvalOptions,
) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, ids) = instantiating_all(label, &[first, second]);
    let mate = Node::Mate {
        a: fixture::head(in_part(ids[0], first_block, CapEnd::End)),
        b: fixture::head(in_part(ids[1], second_block, CapEnd::Start)),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: frame([0.0, 0.0, 1.0]),
            b: frame([0.0; 3]),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Aligned,
            clocking: Some(0.0),
        },
    };
    let reach = mate_reach::<f64>(authoring, Tol::witness());
    let (doc, _) = at_the_door(&doc, &reach, mate).unwrap_or_else(|(_, f)| panic!("{f}"));
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
    let first = store.insert_part(leaf_labelled("part-descent-first"), Tol::witness());
    let second = store.insert_part(leaf_labelled("part-descent-second"), Tol::witness());
    let (assembly, ids) = mated(
        "part-descent-mated",
        first,
        second,
        &with_resolver(store.clone()),
    );
    let assembly_ref = store.insert(assembly.clone(), Tol::witness());
    let (top, top_ids) = instantiating_all("part-descent-mated-top", &[assembly_ref]);
    let opts = with_resolver(store);

    let own = run(&assembly, &opts);
    for id in &ids {
        assert!(
            matches!(own.result(*id), Some(NodeResult::Ok(_))),
            "the mated instances place at the top"
        );
    }
    let ev = run(&top, &opts);
    assert!(
        matches!(ev.result(top_ids[0]), Some(NodeResult::Ok(_))),
        "the mated assembly evaluates below the top"
    );
    let volume = |doc: &ProfileDoc, ev| {
        topo::mass_properties(
            &product(doc, ev, Tol::witness()).expect("the product gathers"),
            Tol::witness(),
        )
        .expect("mass properties")
        .volume
    };
    assert_eq!(
        volume(&top, &ev).to_bits(),
        volume(&assembly, &own).to_bits(),
        "below the top the assembly is the assembly it is at the top"
    );
}

/// **A part no instance asks for is evaluated, and its failure reaches
/// nothing.** Below the top, `holder`'s mate is over a part the store
/// cannot resolve, so the mate faults and neither instance's placement
/// asks for its part. The descent evaluates the resolvable one anyway,
/// with the two documents under it. Whether that subtree is healthy or
/// ends in a missing document, the top's refusal is drawn the same, and
/// it is the one `holder`'s own evaluation, which asks for nothing,
/// draws.
#[test]
fn a_part_no_instance_asks_for_is_evaluated_and_its_failure_reaches_nothing() {
    let mut authoring = PartStore::new();
    let asked = authoring.insert_part(leaf_labelled("part-descent-asked"), Tol::witness());
    let lost = authoring.insert_part(leaf_labelled("part-descent-lost"), Tol::witness());
    let asked_ref = asked.0;
    // `lost` is the mate's second operand, so its group roots the pair
    // and the solve asks for its part first.
    let (holder, ids) = mated(
        "part-descent-holder",
        asked,
        lost,
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
            own.part_evaluations, 0,
            "at the top the holder asks for no part: the mate faults first"
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
            .expect("the instance carries the holder's failed root");
        assert!(
            ids.contains(&level.node),
            "the holder's failed root is one of its instances"
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
