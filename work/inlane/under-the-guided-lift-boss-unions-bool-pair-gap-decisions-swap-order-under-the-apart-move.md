---
id: under-the-guided-lift-boss-unions-bool-pair-gap-decisions-swap-order-under-the-apart-move
kind: issue
title: Under the guided lift, boss_union's apart move decides two bool_pair_gap margins in the other order (read as a laundering), then parts at pcurve_chart_azimuth_frame, decision 3996 of 11384; NOTHING PINS THIS once the row runs in step
status: closed
opened: 2026-10-17
refs: [kitchen-sink-census-pairs-planes-out-of-order]
priority: P2
cost: M
closed: 2026-10-18
---


Found by the shared replay session unit (`inlane/sym-replay-session`), guided lift, apart move.

**NOTHING PINS THIS.** The gating row `revalue_boss_union` now runs under the in-step move, which does not see it. The census below is the only reading until this is fixed and the row returns to the apart move.

- **Move:** Apart.
- **What the instrument reports:** `laundered {"bool_pair_gap"}` at decisions #3993 and #3996. The builds agree through #3996 and first differ at #3997, of 11,384, where `p0` decides `pcurve_chart_azimuth_frame`.
- **What the numbers say:** an ORDER swap, not a constant.
  - #3993: p0 −1.3007496684315063, re-valued at p1 −1.3007497491109032, p1 build −0.5000004552002293.
  - #3996: p0 −1.5500000239999998, re-valued −1.5500017795501375, p1 build −1.3007497491109032.
  - The p1 build's #3996 is bit-for-bit #3993's re-valued margin. So the two builds decided the same `bool_pair_gap` margins in a different order, and position #3993 at p1 is another pair's gap.
  - So either the order the pairs are visited in depends on their values (a sort by gap), or a pair enumeration moved. That is a branch the instrument cannot tell from a laundering. It then parts at `pcurve_chart_azimuth_frame`.
- **Repro:** `REVALUE_APART=1 REVALUE_DETAIL=1 REVALUE_DOC=boss_union cargo nextest run -p editor-core --run-ignored only -E 'test(=revalue_corpus::revalue_census)' --no-capture`

**Same class as `work/coincide/kitchen-sink-census-pairs-planes-out-of-order.md`**: kitchen_sink's `bool_plane_parallel` decisions are re-valued as laundered because the `p1` build decided them in another order (its re-valued #2992 is the `p1` build's #2989 bit for bit). Both read a pairing-order artefact as a laundering because `tally` pairs decisions by position. One fix to the instrument (key a decision by what it decides, not by its index) serves both. This item stays separate for boss_union's parting, which that one does not have.

**Owed:**
- Find what orders `bool_pair_gap`'s decisions. If it is value-sorted, decide whether the instrument should key decisions by pair rather than position.
- Explain the `pcurve_chart_azimuth_frame` parting.
- Return `revalue_boss_union` to the apart move.

## Closed

**What ordered the decisions: a candidate-axis set merged by value.**
- **Where the axes come from.** The narrow phase's separating directions (`topo::boolean::separating::operand_axes`) were every planar face's outward normal on either operand, less any whose bracket key (`boxes::axis_key`) equalled a kept axis's or its negation's.
- **What the merge did.** That comparison is an exact value equality outside the decision funnel. It merges exactly where faces are parallel to the last bit, and that is where they are axis aligned.
  - At boss_union's nominal, the two operands' 8 planar faces gave 3 axes.
  - Under the apart move every face turns by its own ~1e-7, and even a box's opposite caps differ by rounding, so they gave 8.
  - Each extra axis is another `apart_along` read, so every `bool_pair_gap` after it sat at another index. The p1 build's #3996 was p0's #3993.
  - The instrument pairs by index, so it read a laundering. The parting at `pcurve_chart_azimuth_frame` (#3997) was the same misalignment.
- **Not the parallelism.** Making the comparison a banded decision would not help. Faces parallel only by value, like boss_union's faces 4 and 6, tilt apart by ~7e-8 under the move, past the band, so a banded merge would flip there instead.

**The fix: one candidate axis per planar face, with no merge.** The set is now a function of the operands' topology, not of the pose.
- **Answers.** A repeated direction reads a gap its twin read, and adds no separation the twin lacks, so no `apart` answer changes.
- **Cost.** Axis-aligned operands read their repeated directions: two boxes read 12 axes where they read 3. At a generic pose they always did.
- **Removed.** `AxisKey`, `boxes::axis_key` and its `Bounds` census entry are gone, so the narrow phase no longer brackets to compare.

**Rows.**
- `topo::boolean::separating::tests::the_axes_are_one_per_planar_face_at_every_pose`: two cubes read 12 axes, aligned and turned by 1e-7 rad. Red under a value-equality merge, which reads 3 aligned.
- **Apart move.** boss_union now compares 5,804 decisions with 4,699 moved and nothing laundered. It then parts at decision 5804, `point_in_arc_loop_side`: a vertex exactly on an axis-aligned schedule ray's line at the nominal (0 → 7e-8), so the walk grazes at p0 and not at p1. That is plate_param's class, the ray schedule's axis directions meeting axis-aligned geometry, and its item (`under-the-guided-lift-plate-params-apart-move-parts-at-a-point-exactly-on-its-loop`) now owns it.
- **In step.** The census falls back to the in-step move while the apart move parts, so `revalue_boss_union` stays in step: 11,411 compared, 6,517 moved, floor 3,258 (was 3,236). It returns to the apart move with that item's fix.
