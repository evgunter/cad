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

PR: evgunter/cad#3982
Branch: reach/split-gate-oriented-box
Frozen head to verify: 4739b3305342041c3d713541728151ac98958b24
Review to read for the findings: the dual review on analysis/reach-dual/3982-r1 and -r2 (frozen 7ca2662950)

Lane claims (last fix pass):
- reach_split_gate_window.rs: cuts 1e-4 s and 1e-7 s into each face refuse at the gate; clear cuts split with volumes checked against an independent grid integral.
- Mutants killed: M3 crest offset read from window's high end; M4 torus v crest read on the u window; M4b sphere zone narrowed by 1e-6 r; M6 spiric axis read unrotated (killed by boxes::tests::the_spiric_edge_reach_holds_the_curve_in_an_aimed_frame). Also pad dropped / pad*0 killed by split_gate_per_face::the_box_is_read_with_its_pad. Run all six.
- zone_extent reads each window end from the rim's stored (h, rho) pair (mutant: revert to sqrt(r^2-h^2) -- does any row notice? report either way, not a blocker).
- coordinate-i-from-i invariant enforced by a row over all seven extents.
- Items filed: reach/split-gate-zone-ignores-the-azimuth-window (P1), reach/split-gate-torus-ring-fallback-has-no-door (P3).
