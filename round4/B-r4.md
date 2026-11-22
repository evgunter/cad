## For Ev

**Recommendation** (`likely`). Everything agreed stays. On the one open point I hold the faces' reach, with a correction the case put against it forces: the reach must be the **shorter** face's, measured across the edge.

1. Tier 3 decides each edge's class once: crease with its turn, smooth seam, or cusp end. The finished body keeps the class, the edge's reach, and each face's pole or apex vertex. The embedding question reads these and never classes an edge.
2. In every cell beside a shared edge or vertex, the question makes one banded reading: the rate at which the two faces part there (both faces' rates off the separator, added), times the reach, against K·ε. That is the end state.
3. The reach is how far the shorter of the two faces runs from what they share, measured across the edge. It is one number per edge or vertex, read once and kept on the verdict.
4. The class decides which argument may run. The other face's own surface is a separator only across a crease. A cusp end goes to the second-order argument.
5. Where tier 3 holds no fact (a vertex shared alone, a pole, a face against itself) the question's reading is the only one, at the same kind of reach. Tier 3 gains no new check.
6. Whether tier 3's own lever is right on a face that curves back stays a separate tier 3 question.

**Terms.**
- *Separator*: a surface through the shared edge; the question shows the two faces lie on opposite sides of it.
- *Rate*: the sine of the angle at which a face leaves the separator, enclosed over a cell.
- *Lever*: the length a rate is multiplied by before it is compared with the band K·ε.
- *Reach*: how far a face runs from the edge. Today's code takes the farther face's, measured from the edge's middle.
- *Tier 3's lever*: the least of today's reach and the two faces' curvature radii.

**The case put against the reach, answered.**
- The case: a strip of a 1 mm cylinder, 1e-7 m wide, leaves a 1 m plane at a sine of 1e-4, curls toward it, and ends 5e-12 m from the plane while still leaving at a rate of 2e-8.
- With the reach as built (1 m, the plane's): 2e-8 × 1 = 2e-8, above K·ε = 1e-8. The cell certifies a face that lies inside the band over its whole width. That is wrong, and I concede it against the reach as built. `sure` (arithmetic)
- Tier 3's lever (1 mm) refuses it, but for a reason that is not the cause. Make the strip flat: 1e-7 m wide, folded off the plane at a sine of 5e-5, far edge again 5e-12 m away. No face is curved, so tier 3's lever is the reach, 1 m. Tier 3 reads a crease, and the cell reads 5e-5 × 1 m and certifies. The same face inside the band passes at tier 3's lever. `sure` (arithmetic)
- The cause is that the lever is the big face's extent. The strip is ε off over 1e-7 m, so its direction is uncertain by ε / 1e-7, about 0.01 radian. The plane's metre says nothing about that.
- With the shorter face's reach (1e-7 m) both strips refuse: 2e-8 × 1e-7 and 5e-5 × 1e-7 are far below ε. The same reach in tier 3 reads both edges as tangent rather than creases. `likely`
- This is not a new rule. `[topology.census]` already reads two cells that share an edge "over the shorter cell", with "the cell's own extent as the arm". The profile's 2-D form of this reading is levered at the shorter of the two legs' arms, which its filed row records as your ruling on #5354.

**The case put against tier 3's lever still stands.**
- A cone that owns its apex, cut by a plane through its axis, has an edge from the apex to the rim. Tier 3's lever along it is the cone's radius at each point, which goes to 0 at the apex.
- One lever per edge is then shorter somewhere than what tier 3 used, so the question can refuse for good a crease tier 3 passed. A lever per stretch is 0 on the last stretch, and needs a concession inside the band of the apex.
- The reply so far is that the verdict "would then keep it per stretch". That does not say what the last stretch reads.
- The shorter face's reach is one number for the whole edge and does not vanish at the apex.

**Why the curvature radii need not be in the cell's lever.**
- Measured across the edge, a face that curls away from the edge cannot reach farther than its own diameter. So the curvature that can turn the angle is already in the reach.
- The curvature left out runs along the edge, as where a 1 mm rod stands on a plate. It does not turn the angle: the rod's direction is fixed by its length. Tier 3's lever is 1 mm there, a thousand times too strict. `likely`, by hand

**The other side, at its strongest.** One meter should say how far from an edge two faces may be within the band of each other; if tier 3 holds that an angle governs the gap only out to L, the question should not accept nearness beyond L. Two meters for one nearness have to be kept in step by hand each time tier 3's changes, where a lever read from the verdict follows by itself. And at the reach as built the reading can certify a face that never leaves the band, with nothing but an unrelated pair of faces to catch it.

**The one sentence to rule on.** Beside a shared edge, is the rate at which two faces part levered at the reach of the shorter face across the edge, or at the lever tier 3 decided that edge's class at?

**This answer now depends on how the reach is settled.** I said before that it did not. It needs the reach measured across the edge, which the open ONEROUND question already proposes, and it needs the shorter face's, which that question does not cover. If the reach stays the farther face's, the strip case stands against my side.

**Text I would change.** `[topology.tiers]`, "That reading is the edge's wedge as tier 3 reads it, one reading for both: an angle metered at the reach…" (agent-written from your "presumably unified with the other case"). It should say:
- the question reads from the verdict what tier 3 decided at a shared edge or singular vertex, and decides none of it;
- in each cell it reads the rate at which the two faces part, at the reach of the shorter of the faces it governs, never at a cell of its own subdivision, passing by more than the band;
- where tier 3 holds no fact, that reading is the only one.

## For the orchestrator

**(a) The case against my side, concretely.** As in `For Ev`. Predictions for the measuring lane (first reading by hand):

| case | reach as built | tier 3's lever | shorter face's reach across |
|---|---|---|---|
| A's cylinder strip (r 1e-3, sine 1e-4, width 1e-7, far rate 2e-8) | 2e-8, certifies (wrong) | 2e-11, refuses | 2e-15, refuses |
| flat twin (width 1e-7, sine 5e-5) | 5e-5, certifies (wrong) | 5e-5, certifies (wrong) | 5e-12, refuses |
| cone with its apex, axial plane | holds (the PR's row) | 0 on the last stretch | holds |

Please add the flat twin to the lane's build. It is A's body with the cylinder replaced by a plane, and it is the case that tells "which face's reach" from "whose lever".

**(b) What moved me.**
- A-r3's worked case: I concede it against `arm_of` as built. My round-3 claim that the reading "follows from tier 3's crease on the edge, and adds only off the edge" holds, but it gave no reason to think the reach itself was the right length. It is not, where one face is narrow.
- A-r3's statement of what a pass proves (nearness lies within L of the edge), and the remark that at L = the faces' whole run that is empty. Right for the farther face's reach. With the shorter face's reach and a cell that spans that face, a pass means the two are a band apart by that face's far side, which is C-r3's far-side test made true.
- C-r3's framing (a fact of the pair, not of the edge): adopted. It is why the lever is the pair's shorter extent and not something tier 3 decided about the edge.
- Not moved: tier 3's lever as the cell's lever. The flat twin shows it does not address the case's cause, and the apex case is unanswered.

**What A-r3 gets right that I should not lose.**
- Curvature can make a face's direction uncertain by ε over its radius. Across the edge that is inside the shorter reach. I argued it by hand for cylinders along a ruling and across a rim, not for cones or tori; a reviewer should try a torus face leaving a spiric.
- "One lever on the verdict settles it once" is true of placement. The reach is kept on the verdict too, so `arm_of` goes either way.

**Touches other ground.**
- `wedge_reach.rs`'s module doc says "The reach is the farther face's". That is tier 3's recipe and ONEROUND's doors. Under my answer the question's reach is the shorter face's; whether tier 3's class should use the same is a tier 3 question, to go with the fold's. I think it should: a 1 µm chamfer tilted 1e-5 off a metre face reads crease today, with its far edge 1e-11 m off the big face's plane.
- I found no README or DESIGN sentence that ratifies "farther" (searched DESIGN and the three crate READMEs). `crates/geom-core/README.md`'s "a lever is never shorter than the region its verdict governs" is the nearest; the region two faces part over is the shorter one's.
- The #5354 citation is from `work/honeone/the-wedge-reach-counts-a-faces-run-along-its-edge.md` ("levered by the shorter of the two legs' arms", `seg::joint_tangency`). I did not read PR 5354 itself.
- The vertex-alone reading (`reach_from_vertex`, the farthest of both faces) has the same question: the shorter face's reach from the vertex.

**(c)** As in `For Ev`.

**(d)** As in `For Ev`, three sentences.

**If Ev rules for tier 3's lever**, two things must be specified with it: what the last stretch before an apex reads, and that the flat twin is refused by something else, since that lever passes it.

**Not checked.**
- I ran nothing. All figures are by hand.
- That tier 3 at the shorter reach would refuse A's strip as a lamina (opposed normals, second-order parting 5e-12): expected, not traced through `edge_wedge_over_as`.
- How a body closes A's strip. If the far side is a face that shares no edge with the plane, the distance test refuses the body under every lever, as A-r3 says. The levers differ only where the closing face shares an edge with the plane.
- Read this round: `round4/dispatch.md`, `round3/A-r3.md`, `round3/C-r3.md`, and the honeone row on `origin/embed/remeter`.
