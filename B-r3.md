# Designer B, round 3: shared-vertex pairing

**Final position:**
1. **Default side:** the kept side, read from the op. Out is wrong, as measured on an ∩ pose.
2. **Shared vertex:** start from the kept side; a pair whose kept-side runs are not clear takes the other start. The choice must be made jointly, not one blamed pair at a time. `SharedVertexCrossings` stays typed where no choice is clear. My nesting placement is not needed by any pose that exists, so I withdraw it.
3. **Turning:** not needed. Retire it and `hang_at_shared`.

## What moved since round 2

- **I withdraw R's cross-pair nesting placement** (question 2).
  - Measured: kept start, plus a per-pair switch to the other start, builds every pose that exists. Nothing needs nesting.
  - The theorem from round 2 still holds, but only as the known construction for a typed limit that has no witness. It says pairing along the partner's kept arcs never crosses, and puts every null edge on one cone. The limit is a link with other links on both of its sides.
- **The default is still the kept side, and it now has a measured counterexample against Out** (question 1).
- **Per-pair choices are not independent** (new; refutes a claim in A's report).
  - On the cleave witness, the prototype's loop flips one blamed pair at a time, and it dead-ends.
  - A joint choice, the fewest departures from the kept side, builds it.

## For Ev

**Recommendation (likely).**
- Every vertex pair starts its pairing where its runs lie on the side the op keeps. That is main's rule.
- At a vertex other crossing pairs share, a pair whose runs on that side would hold another pair's crossings takes the other start instead.
  - The pairs at one vertex choose together: the fewest that leave the kept side.
  - There is no run turning.
  - Where no choice is clear, the typed `SharedVertexCrossings` stays.
- The finish's vertex-set check stays retired.
- The op contributes only the kept side, which is the same fact the result is selected by.

**Terms.**
- *Pair*: the crossings of one vertex of A with one vertex of B at the point.
- *Start*: which of a pair's two ways of joining its crossings in order is used.
- *Run*: the stretch of the vertex's boundary that a null edge splits off.
- *Clear*: holds no other pair's crossing.
- *Cone*: one boundary loop of the result at the point. Each cone is one vertex (your PR 4057 ruling).

**Measured.** Debug builds, on the build branch (PR 6832) plus the probe. The probe is A's switchable prototype with my kept-side start and a joint choice added.

| What | Kept start + joint choice, no turning | Out default (A) | Main |
|---|---|---|---|
| Battery | 5,190 OK SOUND, line-for-line equal to A's run | 5,190 | 4,208 |
| Cleave-witness file (27 tests, incl. all six ops) | 27 pass | 27 pass | — |
| Family 1 (4 editor-core tests) | 4 pass | 4 pass | 4 (per C) |
| Family 2 (22 sweep tests) | 22 pass | 22 pass | 22 (per C) |
| New pose, **∩ one-segment** | **builds**, exact volume | **refuses** `UnpairedLooseEnds{4}` | **builds**, the same as kept |

**Question 1: Out against kept.**
- They differ only in ∩, where A keeps In. In ∪, and in ∖ for its first operand, Out *is* the kept side.
- Every four-crossing pair that A measured at an unshared vertex is a ∪ or a ∖. That is why Out passed.
- I built the pose that separates them: the complement of family 2 near its tip.
  - The first operand is a block with a blind pocket, r = 0.3, floor at z = 0.
  - The second is a slab, z from 0 to 0.5, with a peg r = 0.1 hanging below it, tangent inside the pocket's rim at one point.
  - Near that point, their ∩ is the complement of the boss ∪ holed plate. It pinches into two cones.
- Its unshared vertex pair has four crossings in ∩:
  - The kept start pairs each rim with itself. It builds, with exact volume, two solids, tier 2 and the certificate passing.
  - The Out start pairs each pocket-rim crossing with the peg-rim crossing beside it. That joins germs of two different cones, and the join refuses `UnpairedLooseEnds{4}`. The mechanism is the same as family 2 in ∪, mirrored.
  - Main builds the pose exactly as the kept start does. So the Out default would break a pose main builds.
- (sure, measured)
- Tier 3′ reads `CensusUndecidable` on this ∩ under every start and in the two-arc form as well. That is the census's curved-face limit, not the pairing.
- C's reason for the kept side (the dropped side is the one that holds coincident slivers) and mine (a kept arc is a stretch of one cone) agree here: Out arcs are the coincident slivers in this ∩. In this same pose's ∖ they are real kept faces.

**Question 2: shared vertex.** No pose that exists needs nesting across pairs. The kept start plus the other start covers the battery, the cleave witness and family 1 (likely in general; sure on what was measured). So I withdraw R's placement.
- What R guarantees stays true, and belongs in the limit's row as the known construction: pairing along the partner's kept arcs never crosses, and puts every null edge on one cone. It needs that placement, which today only the unwitnessed shapes would use: three nested cones, or a shell beside a separate piece.
- The joint choice matters (sure, measured).
  - The reconcile's reading of struts nesting depends on which way the other pairs run. So "flip the pair the check blames" can dead-end.
  - In the cleave witness's `y ∪ cube` it flips the two corners and never reaches the band, which is the pair that has to move.
  - The fewest-departures choice over the pairs at the vertex builds it. At most four pairs share a vertex in anything measured, so 2⁴ readings.
  - A germ-only clearance test, which is the per-pair, independent shape A describes, would also do it if built as a real reading. A's prototype loop is not that.

**Question 3: turning.** Not needed (likely; measured on the branch, which has no turning).
- For a pair with two crossings, taking the other start *is* turning its run.
- For four crossings: a turn clears a blocked run only when everything outside that run is clear. In that case the other start's runs, which lie outside it, are clear too.
- So the per-pair start choice covers every turn. `hang_at_shared`, which only turned runs need, retires with it.
- C's "keep turning" is safe but redundant.

**Ratified text.** None governs this (round-1 search). The module docs on the unmerged branch ("the op is not read", the disjoint-pieces premise) need rewriting under this rule.

**Confidence.**
- The kept-side default against Out: sure for this pose; likely in general.
- No pose today needs nesting: likely.
- Turning retires: likely.
- The joint choice is required in place of greedy blame: sure, measured.
- The recommendation as a whole: likely.

## For the orchestrator

- **Probe.** `af-svc/B-r3-probe.patch`, relative to `0fd699740c6`. It includes A's prototype plus:
  - `SVC_B_KEPT`: the kept-side start for every pair.
  - `SVC_B_SEARCH`: the joint choice over shared pairs, fewest departures first.
  - The test `svc_b_dual_crescent_probe`, `#[ignore]`, in `a_closed_conic_in_the_partners_face.rs`.
- **Battery log.** `af-svc/B-r3-bat-all.txt`. I diffed its outcome and volume lines against A's: equal.
- **Measured on main (`1a13ef4bf9a`)**: the dual pose only. ∪ and ∩ build there exactly as under the kept start.
- **Not measured.**
  - The topo test binary as a whole: debug was over 10 minutes, so I stopped it. I ran its cleave-witness file only.
  - The topo and sweep ci profiles, the slow set beyond the family-2 file, and the gates.
  - A germ-only clearance test, as opposed to the search.
- **Possible row.** The dual pose's **∖, one-segment form**, refuses under every start: main and kept give `UnpairedLooseEnds{4}`, In gives `SingleSiteSectionLoop{2}`. That result has one cone at the point, so the pairing is not at fault. It is the join's handling of a closed rim lying in its partner's coplanar face. The two-arc form builds. It is a typed refusal on valid input: a row-2 limit.
- **Refutes A's report.**
  - "Each pair's choice is independent": refuted by the cleave witness under the kept start.
  - "The rim lanes join the cross-horn pairing in ∩": refuted by the dual pose.
- **Process.** Debug builds only, all in the foreground. Clone and target deleted; main checkout checked clean, on `seal/orchestrator`.
