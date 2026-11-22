FINAL — second delta, PR 7096. Reviewed and ran: 8ad2d3fd0f. Base: merged main 54e6a5c5b8. Two commits landed while I ran (4e82b645ea, 469c4b6c54; 290 lines); I read their diff, I did not run them.

**Verdict: APPROVE-WITH-FIXES.** No MAJOR of the walk's own. Both first-review MAJORs are cured on this head. What is owed is the description's account of the D-prism cells, two stale tracker texts, and a green hosted run on the final head.

## Section A (the integration)

- **A1 — held on `mesh`'s integration suite** (not taken: `sweep`, `step-import`). Own per-patch digest hook on both trees, `CAD_FUZZ_SEED=1`, 11,726 common (body, δ) keys, 303/303 rows green on head and 296/296 on main.
  - Every other face: 39,783 patches byte-identical (Cone 2,448, Cylinder 6,983, Plane 23,777, Sphere 2,824, Torus 2,828, Nurbs 916, Spindle 7).
  - Non-plane neighbours of class faces: 85, all identical. Plane neighbours: 1,098 moved (71,306 → 55,358 triangles), 4,911 identical.
  - Class faces: 1,021 moved (Cone 703,708 → 38,408; Cylinder 54,802 → 3,890). No patch grew.
  - Refusals: none added, 7 removed. The 24 rectangle-lane rim-sharers the description names are not in my common set.
  - By reading: `TrimPolygon::faces_holding_side` is main's inline test with the same meaning (`!neighbours && ∃ f ∈ fa ∩ fb` ⇔ non-empty list); `emit` reads emptiness only.
- **A2 — held.** No walked body reaches `OverusedSide` or `UnclearedSharedSide` on any cell, at three ε.
  - With `may_make` forced false, the census refuses 54 of my 1,728 bored-ball cells (meshed 1,566 → 1,512). That is the same count as the unsplit body's `OverusedSide` cells (54); I did not match the two sets cell by cell. `a_walked_face_whose_neighbour_holds_both_rims_shares_no_edge` and `no_walked_triangle_has_an_edge_a_neighbour_makes` go red.
  - Needed against cut: 54 cells need the cut; the rule cuts on 504 bore cells (every R 1.3 and R 3 cell at δ at or past the shell thickness is an over-cut) and 180 band cells.
  - `holds_the_relation`'s per-cell count pins the PR's own bodies only (R 1.05, band). An unasked cut on a body of another thickness would not be caught by it.
- **A3 — corrected; no new MAJOR.** 19,426 cells in common with main (four coarsest δ per body; main's lattice is too heavy below that).
  - 18,884 manifold on both. Main refuses and the head meshes: 264 (`Triangulation` 252, `UnclearedSharedSide` 12).
  - **Meshes on main, refuses on head: 12 cells**, all the ruled thin D-prism family: 0.5 rad at δ 0.1 and 0.2 rad at δ 0.03, EVERY line edge split, six split patterns each. The refusing face is a planar cap (`UnresolvedWinding` on the collinear triple chord-end / split point / chord-end; `Triangulation` at five points), not the walked face.
  - Denser than main: 440 cells. 416 are bore cells at δ at or past the thickness (R 1.05: 62→64, 78→80, 102→104), which the description states. 24 are D-prism cells at +2 (0.5 rad δ 0.1, 0.2 rad δ 0.03; lines 1 and 2: 12→14), which it does not.
  - Triangle totals over the common cells: 9,757,362 on main, 796,024 on head.
- **A4 — held by reading; one statement missing at the site.** A planar neighbour always cuts (midpoint in its plane). A foot off the trimmed domain under-reads the distance, which errs toward cutting. No foot, or undecided, cuts. The other review's point is right: `may_make` reads the neighbour's TRUE surface distance against δ(1 + ALLOWANCE_SHARE), while the neighbour's pass is on its COMPUTED bound. The exclusion is sound only while the band's decision margin dominates that rounding. `may_make`'s doc at 8ad2d3fd0f does not say so.
- **A5 — held by reading.** Nine `RouterDefect` sites, each carrying its face: curved.rs 227, 233, 645; chords.rs 550, 611; walk.rs 267, 271; trimmed.rs 591, 3627. No `MissingEntity "(router defect)"` is left in `mesh/src`. Curved's two rows re-pin by variant (curved.rs 1353, 1487). Moving main's sites here is the right call: one refusal for one kind of defect, and the text of `MissingEntity` is wrong for them. Not run: the Python census against a wheel.
- **A6 — held as far as run.** 7198's rule and the census are live (mutant above). The zip, `Trim::Walked` and 7054's pins are live (mutants below). I did not read all nine conflict resolutions line by line; `trimmed.rs`, `tessellate.rs`' walked arm and `ruled_walk.rs` I did.
- **A7 — corrected, three places.**
  1. Thin D-prism: "exactly as the unsplit body refuses on both trees" is false on this head. The unsplit D-prism meshes on both trees at every δ I ran (12 triangles at δ 1 to 0.1, 8 at 0.03). What refuses on both trees is the body with only a cap's straight chord split. The count on my split patterns is 12 against merged main, not 36. The late commit's row name (`…refuses_as_its_split_caps_do`) has it right; the description should follow.
  2. The 24 D-prism cells at +2 against main are not among the stated over-cut families.
  3. The mutant table's "the zip off: 4 rows" is 3 `split_rulings` rows on this head (the unit row is in `--lib`, which I did not run).
  The bored-ball table's direction and the +2 at coarse δ match my cells.
- **A8 — not verifiable yet.** The hosted run on 8ad2d3fd0f was superseded: run 45067181212 is for 469c4b6c54, with `change filter` queued at 20:34Z when I read it. Locally at 8ad2d3fd0f, release: `mesh --test all` 303/303 with the default filter off (slow rows included) at the default ε; my probe at ε 1e-9, 1e-6, 1e-12. Not run: `mesh --lib`, `geom-brep`, the viewer's slow rows, the wheel.

## Section B (first delta's claims, on this head)

- **B1 — held.** One-chord apex faces with both sides split (1°, 5°, 30°, 45°, 0.1°, 0.001°; the bicone; scale 1 and 1000) all mesh with the count relation and my full oracle. Cones: 14,635 / 16,320 / 9,450 meshed at ε 1e-9 / 1e-12 / 1e-6, zero failures. Fans by construction: the first triangle holds b₀→a₀ and the last aₙ→bₘ whenever n + m ≥ 2, and the corner is the third index; I found no face where it fails.
- **B3 — held by reading.** `route_of`'s fourth-fact arm is the one caller of `walkable`. `face_target` reads `Lane`. `walked()` is gone. With the route's walked fact off, 8 `split_rulings` rows go red.
- **B5 — held.** With `developed_runs` returning nothing, 5 rows go red: the two cut-cone rows, `a_run_of_three_on_a_cut_cone_face_meshes`, `two_runs_anchored_by_the_apex_on_a_cut_cone_face_mesh`, `a_run_anchored_by_the_apex_on_a_needle_refuses_unresolved`. Each behaviour's own mutant was not run.
- **B7 — held by reading.** `sizing::tests::a_walked_triangle_lands_at_delta` exists beside the step functions; the module doc states the apex row and the zip. Not run (`--lib`).
- **B9 — partly.** The bore filing exists and the late commit rewrote it; I did not check its cell list against my 54.
- **B10 — held.** Each mutant, with its red rows:
  - zip off: 3;
  - zip order reversed: 1 (`a_one_chord_face_through_the_apex_with_both_sides_split_meshes`);
  - cut never: 2;
  - cut always: 1 (the bored-ball row);
  - route's walked fact off: 8;
  - re-split off: 5.

## Findings

- **MINOR (DEMONSTRATED):** the description's D-prism account, as A7.1 and A7.2.
- **MINOR (DEMONSTRATED):** the over-cut is unpinned off the PR's own thickness. R 1.3 and R 3 bores are cut on about 450 cells where the census needs none.
- **MINOR (BY-INSPECTION):** `may_make`'s soundness rests on the band dominating the neighbour's certificate rounding; unstated at the site (8ad2d3fd0f).
- **MINOR (BY-INSPECTION):** stale tracker text.
  - `work/tesslane/a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta.md` and `a-cylinder-wall-with-a-split-ruling-exceeds-its-certificate.md` still cite renamed or removed rows.
  - The unit file's "Not measured" still says "The walk itself: nobody has built it" and that cure 1 is unmeasured.
- **NOTE:** `walk_face`'s doc at 8ad2d3fd0f describes the removed centroid form and lists `MissingEntity`; 4e82b645ea says it fixes this (read, not run).
- **NOTE:** main's defects, base-identical: a Plane face with two hair-splits at one corner (`UnresolvedWinding`, 14 cells); the unsplit bore (`OverusedSide`, filed).

## Fate of my earlier findings

- MAJOR-1 (one-chord apex refusal): fixed (the zip); re-run at three ε.
- MAJOR-2 (four-use edge): fixed (the cut, with main's census behind it); re-run.
- MAJOR-3 (CI red on the PR's own row): fixed at 7a6f5bfe6e (`test (app)` green then); unverified on the final head.
- MINOR-4 (7054 pins): fixed; mutant re-run.
- MINOR-5: the dead `continue` is removed; the apex flags are kept with a reason — accepted.
- MINOR-6 (3-D row): fixed.
- MINOR-7: the needle and undecided-apex rows exist; the both-nappe fallback has no row (no body makes one) — accepted.
- Style:
  - `rim_runs` duplicate: fixed.
  - Second predicate (`walked()`): fixed, by the route.
  - `edge_carrier`: one read in 4e82b645ea (read only).
  - `walk_face`'s third copy of the per-triangle loop: not addressed.
  - Ear prose enforced by nothing: now enforced by the cut and the census.
  - `trimmed.rs` header: fixed.
  - Count oracle dropping to winding alone, and stage-1's home: not re-checked.
- Interim's D-prism note: superseded by A3 and A7.1.

## Executed / read only

- **Executed** (release, this container): the probe on head at three ε; the probe on main at the default ε (four coarsest δ); D-prism detail on both trees; the `mesh` integration suite with the digest hook on both trees; six mutants on head.
- **Read only:** the lift, `walk_face`, `may_make`, `ruled_walk.rs`, the nine `RouterDefect` sites, the PR description (as it read at about 19:57Z on 22 November), the two late commits' diff.
- Nothing committed, pushed or posted; the tree is back at 8ad2d3fd0f and the base worktree is removed.
