# PR 7043 — delta re-review at `6f41f1b0ef` (first review at `b6993486b8`)

## Verdict: NOT-MERGEABLE-AS-IS at `6f41f1b0ef`, on one finding

Everything the fix pass was asked for is done and holds. The one blocker is not in the fix pass: this unit has turned two pinned viewer rows red since the frozen head, and at `6f41f1b0ef` they are neither re-pinned nor listed. A later commit, `05a7f8dbfd`, has a message saying it re-pins them; I have not read it. If that commit does only that, and its account matches MAJOR-1 below, my verdict on it would be APPROVE.

## Findings

- **MAJOR-1 · DEMONSTRATED · `crates/viewer/tests/display_budget.rs`, `review_pick_r2.rs`.** `display_budget::the_budget_commits_the_delta_it_always_has` and `review_pick_r2::the_certified_determinant_refuses_no_genuine_crossing_over_the_corpus`:
  - pass on the merge base `b0c685281a`;
  - fail on `b6993486b8` and on `6f41f1b0ef`, identically: tallies (451512, 147856, 16596, 8142) against the pinned (451800, 147952, 16824, 8244).
  - Cause: the clearance itself, first pass. The corpus's 24 trimmed cylinder walls each lay 3 to 26 fewer nodes (e.g. 429 → 426, 12,775 → 12,749), so fewer triangles and rays. In the pick row only the pin assertion fails; its two substantive assertions (no wide winner, no genuine crossing refused) pass.
  - Not the fix pass, not the both-bands rule: those 24 patches are bit-identical on the frozen and fix heads.
  - Not machine-dependent: my numbers are the stacked lane's.
  - Why hosted CI is green: both rows are in the nextest slow set (`.config/nextest.toml`, `viewer::all`), filtered from the per-PR `ci` profile; nightly only. Neither first review ran the viewer (mine included).
  - What is owed: both pins moved with a stated reason, and the PR's "What moved" listing them.
- **MINOR-1 · DEMONSTRATED · hosted CI.** Run 44022681301 on `6f41f1b0ef` is **cancelled** (superseded, as at the frozen head). The green run 44025719402 is on `05a7f8dbfd`. Local on `6f41f1b0ef`: mesh 497/497.
- **NOTE-1 · BY-INSPECTION · PR body, first section.** It says the nine cells are rectangle-lane faces. Consistent with what I measure (the 45 trimmed cone faces of the new row all mesh in the lane, so the refusing face is not the lane's). My first report said "trimmed lane"; that was wrong.
- **NOTE-2 · DEMONSTRATED · `lattice_candidates`.** The cone-step mutant (larger gap) still survives the whole suite. It is now declared at its site as a choice no body distinguishes; I accept that.

## Claims

1. **`emit` refuses at once: held, with a limit.**
   - The bare-sign mutant now reds `a_lattice_laid_without_clearance_cuts_a_triangle_the_lane_refuses` and nothing else (it survived before).
   - Refusing at once is right by inspection: a fold drop needs a resolved normal, and an ear's split puts a point inside the thin triangle, making thinner ones; neither can resolve it.
   - I could not build the attempt at the shipped clearance. The candidate, the cut needle frustum's small rim, needs δ near 10⁻⁸; it meshes resolved at 10⁻⁶ and 10⁻⁷ (38.5 M triangles) and ran the container out of memory at 3·10⁻⁸. Its ears are the cap's ears, so the body refuses on the plane cap there either way.
   - The clearance-0 row pins a configuration the kernel never runs. That is honest as stated in the PR, and it is the only body that reaches the code.
2. **No mesh moved in the fix pass: held; the viewer reds corrected (MAJOR-1).**
   - Mesh suite, every trimmed patch on both heads: 1,642 face×δ records (678 analytic, 964 spline), 0 patches moved, 0 answers changed, 0 certificate readings changed, main's new `cert.rs` included.
   - My census on both heads: 1,261 cells, 1,779 analytic trimmed faces, 0 moved, every answer identical.
3. **Both bands must clear: held.** It is the stricter reading of "its own step" and a defensible one. Cost on everything I can build: nothing (0 of 964 spline records moved). The either-band mutant reds only `a_location_two_bands_lay_is_laid_only_where_both_clear`: no body tells the two apart.
4. **The apex window: held.** `lay` against brute force with lone apex copies: 5,042 random polygons, 0 disagreements (it failed before). My two-case probe now agrees on all 30 nodes. The old-window mutant reds `a_side_from_the_apex_is_searched_at_every_azimuth` alone. No mesh moved.
5. **`inside_point`'s miss: held by reading.** The stated cause matches the code (`developed`'s 26-bit cut on the round trip). No body of mine reaches it: 0 `Triangulation` on a curved face over the suite and census, needle funnels at 10⁻⁴ and 10⁻⁶ included. P3 is right while nothing reaches it. It would be a regression against main if one did, and the row says so.
6. **The payload: held.** Display, `errors.rs`, the binding (`value`, `bound`), the stub and `reach_probes::got` all read the new shape; text true on the planar and rectangle lanes. Boxing is the ordinary answer to `result_large_err`.
7. **Clearance as a parameter, `on_crease`: held.** No `cfg(test)` switch remains. A column exactly on a crease lays the same locations as before (the crease's own copy stands in for the native one); 0 spline patches moved.
8. **My first findings:** below.
9. **The PR body's first section: held**, with NOTE-1. What is missing from the body is MAJOR-1.
10. **The README's two sentences: held.** Both true of the code; no decision moves.
11. **Sweep: held.** Both `mass_props_are_thread_count_invariant` rows are red on `origin/main` (`cdee746161`) here: main's.
12. **Hosted CI on `6f41f1b0ef`: corrected** (MINOR-1).

## Fate of my first findings

- MINOR-1 (nine cells unreported, unpinned): **fixed** — PR body, new row, work item.
- MINOR-2 (emit reader unpinned): **fixed differently** — refuses at once; mutant now caught.
- MINOR-3 (cone choices unpinned): **declined with a reason I accept** — said at each site; cone-step mutant still survives (NOTE-2).
- MINOR-4 (no hosted run on the head): **recurs** (MINOR-1 above).
- NOTE-1 (apex window): **fixed**, with a row.
- NOTE-2 (scale drops the anchor): **filed** P2, not changed; accepted.
- NOTE-3 (band row's doc): **fixed** — doc trimmed to what it asserts.
- NOTE-4 (page vs crease nodes): **fixed** — sentence re-worded.
- NOTE-5 (crease clearance distance): **fixed** — row at 0.99 and 1.01.
- Style, stale STL test comment: **fixed**. "Eight epsilons" restated: **fixed** in docs and Display; the test literal stays, on purpose.
- Style, refusal without payload: **fixed**. `Winding` three-state: **fixed** (gone).
- Style, crease node's step; either-band: **fixed** (per band; both clear).
- Style, `cfg(test)` thread-local: **fixed** (parameter).
- Style, lane runs retries before the join refuses: **fixed** (refuses at once).
- Style, retired-mechanism vocabulary: **partly** — comments corrected; `MAX_GRID_RETRIES`, `Slot::Grid`, `candidates` stay. Fine.
- Style, `Side::Developed` per-side fields; the 1.001 tolerance: **not changed**; the tolerance now has its reason at the row. Fine.

## Executed and read

- **Executed:** instrumented mesh suite on both heads; the two viewer rows on the merge base, the frozen head and the fix head; my census on the fix head; `lay` probes; four mutants on the fix head (bare sign, cone step, either band, old apex window), whole mesh suite each; sweep's five `mass_props…` rows on `origin/main`.
- **Read only:** `pncad-py` delta, the two filed rows, the PR body.
- **Not run:** stl, Python, tour, any ε row but the default.
- **Seen:** no other review's material; the commit message of `05a7f8dbfd`, through the check-run lookup, not fetched.
- **Operational:** the container ran out of disk once (three viewer builds) and out of memory once (the 3·10⁻⁸ needle); I deleted finished build directories and carried on. Nothing committed, pushed or posted; working trees restored.
