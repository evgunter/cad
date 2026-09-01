//! CERT-10 review probe — RECORD of an executed measurement.
//!
//! The PR body's §2 argues the fold's 1.01-1.20x cost "is more than
//! given back" because `trimmed.rs` used to ask for the per-cell grid
//! and the whole-patch bound of the same face back to back, so "a
//! shipped integral face therefore assembles its derivative nets ONCE
//! where it used to assemble them twice."
//!
//! MEASURED at the frozen head f5ab8bab, by temporarily instrumenting
//! `patch_bound::patch_cells` (the assembly), `nurbs_cell_grid`,
//! `NurbsCellGrid::patch` and `face_bound`'s memo, then running
//! `mesh::tessellate(&swept_elbow(Tol::witness()), 1e-2, ...)`:
//!
//! ```text
//! nurbs faces = 4
//! patch_cells (THE assembly) = 8   -> 2.00 per face
//! nurbs_cell_grid calls       = 4
//! face_bound MISSES           = 4
//! face_bound memo HITS        = 12
//! NurbsCellGrid::patch reads  = 0
//! ```
//!
//! `NurbsCellGrid::patch` — the "reading of the cells the grid already
//! assembled" the argument rests on — is NEVER CALLED. `tessellate()`
//! runs `compute_chords` before the per-face dispatch
//! (`tessellate.rs`), and the chord pass's `nurbs_tighten` calls
//! `face_bound` on every described NURBS face adjacent to an edge,
//! populating the shared `FaceBounds` memo. By the time
//! `tessellate_trimmed` evaluates `bounds.entry(fk).or_insert_with(||
//! grid.patch())`, the entry is always present. The count is still 2
//! assemblies per shipped face; the fold's cost is net.
//!
//! The instrumentation is not committed (it edits three files under
//! review). This file records the numbers so the finding is
//! reproducible: re-add a `fetch_add` at the head of
//! `patch_bound::patch_cells` and re-run the body above.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use geom_core::Tol;
use sweep::test_support::swept_elbow;

/// The uninstrumented half that still runs: the fixture really does
/// carry four described NURBS faces, and the chord pass really does
/// precede the per-face dispatch (so the memo is warm).
#[test]
fn cert10r1_the_fixture_behind_the_assembly_accounting() {
    let body = swept_elbow(Tol::witness());
    let nurbs_faces = body
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Nurbs(_) | geom::Surface::Approx(_))
            )
        })
        .count();
    assert_eq!(nurbs_faces, 4, "the accounting above was taken on 4 faces");
    let m = mesh::tessellate(&body, 1e-2, Tol::witness()).expect("tessellates");
    assert!(!m.positions.is_empty());
}
