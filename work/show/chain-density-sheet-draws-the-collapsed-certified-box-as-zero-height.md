---
id: chain-density-sheet-draws-the-collapsed-certified-box-as-zero-height
kind: issue
title: the chain density sheet draws the collapsed certified box as zero-height rects and still calls the advisory cloud nine times it
status: closed
opened: 2026-10-03
priority: P4
cost: E
closed: 2026-10-03
---


Render commit `f1e797bb24` (`render(mc): re-baseline committed cells`,
on `show/per-body-delta`, merged via #3905) changed
`demos/renders-mc/chain-density.svg`. Eleven teal (`#0f766e`) rects went
to `height="0.00"`, for example `y="262.22" height="1.55"` became
`y="263.00" height="0.00"`. They are not histogram bars. They are the
**certified enclosure per joint pin**, drawn by `Panel::certified_box`
(`demos/tour/src/mcchain.rs:749`), and the table's "certified" column
went to `0.0000 mm`.

## Cause: a deliberate data re-baseline, not render noise

The sheet does not compute the certified box. It draws the
`CERTIFIED_PIN_BOX` literals (`demos/tour/src/chain.rs:242`). Only one
commit since the last hand commit of the sheet (`2c50daa394`) changed
them: `a30963372a` ("tour: chaintol re-baselined to the placed rows'
angular comparisons"), merged to main in #3759 (`82b52c36cb`). That
merge came after the 2026-10-02 local tour that matched byte for byte
(`4f1df81a39`). Since the extrude closes with the pcurve mint, the
certified lane refuses at `pcurve_loop_continuity` /
`pcurve_trim_containment`. That moved the box from `0.111` to
`6.751e-8` of the study and the half-widths across the chain from
`3.996e-5..3.996e-4` m to `2.430e-11..2.430e-10` m. At the wide panel's
scale, `2·dy·s` is about 1e-6 px, so it prints as `0.00`. The CI
re-render is faithful to the constants. #3921, #3894, #3914 and #3898
do not touch the teal data (#3898's edits to `chain.rs`/`mcchain.rs`
move no pixel of the sheet).

Reproduced locally (Linux, release) on origin/main `69700de2c1`. The
full tour's `out/mc/chain-density.svg` is byte-identical to the
committed, zeroed file, so this is not platform non-determinism. Across
the #3759 merge, `CERTIFIABLE_FRACTION` is `1.110e-1` on its first
parent and `6.751e-8` on the merge.

The collapse itself is the kernel's and is already filed:
`work/pcert/pcurve-loop-decisions-state-a-3d-identity-plus-a-branch-margin`
(P0). When that lands it restores `[1.0, 0.3702, 0.1851, 0.111]`. This
row is about what the SHEET says while the box is collapsed.

## What the sheet gets wrong at the new box

1. **A zero-height rect is drawn and called a line.** `certified_box`
   floors only the width along the chain at `CERTIFIED_MIN_PX`
   (`mcchain.rs:756`). The height across the chain, `2.0 * dy * s`
   (`mcchain.rs:761`), has no floor. The legend still says "the box
   draws as a line and is widened to 5 px to be seen at all"
   (`mcchain.rs:1056`), and `chain.rs:240-241` says "the sheet widens
   it to be seen". It is now a 5-px horizontal stroke at the nominal pin,
   and nothing tells a reader it is a sub-picometre box and not a
   rendering defect.
2. **"nine times the certified box" is a stale literal**
   (`mcchain.rs:1099`). It was `1/0.111`. At `6.751e-8` the ratio is
   about `1.5e7`. That sentence is now false on a committed artifact.
   Derive it from `CERTIFIABLE_FRACTION` or drop it.
3. **The table's certified column prints `0.0000 mm`**
   (`mcchain.rs:971`, `{:.4} mm`). The measured `2.430e-11` m is lost.
   Use an exponent format, as the legend already does since
   `a30963372a`.

`check_drawn` checks only polygons and pin dots, so none of this can
fail the tour.

## Fix

This is demo-layer only, with no kernel change. Draw the across-the-chain
extent with a floor too, or as a marked tick, and say so in the legend.
Compute the advisory-over-certified ratio. Print the column in `e`
notation. Re-render `chain-density.svg`. When the pcert follow-on
restores `0.111`, the same code draws the old boxes again.

## Closed

Fixed in the PR that filed this row, in `demos/tour/src/mcchain.rs`:

- `Panel::certified_box` floors both sides at `CERTIFIED_MIN_PX`,
  centred on the nominal pin. The legend says which sides are floored
  (`floored_sides`, computed from `CERTIFIED_PIN_BOX` at the panel's
  scale) and points to the table for the true half-widths. The
  `CERTIFIED_PIN_BOX` doc in `chain.rs` says the same.
- The "N times the certified box" ratio is `1 / CERTIFIABLE_FRACTION`
  (`study_over_certified`). It is now `1.48e7` and will be `9.0` again
  at `0.111`. The legend for a non-default ε formats the default-ε
  fraction from the constant as well.
- The table's certified column prints `{:.3e} m`.
- `check_certified` reads the overlay back out of the finished sheet.
  It checks the count of boxes per panel, that each box is centred on
  its pin, and each side as `max(2·half·s, floor)`. It also checks the
  table cells against `CERTIFIED_PIN_BOX` and the legend's ratio against
  `CERTIFIABLE_FRACTION`. With the height floor removed, the tour reds:
  `pin 2's certified box is drawn 0 px across the chain`.

The data collapse itself stays with
`work/pcert/pcurve-loop-decisions-state-a-3d-identity-plus-a-branch-margin`.
