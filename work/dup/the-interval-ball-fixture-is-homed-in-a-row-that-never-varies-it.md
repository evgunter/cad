---
id: the-interval-ball-fixture-is-homed-in-a-row-that-never-varies-it
kind: issue
title: m5_s12's ball(r) is parameterised only for another file, and sweep::test_support already hosts the revolve family
status: closed
opened: 2026-09-19
priority: P4
cost: E
closed: 2026-09-26
pr: 3284
---

## Finding

- **Where**: `crates/sweep/tests/m5_s12_curved_ops_interval.rs`,
  `certified::ball` (and `certified::recut_ball` beside it), against
  `crates/sweep/src/test_support.rs`'s revolve family.
- **Importance**: low-medium
- **Confidence**: sure about both facts below; the candidate home is a
  proposal and is not measured
- **Raised by**: the `dup/private-box-builders` lane, 2026-09-19, in
  its own fourth-pass review — this is residue of that PR's own repair,
  not something it found elsewhere

That PR folded a ball fixture that was written out verbatim in two
files. The fold is right and the duplication is gone. Where it landed
is the question this row holds, and two facts say it is the wrong home:

1. **The parameter exists only for the other file.** `ball`'s own
   rustdoc concedes it: *"It takes `r` because that suite's E1 row
   varies it; this file only ever wants 1."* A knob never turned in its
   own home is a fixture living where it does not belong.
2. **`m5_s12_curved_ops_interval::certified` is now a fixture home for
   a file that is not it.** `review_arceval_r1_probes` imports `ball`,
   `plate`, `recut_ball` and a constant from it. That is a shipped
   row's module serving as a vocabulary module for a reviewer probe,
   which is the shape this program exists to notice.

**A candidate home the fold did not weigh.** `sweep::test_support`
already hosts the revolve family — `revolved_about_y_at`, `dome`,
`ball_poled_z_at` — and a y-poled unit ball is the same kind of member
as those. It is generic in the `Decide` scalar, so the `Interval` lane
is served without a second copy, and both consumers already name that
module for `brick`.

**Why not in that PR.** The move is cross-crate (`crates/sweep/tests/`
to `crates/sweep/src/test_support.rs`), it has to answer the routing
question that module's own header sets — *"A fixture only earns a place
here once a consumer OUTSIDE this crate needs it or a second suite
inside it does"* — and the PR carrying it was four review passes deep.
It is a unit, not a fix.

The constant (`RECUT_MAPPED_ENCLOSURE_HI`) does **not** move with the
ball and must not: it is a measurement of one row's chain, and its home
is the row that owns the claim.


## Closed (2026-09-26, PR #3284)

**The premise, checked first at `0c1932667`.** RING-4 deleted the
kernel `interval` feature, not the scalar: `m5_s12_curved_ops_interval`
still builds `Body<Interval>` unconditionally, and `certified::ball(r)`
and `review_arceval_r1_probes`' import of it were as the row describes.

**The ball has one door**: `sweep::test_support::ball_poled_y(r, c,
tol)`, generic in the scalar, the shape of `ball_poled_z_at`'s.
`ball_poled_z_at` and it share one private first step (the lamina
revolved at the origin), so the lamina is written once in that module.

**Naming.** The door is generic but has no `_at` suffix, where
`ball_poled_z_at` does: it follows the extrusion family, whose generic
doors carry no suffix, and adds no `f64` twin, since
`work/helper/sweep-test-support-two-wrapper-conventions.md` counts
those `x` / `x_at` pairs as avoidable. `ball_poled_z` / `ball_poled_z_at`
keep their pair; that row is where the two conventions get reconciled.

`certified::ball` is gone; `recut_ball` is the door at
`(1.5, 1.5, 0.5)`; E1 calls the door at the origin. `plate`,
`recut_ball` and `RECUT_MAPPED_ENCLOSURE_HI` stay where the row said
they must.

**The class was bigger than the row.** A construction census (the
lamina's first vertex, every tracked file) found the same ball — the
door's body exactly, then translated — as a private `ball_at(r, c)` in
eight suites (`m5_pr12_battery`, `m5_pr12_die`, `m5_s12_curved_ops`,
`m5_s13_pips`, `m5_s13_pips_interval`, `m5_s13_review_probes`,
`m6_rider`, `m6_surgery`), and as a local fn that already bore the
door's name, `review_ring_clearance_r1_probes::ball_poled_y(r, c)`.
All folded. Each fold was measured `Debug`-equal to the old builder at
the radii and centres used (both scalars), including the zero
translation E1 now passes, with a 1e-12 offset as the negative
control. The two `m5_s13_pips` files keep, at their import, the one
sentence their `ball_at` docs carried about WHY the ball is y-poled.

**Plants** (filter: every suite that builds a y- or z-poled ball, 106
rows):

| plant | reds | per site |
| --- | --- | --- |
| the shared first step panics at its caller | 50 / 106 | `ball_poled_z_at` 12 rows; `m5_pr12_battery` 3; `m5_pr12_die` 3; `m5_s12_curved_ops` 1 + 1; `m5_s12_curved_ops_interval` (recut) 1; `m5_s13_pips` 4 + 1 + 1 + 1 + 1 + 1; `m5_s13_pips_interval` 1 + 1; `m5_s13_review_probes` 8 sites × 1; `m6_rider` 1 + 1 + 1; `m6_surgery` 6; `review_arceval` E1 1 |
| the door panics at the eleven calls masked above | 8 / 27 | `m5_s13_pips` ~:282, ~:309, ~:338; `m5_s13_review_probes` ~:257; `m6_rider` ~:47, ~:62, ~:79; `review_ring_clearance_r1` ~:362 |
| the same at `review_ring_clearance_r1` ~:372, ~:392, ~:397, one per run | 1 / 6 each | `r1_diag_cylinder_pierces` each time |
| radius ×1.01 | 25 / 106 | — |

The rest of the class — origin balls, a lamina slid along the axis,
copies in four other crates — is filed with its hit list as
`work/dup/the-y-poled-ball-is-still-spelled-per-suite.md`.

**Fix pass (2026-09-26, PR #3284).** `m5_pr12_die` and `m6_surgery` each still defined
the same `ball_poled(r, c, pole)` — the y-ball turned onto any pole,
byte-identical bodies in two files this PR had edited, invisible to
the lamina grep once the lamina was gone from them. It is now
`sweep::test_support::ball_poled(r, c, pole, tol)`, sharing the ball
doors' first step; measured `Debug`-equal to the old function at the
pips' radius for all six axis poles, and at `+z` equal to
`ball_poled_z`. Plants: a panic at the caller reds 9 / 10
(`m5_pr12_die` 3, `m6_surgery` 6; the loop's second call is the same
row); the pole NEGATED reds 0 / 10 — a ball flipped end for end is the
same point set, a symmetric plant — and the pole cycled onto a
perpendicular axis reds 9 / 10.
