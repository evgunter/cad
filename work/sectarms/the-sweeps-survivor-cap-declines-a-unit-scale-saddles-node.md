---
id: the-sweeps-survivor-cap-declines-a-unit-scale-saddles-node
kind: issue
title: The regularity sweep's survivor cap declines a unit-scale saddle's node, so 3b lists only nodes whose in-band disc is small
status: open
priority: P2
cost: M
opened: 2026-11-04
---


Found while building NURBS × implicit stage 3b, which lists the nodes
the classification's regularity sweep decides.

## What happens

The sweep (`crates/geom-brep/src/pair_section/spline_implicit/sweep.rs`)
gives each cell of the wall's rectangle one of three fates:
- excluded where `φ` over it clears the band;
- regular where the wall's slope off the partner along `u` or `v`,
  levered by the feature extent, is decided off zero;
- otherwise halved, down to the accounting floor.

A cell that is neither at the floor survives. Past
`REGULAR_SURVIVORS = 256` survivors the sweep stops and declines
`Budget`. Near a node every slope tends to zero, so the cells within the
band of the node are survivors, and their number grows as the in-band
disc does: roughly `(band / |H|) / floor²`.

Measured against `z = 0` at ε 1e-9, every fixture over `[0, 1]²`:

| wall | `k = 1` | `k = 2` | `k = 4`, 8, 16 |
|---|---|---|---|
| cross saddle `k·(x − ½)(y − ½)` | `Declined(Budget)`, 257 survivors | `LoopFree`, one node | `LoopFree` |
| oblique saddle `k·((x − ½)² − 2(y − ½)²)` | `Declined(Budget)` | `LoopFree` | `LoopFree` |

The same budget refusal appears on the saddle `(x − ½)² − (y − ½)²` over
`y ∈ [0.1, 0.9]` (a stretched chart).

So a unit-scale saddle's node is never decided. Its section falls to the
whole reading with no node handed in, and the march refuses at the
node. Stage 3b's rows use `k ≥ 2`.

## What closes it

Survivors that cluster about one decided point should count as that
point once, not once per floor cell. For example, a cluster's cells
could be counted against the cap only after the cluster is decided, or
the cap could be read per cluster. Add a row with the `k = 1` cross saddle
listing its node.

## A nearly osculating node waits on it

`a-spline-implicit-arc-is-carried-to-its-node` owes a row for a nearly
osculating node: a sphere of radius 1 centred `(0, 0, 1)` resting on the
biquadratic wall `z = 0.505x² + 0.495y²` over `[−0.1, 0.1]²`, its node at
the origin with `φ`'s Hessian about a hundredth of either surface's.
Measured on `sectarms/spline-node-carry` at ε 1e-9: the sweep's one
piece banks 257 survivors at a floor of `4.97e-9` and declines `Budget`;
the whole reading then marches through the node unlisted and refuses
`TubeStraddles` (as `Unaccounted`) after about ten minutes in debug. Its
in-band disc reaches about `band/(|H|·extent)`, `7e-7` m here, so it
holds of the order of `10⁵` floor cells: counting a cluster once does
not close it while every one of those cells is still visited. The row
lands with this item.

