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

PR: evgunter/cad#3976
Branch: reach/lily-leaf-1e12
Frozen head to verify: a17c1efbd0288deb0dc980760546806aa06ff8ba
Review to read for the findings: the single review on branch analysis/reach-review/3976 (read its report for the frozen head)

Lane claims (last fix pass):
- tests/k_probe_brackets.rs checks lo<=hi at every eps (strict for brackets), that both swept leaves are read with ranges containing the Pappus closed-form volume, and that the bracket set is exactly the two leaves at 1e-12 and empty at 1e-9/1e-6.
- Mutants it says now fail: (a) plant volume_hi = volume_lo -> "is not an enclosure"; (b) a range lo<hi displaced off the true volume -> "Pappus ... outside". Add your own: (c) make the bracket set include a third body / drop one leaf.
- The lily Pappus row asserts containment at any width (no skip). Green at all 3 eps.
- k_report.rs reverted to main (check: no diff vs main).
- Wild's bracket branch was removed; wild now requires a number from the import's volume reading; 8/8 cells exit 0.
- New topo::VolumeReading type is the one home for number/bracket/fail; tour gate, lily row and klein use it; teapot keeps its own match.
- scripts/k_probe_sweep.sh exits 0 at all three eps on this head (this was red on main at 1e-12 at lily_leaf_b -- confirm it is green here).
