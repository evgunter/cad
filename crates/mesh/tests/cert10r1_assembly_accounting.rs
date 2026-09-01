//! CERT-10 review probe (R1), PROMOTED to a permanent check.
//!
//! # What it caught
//!
//! The PR body's §2 argued the fold's 1.01-1.20x cost was "more than
//! given back" because `trimmed.rs` asked for the per-cell grid and the
//! whole-patch bound of the same face back to back, so a shipped
//! integral face "assembles its derivative nets ONCE where it used to
//! assemble them twice". **The reviewer instrumented it and the claim
//! was false**, and the mechanism is structural rather than incidental:
//! `tessellate()` runs `compute_chords` BEFORE the per-face dispatch,
//! and the chord pass's `nurbs_tighten` calls `face_bound` on every
//! described NURBS face adjacent to an edge. With a memo of the
//! WHOLE-PATCH bound, the trimmed lane's later ask was therefore always
//! a hit — on a value it could not refine into cells — so
//! `NurbsCellGrid::patch` was never reached and the lane paid for two
//! assemblies per face:
//!
//! ```text
//! swept_elbow, memo of NurbsFaceBound (the reviewed head f5ab8bab):
//!   nurbs faces = 4   assemblies = 8  (2.00/face)   patch reads = 0
//! ```
//!
//! # What the fix pass did about it
//!
//! The memo now holds the CELL TABLE (`nurbs_cert::face_grid`), and the
//! whole-patch bound is a reading of it. The chord pass fills the memo
//! with the finer fact; the trimmed lane clones it. Re-measured:
//!
//! ```text
//! swept_elbow, memo of NurbsCellGrid (this head):
//!   nurbs faces = 4   assemblies = 4  (1.00/face)
//! ```
//!
//! # Why this file can assert rather than narrate
//!
//! The reviewer's measurement needed instrumentation in three files
//! under review, so it could only be recorded as prose. The count is
//! now a first-class measurement: `mesh::budget` — the meter that
//! already exists for exactly this kind of question — counts assemblies
//! behind its own feature, so the check runs in the armed lane and
//! costs a shipped build nothing.
#![cfg(feature = "budget")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use geom_core::Tol;
use mesh::budget::{self, Mode};

/// Every described NURBS face costs the tessellation **exactly one**
/// certified cell-table assembly — the most expensive thing the lane
/// does per face. A second one means a consumer asked through a door
/// that bypasses the memo, which is the regression this row exists for.
#[test]
fn cert10r1_one_assembly_per_described_nurbs_face() {
    let body = sweep::test_support::swept_elbow(Tol::witness());
    let faces = body
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Nurbs(_) | geom::Surface::Approx(_))
            )
        })
        .count();
    assert_eq!(faces, 4, "the accounting above was taken on 4 faces");
    budget::arm(Mode::Sizing);
    mesh::tessellate(&body, 1e-2, Tol::witness()).expect("tessellates");
    let assemblies = budget::assemblies();
    let _ = budget::take();
    assert_eq!(
        assemblies, faces,
        "{assemblies} cell-table assemblies for {faces} described NURBS faces — the \
         reviewed head measured 2.00/face because the memo held the whole-patch bound, \
         a fact too coarse for the trimmed lane to refine; see this file's docs"
    );
}
