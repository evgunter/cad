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
//! own `every_suite_file_is_aggregated`. Refs #651.

// Per the workspace convention recorded in the root Cargo.toml: test code
// may allow the panic family, because panicking IS a test's failure
// mechanism.
#![allow(clippy::expect_used, clippy::panic)]

/// The body of a `[header]` section: every line after it, up to the next
/// section header. `None` if the header is absent.
fn section<'a>(manifest: &'a str, header: &str) -> Option<Vec<&'a str>> {
    let mut lines = manifest.lines().map(str::trim);
    lines.find(|l| *l == header)?;
    Some(lines.take_while(|l| !l.starts_with('[')).collect())
}

#[test]
fn dev_profile_still_optimizes_spade_and_mesh() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_dir.join("..").join("..").join("Cargo.toml");
    let text = std::fs::read_to_string(&root)
        .expect("the workspace root Cargo.toml is two levels above this crate");

    // Anti-vacuity: this scanner keys on a literal header line, so a
    // reformat that inlines the tables would make every check below pass
    // by finding nothing. Prove the shape it assumes still exists.
    assert!(
        section(&text, "[workspace]").is_some(),
        "no `[workspace]` section found in {} — the manifest's format \
         changed and this guard was about to pass vacuously",
        root.display()
    );

    for pkg in ["spade", "mesh"] {
        let header = format!("[profile.dev.package.{pkg}]");
        let body = section(&text, &header).unwrap_or_else(|| {
            panic!(
                "{header} is gone from the workspace manifest. It is a \
                 MEASURED decision (2026-07-21: one washer test 91.7 s at \
                 opt 0 vs ~1.2 s optimized), and deleting it makes the \
                 whole test suite slow again with nothing pointing at the \
                 cause. Restore it, or re-measure and delete this guard \
                 with the block."
            )
        });
        assert!(
            body.iter().any(|l| {
                let mut kv = l.splitn(2, '=');
                kv.next().map(str::trim) == Some("opt-level")
                    && kv.next().map(str::trim) == Some("2")
            }),
            "{header} exists but no longer sets `opt-level = 2` (body: \
             {body:?}). See the measurement note above the block."
        );
    }
}
