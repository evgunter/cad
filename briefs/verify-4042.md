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
5. Push a short evidence file, `verify.md`, to branch `analysis/reach-verify/4042`. Make it an orphan or off-main branch, with only that file.
   - Contents: a table of mutant → row → red/green; the ε results; the claim checks; a verdict of VERIFIED, or NOT VERIFIED with the blocking points.
   - Commits end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.
6. End your turn with the same verdict and table as plain text.

Disk is limited. Build only the crates you need, and run `cargo clean -p` between ε runs only if space runs short.

PR: evgunter/cad#4042
Branch: reach/conic-quadric-one-door
Frozen head to verify: 10629e13a4d9c2f5cea8a25d871a32de1bb04a6d (origin/main merged at 515e79ebf)
Review to read for the findings: the single full review on analysis/reach-review/4042, frozen at 395cdbef14 (review.md; its probes/ include door_fuzz.rs, install.sh, tilt_mutant.py, drop_a2_mutant.sh, drop_a2_circle_mutant.sh, door_log.py — use them).

This PR merges the circle × cylinder, circle × sphere and ellipse × sphere/wall root doors into one `boolean::conic_quadric::conic_quadric_roots`, choosing its first-harmonic arm on the second harmonic's amplitude A₂ instead of the circle's tilt. The central bar: no wrong root, no certified miss through a crossing, no `OnSurface` beyond the band, and no body main builds that the head refuses.

Lane claims:
- **MINOR-1.** New row `circle_wall_rows::a_near_square_circle_inside_the_band_by_its_second_harmonic_is_not_on_the_wall` (1 m and 100 m walls, A₂ = 0.7·zero, true depth 1.4·zero). Under `drop_a2_circle_mutant.sh` it is the only red row, at all three ε; green without it. Also re-run `tilt_mutant.py` (both earlier new rows red) and `drop_a2_mutant.sh`.
- **Rounding.** `rounding_charge(terms)` bounds the residual per θ: measured |δc₀|+|δA₁|+|δA₂| ≤ 0.19 of the charge over 40,000 exact-rational trials (method in the PR body). Spot-check the claim by your own small exact or high-precision computation on ~1,000 random conics; a single exceedance is a finding.
- **Style.** One home for the test oracle (`conic_oracle`, with extremum refinement in `crossings`); `ellipse_torus` has one desync; no "square arm" left in `tang_circle_cylinder.rs`.
- **No regression.** Re-run the reviewer's `door_fuzz` (≥ 1,000 poses per ε, all three ε) on the head: 0 wrong. Re-run `door_log.py` on head vs origin/main over `-p topo -p sweep` at 1e-9: identical door inputs and answers.
- **Suites.** nextest `-p topo -p sweep` green at 1e-9 and 1e-12; at 1e-6 only `pocket_ring_steep_ellipse` (main's).
