//! **The `die_tool` document's saved bytes, pinned — the fixture the
//! Python re-authoring is measured against.**
//!
//! # The banked question
//!
//! `work/lib/log.md` carried "die_tool's Python re-authoring (banked
//! behind its Revolve/datum half)" from LIB-PYPU onward: PYPU bound
//! the placement vocabulary (`Node.placed_union_at`, `Frame`,
//! `PatternKind`) and authored the LINEAR twin — `heat_sink_fins`,
//! which is extrude-only — while `die_tool`'s prototype is a REVOLVE
//! about a `Datum::Axis`, and the Python die scene reached its ball
//! only through the retired equator workaround
//! (`crates/editor-core/tests/corpus/die_pips.rs`'s deviation (b),
//! deleted at `7581fb65d`: the revolve name emitter refused an
//! all-on-axis two-pole meridian, so the ball was charted as two
//! quarter arcs meeting at an off-axis equator vertex).
//!
//! The Rust corpus document has carried the NATURAL meridian since the
//! day it was authored — `die_tool` (`54f44ac90`) postdates the
//! workaround's deletion by one commit, and `die_pips::half_disc_
//! program`, which it reuses, is the bulge-1 semicircle pole to pole.
//! What was left banked was the crossing: nothing executed the whole
//! `die_tool` vocabulary — Profile, Datum(Axis), Revolve,
//! PlacedUnion(Explicit), Boolean(Subtract) — from Python.
//!
//! # What this file pins, and why as BYTES
//!
//! `crates/pncad-py/tests/test_placed_union.py::TestTheDieTool`
//! re-authors the document through the bound Python doors and asserts
//! its `Doc.save()` text against the file this test writes. A
//! transcription proved by eye drifts silently; a byte file does not —
//! `corpus/die_composed_tour.rs`'s reasoning, applied in the other
//! direction (there the tour authors and the corpus replays; here the
//! corpus authors and Python must reproduce).
//!
//! Two lines are excluded from the Python-side comparison and neither
//! is model content: the document IDENTITY (answered by the label —
//! the Python side authors under `Doc("mod")`, the same label
//! `fixture::Recorder` derives from, so the ids in fact agree and the
//! exclusion is belt-and-braces) and the snapshot's ONE `"epsilon"`
//! line, which CI's ε rows sweep by design (`crates/pncad/tests/
//! all.rs::plate_param_authors_facade_only_and_its_saved_text_is_
//! pinned` states that disposition; this file inherits it).
//!
//! The edit log is saved EMPTY on purpose. `CorpusDoc` carries both a
//! replayed snapshot and the log that built it, and Python's `Doc` is
//! built by `insert` and carries no log at all; saving the snapshot
//! with `&[]` is the shape both sides can say, so the pin compares
//! recipes rather than authoring histories.
//!
//! Regenerate with:
//!
//! ```text
//! PNCAD_BLESS=1 cargo test -p editor-core --test all \
//!     lib_dietool_crossing
//! ```
//!
//! run under a DEFAULT environment (the committed file carries the
//! default ε).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use geom_core::Tol;

/// The committed fixture, relative to the crate manifest.
const FILE: &str = "tests/corpus/die_tool.pncad";

/// The `die_tool` corpus document's saved text, pinned as the file the
/// Python re-authoring is compared against.
///
/// The pin is exact except the snapshot's ONE `"epsilon"` line — the
/// `plate_param` disposition, quoted in the module docs. Each side must
/// carry exactly one such line; a duplicated or missing one is fixture
/// damage rather than sweep variance and fails here.
#[test]
fn the_die_tool_documents_saved_text_is_pinned() {
    let d = corpus::documents()
        .into_iter()
        .find(|d| d.name == "die_tool")
        .expect("die_tool is registered");
    let text = editor_core::persist::save(&d.doc, &[], Tol::witness()).expect("the document saves");

    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(FILE);
    if std::env::var_os("PNCAD_BLESS").is_some() {
        std::fs::write(&path, &text).expect("the fixture writes");
        return;
    }
    let recourse = "regenerate it: PNCAD_BLESS=1 cargo test -p editor-core \
                    --test all lib_dietool_crossing (default env)";
    let committed = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the die_tool fixture is missing: {e} — {recourse}"));

    let sans_epsilon = |t: &str| -> String {
        let (kept, excluded): (Vec<&str>, Vec<&str>) = t
            .lines()
            .partition(|l| !l.trim_start().starts_with("\"epsilon\":"));
        assert_eq!(
            excluded.len(),
            1,
            "expected exactly one \"epsilon\" line, found {}",
            excluded.len()
        );
        kept.join("\n")
    };
    assert_eq!(
        sans_epsilon(&text),
        sans_epsilon(&committed),
        "the saved die_tool text moved — {recourse}"
    );
}
