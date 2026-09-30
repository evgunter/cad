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

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    BooleanOp, DocRef, DocumentId, EvalOptions, MateReach, Node, NodeError, NodeErrorKind,
    NodeResult, PartFault, PartResolver, ProfileDoc, ReachRefusal, RecipeNodeId, mate_reach,
    product,
};
use fixture::resolver::PartStore;
use fixture::{insert, len, on_frame, run, square};
use geom_core::Tol;
use std::sync::Arc;

/// The deepest nesting that evaluates: `PartFault::DepthExceeded`'s
/// sentence names it.
const BOUND: usize = 1024;

/// The wasm32 build's default stack, the smallest an evaluating door
/// runs on (the viewer's workers, a Rust test thread and a Python
/// thread all get more).
const WASM_STACK: usize = 1 << 20;

/// Runs `f` on a thread with the wasm32 build's stack.
fn on_the_smallest_stack<R: Send + 'static>(f: impl FnOnce() -> R + Send + 'static) -> R {
    std::thread::Builder::new()
        .stack_size(WASM_STACK)
        .spawn(f)
        .expect("the thread starts")
        .join()
        .expect("the subject returns")
}

/// The leaf part: a block with a boss unioned onto it, so the leaf's
/// own evaluation runs a boolean on top of whatever the chain costs.
fn leaf() -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive("part-depth-leaf"), Tol::witness());
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
        },
    );
    insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: block,
            b: boss,
            declare: None,
        },
    )
    .0
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

/// The refusal a failed node raised.
fn refusal(result: Option<&NodeResult<f64>>) -> NodeErrorKind {
    match result {
        Some(NodeResult::Failed(e)) => e.kind.clone(),
        other => panic!("expected a failed node, got {other:?}"),
    }
}

/// The last refusal a part refusal carries down its chain, and how many
/// documents down it was raised.
fn deepest(kind: &NodeErrorKind) -> (usize, NodeErrorKind) {
    let levels: Vec<_> = kind.carried_chain().collect();
    let last = levels
        .last()
        .map_or_else(|| kind.clone(), |level| level.refusal.kind().clone());
    (levels.len(), last)
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

        let (levels, last) = deepest(&refusal(run(&top, &opts).result(instance)));
        assert_eq!(
            levels, BOUND,
            "the refusal is raised by the document at the bound, carried up through every \
             document above it"
        );
        let NodeErrorKind::Part {
            fault: PartFault::DepthExceeded,
            ..
        } = last
        else {
            panic!("the chain ends in DepthExceeded, not {last:?}");
        };
        let sentence = NodeError {
            node: RecipeNodeId(0),
            kind: last,
            escalations: Arc::new(Vec::new()),
        }
        .to_string();
        assert!(
            sentence.contains(&format!("deeper than {BOUND} documents")),
            "the sentence states the bound this suite tests: {sentence}"
        );
        let problems = test_utils::refusal::problems("DepthExceeded", &sentence, &[], false);
        assert!(problems.is_empty(), "{problems:#?}");

        // The mate solve's reach descends through the same cache from
        // outside any evaluation, and refuses the same way.
        let reach = mate_reach::<f64>(&opts, Tol::witness());
        let Err(ReachRefusal::PartUnresolved { fault }) = reach.reach(&top_ref) else {
            panic!("the reach refuses the part that nests too deep");
        };
        let (levels, last) = deepest(&NodeErrorKind::Part {
            doc_ref: top_ref,
            fault,
        });
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
            "the reach's chain ends in DepthExceeded, not {last:?}"
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
            "a chain at the bound evaluates, got {:?}",
            ev.result(instance).map(|r| matches!(r, NodeResult::Ok(_)))
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
