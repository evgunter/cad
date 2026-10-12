---
id: section-cert-axis-pose-decides-tilt-and-offset-one-at-a-time
kind: issue
title: section_cert's axis_pose decides the axes' tilt and offset one at a time, not their sum
status: closed
opened: 2026-10-07
priority: P2
cost: M
branch: cleave/section-cert-axis-pose
closed: 2026-10-11
pr: 4657
---

## What

`topo::boolean::section_cert`'s `axis_pose`
(`crates/topo/src/boolean/section_cert.rs:745`) decides
`section_axes_tilt` (levered) and then `section_axes_offset`; both Zero
gives `Pose::Coaxial`, which serves the torus×cylinder and torus×torus
coaxial section certificates from the exact coaxial formula with no
allowance for the tilt or the offset (near 668 and 692–699).

Found by the sweep of the TANG unit that closed
`cylinder-axis-rows-decide-tilt-and-gap-one-at-a-time` (`work/tang/`),
and filed on this slate, whose ground it lands on.

## The shape of a fix

Decide the served verdict on one margin carrying both terms, as the
section classifiers' `decide_across` (`crates/geom-brep/src/intersect.rs`)
and `carrier_cyl_reach` (`crates/topo/src/boolean/carrier_eq.rs`) do:
the zero side on `|datum| + tilt·lever`, a definite side on the datum
shrunk toward zero by the tilt, the tilt row kept only to route.

## Built (branch cleave/section-cert-axis-pose)

**Measured first, on main, at ε = 1e-9, 1e-6 and 1e-12** (identical in
bands at all three). Two near-coaxial families, each tilted in the plane
of its offset so the terms add where the section lies: a wall at a fat
torus's inner equator and a second torus beside the donut's tube, against
the closed form (the wall's, or the second tube's, true stand-off from
the tube per meridian). At `0.9 + 0.9` bands (offset at the pivot, tilt
at its lever) main served the coaxial reading while the partner stood
`1.21`–`1.31` bands off it (`1.28`–`1.38` at `0.95 + 0.95`). At the
default K = 10 every verdict it served was right, because each served row
stands `K` bands past its flip: **no wrong answer, no invalid body; a
served claim off by up to 1.4 bands**. Under a narrow band (`K = 1.2`,
which `CAD_AMBIGUITY_K` admits) main served two essential parallels, or
nothing, where the closed form crosses the tube in some meridians and not
others: 6 wrong verdicts in the family, each under a fifth of a band
deep.

**Fixed at the owning decision.** `axis_pose` keeps the tilt row to
route, then reads the partner as its image under the turn about the
pivot onto the torus axis's direction (the chord between the axes times
the reach's farthest distance from the pivot, plus the arm's own turn
where it reads a point other than the pivot: the second torus's centre,
the cone's apex) and decides the offset across the reach: Coaxial on
`e + swing`, Parallel on `e − swing`, a straddle no reading. Every row an
arm serves on either reading is read across beside that swing
(`signs_across`, each with a `_floor` row): the coaxial wall, tube and
cone rows, the parallel wall's four rows (its touch included), the
parallel cylinder's apex row, the coaxial cones' apex gap; and the
cylinder pair's coincidence row beside its own tilt.

Rows: `section_cert_axis_pose_rows.rs` (split, inside, narrow K, and the
parallel floors), every ε.

### Fix pass (review of `0c9b997823`)

- The wall pair's coincidence row reads its swing as a turn about the
  pivot on the second axis: radial motion slides along the wall, so only
  a point's station (at most the reach's radius) moves it at first
  order, `sin·radius + r₂·sin²`. Wide walls clearly apart (r = 100 at
  20 and 40 bands, r = 1000 at 200 over a unit reach) serve their
  rulings again, where the radius-levered swing refused them.
- One story: the rows read through `geom_brep::intersect::decide_across`
  (made public; its far row now goes through the magnitude door), the
  chord is `carrier_eq`'s `line_tilt`, and the pivot and lever are
  `ExtentBall`'s `foot_on` and `lever_from` (`Reach::ball`); `signs` and
  `signs_across` share one loop.
- A near-coaxial straddle refuses R-tan naming the offset row in the
  torus arms (it was R-reach); the cone arms hand it to the ruling
  charts, which bridge nothing.
- Pins a halved swing, and each dropped turn, turn red: the straddles
  sit between the whole swing's floor and half of it, and
  `the_turn_of_a_point_read_off_the_pivot_counts` reads a tiny reach
  about the pivot where the turn of the torus centre, the cone's apex
  or the second apex is the residue past the band.

## Closed (PR 4657, 2026-10-11)

`axis_pose` decides the offset across the reach on one margin carrying
the tilt's swing and each arm's second-order turn, through geom-brep's
`decide_across` (now public, with its far row through the magnitude
door). The cylinder pair's swing is `sin·reach + r₂·sin²`. Across the
topo, sweep and geom-brep suites at all three ε, no verdict moved; the
bounds were checked against dense samples (worst ratio 0.98). Two
second-order terms are too small to pin (the cone turn's `e·tilt`, and
the wall pair's `r₂·sin²` below kilometre walls); their derivations live
at the sites.
