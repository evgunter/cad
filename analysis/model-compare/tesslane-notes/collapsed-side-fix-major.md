tesslane/collapsed-side — the MAJOR (banded read) is fixed. HEAD d6274e05aa9c7f1bdc8b19f873da36ccd0efc5de (confirmed with ls-remote). Commits: 675f00031c merges main; 3d33c0b7ff is the fix; d6274e05aa is a work-file wording change. No trailers.

1. ONE BANDED READ. `CollapsedSides::holds(p, band)` is now `chart_table::at_a_singular_point(sides' points, p, band)`. That function is the one read `AnalyticRow::is_singular` and `at_a_pole` already make; it came in with the main I merged, and I made it pub(crate) and added the spline sides to its doc. The sides themselves still come from the home (`singular_points`).
   - `walk` decides each side with the same read. A vertex that is undecided against any side, or decided on two sides, is not walked.
   - No bit-identity read is left in `mesh/src`.
   - `bash scripts/run-gates.sh` exits 0, with `bit-identity-consumer OK: no bit-identity consumer outside the 8 non-consumer rows`. The gate's list is unchanged.
2. THE DISAGREEMENT IS GONE, AND THE ROW FLIPPED. It is now `a_vertex_in_a_sides_band_is_on_it_and_meshes_as_the_vertex_on_its_point`: the wedge with every vertex point restated an ulp up.
   - The vertex is off the side's point by a nonzero gap and decided on it at the band.
   - The polygon reads it singular and walks its side.
   - At δ 0.1, 1e-2 and 1e-3, its per-face triangle counts equal the plain wedge's. Under the exact read it refused at 1e-2.
   - Deleted `work/tesslane/a-collapsed-sides-vertex-a-hair-off-its-point-meshes-only-at-a-coarse-chordal.md`.
   - The ulp unit row is re-meant as `a_collapsed_sides_vertex_is_decided_at_the_band`: the side's own point, an ulp off and half the band's zero are on; twice the escalation is off; between them is undecided.
   - Mutant 5 is now an exact read in place of the band. It reds that row and the in-band row.
3. ε. `the_class_meshes_to_one_digest_at_every_eps` pins one FNV-1a digest per class body at δ 1e-2, over position bits and triangles. The values are identical when measured in three processes at ε 1e-9, 1e-6 and 1e-12, and the suite's ε rows re-derive them. Added a dated section, "A second reader (2026-11-23)", to `work/tesslane/pole-identification-lets-eps-choose-between-two-meshes-of-a-cone-face.md`. No guard built.
4. UNDECIDED. A spline vertex the band cannot place refuses exactly as the analytic arms do, through the same `singular_vertices`: `TessellateError::SphereTrimShape { face, edge, cause: SphereTrimCause::SingularUndecided }`. The variant's NAME is wrong for a spline side: it says "sphere", and its siblings name sphere and cone shapes. It is the right refusal in kind (a classification the band cannot make). I added no variant; the rename is yours to route.
5. The corner issue's last paragraph now says the mesh would read the side decision at the band, as it now reads incidence.

LOCAL RUNS:
- whole mesh suite on the merged tree: 556/556 pass;
- the 9 collapsed rows at ε 1e-9, 1e-6 and 1e-12: all pass;
- clippy -D warnings with and without `budget`, and fmt;
- run-gates.sh: rc 0;
- check-slow-set-names: no dead entry;
- work lint: passes.
- Slow set: added the in-band row (0.37 s) and the digest row (2.53 s), and renamed the in-band entry.

MUTANTS, all caught:
- 1, sides never read: 6 rows red.
- 2, copy kept but no walk: 6 rows red.
- 3, walk axis swapped: 7 rows red.
- 4, corner walked: the axis row.
- 5, exact read: the band row and the in-band row.
- 6, one piece too many: 4 rows red, the digest row among them.
- 7, walked at twice the step: 4 rows red, the digest row among them.

For the PR body:
- the Cure section's identification bullet becomes "decided at the run's band through `at_a_singular_point`, the read topo's lift and the analytic arms make; the polygon keeps a copy exactly where the lift turns the face";
- drop the exact-read sentences;
- the ε row and the pole-identification note join Rows;
- "Still refuses" loses the in-band case.

I'm waiting for the rest of the review findings.
