---
id: lofted-concave-prism-pad-pin-moved-at-1e-6
kind: issue
title: main red at eps 1e-6: the lofted concave prism's volume pad reads 8.637e-4 against its 8.9566e-4 pin
status: open
opened: 2026-10-31
priority: P1
cost: E
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
