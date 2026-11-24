---
id: main-red-two-slow-rows-no-row-holds-at-da61db71a
kind: issue
title: Main red, held by no row: predicate_name_coverage, the cones-apex rows at 1e-6, and the ring path's re-forked planar sense flip
status: open
opened: 2026-11-23
priority: P0
cost: M
---


Found by DOCTAIL's `main-red-the-identity-turns-decisions-moved-the-m10-slow-rows` lane. It ran the whole slow set on `origin/main` `da61db71a` at the default ε, 1e-6 and 1e-12, and re-ran every red on `fa42c60`, PR 7434's first parent. Both rows below are red there too, so neither is 7434's. No open row names either.

- **`test-utils` `predicate_name_coverage::every_deciding_crate_runs_a_roster_and_every_referenced_name_is_decided`** (`predicate_name_coverage.rs`, the assertion near :217). Red at the default ε and at 1e-6. Three literals are "a predicate literal no crate decides":
  - `ct_critical_gap_straddle` in `crates/geom-brep/src/pair_section/gap.rs` (about :712);
  - `pc_parallel_gap` in `crates/geom-brep/src/locus.rs` (about :1156);
  - `pc_parallel_gap` in `crates/sweep/src/offer_rows.rs` (about :357).

  The closed `main-red-predicate-name-coverage-step-import-decides-names-and-runs-no-roster` had another cause. geom-brep's `recourse_roster` also names `PC_PARALLEL_GAP`, as "a site names the const, not declared a carrier" (`main-red-geom-breps-recourse-roster-misses-three-ssi-enclose-cone-sites`). So one change may have moved both.
- **`sweep` `a_plane_through_a_cones_apex::a_plane_through_a_{30,45}_degree_cones_apex_cuts_its_closed_forms`** (`a_plane_through_a_cones_apex.rs`, the assertion near :218). Red at 1e-6 only. At α = 30° and 45°, θ = 0, tilt 0.001, normal 1·n, the plane refuses `Reduce(CrossingEscalated { edge: EdgeKey(6v1), fault: Method { door: PlaneCrossing, fault: Unsettled(Tangency) } })`, where the row pins `ConsecutiveOnSectors`. The default ε is green.

- **`topo` `face_normal::tests::the_planar_sense_flip_lives_in_one_place`** (`face_normal.rs`, the assertion near :1425). Red on `origin/main` after `da61db71a`, at every ε, because it is a source scan. It reports: "`crates/topo/src/ring_path.rs` mints an `OutwardNormal` from a plane's chart normal — the planar sense flip has been re-forked out of face_normal.rs". `ring_path.rs` arrived with GERM's ring-path work (`2fee1eda6e`, "the ring path's plane-meet names keep clear of the boolean engine's"). The test is green on `fa42c60`. EXPORT's open `main-red-one-record-folds-the-face-sense-by-hand` is a sibling census (`no_source_file_folds_the_bit_by_hand`, step-import's `one_record.rs`), not this site.

- **`check-slow-set-names` reds every PR whose test scope takes viewer.** This was seen on PR 7549's `test (app)`. The error is: `slow-set entry matches no test: viewer: app::properties_pane_tests::refused_slot_entry::the_range_buttons_grey_from_the_admission_door`.
  - The test exists (`crates/viewer/src/app.rs`, `mod properties_pane_tests` → `mod refused_slot_entry`). But it compiles only under `--features app`, and the name check lists default-feature tests only.
  - The entry came in with OFFER's `f712510c37` (PR 7548).
  - This is the class CIW's closed `an-app-feature-only-test-cannot-be-listed-in-the-slow-set` names. Either drop the entry, so the row runs in the viewer app-feature job's fast set, or have the name check list `--features app`.

- **Two editor-core slow rows, red at the default ε, 1e-6 and 1e-12 on main as of `02ce465b18`.** Both are green on `da61db71a`. Seen on PR 7549's local evidence run; neither file reads from that PR's edits.
  - `corpus_finish_count::the_corpus_runs_its_pinned_count_of_finishes` (`corpus_finish_count.rs`, the assertion near :42): "the corpus ran 398 finishes, pinned 390".
  - `m10_p_lift::at_sym_a_fixed_box_is_the_build` (`m10_p_lift.rs`, the assertion near :585): "a margin of the held run moved with a parameter symbol: a Fixed axis bound one", left 702.

Neither of the first two was bisected, and neither are these.

**Also seen, on a row GREEN already holds:** `sweep` `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs` is red on `fa42c60`, on `ccabcfb` and on `da61db71a` alike. But its β 0.5, spin 0.5 refusal reads differently:
- `Euler(RebasedCarrier { edge: EdgeKey(15v1), error: PeriodicView(Span { … }) })` on `fa42c60` and `ccabcfb`;
- `Euler(DescriptionUncertified { edge: Keyed(EdgeKey(15v1)), … })` on `da61db71a`.

Opt-level 0 and 2 agree. So a merge in `ccabcfb..da61db71a` changed which door refuses there. It was not bisected.

## 2026-11-23 — the first bullet's two names resolve on main; the row is red on other causes (GREEN U17, PR 7670)

The probe run 45310731023 (`origin/main` `82273ca89b` plus a comment line in the coverage file so the row runs) no longer reports `ct_critical_gap_straddle` or `pc_parallel_gap`. CIW's lane declared their carriers in geom-brep's roster on the branch of PR 7496: `NameCarrier::Call("GapRows")` (`be8f73fde2`) reads `rows.rs`'s `CT_GAP.straddle`, and `NameCarrier::Const("PC_PARALLEL_GAP")` (`65b8387238`, and `c0db369c8b` via PR 7408) reads the `pc_parallel_gap` pair. The row's name is unchanged on main. So **the first bullet is fixed on main**, and PR 7670 carries no change for it. The row is still red on two new causes, filed as `main-red-predicate-name-coverage-meets-quantitys-decide-and-geom-cores-test-local-row-names`: `quantity`'s `Exact::decide` runs no roster, and four `row`/`row_floor` literals sit in geom-core's `k_stats_doors.rs`. The other bullets are another lane's.
