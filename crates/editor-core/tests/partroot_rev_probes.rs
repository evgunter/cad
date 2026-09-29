//! Review probes (lane `partroot-rev`) over PR 3459's carried refusal,
//! at the kernel layer. A red row is a finding.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::print_stderr)]

use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    CancelToken, DocRef, DocumentId, EvalOptions, Evaluation, Expr, Node, NodeErrorKind,
    NodeRefusal, NodeResult, PartFault, PartResolver, ProfileDoc, RecipeNodeId, ResolveFailure,
    ResolveFault, content_pin, evaluate,
};
use fixture::{ang, insert, len, on_frame, scl, square};
use geom_core::Tol;

#[derive(Debug, Default)]
struct Store(BTreeMap<DocumentId, ProfileDoc>);

impl Store {
    fn put(&mut self, doc: ProfileDoc) -> DocRef {
        let pin = content_pin(&doc, Tol::witness()).expect("pin");
        let id = doc.id();
        self.0.insert(id, doc);
        DocRef { id, pin }
    }
}

impl PartResolver for Store {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        self.0.get(&doc_ref.id).cloned().ok_or(ResolveFailure {
            fault: ResolveFault::Unresolved,
            message: "no such document".to_string(),
        })
    }
}

fn opts(store: Store) -> EvalOptions {
    EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    }
}

fn run(doc: &ProfileDoc, opts: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), opts, Tol::witness())
}

/// A part whose one root is an extrude whose distance divides by zero.
fn failing_extrude(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    insert(
        doc,
        Node::Extrude {
            profile,
            distance: Expr::div(len(1.0), scl(0.0)).unwrap(),
        },
    )
}

fn wrapper(label: &str, inner: DocRef) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    insert(doc, Node::instantiate_part(inner))
}

/// The drawn lines as the tree walks them, with each level's `doc_ref`.
fn chain(kind: &NodeErrorKind) -> Vec<(Option<DocumentId>, RecipeNodeId, String)> {
    let mut out = Vec::new();
    let mut at = kind;
    while let Some((node, refusal)) = at.carried() {
        let doc = match at {
            NodeErrorKind::Part { doc_ref, .. } => Some(doc_ref.id),
            _ => None,
        };
        out.push((doc, node, refusal.line_at(node)));
        at = refusal.kind();
    }
    out
}

fn failed(ev: &Evaluation<f64>, id: RecipeNodeId) -> &editor_core::NodeError {
    match ev.result(id) {
        Some(NodeResult::Failed(e)) => e,
        other => panic!("expected Failed at {id:?}, got {other:?}"),
    }
}

/// **Probe 1 — a real depth-3 chain.** assembly → p1 → p2 → p3, p3's
/// root refuses. Every level keeps its `doc_ref`, in order, and the last
/// line is p3's own node error exactly as p3's own evaluation renders it.
#[test]
fn probe_1_depth_three_chain_keeps_every_level() {
    let mut store = Store::default();
    let (p3, p3_root) = failing_extrude("rev-p3");
    let p3_own = failed(&run(&p3, &EvalOptions::default()), p3_root).to_string();
    let r3 = store.put(p3);
    let (p2, p2_root) = wrapper("rev-p2", r3);
    let r2 = store.put(p2);
    let (p1, p1_root) = wrapper("rev-p1", r2);
    let r1 = store.put(p1);
    let (asm, inst) = wrapper("rev-asm", r1);
    let ev = run(&asm, &opts(store));
    let top = failed(&ev, inst);
    let lines = chain(&top.kind);
    let mut drawn = vec![top.to_string()];
    drawn.extend(lines.iter().map(|(_, _, l)| l.clone()));
    eprintln!("PROBE-1\n{}", drawn.join("\n"));
    assert_eq!(
        lines.iter().map(|(d, n, _)| (*d, *n)).collect::<Vec<_>>(),
        vec![
            (Some(r1.id), p1_root),
            (Some(r2.id), p2_root),
            (Some(r3.id), p3_root)
        ],
        "one level per document, in order, each keeping its doc_ref"
    );
    assert_eq!(lines[2].2, p3_own, "the last line is p3's own rendering");
    for (i, line) in drawn.iter().enumerate() {
        let p = test_utils::refusal::problems(&format!("depth-3/{i}"), line, &[], false);
        assert!(p.is_empty(), "{p:?}");
    }
}

/// **Probe 2 — a part whose root is poisoned, not failed.** Measured, so
/// the same input can be compared with main (examples/partroot_rev_main.rs).
#[test]
fn probe_2_poisoned_root() {
    let mut store = Store::default();
    let (part, extrude) = failing_extrude("rev-poisoned");
    let (part, _moved) = insert(
        part,
        Node::Transform {
            input: extrude,
            translation: [len(0.01), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    let r = store.put(part);
    let (asm, inst) = wrapper("rev-poisoned-asm", r);
    let ev = run(&asm, &opts(store));
    let top = failed(&ev, inst);
    eprintln!("PROBE-2 {top}\n  carried: {:?}\n  kind: {:?}", top.kind.carried().is_some(), top.kind);
    assert!(
        top.kind.carried().is_some(),
        "the extrude's refusal, which is why the part has no body, reaches the assembly"
    );
}

/// **Probe 3 — a part whose root is Ok but whose product refuses
/// otherwise** (no body-denoting root: a lone frame).
#[test]
fn probe_3_root_ok_product_refuses() {
    let mut store = Store::default();
    let doc = ProfileDoc::empty(DocumentId::derive("rev-frame-only"), Tol::witness());
    let (doc, _) = insert(doc, fixture::xy_frame());
    let r = store.put(doc);
    let (asm, inst) = wrapper("rev-frame-only-asm", r);
    let ev = run(&asm, &opts(store));
    let top = failed(&ev, inst);
    let line = top.to_string();
    eprintln!("PROBE-3 {line}\n  kind: {:?}", top.kind);
    assert!(top.kind.carried().is_none());
    let p = test_utils::refusal::problems("root-ok", &line, &[], false);
    assert!(p.is_empty(), "{p:?}");
}

/// **Probe 4 — `NodeRefusal`'s `Eq` in both directions.**
#[test]
fn probe_4_node_refusal_eq() {
    let r = |d: f64, p: f64| {
        NodeRefusal::from(NodeErrorKind::ToleranceConflict {
            document_eps: d,
            process_eps: p,
        })
    };
    let a = r(0.0, 1e-9);
    assert_eq!(a, a.clone(), "reflexive through the pointer");
    assert_eq!(a, r(0.0, 1e-9), "equal values built apart");
    assert_ne!(r(0.0, 1e-9), r(-0.0, 1e-9), "-0.0 separates");
    eprintln!(
        "PROBE-4 display 0.0: {} | -0.0: {}",
        r(0.0, 1e-9),
        r(-0.0, 1e-9)
    );
    assert_eq!(r(f64::NAN, 1e-9), r(f64::NAN, 1e-9), "NaN identifies");
    // Two different documents, same drawn sentence: Eq separates them.
    let part = |label: &str| {
        NodeRefusal::from(NodeErrorKind::Part {
            doc_ref: DocRef {
                id: DocumentId::derive(label),
                pin: editor_core::ContentPin::of_bytes(b"x"),
            },
            fault: PartFault::NoResolver,
        })
    };
    assert_eq!(part("x").to_string(), part("y").to_string());
    assert_ne!(part("x"), part("y"));
}

/// **Probe 5a — hex planted in a non-admitted row reds.**
#[test]
fn probe_5a_hex_in_a_non_admitted_row_reds() {
    let text = "node 5 failed: instantiating 11c1eee0e02516b19e263d060a3c9f80@9515831d455a: \
                the resolver refused. Recourse: open the part";
    let rows = vec![("Part/NoResolver".to_string(), text.to_string())];
    let p = crate::refusal_concision_chains::over_budget(&rows);
    eprintln!("PROBE-5a {p:?}");
    assert!(!p.is_empty(), "a hex id on a non-admitted row is red");
}

/// **Probe 5b — what the chains admission lets through.** The PR says
/// each admission is "only for its 'dumps an arena key' problem, and
/// only when the text has no `Key(`". The chains roster admits through
/// `FILED_DEBUG`, which admits a `Debug` struct and a `Key(` too.
#[test]
fn probe_5b_admitted_row_admits_only_the_hex() {
    let text = "node 5 failed: instantiating the part: the reference loop returns to \
                FaceKey(3v1) DocRef { id: 1 }. Recourse: break the loop";
    let rows = vec![("Part/ReferenceCycle".to_string(), text.to_string())];
    let p = crate::refusal_concision_chains::over_budget(&rows);
    eprintln!("PROBE-5b {p:?}");
    assert!(
        !p.is_empty(),
        "an arena key proper and a Debug struct on an admitted row are still red"
    );
}

/// **Probe 5c — the detector's shape blind spot**: a pin prefix that
/// happens to be all decimal digits.
#[test]
fn probe_5c_all_digit_pin() {
    let t = "instantiating 11c1eee0e02516b19e263d060a3c9f80@951583145512: gone";
    let only_pin = "the part pinned at 951583145512 is gone";
    eprintln!(
        "PROBE-5c with-id {} pin-only {}",
        test_utils::refusal::arena_key(t),
        test_utils::refusal::arena_key(only_pin)
    );
    assert!(test_utils::refusal::arena_key(only_pin), "a pin with no letter");
}

/// **Probe 6 — the budget on the tree's carried line, prefix included.**
/// The viewer draws a part's carried line as `in {file}, {line}`
/// (`viewer::tree::carried_lines`), `{file}` being the file name or
/// `PartFiles::NO_FILE`. The kernel row measures `line_at` alone.
#[test]
fn probe_6_longest_roster_refusal_with_the_viewer_prefix() {
    let longest = crate::refusal_concision_chains::node_refusals_for_review()
        .into_iter()
        .map(|(name, kind)| {
            let t = crate::refusal_concision_chains::as_the_viewer_shows_it(kind);
            (t.split_whitespace().count(), name, t)
        })
        .max()
        .unwrap();
    eprintln!("PROBE-6 longest {} words: {} :: {}", longest.0, longest.1, longest.2);
    let mut rows = Vec::new();
    for file in ["bracket.pncad", "a part with no file in this directory"] {
        rows.push((longest.1.clone(), format!("in {file}, {}", longest.2)));
    }
    for (n, t) in &rows {
        eprintln!("PROBE-6 {} words: {n}", t.split_whitespace().count());
    }
    let p = crate::refusal_concision_chains::over_budget(&rows);
    assert!(p.is_empty(), "{}", p.join("\n"));
}
