---
id: ssi-a-corner-on-a-side-the-plane-runs-along-refuses-as-a-graze
kind: issue
title: ssi: a corner of a wall side the plane runs along within the band refuses BoundaryGraze naming that side, though the locus may lie far from it
status: open
opened: 2026-10-03
priority: P2
cost: M
---



(SSI implementer on PR 3862, from the `SideClass::Apart` audit,
2026-10-03.)

## What

In `ssi/boundary.rs`'s `Pass::run`, a side within the band of the plane
that no strip decides (`side_region` returns `None`: no rung has the
wall's slope across it one-signed, or none holds the locus's cover
inside its strip) becomes a roots section, so its two corners are
classified by `Pass::corner_class`. Where the plane runs along that side
(its distance constant, or nearly, along it), the partial along the side
is zero over every corner cell, never one-signed, and the corner refuses
`SsiError::BoundaryGraze` naming the side: "the plane grazes the wall's
u = low side … the intersection meets it tangentially". The locus may
lie nowhere near that side.

Measured 2026-10-03 at ε 1e-9 (`Kε` = 1e-8), on the trough
`y = 4b·u(1 − u)`, `x = u`, `z = v`, against the plane `y = δ`:

| `b` | `δ` | truth | answer |
|---|---|---|---|
| `0.5Kε` | `0.3Kε` | two lines, `u ≈ 0.18` and `0.82`, each 1 m | `BoundaryGraze`, `u = low` side, bracket `[0, 2.4e-7]` |
| `0.5Kε` | `0.9Kε` | empty (the wall peaks `0.4Kε` short of the plane) | the same graze |
| `0.9Kε` | `0.95Kε` | empty | the same graze |

and on `y = s·u − (s + 1)·u²` (rising off `u = 0` at slope `s = 4Kε`,
turning back at once) against `y = 0.5Kε + 0.2Kε·z`: `BoundaryGraze` on
the `v = low` side at its corner.

## Why it matters

The refusal is honest (the wall lies within the band of the plane over
a strip along the side, so nothing there is certain), but its ending is
the wrong family: it names a tangency of the locus to the side and the
move-the-geometry lever, where the geometry is the surfaces' near
tangency across the strip, C7's regime (`BoundaryTangent`'s ending, or
the march's transversality). A side `side_region` decided over a strip
(`Region`, `Clear`, `Apart`) has its corners skipped, which is why the
same plane parallel to a side whose strip holds the cover traces.

## Open

Whether a corner on a side within the band that no strip decides should
be classified at all, or left to the side's own roots and the sweep
(the measurement to take: the trough rows with the corner classification
skipped on such a side, at every ε); or whether the corner's zero
partial along a side within the band should end as `BoundaryTangent`
rather than as a graze.
