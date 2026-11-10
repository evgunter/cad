---
id: a-rigid-map-admitted-on-the-metre-band-mints-frames-check-1-refuses
kind: issue
title: transform_rigid admits a map whose columns are off unit by up to ~eps, dimensionless and unlevered; the mapped cylinders of radius > ~1 m then refuse at check 1, at a door that does not name the map
status: review
opened: 2026-09-25
priority: P1
cost: M
refs: [3238]
pr: 6717
branch: onehome/rigid-motion
---


## What

`topo`'s rigidity door (`crates/topo/src/transform.rs`, `check_rigid`,
reached through `transform_rigid`) admits a map when each of its seven
defects — `cᵢ·cᵢ − 1`, `cᵢ·cⱼ`, `det − 1` — decides `Zero` against the
LINEAR (metre) band. The defects are dimensionless and unlevered; the
site says so and flags it as ledger row **F10** (`decide_flagged(…,
"F10")`: "the natural arm is the model/session-box extent — an
arm-policy question deferred by the row";
`work/meta/decide-flagged-dimensional-debt-inventory.md`). So at
ε = 1e-9 a map whose columns are 4e-10 off unit is admitted as rigid.

ATREST-13 (PR 3238) made tier 3's check 1 read every analytic frame's
unit-ness and orthogonality to within ε of LOCUS MOVEMENT at the kind's
radius (`geom::Surface::representability_margins`,
`geom::Curve3::representability_margins`, the frame margins in
`crates/geom/src/convention.rs`). A frame mapped through an admitted
map carries the map's column defect `δ` into its `axis`/`u_ref`; every
mapped cylinder, sphere, torus or circle with `δ·r > ε` — at
δ = 4e-10, ε = 1e-9, any `r > 2.5 m` — then refuses at check 1
(`UnrepresentableSurfaceDatum { measure: Length, … }`), at a later door
that names the face, not the map. Found by reading, not measured: no
corpus body was seen doing it (ATREST-13's corpus instrument found no
frame beyond 0.15 ε of movement).

## The two ways it could close (superseded by the decision below)

- **Lever the acceptance**: meter the map's defects by the extent they
  will be applied over (F10's own suggestion — the model or session
  box), so an admitted map moves no mapped point by more than ε.
- **Re-orthonormalize at mint**: keep the acceptance, and have the
  transform re-derive each mapped frame orthonormal (normalize `axis`,
  project and normalize `u_ref`) so the stored frame is unit and
  orthogonal to rounding whatever the map's slack.

## Decided (fork 256, PR 6589)

Ev, 2026-11-10, on PR 6589, approving that a finished body moved by an
isometry keeps its verdict: today's admission "seems like a real
problem". The admitted map is re-minted at its admission door
(Gram–Schmidt, as `mate::solve::admitted` already does on its
undecided arm), and the rigidity margins are decided levered by the
session box, so an admitted map moves no in-box point beyond ε; a
matrix off orthogonal beyond that refuses `NotRigid` at its door,
naming the frame. This retires F10. Every `RigidMotion` is then exact
apart from rounding, which the carried verdict
(`a-rigid-motion-rewrites-and-recertifies-every-edge`) relies on, so
this row is built first, before the `moved` door.

**Revised by PR 6717's review, final state (fork 258, 2026-11-10).**
Ev ratified on PR 5068 that a frame exists at a scalar only by being
minted there, so no door lifts a frame up from `f64` and nothing
re-derives rigidity from twelve numbers at a box scalar. The rule:

- **Every motion is minted by construction at its scalar.** The mate
  solve's coset algebra carries `RigidMotion`: each pose's frame is
  `RigidMotion::aimed`, a coset's representative is
  `frame_of(to) ∘ frame_of(on)⁻¹` composed as motions, `ball_meets`
  composes the turns, the chart's least turn is `RigidMotion::least_turn`
  and its `S·O` the turn's own axes signed (`with_signed_axes`), and
  `settle`'s arms are the rotation mints, `spun_about`, `standing_at`
  and `translated`. `solve::admitted` is gone. The sphere re-cut mints
  its turns (`turned_between`).
- **`check_rigid` exists only at `f64`.** It admits a map whose seven
  margins are within their own rounding (`16u·t`, the admission floor's
  statement) with its bits kept, and refuses any other `NotRigid`,
  naming the margin. No band, no box lever, no re-mint. Its callers are
  the viewer's layout load and gesture (`Frame::admission_fault`) and a
  library caller's matrix.
- **STEP builds each placement by `build_axes`** as a
  `RigidMotion::from_axis_and_reference` mint, then `B ∘ A⁻¹` composed
  as motions. Where a frame's stated fields are no rigid frame within
  their rounding, the move is reported (`PlacementReMinted`, naming the
  transform, with the farthest a session-box point moved).

Done when:
- F10's `decide_flagged` site is gone, and the census count drops by
  one.
- `check_rigid` is `f64`-only and band-free. 2000 honest rotations are
  admitted at `f64`, and a margin beyond rounding refuses.
- The mate solve composes every placed motion; `solve::admitted`,
  `Composed` and `SOLVE_MAP_REWITNESS` are gone, and the M10 box-lane
  rows and the 1e-12 rows run green.
- STEP mints every placement by `build_axes`. A frame beyond rounding
  is reported, and it places a 2.5 m cylinder about 1 km out past
  check 1. A frame within rounding reports nothing.
- The sphere re-cut mints its turns (`turned_between`).
- No up door: `Frame::affine`/`Frame::motion` are `f64`-only. The one
  remaining up crossing, `RigidMotion::<f64>::computed_at`, is filed
  (`work/onehome/a-skin-stations-turn-crosses-up-from-f64-through-computed-at.md`).

## Fence

`crates/topo/src/transform.rs` (shell's ground per `work.py
territory`, shared with offset). Cross-reference F10.

- 2026-11-02 — moved from SHELF to SHELL by `git mv` when SHELF merged into SHELL, id and body unchanged (the track budget now counts only P0, P1 and unbanded rows, and SHELF had fallen far under it).
- 2026-11-10 — moved from SHELL to ONEHOME and raised P3 → P1 by Ev's PR 6589 comment; decided by fork 256; built first in the rigid-motion unit. (ONEHOME orchestrator)
- 2026-11-10 — the done-when's numbers corrected: "a column 1e-10 off unit passes" was the orchestrator's error. Levered by the session box, 1e-10 is about 200ε at the default ε, so it refuses at the kernel band, and passes only at a STEP file's band that admits it (`[import]`, adoption). A point map is decided at `f64` (an application of fork 256, not a fork). (ONEHOME orchestrator, via the rigid-motion lane)
- 2026-11-10 — Decided/Done-when rewritten to fork 258's final state: every motion minted by construction, `check_rigid` `f64`-only with no band, STEP by `build_axes`. (ONEHOME rigid-motion lane)
