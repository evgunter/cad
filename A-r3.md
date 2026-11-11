## For Ev

**Recommendation (likely).**
- **Default side:** at a vertex no other crossing pair shares, a pair starts on the side the op keeps of its lead. This is main's start.
- **Shared vertex:** a pair keeps that start if its runs hold none of the other pairs' germs. Otherwise it takes the other start. If neither start is clear, it refuses with `SharedVertexCrossings`, typed.
- **No turning:** individual runs are not turned, and `hang_at_shared` goes.

This is close to designer C's rule, minus the turning. I measured it as a prototype (debug):
- topo ci: 1,077 pass.
- sweep ci: 2,824 pass.
- editor-core lib: 412 pass, including all four `touch_reread_rows` tests (family 1).
- The family-2 file: 22 of 22 pass.
- The cleave witness builds.
- Battery: 5,190, by composition (argued below, not re-run).

**Terms.**
- *Lead*: the solid whose vertex several pairs share, otherwise A.
- *Germ*: one crossing of the two link curves on the small sphere round the point.
- *Run*: the arc a null edge moves to its copy vertex.
- *Start*: which of the two consecutive matchings of a pair's germs it takes.
- *Kept / dropped side*: the side the op keeps or drops.
- *Clear*: holding no other pair's germ.

**What moved since round 2.** I give up "Out, op-free" as the default. Kept and Out are measured equal everywhere, but kept is the one with a reason behind it.
- **Out and kept differ only in ∩, at unshared vertices (sure, probed).** In ∪ and ∖ they are the same arcs of A. The lead at an unshared vertex is always A.
- **Where they differ, both build (sure, probed).**
  - Over topo, sweep and editor-core ci, they differ on 18 pairs with more than two germs, in 5 tests. Those are four- and six-crossing corners, the incidence check and the saddle.
  - They also differ on 680 two-germ pairs, in 98 tests.
  - Every one builds under either default.
  - I found no pose where one builds and the other does not. I could not build a ∩ pose with coincident faces and four germs.
  - Complementing family 2 into its ∩ dual turns the touch into containment, which leaves no germs at all. So I cannot show the counterexample exists, nor that it cannot.
- **Why kept anyway (likely).**
  - `recl` codes coincident sectors by the op (Eq. 15.3, `tables::eq15_3_lump`). So the codes the pairing reads already depend on the op, and an "op-free" Out rule over them buys no real independence from it.
  - Family 2's failure is a run laid on a coincident sliver coded as the dropped side. Kept arcs never are such slivers.
  - A kept run's copy holds one stretch of one result cone, which is what the join's one-site rim lane needs.
  - Out avoids dropped slivers only in ∪ and ∖. In ∩ it lands on dropped arcs: safe on every pose measured, but by luck, not by construction.
- **The two defaults are not worth two branches of rule.** Either can be swapped later; this is reversible.

**On designer B's placement (nesting chords across pairs).** No existing pose needs it (sure, probed). With the per-pair choice above, the battery, the cleave witness, family 1 and every ci suite build, and nothing refuses `SharedVertexCrossings`.
- Nesting is needed only where a pair's curve has other curves on both sides. Then neither start is clear. Examples:
  - three nested cones of one operand at one apex, e.g. `(P1 − P2) ∪ P3`;
  - a shell with a separate piece beside it.
  - Both can be constructed, but neither has a witness yet.
- B's own pairing at a shared vertex (along the partner's kept arcs) needs nesting on some battery lines, by B's own account. The per-pair choice needs it on none. That counts against that pairing, not for building nesting now.
- B's invariant, "every null edge on one cone", is a good one. But at a shared vertex the clear side can be the dropped side. The battery's `ab ∪` lines are an example: their runs are In arcs. Those builds rely on `split_cones`, and all 5,190 are SOUND. So the invariant holds only at unshared vertices, where the kept default gives it.
- Recommendation: keep the typed refusal for the no-clear-side case, with a row owing a witness, and build nesting when a pose reaches it.

**Turning is not needed under a per-pair choice (likely, argued and measured).**
- For two germs, turning the run is the same plan as taking the other start.
- For four or more germs, a turned run is the complement of one run. It holds the pair's other germs, and so arcs on both sides of the pair's curve.
  - It is clear only if every other pair's germ lies inside that one run. In that case the other start is clear too, and the per-pair choice already takes it.
  - Where no start is clear, a turned run holds germs on both sides and is not clear either.
- Measured: the prototype has no turning (the branch removed it), and nothing it covers needs it.
- This matches the implementer's reading of main: 0 turns in 372 shared-vertex readings.
- C's family-1 evidence for keeping turning comes from two-germ runs, where a turn and a re-pair coincide.
- Forced struts (`spelled_reversed`), strut-in-strut nesting (`holds_whole`) and ties (`tied_held`) are not turning, and they stay.

**Ratified text.** None governs this (sure). The branch's "the op is not read" and "the pieces of one operand at a point are disjoint" are unmerged agent text, and both should go. The module docs should say instead:
- which side the kept start takes;
- that the codes carry the op at coincident faces;
- that partner curves are disjoint curves, not disjoint disks.

**Confidence.**
- The recommendation: likely.
- Out equals kept outside ∩: sure.
- Both defaults build every measured ∩ pose: sure.
- That an Out-breaking ∩ pose exists: unsure.
- No current pose needs nesting: sure on what was run.
- Turning is redundant: likely.

**Final position.**
1. Default side: the kept side. Main's start, which reads the op through the side it keeps.
2. Shared vertex: a per-pair clear start, with no cross-pair nesting. Nesting is needed only by the unwitnessed no-clear-side case, which refuses typed for now.
3. Turning: retire it, with `hang_at_shared`. The per-pair choice covers it.

## For the orchestrator

- **Prototype.** Built on `0fd699740c6` from `af-svc/svc-a-r2-prototype.patch`, plus two additions:
  - `SVC_A_UNSHARED_KEPT` takes the kept start at unshared vertices.
  - `SVC_A_DIFF=<file>` logs every unshared pair where Out and kept differ.
  - The variant measured is `SVC_A_CLEAR=1 SVC_A_UNSHARED_KEPT=1`. Its `SVC_A_CLEAR` retry starts from In and flips when blocked. At a shared vertex only one start is clear when one is clear at all, so this gives the same plans as "kept, else the other".
  - All runs were debug, through the slot.
- **The battery under the kept variant was not run.** The 5,190 is by composition. Each vertex pair is planned on its own, and the kept variant's plans are:
  - round 2's measured prototype at shared vertices and in ∪ and ∖;
  - the branch's In start at unshared ∩ pairs.
  - Both of those runs gave 5,190 with identical outcome and volume lines.
  - Run it once before landing.
- **Not run:** the slow set outside the family-2 file, topo lib unit tests, the gates.
- **For the implementer.**
  - The landing shape is the choice made in `reconcile_shared` against the other pairs' germs, not my retry loop in the reduction.
  - The other pairs' germs do not depend on which start those pairs take, so each pair's choice is independent and needs no fixed point.
  - The lead and the per-solid `run_holders` from the branch carry over.
- **Possible P3 row.** The join's one-site rim lane joins a closed one-vertex rim only when its two germs share a null edge. Nothing states that dependence, and under the kept default nothing reaches it.
- Every git command this round used `git -C <clone>`. Clone and target are deleted at the end of this run.
