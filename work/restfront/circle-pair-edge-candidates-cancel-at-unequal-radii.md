---
id: circle-pair-edge-candidates-cancel-at-unequal-radii
kind: issue
title: The at-rest validator's circle x circle edge-pair candidates are placed by differences of squares, so at radii orders of magnitude apart they sit off the smaller circle and a crossing reads apart
status: closed
opened: 2026-10-13
priority: P0
cost: E
branch: paths/circle-pair-shared-home
closed: 2026-10-14
---


Found by the sweep of `paths/arc-arc-secant-arm-misses-crossings-at-unequal-radii`,
which fixed the same algebra in the profile validator, and reproduced
here by a review of that PR (#5032) and again by its fix pass. It is a
live false acceptance: arm 5 reads two crossing edges apart, so a face
loop that crosses itself passes the validator's check 9.

## Reproduction

`segments_meet`, called directly (a `validate::tests` row, `z = 0`,
axis `+z`, `u_ref` `+x`) with the profile issue's second pair as
`MeetSegment::Arc`s, each trim `[t0, t1]` counter-clockwise:

```rust
let big = seg((6.744_101_644_309_793, 79.705_363_072_029_3), 80.443_149_988_593_5,
    5.633_354_791_777_812 - 0.902_846_282_788_287_3, 5.633_354_791_777_812);
let small = seg((8.183_130_461_020_52, -0.750_331_493_417_868_4), 0.031_696_690_544_870_94,
    2.274_954_781_868_850_7, 2.274_954_781_868_850_7 + 4.970_712_391_069_452);
segments_meet(big, small, n, band)
```

The circles cross 2.0e-11 inside the large arc's end and 0.157 inside
the small one (a 50-digit check, in the profile issue). Measured on
#5032's fix-pass head:

| ε | `(big, small)` | `(small, big)` |
|---|---|---|
| 1e-9 | `Meet` | `Meet` |
| 1e-12 | **`Apart`** | `Meet` |

Only the order with the larger circle first is wrong, because the
cancellation is at `c₁`'s scale. The first profile pair (r = 62.76
against 1.28e-7) reads `Meet` both ways at both ε. The half-chord's
`.max(T::zero())` clamp also hides a radicand the rounding drove
negative: the candidates then collapse onto the foot with nothing
saying so.

## The shape

`crates/topo/src/validate.rs`, the circle × circle arm of the edge-pair
intersector (the one reading `ring_outer_meet_centres`), places its
candidates at `a = (d² + r₁² − r₂²)/2d` from `c₁` and the half-chord
`((r₁ − a)(r₁ + a)).max(0).sqrt()`. Factoring after `a` is rounded does
not help: `r₁ − a` inherits `a`'s rounding, about `u·r₁²/d`, so where
`r₁ ≫ r₂` the half-chord's square, at most `r₂²`, carries an error of
about `u·r₁²`. The candidates then sit off the smaller circle by up to
its whole radius; `meet_at` reads them off its trim, and a pair of arcs
that cross can read apart.

## Measured on the profile twin

`crates/profile/src/seg.rs` `arc_arc` had the same closed form. Against
a 60-digit oracle, over radius ratios 1 to 10¹² and centres up to 10³
from the origin, its crossings sat off the smaller carrier by up to
1.9e8 ulps of the largest of the radii and the centres' coordinates
(all of the smaller radius past a ratio of about 10⁸). Two arc pairs
from the issue read no contact while crossing, and a profile whose arcs
cross validated.

## Where to look

`seg.rs` `Circles` (`radical`, `crossings`) takes both from the clearances:
`r₁ − a = (r₁ + r₂ − d)(d − r₁ + r₂)/2d` and
`h² = (r₁ + r₂ − d)(d − r₁ + r₂)(d + r₁ − r₂)(d + r₁ + r₂)/4d²`, with
`r₁ − r₂` and `r₁ + r₂` formed once. Measured there at no more than 3.6
ulps of the same scale off either carrier across the whole range, and
Interval encloses (it is an identity). The tangent arm's foot
(`a = ±r₁`) is `decided-tangent-point-is-the-radical-foot`'s matter,
not this one's.

A shared `geom_core` home for that algebra is
`work/issues/circle-pair-radical-chord-has-no-shared-home.md`; fixing
this site before it lands mints another copy.

## Outcome

Fixed by PATHS on `paths/circle-pair-shared-home`, with the shared home
(`work/issues/circle-pair-radical-chord-has-no-shared-home.md`).
`validate.rs` `circle_pair` decides `ring_outer_circles_apart` and
`_nested` from `geom_core::Circles` (`external()`, and `-internal()`,
the same value as `|r₁ − r₂| − d` bit for bit), and arm 5's circle ×
circle candidates take the foot (`radical()`) and the half-chord from
the same value, so the half-chord's four factors are the margins just
decided. In the secant branch the half-chord is unclamped
(`half_chord()`): its square is a product of definite factors, so a
negative there cannot arise from the decided margins, and if it did
the poison reaches the trims as an invalid margin and the pair reads
`Unsure`, never `Apart`. At a tangency, or with the existence margin in
the band, the circles may stand apart or nested by up to the band,
the chord does not exist, and its ends are the foot (`max(0)`, stated
at the site).

Row, red first (commit `b12ad4cfd1`, red at 1e-12: the second pair,
larger arc first, `Apart` at `f64`):
`validate::tests::arcs_crossing_at_unequal_radii_read_meeting`. Both
pairs, both orders, at `f64` and `Interval`, now read `Meet` at 1e-6,
1e-9 and 1e-12 (before: `Apart` once at `f64` and `Unsure` twice at
`Interval`, at 1e-12; `Unsure` once at `Interval`, at 1e-9). It also
pins that a poisoned candidate reads `Unsure`.

