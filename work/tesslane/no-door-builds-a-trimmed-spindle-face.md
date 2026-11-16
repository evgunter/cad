---
id: no-door-builds-a-trimmed-spindle-face
kind: issue
title: No door builds a trimmed spindle face: the trimmed lane's spindle row is unreached
status: open
opened: 2026-11-14
---


## Finding

The trimmed lane has a spindle row (`crates/mesh/src/chart_table.rs:97`,
its lattice steps at `:308`, its certificate at `:364`), and
`crate::tessellate::lane_of` routes a spindle face there when its outer
loop carries a non-iso carrier. No door builds such a face, so no body
in the suite meshes on that row:

- A revolve of a profile meeting its axis at two vertices (the lemon,
  a bulge loop from `(0, −√0.75)` to `(0, √0.75)`) builds two spindle
  faces bounded by meridians alone, and they mesh on the curved lane.
- Splitting the lemon by a tilted, an axial or a level plane refuses
  `SplitReduceError::CurvedBooleanUnsupported { kind: Spindle }`
  (`crates/topo/src/splitting/neighborhood.rs:54`, `:114`).
- All six booleans of the lemon with a brick refuse
  `BooleanError::CurvedPairUnsupported { site: OperandGate, kind: Spindle }`
  (`crates/topo/src/boolean/reduce.rs:625`).

Measured on `tesslane/chart-table` (PR 6950's fix pass) with a scratch
probe beside `crates/mesh/tests/reach_probes.rs`.

## What is unreached

On the spindle row, nothing past the lane choice has run on a body: the
row's lattice steps (`cap_angular` of the torus steps at a signed major
radius), its certificate (`cert::cert_torus` read at `|R + r·cos φ|`'s
larger lobe), its two apexes as singular points (`Chart::poles`, read
as the row's ends), and the chord pass's speed bound on a spiric or
projected image on a spindle chart.

## What would reach it

A split or a boolean that admits a spindle operand: the split's star
walk taking a spindle sector, and the boolean's operand roster taking a
spindle face against a plane. The first body that does is the row's
first fixture: a lemon split by a tilted plane, meshed at three δ with
`reach_probes`' geometry check (volume, winding against the surface's
own normal, sampled deviation, and the shape bound).
