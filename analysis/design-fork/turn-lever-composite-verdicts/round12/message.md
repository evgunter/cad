Round 12 on the turn-lever fork: the second run is in, and I need your final text for Ev.

READ, in full, on `origin/oneround/turn-lever` at `7b62af0248` (`git fetch origin oneround/turn-lever`, then `git show FETCH_HEAD:<path>`):
- `analysis/oneround/turn-lever/g2.md`: the line site decided two-sided as the run's own verdict, with the bound from below reading each boundary edge's farthest point from the line (critical points on conics, 17 samples elsewhere) and a truth for plane, cylinder and cone faces by convexity along rulings. Config A (your passage (A) as signed, the bound from above being the covers') is complete at 1e-9 on four suites and at 1e-12 and 1e-6 on core and sweep. Config B (a closed-form bound from above) is still running.
- `analysis/oneround/turn-lever/g1-above-below.md`: the 80 reads of the first run where the bound from above fell below the truth.

What the lane reports, for you to check against the notes and not take from me:
1. Across all five runs of config A: no served verdict differs from the truth's verdict where a truth exists (9.1 million such verdicts at 1e-9); no read with the bound from above below the bound from below; no wrong-direction pair; one served verdict moved against the middle site (the keel's `bool_rider_wall_side`, Negative to Zero, both bounds 0.067082 and the truth agreeing, the middle site having read 1.002).
2. What is still refused on bodies that are not faces of no width: none from a bound from above loose on arcs, none from the site on curved faces; poison above (the turns' site mint at a sphere's pole cap, a placeholder seed mid-construction, a datum that describes no curve) and three true widths inside the band (5.0e-9, 5.3e-9 and 1.5e-9 at ε 1e-9).
3. Twenty tests fail by intent on faces of no width (the arm gate at lever 0 or 2.8e-17).
4. The 80 reads are not a cover that is not a cover: all 46 sampled are `euler::certify_rebased_run` reading an arc that stands 1.0 to 2.7 m off the plane of the face it bounds, before the move lands, where the first run's closed-form truth (in the face's plane) does not hold. 34 in the same 16 tests were not attributed one by one.
5. The lane's own two cautions: the convexity argument had a hole at a cone's apex (guarded from the 1e-6 runs on; at 1e-9 and 1e-12 it affects only which reads are labelled with a truth), and "equal to the truth" is partly circular (on exact reads the truth IS the bound from below; the independent checks are items 1's first three counts).
6. Beside the fork: certification's `tangent_normal_parallel` decides `sin θ / |κ_rel|`, an arm of 1/|κ_rel| that the reach never caps; in the open row's extrude table it, not the wall join, now refuses.
7. Not in yet: config B; the symbolic lanes' cost of the curve-reading bound from below (on `Interval`, `Dual` and `Sym` it reads a conic by 17 samples); the source of the NaN derivative the bound from above carries on the `Dual` lane on covered faces (188 of 202 sampled reads), which decides whether the two-sided rule refuses covered faces on `Dual`.

QUESTIONS, each answered in a line or two before your text:
(a) By the criterion you both signed ((B) narrows only on a verdict still wrong with truthful bounds on both sides): does (B) narrow, stand as signed, or need this run's config B before you can say?
(b) Does anything in (A), (B) or (C) as signed change? Quote any changed sentence in full.
(c) Is the arc piece's bound from below (item 2 of the note's changes) part of what Ev is asked, part of the build that follows, or a separate fix that gates it?
(d) Does item 7's open `Dual` question have to be answered before Ev is asked, or is it a consequence Ev is told is unmeasured? Say what it would change in the text if the answer were "every covered face refuses on `Dual`".
(e) Is `tangent_normal_parallel`'s uncapped arm inside this fork, or a row beside it?

THEN: your complete, final `## For Ev` section as `docs/prompts/designer.md` §5 defines it (the decision first; each answer as a final state; every term defined; confidence on the recommendation and on each load-bearing claim; unsigned, naming no lane and no model; Ev reads it on a phone and is fluent in the geometry), and a `## For the orchestrator` section. It goes in the PR body verbatim under your label. Ev's standing correction applies to the README text (PR 7410): ratified text says what a thing is, not a list of past misuses. Return it as your final message; write no file. At most 150 lines.
