//! CERT-10 review probe: does the fold actually REMOVE an assembly per
//! shipped face? The PR body argues the fold's 1.01-1.20x cost "is more
//! than given back" because `trimmed.rs` used to ask for the per-cell
//! grid and the whole-patch bound of the same face back to back. But
//! the whole-patch bound was already MEMOIZED (`FaceBounds`, threaded
//! from `tessellate()` through both passes), and the chord pass runs
//! first. This row counts the assemblies.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use core::sync::atomic::Ordering;

use geom_core::Tol;
use mesh::nurbs_cert::{PROBE_ASSEMBLIES, PROBE_FACE_BOUND_MISS, PROBE_GRIDS, PROBE_MEMO_HITS, PROBE_PATCH_READS};
use geom_brep::patch_bound::PROBE_PATCH_CELLS;
use sweep::test_support::swept_elbow;

#[test]
fn cert10r1_how_many_assemblies_per_shipped_nurbs_face() {
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
    PROBE_ASSEMBLIES.store(0, Ordering::Relaxed);
    PROBE_PATCH_READS.store(0, Ordering::Relaxed);
    PROBE_MEMO_HITS.store(0, Ordering::Relaxed);
    PROBE_GRIDS.store(0, Ordering::Relaxed);
    PROBE_FACE_BOUND_MISS.store(0, Ordering::Relaxed);
    PROBE_PATCH_CELLS.store(0, Ordering::Relaxed);
    let _ = mesh::tessellate(&body, 1e-2, Tol::witness()).expect("tessellates");
    let a = PROBE_ASSEMBLIES.load(Ordering::Relaxed);
    let pr = PROBE_PATCH_READS.load(Ordering::Relaxed);
    let mh = PROBE_MEMO_HITS.load(Ordering::Relaxed);
    let g = PROBE_GRIDS.load(Ordering::Relaxed);
    let fbm = PROBE_FACE_BOUND_MISS.load(Ordering::Relaxed);
    let pc = PROBE_PATCH_CELLS.load(Ordering::Relaxed);
    println!(
        "[cert10r1] nurbs faces = {nurbs_faces}; cell-net ASSEMBLIES = {a} \
         ({:.2} per face); NurbsCellGrid::patch reads = {pr}; face_bound memo HITS = {mh}; nurbs_cell_grid calls = {g}; face_bound MISSES = {fbm}; patch_cells (THE assembly) = {pc} ({:.2} per face)",
        a as f64 / nurbs_faces as f64,
        pc as f64 / nurbs_faces as f64
    );
    // The claim under test: "a shipped integral face therefore
    // assembles its derivative nets ONCE where it used to assemble
    // them twice." If assemblies-per-face is still 2, the fold's cost
    // is not given back on the shipped path.
    assert!(nurbs_faces > 0, "the fixture has no NURBS faces");
}
