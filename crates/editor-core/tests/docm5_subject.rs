//! **The registry's subject, gathered once.**
//!
//! The registry took the shape a caller with a product in hand needs,
//! and kept a wrapper for the caller that has none. What that owes is a
//! claim of EQUALITY rather than of behaviour: the wrapper is the gather
//! plus the door, so on any document where the gather succeeds the two
//! spellings must be indistinguishable.
//!
//! Where they part is the refusal, and it is stated rather than
//! implied: `ChecksError::Product` is the WRAPPER's arm — the gather is
//! its — and a door handed a subject cannot raise it.
//!
//! **The gather counter is `cfg(debug_assertions)`, and the statements
//! that read it carry the same gate.** `gathers_on_this_thread` exists
//! only where debug assertions do, so a row here that counted gathers
//! unconditionally made the whole crate uncompilable with them off. The
//! gate is per STATEMENT, not per row: no test becomes debug-only, and
//! every assertion that does not read the counter — the two spellings'
//! equality, the skipped lists, the source counts, the refusal arms —
//! is checked in every profile.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::corpus;
use crate::fixture;

use editor_core::{
    Advisory, CheckId, ChecksConfig, ChecksError, ChecksReport, DocumentId, Evaluation, Node,
    NodeStanding, ProductError, ProfileDoc, RecipeNodeId, Severity, Subject, product_recorded,
    run_checks, run_checks_on,
};
use fixture::{insert, len, on_frame, square};
use geom_core::Tol;

/// **A world whose gather refuses**: a cube instance pinned to the
/// world and a second placed on it by a constraint whose side names a
/// face the part does not have, so that placement fails and the gather
/// refuses it typed. Answers the document, the options that resolve its
/// part, and the failed placement.
fn failed_placement_world(id: &str) -> (ProfileDoc, editor_core::EvalOptions, RecipeNodeId) {
    use editor_core::{CapEnd, EntityKind, RoleSeg, StableName};
    let mut store = fixture::resolver::PartStore::default();
    let part = ProfileDoc::empty_derived(&format!("{id}-part"), Tol::witness());
    let (part, profile) = on_frame(
        part,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (part, body) = insert(
        part,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let (part_ref, body) = store.insert_part((part, body), Tol::witness());
    let opts = fixture::resolver::with_resolver(store);
    let doc = ProfileDoc::empty_derived(id, Tol::witness());
    let (doc, base) = fixture::insert_pinned(doc, crate::fixture::mated_instance(part_ref), &opts);
    let (doc, placed) = insert(doc, fixture::mated_instance(part_ref));
    let nowhere = StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId::new(0, 99),
        path: vec![RoleSeg::Cap(CapEnd::End)],
    };
    let mate = fixture::rest(
        fixture::head_at(
            placed,
            fixture::resolver::in_part(placed, body, CapEnd::Start),
        ),
        fixture::head_at(base, nowhere),
    );
    let (doc, _) = fixture::constrain_at(doc, placed, mate);
    (doc, opts, placed)
}

/// The two spellings of the registry over one document.
fn both_ways(doc: &editor_core::ProfileDoc, ev: &Evaluation<f64>) -> (ChecksReport, ChecksReport) {
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
/// whose world holds nothing reaches `Subject::EmptyProduct` through
/// the wrapper and reports clean; the same subject spelled by hand
/// reports the same thing.
#[test]
fn an_empty_world_is_the_empty_product_subject() {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty_derived("docm5-empty", tol);
    let ev: Evaluation<f64> = corpus::eval(&doc);
    assert!(
        matches!(
            product_recorded(&doc, &ev, tol).expect_err("nothing is placed"),
            ProductError::EmptyProduct
        ),
        "the premise: this is the arm the wrapper maps to a subject"
    );
    let cfg = ChecksConfig::default();
    let wrapped = run_checks(&doc, &ev, &cfg, tol).expect("an empty document is checkable");
    let direct = run_checks_on(&doc, &ev, Subject::EmptyProduct, &cfg, tol).expect("so is it here");
    assert_eq!(wrapped, direct);
    assert_eq!(wrapped.findings, Vec::new(), "and it is clean");
    assert_eq!(
        wrapped.skipped,
        vec![CheckId::ChartCoherence],
        "with the one default-Off resident named, and nothing else"
    );
}

/// **A2 — a gather that refuses reaches the door as
/// `Subject::Unavailable`, and the door raises it AFTER the residents
/// that read no subject have answered.**
///
/// The gather refusal here is a placement that failed: a constraint
/// whose side names no face of its part fails the placement holding it,
/// and the gather refuses that placement typed. Connectedness reads the
/// evaluation, so where it runs it answers first, refusing the same
/// placement's standing; with it off, the at-rest resident reads the
/// subject, finds none, and the door refuses `Product` carrying the
/// gather's own sentence. Nothing is run and discarded, and no arm
/// claims the document denotes no body when it does.
#[test]
fn a_gather_refusal_reaches_the_door_and_refuses_after_the_subject_free_residents() {
    let tol = Tol::witness();
    let (doc, opts, placement) = failed_placement_world("docm5-collide");
    let ev: Evaluation<f64> = crate::fixture::run(&doc, &opts);
    let gathered = || product_recorded(&doc, &ev, tol).expect_err("a failed placement");
    let refusal = gathered();
    assert!(
        matches!(
            refusal,
            ProductError::Read(NodeStanding::Poisoned { through, .. }) if through == placement
        ),
        "the premise: the gather refuses the product's `⊔`, poisoned through the failed \
         placement: {refusal:?}"
    );
    let carries_the_gathers_refusal = |err: ChecksError| match &err {
        ChecksError::Product {
            refusal: Some(carried),
        } => {
            assert_eq!(
                format!("{:?}", carried.error()),
                format!("{refusal:?}"),
                "the gather's own refusal, whole"
            );
            assert_eq!(err.to_string(), format!("checks: {refusal}"));
            assert_eq!(
                err.spoken(&doc),
                format!("checks: {}", refusal.spoken(&doc)),
                "said by the frame holding the checked document"
            );
        }
        other => panic!("expected the subject refusal, got {other}"),
    };

    let subject_free_off = ChecksConfig {
        connectedness: Severity::Off,
        ..ChecksConfig::default()
    };
    // Through the wrapper, which gathers.
    carries_the_gathers_refusal(
        run_checks(&doc, &ev, &subject_free_off, tol).expect_err("the registry refuses"),
    );
    // And through the door, handed the same fact directly.
    carries_the_gathers_refusal(
        run_checks_on(
            &doc,
            &ev,
            Subject::refused(gathered()),
            &subject_free_off,
            tol,
        )
        .expect_err("the door refuses"),
    );

    // Where connectedness runs, it reads the evaluation and answers
    // first, through either spelling: its own refusal of the failed
    // placement comes before the subject's.
    let standing = ChecksError::Placement(NodeStanding::Failed { node: placement });
    assert_eq!(
        run_checks(&doc, &ev, &ChecksConfig::default(), tol).expect_err("the registry refuses"),
        standing
    );
    assert_eq!(
        run_checks_on(
            &doc,
            &ev,
            Subject::refused(gathered()),
            &ChecksConfig::default(),
            tol
        )
        .expect_err("the door refuses"),
        standing
    );
}

/// **The gather is LAZY: a run no resident needs a subject for takes
/// none.** (R2's `…_does_not_need_a_subject` and
/// `…_still_pays_for_the_gather`, adopted — the second as a COUNT
/// rather than a stopwatch, which is the same claim without a clock in
/// it.)
///
/// With the subject-reading residents `Off`, a document whose gather
/// refuses reports cleanly and a document whose gather succeeds pays
/// nothing: the counter reads 0 across the call, and the crossed pair
/// below — which the gather refuses — reports.
#[test]
fn a_run_that_needs_no_subject_does_not_gather() {
    let tol = Tol::witness();
    let off = ChecksConfig {
        unproven_coincidence: Advisory::Off,
        at_rest: Severity::Off,
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
    #[cfg(debug_assertions)]
    let before = editor_core::gathers_on_this_thread();
    let report = run_checks(&plate, &ev, &off, tol).expect("the registry runs");
    #[cfg(debug_assertions)]
    assert_eq!(
        editor_core::gathers_on_this_thread() - before,
        0,
        "no enabled resident reads a subject, so nothing is gathered"
    );
    assert_eq!(
        report.skipped,
        vec![
            CheckId::ChartCoherence,
            CheckId::UnprovenCoincidence,
            CheckId::AtRest
        ],
        "and the skip is visible"
    );

    // A document that would NOT gather: the refusal never arises,
    // because the gather never runs.
    let (collide, opts) = crate::fixture::unplaced_world("docm5-lazy-collide");
    let ev: Evaluation<f64> = crate::fixture::run(&collide, &opts);
    assert!(
        product_recorded(&collide, &ev, tol).is_err(),
        "the premise: this document's gather refuses"
    );
    #[cfg(debug_assertions)]
    let before = editor_core::gathers_on_this_thread();
    let report = run_checks(&collide, &ev, &off, tol)
        .expect("with the subject-reading residents off there is nothing to gather for");
    #[cfg(debug_assertions)]
    assert_eq!(editor_core::gathers_on_this_thread() - before, 0);
    assert_eq!(
        report.skipped,
        vec![
            CheckId::ChartCoherence,
            CheckId::UnprovenCoincidence,
            CheckId::AtRest
        ]
    );
}

/// **The coincidence lint needs the product.** The at-rest rows ride
/// it: a mated pair's, and every contact between two copies (D10), so a
/// document whose gather refuses has rows nobody can read, mate or no
/// mate (a failed placement here). With the at-rest check and connectedness
/// (which reads the evaluation, and would answer first) off and the
/// lint on, the run refuses with the gather's refusal rather than
/// reporting the nodes' rows as if they were all.
#[test]
fn the_coincidence_lint_refuses_when_the_gather_refuses() {
    let tol = Tol::witness();
    let cfg = ChecksConfig {
        connectedness: Severity::Off,
        unproven_coincidence: Advisory::Warn,
        at_rest: Severity::Off,
        ..ChecksConfig::default()
    };
    let (collide, opts, _) = failed_placement_world("docm5-lint-collide");
    let ev: Evaluation<f64> = crate::fixture::run(&collide, &opts);
    assert!(
        product_recorded(&collide, &ev, tol).is_err(),
        "the premise: this document's gather refuses"
    );
    assert!(
        matches!(
            run_checks(&collide, &ev, &cfg, tol),
            Err(editor_core::ChecksError::Product { .. })
        ),
        "the lint's at-rest rows ride the product"
    );
}

/// **DI3 at the door.** (R2's two rows, adopted.)
///
/// `run_checks_on` binds `doc` to `ev` itself rather than inheriting a
/// gather's pairing check, because connectedness reads
/// `doc.placements()` against `ev.value(placement)` and a foreign
/// evaluation of one recipe answers every one of those lookups. Both arguments are checked: the
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
        run_checks_on(&apart, &ev_overlapping, Subject::EmptyProduct, &cfg, tol),
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
/// does.** A placement without a value in this evaluation means the
/// checks could not run AT ALL, and that holds whether or not the
/// document would also have gathered — so the refusal names the
/// placement, rather
/// than forwarding the gather's account of the same state.
#[test]
fn a_placement_without_a_value_refuses_as_the_registrys_own_precondition() {
    let tol = Tol::witness();
    let (doc, placement) = failing_placement("docm5-precedence");
    let ev: Evaluation<f64> = corpus::eval(&doc);
    assert!(
        ev.value(placement).is_none(),
        "the premise: the placement produced no value"
    );
    assert!(
        product_recorded(&doc, &ev, tol).is_err(),
        "and the gather refuses this pair too, so the two arms compete"
    );
    match run_checks(&doc, &ev, &ChecksConfig::default(), tol).expect_err("refuses") {
        ChecksError::Placement(standing) => assert_eq!(standing.node(), placement),
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

    #[cfg(debug_assertions)]
    let before = editor_core::gathers_on_this_thread();
    run_checks(&doc, &ev, &cfg, tol).expect("the registry runs");
    #[cfg(debug_assertions)]
    assert_eq!(
        editor_core::gathers_on_this_thread() - before,
        1,
        "one gather per run of the wrapper"
    );

    let subject = product_recorded(&doc, &ev, tol).expect("gathers");
    #[cfg(debug_assertions)]
    let before = editor_core::gathers_on_this_thread();
    run_checks_on(&doc, &ev, Subject::Product(&subject), &cfg, tol).expect("the door runs");
    #[cfg(debug_assertions)]
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

/// **Nothing shares the product to get around the ordering.** The
/// landing's three consumers are ordered so that one gather is enough;
/// a `Clone` or an `Arc` on `Product<T>` would be the other answer, and
/// these modules do not take it. The needles end at the generic's `<`,
/// so a type that only shares the prefix (`ProductError`) is not one.
///
/// One copy is made, and it is not this ordering's: a door's census
/// verdict is the door's, kept on its value (`product::Census`) for
/// every reader of it, and a gather that reads the door hands its
/// consumer a copy of that verdict to own (`Product::fresh`). The
/// verdict itself is lent, never put behind an `Arc`.
///
/// Read through the shared reader's CODE view
/// ([`test_utils::source::code_only`]), because every needle here is a
/// code fragment — a type application, an impl header, a derive above
/// an item — and the claim is about what these modules DO, not about
/// what their prose says they do not. The view keeps every code byte
/// at its own offset and blanks comments and literals, so a real
/// `Arc<Product<T>>` is still seen exactly where it is written while a
/// sentence naming one stops answering for it. This site's ledger row
/// is in `crates/test-utils/tests/reader_census.rs`.
#[test]
fn nothing_clones_or_shares_the_product() {
    for (path, source) in [
        ("checks.rs", include_str!("../src/checks.rs")),
        (
            "checks/at_rest.rs",
            include_str!("../src/checks/at_rest.rs"),
        ),
        ("product.rs", include_str!("../src/product.rs")),
    ] {
        let code = test_utils::source::code_only(source);
        assert!(
            !code.contains("Arc<Product<"),
            "{path}: the product is handed on, never shared"
        );
        assert!(
            !code.contains("Clone for Product<"),
            "{path}: and never cloned"
        );
    }
    assert!(
        !test_utils::source::code_only(include_str!("../src/product.rs"))
            .contains("#[derive(Debug, Clone)]\npub struct Product<"),
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
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    crate::fixture::place(doc, body).0
}

/// A document whose one placement does not evaluate: two slabs whose
/// caps stand apart by a sliver inside the ambiguity band, united and
/// placed. The boolean refuses the in-band pair, so the placement is
/// live and valueless — the state both the registry's precondition and
/// the gather have something to say about. Answers the placement.
fn failing_placement(id: &str) -> (ProfileDoc, RecipeNodeId) {
    let tol = Tol::witness();
    let doc = ProfileDoc::empty(DocumentId::derive(id), tol);
    // The lower slab half as tall at first, so the union evaluates and
    // its copy stands in the world on the lower slab's bottom.
    let (doc, a) = slab(doc, 0.5);
    // A gap strictly inside the ambiguity band: the margins decide the
    // two caps neither one carrier nor two, so the union refuses.
    let band = tol.get();
    let gap = 0.5 * (band.eps + band.k * band.eps);
    let (doc, b) = slab(doc, 1.0);
    let (doc, b) = crate::fixture::slid(doc, a, b, [0.0, 0.0, 1.0 + gap]);
    let (doc, fused) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
    );
    let (doc, placement) = crate::fixture::place(doc, fused);
    // Then the lower slab raised to a unit height, its top `gap` under
    // the upper slab's bottom.
    let (doc, _) = crate::fixture::step(
        doc,
        editor_core::DocEdit::SetParam {
            node: a,
            slot: editor_core::SlotId::Distance,
            value: len(1.0).into(),
            fresh: Vec::new(),
        },
    );
    (doc, placement)
}

/// A unit-footprint slab on its own xy, up by `dz`.
fn slab(doc: ProfileDoc, dz: f64) -> (ProfileDoc, RecipeNodeId) {
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
            profile: profile.into(),
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
    let doc = corpus::heatsink::placed_fins(160);
    let ev: Evaluation<f64> = corpus::eval(&doc);
    let product = product_recorded(&doc, &ev, tol).expect("the heat sink gathers");
    assert_eq!(
        (product.body.solids().count(), product.body.faces().count()),
        (161, 966),
        "the point the registry split is measured at: the base and 160 fins, six faces each"
    );
}

/// Two extruded squares, the second offset by `apart` in x, each placed,
/// so the at-rest resident has a pair to read and has
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
    let (doc, a) = insert(
        doc,
        Node::Extrude {
            profile: first.into(),
            distance: len(1.0),
        },
    );
    let (doc, second) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(apart, 0.0, 0.5)],
    );
    let (doc, b) = insert(
        doc,
        Node::Extrude {
            profile: second.into(),
            distance: len(1.0),
        },
    );
    crate::fixture::place_all(doc, &[a, b])
}
