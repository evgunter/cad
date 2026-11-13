---
id: the-certificate-runs-out-of-pairs-on-eight-finished-tour-bodies
kind: issue
title: The injectivity check runs out of its 65,536 cell pairs on eight finished tour bodies (the die family, the Klein bulb, the snowman) in 2.5-6.3 s, so the at-rest gate would refuse them
status: closed
closed: 2026-11-13
opened: 2026-11-12
priority: P0
cost: M
---


## Finding

Measured by CLEAR's lane clear-gate (`the-at-rest-gate-certifies-embedding`,
its Measured section). Main at `348ec09727`; the body-level question
the gate would ask (`StrictlyPositive`, one body against itself,
`DEFAULT_MAX_CELL_DEPTH`, 65,536 pairs), at `f64`, on bodies the tour
builds and tier 3 passes. Each refuses `Budget(Pairs)`:

| body | faces | refuses after | deepest |
|---|---:|---:|---:|
| `diepips` | 48 | 3.6 s | 24 |
| `diecomposed` | 89 | 6.3 s | 21 |
| `diechamfer` | 68 | 3.3 s | 24 |
| `klein_bulb` | 12 | 2.5 s | 24 |
| `snowman_rolled` | 5 | 2.7 s | 32 |
| `snowman_union`, `snowman_bitten`, `snowman_lens` | 4 | 2.6–3.0 s | 32 |

The planar die with 21 square pips holds in 0.11 s (6,283 pairs), so
face count alone does not explain the cost.
Which face pairs hold the frontier was not read; the receipt's
`across` map (`BoundReport::across`, `clearance.rs`) has it.

## Why P0

`the-at-rest-gate-certifies-embedding` (P0) cannot land its gate while
these bodies refuse: the gate would refuse them at their doors.

## Fix shape

Read `across` for `snowman_union` (four faces) first: a sphere against
a sphere along their seam circle is the likely frontier.

## Diagnosis (CLEAR, lane clear-pairs, 2026-11-12)

### The snowman: a sphere's pole, between the two half-bands of one ball

`snowman_union` (main `c51fc90f58`, `f64`, the gate's question at
65,536 pairs, the frontier instrumented per split): 92,093 splits, and
91,850 of them are the two half-bands of ONE ball against each other
(`2v1`–`5v1` the bottom ball, 47,516; `3v3`–`4v3` the head, 44,334),
the departure arm splitting. The four pairs across the waist split
about 50 times each and certify. At depth 32 every frontier cell has
`v` in `[−π/2, −π/2 + π/256]` (the bottom ball's south pole) or the
mirror strip at the head's north pole, `u` cut into strips 0.025 wide.

Why they never decide: a cell holding the pole vertex is near BOTH
seam meridians and the pole, since every ball round the pole meets
both meridians there. `present_chart` read the pole as a loose shared
vertex that no corner of two clips holds, and answered `Local`, so
every pair split. The pole is a whole side of the sphere's chart, the
latitude line `v = ±π/2`, and no split ever moves a cell off it, so a
polar strip against every polar strip of the other half-band is a
pair forever: the count is quadratic in the strips, which double.
`plane_through` had a second gap there: it took a conic's own plane
only where ONE shared edge is near, and a polar cell is near two (the
two meridians, both arcs of one great circle).

The single ball (2 faces) holds at its root. The union holds once its
poles are read.

### The die family: the same pole, then a tangent rim

`diepips` and `diechamfer` are a sharp cube with 21 spherical dimples
(`diechamfer` also has chamfered edges). Each pip ball is charted with
its pole along the face it cuts, so each dimple is two half-bands
meeting at a pole, which is the snowman's frontier. Both hold once the
pole reads.

`diecomposed` is the pipped cube with its box edges blended and its 21
pip rims replaced by torus bands. Its poles read, and it still runs
out of pairs, at depth 29. Two thirds of its split attempts are the
departure arm on a die face (a plane) against a torus band, along the
rim circle where the band meets the plane tangentially. No separator
the arm tries has a first-order sign on both sides of that G1 edge.
Filed as
`the-certificate-has-no-separator-across-a-plane-tangent-to-a-torus-band-so-the-composed-die-runs-out-of-pairs`
(P0, M).

### `klein_bulb`: two parallel walls read by their boxes

A different cause, on the distance arm. The bulb's inner and outer
wall faces (a cone against a cone, a torus against a torus, and so
on) share nothing and stand 0.05 apart. 43,159 split attempts are the
two cones and 21,779 the two tori. `separated_by_support` reads an
analytic cell's support off its axis-aligned box (`Hull::Box`), so it
separates nothing the box test does not. Filed as
`the-support-test-reads-an-analytic-cell-by-its-box-so-the-klein-bulbs-walls-run-out-of-pairs`
(P0, M).

## Fix (lane clear-pairs)

In `topo::injective`:
- `present_chart` reads a sphere's pole as a collapsed side of its
  chart, as it reads a cone's apex: where the cell reaches the latitude
  `±π/2` and a local vertex is decided at that pole
  (`SELF_INTERSECTION_APEX`, the apex's site), the side `v = ±π/2`
  is a clip with `collapsed` set to that vertex. `zero_of` then holds
  the pole on a zero side: `Side` on its own, `Corner` beside a
  meridian.
- `plane_through` takes a conic's own plane wherever every near shared
  edge is a planar conic (the first one's), not only where one is
  near. A polar cell is near both seam meridians, two arcs of one great
  circle.

Both only choose a presentation and a separator. `sign_on` still
decides every zero side on the separator and every derivative across
it, so no pair certifies that the arm's argument does not cover. No
budget moved and no tolerance was added.

## Pins

- `a_two_ball_union_holds_with_its_poles_read`
  (`crates/sweep/tests/self_intersection_proves_injectivity.rs`): the
  snowman union holds at `f64` and at `Interval` in under 600 pairs
  (577 and 595), every touching face pair certified across.
- Mutants, each run: the pole arm switched off (`reaches` false), and
  `plane_through` back to one near conic. Each reads
  `Budget(Pairs)` at 65,536 pairs, and the pin goes red.

## Measured (lane clear-pairs, 2026-11-12)

The clear-gate harness, the tour's `ci-tour` profile (opt-level 1,
assertions on, so slower than the release figures above),
`RAYON_NUM_THREADS=1`, at the merge of main `a15896ac42`:

| body | faces | verdict | certificate | pairs | deepest |
|---|---:|---|---:|---:|---:|
| `diepips` | 48 | Holds | 0.65 s | 4,995 | 20 |
| `diechamfer` | 68 | Holds | 0.63 s | 5,658 | 17 |
| `diecomposed` | 89 | `Budget(Pairs)` | 11.2 s | 65,536 | 29 |
| `klein_bulb` | 12 | `Budget(Pairs)` | 2.9 s | 65,536 | 24 |
| `snowman_rolled` | 5 | Holds | 0.10 s | 1,412 | 18 |
| `snowman_union` | 4 | Holds | 0.07 s | 497 | 14 |
| `snowman_bitten` | 4 | Holds | 0.06 s | 504 | 19 |
| `snowman_lens` | 4 | Holds | 0.06 s | 496 | 28 |

Six of the eight certify. The two that remain have their own rows
(above), so this row stays open until they close.

The whole tour walk, the same harness before and after the change on
the same merge: 76 of 85 bodies hold (66 before). The ten that moved
are the six above and four that refused `Uncertified { Departure }`
at the depth floor, a plane against a sphere patch at its pole:
`diefillet` (4,133 pairs) and `heatsink5`, `7` and `9` (16,093–16,839
pairs). No other body's verdict or pair count changed.

## Closed (CLEAR orchestrator, 2026-11-13)

Seven of the eight bodies certify:
- the four snowman bodies, `diepips` and `diechamfer`: PR 6892, through the sphere pole read as a collapsed side;
- `klein_bulb`: PR 6902, through the support test reading an analytic cell by its mean-value form.

The eighth, `diecomposed`, is its own P0 row:
`the-certificate-has-no-separator-across-a-plane-tangent-to-a-torus-band-so-the-composed-die-runs-out-of-pairs`.
Nothing else is left on this row.
