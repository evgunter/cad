//! R1 probe helper: dump a tube-bearing save for the stale-build
//! refusal probe (run against the merge-base build).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;

use corpus::tube_ring;
use editor_core::{ProfileDoc, save};
use geom_core::Tol;

#[test]
fn dump_tube_bearing_save() {
    let d = tube_ring::document();
    let empty = ProfileDoc::empty_derived("r1-stale-probe", Tol::witness());
    let text = save(&empty, &d.edits, Tol::witness()).expect("saves");
    assert!(text.contains("\"Tube\""), "the save must name the new kind");
    // Writes only when asked: the row is a helper for the stale-build
    // probe, not an assertion in its own right.
    if let Ok(out) = std::env::var("R1_DUMP") {
        std::fs::write(&out, &text).expect("writes");
    }
}
