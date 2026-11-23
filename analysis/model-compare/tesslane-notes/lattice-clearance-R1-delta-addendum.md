# PR 7043 delta — addendum on `05a7f8dbfd` (your message of 05:40, read after my final report)

## Verdict on `05a7f8dbfd`: APPROVE, conditional on one sweep I could not finish

The re-pin commit clears the MAJOR of my delta report. What stands between this and a plain APPROVE is the tour's suite, which I could not complete here; everything else that depends on `mesh` is green.

## The three questions

1. **Is the implementer's account right? Yes.** I measured it independently before reading this message:
   - both rows pass on the merge base `b0c685281a` and fail on `b6993486b8` and `6f41f1b0ef` with identical output, so the first pass moved them and the fix pass did not;
   - the 24 trimmed patches those rows mesh are bit-identical on the frozen and fix heads;
   - the cause is the clearance: each trimmed cylinder wall of the corpus lays 3 to 26 fewer nodes.
   I did not build `ba68f8ed0a`; the frozen head stands in for it.
2. **Are the re-pins right? Yes.**
   - The commit touches only `crates/viewer/tests/display_budget.rs` (the two `cut_cylinder` entries: `largest_probe` 4544 → 4520, `probe_triangles` 7848 → 7816; 81336 → 81208, 84640 → 84504) and `review_pick_r2.rs` (`PINNED` → (451512, 147856, 16596, 8142)).
   - The new tally is exactly what I measured on both heads before the commit existed.
   - Both rows pass at `05a7f8dbfd` here.
   - In the pick row the two substantive assertions (no wide winner, no genuine crossing refused) passed before the re-pin too; only the count moved.
3. **Is any other slow-set row that pins a mesh-derived number still red? None that I could run.** At `05a7f8dbfd`, default nextest profile (no slow-set filter), whole suites of every crate that depends on `mesh`:
   - `viewer` (default features): 1,151 of 1,151;
   - `editor-core`, `pncad`, `stl`: 3,556 of 3,556;
   - `mesh` was 497 of 497 at `6f41f1b0ef`, and the commit does not touch it.

## Not covered

- **`demos/tour`: not completed.** The first attempt ran the container out of disk; the second, alone in a debug build, passed 7 rows in 16 minutes with the `eps_regression` rows still running, and I stopped it. The nightly runs it `--release`. No red seen, but 7 rows is not a sweep.
- **`demos/wild`, `pncad-py`'s Python suite, the viewer's `app`-feature rows, any ε row but the default: not run.**
- **`sweep`:** only the five `mass_props_are_thread_count_invariant` rows, on `origin/main` (two red there, as reported).

## Hosted CI

Run 44025719402 on `05a7f8dbfd` is complete and green, job by job (14 jobs success or skipped, `gate ok` success). Whether its per-PR filter ran the two re-pinned slow-set rows I did not check from the job log; they pass here.

Nothing committed, pushed or posted; working trees clean. I read the re-pin commit's diff only because your message asked; no other review's material seen.
