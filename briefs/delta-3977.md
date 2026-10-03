# Delta review: PR evgunter/cad#3977 (check 7 reads the interval enclosure), after fix pass 1

You are a DELTA REVIEWER for the REACH orchestrator. Read CLAUDE.md, docs/prompts/reviewer-style-lane.md and docs/prompts/implementer-discipline.md first.

## What you are reviewing
- **Branch:** reach/check7-interval.
- **Frozen head:** 742850049c84bb89785e792220e37d08eb23ecd7. Review this exact commit, even if the branch moves.
- **The dual review this pass answers:** it froze at 1d4f235512 and both reviewers said NOT-MERGEABLE-AS-IS, with 2 MAJORs each. The reports are on analysis/reach-dual/3977-r1 and analysis/reach-dual/3977-r2 (review.md plus probes). Read both.
- **The orchestrator's rulings for fix pass 1:**
  1. Tighten the enclosure, so that the far-origin blow-up is gone.
  2. Check 10 escalates. A shell role it cannot decide refuses typed; it is not skipped.
  3. The `at_infinity_side` sign comes into this unit, read through the scalar's lane and refusing typed when the sign is undecided.
  4. A row pins the widening.
  5. The citation is fixed, and contact9 samples four points.
  6. One role reader and one exact-interval reading.

## What you do
1. **Each finding of both reviews:** decide whether the fix pass closes it. Answer CLOSED, PARTLY or OPEN, and give evidence by execution wherever possible. Mount the reviewers' own probes on the new head where they apply.
2. **New defects introduced by the pass:** review the diff 1d4f235512..742850049c adversarially.
   - The new enclosure is "each planar face summed about a vertex of its own, flux at its plane's distance from the loop-point centroid". Is it a sound enclosure of the exact volume for every face kind check 7 sees, curved faces included? Try to break it with far-origin poses, large scales, near-degenerate faces, and non-planar faces.
   - Check 7 now certifies both signs. Try a valid body it now wrongly refuses, and an inverted body it passes.
   - `ShellRoleUndecided`: check it is wired everywhere, including editor-core, the Python tags and the binding census.
   - `at_infinity_side` and the new `AtRestPolicy` bound on `point_in_solid`.
3. **The behaviour changes the lane disclosed.** Judge whether each is acceptable or a defect:
   - about 40% cost on `validate_geometric`;
   - an in-band cavity is now refused at tier 3, and `dsc_checks::in_band_void_shell_escalates_with_its_valued_ending` was re-pinned;
   - telemetry predicate names moved;
   - the nightly k-lint row was not run. Run `tools/k-lint` the way the nightly does, if that is feasible, and report whether the dev-probe row goes red.
4. **The design question the PR body raises:** whether the `_structural` doors may certify through a lane. State your reading, but don't decide it.
5. **Mutants:** run the obvious ones. Revert the new enclosure to the old one. Flip check 10 back to skipping. Read `at_infinity_side` without the lane. Report which rows go red.
6. **ε runs:** the PR's new and changed rows at `CAD_TOLERANCE_EPS` 1e-9, 1e-6 and 1e-12, and the topo, sweep and editor-core suites at 1e-9. Check each red against origin/main.

## Output
Push `review.md` and any probes to branch analysis/reach-delta/3977, with commits ending `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`. The report contains:
- a verdict: APPROVE, APPROVE-WITH-FIXES or NOT-MERGEABLE-AS-IS;
- MAJOR/MINOR/NOTE counts;
- a table mapping each original finding to CLOSED, PARTLY or OPEN;
- the new findings, each with a file:line and its evidence.

Change nothing on the PR branch, merge nothing, and post no GitHub comments. If the permission system denies a step, don't pursue it another way: record it as unexercised. End your turn with the verdict and the table.
