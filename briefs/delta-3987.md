# Delta review: PR evgunter/cad#3987 (the boolean door takes and returns AtRestBody), after fix pass 1

You are a DELTA REVIEWER for the REACH orchestrator. Read CLAUDE.md, docs/prompts/reviewer-style-lane.md and docs/prompts/implementer-discipline.md first.

## What you are reviewing
- **Branch:** reach/door-finished-body.
- **Frozen head:** 07ca5a8d051d9df2b1e8ff3e7e9e1aae50ca7593. Review this exact commit.
- **The dual review this pass answers:** it froze at b8eb4dd0eb, and both reviewers said NOT-MERGEABLE-AS-IS. The reports are on analysis/reach-dual/3987-r1 and analysis/reach-dual/3987-r2 (review.md plus probes).
- **The coded fix list and the orchestrator's rulings:** read them with `git fetch origin analysis/reach-briefs/2026-10-03 && git show FETCH_HEAD:briefs/fix-3987-1.md`. The F-numbers there are the findings.
- **Known and accepted:** one red at ε 1e-12, `contact9_side_codes::a_vertex_pair_reads_a_dipping_chord_at_its_far_vertex`. It needs PR #3977, which merges first (ruling F2). Confirm that it is that row only, and that it is the same failure #3977 fixes.

## What you do
1. **Each finding F1–F18:** decide CLOSED, PARTLY or OPEN, with evidence by execution wherever possible. Mount the reviewers' own probes on the new head.
   - Most important is F1: an inside-out operand at a dual must refuse `InsideOutOperand` for every op, in both orders. Try other dual-scalar regressions in the same class: stranded operands, and the result gate at duals, which the lane says it fixed with `ops::structural_gate`.
2. **New defects introduced by the pass:** review the diff b8eb4dd0eb..07ca5a8d051d9df2b1e8ff3e7e9e1aae50ca7593 adversarially, excluding merges from main.
   - `boolean_reduce` and `boolean_reduce_declared` now take `&AtRestBody`, and their callers moved. Look for lost coverage, and for any caller that now finishes a body it shouldn't.
   - The new `topo::test_support::maximal_faces_gate`.
   - The lane's claim that the closed-edge witnesses cannot be finished operands.
3. **The nightly k-lint row:** the lane claims 104 flags on both main and the branch, identical line for line. Re-run it the way the nightly does, if that is feasible, and report.
4. **Mutants:**
   - drop the dual's orientation read;
   - drop `structural_gate`;
   - make the public reduce take `&Body` again (if that compiles);
   - revert the F5 pins.

   Report which rows go red.
5. **ε runs:** the PR's new and changed rows at `CAD_TOLERANCE_EPS` 1e-9, 1e-6 and 1e-12, and the topo, sweep and editor-core suites at 1e-9. Check each red against origin/main.

## Output
- Push `review.md` and the probes to branch analysis/reach-delta/3987. Commits end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.
- The report gives:
  - the verdict, APPROVE, APPROVE-WITH-FIXES or NOT-MERGEABLE-AS-IS, with MAJOR/MINOR/NOTE counts;
  - a table mapping each original finding to CLOSED, PARTLY or OPEN;
  - the new findings, each with file:line and evidence.
- Change nothing on the PR branch, merge nothing, and post no GitHub comments.
- If the permission system denies a step, don't pursue it another way: record it unexercised.
- End your turn with the verdict and the table.
