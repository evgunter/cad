//! **The registry's subject and the assembly gate's, gathered once.**
//!
//! Two doors here took the shape a caller with a product in hand
//! needs, and each kept a wrapper for the caller that has none. What
//! that owes is one claim per door, and it is a claim of EQUALITY
//! rather than of behaviour: the wrapper is the gather plus the door,
//! so on any document where the gather succeeds the two spellings must
//! be indistinguishable. A row that only exercised the new door would
//! prove the door works and say nothing about whether the old spelling
//! still means what it used to.
//!
//! Where they part is the refusal, and it is stated rather than
//! implied: `ChecksError::Product` and `AssemblyError::Product` are the
//! WRAPPERS' arms — the gather is theirs — and a door handed a subject
//! cannot raise them.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::corpus;
use crate::fixture;

use editor_core::{
    Assembly, AssemblyError, BooleanOp, ChecksConfig, ChecksError, ChecksReport, DocumentId,
    Evaluation, Node, ProductError, ProfileDoc, RecipeNodeId, Subject, assemble, assemble_gathered,
    product_recorded, run_checks, run_checks_on,
};
use fixture::{insert, len, on_frame, square};
use geom_core::Tol;
use topo::Body;

/// The two spellings of the registry over one document.
fn both_ways(doc: &ProfileDoc, ev: &Evaluation<f64>) -> (ChecksReport, ChecksReport) {
    let cfg = ChecksConfig::default();
    let tol = Tol::witness();
    let wrapped = run_checks(doc, ev, &cfg, tol).expect("the registry runs over the corpus");
    let gathered = product_recorded(doc, ev, tol).expect("the corpus gathers");
    let direct = run_checks_on(doc, ev, Subject::Product(&gathered), &cfg, tol)
        .expect("and so does the door under it");
    (wrapped, direct)
}

/// **A2 — the wrapper IS the door**, on every corpus document.
///
/// `ChecksReport` compares by value, so this is the whole report:
/// every finding in order, and the skipped list with them.
#[test]
fn the_registry_wrapper_and_its_door_agree_on_every_corpus_document() {
    for entry in corpus::documents() {
        let ev: Evaluation<f64> = corpus::eval(&entry.doc);
        let (wrapped, direct) = both_ways(&entry.doc, &ev);
        assert_eq!(wrapped, direct, "{}", entry.name);
    }
}

/// **A2 — the empty document is a subject, not a refusal.** A document
/// whose roots denote no body reaches `Subject::NoBodyRoots` through
/// the wrapper and reports clean; the same subject spelled by hand
/// reports the same thing.
#[test]
fn a_document_with_no_body_denoting_root_is_the_no_body_roots_subject() {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty_derived("docm5-empty", tol);
    let ev: Evaluation<f64> = corpus::eval(&doc);
    assert!(
        matches!(
            product_recorded(&doc, &ev, tol).expect_err("nothing denotes a body"),
            ProductError::NoBodyRoots
        ),
        "the premise: this is the arm the wrapper maps to a subject"
    );
    let cfg = ChecksConfig::default();
    let wrapped = run_checks(&doc, &ev, &cfg, tol).expect("an empty document is checkable");
    let direct = run_checks_on(&doc, &ev, Subject::NoBodyRoots, &cfg, tol).expect("so is it here");
    assert_eq!(wrapped, direct);
    assert_eq!(wrapped.findings, Vec::new(), "and it is clean");
    assert!(wrapped.skipped.is_empty(), "with nothing skipped");
}

/// **A2 — a gather that REFUSES never reaches the door.**
///
/// The reachable refusal over two well-formed documents is the DI3
/// pairing one: two documents replayed from one edit log carry the same
/// node ids, so a foreign evaluation answers every lookup and only the
/// gather's own pairing door catches it. Through the wrapper that is
/// `ChecksError::Product`, carrying the gather's own sentence; the door
/// under it is never offered the pair, because there is no subject to
/// offer it.
#[test]
fn a_gather_refusal_is_the_wrappers_refusal_and_the_door_never_sees_it() {
    let tol = Tol::witness();
    let a = twin("docm5-pair-a");
    let b = twin("docm5-pair-b");
    let ev_a: Evaluation<f64> = corpus::eval(&a);

    let refusal = product_recorded(&b, &ev_a, tol).expect_err("the pairing door refuses");
    assert!(
        matches!(refusal, ProductError::EvaluationOfAnotherDocument { .. }),
        "the premise: {refusal:?}"
    );
    match run_checks(&b, &ev_a, &ChecksConfig::default(), tol).expect_err("so does the registry") {
        ChecksError::Product { reason } => {
            assert_eq!(reason, refusal.to_string(), "the gather's own sentence");
        }
        other => panic!("expected the subject refusal, got {other}"),
    }
}

/// **A2 — the registry's own preconditions answer BEFORE its subject
/// does.** A root without a value in this evaluation means the checks
/// could not run AT ALL, and that holds whether or not the document
/// would also have gathered — so the refusal names the root, rather
/// than forwarding the gather's account of the same state.
#[test]
fn a_root_without_a_value_refuses_as_the_registrys_own_precondition() {
    let tol = Tol::witness();
    let (doc, root) = failing_root("docm5-precedence");
    let ev: Evaluation<f64> = corpus::eval(&doc);
    assert!(
        ev.value(root).is_none(),
        "the premise: the root produced no value"
    );
    assert!(
        product_recorded(&doc, &ev, tol).is_err(),
        "and the gather refuses this pair too, so the two arms compete"
    );
    match run_checks(&doc, &ev, &ChecksConfig::default(), tol).expect_err("refuses") {
        ChecksError::Root { node } => assert_eq!(node, root),
        other => panic!("expected the registry's own precondition, got {other}"),
    }
}

/// **A2 — no resident derives its own subject.** The grep is the
/// claim: the registry gathers in exactly one place, its wrapper.
#[test]
fn the_registry_gathers_in_one_place() {
    assert_eq!(
        gathers_in(include_str!("../src/checks.rs")),
        1,
        "one gather in the registry, in `run_checks`; a resident that \
         gathers for itself is the shape this door removed"
    );
}

/// **A3 — `assemble` has no logic of its own**, on every corpus
/// document: the wrapper's verdict and the door's agree, bodies by
/// their measured identity and refusals by their rendering.
///
/// The corpus is part documents, so the A5 gate runs here over products
/// with no minted declaration; the mate fixtures that exercise the
/// verdicts themselves are in `asm_r2b_assembly`, `mate1_member_vocab`
/// and the mate suites, and every one of them goes through `assemble`,
/// which is now this door plus a gather.
#[test]
fn the_assembly_wrapper_and_its_door_agree_on_every_corpus_document() {
    let tol = Tol::witness();
    for entry in corpus::documents() {
        let ev: Evaluation<f64> = corpus::eval(&entry.doc);
        let wrapped = assemble(&entry.doc, &ev, tol);
        let gathered = product_recorded(&entry.doc, &ev, tol).expect("the corpus gathers");
        let direct = assemble_gathered(gathered, tol);
        agree(entry.name, wrapped, direct);
    }
}

/// One document's two assembly verdicts, held to equality.
///
/// `Body` carries no bit-equality door, so the identity asserted is the
/// measured one: the same entity counts and the same mass properties,
/// whose `Debug` rendering of an `f64` round-trips and is therefore an
/// exact comparison rather than a tolerant one.
fn agree(
    name: &str,
    wrapped: Result<Assembly<f64>, AssemblyError>,
    direct: Result<Assembly<f64>, AssemblyError>,
) {
    match (wrapped, direct) {
        (Ok(a), Ok(b)) => {
            assert_eq!(measure(&a.body), measure(&b.body), "{name}: the same body");
            assert_eq!(a.minted.len(), b.minted.len(), "{name}: the same minting");
            assert_eq!(
                a.contacts.patches.len(),
                b.contacts.patches.len(),
                "{name}: the same declared contacts"
            );
            assert_eq!(a.names.len(), b.names.len(), "{name}: the same name table");
        }
        (Err(a), Err(b)) => assert_eq!(a.to_string(), b.to_string(), "{name}: the same refusal"),
        (a, b) => panic!(
            "{name}: the wrapper and the door disagree: {:?} vs {:?}",
            a.map(|_| "Ok"),
            b.map(|_| "Ok")
        ),
    }
}

/// A body as an exactly-comparable measurement.
fn measure(body: &Body<f64>) -> String {
    format!(
        "{}|{}|{}|{:?}",
        body.solids().count(),
        body.faces().count(),
        body.edges().count(),
        topo::mass_properties(body, Tol::witness())
    )
}

/// **A3 — the gather refusal is the WRAPPER's arm.** The door takes a
/// product, so `AssemblyError::Product` is unreachable from it; the
/// wrapper is where a document that does not gather becomes one.
#[test]
fn the_assembly_gathers_in_one_place() {
    assert_eq!(
        gathers_in(include_str!("../src/assembly.rs")),
        1,
        "one gather in the assembly module, in `assemble`"
    );

    let tol = Tol::witness();
    let a = twin("docm5-asm-pair-a");
    let b = twin("docm5-asm-pair-b");
    let ev_a: Evaluation<f64> = corpus::eval(&a);
    match assemble(&b, &ev_a, tol).expect_err("the wrapper inherits the gather's refusal") {
        AssemblyError::Product(_) => {}
        other => panic!("expected the gather's arm, got {other}"),
    }
}

/// **Nothing shares the product to get around the ordering.** The
/// landing's three consumers are ordered so that one gather is enough;
/// a `Clone` or an `Arc` on `Product` would be the other answer, and
/// these modules do not take it.
#[test]
fn nothing_clones_or_shares_the_product() {
    for (path, source) in [
        ("checks.rs", include_str!("../src/checks.rs")),
        ("assembly.rs", include_str!("../src/assembly.rs")),
        ("product.rs", include_str!("../src/product.rs")),
    ] {
        assert!(
            !source.contains("Arc<Product"),
            "{path}: the product is handed on, never shared"
        );
        assert!(
            !source.contains("Clone for Product"),
            "{path}: and never cloned"
        );
    }
    assert!(
        !include_str!("../src/product.rs").contains("#[derive(Debug, Clone)]\npub struct Product"),
        "product.rs: the derive would be the same answer by another spelling"
    );
}

/// Calls to the gather in one source file, comments excluded — a
/// mention in prose is not a second gather.
fn gathers_in(source: &str) -> usize {
    source
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .filter(|line| line.contains("product_recorded("))
        .count()
}

/// An extruded square, the smallest document that gathers a product.
/// Two of these under two ids are one recipe under two identities,
/// which is what the DI3 pairing rows need: the node ids coincide, so
/// a foreign evaluation answers every lookup and only the gather's own
/// pairing door catches it.
fn twin(id: &str) -> ProfileDoc {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty(DocumentId::derive(id), tol);
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
            distance: len(1.0),
        },
    )
    .0
}

/// A document whose one root does not evaluate: two solids resting
/// face on face, united. The union of coincident faces is what the
/// boolean refuses, so the root is live and valueless — the state both
/// the registry's precondition and the gather have something to say
/// about.
fn failing_root(id: &str) -> (ProfileDoc, RecipeNodeId) {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty(DocumentId::derive(id), tol);
    let (doc, a) = slab(doc, 0.0, 1.0);
    let (doc, b) = slab(doc, 1.0, 1.0);
    insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: None,
        },
    )
}

/// A unit-footprint slab from `z0` up by `dz`.
fn slab(doc: ProfileDoc, z0: f64, dz: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(dz),
        },
    )
}

/// **The measured point is pinned exactly**, because a millisecond is
/// only worth reading beside the size it was taken at.
///
/// `m4_pr8_latency`'s registry-split row states its two terms for the
/// corpus heat sink at 160 fins. Its wall clock is machine-dependent
/// and re-taken on a hosted runner per merge; these two counts are
/// neither — they are exact, ε-independent and derived from the recipe,
/// so they gate here on every PR. A document edit that moved them would
/// otherwise leave the committed history comparing two different sizes
/// under one name.
#[test]
fn the_registry_split_is_measured_at_a_pinned_point() {
    let tol = Tol::witness();
    let entry = corpus::documents()
        .into_iter()
        .find(|d| d.name == "heat_sink")
        .expect("the corpus carries the heat sink");
    let doc = editor_core::apply(
        &entry.doc,
        &editor_core::DocEdit::SetDocParam {
            name: editor_core::ParamName::new("fins"),
            value: editor_core::DocParam::Count { value: 160 },
        },
        tol,
    )
    .expect("the fin count is a document parameter")
    .doc;
    let ev: Evaluation<f64> = corpus::eval(&doc);
    let product = product_recorded(&doc, &ev, tol).expect("the heat sink gathers");
    assert_eq!(
        (product.body.solids().count(), product.body.faces().count()),
        (161, 991),
        "the point the registry split is measured at"
    );
}
