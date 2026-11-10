---
id: a-rigidly-copied-cylinder-walls-incidence-identity-has-no-sym-rule
kind: issue
title: On a rigidly copied body, a cylinder wall's incidence identity is unprovable under the shipped Sym rules, costing 12 numeric decisions in the m10 D-tab
status: open
priority: P2
cost: M
opened: 2026-11-10
---


Filed by the PCERT main-reds lane (PR 6528, review fix EE9).

## What

`editor-core m10_bulge_interval`'s D-tab copies a body through a rigid
placement whose map runs at the nominal. On the copy, a line carrier
on the cylinder wall stands at the wall's radius only through the
map's rotation: `|R·(o − c)|² = r²` holds by `cos² + sin² = 1` (and
the rotation's orthogonality), a value identity the shipped Sym rules
do not reduce. Every reading of that fact on the copied body therefore
decides numerically at the witness.

In the D-tab (`crates/editor-core/tests/m10_bulge_interval.rs`,
`D_TAB_AT_THE_NOMINAL`) that is 12 numeric decisions, 4 in each of:

- `pcurve_cylinder_chart_incident [16, 4]`;
- `pcurve_line_chart_incident [20, 4]`;
- `pcurve_chart_radial_moving [18, 24]`, the 4 that PR 6504's class
  gate standoff added (`[22, 20] → [18, 24]`).

The last of the three is a second read of the same fact
(`work/pcert/a-cylinder-lines-class-gate-re-decides-its-incidence-as-the-standoff.md`);
fixing that removes those 4 without a rule. The other 8 need one.

## Fix

A rule that reduces a rigidly mapped point's distance from a mapped
axis to the unmapped one (the rotation's orthonormality as a ring
identity), or a receipt carrying the placement's isometry so the
incidence margin is the unmapped body's. Re-pin the three D-tab rows
when it lands.
