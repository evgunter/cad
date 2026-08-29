//! Aggregated integration-test binary for `mesh`.
//!
//! Every `tests/*.rs` suite is included here VERBATIM via `#[path]`, so
//! this one binary stands in for one test target per suite.
//! The suite count is deliberately NOT restated in prose here:
//! `every_suite_file_is_aggregated` below checks this file against the
//! directory on every run, and a number written out beside it is a
//! second, unchecked copy of a set the compiler already knows.
//!
//! The files themselves are untouched: each keeps its own `//!` docs, its inner
//! attributes (`#![cfg(feature = "interval")]` and friends work as
//! module-level attributes), and its own `mod <helper>;` lines — a
//! `#[path]` module's child modules resolve against the DIRECTORY
//! CONTAINING the path file, i.e. `tests/`, exactly as when each file was
//! its own crate root.
//!
//! WHY ONE BINARY: on the CI runner (2 vCPU) the per-binary codegen+link
//! constant dominated the workspace build job — the suites are small, so
//! that constant was the bill. The figures are deliberately NOT restated
//! here: they were measured once, nothing in the repo re-takes them, and
//! the LINK/DEBUGINFO note in .github/workflows/ci.yml is the one place
//! that carries them with their date, their provenance run and the record
//! of what has since changed.
//!
//! ADDING A SUITE: drop the file in `tests/` AND add a `#[path]` line
//! below. `autotests = false` in Cargo.toml means a file that is not
//! listed here does not compile and does not run — `every_suite_file_is_
//! aggregated` below fails loudly if you forget.
//!
//! Test IDs gain a module prefix (`export::round_trip` rather than
//! `round_trip`, under binary `all` rather than binary `export`); the set
//! of tests is otherwise identical.

// Each suite keeps its own verbatim `mod <helper>;`, so a shared helper is
// loaded once per suite that uses it. That is deliberate — the alternative
// is editing the suites — and it is what `duplicate_mod` is warning about.
// Allowed HERE ONLY, by name: no blanket `#![allow]`, which would weaken
// the lint gate for every suite module included below.
#![allow(clippy::duplicate_mod)]

#[path = "budget_meter.rs"]
mod budget_meter;
#[path = "errors.rs"]
mod errors;
#[path = "exact_vs_mesh.rs"]
mod exact_vs_mesh;
#[path = "fitted_refusals.rs"]
mod fitted_refusals;
#[path = "genus.rs"]
mod genus;
#[path = "issue111_az_needle.rs"]
mod issue111_az_needle;
#[path = "m5_pr11_trimmed.rs"]
mod m5_pr11_trimmed;
#[path = "m5_s10_face_sense.rs"]
mod m5_s10_face_sense;
#[path = "m5_s11_concave_sense.rs"]
mod m5_s11_concave_sense;
#[path = "m7_nurbs_trimmed.rs"]
mod m7_nurbs_trimmed;
#[path = "newell_probes.rs"]
mod newell_probes;
#[path = "prisms.rs"]
mod prisms;
#[path = "probe_review.rs"]
mod probe_review;
#[path = "profile_overrides.rs"]
mod profile_overrides;
#[path = "review_m2_pr6_cert_oracle.rs"]
mod review_m2_pr6_cert_oracle;
#[path = "review_m2_pr6_checkmesh_audit.rs"]
mod review_m2_pr6_checkmesh_audit;
#[path = "review_m2_pr6_determinism.rs"]
mod review_m2_pr6_determinism;
#[path = "review_m2_pr6_errors.rs"]
mod review_m2_pr6_errors;
#[path = "review_m2_pr6_walk_shapes.rs"]
mod review_m2_pr6_walk_shapes;
#[path = "review_m3_pr1_mesh.rs"]
mod review_m3_pr1_mesh;
#[path = "revolves.rs"]
mod revolves;
#[path = "wedge.rs"]
mod wedge;

/// Guards the `autotests = false` hazard: a suite file added under
/// `tests/` but not declared above would silently stop being compiled
/// and run. Both directions are asserted — every file on disk is
/// declared, and every declaration answers to a file, so no number
/// about this file is stated in prose without being computed.
///
/// The walk is `test_utils::source::suite_files`, which recurses into
/// group directories and tells a suite from a shared helper by Rust's
/// own module rule; read it before adding either.
#[test]
// Scoped to this fn on purpose: a crate-root `#![allow]` in this file would
// weaken the lint gate for every suite module included above.
#[allow(clippy::expect_used)]
fn every_suite_file_is_aggregated() {
    let root = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("tests");
    // Comments blanked, string literals KEPT — see
    // `test_utils::source::code_and_literals`, which states why.
    let src = test_utils::source::code_and_literals(include_str!("all.rs"));
    let found = test_utils::source::suite_files(&root);
    let missing: Vec<&String> = found
        .iter()
        .filter(|rel| !src.contains(&format!("#[path = \"{rel}\"]")))
        .collect();
    assert!(
        missing.is_empty(),
        "suites under tests/ are not declared in tests/all.rs, so `autotests = false` \
         is silently dropping them: {missing:?}. Add a `#[path]` line for each."
    );
    // The converse, computed rather than restated: one `#[path]` line
    // per suite file, no orphan declaration. The `format!` above spells
    // its quote ESCAPED, so it is not one of these matches.
    let declared = src.matches("#[path = \"").count();
    assert_eq!(
        declared,
        found.len(),
        "tests/all.rs declares {declared} suites but {} suite files exist under tests/",
        found.len()
    );
}

/// **The ε inventory — `sizing::SizingTols`'s ledger written as a gate rather
/// than as a sentence.**
///
/// Every `eps` identifier in the PRODUCTION half of `crates/mesh/src`
/// (comments and literal bodies removed by
/// [`test_utils::source::code_only`]) is counted per file and pinned
/// below, so a new ε read cannot land without either appearing in this
/// table or turning it red.
///
/// **Why a gate and not a list.** `sizing::SizingTols` used to carry a list —
/// *"ε reaches three places from here and no more"* — and it was short
/// by one for two milestones: `walk::iso_side_starts` reads ε to decide
/// whether a traversal opens an iso side or repeats its predecessor's
/// coordinate bitwise. A wrong list is invisible. This row is what
/// makes the ledger falsifiable; the *meaning* of each read stays at
/// its own site, because a count cannot carry it.
///
/// # What the tokens are, per file
///
/// Roles, not line numbers: lines move, and this crate's own S65
/// record cites by name for that reason. A **hand-off** passes `eps`
/// on; a **terminal read** compares or adds it.
///
/// - **`tessellate.rs` — 3.** ε ENTERS the crate here and nowhere
///   else: `Tol::witness().get().eps` (two tokens on one line) and the
///   `Tol` field initialiser.
/// - **`sizing.rs` — 1.** The `Tol::eps` field declaration. This
///   module computes every step and count in the crate and reads ε in
///   none of them.
/// - **`curved.rs` — 6.** Two hand-offs out of `Tol`
///   (`walk::loop_polygon`, `require_swept_rectangle`), two `eps`
///   parameters, one hand-off to `entries_off_bbox`, and
///   `entries_off_bbox`'s own single `gap_is_noise` call — the banded
///   swept-rectangle domain guard, which REFUSES a face and moves no
///   coordinate. 2 + 2 + 1 + 1 = 6. It was **7** until #887 gave the
///   guard a degenerate-lever arm and the two axes' calls became one
///   closure applied twice; the read's KIND did not change, and the
///   guard is still ONE of the four `gap_is_noise` call sites below.
/// - **`trimmed.rs` — 1.** `d / (bound + tol.eps)` in the deviation
///   probe. **Not a bar**: ε is a continuous addend in a denominator,
///   so it scales `worst_ratio` — a `pub` measurement field — at every
///   call, monotonically. The block is absent from a default build
///   (`budget` feature). Counted after a cut since #887, which gave
///   the file its first test module; the total did not move, because
///   that module reads no ε.
/// - **`walk.rs` — 13.** **Four** `eps` parameters (`gap_is_noise`,
///   `closing_column`, `iso_side_starts`, `loop_polygon`), **five**
///   hand-offs (`closing_column`'s and `loop_polygon`'s two
///   `gap_is_noise` calls, `loop_polygon`'s calls to
///   `iso_side_starts` and `closing_column`), and **four** terminal
///   reads: `gap_is_noise`'s `gap * lever < eps` (one predicate, four
///   call sites — the domain guard above plus three `debug_assert`
///   detectors that gate nothing), `iso_side_starts`' `radial > eps`,
///   `pole_v`'s `norm() <= eps`, and `loop_polygon`'s
///   `coincident_declared` closure, whose `d <= eps` asks whether two
///   DECLARED vertices of one loop are the same point.
///   4 + 5 + 4 = 13.
///
///   It was **12** until `coincident_declared` landed. That read
///   **gates nothing and moves no coordinate**: it is the condition of
///   a `debug_assert` (D2 addendum row 5), so its only effect is to
///   panic. It CAPTURES `eps` rather than taking it as a parameter,
///   which is why only the terminal-read term moved.
///
/// Eight consumer sites, five terminal reads across the crate. **The
/// per-file totals above are pinned; every other number in this doc is
/// hand-written and is not.** They are checkable — each file's
/// breakdown sums to its pinned total, which is the arithmetic a
/// reader can run and the reason it is written that way. An earlier
/// draft of `walk.rs`'s line said *"three parameters, four hand-offs
/// and three terminal reads"* and summed to 10 against a pinned 12: a
/// hand-written narrative that did not add up, in the doc of the pin
/// that replaced a hand-written list for going stale.
///
/// **That has now happened twice.** #887 moved `curved.rs` from 7 to
/// 6, re-pinned the total, and left this doc's breakdown of it summing
/// to the old number — the pin stayed GREEN while its own account of
/// what it counted went false, which is the failure mode the pin
/// exists to make impossible one level down. The arithmetic above is
/// the only thing standing between a reader and that, and it is
/// hand-run. **Whoever next changes an ε read re-runs every sum on
/// this list, not just the file they touched.**
///
/// # What this cannot match, and it is a work order
///
/// 1. **A read that does not spell `eps`.** `Tol::witness().get().eps`
///    bound to another name, or ε reached through a helper that
///    already applied it, is invisible here. The mechanism that would
///    close that is a TYPE — ε as a newtype whose only operations are
///    named — and it spans `walk`, `curved`'s guard bodies, `trimmed`,
///    `tessellate` and `sizing` at once. Filed as **issue #881**.
/// 2. **Which KIND of read it is.** This row reds when the inventory
///    moves; it cannot say whether the new read refuses, classifies or
///    scales. That judgement is the reader's, at the site.
/// 3. **The test half of each file** — deliberately, because a test
///    that reads ε is not a place ε reaches the mesh. **The cut is
///    crude and its failure modes are not symmetric.** It is the first
///    line equal to `#[cfg(test)]` at column 0; the row asserts there
///    is at most one, so the cut is unambiguous. A file with no such
///    line counts WHOLE — conservative, so it over-counts rather than
///    under-counts. `tessellate.rs` is the crate's only such file now;
///    `trimmed.rs` was one until #887 gave it a test module, which is
///    why this sentence states the RULE and treats the roster as
///    perishable. An
///    INDENTED `#[cfg(test)]` does not cut at all (`nurbs_cert.rs` has
///    two, gating items inside a module) — also conservative. The one
///    unsound direction is **production code placed after a trailing
///    test module**, which this cut would not see; nothing in the crate
///    is written that way and nothing checks that it stays so.
/// 4. **Compensating changes inside one file.** A read deleted and
///    another added in the same file leaves the total unmoved. A read
///    MOVED between files IS caught — both counts change.
/// 5. **An identifier a macro assembles** (`concat_idents!`, `paste!`),
///    or one inside an `include!`d file. [`test_utils::source::code_only`]
///    is a lexer, not an expander, so this one is a property of the
///    METHOD and not of this tree: there is nothing to re-check per
///    crate, and no mechanism could close it short of expansion.
///    **The raw-string hole that used to stand here is gone**, and it
///    was closed by a mechanism rather than by a claim: the shared
///    lexer now models all five string prefixes, pinned by
///    `test_utils::source`'s own
///    `a_raw_string_closes_at_its_own_delimiter_and_loses_no_following_code`,
///    which is the register that re-takes it. This row therefore no
///    longer asserts anything about raw strings in `crates/mesh/src` —
///    it does not need to.
///
/// # The walk is shared, and that was not free
///
/// The traversal is [`test_utils::source::rust_sources`], **recursive**.
/// An earlier version of this row hand-rolled a flat `read_dir`, which
/// left `crates/mesh/src/<any-subdir>/*.rs` invisible — the pin stayed
/// green over two planted reads. Sharing the *predicate* and re-forking
/// the *walk* is half a fix: `topo`'s equivalent traversal
/// (`source_walk::crate_sources`) is `pub(crate)`, the identical
/// obstacle `code_only` was moved to remove, and re-forking it
/// reproduced exactly the defect the sharing was for.
#[test]
#[allow(clippy::expect_used)]
fn the_eps_inventory_is_pinned() {
    const PINNED: [(&str, usize); 5] = [
        ("curved.rs", 6),
        ("sizing.rs", 1),
        ("tessellate.rs", 3),
        ("trimmed.rs", 1),
        ("walk.rs", 13),
    ];
    let src = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    let needle = concat!("e", "ps");
    let mut found: Vec<(String, usize)> = Vec::new();
    for path in test_utils::source::rust_sources(&src) {
        let name = path
            .strip_prefix(&src)
            .expect("a walked file lies under mesh/src")
            .to_string_lossy()
            .replace('\\', "/");
        let text = std::fs::read_to_string(&path).expect("a readable source file");
        let code = test_utils::source::code_only(&text);
        let cuts = code.lines().filter(|l| *l == "#[cfg(test)]").count();
        assert!(
            cuts <= 1,
            "{name} has {cuts} top-level `#[cfg(test)]` lines, so the production/test \
             cut is ambiguous. See this row's docs on what the cut assumes."
        );
        let prod: String = code
            .lines()
            .take_while(|l| *l != "#[cfg(test)]")
            .collect::<Vec<_>>()
            .join("\n");
        // Identifier occurrences, not substrings: `steps` and
        // `grid_steps` are not ε reads and outnumber the real ones.
        let reads = prod
            .match_indices(needle)
            .filter(|(i, _)| {
                let before = prod[..*i].chars().next_back();
                let after = prod[i + needle.len()..].chars().next();
                let ident = |c: Option<char>| c.is_some_and(|c| c.is_alphanumeric() || c == '_');
                !ident(before) && !ident(after)
            })
            .count();
        if reads > 0 || PINNED.iter().any(|(pinned, _)| *pinned == name) {
            found.push((name, reads));
        }
    }
    found.sort();
    let pinned: Vec<(String, usize)> = PINNED
        .iter()
        .map(|(path, reads)| ((*path).to_string(), *reads))
        .collect();
    assert_eq!(
        found, pinned,
        "the ε inventory moved. That is not a failure to silence: a read was added, \
         removed or renamed, and `sizing::SizingTols`'s ledger plus this row's per-file \
         breakdown both owe an update. Classify the new read at its site — does it \
         REFUSE, CLASSIFY, or SCALE a number? — then re-pin."
    );
}
