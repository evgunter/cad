//! The workspace's `[profile.dev.package]` opt-2 overrides, pinned.
//!
//! The root `Cargo.toml` optimizes exactly `spade` and `mesh` in the dev
//! profile, on a 2026-07-21 measurement (one washer test: 91.7 s at opt 0
//! vs ~1.2 s optimized). Nothing else in the tree notices if that block is
//! deleted, and the symptom — a suite that is slow again — is not one
//! anybody attributes to a missing manifest stanza. So this row goes red
//! instead.
//!
//! **What this pins is the DECISION, not the timing.** A timing cannot be
//! asserted here: it is box-dependent and belongs to the perf lane
//! (`memories/perf-measurement-lane.md`, `docs/PERF-PLAN.md`), which
//! reports and never gates. What it can pin is that the two overrides the
//! measurement bought are still present and still `opt-level = 2` — the
//! part that vanishes silently. Widening the block is fine and does not
//! fail this row; the scope note beside it in `Cargo.toml` records why
//! extending it to the geom crates was measured and rejected (#52).
//!
//! Manifest-text guard, same shape as `pncad-py`'s
//! `crate_lints_match_the_workspace_minus_unsafe_code` and this crate's
//! own `every_suite_file_is_aggregated`. Its falsification lives in
//! [`scanner_fires_on_planted_violations`] rather than in a PR body, per
//! S13's steelman standard for text-matching gates. Refs #651.

// Per the workspace convention recorded in the root Cargo.toml: test code
// may allow the panic family, because panicking IS a test's failure
// mechanism.
#![allow(clippy::expect_used, clippy::panic)]

/// The body of a `[header]` section: the lines after it up to the next
/// section header, with blank lines and comments dropped. `None` if the
/// header is absent.
///
/// **A text scanner, not a TOML parser — its blind spots are stated here
/// rather than left to be discovered** (§C15; the posture `pncad-py`'s
/// `toml_table` takes). It tolerates the two ordinary TOML forms a human
/// edit produces: a trailing `# comment` on a value, and quoting around
/// bare keys (`[profile.dev.package."spade"]`). It does **not** understand
/// inline tables, dotted keys, multi-line values, `{}`-wrapped bodies, or
/// a `#` inside a quoted string.
///
/// Every one of those is a **loud** blind spot, not a silent one: each
/// makes the literal header line disappear, so the caller's missing-section
/// arm panics. There is no input on which this scanner reports a section it
/// did not find, and none on which the caller passes by matching nothing —
/// an absent header panics and an empty body fails the `any()`.
fn section<'a>(manifest: &'a str, header: &str) -> Option<Vec<&'a str>> {
    let strip = |l: &'a str| l.split('#').next().unwrap_or(l).trim();
    let mut lines = manifest.lines().map(strip);
    lines.find(|l| l.replace('"', "") == header)?;
    Some(
        lines
            .take_while(|l| !l.starts_with('['))
            .filter(|l| !l.is_empty())
            .collect(),
    )
}

/// The check, over manifest text. Split out from the test only so
/// [`scanner_fires_on_planted_violations`] can drive it with fixtures;
/// `label` names the source in the messages.
fn check_dev_profile(text: &str, label: &str) {
    // NOT anti-vacuity — this scanner has no vacuous-pass path (see
    // `section`'s docs). What this arm checks is the PATH ARITHMETIC:
    // `../../Cargo.toml` is the workspace root only while the layout
    // holds, and reading some *other* real manifest would otherwise fail
    // below with a confusing "the block is gone" instead of "you read the
    // wrong file". (`pncad-py`'s sibling assertion, which this file
    // otherwise copies, does guard a genuine vacuity hazard: its scanner
    // returns an empty map for a missing header and its comparison is
    // then empty-vs-empty. Ours panics.)
    assert!(
        section(text, "[workspace]").is_some(),
        "no `[workspace]` section in {label} — this is not the workspace \
         root manifest, so every check below is about to test the wrong file"
    );

    for pkg in ["spade", "mesh"] {
        let header = format!("[profile.dev.package.{pkg}]");
        let body = section(text, &header).unwrap_or_else(|| {
            panic!(
                "{header} is gone from {label}. It is a MEASURED decision \
                 (2026-07-21: one washer test 91.7 s at opt 0 vs ~1.2 s \
                 optimized), and deleting it makes the whole test suite \
                 slow again with nothing pointing at the cause. Restore \
                 it, or re-measure and delete this guard with the block."
            )
        });
        assert!(
            body.iter().any(|l| {
                let mut kv = l.splitn(2, '=');
                let unquote = |s: &str| s.trim().trim_matches('"').to_owned();
                kv.next().map(unquote).as_deref() == Some("opt-level")
                    && kv.next().map(unquote).as_deref() == Some("2")
            }),
            "{header} exists but no longer sets `opt-level = 2` (body: \
             {body:?}). See the measurement note above the block."
        );
    }
}

#[test]
fn dev_profile_still_optimizes_spade_and_mesh() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_dir.join("..").join("..").join("Cargo.toml");
    let text = std::fs::read_to_string(&root)
        .expect("the workspace root Cargo.toml is two levels above this crate");
    check_dev_profile(&text, &root.display().to_string());
}

/// A clean fixture, carrying both tolerated forms: an inline comment on a
/// value (the manifest this guards is comment-dense, and the stanza's whole
/// point is a dated measurement someone may well annotate) and a quoted
/// package key.
const CLEAN: &str = "\
[workspace]
members = [\"crates/mesh\"]

# Test-speed hot spot (measured 2026-07-21).
[profile.dev.package.spade]
opt-level = 2  # 2026-07-21; see the note above
[profile.dev.package.\"mesh\"]
opt-level = 2
# Scope note: extending this was measured and rejected (#52).
";

/// The self-test: a clean fixture must pass and every planted violation
/// must fire. S13's steelman ratified this shape for the repo's
/// text-matching gates (*"a clean fixture must pass, a planted violation
/// must fire"*); the shell gates carry it as `--selftest`, and for a Rust
/// `#[test]` guard the equivalent is a sibling row.
///
/// It is worth having here for a reason specific to this scanner: the
/// tolerances in `section` were added to stop a **false red** (an inline
/// comment on `opt-level`), and a tolerance widened by hand is exactly the
/// edit that can quietly make a gate stop catching things.
#[test]
fn scanner_fires_on_planted_violations() {
    let red = |fixture: &str| {
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let outcome = std::panic::catch_unwind(|| check_dev_profile(fixture, "<fixture>"));
        std::panic::set_hook(hook);
        outcome.is_err()
    };

    assert!(!red(CLEAN), "the clean fixture must pass");

    for (what, planted) in [
        (
            "block deleted",
            CLEAN.replace("[profile.dev.package.\"mesh\"]\nopt-level = 2\n", ""),
        ),
        (
            "opt-level 3",
            CLEAN.replace("opt-level = 2\n", "opt-level = 3\n"),
        ),
        (
            "opt-level 0",
            CLEAN.replace("opt-level = 2\n", "opt-level = 0\n"),
        ),
        (
            "opt-level \"s\"",
            CLEAN.replace("opt-level = 2\n", "opt-level = \"s\"\n"),
        ),
        (
            "header commented out",
            CLEAN.replace(
                "[profile.dev.package.spade]",
                "#[profile.dev.package.spade]",
            ),
        ),
        (
            "[workspace] renamed",
            CLEAN.replace("[workspace]", "[workspace-x]"),
        ),
        (
            // The documented blind spot, planted to prove it is LOUD:
            // an inlined-table reformat is not understood, and fails.
            "tables inlined",
            CLEAN.replace(
                "[profile.dev.package.spade]\nopt-level = 2  # 2026-07-21; see the note above\n",
                "[profile.dev.package]\nspade = { opt-level = 2 }\n",
            ),
        ),
    ] {
        assert_ne!(
            planted, CLEAN,
            "planted violation `{what}` did not change the fixture — the \
             fixture text drifted from the replacement pattern"
        );
        assert!(red(&planted), "planted violation `{what}` did not fire");
    }
}
