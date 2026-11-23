Round 14 on the turn-lever fork: the unfolded recount is in for core, and the `Dual` question is answered. Read both below (the lane's messages, verbatim in substance), then answer the three questions and return your final text.

A. THE UNFOLDED RECOUNT, core (topo + step-import), ε 1e-9, all 4,799 tests. "raw" is the covers' own far distance from the line (the edge's own cover and each face cover, max) with NOTHING folded in; "below" is the bound from below with the edge points.

| class | reads | raw<below, >1e-12 rel | raw<below, >ε | raw<truth, >1e-12 rel | raw<truth, >ε | Zeros served at such reads |
|---|---|---|---|---|---|---|
| plane, straight edges | 2,422,531 | 160 | 0 | 160 | 0 | 0 |
| plane, straight (inexact) | 484,564 | 6 | 0 | – | – | 0 |
| plane, arcs | 222,780 | 10 | 0 | 10 | 0 | 0 |
| plane, arcs (inexact) | 1,237 | 0 | 0 | – | – | 0 |
| cylinder/cone wall | 10,737 | 108 | 108 | 108 | 108 | 0 |
| cylinder/cone (inexact) | 4,925 | 21 | 21 | – | – | 0 |
| other | 402,314 (395,415 poison) | 0 | 0 | – | – | 0 |

- On plane faces with straight edges the unfolded cover against the VERTICES is never below, past either threshold; the 160 hits are against carrier ends that are not vertices. Every relative-only hit is rounding: at most 2.4e-14 absolute, across 7 doors including the at-rest tier-3 door.
- EVERY read past ε (129) is at one of two doors, both certification mid-construction, and on each the boundary is OFF the surface its face carries: `euler::certify_edge_spec` (an edge certified onto a face still carrying its seed surface: a cylinder of radius 0.0005 with radius-1 rims, the boundary 1 to 2 off the surface; cones too; STEP import's corpora) and `euler::certify_rebased_run` (a rebased run before its move lands; cone boundary points 0.74 to 1.41 off the cone). No read past ε anywhere has its boundary on its surface. The lane records each wall's boundary-off-surface distance to tell these apart.
- Zeros, or any verdict needing the bound from above, at an inverted read: NONE. Both doors are certification, which takes the bound from below only.
- THE DESIGN'S RULE, RUN (an inversion past ε means no bound from above; definite signs stand, Zeros refuse), core at 1e-9: test outcomes changed against the folded run: 0 of 4,799; levered verdicts refused 90 in both; straddles 7 in both, identical by decision; moves against the middle site 34 in both. (The design run made 66 fewer verdicts of 1.8 M; that path difference is not attributed.)
- The ruled truth with the apex guard on: the only cylinder and cone reads below it are the 108, all with the boundary off the surface, where that truth does not apply.
- NOT IN: the same recount and design run on the sweep suite (running; up to four hours each), where most curved walls of finished bodies are; 1e-12 and 1e-6; the symbolic lanes' cost of the curve read; the interior-maximiser check of the ruled truth.

B. THE `Dual` LANE: answered. The NaN is in the derivative only and comes from `geom::cover`'s `gram_top`, a square root at zero on every orthonormal frame (`Dual::sqrt` forms 0/0); every curved cover piece's far distance carries it. No decision reads it: `impl Decide for Dual` classifies on the value alone, and `is_poison` and `Bounds` are value-only. In the second run: 116,359 `Dual` verdicts at 1e-9, 0 refused, 0 straddles, 0 moved. The lane's earlier "poison" label on those 188 reads was its own log parser failing on the NaN. So the two-sided rule does not refuse covered faces on `Dual`. (Filed as its own row for what it does cost, a sensitivity read.)

C. Two figures in your texts disagree and neither is the recount's: one text cites "1.39 M reads" of arc-bounded plane faces in the first run with the cover at or above the truth on all but 80, the other "402,282" where the vertices under-read the truth. Cite the recount's table where it now answers the point, and state any first-run figure only with what it counts.

QUESTIONS, a line or two each, before your text:
(1) Is the core recount with the design's rule run enough to ask Ev now, the sweep's recount stated as running, or does the sweep have to be in first? If it has to be in, say what result on the sweep would change a sentence of (A), (B) or (C), and what would only change the list of what refuses.
(2) Does anything in (A), (B) or (C) change? Quote any changed sentence in full.
(3) The 129 reads are an edge certified onto a face whose recorded surface is not yet its own. In the build, is that the typed missing bound from above (as you both said in round 13), and is it step 1's or already true of certification, which reads from below alone?

THEN your complete, final `## For Ev` section (the decision first; unsigned; every term defined; confidence on the recommendation and each load-bearing claim; the `Dual` paragraph and the evidence paragraphs brought up to date; nothing cited as evidence that holds by construction), and any change to `## For the orchestrator`. Final message only; write no file. At most 150 lines.
