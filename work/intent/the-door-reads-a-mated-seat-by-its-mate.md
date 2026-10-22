---
id: the-door-reads-a-mated-seat-by-its-mate
kind: unit
title: the coincidence door reads an at-rest row on two poses a mate equates as structural, by the first rung, with no Sym replay
status: closed
opened: 2026-10-22
closed: 2026-10-22
priority: P0
cost: M
branch: intent/door-reads-a-mated-seat
pr: 5587
---


## What

The ruling (PR 5555, `[coincidence.structural]`) is that a mate is the
first rung's statement. The two poses a placement's mate equates, read
with the sense the mate states, are one construction read twice,
wherever the placement evaluates. Today the door ignores this. It
re-derives a mated seat through the `Sym` replay
(`crates/editor-core/src/coincide/replay.rs` `prove_row`, behind
`coincide::REPLAYS_PLACED_COPIES`). The replay proves mate6's seat only
from 512 terms, and the tour bench's seats not at all.

## Owed

- **The match.** An at-rest row whose two cells are the two poses a
  mate in the copy's bundle equates is `Structural` at rung 1, with no
  symbolic work. The row's relation must agree with the mate's `Flip`
  parity. A target that a value moves off the mate's plane (a standoff)
  does not match. The product's at-rest rows already carry
  `AtRestBy::Mate` (`assembly.rs` `AtRestRows::decide`).
- **The replay** runs only for rows that rung 1 leaves. It resolves
  parts at `Sym` only when such a row exists, and the switch retires.
  A row frozen by the budget names the budget in its recourse, not "one
  construction".
- **Rows:**
  - mate6's seat, the tour bench's three seats, and a top seated on a
    pattern copy each give 0 `unproven-coincidence` findings;
  - the viewer rows `tree_badges` and `story_assembly::the_windmill_story`
    run at main's times;
  - a standoff seat stays `Numeric`.
- **Ride-alongs:** `the-h-seat-probe-measures-the-frames-d-retired`, and
  the owed list in `a-mate-placed-seat-is-unproven-at-the-doors-budget`
  (§ "Owed when … turns on").
- **Closes:** `a-mate-placed-seat-is-unproven-at-the-doors-budget` and
  `placed-carriers-compare-through-their-frames`, for mated rows. Rows no
  mate states go to the census unit.

## Closed (2026-10-22)

- **The match** is `coincide.rs` `stated_by_a_mate`, part of the first
  rung. A row's two cells are matched by construction
  (`construction`) against each mate side's face, read in the copy the
  census reads it in (`side_copies`: the body itself where it is a
  placement's output, else under the one `Place` copying it, or under
  each `Place` copying a `Part` pick of it). The mate's `on` copy must
  be its own placement's, and the bundle must pin (`mate::solve::pins`).
  The relation must agree with the parity of the mate's flips
  (`mate::planes_equated`). A standoff, or any other construction off a
  face's plane, matches nothing. An `OnCarrier` row that lays a cap's
  rim edge or vertex on a face also matches, when that cap and the face
  are the mate's two faces (`on_its_cap`): the extrude mints its rim on
  its cap's plane. This is what keeps the boolean rows at a mated seat
  (the union's corners) off the replay.
- **The replay** runs only for the rows the first rung leaves
  (`replay::rungs` takes the mask). It resolves parts only where a left
  row needs one: an at-rest face pair it reads off the copies, or a row
  of a node that stands on an instance. `REPLAYS_PLACED_COPIES` is gone.
- **The budget's recourse.** `sym::deferred::Undischarged::OverTheBudget`
  says when the failed Zero's early form froze a node on the budget, and
  the door answers `Recourse::Budget` there.
- **Rows** (`mate6_gather_mints`, `fix_pattern_mate_crossing`,
  `asm2a_instantiate`): mate6's seat is proven, the tour bench's three
  seats leave 0 findings (asserted in the tour), a top seated on a
  pattern copy is proven, and `asm2a`'s wall row is `Numeric` with the
  resolver. The red rows are a cell that is not the mate's face, a sense
  the row does not share with the mate (both ways), a standoff, a bundle
  that does not pin, and a vertex of the cap the mate does not seat. The
  viewer rows `tree_badges::a_union_over_a_chained_copy_points_at_the_refused_placement`
  and `story_assembly::the_windmill_story` run at main's times: every
  row they ask about is the first rung's.
- **The owed list** of `a-mate-placed-seat-is-unproven-at-the-doors-budget`
  is answered there, bullet by bullet.
- **Not done here:** the equivalence that constructions and mates
  generate past one mate's two faces (a flag mate's walls, a pivot's
  shank) is `flag-and-pivot-pose-kinds`. Contacts no mate states
  (unmated flush copies, recorded today as edge and vertex rows) are
  `the-census-records-a-planar-area-contact-as-one-face-pair`.

