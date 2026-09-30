//! Review probes for PR 3501 (lane depth-rev). Not for merge.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, ContentPin, DocRef, DocumentId, EvalOptions,
    MateFrame, MatePrimitive, Node, NodeErrorKind, NodeResult, PartFault, PartResolver,
    ProfileDoc, RecipeNodeId, ResolveFailure, ResolveFault,
};
use fixture::resolver::{PartStore, in_part};
use fixture::{at_the_door, insert, len, on_frame, run};
use geom_core::Tol;

/// A resolver that checks no pins: answers whatever is stored under the reference.
#[derive(Default, Debug)]
struct MapStore(BTreeMap<DocRef, ProfileDoc>);

impl PartResolver for MapStore {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        self.0.get(doc_ref).cloned().ok_or(ResolveFailure {
            fault: ResolveFault::Unresolved,
            message: "no such document".into(),
        })
    }
}

fn dref(label: &str) -> DocRef {
    DocRef {
        id: DocumentId::derive(label),
        pin: ContentPin([7u8; 32]),
    }
}

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

fn block(label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 0.5)],
    );
    insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    )
    .0
}

fn small<R: Send + 'static>(f: impl FnOnce() -> R + Send + 'static) -> R {
    std::thread::Builder::new()
        .stack_size(1 << 20)
        .spawn(f)
        .unwrap()
        .join()
        .unwrap()
}

fn chain_summary(kind: &NodeErrorKind) -> String {
    let levels: Vec<_> = kind.carried_chain().collect();
    let last = levels.last().map(|l| l.refusal.kind());
    let tail = match last {
        Some(NodeErrorKind::Part { fault, .. }) => match fault {
            PartFault::ReferenceCycle { cycle } => format!("ReferenceCycle(len {})", cycle.len()),
            PartFault::DepthExceeded => "DepthExceeded".into(),
            other => format!("{}", other),
        },
        Some(k) => format!("{k}"),
        None => "none".into(),
    };
    format!("levels={} last={tail}", levels.len())
}

/// T -> d1 -> ... -> dN -> d_back (a cycle closing at depth N).
fn cyclic_chain(n: usize, back_to: usize) -> (ProfileDoc, RecipeNodeId, EvalOptions) {
    let refs: Vec<DocRef> = (1..=n).map(|i| dref(&format!("rev-cyc-{i}"))).collect();
    let mut store = MapStore::default();
    for i in 0..n {
        let next = if i + 1 < n { refs[i + 1] } else { refs[back_to - 1] };
        let (d, _) = instantiating_all(&format!("rev-cyc-{}", i + 1), &[next]);
        store.0.insert(refs[i], d);
    }
    let (top, ids) = instantiating_all("rev-cyc-top", &[refs[0]]);
    let opts = EvalOptions {
        resolver: Some(Arc::new(store) as Arc<dyn PartResolver>),
        ..EvalOptions::default()
    };
    (top, ids[0], opts)
}

fn failed(r: Option<&NodeResult<f64>>) -> &NodeErrorKind {
    match r {
        Some(NodeResult::Failed(e)) => &e.kind,
        Some(NodeResult::Ok(_)) => panic!("expected failure, got ok"),
        _ => panic!("no result"),
    }
}

#[test]
fn p1_a_cycle_closing_at_the_bound_is_still_a_cycle() {
    for (n, back) in [(1024usize, 1usize), (1024, 1000), (1025, 1), (1023, 1)] {
        let s = small(move || {
            let (top, inst, opts) = cyclic_chain(n, back);
            let ev = run(&top, &opts);
            chain_summary(failed(ev.result(inst)))
        });
        eprintln!("P1 chain n={n} back_to={back}: {s}");
    }
}

/// Diamond with mixed faults below the top: T -> A; A -> {B, B, C, missing, A(cycle)};
/// B -> D; C -> D'(no such). Prints the full text of every level.
#[test]
fn p2_mixed_faults_below_the_top_render_the_same() {
    let mut store = MapStore::default();
    let d = dref("rev-mix-d");
    store.0.insert(d, block("rev-mix-d"));
    let b = dref("rev-mix-b");
    store.0.insert(b, instantiating_all("rev-mix-b", &[d]).0);
    let c = dref("rev-mix-c");
    store
        .0
        .insert(c, instantiating_all("rev-mix-c", &[dref("rev-mix-missing")]).0);
    let a = dref("rev-mix-a");
    store.0.insert(
        a,
        instantiating_all("rev-mix-a", &[b, b, c, dref("rev-mix-none"), a]).0,
    );
    let (top, ids) = instantiating_all("rev-mix-top", &[a, b]);
    let opts = EvalOptions {
        resolver: Some(Arc::new(store) as Arc<dyn PartResolver>),
        ..EvalOptions::default()
    };
    let ev = run(&top, &opts);
    for id in ids {
        match ev.result(id) {
            Some(NodeResult::Failed(e)) => {
                eprintln!("P2 node {} : {}", id.0, e.kind);
                for l in e.kind.carried_chain() {
                    eprintln!("P2   | {:?} {}", l.document, l.line());
                }
            }
            Some(NodeResult::Ok(_)) => eprintln!("P2 node {} ok", id.0),
            _ => eprintln!("P2 node {} none", id.0),
        }
    }
    eprintln!("P2 part_evaluations={}", ev.part_evaluations);
}

fn frame(origin: [f64; 3]) -> MateFrame {
    MateFrame {
        origin,
        axis: [0.0, 0.0, 1.0],
        reference: [1.0, 0.0, 0.0],
    }
}

/// The disclosed case: below the top, an instance whose placement refused
/// through a mate fault. `lost` is unresolvable and is the mate's `a`,
/// so the lever may refuse before it asks for the healthy part's reach.
#[test]
fn p3_the_disclosed_case_a_mate_faulted_instance_below_the_top() {
    for broken_healthy in [false, true] {
        let mut full = PartStore::new();
        let part_ref = full.insert(block("rev-dis-part"), Tol::witness());
        let lost_ref = full.insert(block("rev-dis-lost"), Tol::witness());
        let authoring = fixture::resolver::with_resolver(full.clone());
        let (b, ids) = instantiating_all("rev-dis-b", &[lost_ref, part_ref]);
        let mate = Node::Mate {
            a: fixture::head(in_part(ids[0], CapEnd::End)),
            b: fixture::head(in_part(ids[1], CapEnd::Start)),
            class: ContactClass::Rest,
            alignment: Alignment {
                a: frame([0.0, 0.0, 1.0]),
                b: frame([0.0; 3]),
                primitive: MatePrimitive::FrameCoincidence,
                sense: AxisSense::Aligned,
                clocking: Some(0.0),
            },
        };
        let reach = editor_core::mate_reach::<f64>(&authoring, Tol::witness());
        let (b, _mate) = at_the_door(&b, &reach, mate).unwrap_or_else(|(_, f)| panic!("{f}"));
        // The evaluation store: the part, and B; `lost` is absent.
        let mut store = MapStore::default();
        let part_doc = if broken_healthy {
            // the "healthy" instance's part now itself fails: it instantiates a missing doc
            instantiating_all("rev-dis-part", &[dref("rev-dis-nowhere")]).0
        } else {
            block("rev-dis-part")
        };
        store.0.insert(part_ref, part_doc);
        let b_ref = dref("rev-dis-b");
        store.0.insert(b_ref, b.clone());
        let opts = EvalOptions {
            resolver: Some(Arc::new(store) as Arc<dyn PartResolver>),
            ..EvalOptions::default()
        };
        // B at the top: lazy cache, the reference answer.
        let ev_b = run(&b, &opts);
        eprintln!(
            "P3 broken={broken_healthy} B-at-top part_evaluations={}",
            ev_b.part_evaluations
        );
        for &id in ids.iter() {
            match ev_b.result(id) {
                Some(NodeResult::Failed(e)) => eprintln!("P3   B node {}: {}", id.0, e.kind),
                Some(NodeResult::Ok(_)) => eprintln!("P3   B node {}: ok", id.0),
                _ => eprintln!("P3   B node {}: none", id.0),
            }
        }
        let (top, tids) = instantiating_all("rev-dis-top", &[b_ref]);
        let ev = run(&top, &opts);
        eprintln!(
            "P3 broken={broken_healthy} T-over-B part_evaluations={}",
            ev.part_evaluations
        );
        match ev.result(tids[0]) {
            Some(NodeResult::Failed(e)) => {
                eprintln!("P3   T: {}", e.kind);
                for l in e.kind.carried_chain() {
                    eprintln!("P3   | {:?} {}", l.document, l.line());
                }
            }
            Some(NodeResult::Ok(_)) => eprintln!("P3   T ok"),
            _ => eprintln!("P3   T none"),
        }
    }
}

/// Scaling of the at-bound work (names nest per level): time at a few depths.
#[test]
fn p4_time_by_depth() {
    for n in [128usize, 256, 512] {
        let t = std::time::Instant::now();
        let ok = small(move || {
            let mut store = MapStore::default();
            let mut prev = dref("rev-t-leaf");
            store.0.insert(prev, block("rev-t-leaf"));
            for i in 1..=n {
                let r = dref(&format!("rev-t-{i}"));
                store.0.insert(r, instantiating_all(&format!("rev-t-{i}"), &[prev]).0);
                prev = r;
            }
            let (top, ids) = instantiating_all("rev-t-top", &[prev]);
            let opts = EvalOptions {
                resolver: Some(Arc::new(store) as Arc<dyn PartResolver>),
                ..EvalOptions::default()
            };
            let ev = run(&top, &opts);
            matches!(ev.result(ids[0]), Some(NodeResult::Ok(_)))
        });
        eprintln!("P4 depth {n}: ok={ok} {:?}", t.elapsed());
    }
}
