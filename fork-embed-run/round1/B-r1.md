## For Ev

**Recommendation (likely).** Lever the question at each face's **near run**: how far the face runs out from the shared edge before any part of its boundary that does not touch that edge cuts in (or the face runs out), read from below. The pair's lever is the lesser of the two. The built definition is the **far run**: the farthest any part of the face stands from the edge's whole line or circle.

Read at the near run, the number is a distance a consumer can use: how far the nearest part of either face's boundary that does not meet the shared edge stands from the other face. The promise about a certified solid becomes: **its boundary comes within the band (K·ε) of itself only beside an edge or vertex its faces share, and only between the faces and edges that meet there.**

**Premise check: there are two facts here, not one lever in dispute.**
- *Is the angle real?* A face whose points are ε off can tilt about the edge by ε over its far arm, so the far run is the right lever for this. An ear counts. This is tier 3's wedge fact, the edge's class.
- *Does the in-band strip beside the edge close before either face ends?* Every crease has such a strip, about band/sine wide. It is harmless only if nothing else of either face's boundary lies in it. The right lever is the near run. An ear does not count.
- One reader is right about the first fact and the other about the second. The built code gives the first fact's lever to the question. So the question decides tier 3's fact a second time, which the same ratified sentence forbids ("decides none of it"), and nobody decides the second fact.
- The second fact is the one that answers your PR 7095 question, whether the margin tolerates geometry being ε off. A face ε off moves the edge sideways by ε/sine. For the strip (1e-7 m wide, sine 5e-5) that is 2e-5 m, 200 strip widths. An ear pins the tilt and does nothing about this (sure).
- The far run certifies a pair in which an edge of one face lies 5e-12 m from the other face without meeting it. That breaks the sentence before it: "Pieces of a solid's boundary that do not meet … are more than K·ε apart" (sure).

**Provenance.** The sentence was written by three designers as a reconciled text (commit `b130a7bbd5`, fork-log row 304). You approved it in a PR 7456 comment, with a stated doubt on this lever. The words "narrower" and "measured across it" are not yours. "Measured across it" admits both runs, which is why the readers split.

**Ratified text I would change.** In `[topology.tiers]`, replace "Over each cell it reads the rate … passes by more than the band" with:

> Over each cell it reads how far the two faces stand apart where the first of them ends: the sine of the wedge between them, never more than one, levered at how far the narrower face runs out from what they share before any other part of its boundary cuts in ([tolerance.epsilon]), never at a cell of the question's own subdivision, and that passes by more than the band. So a solid's boundary comes within the band of itself only beside a cell its faces share, between the faces and edges that meet there.

This also changes "the faces' rates summed" (case 4):
- The sum reads the chord between the two faces' far sides, 2·sin(θ/2)·run.
- A point of one face is never farther from the other face than run·sin θ, or the run itself past a right angle (sure).
- For flat faces the wedge's sine can be composed from the two rates, sin(α+β) capped at one. That reads the same under the carrier and under the bisecting plane, which the sum was introduced to achieve (likely).

**The answers as final states**

**A. Near run (recommended).**
- Makes true: within a cell, the least rate times the near run is a lower bound on how far every boundary part that does not meet the shared edge stands from the other face (likely; it rests on the rate being enclosed over the whole cell).
- Refusals stay a length against the band, with the lever as recourse.
- The far-run definition agrees with it wherever a face's boundary first cuts in at its farthest point: rectangle, triangle, disc on its rim, the sector sliver. It is too long elsewhere: ear, L, hole, a hexagon by a factor of two. Too long is the certifying direction.
- Leaves possible: a refusal where the nearest boundary part stands past the other face's side, for example a narrow slot at the edge's end under a very shallow wedge. That is the safe direction.
- A curved face that curls back inside the band is still tier 3's open fold question.
- Reversible: one recipe behind one kept number.

**B. Far run (as built), with the promise narrowed to "the two faces are distinct".**
- This matches `[topology.census]`'s number for a meeting pair ("largest distance over the shorter cell").
- It leaves a face's edge in the band of the other face certified by the question. Refusing such a body falls to other checks.
- The census reads vertex against face on plane bodies. Its own module docs list same-solid curved pairs as undetected.
- The "do not meet" sentence then is not something the question itself keeps.

**C. No lever: beside a crease a cell proves a sign only, and never discharges a cell holding a boundary part that does not meet the shared edge.**
- Same promise as A, with nothing kept.
- Every narrow face is walked down to its width, and an in-band strip refuses as a spent budget with no length to report.
- I rank it below A for that. It was one designer's first end state in row 304 and was withdrawn.

**What each case reads under A** (ε = 1e-9, K = 10)

| case | near run | reading | verdict | why |
|---|---|---|---|---|
| 1 flat strip | 1e-7 | 5e-12 | refuses, as today | its far edge is in the band of the plate |
| 2 eared strip | 1e-7 (the ear is past where the face first ends) | 5e-12 | **refuses** (today certified) | same far edge, same 5e-12; the ear changes the tilt arm only |
| 3 sliver, 10 m | 9.6 m, both faces | about 480ε | certified, as today | the 20 mm in-band stretch holds only the two faces and the side edges, which meet the tip edge at its ends |
| 3 sliver, 1 m | about 1 m | about 50ε | certified, as today | same |
| 3 sliver, 20ε at far end | 9.6 m | about 2 bands | should certify | today's budget refusal is the walk's, not the reading's |
| 4 strip 0.6·K·ε at 2.6 rad | 0.6·K·ε | 0.6 band (sine capped at one) | **refuses** (today certified) | its far edge is 0.6 band from the shared edge itself |
| 4 strip 0.8·K·ε at a right angle | 0.8·K·ε | 0.8 band | **refuses** under every separator | today the plane arm certifies it |
| 5 L, edge along the thin arm | arm's width | sine × width | by that | the arm's far edge is the nearest part |
| 5 hole beside the edge | distance to the hole's rim | sine × that | by that | the rim does not meet the edge |
| 5 line touched again far away | unchanged by the far touch | n/a | n/a | it is not met going out from the edge |
| 6 spline edge | from the edge's own span, as for every kind | n/a | unmeasured | under a least-distance reading a station far along the edge is never the least, so the retired over-read cannot certify |

**What tier 3 keeps.**
- The same slots as now: per face, one length per boundary edge and per vertex shared alone.
- The recipe changes: the least distance from the cell to each boundary edge that shares no vertex with it and each vertex not on it, read from below.
- Three stations per edge bound a least distance from above, which is the unsafe side. The whole stranger edge's cover must be read.
- Where nothing cuts in (a disc on its rim), the present far-run bound stays.
- Vertices take the same rule.

**Check 4 (case 7).** Under A the two readers want different lengths for different facts, so "one recipe on the verdict serves both" is answered no. The near run is never longer than the far run, so the question refusing a body tier 3 passed is the right verdict at the right door. Which face's far arm check 4 takes stays that row's question.

**Confidence.**
- Recommendation: likely.
- Case 2 lies in the band whatever the ear: sure.
- The sum over-reads at wide wedges: sure.
- The near run makes the cell reading a true bound: likely.
- No valid ordinary body is newly refused: unsure, nothing was run.
- Whether any finished body is wrongly certified today: unsure. The answer does not rest on it.

## For the orchestrator

**What I read and did not.**
- I ran nothing. Every case reading is reasoned from the definitions and the two readers' measurements.
- `gh` is unavailable, so I did not read PR 7456's body. I used the row's `## Ruling` and `## Spec`, fork-log row 304 and commit `b130a7bbd5`.
- I read `parts`, `departure`, `extent_of`, `Facts::of`, `BoundaryStations`, `Locus` and `sign_on`'s doc comment, not `sign_on`'s body. The claim that the rate is enclosed over the whole rectangle rests on that comment and on the mean-value argument at the one-shared-vertex arm.

**Provenance found** (`git log --all -S`, `docs/DESIGN.md`).
- "the narrower of" and "never at a cell of the question": `b130a7bbd5`, 22 Nov, agent-written, approved by comment.
- "metered at the reach of the faces it governs": `bd47c7c8a6`, PR 7095.
- "over the shorter cell" and "its extent is not a number the census reads": `ea68daa237`, 9 Oct, both designers on Ev's question.
- "do not meet (no shared edge": `1734a9746b`.
- `[tolerance.epsilon]`'s lever list: `3e1da7e374`, Ev's ruling on PR 5737.
- `-S` output is noisy with `design-fork:` record commits. I filtered them.

**Errors or gaps in the brief.**
- It frames the readers as disputing one lever. They hold two different facts (above).
- "Run … read from below" is safe for a farthest distance. For a least distance, stations are the unsafe side.
- Case 2's "not a finished body" suggests the far run is harmless. It shows only that something else must refuse such a body, and I could not confirm what does for curved faces. `census.rs`'s module docs list "the other SAME-solid distinct-key curved pairs" as undetected.

**What I did not design.**
- The exact recipe for "followed outward from the edge". The safe stand-in is the least cover distance from the edge's own span to every boundary edge sharing no vertex with it and every vertex off it. It under-reads at a slot on the same line, in the refusing direction.
- An adjacent curved edge that leaves the shared edge's end and dips back beside it has no stranger vertex. The stand-in misses it. I have no case for it.

**Defects off the question** (not filed; I edit nothing).
- `parts`' doc ("differ by the wedge's square over eight") holds for small wedges only. It is wrong under any answer.
- `rest_facts.rs`'s module doc says "measured across it"; the recipe is a farthest distance from the whole carrier.
- The unit row's spec item 5 calls the question refusing a tier-3-passed strip "the right verdict at the wrong door". Under A that sentence and the restfront row's design question need rewording.
- The 10 m sliver at 20ε reads about two bands and refuses `Budget Depth 40`. A reading past the band that cannot certify deserves its own row under `work/embed/`.
- `a_crease_under_the_carrier_and_under_the_bisecting_plane_decides_alike` tests one small sine at extent 1. It does not pin the wide-wedge disagreement the reader measured.

**If Ev picks A, re-pins to expect.** The eared-strip probe and the wide-wedge probe flip to refuse. The brick row's runs are unchanged. The sector sliver is unchanged. The half-cone's kept extent (0.70, the cut plane's) is probably unchanged; I did not work it.
