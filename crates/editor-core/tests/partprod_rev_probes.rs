//! Review probes for PR 3482 (`partprod-rev`): the poisoned part root
//! nested two deep, `through` a failed instance, and the `PartProduct`
//! recourse over every class a real document reaches.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    CancelToken, CarriedIn, ContentPin, Datum, DocRef, DocumentId, EvalOptions, Evaluation, Node,
    NodeErrorClass, NodeErrorKind, NodeResult, PartFault, PartResolver, ProfileDoc, RecipeNodeId,
    ResolveFailure, ResolveFault, content_pin, evaluate,
};
use fixture::{ang, insert, len, on_frame, scl, square};
use geom_core::Tol;

#[derive(Debug, Default)]
struct Store {
    docs: BTreeMap<DocumentId, ProfileDoc>,
}

impl Store {
    fn insert(&mut self, doc: ProfileDoc) -> DocRef {
        let pin = content_pin(&doc, Tol::witness()).expect("the pin computes");
        let id = doc.id();
        self.docs.insert(id, doc);
        DocRef { id, pin }
    }
}

impl PartResolver for Store {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        self.docs
            .get(&doc_ref.id)
            .cloned()
            .ok_or_else(|| ResolveFailure {
                fault: ResolveFault::Unresolved,
                message: "no such document".to_owned(),
            })
    }
}

fn run(doc: &ProfileDoc, store: Option<Arc<Store>>) -> Evaluation<f64> {
    let opts = EvalOptions {
        resolver: store.map(|s| s as Arc<dyn PartResolver>),
        ..EvalOptions::default()
    };
    evaluate::<f64>(doc, None, &CancelToken::new(), &opts, Tol::witness())
}

fn moved(doc: ProfileDoc, input: RecipeNodeId, dx: f64) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Transform {
            input,
            translation: [len(dx), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    )
}

/// A part whose extrude refuses (1/0) and whose one root is a transform
/// over it.
fn broken(label: &str) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: editor_core::Expr::div(len(1.0), scl(0.0)).unwrap(),
        },
    );
    let (doc, root) = moved(doc, extrude, 0.1);
    (doc, extrude, root)
}

fn failure(ev: &Evaluation<f64>, node: RecipeNodeId) -> &editor_core::NodeError {
    match ev.result(node) {
        Some(NodeResult::Failed(e)) => e,
        other => panic!("node {node:?} did not fail: {other:?}"),
    }
}

fn own_line(doc: &ProfileDoc, node: RecipeNodeId, store: Option<Arc<Store>>) -> String {
    failure(&run(doc, store), node).to_string()
}

fn standard(name: &str, text: &str) -> Vec<String> {
    test_utils::refusal::problems(name, text, &[], false)
}

/// Depth 2: assembly -> bracket (root: transform over an instance of
/// `broken`) -> broken (root: transform over the refusing extrude).
/// The outer instance's `through` is itself a failed instance.
#[test]
fn probe_poisoned_root_two_deep_through_a_failed_instance() {
    let mut store = Store::default();
    let (broken_doc, extrude, broken_root) = broken("probe-broken");
    let broken_ref = store.insert(broken_doc.clone());

    let bracket = ProfileDoc::empty(DocumentId::derive("probe-bracket"), Tol::witness());
    let (bracket, inner) = insert(bracket, Node::instantiate_part(broken_ref));
    let (bracket, bracket_root) = moved(bracket, inner, 0.2);
    assert_eq!(bracket.roots(), &[bracket_root]);
    let bracket_ref = store.insert(bracket.clone());

    let asm = ProfileDoc::empty(DocumentId::derive("probe-asm"), Tol::witness());
    let (asm, outer) = insert(asm, Node::instantiate_part(bracket_ref));
    let store = Arc::new(store);
    let ev = run(&asm, Some(Arc::clone(&store)));
    let error = failure(&ev, outer);

    let NodeErrorKind::Part { fault, .. } = &error.kind else {
        panic!("a part fault: {error:?}");
    };
    let PartFault::PartRootPoisoned { root, through, refusal } = fault else {
        panic!("expected PartRootPoisoned, got {fault:?}");
    };
    assert_eq!((*root, *through), (bracket_root, inner));
    assert_eq!(error.kind.class(), NodeErrorClass::PartRootPoisoned);
    // The refusal at `through` is itself the inner instance's poisoned-root fault.
    let NodeErrorKind::Part { fault: inner_fault, .. } = refusal.kind() else {
        panic!("the carried refusal is the inner instance's: {refusal:?}");
    };
    assert!(
        matches!(inner_fault, PartFault::PartRootPoisoned { root, through, .. }
            if (*root, *through) == (broken_root, extrude)),
        "{inner_fault:?}"
    );
    assert_eq!(refusal.kind().class(), NodeErrorClass::PartRootPoisoned);

    let levels: Vec<_> = error
        .kind
        .carried_chain()
        .map(|l| (l.document, l.node, l.line()))
        .collect();
    let bracket_line = own_line(&bracket, inner, Some(Arc::clone(&store)));
    let broken_line = own_line(&broken_doc, extrude, None);
    assert_eq!(
        levels,
        vec![
            (CarriedIn::Part(&bracket_ref), inner, bracket_line),
            (CarriedIn::Part(&broken_ref), extrude, broken_line),
        ],
        "one level per document, ending at the failing node"
    );
    let mut problems = standard("outer", &error.to_string());
    for (i, (_, _, line)) in levels.iter().enumerate() {
        // The last level is the extrude's own kernel line, not this
        // PR's text: its recourse is its own row's business.
        problems.extend(
            standard(&format!("level {i}"), line)
                .into_iter()
                .filter(|p| !(i == levels.len() - 1 && p.contains("states no recourse"))),
        );
    }
    eprintln!("PROBE outer: {error}");
    for (_, _, l) in &levels {
        eprintln!("PROBE   {l}");
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// `through` is a failed instance whose fault carries nothing (the
/// reference does not resolve): the chain has one level, ending at that
/// instance in the bracket's id space.
#[test]
fn probe_poisoned_root_through_an_unresolved_instance() {
    let mut store = Store::default();
    let missing = DocRef {
        id: DocumentId::derive("probe-missing"),
        pin: ContentPin([7u8; 32]),
    };
    let bracket = ProfileDoc::empty(DocumentId::derive("probe-bracket-2"), Tol::witness());
    let (bracket, inner) = insert(bracket, Node::instantiate_part(missing));
    let (bracket, bracket_root) = moved(bracket, inner, 0.2);
    let bracket_ref = store.insert(bracket.clone());
    let asm = ProfileDoc::empty(DocumentId::derive("probe-asm-2"), Tol::witness());
    let (asm, outer) = insert(asm, Node::instantiate_part(bracket_ref));
    let store = Arc::new(store);
    let ev = run(&asm, Some(Arc::clone(&store)));
    let error = failure(&ev, outer);
    assert_eq!(error.kind.class(), NodeErrorClass::PartRootPoisoned);
    let NodeErrorKind::Part { fault: PartFault::PartRootPoisoned { root, through, .. }, .. } =
        &error.kind
    else {
        panic!("{error:?}");
    };
    assert_eq!((*root, *through), (bracket_root, inner));
    let levels: Vec<_> = error
        .kind
        .carried_chain()
        .map(|l| (l.document, l.node, l.line()))
        .collect();
    assert_eq!(
        levels,
        vec![(
            CarriedIn::Part(&bracket_ref),
            inner,
            own_line(&bracket, inner, Some(Arc::clone(&store)))
        )]
    );
    eprintln!("PROBE outer: {error}");
    eprintln!("PROBE   {}", levels[0].2);
    let mut problems = standard("outer", &error.to_string());
    problems.extend(standard("level 0", &levels[0].2));
    // The unresolved line itself is filed short of the standard; report only.
    eprintln!("PROBE problems: {problems:#?}");
    assert!(standard("outer", &error.to_string()).is_empty());
}

/// Naming, reached by a real part: a split and a move of one block as
/// two roots alias the block's strict wall name. The instance's line
/// must pass `problems` with its class's recourse.
#[test]
fn probe_part_product_naming_through_a_real_part() {
    let mut store = Store::default();
    let doc = ProfileDoc::empty(DocumentId::derive("probe-naming"), Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, a) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
        },
    );
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.5), len(0.0), len(0.0)],
            normal: [scl(1.0), scl(0.0), scl(0.0)],
        }),
    );
    let (doc, _split) = insert(doc, Node::Split { target: a, tool: plane });
    let (doc, _moved) = moved(doc, a, 2.0);
    let part_ref = store.insert(doc);
    let asm = ProfileDoc::empty(DocumentId::derive("probe-naming-asm"), Tol::witness());
    let (asm, outer) = insert(asm, Node::instantiate_part(part_ref));
    let ev = run(&asm, Some(Arc::new(store)));
    let error = failure(&ev, outer);
    eprintln!("PROBE naming: {error}");
    assert!(
        matches!(&error.kind, NodeErrorKind::Part { fault: PartFault::PartProduct { kind, .. }, .. }
            if *kind == editor_core::ProductErrorKind::Naming),
        "{error:?}"
    );
    let problems = standard("naming", &error.to_string());
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// Reachability of RootInvalid / ProductInvalid / Graft from a real
/// document: a handful of candidates gathered directly, the classes
/// printed.
#[test]
fn probe_invalid_classes_reachable_from_real_documents() {
    let tol = Tol::witness();
    let mut seen = Vec::new();
    let extr = |label: &str, poly: Vec<(f64, f64)>, d: f64| {
        let doc = ProfileDoc::empty(DocumentId::derive(label), tol);
        let (doc, p) = on_frame(doc, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![poly]);
        insert(
            doc,
            Node::Extrude {
                profile: p,
                distance: len(d),
            },
        )
        .0
    };
    let ccw = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let cw: Vec<_> = ccw.iter().rev().copied().collect();
    let cases = vec![
        ("negative distance", extr("probe-neg", ccw.clone(), -1.0)),
        ("clockwise profile", extr("probe-cw", cw.clone(), 1.0)),
        ("clockwise, negative", extr("probe-cwneg", cw, -1.0)),
    ];
    for (label, doc) in cases {
        let ev = run(&doc, None);
        let verdict = match editor_core::product_named(&doc, &ev, tol) {
            Ok(_) => "Ok".to_owned(),
            Err(e) => format!("{:?}: {e}", e.kind()),
        };
        eprintln!("PROBE reach {label}: {verdict}");
        seen.push(verdict);
    }
}

/// The three classes `product_recourse` answers `None` for, wrapped as
/// `PartProduct` exactly as `product_fault` would wrap them (message =
/// the gather's rendering less its stage word), held to `problems`.
#[test]
fn probe_forwarding_classes_through_part_product() {
    use editor_core::{ProductError, SourceFinding};
    let cases = vec![
        ProductError::RootInvalid {
            findings: vec![SourceFinding {
                node: RecipeNodeId(3),
                output: 0,
                errors: vec![topo::ValidationError::NegativeVolume {
                    solid: topo::SolidKey::default(),
                }],
            }],
        },
        ProductError::ProductInvalid {
            errors: vec![topo::ValidationError::NegativeVolume {
                solid: topo::SolidKey::default(),
            }],
        },
        ProductError::Graft {
            node: RecipeNodeId(5),
            source: Box::new(topo::BooleanError::Band(geom_core::BandError::Empty {
                zero: 1.0,
                escalate: 0.5,
            })),
        },
    ];
    let mut all = Vec::new();
    for err in cases {
        let kind = err.kind();
        let message = err
            .to_string()
            .strip_prefix("product: ")
            .expect("the stage word opens it")
            .to_owned();
        let e = editor_core::NodeError {
            node: RecipeNodeId(5),
            kind: NodeErrorKind::Part {
                doc_ref: DocRef {
                    id: DocumentId::derive("probe-fwd"),
                    pin: ContentPin([1u8; 32]),
                },
                fault: PartFault::PartProduct { kind, message },
            },
            escalations: Arc::new(Vec::new()),
        };
        let text = e.to_string();
        eprintln!("PROBE fwd {kind:?}: {text}");
        all.extend(standard(&format!("{kind:?}"), &text));
    }
    eprintln!("PROBE fwd problems:\n{}", all.join("\n"));
}
