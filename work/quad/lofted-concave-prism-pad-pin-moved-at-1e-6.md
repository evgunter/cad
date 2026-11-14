---
id: lofted-concave-prism-pad-pin-moved-at-1e-6
kind: issue
title: main red at eps 1e-6: the lofted concave prism's volume pad reads 8.637e-4 against its 8.9566e-4 pin
status: closed
opened: 2026-10-31
priority: P1
cost: E
closed: 2026-11-14
---


## Finding

Found by EPSROWS (CLEAR) while running the sweep's loft and sweep rows
at every ε. At `CAD_TOLERANCE_EPS=1e-6`,
`m5_s11_concave_sense::loft_concave_arc_walls_face_out_and_a_flip_is_invisible_below`
(`crates/sweep/tests/m5_s11_concave_sense.rs`, `assert_rational_volume`)
fails on base `855f2757` and with the EPSROWS change alike, at the same
value:

```
the lofted concave prism: volume pad 0.0008636719159652955 moved from the
0.00089566 the ceiling's comment cites
```

The pad moved down by 3.6 %, past the pin's 1 % window. It still sits
under the ceiling (`4·target`), and the oracle check above it passes. The
row was green in nightly #77 (base `39bbdfc7`), so a change merged after
that moved it. QUAD's per-span round-zero work (#6092) is the likely
mover, as it was for the earlier move this pin took
(`rational-wall-pad-pins-trail-the-quotient-statement`); that is not
bisected.

The default ε and `1e-12` rows of the same test pass.

## What is owed

Decide whether the narrower pad is right. If it is, re-measure and
re-pin `LOFTED_CONCAVE_PAD_AT_1E6` and the ceiling comment's 8.96e-4.

## Closed (2026-11-14, GREEN, `green/eps-1e-6`, PR 6934)

Bisected, first-parent, at ε 1e-6: green at `9d79b921e`, `863eb66c7`, `039c43246` and `f249ffe38`; red at `f8e239379`, the merge of PR 6092 (`quad/per-span-round-zero`), whose first parent is `f249ffe38`; red at `bc7aaf7b6`, `c0a070661` and main `b363f5fa1`, at the same value throughout, `8.636719159652955e-4`. The nightly agrees: green on 10-30, red since 10-31. So the mover is the one this row guessed.

The narrower pad is the right one to hold. The pin is a measure of what the fixed schedule achieves, and PR 6092 changed the schedule on purpose: round zero runs per knot span. It re-pinned the sibling measures its own gate ran (`BLADE_10_PAD_AT_DEFAULT_EPS` and `ARC_PRISM_PAD_AT_DEFAULT_EPS`, `2.2106e-6` to `1.1066e-6`); this one is read only at ε 1e-6, which that gate did not run. The enclosure still contains the extruded twin's closed form (the assertion above the pin, which passes) and the pad moved down, 3.6 %, under the same ceiling.

`LOFTED_CONCAVE_PAD_AT_1E6` holds `8.636_7e-4` and the ceiling's comment cites 8.64e-4, 0.84 times the target.
