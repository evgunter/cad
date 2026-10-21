---
id: uneven-station-lofts-cost-triangles-without-the-column-snap
kind: issue
title: Uneven-station lofts cost up to 14% triangles at degree 2, and 62% on a degree-3 loft, without the retired column snap; a cost lever if a real body needs it
status: open
opened: 2026-10-21
priority: P3
cost: E
---


## What

The malign-band column snap is retired
(`work/tess/the-lower-malign-arm-is-unwitnessed`): over the lattice metric no
corpus body needs it, and on faces built to fire it, outcomes are the same
with and without it. Its one effect there was cost. Degree-2 round lofts
through stations at z = 0, e, 2e, 0.5, 1 (radii 1, 0.98, 0.96, 0.8, 0.5),
at the rows where the snap changed counts:

| arcs | e | δ | snapped bands | triangles with snap | without | ratio |
|---|---|---|---|---|---|---|
| 4 | 0.01 | 5.000e-02 | 64 | 7596 | 8396 | 1.105x |
| 4 | 0.01 | 1.703e-02 | 68 | 14452 | 16076 | 1.112x |
| 4 | 0.01 | 1.301e-02 | 36 | 19388 | 21188 | 1.093x |
| 2 | 0.01 | 5.000e-02 | 34 | 8184 | 8760 | 1.070x |
| 2 | 0.01 | 1.703e-02 | 34 | 16044 | 18284 | 1.140x |
| 2 | 0.01 | 1.301e-02 | 18 | 21564 | 23820 | 1.105x |
| 4 | 0.003 | 5.000e-02 | 68 | 7572 | 7940 | 1.049x |
| 4 | 0.003 | 3.820e-02 | 64 | 8548 | 9220 | 1.079x |
| 2 | 0.003 | 5.000e-02 | 34 | 8520 | 8736 | 1.025x |
| 2 | 0.003 | 3.820e-02 | 34 | 9336 | 9848 | 1.055x |
| 4 | 0.001 | 5.000e-02 | 68 | 8060 | 7916 | 0.982x |
| 4 | 0.001 | 3.820e-02 | 64 | 8548 | 9220 | 1.079x |
| 2 | 0.001 | 5.000e-02 | 34 | 8516 | 8868 | 1.041x |
| 2 | 0.001 | 3.820e-02 | 34 | 9596 | 10108 | 1.053x |

Up to 14% more triangles without it on this degree-2 family, and one row
with fewer. **At degree 3 the cost is larger**: #5551's FULL review measured
a degree-3 loft at 11820 triangles with the snap and 19100 without at
δ = 3.82e-2 (1.616x, https://github.com/evgunter/cad/pull/5551#issuecomment-6314358748).
On the tour it is 0%: the snap fired on no tour band over the metric.

## If a real body needs it

Reinstating the snap as a COST lever, not a soundness one, means judging
"would aligning these bands' columns to the patch count save triangles" on
the band schedule. The retired code is at the parent of the retiring
commit (`NurbsCellGrid::band_schedule`'s fixpoint, `NurbsFaceBound::step_v_at`).
Recorded so the option is not lost; nothing to do until a body pays it.
