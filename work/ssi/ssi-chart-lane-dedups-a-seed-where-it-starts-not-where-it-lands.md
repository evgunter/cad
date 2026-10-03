---
id: ssi-chart-lane-dedups-a-seed-where-it-starts-not-where-it-lands
kind: issue
title: ssi: plane × NURBS dedups a seed against the tubes where it starts, the ℝ³ lane where Newton lands it
status: closed
opened: 2026-10-02
priority: P1
cost: E
closed: 2026-10-03
---


Found in the §5 sweep of `ssi-a-seed-refined-off-the-chart-is-marched`.
Not measured on any fixture.

## What

The two doors deduplicate seeds against the tubes of the branches
already found, and they do it differently.

- `cylinder_sphere_ssi` (`crates/geom-brep/src/ssi.rs` ~:1687) refines
  the seed first and tests where Newton *lands* it. Its comment
  explains why: a cell centre outside every tube can land squarely
  inside one, re-march a branch already found, and return a duplicate
  `SsiBranch`. That is two carriers for one component, and an
  exhaustiveness receipt that counts one tube twice.
- `plane_nurbs_ssi` (~:1933) tests the raw seed `(u, v)` against the
  chart tubes and marches without refining. It is open to the
  duplicate that the ℝ³ lane closed.

## Fix shape

Refine first, then test the landed `(u, v)` (`state[2..4]`) against the
chart tubes, as the ℝ³ lane does. Pin it with a wall whose seeder hands
a seed outside a found branch's chart tube that Newton lands inside
it. Building that fixture is most of the work.

## Closed (2026-10-03)

**The rule landed with PR 3862**, before this row was measured. Its
boundary pass rewrote `plane_nurbs_ssi`'s seed loop: it refines each
seed first and tests the landed `(u, v)` against the chart tubes and
the boundary regions. A march refusal held from a seed is keyed at
the same landed point, and it is dropped when a final tube holds that
point. The ℝ³ lane's rule differs only in the shape it tests: there, a
box of half-width ε round the landing point must lie inside a tube;
here, the landing point itself must.

**Measured on main.** On the SSI suite (`--test all` filtered to
`ssi|m5_pr7|exhaust|pxn|m7_8`), the two rules disagree only on the
region rows, `a_walls_weights_move_no_answer`,
`a_rational_walls_corner_and_side_in_band_answer_a_region_only_where_coincident`
and, at ε 1e-6, `a_rational_walls_plane_three_eps_off_its_edge_meets_the_certificate_limit`.
No outcome differs:

| | default ε | 1e-6 | 1e-12 |
|---|---|---|---|
| seeds outside, landed inside, which the start rule marches | 637 | 712 | 637 |
| … `SeedOffDomain`, no branch | 601 | 604 | 601 |
| … an open trace, discarded | 36 | 108 | 36 |
| seeds inside, landed outside, which the start rule skips | 3391 | 3390 | 3391 |
| … `SeedOffDomain` under the landing rule | all | all | all |

So on the suite, the start rule never marched a duplicate and never
skipped a branch. It only spent marches. A closed loop is what turns a
wasted march into a duplicate. On every chart-lane loop in the suite
(the dome's level cut, for one), each seed's start and landing agree.

**A loop round the dome's apex separates the rules.** The plane
`y = −d/4 + δ` cuts `W(d)` in a loop of radius about `√δ`. The cap
inside the loop lies within δ of the plane, so it is seeded. Those
cell centres lie outside the loop's chart tube, and Newton carries
them onto the loop. Branches returned at band 1e-6, `d = 1`:

| δ | extent 1, start rule | extent 1, landing rule | extent 0.5, start rule | extent 0.5, landing rule |
|---|---|---|---|---|
| 0.02, 0.01 | 1 | 1 | 1 | 1 |
| 0.005 | 499 | 1 | 2 | 1 |
| 0.002 | 696 | 1 | 161 | 1 |
| 0.001 | 1333 | 1 | not run | 1 |

`d = 3` at δ ≥ 0.001 gives one branch under the landing rule. The
start rule's counts there were not taken.

`m5_pr7_ssi.rs`'s
`a_loop_whose_seeds_newton_carries_into_its_tube_is_found_once` pins
`d = 1`, `δ = 0.005`, extent 1. Mutation check: with the test put
back on the raw seed, it fails with 499 branches.
