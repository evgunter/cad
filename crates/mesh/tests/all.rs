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

#[path = "cert10r1_assembly_accounting.rs"]
mod cert10r1_assembly_accounting;

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
#[path = "issue303_signed_volume_recentring.rs"]
mod issue303_signed_volume_recentring;

#[path = "iso_rectangle_door.rs"]
mod iso_rectangle_door;
#[path = "issue1362_band_placement.rs"]
mod issue1362_band_placement;
#[path = "issue555_subfloor_cap.rs"]
mod issue555_subfloor_cap;
#[path = "issue685_nu1_sizing.rs"]
mod issue685_nu1_sizing;
#[path = "issue896_pole_guard.rs"]
mod issue896_pole_guard;
#[path = "issue897_s65_cost.rs"]
mod issue897_s65_cost;
#[path = "m5_pr11_trimmed.rs"]
mod m5_pr11_trimmed;
#[path = "m5_s10_face_sense.rs"]
mod m5_s10_face_sense;
#[path = "m5_s11_concave_sense.rs"]
mod m5_s11_concave_sense;
#[path = "m7_nurbs_trimmed.rs"]
mod m7_nurbs_trimmed;
#[path = "mesh7r1_probes.rs"]
mod mesh7r1_probes;
#[path = "newell_probes.rs"]
mod newell_probes;
#[path = "prisms.rs"]
mod prisms;
#[path = "probe_review.rs"]
mod probe_review;
#[path = "profile_overrides.rs"]
mod profile_overrides;
#[path = "r1_probe_bool_route.rs"]
mod r1_probe_bool_route;
#[path = "r1_probe_hash.rs"]
mod r1_probe_hash;
#[path = "r1_probes_issue1362.rs"]
mod r1_probes_issue1362;
#[path = "r1_probes_issue303.rs"]
mod r1_probes_issue303;
#[path = "r2_bool_door.rs"]
mod r2_bool_door;
#[path = "r2_bytes.rs"]
mod r2_bytes;
#[path = "r2_cert9_probes.rs"]
mod r2_cert9_probes;
#[path = "r2_mesh1_probes.rs"]
mod r2_mesh1_probes;
#[path = "r2_mesh2_probes.rs"]
mod r2_mesh2_probes;
#[path = "r2_mesh6_probes.rs"]
mod r2_mesh6_probes;
#[path = "r2_split_door.rs"]
mod r2_split_door;
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
/// Roles, not line numbers: lines move, so every citation in this
/// crate's ε inventory names a target instead. A **carrier** passes
/// the band on (a parameter, a field, a hand-off); a **terminal read**
/// is one of `Eps`'s four operations, and the operation column is
/// the inventory.
///
/// - **`tessellate.rs` — 2 carriers, no reads.** ε ENTERS the crate
///   here and nowhere else: `Eps::at(tol)` bound to a local, and the
///   `SizingTols` field initialiser. 1 + 1 = 2.
/// - **`sizing.rs` — 2 carriers, no reads.** The `SizingTols::eps`
///   field declaration and the `tol.eps()` inside `Eps::at` — the
///   crate's ONE raw read of `Tol::eps()`, which is the seam and is
///   pinned here so a second one moves this number. The four
///   operations are DEFINED in this file and called in none of it, so
///   its read column is empty; a definition is not a read. 1 + 1 = 2.
/// - **`curved.rs` — 6 carriers, no reads.** Two hand-offs out of
///   `SizingTols` (`walk::loop_polygon`, `require_swept_rectangle`),
///   two `eps` parameters, one hand-off to `entries_off_bbox`, and
///   `entries_off_bbox`'s own single `gap_is_noise` call — the banded
///   swept-rectangle domain guard, which REFUSES a face and moves no
///   coordinate. 2 + 2 + 1 + 1 = 6. The guard's terminal read is
///   `gap_is_noise`'s, counted in `walk.rs`: this file hands the band
///   to a predicate rather than comparing against it, which is why it
///   carries six carriers and no operation.
/// - **`trimmed.rs` — 1 carrier, 1 `pad`.** `d / tol.eps.pad(bound)`
///   in the deviation probe. **Not a bar**: ε is a continuous addend
///   in a denominator, so it scales `worst_ratio` — a `pub`
///   measurement field — at every call, monotonically. The block is
///   absent from a default build (`budget` feature). The one token and
///   the one call are the same expression.
/// - **`walk.rs` — 14 carriers, 5 reads (1 `separates`, 3
///   `coincident`, 1 `dominates`).** **Four** `eps` parameters
///   (`gap_is_noise`, `closing_column`, `iso_side_starts`,
///   `loop_polygon`), **five** hand-offs (`closing_column`'s and
///   `loop_polygon`'s two `gap_is_noise` calls, `loop_polygon`'s calls
///   to `iso_side_starts` and `closing_column`), and **five** terminal
///   reads: `gap_is_noise`'s `dominates(gap * lever)` (one predicate,
///   four call sites — the domain guard above plus three
///   `debug_assert` detectors that gate nothing), `iso_side_starts`'
///   `separates(radial)`, `pole_index`'s `coincident(norm)` (the
///   pole-membership find — the ONE home both `pole_v` and the
///   issue-896 guard consume), `loop_polygon`'s
///   `coincident_declared` closure, whose `coincident(d)` asks
///   whether two DECLARED vertices of one loop are the same point,
///   and the issue-896 guard's own `coincident(gap)`, asking whether
///   a junction × pole pair the classification passes over coincides.
///   4 + 5 + 5 = 14 carriers; 1 + 3 + 1 + 0 = 5 reads.
///
/// Nine carrier sites and **six terminal reads across the crate** —
/// `walk.rs`'s five plus `trimmed.rs`'s `pad`, which is the whole of
/// the read column and sums to the same six the operation totals do
/// (1 `separates` + 3 `coincident` + 1 `dominates` + 1 `pad`).
///
/// # Why the operation column is the pin that matters
///
/// The carrier column is a token walk and always was. The operation
/// column is not: `Eps` holds its band in a private field with no
/// accessor, so **a `mesh` read of ε is one of these four calls or it
/// does not compile**. What that leaves for a textual row is the one
/// thing the type cannot forbid — a raw f64 re-extracted through a new
/// accessor on `Eps` itself, or a second `Tol::eps()`. The second is
/// the easy half: it spells `eps`, so it moves the carrier count of
/// whichever file writes it, and `sizing.rs` is pinned at 2 because
/// the seam lives there. The first is hole 1 below, which states
/// which column catches it, having planted it.
/// A read that ports to no operation
/// would have to be recorded at its site with the reason; there is no
/// such read in the crate today, and the read column being exactly
/// the crate's terminal-read count is the statement of that.
///
/// **The per-file totals above are pinned; every other number in this
/// doc is hand-written and is not.** They are checkable — each file's
/// breakdown sums to its pinned totals, which is the arithmetic a
/// reader can run and the reason it is written that way. An earlier
/// draft of `walk.rs`'s line said *"three parameters, four hand-offs
/// and three terminal reads"* and summed to 10 against a pinned 12: a
/// hand-written narrative that did not add up, in the doc of the pin
/// that replaced a hand-written list for going stale.
///
/// **That has happened twice.** #887 moved `curved.rs` from 7 to
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
/// 1. **A read that does not spell `eps` — retired as a hole, and it
///    was closed by a MECHANISM rather than by this row.** ε bound to
///    another name is now an `Eps`, and an `Eps` has no accessor, so
///    a read of it is one of the four operations the second column
///    counts. The way back out is a raw accessor written on `Eps`
///    itself, and both of its halves are pinned: the accessor UNUSED
///    is dead code and reds the lint gate, and USED it must name the
///    band at its call site, which moves that file's carrier count —
///    while a raw comparison put where an operation stood moves the
///    read count down. Measured, not asserted: planting
///    `if d <= eps.raw()` over `loop_polygon`'s `coincident` call reds
///    this row on `walk.rs`'s read column (`[1, 3, 1, 0]` →
///    `[1, 2, 1, 0]`). What that plant also showed is that the CARRIER
///    column does not see the accessor's own body — `fn raw(self) ->
///    f64` in `sizing.rs` spells no `eps` — which is why the READ
///    column, not the carrier column, is this row's load-bearing half.
///    **The residue is UFCS**: `Eps::coincident(band, x)`, with the
///    band held under a name that is not `eps`, is a real read that
///    neither column sees — the read column matches `.coincident(`
///    with a leading dot, and the carrier column matches the
///    identifier `eps`. Nothing in the crate is written that way and
///    nothing checks that it stays so; it is recorded here because a
///    bypass hunt should start from the known gap rather than from
///    scratch. Widening the read column to the bare method name would
///    fire on this row's own prose, which is the trade not taken.
/// 2. **Which KIND of read it is — no longer the reader's alone.** The
///    operation column says whether a read separates, identifies,
///    dominates or pads, because the caller had to pick one to
///    compile. What it still cannot say is whether the KIND CHOSEN is
///    the right one: `coincident` where `dominates` was meant differs
///    only at the band edge, and no count can see that. That judgement
///    is the reader's, at the site, and `Eps`'s own rows pin the
///    edges it turns on.
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
    // The four operations, in the order the read column reports them.
    const OPS: [&str; 4] = ["separates", "coincident", "dominates", "pad"];
    // (file, ε CARRIER tokens, one READ count per op above).
    const PINNED: [(&str, usize, [usize; 4]); 5] = [
        ("curved.rs", 6, [0, 0, 0, 0]),
        ("sizing.rs", 2, [0, 0, 0, 0]),
        ("tessellate.rs", 2, [0, 0, 0, 0]),
        ("trimmed.rs", 1, [0, 0, 0, 1]),
        ("walk.rs", 14, [1, 3, 1, 0]),
    ];
    let src = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    let needle = concat!("e", "ps");
    let mut found: Vec<(String, usize, [usize; 4])> = Vec::new();
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
        // `grid_steps` are not ε carriers and outnumber the real ones.
        let carriers = prod
            .match_indices(needle)
            .filter(|(i, _)| {
                let before = prod[..*i].chars().next_back();
                let after = prod[i + needle.len()..].chars().next();
                let ident = |c: Option<char>| c.is_some_and(|c| c.is_alphanumeric() || c == '_');
                !ident(before) && !ident(after)
            })
            .count();
        // CALLS, not definitions: the leading `.` is what separates
        // `eps.coincident(d)` from `fn coincident(` in `sizing.rs`,
        // and it is why the file that DEFINES the four operations
        // reports none of them. Prose cannot reach this either — the
        // walk is over `code_only`, and `dominates` is a word this
        // crate's comments use.
        let reads = OPS.map(|op| prod.matches(&format!(".{op}(")).count());
        if carriers > 0
            || reads.iter().any(|n| *n > 0)
            || PINNED.iter().any(|(pinned, _, _)| *pinned == name)
        {
            found.push((name, carriers, reads));
        }
    }
    found.sort();
    let pinned: Vec<(String, usize, [usize; 4])> = PINNED
        .iter()
        .map(|(path, carriers, reads)| ((*path).to_string(), *carriers, *reads))
        .collect();
    assert_eq!(
        found, pinned,
        "the ε inventory moved. That is not a failure to silence. A moved CARRIER \
         count means the band reaches a new place, or a raw f64 was extracted from \
         `Eps` — `sizing::SizingTols`'s ledger and this row's per-file breakdown both \
         owe an update. A moved READ count means a terminal read was added, removed \
         or re-kinded: the operation the caller picked IS its classification, so \
         check the band edge is the one that read wants (`Eps`'s own rows pin the \
         edges), then re-pin."
    );
    // The crate's terminal reads and its named operations are the same
    // six things, which is the whole claim: `Eps`'s methods ARE the
    // inventory. Stated as a total so the per-file rows above cannot
    // drift from the sentence in this row's docs.
    let total: usize = found.iter().flat_map(|(_, _, r)| r.iter()).sum();
    assert_eq!(
        total, 6,
        "the crate has six terminal ε reads; the operation columns must sum to them"
    );
}
