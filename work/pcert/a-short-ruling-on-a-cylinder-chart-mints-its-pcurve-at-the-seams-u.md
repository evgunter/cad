---
id: a-short-ruling-on-a-cylinder-chart-mints-its-pcurve-at-the-seams-u
kind: issue
title: a short ruling on a cylinder chart mints its pcurve at the seam's u, and certification refuses MapResidual
status: open
opened: 2026-10-15
priority: P2
cost: M
---

Filed by TANG (U2p, branch `tang/pinch-u2p-pair-reading`).

## What

A result edge that is a short ruling of a cylinder wall gets its pcurve
on that wall's chart minted at `u = −0.0`, the chart's seam (`u_ref`),
whatever azimuth the ruling stands at. Certification then refuses
check 3, `|S(P(t)) − C(t)|`, by the chord between the two azimuths:
`Merge(BooleanMerge(Pcurve{Certify{ResidualExceeded{MapResidual,
sample 0}}}))`, or at rest `Pcurves{…}`. It is a wrong mint caught by
certification, not imprecision.

## Reproduction, with no node and no cylinder pair (reachable on main)

B is a unit rod about `y` over `[−2, 2]`, its seam turned 0.4 rad about
its own axis. The second operand is the brick `x ∈ [0, 2]`,
`y ∈ [−h, h]`, `z ∈ [0.3 + 1e-3, 2]`. Its bottom face cuts B's wall
along a ruling `2h` long. Every op:

| ε | h = 1e-8 | h = 1e-6 | h = 1e-3 |
|---|---|---|---|
| 1e-9 | `MapResidual` | builds | builds |
| 1e-12 | `MapResidual` | `MapResidual` | builds |

These are the direct poses. Re-posed, the same cut fails first at the
ring run's winding, which is not a pcurve mint and is filed separately
(`work/hone/a-short-ruling-ring-run-refuses-joindesync-at-a-fine-eps.md`).

## Where U2p meets it

ε 1e-12. A is a unit rod about `z` over `[dz, 2]`, against B as above.
Near the node, the ruling where A's cap meets B's wall between the two
pierces is `2·dz` long. Its pcurve on B's chart is `Harmonic { p0:
(−0.0, 2.0), pl: (0, 1) }`, so `u = −0.0`, where the ruling stands about
0.4 rad from B's seam. The residual is 0.397 m. A second, 2.04 m
ruling on the same chart in the same body also mints `u = −0.0`
(residual 0.0497).

This fails for every re-posed pose with dz from 3e-10 to 1e-7, and for
direct poses with dz from 1e-8 to 1e-7. dz ≥ 1e-4 builds. U2p's 1e-12
near-node cap rows assert this refusal and cite this file.

## Not yet localized

Where the mint chooses the ruling's `u`: `crates/topo/src/pcurves.rs`
(the result's pcurve mint pass) or the closed-form harmonic lane in
`crates/geom-brep/src/pcurve_cache.rs`. A `u` read off the chart at
the ruling's own point, rather than a default, is the expected fix.

