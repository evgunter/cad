---
id: the-plane-poses-offset-loses-digits-at-far-frames-so-the-ring-path-cannot-drop-its-charge
kind: issue
title: The plane pose's offset n·(c − o) loses digits at far frames, so the ring path cannot read the crossing reading uncharged (U3b's ring-path move waits on it)
status: closed
closed: 2026-11-23
opened: 2026-11-21
branch: germ/plane-pose-far-frame
pr: 7411
---


Filed by GERM's `germ/crossing-consumers` lane (U3b of
`docs/GERM-CROSSING-U3-SPEC.md`).

## Finding

U3b moves `ring_path::plane_roots` (`crates/topo/src/ring_path.rs`) onto
`curve_face::cross` and drops its `slack_of` charge
(`[simple.conditioned]`). The spec makes that move wait, if it reds
ring-path rows, on
`ring-path-readings-decide-past-their-resolution-at-far-frames`, which
"re-poses the frames". That row is **closed** (PR 5012), and its fix was
the very charge U3b drops, so nothing re-poses the frames.

The move is on the branch `germ/crossing-consumers-ring-path-parked`
(one commit on `4303b022e8`). It reds
`ring_path::tests::a_graze_against_a_far_centred_sections_plane_is_read_wrong_nowhere`,
read uncharged through the pose (a unit circle grazing the plane of a
section conic stored `far` along it, ε 1e-12, gap within twice the
escalation, wrong where a decided reading disagrees with the exact gap):

| far | wrong of 2000 |
|---|---|
| 1e4 | 22 (26 on another seed) |
| 1e5 | 135 |
| 1e6 | 376 |

Every other ring-path row passes (49 run). One row
(`graze_rows::a_graze_within_the_gaps_band_escalates_and_the_next_path_decides`)
only renames its predicate to `curve_face_pose_belly_graze`.

## Why

The plane arm's pose (`curve_face::pose`, `plane_conic`, through
`curve_pair::pose::height`) reads the conic's height `d₀ = n·(c − o)`
with `o` the plane's stored origin. With `o` a megametre from the arc,
the difference `c − o` rounds at `ulp(|o|)`, which at 1e4 already
passes the zero band at ε 1e-12. The row's oracle computes the same
gap exactly (`two_sum`, `two_product`), so the loss is the formula's,
not the stored data's: a re-pose of the offset (compensated, or read
about a point of the plane near the arc) is the `[simple.conditioned]`
fix, and it is the pose's (`curve_face_pose_belly_graze`,
`_plane_parallel`, `_plane_offset`, `_graze_side`), not the ring
path's.

## Settles when

The far-centre row passes uncharged through the pose, and the parked
ring-path commit lands with U3b's retirements
(`split_ring_path_meets_plane`, the hand-rolled harmonic, the charge
on these readings).

## Closed

2026-11-23, by the GERM orchestrator. PR 7411 (`germ/plane-pose-far-frame`, merge `3547fe5764`).
- The plane arm's offset is re-posed near its geometry with no rounding charge. It is checked against an exact `Expansion` oracle (`geom_core::real::tests::a_far_origins_offset_is_read_to_its_own_digits`; worst error 0.994 u-relative over 20k poses per far, 1e4–1e12).
- `ring_path::plane_roots` reads `curve_face::cross` on the arc's own stored frame, with the `slack` charge dropped. `a_graze_against_a_far_centred_sections_plane_is_read_wrong_nowhere` is green uncharged at far 1e4/1e5/1e6.
- Refusals are classified exhaustively and faults fail loud.
- The review's class findings are filed:
  - `the-plane-arms-harmonic-amplitude-rounds-at-a-far-centred-conics-reach` (P1);
  - `plane-line-reads-its-offset-at-a-point-formed-far-along-the-line`;
  - `the-ring-paths-plane-readings-beside-plane-roots-still-decide-on-charged-heights`.
