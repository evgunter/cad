---
id: near-tangent-pierce-poses-reach-three-classification-invariants
kind: issue
title: Near-tangent pierce poses reach ClassificationInvariant at three sites (an ON sector pair on distinct carriers, a pierce germ not within one sector, a coplanar sector on a distinct plane): 345 runs
status: open
opened: 2026-10-08
priority: P1
cost: M
refs: [4335]
---

## What

Found by JOIN's near-tangent census measurement
(`near-tangent-boolean-results-ship-with-an-escalated-tier-3-census`, its
`## Measured`), on main `047d10d5`, release. These are outside that row's
question, and filed here because the sites are this program's.

Over the probe's 28 800 near-tangent runs at ε = 1e-9, 345 refuse
`ClassificationInvariant`, a kernel-defect refusal, at three sites:
- `boolean/recl.rs`, "geometrically-ON sector pair with
  definitely-distinct carriers": 162 runs (convex 45, w345 48, w60 69),
  at d = ±1e-5, ±1e-6 and ±1e-7. Witness: `convex nt e0 a4 d1e-5 pc U`.
- `boolean/vtxfac.rs`, "pierce germ direction not uniquely within its
  sector": 138 runs (Ltop 36, Lbot 30, Lmirror 24, asym 24, notch307 12,
  convex 6, w345 6), almost all at d = ±1e-9. Witness:
  `Ltop nt e0 a12 d1e-9 pc U`. `pierce-germ-direction-within-is-levered-at-the-sector-arm`
  (CONTACT) may be its cause.
- `boolean/vtxfac.rs`, "geometrically coplanar sector with
  definitely-distinct plane": 45 runs (convex 18, w345 9, w60 18), at
  ±1e-5 to ±1e-7. Witness: `convex nt e0 a3 d1e-5 pc U`.

At ε = 1e-12 and 1e-6 the same sites fire, at other tilts.

Repro: `NT_ONLY=convex NT_POSE="nt e0 a4" NT_D=1e-5 cargo run -p sweep
--release --example near_tangent_census_probe`.

## Owed

For each site, find which earlier decision disagrees with the later one,
and settle it: a typed refusal in band, or the two readings made one.

## Built (branch cleave/near-tangent-invariants)

Re-taken on main `19c43d1a4b`: all three sites reproduce exactly (345 at
1e-9; 108 at 1e-6, the germ site only; 552 at 1e-12). None was a
bug-only state; each was two readings of one question.

- **The ON sector pair and the coplanar sector** (`recl.rs`
  `require_same`, `vtxfac.rs`' coplanar lump): an unglued pair's extent
  ball was centred at the world origin, so the plane ladder read the
  offset at the corner's distance from the origin while the tilt was
  read at the arm. A sliver sector 2e-5 m long, where the cube's side face
  crosses the prism's at 1e-5 rad, read coplanar at its arm and
  `Distinct` at the origin. The ball is centred at the
  corner (and, in the sweep, the maximal-faces gate's at the shared
  edge). Those runs now refuse as the unglued near-coincidence
  (`Coincidence(Carriers)`, `carrier_unglued_coincidence`).
- **The pierce germ** (`vtxfac.rs` `pierce_germ_dir`): the germ's ray
  was re-read by sector membership levered at the arm, where a bound
  read On at its reach puts the section line a definite angle outside
  the sector, so neither ray read within. The ray is now the
  transition's own (`OutBound`).

Census after, per site, every ε row: **0 / 0 / 0** at 1e-9, 1e-6 and
1e-12, and 0 at 1e-6 with the tilts at 1e-4 where the carrier sites
fired on main (45 runs). Pin: `sweep` `near_tangent_pierce_poses` (28
poses the census named, two orders each; on main it fails 31 / 16 / 18
runs at the three rows), with `vtxfac`'s
`a_pierce_germ_is_the_ray_its_transition_names` and `refusal_routes`'
`the_maximal_faces_gate_reads_a_tilted_neighbour_alike_wherever_it_stands`.

Moved elsewhere in the census, filed: 12 runs at 1e-9 now reach
`JoinDesync` (`a-minted-edge-along-a-folded-graze-refuses-join-desync`);
the fourth site, untouched, at 1e-12
(`single-on-edge-crossing-without-a-flanking-record-at-1e-12`).
