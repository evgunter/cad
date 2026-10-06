---
id: a-rational-wall-beside-an-integral-one-skins-its-shared-corner-off-unit-weight
kind: issue
title: A rational loft wall skins its shared corner rows with synthesized weights an ulp off 1, so its seam refuses against the integral neighbour
status: open
opened: 2026-10-06
priority: P2
---


## Finding

`skin_on` (`crates/sweep/src/skin.rs`) chooses its lane for the WHOLE
surface: the Cartesian lane only when every weight of every section is
exactly `1.0`, else the homogeneous lane for every control row. A
rational wall (an arc segment) still has integral rows — its two end
rows, the corners it shares with its neighbours, carry weight exactly
`1.0` in every section — and the homogeneous lane synthesizes their
weight channel by LU, landing an ulp off `1.0` for most
parameterizations (the same round-trip the "no synthesized weight
channel" section of `skin`'s docs measures). The neighbouring line wall
is integral and keeps exact `1.0`. The seam between them is the line
wall's `u` iso on one side and the rational wall's corner column on the
other, and `topo::mint_pcurves` refuses it:

```text
Pcurve(Certify { …, error: IsoUnsupported { what: "the seam carrier is not
the chart's own column (its knot/weight structure differs from the
traversed row's) — the hull comparison needs one spline space" } })
```

**Measured** (2026-10-06, on `carve/loft-v-is-the-whole-sets`, and with
that branch's parameter helper restricted to the first strip, i.e. the
rule on `main`): `loft_body` on `tests/common::arc_section(1.0)` at
v-degree 2 refuses with that error at z = `[0, 1, 3]` and at
z = `[0, 0.25, 0.7, 1.3, 2.0, 2.1]`, and builds at z = `[0, 0.3, 1.1, 2]`.
Evenly stacked sections parameterize at exact `k/(n − 1)`, which is why
`cert5_offgrid_knot_rational`'s blades and `m8_3_rational_volume` never
meet it.

On the branch, the corner column of the six-station blade read
`[1.0, 0.9999999999999993, 1.0000000000000002, 1.0, 1.0, 1.0]` while its
parameters were an ulp off `k/5`.

## The fix it points at

Choose the lane per control ROW, not per surface: a row whose weight is
exactly `1.0` in every section is interpolated in Cartesian ℝ³ and its
column carries exact `1.0`. That is `skin`'s own fit-contract argument
(integral in ⇒ integral out, decided by `==` on the input's bits) applied
at the granularity the seams need, and it leaves the rational rows
bitwise unchanged (the collocation substitutes each right-hand column
independently). A per-row draft was written on that branch and taken out
again, unmeasured in isolation (it was only ever run together with a
parameter-rounding bug the branch then fixed), to keep that unit
minimal. Its first measurement should be the rows above, plus
`cert5_offgrid_knot_rational` and `m8_3_rational_volume`.
