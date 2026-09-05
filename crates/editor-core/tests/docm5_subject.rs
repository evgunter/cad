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
    Advisory, Assembly, AssemblyError, BooleanOp, CheckId, ChecksConfig, ChecksError, ChecksReport,
    DocumentId, Evaluation, Node, ProductError, ProfileDoc, RecipeNodeId, Severity, Subject,
    assemble, assemble_gathered, product_recorded, run_checks, run_checks_on,
};
use fixture::{ang, insert, len, on_frame, scl, square};
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

/// **A2 — a gather that refuses reaches the door as
/// `Subject::Unavailable`, and the door raises it AFTER the residents
/// that read no subject have answered.**
///
/// The reachable gather refusal over one well-formed document is the
/// naming collision the spec names: two `Transform`s of one extrude are
/// two roots carrying one extrude's minted names into one product
/// table. Connectedness reads the evaluation and answers first (it has
/// nothing to say here); the separation resident reads the subject,
/// finds none, and the door refuses `Product` carrying the gather's own
/// sentence. Nothing is run and discarded, and no arm claims the
/// document denotes no body when it does.
#[test]
fn a_gather_refusal_reaches_the_door_and_refuses_after_the_subject_free_residents() {
    let tol = Tol::witness();
    let doc = naming_collision("docm5-collide");
    let ev: Evaluation<f64> = corpus::eval(&doc);
    let refusal = product_recorded(&doc, &ev, tol).expect_err("two roots collide");
    assert!(
        matches!(refusal, ProductError::Naming { .. }),
        "the premise: {refusal:?}"
    );

    // Through the wrapper, which gathers.
    match run_checks(&doc, &ev, &ChecksConfig::default(), tol).expect_err("the registry refuses") {
        ChecksError::Product { reason } => {
            assert_eq!(reason, refusal.to_string(), "the gather's own sentence");
        }
        other => panic!("expected the subject refusal, got {other}"),
    }

    // And through the door, handed the same fact directly. The arm is
    // the same, and it is raised whether or not the connectedness
    // resident ran: what it is NOT raised by is a resident that reads
    // no subject.
    let unavailable = || Subject::Unavailable {
        reason: refusal.to_string(),
    };
    for cfg in [
        ChecksConfig::default(),
        ChecksConfig {
            connectedness: Severity::Off,
            ..ChecksConfig::default()
        },
    ] {
        match run_checks_on(&doc, &ev, unavailable(), &cfg, tol).expect_err("the door refuses") {
            ChecksError::Product { reason } => assert_eq!(reason, refusal.to_string()),
            other => panic!("expected the subject refusal, got {other}"),
        }
    }
}

/// **The gather is LAZY: a run no resident needs a subject for takes
/// none.** (R2's `…_does_not_need_a_subject` and
/// `…_still_pays_for_the_gather`, adopted — the second as a COUNT
/// rather than a stopwatch, which is the same claim without a clock in
/// it.)
///
/// With the one subject-reading resident `Off`, the base never gathered
/// (the gather lived inside that resident), so a document whose gather
/// refuses reported cleanly and a document whose gather succeeds paid
/// nothing. Both hold again: the counter reads 0 across the call, and
/// the crossed pair below — which the gather refuses — reports.
#[test]
fn a_run_that_needs_no_subject_does_not_gather() {
    let tol = Tol::witness();
    let off = ChecksConfig {
        separation: Advisory::Off,
        ..ChecksConfig::default()
    };
    assert!(
        !off.needs_a_subject(),
        "the premise: nothing enabled reads a subject"
    );
    assert!(
        ChecksConfig::default().needs_a_subject(),
        "and the default configuration does"
    );

    // A document that WOULD gather: the count is the claim.
    let plate = twin("docm5-lazy-plate");
    let ev: Evaluation<f64> = corpus::eval(&plate);
    let before = editor_core::gathers_on_this_thread();
    let report = run_checks(&plate, &ev, &off, tol).expect("the registry runs");
    assert_eq!(
        editor_core::gathers_on_this_thread() - before,
        0,
        "no enabled resident reads a subject, so nothing is gathered"
    );
    assert_eq!(
        report.skipped,
        vec![CheckId::Separation],
        "and the skip is visible"
    );

    // A document that would NOT gather: the refusal never arises,
    // because the gather never runs.
    let collide = naming_collision("docm5-lazy-collide");
    let ev: Evaluation<f64> = corpus::eval(&collide);
    assert!(
        product_recorded(&collide, &ev, tol).is_err(),
        "the premise: this document's gather refuses"
    );
    let before = editor_core::gathers_on_this_thread();
    let report = run_checks(&collide, &ev, &off, tol)
        .expect("with the subject-reading resident off there is nothing to gather for");
    assert_eq!(editor_core::gathers_on_this_thread() - before, 0);
    assert_eq!(report.skipped, vec![CheckId::Separation]);
}

/// **DI3 at the door.** (R2's two rows, adopted.)
///
/// `run_checks_on` binds `doc` to `ev` itself rather than inheriting a
/// gather's pairing check, because connectedness reads `doc.roots()`
/// against `ev.value(root)` and a foreign evaluation of one recipe
/// answers every one of those lookups. Both arguments are checked: the
/// evaluation, and the product a `Subject::Product` carries.
#[test]
fn the_subject_door_refuses_an_evaluation_or_a_subject_of_another_document() {
    let tol = Tol::witness();
    let cfg = ChecksConfig::default();
    let overlapping = twin_pair("docm5-di3-overlap", 0.4);
    let apart = twin_pair("docm5-di3-apart", 4.0);
    let ev_overlapping: Evaluation<f64> = corpus::eval(&overlapping);

    // The premise: the two documents are one recipe under two
    // identities, and they say different things.
    let honest = run_checks(&overlapping, &ev_overlapping, &cfg, tol).expect("its own report");
    assert!(
        !honest.findings.is_empty(),
        "the premise: the overlapping twin has something to report"
    );

    let expect_pairing = |got: Result<ChecksReport, ChecksError>, what: &str| match got {
        Err(ChecksError::EvaluationOfAnotherDocument { expected, found }) => {
            assert_eq!(
                expected,
                apart.id(),
                "{what}: names the document asked about"
            );
            assert_eq!(found, overlapping.id(), "{what}: and the one handed");
        }
        Ok(report) => panic!(
            "{what}: the door answered about another document ({} finding(s))",
            report.findings.len()
        ),
        Err(other) => panic!("{what}: expected the pairing refusal, got {other}"),
    };

    // A foreign evaluation with a subject smuggled in beside it.
    let crossed = product_recorded(&overlapping, &ev_overlapping, tol).expect("the twin gathers");
    expect_pairing(
        run_checks_on(
            &apart,
            &ev_overlapping,
            Subject::Product(&crossed),
            &cfg,
            tol,
        ),
        "a foreign product",
    );
    // And with no subject at all — the arm that skips the only resident
    // that could have refused.
    expect_pairing(
        run_checks_on(&apart, &ev_overlapping, Subject::NoBodyRoots, &cfg, tol),
        "no subject",
    );

    // A product of THIS document beside an evaluation of another is the
    // same hole one step later, and is refused for the evaluation.
    let ev_apart: Evaluation<f64> = corpus::eval(&apart);
    let own = product_recorded(&apart, &ev_apart, tol).expect("gathers");
    expect_pairing(
        run_checks_on(&apart, &ev_overlapping, Subject::Product(&own), &cfg, tol),
        "own product, foreign evaluation",
    );
    // The mirror: this document's evaluation with another's product.
    expect_pairing(
        run_checks_on(&apart, &ev_apart, Subject::Product(&crossed), &cfg, tol),
        "own evaluation, foreign product",
    );
    // And the honest pair still runs.
    run_checks_on(&apart, &ev_apart, Subject::Product(&own), &cfg, tol).expect("the true pair");
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

/// **A2 — no resident derives its own subject**, counted rather than
/// grepped: one `run_checks` over a document that gathers costs exactly
/// ONE gather, and the door under it costs none.
///
/// The counter sees every spelling of the gather
/// (`product`/`product_named`/`product_recorded` are one
/// implementation), which is what a source grep cannot promise; the
/// grep beside it is kept only for what the counter cannot see — a
/// gather on a path this document does not take.
#[test]
fn the_registry_gathers_once_and_the_door_under_it_never_does() {
    let tol = Tol::witness();
    let cfg = ChecksConfig::default();
    let doc = twin("docm5-one-gather");
    let ev: Evaluation<f64> = corpus::eval(&doc);

    let before = editor_core::gathers_on_this_thread();
    run_checks(&doc, &ev, &cfg, tol).expect("the registry runs");
    assert_eq!(
        editor_core::gathers_on_this_thread() - before,
        1,
        "one gather per run of the wrapper"
    );

    let subject = product_recorded(&doc, &ev, tol).expect("gathers");
    let before = editor_core::gathers_on_this_thread();
    run_checks_on(&doc, &ev, Subject::Product(&subject), &cfg, tol).expect("the door runs");
    assert_eq!(
        editor_core::gathers_on_this_thread() - before,
        0,
        "and the door, handed its subject, gathers nothing"
    );

    // What the counter cannot see is a gather on a path this document
    // does not take, so the source count stands beside it.
    assert_eq!(
        gathers_in(include_str!("../src/checks.rs")),
        1,
        "and one gather call in the registry's source, in `run_checks`"
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
        "one gather call in the assembly module's source, in `assemble`"
    );

    let tol = Tol::witness();
    let doc = twin("docm5-asm-one-gather");
    let ev: Evaluation<f64> = corpus::eval(&doc);
    let before = editor_core::gathers_on_this_thread();
    assemble(&doc, &ev, tol).expect("the wrapper assembles");
    assert_eq!(
        editor_core::gathers_on_this_thread() - before,
        1,
        "one gather per `assemble`"
    );
    let product = product_recorded(&doc, &ev, tol).expect("gathers");
    let before = editor_core::gathers_on_this_thread();
    assemble_gathered(product, tol).expect("the door assembles");
    assert_eq!(
        editor_core::gathers_on_this_thread() - before,
        0,
        "and none in the door under it"
    );

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
///
/// Read through the shared reader's CODE view
/// ([`test_utils::source::code_only`]), because every needle here is a
/// code fragment — a type application, an impl header, a derive above
/// an item — and the claim is about what these modules DO, not about
/// what their prose says they do not. The view keeps every code byte
/// at its own offset and blanks comments and literals, so a real
/// `Arc<Product>` is still seen exactly where it is written while a
/// sentence naming one stops answering for it. This site's ledger row
/// is in `crates/test-utils/tests/reader_census.rs`.
#[test]
fn nothing_clones_or_shares_the_product() {
    for (path, source) in [
        ("checks.rs", include_str!("../src/checks.rs")),
        ("assembly.rs", include_str!("../src/assembly.rs")),
        ("product.rs", include_str!("../src/product.rs")),
    ] {
        let code = test_utils::source::code_only(source);
        assert!(
            !code.contains("Arc<Product"),
            "{path}: the product is handed on, never shared"
        );
        assert!(
            !code.contains("Clone for Product"),
            "{path}: and never cloned"
        );
    }
    assert!(
        !test_utils::source::code_only(include_str!("../src/product.rs"))
            .contains("#[derive(Debug, Clone)]\npub struct Product"),
        "product.rs: the derive would be the same answer by another spelling"
    );
}

/// Calls to the gather in one source file, read through the shared
/// reader's CODE view.
///
/// [`test_utils::source::code_only`] blanks comments and literals and
/// keeps every code byte at its own offset, so the per-line test below
/// is a SELECTION over that view rather than a second lexer — and the
/// hand-rolled `//`-prefix filter this replaces is gone with it. The
/// view drops two things that filter kept, both in the safe direction:
/// a gather named after code on a line whose comment trails it, and
/// one named inside a string literal. Neither appears in the two files
/// this reads; the lines it matches are byte-identical to the raw
/// ones. This site's ledger row is in
/// `crates/test-utils/tests/reader_census.rs`.
///
/// ALL THREE SPELLINGS: `product`, `product_named` and
/// `product_recorded` are one implementation with fields dropped, so a
/// module that called either of the first two would be gathering just
/// as much. WHAT THIS CANNOT SEE, stated because a count that hides its
/// blind spots is not a receipt: a gather behind an alias or a macro,
/// one reached through a helper in another module, and the DIFFERENCE
/// between a call on a hot path and one on a path nothing takes. The
/// rows above pair it with the debug counter, which sees exactly what
/// this cannot — what an actual run costs — and is blind to what this
/// sees.
fn gathers_in(source: &str) -> usize {
    test_utils::source::code_only(source)
        .lines()
        .filter(|line| {
            line.contains("product_recorded(")
                || line.contains("product_named(")
                || line.contains(" product(")
        })
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

/// Two `Transform`s of one extrude, both roots: each carries the same
/// extrude's minted names, so the two roots' rows collide in the
/// product's one name table (`ProductError::Naming` — the refusal the
/// spec names, and the only gather refusal reachable over a document
/// whose roots all evaluate).
fn naming_collision(id: &str) -> ProfileDoc {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty(DocumentId::derive(id), tol);
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
            distance: len(1.0),
        },
    );
    let moved = |doc, dx: f64| {
        insert(
            doc,
            Node::Transform {
                input: extrude,
                translation: [len(dx), len(0.0), len(0.0)],
                rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                rotation_angle: ang(0.0),
            },
        )
    };
    let (doc, _) = moved(doc, 3.0);
    moved(doc, 6.0).0
}

/// Two extruded squares, the second offset by `apart` in x — two roots,
/// so the separation resident's pair walk actually runs and has
/// something to disagree about between two twins. Node ids are assigned
/// in insertion order, so two of these under two document ids are one
/// recipe under two identities: every lookup a foreign evaluation is
/// asked for answers, which is the state DI3 exists for.
fn twin_pair(id: &str, apart: f64) -> ProfileDoc {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty(DocumentId::derive(id), tol);
    let (doc, first) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile: first,
            distance: len(1.0),
        },
    );
    let (doc, second) = on_frame(
        doc,
        [apart, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    insert(
        doc,
        Node::Extrude {
            profile: second,
            distance: len(1.0),
        },
    )
    .0
}
