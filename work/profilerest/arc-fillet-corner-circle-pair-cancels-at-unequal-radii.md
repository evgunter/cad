---
id: arc-fillet-corner-circle-pair-cancels-at-unequal-radii
kind: issue
title: An arc-leg fillet corner between two circular carriers is placed by differences of squares, so at radii orders of magnitude apart it sits off the smaller carrier or is lost to a negative radicand
status: open
opened: 2026-10-14
priority: P2
cost: M
design: true
---



Found by the sweep of `paths/circle-pair-shared-home` (the unit that
moved the circle-pair algebra to `geom_core::Circles`). #5032's sweep
missed it: its spelling is `d_sq + r1_sq - r2_sq` over squared radii.

## The shape

`crates/profile/src/path/arc_fillet.rs` `circle_circle`, the corner of
two circular fillet sides (`derive`, `SideCarrier::Circle` ×
`SideCarrier::Circle`), takes the squared radii and places the corner
at `k = (d² + r₁² − r₂²)/2d²` along `c₁ → c₂` and `h = √(r₁²/d² − k²)`
across it: the textbook radical form, with the same cancellation as
the rows `arc-arc-secant-arm-misses-crossings-at-unequal-radii` fixed.
Its three margins (`d`, `r₁ + r₂ − d`, `d − |r₁ − r₂|`) are a third
spelling of the circle-pair margins (noted in
`check-9-arm-5-is-a-second-copy-of-seg-planar-edge-pair-intersector`).

## Measured

The same IEEE arithmetic in Python, the corners' distance from both
circles against an 80-digit residual, in ulps of the largest radius or
centre coordinate:

| pair | larger first | smaller first |
|---|---|---|
| r = 80.44 × 0.0317 (the profile issue's second pair) | 133 ulps (2.4e-12) | 0.44 |
| r = 1e3 × 1e-3, d = 1e3 + 1e-4 | 2.2e5 ulps (4.9e-8, 49ε at 1e-9) | 0.03 |
| r = 62.76 × 1.28e-7 (the first pair) | NaN: `r₁²/d² − k²` rounds negative | 0.25 |

**What a misplaced corner does downstream** (2026-10-14, the fix pass
of PR 5124). A lens through the public door, `Open.arc_fillet_arc`
with both sides `Center`, counter-clockwise, the larger circle first:
centres on the x axis, both carriers through one f64 start point `p`,
so the true corner is `p` mirrored across the axis. Each case ran
twice, once with `circle_circle` as it is and once with its corner
taken from `geom_core::Circles2::crossings` (a throwaway patch), at
the default ε (1e-9):

| R × r, d − R | fillet r | corner off the small carrier | outcome |
|---|---|---|---|
| 1 × 1e-7, 1e-8 | 3e-8 | 4.6e-10 (0.46ε) | builds; the arrival tangent point sits 4.6e-10 off the small carrier (1.9e-17 from the exact corner); `Profile::validate` accepts it |
| 1 × 1e-6, 1e-7 | 1e-7 | 1.0e-10 | builds and validates; that tangent point 1.0e-10 off (3.7e-17) |
| 10 × 1e-3, 1e-4 | 1e-4 | 7.3e-12 | builds and validates; 7.3e-12 off (3.7e-16) |
| 1 × 1e-5, 1e-6 | 3e-6 | 5.3e-12 | builds and validates; 5.3e-12 off (6.7e-18) |
| 100 × 1e-4, 1e-5 | 1e-5, 3e-5 | 4.9e-9 (4.9ε) | refuses at the advance gate, "whether the corner lies ahead of its anchor on an arc side is undecided" (margin −4.9e-9). The exact corner passes that gate and refuses later, on the offset tangency or the stored arc's band |
| 1e3 × 1e-3, 1e-4 | 1e-4, 3e-4 | 4.9e-8 (49ε) | refuses on the offset tangency (`carrier_circles_internal`) with either corner, so the misplacement is not what decides |

So a misplacement below the band ships silently: the construction
carries the corner's error into the arrival tangent point, which lands
off its carrier by that error and validates. Here that was up to half
the band. A misplacement past the band was refused in every case
tried, sometimes by a gate reading the misplaced corner's own margin.
Hence P2: error propagation into emitted coordinates, not a wrong
answer past the tolerance.

## Where to look

The fix is blocked on a ratified rule. `arc_fillet.rs`'s module doc
states LB4, the squared-radius rule: every derived corner is computed
from squared radii and never round-trips `√(R²)²`; the `sqrt`s in the
file feed classification margins only, never an emitted coordinate;
the squared form is what lands the rocker eye's corner bit for bit
(`path_differential` `the_derived_circle_by_circle_corner_lands_on_the_authored_one`).
PATHS-DESIGN §2b keeps LB4, and `work/lib/log.md` records it ("the
squared-radius circle×circle form is ruled IN as the design").
`geom_core::Circles2::crossings` takes the radii themselves (its
factors are `r₁ ± r₂ ± d`), so reading it here breaks LB4. There are
two ways out:

- **A cancellation-free form over squared radii.** It keeps LB4, and
  it has to land the eye's corner bit for bit as the current form
  does. `h²`'s factored form needs `r₁` and `r₂` themselves, so this
  form is not yet known to exist.
- **A change to LB4**, so that a derived corner may read the radii
  (and `Circles2`, deciding the three margins from its
  `identity`/`external`/`internal`, with the concentric `d` gate its
  own). That is a change to a ratified decision and needs Ev, on an
  `[ev]` PR, with the eye row's verdict under the new form measured
  first.

The three margins (`d`, `r₁ + r₂ − d`, `d − |r₁ − r₂|`) are a third
spelling of the circle-pair margins whichever way is taken.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILEREST (`work/profilerest/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam (PROFILE, PROFILEREST, PROFILETAIL, PROFILEPROSE; `docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.

**It waits on a ratification, not on an item.** The fix needs Ev's ruling on LB4, the squared-radius rule that `PATHS-DESIGN` §2b keeps ("Where to look" above), on an `[ev]` PR. No item can carry that trigger, so the row stays `open` with `design: true` rather than `parked`.

2026-10-29 (SECT, `sect/circle-crossing-once`): `circle_circle`'s
reach and internal clearances now read `geom_core::Circles` (`reach()`,
`internal()`), bit for bit, so the margins are no longer a third
spelling. The corner is untouched and its doc says why (LB4); the fork
above is unchanged.
