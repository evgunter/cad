PR 7043 delta re-review at `6f41f1b0ef` — INTERIM (still running; one chain at a time)

**Verdict so far: NOT-MERGEABLE-AS-IS on one finding, pending one confirming run.**

**MAJOR (DEMONSTRATED, pending the main leg): two viewer rows are red on this unit, and have been since the frozen head.**
- `viewer::all display_budget::the_budget_commits_the_delta_it_always_has` and `review_pick_r2::the_certified_determinant_refuses_no_genuine_crossing_over_the_corpus` fail here on `6f41f1b0ef` AND on `b6993486b8`, with identical output on both: tallies (451512, 147856, 16596, 8142) against the pinned (451800, 147952, 16824, 8244).
- Those are the stacked lane's numbers exactly, so this is not a mesh that differs between machines.
- It is not the fix pass and not the both-bands rule: the reds predate the fix pass, and the 24 trimmed faces those two tests mesh have bit-identical patches on the frozen and fix heads.
- Why hosted CI is green: both tests are in the nextest slow set (`.config/nextest.toml`, the `viewer::all` lines), which the per-PR `ci` profile filters out unless the diff's added lines spell them. They run nightly only. Neither first review ran the viewer either (mine did not).
- The candidate cause is the clearance itself (fewer nodes laid on the viewer corpus's trimmed faces, so fewer triangles: all four tallies fall). I am now running the two tests on the merge base to confirm they are green there; the final report will say.
- What it means: the PR's "What moved" list is incomplete (two pinned viewer tallies move and are not re-pinned or explained), and merging as is turns the nightly red.

**Held so far:**
- Claim 2, mesh suite: every trimmed patch digested on both heads (1,642 face×δ records, 678 analytic and 964 spline): 0 moved, 0 answers changed, 0 certificate readings changed. The fix head's mesh suite is 497/497 here; the frozen head's 489/489.
- Claim 12: hosted run 44022681301 on `6f41f1b0ef` was still in progress at my last read (tools, step import, interval green; the rest running). Will re-read.

**Still to run:** viewer rows on the merge base; my 1,349-cell census on the fix head against the frozen head's (claims 1–3); the `lay`-against-brute-force probe with lone apex copies (claim 4); re-run of my two surviving mutants and the bare-sign mutant (claims 1, 8); sweep's two rows on `origin/main` (claim 11).

One operational note: the container ran out of disk under three viewer builds; I deleted two finished build directories and carried on.
