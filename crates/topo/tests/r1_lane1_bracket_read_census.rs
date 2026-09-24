//! **The bracket reads in `validate.rs`, counted** — the mechanical
//! guard the `Bounds` scope rule's 2026-09-02 entry did not have.
//!
//! A reviewer probe. `geom-core/src/real.rs`'s `bounds_allowlist`
//! entry for the certified at-rest validator discloses the file's
//! bracket reads by NUMBER ("ONE `lo` call appears in `validate.rs`",
//! and why that one is admissible). `scripts/gates/bounds-allowlist.sh`
//! counts compound `Bounds` BOUNDS, not bracket READS, so a second
//! `lo`/`hi` call in that file falsifies the disclosure with nothing
//! red — checked by planting one and watching the gate pass. Before
//! this unit the disclosed number was zero and the drift was the same
//! shape; it is a count now, which drifts more easily.
//!
//! This row is that guard: the reads are counted over the CODE view
//! (comments and string literals blanked, so the prose that discusses
//! `lo` does not answer), and a change to the number reds here with
//! the entry named. The recourse for a red is to re-argue the new read
//! against the scope rule and move the disclosure in the change that
//! carries the argument — never to edit the number here alone.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use test_utils::source::{code_only, repo_root};

/// The file the disclosure is about, and the reads it discloses.
const SUBJECT: &str = "crates/topo/src/validate.rs";

/// The disclosure's number: check 1's torus tube-radius
/// representability read, `geom_core::Bounds::lo` of a stored datum
/// compared with zero, and nothing else.
const DISCLOSED_READS: usize = 1;

/// The spellings a bracket read can take. `Bounds::lo(x)` is the
/// qualified form this file uses; `.lo()`/`.hi()` are the method forms
/// the allowlist prose greps for and the ones a future read is most
/// likely to arrive as.
const NEEDLES: [&str; 4] = ["Bounds::lo(", "Bounds::hi(", ".lo()", ".hi()"];

#[test]
fn validate_rs_holds_exactly_the_bracket_reads_the_allowlist_discloses() {
    let path = repo_root(env!("CARGO_MANIFEST_DIR")).join(SUBJECT);
    let text = std::fs::read_to_string(&path).expect("validate.rs reads");
    let code = code_only(&text);
    let mut found: Vec<(usize, String)> = Vec::new();
    for (n, line) in code.lines().enumerate() {
        for needle in NEEDLES {
            if line.contains(needle) {
                found.push((n + 1, needle.to_string()));
            }
        }
    }
    assert_eq!(
        found.len(),
        DISCLOSED_READS,
        "{SUBJECT} carries {} bracket read(s), the `Bounds` scope rule's entry in \
         `crates/geom-core/src/real.rs` discloses {DISCLOSED_READS}: {found:?}. \
         `scripts/gates/bounds-allowlist.sh` does not see this — it counts compound \
         bounds, not reads — so this row is the only thing that does. Re-argue the \
         read against the scope rule and move the disclosure with the argument",
        found.len()
    );
}
