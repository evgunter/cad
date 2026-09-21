---
id: ring-2-newtype-over-dinterval
kind: unit
title: RING-2: RingInterval is a newtype over DInterval; poison is dec < Def; every certificate re-pinned with its cause
status: review
opened: 2026-09-21
branch: scalar/ring-2
pr: 3032
---

## What

`H5` ruling 1's cut (ii), on RING-0's acceptance: `RingInterval`
becomes a newtype over `DInterval` with poison `dec < Def` (NaI and
empty included), the backend's arithmetic under the ring's surface
(sign clamp and zero annihilator deleted; `hull`/`clamped_to` keep the
refusing guards; `from_certified` carries the decoration); the dry
run's 22 red rows dispositioned by class — sixteen tighter pins
re-baselined with the cause named per ruling 2, the clamp/overflow
rows consumer changes, the structural-exactness gate re-derived, any
looser bound a filed finding; the 4,363-row coefficient corpora
re-pinned (2,026 tighter, none looser); the 31-site endpoint register
dispositioned (guarded / refusing-by-`is_poison` / safe by
construction) and made an executable census; INSTR's tess-budget data
re-taken by its recipe; no verdict flips anywhere. A Fable spec (it
moves certified bounds). Spec: `docs/RING-2-SPEC.md` (deleted at
merge). Block SCALAR-B5 slot 2. Ground: PROPS (`ring_interval.rs`,
`spline/*`, `props/*`, `offset_fit.rs`, `patch_bound.rs`, `ssi/*`,
`geom/src/*`), TRIM (`pcurve_cache.rs`), MESH (`chords.rs`,
`nurbs_cert.rs`), SHELL (`offset_meters.rs`), INSTR
(`docs/tess-budget-data/`), the unowned `topo/src/props.rs`,
TCOST/TINT (36 test files), `crates/geom-brep/README.md` C9
(naming-only); announced.
