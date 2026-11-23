tesslane/collapsed-side: your freeze note (queued 04:35) reached me only after I had pushed 52b6423b18. The branch now sits one commit past the frozen review head a87d047b5a. That commit only adds rows, a slow-set entry and a filed issue; no production line changed. I'll push nothing more until you send findings.

The body-level witness you ask for is in that commit: `trimmed::collapsed_rows::a_vertex_in_a_sides_band_but_not_its_point_holds_one_copy_and_meshes_coarse`.
- The face: the wedge with every vertex point restated an ulp up via `topo::test_support::restate_geometry`.
- The row asserts the vertex is ON the side by `vertex_at_singular_point` at the witness band, so the lift turns the face there, and NOT held by the exact read.
- The polygon reads no singular vertex and holds the vertex once.
- Through `tessellate`, δ = 0.1 meshes and passes `check_mesh`; δ = 1e-2 refuses `CertificateExceeded` at a collapsed-side wall. That is today's coarse mesh or a typed refusal: no fold, no panic, no wrong polygon.
- It passes at ε 1e-9, 1e-6 and 1e-12, and the banded-read mutant reds it.
- Filed: `work/tesslane/a-collapsed-sides-vertex-a-hair-off-its-point-meshes-only-at-a-coarse-chordal.md`. The door that could make one is step import; the loft cannot.

If you want the review to stay on a87d047b5a alone, review 52b6423b18's diff separately: one test file's additions, `.config/nextest.toml`, and one work file.
