---
id: ring-2-newtype-over-dinterval
kind: unit
title: RING-2: RingInterval is a newtype over DInterval; poison is dec < Def; every certificate re-pinned with its cause
status: closed
opened: 2026-09-21
closed: 2026-09-22
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

## Closed (2026-09-22) — PR 3032

`RingInterval` is `pub struct RingInterval(DInterval)`: poison is `dec
< Def`, the backend's arithmetic is the ring's, the sign clamp and the
zero annihilator are deleted, `hull`/`clamped_to` keep the refusing
guards, `from_certified` carries the crossing scalar's own bracket
capped at `Trv` (`Def` for a certified one — the doc says why); the
public surface unchanged, every caller compiling unedited. The thirty
endpoint-read hazards ask `is_poison()` first; the register is
executable (`ring_endpoint_census.rs`: 115 reads over 15 files, 70
asking, 45 dispositioned, a fixed extra roster for `ssi.rs` and
`interval.rs`); the differential's four allowlist classes collapse to
zero with bit-identical endpoints at a quoted seed; the subnormal and
overflow corners where the newtype gives one step back are pinned.
Every red row dispositioned by class: 16,995 corpus rows 2,294 tighter
and 0 looser (re-taken on both reviewing arms), the meter rows' exactly
zero certificate the honest answer (every reader walked), the
`step-export` sidecars strict subsets, the sweep digests 49 pads
shrunk / 0 grown, the tess-budget baseline re-cut by its recipe with
RING-2's share all tighter and main's fifty-five looser lily cells
characterised and filed (INSTR). The Q9 gate re-derived onto the
answer, its mutant re-attributed to the inner order. C9 re-worded
naming-only; retiring it, `Enclosure`, `crossing_bracket` and the
three guard residues (`hull`, `clamped_to`, the `Def` cap) is RING-3's.
Reviews: dual, both APPROVE WITH FIXES, both MAJORs bilateral (the
baseline's looser cells; the red-row table short by one head), no
tally candidate; sixteen items, fifteen taken, the member-free
`crossing_bracket` route declined by the bounds gate's ratification
rule; rows: INSTR (the baseline drifts between cuts), PROPS ×2 (the
hand-spelled refusal readers; the sampler's own error has no home),
LIB (no public NURBS-faced fixture door), QUAD's row rewritten. Three
lanes implemented on one arm across two container restarts.
