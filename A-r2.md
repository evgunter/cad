## For Ev

**Recommendation (likely): keep the op out of the insertion, but change how a pair chooses its side.** Two changes:
- At a vertex other crossing pairs share, a pair takes the side of its partner curve that holds none of the other pairs' germs. That side is not always "inside the partner".
- At a vertex no other pair shares, both sides are clear, and the pair takes the lead's Out arcs.

Neither step reads the op.

Measured as a prototype (an `SVC_A_` env switch, debug builds):

| What | Result |
|---|---|
| Battery | 5,190 OK SOUND, 0 ERR; every line's outcome and volume match the build's log |
| Cleave witness | builds |
| Family 1 | the build fails all 4 `touch_reread_rows` tests; the prototype passes all 4 |
| Family 2 | 22 of 22 pass in `a_closed_conic_in_the_partners_face` |
| CI profiles | topo 1,077 pass, sweep 2,824 pass, editor-core lib 408 pass |

**Terms.**
- *Curve*: one vertex's link boundary on the small sphere round the point.
- *Germ*: one crossing of two curves.
- *Pair*: the germs of one vertex-vertex contact.
- *Run*: the arc of the lead's curve that a null edge splits off.
- *Lead*: the solid whose vertex is shared, otherwise A.
- *In / Out arc*: an arc of the lead's curve inside or outside the partner vertex's material.
- *Clear side*: the matching of a pair's germs whose runs hold no other pair's germ.

**What moved since round 1.**
1. **The disjointness premise is false as stated** (sure, measured).
   - What does hold: an operand's curves at one point never cross each other, because its boundary is embedded.
   - What does not hold: the regions they bound need not be disjoint. In family 1, `y` is a pyramid less a nested pyramid. Its two vertices' regions are "inside the outer cone" and "outside the inner cone", and these overlap in the material between the cones.
   - So "inside the partner" is a proxy. It is right only when the other operand's vertices bound separate pieces (the battery and the cleave witness).
   - What reliably holds no other germ is the side of each curve that the other curves are not on. In family 1 that is both pairs' Out side.
   - The probe shows exactly this: both plans flip to Out and every test passes. Main got there by turning each run.
2. **The pairing is not representation-only** (sure, measured). In round 1 I said both starts give one body after the zip. That holds for the finish, but not for the join.
   - Family 2's vertex is not shared. The probe shows one vertex pair, lead A, four germs: the boss rim's two germs along A's edge and the hole rim's two along B's edge, tied pairwise in direction (±y).
   - The In arcs are the two "horns", the tips of the glued crescent. In-arc pairing therefore joins a boss-rim germ to a hole-rim germ across each horn.
   - Out-arc pairing joins each rim's own two germs.
   - The join's one-site rim lanes join the rim-own pairing in every op. They join the cross-horn pairing in ∩ only; in ∪ it fails with `UnpairedLooseEnds`.
   - Main built both because its kept-side start happens to give rim-own in ∪ and cross-horn in ∩.
3. **The op is still not needed** (likely). Lead's Out arcs in every op builds all 22 tests in the family-2 file. The op-chosen start passed only because it coincides with Out in ∪. What family 2 refutes is "In arcs as the tie-break", not "op-free".

**The rule, as a final state.**
- At each vertex pair, a pair pairs consecutively in its lead's walk.
- It starts where the lead's runs are Out arcs, unless one of those runs holds another crossing pair's germ. Then it starts the other way (In arcs), if those hold none.
- Other pairs' germs sit where they sit whichever side those pairs take. So each pair's choice is independent, and no fixed point or turning loop is needed.
- At an unshared vertex nothing blocks, so the pair takes Out.
- Cases:
  - Separate pieces: In, as in the build.
  - Nested cones (family 1): Out.
  - A single pair: Out.
  - Both ways blocked: only happens when another curve lies on each side of the pair's curve. An example is three nested cones of one operand round a shared apex, such as `(P1 − P2) ∪ P3`. Constructible, but no witness exists. It stays `SharedVertexCrossings`, typed, as a named limit, with its own row owing a witness and the nesting.
- **The op contributes nothing.** The finish's vertex-set check stays retired, as built.

**Why Out is the tie-break, not In.** Both sides are geometrically valid splits at an unshared vertex. The choice is a convention. Out is the one the measurement supports: it builds family 2 in every op and changes no ci result. In fails family 2 in ∪. Making the join take the cross-horn pairing in ∪ would make the tie-break free. Nothing reaches that once the rule takes Out, so I would not build it now. This is reversible.

**Other options weighed.**
- **(b) Clear side at shared vertices, op-chosen start elsewhere.** Also passes, by main's own record. But it keeps the op in the insertion for a choice the measurement shows the op does not drive.
- **(c) The build as is.** Fails both families.
- **(d) Main's per-run turning.** It cannot re-pair a pair, which is why the 982 refused.

**Ratified text.** None governs this (sure). The In-arc rule exists only on unmerged PR 6832. Ev's "a pinch is one vertex per cone" (PR 4057) is unchanged, and the rule does not lean on it anymore.

**Confidence.**
- Recommendation: likely.
- The premise correction and the family-2 mechanism: sure (probed).
- Out as a safe tie-break broadly: likely. It covers the topo, sweep and editor-core ci profiles plus the battery. The slow set outside the family-2 file was not run.

## For the orchestrator

- **Incident in your main checkout.**
  - My first setup command chained `cd <dir> && …; git checkout …`. The directory did not exist, so the `cd` failed and the commands after the `;` ran in `/home/user/cad`.
  - They checked out `gauge/in-arc-pairing` there as a branch `inarc`, and created a branch `mainb`.
  - I restored it at once: back on `seal/orchestrator` at `c23d139129b`, working tree clean before and after, both branches deleted. The fetches also left objects and `FETCH_HEAD` behind.
  - No files were edited.
  - Please check that checkout and its session.
- **Prototype.** `/tmp/claude-0/-home-user-cad/1dc20da4-ec95-5f44-ba30-f41a85f77ec3/scratchpad/af-svc/svc-a-r2-prototype.patch`, on `0fd699740c6`.
  - `SVC_A_CLEAR` re-plans a shared plan with the other start when the build's invariant names it.
  - `SVC_A_UNSHARED_OUT` takes Out at unshared vertices.
  - `SVC_A_PROBE` prints each plan.
  - `SVC_A_I` filters the battery by `i`.
  - The retry loop only demonstrates the rule. The landing shape is the per-pair choice in `reconcile_shared`, read against the other pairs' germs.
- **Measured on debug builds.** Release builds would not fit a 10-minute window under slot contention, so the battery ran in debug in 12 chunks of `i`.
  - Battery outcome and volume lines diffed equal to `gauge-build-battery-branch.log`, with its `FEV=` suffix stripped. My run printed no F/E/V counts, so topology counts were not compared.
  - Not run: the remainder of the slow set, topo lib unit tests (the `insert.rs` tests will need updating for the rule anyway), the gates, and main's editor-core.
- **Brief error.** Four `touch_reread_rows` tests fail on the build, not two. The extra two are `a_vertex_read_twice_names_its_result_in_every_op` and `…_at_every_pose`. I did not check whether they pass on main. All four pass under the prototype.
- **Possible row.** The join's one-site rim lanes reject the cross-horn pairing in ∪ (`UnpairedLooseEnds`) but join it in ∩. Nothing reaches this under the rule. It is worth a P3 row if a shared vertex could ever force that pairing, for example tangent rims at a shared apex.
- **Cleanup.** Clone and target are deleted at the end of this run.
