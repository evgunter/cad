---
id: near-tangent-pierce-poses-reach-three-classification-invariants
kind: issue
title: Near-tangent pierce poses reach ClassificationInvariant at three sites (an ON sector pair on distinct carriers, a pierce germ not within one sector, a coplanar sector on a distinct plane): 345 runs
status: closed
opened: 2026-10-08
priority: P1
cost: M
refs: [4335]
branch: cleave/near-tangent-invariants
closed: 2026-10-11
pr: 4581
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

## Another witness (2026-10-09, TANG)

The second site, from a pierce whose partner is tipped off the face:
`crates/topo/tests/a_vertex_read_by_two_sector_passes.rs`,
`a_partner_tipped_off_the_face_builds_sound_or_refuses_typed_at_every_pose`.
The plate united with a pyramid lying on its top, its corner at 60°
lifted 100ε off the top (`meeting::lying`), against "inside the lying pyramid"
(`meeting::along_lying_ray`), whose edge along the top at 60° runs 60ε under the lying
pyramid's lifted edge. Every op, both orders, every pose, at ε = 1e-9
and 1e-12: `ClassificationInvariant { what: "pierce germ direction not
uniquely within its sector" }`. At 1e-6 the same cells escalate
`bool_sector_within` in band instead. Identical on main (db51132ff): the partner is
strictly above the top there, so `vtxfac::partner_side` reads it as
before. The row allows it by this item's name for that scene alone.

## Owed

For each site, find which earlier decision disagrees with the later one,
and settle it: a typed refusal in band, or the two readings made one.

## Built (branch cleave/near-tangent-invariants)

Re-taken on main `19c43d1a4b`: all three sites reproduce exactly (345 at
1e-9; 108 at 1e-6, the germ site only; 552 at 1e-12). None was a
bug-only state. In each, the same question was read twice, two ways.

- **The ON sector pair and the coplanar sector** (`recl.rs`
  `require_same`, `vtxfac.rs`' coplanar lump). An unglued pair's extent
  ball was centred at the world origin, so the plane ladder read the
  offset at the corner's distance from the origin while the tilt was
  read at the arm. At the witness, a sliver sector 2e-5 m long, where
  the cube's side face crosses the prism's at 1e-5 rad, read coplanar
  at its arm and `Distinct` at the origin. The ball is now centred at
  the corner, and it reaches over the region the coplanarity was read
  on:
  - `recl`, an all-On record: `pair_search`'s arm.
  - `recl`, a flanker or wedge representative: that sector's reach,
    where a thin sector tilted about a bound refuses on the bounds'
    margin.
  - `vtxfac`: the sector's reach, where its unglued tilt is now read too.

  These runs refuse as the unglued near-coincidence, or as the tilt
  undecided on its bounds.
- **The pierce germ** (`vtxfac.rs` `pierce_germ_dir`). The germ's ray
  was re-read by sector membership levered at the arm, and where a bound
  reads On at its reach, the section line lies a definite angle outside
  the sector, so neither ray read within. The ray is now the
  transition's own (`OutBound`). Membership stays as a D9 row 5
  `debug_assert!`.

Census after the fix, per site, every ε row: **0 / 0 / 0** at 1e-9, 1e-6
and 1e-12, plus 0 at 1e-6 with tilts of ±1e-4, where the carrier sites
fire 45 times on main. With the shared fixture restored bit for bit,
main's census reproduces byte-identically.

Pins:
- `sweep` `near_tangent_pierce_poses`: every op in both orders over 28
  census poses, plus the four runs below. Each built body is held to its
  closed form within ε·area and to tier 3′ (in-band escalations aside).
  On main it fails at every row.
- The same suite's germ witnesses: five poses that build all six runs at
  every row. They go red under a germ sign flipped for half the
  transitions.
- `vtxfac`'s germ tests: `a_pierce_germ_is_the_ray_its_transition_names`
  and `a_runs_germs_are_its_transition_sectors_rays`, the latter over a
  real orbit.
- `refusal_routes`' frame test for the maximal-faces gate.

Four runs at 1e-12 trade the germ invariant for bodies FUSE's class
ships (pinned, `SHIPS_A_FINDING`). Moved elsewhere in the census, and
filed:
- 12 runs at 1e-9 now reach `JoinDesync`
  (`a-minted-edge-along-a-folded-graze-refuses-join-desync`).
- The fourth site, untouched, at 1e-12
  (`single-on-edge-crossing-without-a-flanking-record-at-1e-12`).
- The two germ readers' divergence
  (`two-germ-readers-sign-one-line-two-ways`).

## Closed (PR 4581, 2026-10-11)

All three sites census 0 at every ε row:
- **recl and vtxfac coplanar:** the extent balls are centred at the corner and enclose what their
  decision speaks for, so a verdict no longer depends on where the body sits.
- **vtxfac germ:** signed by its transition's own codes, with a D9 row 5 check that membership
  agrees.
- **reduce `gate_maximal_faces`:** reads at the shared edge, and its ball holds the edge.

At 1e-12, four runs now ship bodies with tier-3′ contact findings instead of refusing
`ClassificationInvariant`. The orchestrator accepted that trade explicitly: it is FUSE's class and
the door runs no census. The runs are pinned at their exact findings (`SHIPS_A_FINDING`).
Follow-ups filed:
- `a-minted-edge-along-a-folded-graze-refuses-join-desync` (P1);
- `single-on-edge-crossing-without-a-flanking-record-at-1e-12` (P1);
- `two-germ-readers-sign-one-line-two-ways` (P3).
