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

PR: evgunter/cad#3984
Branch: reach/planar-lane-curved-carrier
Frozen head to verify: 075ed8f3a645f48a3fe3eb2f8392578d41bdcb26
Review to read for the findings: the dual review on analysis/reach-dual/3984-r1 and -r2, frozen at 8abb6e7931. The orchestrator's fix brief is briefs/fix-3984-last.md on analysis/reach-briefs/2026-10-03, and it uses F-numbers.

Lane claims (last fix pass):
- **The gate stays.** The lane had pushed a deletion of gate_operand_edges (c4f3840bd..5501a3f03) and then reverted it in c0339802c.
  - CHECK: `git diff 8abb6e7931 HEAD -- crates/`, excluding changes that came in from main, must show no gate deletion and no unreviewed behaviour change from that attempt. Its new rows and doc edits may stay only if they are consistent with the gate standing.
  - List every non-main, non-doc code change since 8abb6e7931, and say for each whether the fix brief asked for it.
- **F6.** Two spiric rows.
  - M5 (only the spiric arm routed to frontier()) turns a_spiric_crossing_a_cylinder_wall_refuses red.
  - A spiric answered as Line turns the planar spiric row and each_carrier_kind red.
  - Run both mutants.
- **F2/F5.** The variant's doc says it is unreachable while the gate stands, with one shared recourse constant. Confirm that no public path raises it.
- **F10/F11/F12.** Conic::of is used in the lane and in the curved arm. The unreachable arm and the crossing_lane wrapper are removed. Check that this is behaviour-neutral by running the rows at all three ε.
- **Filed items.** delete-the-boolean-operand-edge-gate (P1), join-and-continuation-sites-blame-the-edge-gate-for-a-spline-edge, split-insert-crossings-second-edge-clears-arm-is-unpinned, rod-minus-brick-minus-slab-refuses-solids-do-not-cross, and lib/cancellation-edit-race-row-assumes-a-slow-evaluation.
- **Battery.** 7161/7161 at all three ε, and the binding census passes.
