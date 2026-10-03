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

PR: evgunter/cad#3973
Branch: reach/ellipse-torus-roots
Frozen head to verify: f6f69915e44dbca2733e1bfbb0572100af648b10
Review to read for the findings: the dual review on analysis/reach-dual/3973-r1 and -r2 (frozen 9f8ab75a24)

Lane claims (last fix pass):
- the_slack_meter_charges_every_term holds five grazes at eps 1e-12; each of the five slack-term mutants the reviewers listed turns it red; also red: meter dropped entirely, degree-4 error bound read at degree 2, old reach formula. Run all eight.
- f_per_metre_hi computed from the frame as stored (non-orthonormal frame charged its defect): mutant = revert to the old (orthonormal-assuming) bound; a row should go red.
- pinned-graze row fails on Miss and checks the meter refuses on its first pose (mutant: return Miss there).
- fuzz row demands exact true count and root position (mutant: drop one root / shift a root by a few bands).
- conic_torus_residual gives the same bits as implicit_residual, pinned by a row.
- 1e-12 crossing row only demands a certificate where the door's rounding is two orders inside the band; 1e-6 sweep row gaps >= 100 eps. Green at all three eps.
- Circle x torus door keeps main's reading (no verdict change except the one r1 found); work/hone/circle-torus-clear-margin-reads-the-floor.md filed.
- The 'no body builds' claim is withdrawn in the PR body.
