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

PR: evgunter/cad#3978
Branch: reach/extent-scan-off-face-tangency
Frozen head to verify: 15bf113bab5d538dd238a1e6cb479deac9745b0b
Review to read for the findings: the dual review on analysis/reach-dual/3978-r1 and -r2 (frozen 65e53562cf)

Lane claims (last fix pass) -- NOTE this pass REMOVED a mechanism; scrutinise the argument hardest:
- The touch-ball guard (boundary_clear_of), spread and margin_bound were deleted as redundant. The argument: on a pair with no event, a small loop inside both faces would force an edge of one face within the margin of the other's carrier, which the crossing layer records or refuses (premise S). So a touch clears only on a pair with no event, when its point is placed outside a face; a pair with an event still refuses as tangent. CHECK THIS ARGUMENT against the code: is premise S actually enforced by the crossing layer for every face kind the extent scan handles? Try to construct a counterexample (a loop wholly inside both faces with no edge near the other carrier, e.g. a hole in both faces around the touch, or a face whose boundary is far from the touch point). The lane says the holed sphere face refuses as an operand -- confirm, and try other face kinds (cylinder, cone, torus).
- New row a_touch_is_the_centre_of_every_loop_its_margin_admits; the lane says five mutants of the touch point and of the event rule turn rows red: mutate (1) touch point offset by a fraction of the margin, (2) touch on one carrier only, (3) event rule ignoring events, (4) event rule clearing evented pairs, (5) placement test inverted.
- Rigid-tilt poses and a plate with a hole about the touch now build; rows red on the previous head and green now.
- SphereQuestion docs now say an in-band margin refuses.
- Bounds allowlist count for ops.rs is now 20 (check scripts gate passes).
