---
id: sibling-batteries-judge-sound-weaker-than-differential-outcome
kind: issue
title: Four differential batteries judge SOUND by their own outcome, weaker than common::differential::outcome
status: closed
opened: 2026-10-08
priority: P4
cost: E
branch: flush/e-tail
pr: 4714
closed: 2026-10-11
---


Found by the sweep of `join1-delta-probes-keep-their-own-outcome`
(branch `join/battery-hygiene`), which routed `join1_delta_probes.rs`
through `common::differential::outcome`. The pattern was a test-side
`outcome` that prints a `SOUND` verdict of its own. Four siblings judge
`SOUND` more weakly than the shared judge, which requires tiers 2 and
3′, the at-rest certificate, a legal operand and the volume to 1e-7.

## What

- `crates/sweep/tests/join1_delta2_harness.rs` `outcome0` / `outcome`:
  a copy of the judge `join1_delta_probes` dropped. Its `SOUND` needs
  neither the certificate nor a legal operand: the `OPERAND=` column is
  printed beside the verdict, not in it. The volume is held to 1e-6
  against a 4 096-chord oracle. `d2_arc_battery` and `d2_brick_battery`
  re-run `join1_delta_probes`' two batteries with that judge.
- `crates/sweep/tests/join3_r2_r1copy.rs` `outcome`: the operand
  (`legal` / `NONOPERAND`) is printed and not in `SOUND`.
- `crates/sweep/tests/join3_r2_probes.rs` `outcome`: the same.
- `crates/sweep/tests/pocket_ring_steep_ellipse.rs`, the battery's
  inline line: no operand, and the volume held to 2e-5 against an
  approximate oracle (`want~`).

Not in the class: `join3_review_r1.rs`'s judge requires all five
checks and prints `NONOP` for a sound body that is no operand.

## The shape of a fix

As in `join1_delta_probes`: route each battery through
`common::differential::outcome`, bring its oracle to 1e-7 (closed
forms, or Richardson-extrapolated chord areas as `join1_delta_probes`
`chord_area` does), delete the local judge, and re-baseline main
against head. Where a battery is a copy of another, consider deleting
the copy instead.

## Fix (branch `flush/e-tail`)

- **`join1_delta2_harness.rs`: deleted.** Its `d2_arc_battery` and
  `d2_brick_battery` were `join1_delta_probes`' two batteries under a
  weaker judge, `d2_r1_hex` was `join1_r1_probes::join1_r1_battery`
  with `J1_SHAPE=hex`, and `d2_r2_rand` was the R2 generator that
  `join3_r2_probes::j3r2_rand` also runs (both orders there).
  `R2_SEED=16 R2_N=1000` replays its default sample through `j3r2_rand`.
- **`join3_r2_r1copy.rs`: deleted.** Its own header called it
  `join1_r1_probes` verbatim but for `outcome`, and `join1_r1_probes`
  has been on the shared judge since.
- **`join3_r2_probes.rs`**: its batteries judge through
  `common::differential::judged_with_seams`. A pose with no closed form
  (an off-centre pocket, a tilted blind cutter) reads `UNJUDGED`, no
  longer `SOUND`, and is still checked by inclusion–exclusion.
- **`pocket_ring_steep_ellipse.rs`**: `steep_quad_prism_battery`
  prints the shared `outcome`, against a closed-form oracle
  (`prism_in_cylinder`: the prism clipped by the cylinder's height is
  one convex polytope, whose faces' heights integrate over the disc in
  closed form, `differential::disc_clip_moments`), cross-checked
  against a quadrature (`the_prism_in_cylinder_oracle_agrees_with_a_quadrature`).
  The cylinder's and the prism's volumes are closed forms too. The
  gating row `steep_ellipse_poses_build_sound` takes the same oracle and
  holds the volume to 1e-7 plus the measured pad, instead of 2e-5.
- **The shared judge** (`common/differential.rs`): `judged` is
  `outcome` with an optional closed form and the measured volume, and
  `judged_with_seams` appends `join1_delta_probes`' seam columns (that
  file's `seams` moved here). The volume test is now 1e-7 beyond the
  body's own certified `volume_pad`, as the gating rows already read it.
- **`join1_r2_rand.rs`** (found by the sweep's second pass, below): the
  gated random battery asserted tiers 2 and 3′ and the volume only. It
  now asserts the shared verdict, and keeps the volume to its exact
  oracle's 1e-9.

## Measured, main against head

Release, ε 1e-9; main at `121741a2b`, head on `flush/e-tail`.

| battery | runs | main | head |
|---|---|---|---|
| `steep_quad_prism_battery` | 960 | 849 SOUND, 111 ERR | 849 BAD, 111 ERR |
| `j3r2_tilted_battery` | 2 160 | 2 153 SOUND, 7 ERR | 2 153 BAD, 7 ERR |
| `j3r2_pocket_battery` | 10 800 | 10 064 SOUND, 722 EMPTY ok, 14 BAD | 5 040 SOUND, 5 024 UNJUDGED, 360 EMPTY ok, 362 EMPTY unjudged, 14 BAD |
| `j3r2_rand` | 2 400 | 2 053 SOUND, 347 EMPTY ok | the same |
| `j3r2_groove_battery`, `j3r2_snowman_battery` | 300 | all SOUND | the same |

Every new BAD on the first two passes tiers 2 and 3′ and the
certificate and fails only the far-brick union,
`Containment(VolumeUncertified)` on the ellipse-trimmed walls: RESTREAD's
`at-infinity-probe-measures-in-closed-form-only`, where the numbers are
added as evidence. The 14 pocket BADs are BAD on main too: two-solid
results failing tier 3′ with `CensusUndecidable`, added as evidence to
RESTREAD's `census-cross-solid-curved-pairs-undecidable-on-shell-results`.
On the steep quad battery the 849 bodies' volumes sit within 2.3e-7 of
the closed form; the nine past 1e-7 are all ∪ or cylinder ∖ prism, which
keep the cylinder's trimmed wall, while every ∩ is within 1e-7.

## Closed (2026-10-11, PR 4714)

Fixed in the E-tail batch, PR 4714 (single FULL review, APPROVE-WITH-FIXES; every fix taken). The PR body has a section for this row.
