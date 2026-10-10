---
id: split-sector-rules-mint-no-invalid
kind: unit
title: split's sector rules and conic graze mint INVALID after a decided sign: route each through its door and keep its ending (mints C1-C5)
status: closed
opened: 2026-10-10
priority: P1
cost: M
parent: topo-mints-indeterminates-outside-the-funnel
refs: [split-escalations-end-a-poisoned-margin-in-the-plane-lever]
closed: 2026-10-10
pr: 4571
---

Unit 1 of `topo-mints-indeterminates-outside-the-funnel`'s re-scope
(2026-10-10, main `98a3817a1d`). The site ids are that section's.

## Sites

- **C1, `splitting/rules.rs:197` (`apply_rule_a`, `split_sector_extent`).**
  The face extent is a magnitude. Read it with `decide_magnitude_reported`.
  A decided Zero is a collapsed face: refuse it as `SliverSector` carrying
  the decided margin, not `INVALID`.
- **C2, `rules.rs:319` (`enters_material` reads `Tangent` after the
  parallelism gate).** One fact is decided twice. The gate's verdict
  should reach `enters_material` as typed input, so that `Tangent`
  cannot follow it (design item 7).
- **C3, `rules.rs:488` (`wall_graze`, `wall_bend_order2`, `Exits` or
  `Tangent` after rule (a) read "enters").** Same shape as C2. First
  confirm that both reads ask the same question of the same data.
- **C4, `rules.rs:500` (the sectors disagree).** Each wall's verdict is
  read over its own extent, and ledger F11 leaves this open. Decide
  whether it is a straddle (the split lever) or a disagreement of two
  `Decided` readings (a typed finding).
- **C5, `splitting/classify.rs:608` (`split_conic_graze_side` decided
  Zero, reachable at K ≤ 2).** Route it through `decide_nonzero`, whose
  rejection keeps the decided margin, tagged.

## Constraint: the endings

Today `SliverSector` renders one constant lever. CLEAVE's
`split-escalations-end-a-poisoned-margin-in-the-plane-lever` asks that
it carry how its reading stands
(`splitting::containment::Escalation::{Margin, Straddle, Decided}`) and
end through `RefusedArm`. That row waits on these mints, so build the two
together, or land this first and leave the endings to that row.

Pin each moved arm's ending before and after. An arm that stops saying
`INVALID` stops reaching any `is_invalid()` defect route.

## Sweep owed

Re-run the re-scope's two passes over `crates/topo/src/splitting/`
before landing.

## Built (branch `cleave/split-rules-mint-no-invalid`)

Measured on `origin/main` `4925e78683` first. Every site minted
`MarginDiag::INVALID`, and each refusal read "margin is invalid (NaN or
a refused enclosure)":

- **C4** reproduces through the public `split` at ε = 1e-9, 1e-6 and
  1e-12. The witness is a bar `20K` long with a half-round end and a
  plane through the smooth edge, tilted `{0.1, 0.3, 0.6}·ε` off the
  flat: 12 poses per row.
- **C1, C2, C3** reproduce at the unit level, on the cube corner with
  pure bands. C1 is unreachable through `classify_neighborhood`, because
  `sector_shape` decides the sector arm past Kε first and the face extent
  is at least that arm. C2, and C3's `Tangent` arm, need K < √2.
- **C5** reproduces at K = 2, at four ε values.

Per site:

- **C1** goes through `decide_magnitude_reported`. A decided Zero is an
  `unreachable!` (D9 row 4) that names the face, the vertex, the margin
  and the band: `sector_shape` has already decided the sector's shorter
  chord past the band, and the extent is at least that chord. The pin is
  a `should_panic`.
- **C2** asks of the same data (`n_face`, `n_SP`, the face extent) twice.
  `Tangent` re-answered the parallelism gate's own question.
  - `geom_brep::normal_alignment` now decides the gate and returns
    `NormalAlignment::{Oblique(sine), Parallel(ParallelNormal)}`.
  - `ParallelNormal` is a proof that only the gate can mint. It carries
    the direction, the arm, the band and the sine margin.
  - Its `sense()` reads through the nonzero gate and has no `Tangent`.
    `enters_material` and `sense()` share one core, `sense_margin`.
  - In `rules`, `ParallelFace` wraps the proof. A decided zero sense is
    the gate's tagged rejection: the face is too short to read it.
- **C3** has two arms.
  - `Tangent` is C2's double read and takes the same door. Rule (a)
    hands rule (b) a `WallReading::{Grazed, Clear(sine), InPlane}` per
    entry.
  - `Exits` does not read the same data: the wall's outward normal
    against the neighbours' sides, which other reads decided. It is
    `SectorReading::GrazeAgainstNeighbours { margin }`, carrying the
    sense margin, and not a contradiction.
  - Whether it can be reached at large K is on
    `graze-against-neighbours-is-unreached-on-a-gated-body-only-for-line-neighbours-at-k-over-four`.
    That row holds the derivation; it holds for line neighbours at K > 4
    and is not proven for conic neighbours, so there is no
    `debug_assert!`.
- **C4** is two decided readings that disagree: `split_sector_coplanar`'s
  sine, read over the grazed face's extent (zero) and over the other
  face's extent (past the band). D4 ¶1's parallelism arm is read per
  face. It is `SectorReading::SmoothEdgeDisagrees(TangencyReadings
  { tangent, clear, band })`. A `WallReading::InPlane` reaching
  `wall_graze` is an `unreachable!`, because rule (a) takes both entries
  of a coplanar plane's sector off the plane.
- **C5** goes through `decide_nonzero`. The rejection carries `D`. The
  offer at K < 2 is on
  `graze-side-tighten-offer-at-k-under-two-recurs-in-band`.

The endings, before → after:

- **`SliverSector`, every arm:** the split's lever → the split's lever.
  `SliverSector` read no `is_invalid()` route.
  - Now it ends through `LeverOnly` at `Reading::Build`.
  - A poisoned funnel margin adds `UNREADABLE_MARGIN_NOTE`. This is the
    `SliverSector` part of
    `split-escalations-end-a-poisoned-margin-in-the-plane-lever`.
- **C5 at the split** (`CrossingEscalated`): the lever → the lever.
- **C5 at the Boolean** (`EdgeOnPlane`, sized): lever plus the
  unreadable note → lever plus "tighten the tolerance below |D|/K".
- **C5 at the census** (`too_close`): the defect ending → the coincidence
  menu.

The two C5 moves are the decided margin read as what it is. Both are
pinned (`classify::tests::a_graze_side_decided_zero_keeps_its_margin`).

Census over `crates/topo/src/splitting/`, production code only:

- **Pass 1.** Before: 8 mints (`rules.rs` ×4, `classify.rs` ×1,
  `containment.rs` ×2, `order.rs` ×1), plus one false positive, the
  return type of `ConicRootFault::diag`. After: 3 (C6, C7, C8), plus the
  same false positive. Merging main (`cc3df241cc`) brings a fourth,
  `order.rs` `split_join_frame_axis` from `f56de98848`, filed on unit 2's
  row.
- **Pass 2** (`terminal_sliver:`): 0 before and 0 after.

## Closed (PR 4571, 2026-10-10)

C1–C5 are off `INVALID`:
- **C1:** the unreachable collapsed extent is an `unreachable!` with its values.
- **C2 and C3's Tangent arm:** the gate's parallel verdict reaches the sense as geom-brep's
  `ParallelNormal` proof, through one door (`normal_alignment`).
- **C3's Exits arm:** `GrazeAgainstNeighbours { margin }`.
- **C4:** `SmoothEdgeDisagrees`, carrying both readings.
- **C5:** `decide_nonzero` keeps `D`, tagged.

`SliverSector` carries its `SectorReading` and ends through `LeverOnly`. The K-dependence of
`GrazeAgainstNeighbours` and the C5 offer at K < 2 are P3 rows.
