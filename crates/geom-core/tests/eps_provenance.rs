//! **ε provenance across the commitment paths** (S22, ruled 2026-08-19).
//!
//! The pure env-bootstrap half is unit-tested inside `tolerance.rs`
//! against the injectable lookup. What cannot be tested there is the
//! part that needs a *fresh process each*: the global `OnceLock`
//! commits once, so "a document stated ε" and "an env bootstrap that
//! nothing overrode" are different processes by construction.
//!
//! Each row below is an `#[ignore]`d probe re-exec'd into its own
//! process with an explicitly CONTROLLED environment — the same
//! self-re-exec pattern `tolerance_init.rs` and `ambiguity_k_env.rs`
//! use. Controlling the environment matters twice over here: the CI
//! ε matrix sets `CAD_TOLERANCE_EPS` for the whole run, so a probe
//! that merely inherited it would assert different things on different
//! matrix rows.
//!
//! **These rows can fail.** A provenance channel that answered a
//! constant would red at least two of them whatever constant it chose:
//! `Env`, `Document`, `Init` and `Default` are each the sole correct
//! answer in some row, and the two document rows differ only in
//! whether `CAD_TOLERANCE_EPS` was set — which is exactly the
//! distinction S22 asked the channel to draw.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::tolerance::{DEFAULT_EPS, ENV_EPS, EpsilonSource, Tolerance};
use geom_core::Tol;

/// Prints the run's provenance and ε in one greppable line.
fn announce(tag: &str) {
    let report = Tolerance::report();
    println!(
        "EPSPROV {tag} source={:?} eps={:e} line={report}",
        report.eps_source, report.tolerance.eps
    );
}

// ---------------------------------------------------------------
// The probes. Each is the sole test in its process.
// ---------------------------------------------------------------

/// No `CAD_TOLERANCE_EPS`: the compiled default, and the report says so.
#[test]
#[ignore]
fn probe_default() {
    // Nothing has asked yet, so nothing has committed — the
    // non-committing door must say `None` rather than force a value.
    assert_eq!(
        Tolerance::committed_report(),
        None,
        "committed_report must not commit"
    );
    assert_eq!(Tol::witness().get().eps, DEFAULT_EPS);
    assert_eq!(Tolerance::eps_source(), EpsilonSource::Default);
    assert!(Tolerance::committed_report().is_some());
    announce("default");
}

/// `CAD_TOLERANCE_EPS` set and accepted, nothing overriding it.
#[test]
#[ignore]
fn probe_env() {
    assert_eq!(Tol::witness().get().eps, 1e-6);
    assert_eq!(Tolerance::eps_source(), EpsilonSource::Env);
    announce("env");
}

/// `CAD_TOLERANCE_EPS` set but MALFORMED: the run decides at the
/// compiled default, so the provenance must say `Default` — and the
/// report must still carry the rejected value (issue #497's shape).
#[test]
#[ignore]
fn probe_env_rejected() {
    assert_eq!(Tol::witness().get().eps, DEFAULT_EPS);
    assert_eq!(Tolerance::eps_source(), EpsilonSource::Default);
    let report = Tolerance::report();
    assert_eq!(report.env_errors.len(), 1);
    assert!(report.to_string().contains("REJECTED"));
    announce("env_rejected");
}

/// An explicit `Tolerance::init` — the env is set and must be ignored
/// by both the value and the provenance.
#[test]
#[ignore]
fn probe_init() {
    Tolerance::init(Tolerance::with_eps(4e-9)).expect("first init");
    assert_eq!(Tol::witness().get().eps, 4e-9);
    assert_eq!(Tolerance::eps_source(), EpsilonSource::Init);
    assert!(Tolerance::report().env_errors.is_empty());
    announce("init");
}

/// A document states ε with no env variable in play.
#[test]
#[ignore]
fn probe_document() {
    Tolerance::init_document_eps(3e-9).expect("document commits");
    assert_eq!(Tol::witness().get().eps, 3e-9);
    assert_eq!(Tolerance::eps_source(), EpsilonSource::Document);
    announce("document");
}

/// **The row S22 named.** `CAD_TOLERANCE_EPS` IS set, and a document
/// states a different ε before anything reads the env. The document
/// wins — that ranking is unchanged — and the provenance must say
/// `Document`, not `Env`, because the env value was never read.
#[test]
#[ignore]
fn probe_document_over_env_bootstrap() {
    Tolerance::init_document_eps(3e-9).expect("document commits first");
    assert_eq!(
        Tol::witness().get().eps,
        3e-9,
        "the document's ε outranks an unread {ENV_EPS}"
    );
    assert_eq!(Tolerance::eps_source(), EpsilonSource::Document);
    announce("document_over_env");
}

/// The complement: the env bootstrap committed FIRST, then a document
/// agreed with it bit-for-bit (the benign-reload path the persistence
/// layer accepts). The number came from the environment and the
/// provenance must keep saying so.
#[test]
#[ignore]
fn probe_env_bootstrap_then_agreeing_document() {
    assert_eq!(Tol::witness().get().eps, 1e-6, "env bootstrap commits first");
    assert_eq!(Tolerance::eps_source(), EpsilonSource::Env);
    // A matching document is a benign reload: `AlreadyInitialized`
    // carrying the equal value, which the persistence layer treats as
    // success. It supplied nothing, so it renames nothing.
    assert!(Tolerance::init_document_eps(1e-6).is_err());
    assert_eq!(Tolerance::eps_source(), EpsilonSource::Env);
    announce("env_then_agreeing_document");
}

// ---------------------------------------------------------------
// The driver: one process per probe, environment stated explicitly.
// ---------------------------------------------------------------

/// Runs `probe` alone in a fresh process with `eps` as
/// `CAD_TOLERANCE_EPS` (`None` = the variable genuinely unset, which
/// an empty string would NOT be — that parses as malformed).
fn run_probe(probe: &str, eps: Option<&str>) -> String {
    let exe = std::env::current_exe().unwrap();
    // Name the probe by MODULE PATH: `tests/all.rs` aggregates every
    // suite into one binary, so libtest sees `<this_module>::<probe>`,
    // while a standalone `module_path!()` has no `::` at all.
    let filter = match module_path!().split_once("::") {
        Some((_, m)) => format!("{m}::{probe}"),
        None => probe.to_string(),
    };
    let mut cmd = std::process::Command::new(&exe);
    cmd.args([filter.as_str(), "--ignored", "--exact", "--nocapture"]);
    if let Some(v) = eps {
        cmd.env(ENV_EPS, v);
    } else {
        cmd.env_remove(ENV_EPS);
    }
    // K is not under test and the matrix never sets it, but an ambient
    // value would ride into the report line — remove it so every row
    // reads the same K.
    cmd.env_remove("CAD_AMBIGUITY_K");
    let out = cmd.output().unwrap();
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(
        out.status.success(),
        "probe {probe} failed in its fresh process:\n{text}\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    text
}

#[test]
fn every_commitment_path_reports_its_own_provenance() {
    let rows = [
        ("probe_default", None, "source=Default"),
        ("probe_env", Some("1e-6"), "source=Env"),
        ("probe_env_rejected", Some("bogus"), "source=Default"),
        ("probe_init", Some("1e-6"), "source=Init"),
        ("probe_document", None, "source=Document"),
        (
            "probe_document_over_env_bootstrap",
            Some("1e-6"),
            "source=Document",
        ),
        (
            "probe_env_bootstrap_then_agreeing_document",
            Some("1e-6"),
            "source=Env",
        ),
    ];
    for (probe, eps, expected) in rows {
        let text = run_probe(probe, eps);
        assert!(
            text.contains("EPSPROV") && text.contains(expected),
            "probe {probe} (CAD_TOLERANCE_EPS = {eps:?}) should report \
             {expected}, got:\n{text}"
        );
    }
}
