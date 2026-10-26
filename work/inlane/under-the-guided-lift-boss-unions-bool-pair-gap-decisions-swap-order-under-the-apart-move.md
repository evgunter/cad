---
id: under-the-guided-lift-boss-unions-bool-pair-gap-decisions-swap-order-under-the-apart-move
kind: issue
title: The revalue instrument pairs p0 and p1 decisions by index, so an order change it cannot see reads as a laundering (boss_union's bool_pair_gap under the apart move, kitchen_sink's bool_plane_parallel); pair them by what they decide
status: open
opened: 2026-10-17
refs: [kitchen-sink-census-pairs-planes-out-of-order, under-the-guided-lift-plate-params-apart-move-parts-at-a-point-exactly-on-its-loop]
priority: P2
cost: M
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

**Evidence, PCERT B1 (PR 5225, 2026-10-17):** the `pcurve_chart_azimuth_frame` parting was the old frame lane's landmark cut, at `x = 0`: it decided which formula read an azimuth on `x`'s sign, a second cut beside the window's own. B1 decides the frame across the window's cut line instead (`pcurve_chart_cut_side`), and the frame name is gone. Re-run with the repro above, the apart move still launders the same two `bool_pair_gap` margins (now #4150 and #4153, bit for bit the values above), and the builds then part at #4154 of 11,971 at `pcurve_chart_radial_moving`. So the frame is not why the builds part. They part at whatever comes next after the order swap. The "explain the parting" sub-item is answered for the frame and no further: the parting goes with the order swap, the first owed item.

## Root cause, and why the kernel is not the fix (INLANE orchestrator's ruling (b), 2026-10-18)

**What orders the decisions.** The narrow phase's candidate separating axes (`topo::boolean::separating::operand_axes`) are every planar face's outward normal on either operand, de-duplicated by exact bracket-key identity (`boxes::axis_key`) against a kept axis or its negation.
- At boss_union's nominal the axis-aligned faces merge, 8 normals to 3 axes.
- Under the apart move every face turns by its own ~1e-7 (even a box's opposite caps differ by rounding), so 8 stay 8.
- Each extra axis is another `apart_along` read, so every `bool_pair_gap` after it sits at another index: p1's #3996 is p0's #3993. The instrument pairs by index and reads a laundering. Its parting at `pcurve_chart_azimuth_frame` (#3997) is the same misalignment.

**The kernel fix tried and rejected.** One axis per planar face with no merge (branch `inlane/bool-pair-gap-order`, kept for its measurements) makes the log a function of topology. The apart move then compared 5,804 decisions with nothing laundered and parted at #5804 `point_in_arc_loop_side`, a vertex exactly on an axis-aligned schedule ray's line (0 → 7.000005503301634e-8), which is plate_param's ray-schedule class. The cost, isolated, main against no merge:

| row (sweep) | main | no merge |
|---|---|---|
| `band_apart_partners_on_a_steep_ellipse` | 0.55 s | 1.4 s |
| `germ_interior_oval::a_torus_oval_refuses_every_op_at_the_guard` | 0.75 s | 1.6 s |
| `germ_plane_torus_frame::the_fat_corner_bars_corners_pair_on_one_oval` | 0.8 s | 1.25 s |
| `rest_mate_every_op::arc_split_bore_intersect_and_differences_are_the_closed_form` | 20 s | 31 s |

About 16 fast-set rows would cross the one-second bar. A non-apart pair tries every axis, and two boxes read 12 where they read 3.

**The ruling.** The merge is exact-identity de-duplication of one computed value, not a predicate. It drops only an axis already read, so no verdict and no separation depends on it: a memo, not a decision. A 2–3× cost on the common axis-aligned case to make the instrument's index pairing line up is the instrument asking the kernel to bend. `operand_axes` now says so in its doc, and `separating::tests::the_axis_merge_keeps_two_normals_one_ulp_apart` pins identity, not tolerance.

## Owed: the instrument pairs decisions by what they decide

- **What changes.** `revalue_corpus` (`tally`) pairs p0's and p1's decisions by position. Pair them by what each decides instead: the predicate and a key for its operands (an identity or form key the recording carries), so a decision the other build took at another index, or not at all, is matched or reported as unmatched, never compared with a neighbour's margin.
- **Folded in.** `kitchen-sink-census-pairs-planes-out-of-order` is folded in here: kitchen_sink's `bool_plane_parallel` swap is the same misreading.
- **Done when** boss_union's apart move reads no laundering at the `bool_pair_gap` swap, kitchen_sink's census reads none at its plane swap, and a row pins each.
- **What stays outside.** boss_union's apart move will then meet plate_param's ray-schedule exact zero further on. That item owns it.

**Pinned (2026-10-19, PATHS #5068's fix pass).** `revalue_corpus::revalue_boss_union_apart_to_its_parting` now holds boss_union under the apart move over the decisions before its parting: `bool_pair_gap` owed to this item as the one laundered name, the parting at `pcurve_chart_radial_moving` (3,961 compared, 3,238 moved). A fix here moves both pins; returning `revalue_boss_union` to the apart move retires the prefix row.

**Evidence, INTENT stage 3 D under FORK-REST (2026-10-20):** boss_union's apart move no longer reads the `bool_pair_gap` swap. Only the identity bundle as written places by no map, so the boss's copy (placed by a mate) runs its map at `p0` as at `p1`; `p0`'s faces are no longer exactly axis-aligned where they merged, and both builds visit the separating axes in one order. Measured: `laundered {}`, 4563 compared (the row's floor is 1619), and the builds still part at `pcurve_chart_radial_moving` with no swap before it, so that parting does not go with the swap after all and stays this item's to explain; `revalue_corpus::revalue_boss_union_apart_to_its_parting` owes no laundering. The instrument's index pairing stays owed for kitchen_sink's `bool_plane_parallel` swap, and for any other document whose two builds merge axes differently.
