---
id: main-red-the-quotient-statement-widens-two-rational-wall-volume-pads-past-their-pins
kind: issue
title: main red at 1e-9: cert5 blade-8 and m8_3's rational wall read a volume pad of 2.2106e-6 against the 2.0016e-6 they pin, since b0962d6af1
status: closed
opened: 2026-10-22
priority: P1
cost: E
closed: 2026-10-23
---


Found by FLUX's F1 lane (PR 5590) while separating main's reds from the
branch's. Red on origin/main at `851d214103`, and on every head since
`b0962d6af1` reached main.

## Measured

At `CAD_TOLERANCE_EPS=1e-9`, two rows pin the volume pad of a rational
arc wall to within 1% of a measured `2.0016e-6`, and both now read
`2.2106e-6`, 10.4% wider:

- `crates/sweep/tests/m8_3_rational_volume.rs:258`,
  `tier3_admits_the_rational_wall_body_and_its_volume_brackets_the_extrusion`
  — `ARC_PRISM_PAD_AT_DEFAULT_EPS` (`:273`): "volume pad
  0.0000022105706852422937 moved from the 0.0000020016 the ceiling's
  comment cites".
- `crates/sweep/tests/cert5_offgrid_knot_rational.rs:333`,
  `a_linear_skin_with_six_offgrid_knots_certifies` —
  `BLADE_8_PAD_AT_DEFAULT_EPS` (`:344`): blade-8 (deg 1), the same pad.

Both rows still pass their ceiling (`4·QUAD_TARGET_LEN_FACTOR·ε`) and
their accuracy check; only the pinned measure moved.

## The commit

Bisected on the m8_3 row: `b0962d6af1` ("quad: the rational lane's
second-derivative and pad bounds read the quotient statement") reads
2.2106e-6 and fails; its parent `e62d39b766` (Merge #5579) passes. It
reached main through `3b08bd4507` (Merge origin/main into
quad/rational-quotient-statement).

## What closes it

The quad lane decides whether the wider pad is the named consequence of
reading the rational bounds off the quotient statement (a change that
makes the bound right). If it is, both pins re-baseline to 2.2106e-6 with
the cause named at the cut, and the looseness is filed. If it is not, the
bound is tightened back. `a-quadrature-faces-pad-scales-with-its-shells-size-and-stored-face-order`, closed by PR 5689
tables the same blade-8 pad under its own order question, and moves with
this row's answer.

## Closed

This is the same red as `rational-wall-pad-pins-trail-the-quotient-statement`,
filed on main in parallel. That row closed it by re-pinning both pads to
2.2106e-6 (9c390fac00).
