You are a VERIFIER for the REACH orchestrator in evgunter/cad, working on the PR named below. Read CLAUDE.md and docs/prompts/implementer-discipline.md first.

The implementation lane has finished its LAST fix pass after review. Your job is to check, independently, that what the lane claims is true. You change no code on the PR branch, you merge nothing and you post no GitHub comments.

Steps:
1. `git fetch origin <branch>`, then check out the frozen head given below. If the branch head has moved past it, say so and verify the new head as well.
2. For each mutant listed, apply it by hand, run the named rows, and record red or green. Then revert it.
   - A mutant the lane says is killed but which stays green is a FINDING.
   - Write each mutant as the smallest source edit that matches the description.
3. Run the PR's new or changed test rows at CAD_EPS (or the repo's ε switch, see `scripts/` and the nightly workflow) 1e-9, 1e-6 and 1e-12.
   - Run the changed crates' nextest suites at 1e-9.
   - For each red, check whether it is also red on `origin/main`. If it is, it is not this PR's.
4. Read the diff of the last fix pass (the commits after the review's frozen head) adversarially against the review findings listed. Check each claim below. Report any claim that is false, overstated or unsupported.
5. Push a short evidence file, `verify.md`, to branch `analysis/reach-verify/<PR>`. Make it an orphan or off-main branch, with only that file.
   - Contents: a table of mutant → row → red/green; the ε results; the claim checks; a verdict of VERIFIED, or NOT VERIFIED with the blocking points.
   - Commits end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.
6. End your turn with the same verdict and table as plain text.

Disk is limited. Build only the crates you need, and run `cargo clean -p` between ε runs only if space runs short.

PR: evgunter/cad#3987
Branch: reach/door-finished-body
Frozen head to verify: 149ed1091331b83e0e1f864c6542e2e1a52396e3
Reviews to read for the findings:
- the dual review on analysis/reach-dual/3987-r1 and -r2, frozen at b8eb4dd0eb;
- the delta review on analysis/reach-delta/3987, frozen at 07ca5a8d05.

The fix briefs are `git show FETCH_HEAD:briefs/fix-3987-1.md` and `briefs/fix-3987-last.md` on analysis/reach-briefs/2026-10-03. Mount the delta reviewer's probes from probes/delta-3987/.

This PR makes the boolean doors take and return AtRestBody (finished bodies). It runs main's orientation read at duals, where no at-rest gate runs. The central bar: no operand or result the door used to refuse now passes, at any scalar.

Lane claims:
- **F1.** An inside-out operand at Dual64 refuses InsideOutOperand, in every op and both orders. Mutant MA, which drops the dual orientation read, turns the row red. The class probes from the delta review pass too.
- **Last pass, MINOR 1.** Public rows for NonMaximalFaces and CoplanarNeighbours are restored through topo::union and boolean_reduce, built from the delta reviewer's three poses. The test-support door stays only for the in-band disc (c).
- **Last pass, MINOR 2.** There are compile_fail doctests: boolean_reduce and boolean_reduce_declared reject &Body. There is also the dual public-reduce row. Mutant MC (back to &Body) turns both doctests and the row red.
- **NOTE 1, the merge with #3977 now on main.**
  - contact9 a_vertex_pair_reads_a_dipping_chord_at_its_far_vertex is green at 1e-12.
  - edit_refusal_recourse now lists positive_volume_exact.
  - The dsc_checks in-band void row now reads the refusal at the failed root node, because the door's result gate refuses first with the identical ShellRoleUndecided finding. Check that this is the same finding, and that ProductError::RootInvalid is still exercised elsewhere.
- **Main's new door callers.** cylinder_sphere_frame, far_thin_disc_sign, pocket_ring_steep_ellipse and pocket_wall_crossing_a_side_face now finish their operands. Check that none of them now refuses something main built.
- **Battery.** 11651/11651 across the whole workspace at 1e-9, 1e-6 and 1e-12.
- **k-lint.** The dev-probe k-lint row is red on main by the same flag set (100 on both). Re-check this against the current main if feasible.
