---
id: no-row-certifies-a-shared-row-at-interval
kind: issue
title: No row certifies a shared row at T = Interval
status: open
opened: 2026-10-29
priority: P3
---


Found by the PR 5959 re-review 2 (its N3).

## What

Every row of `crates/geom-brep/tests/a_shared_row_is_certified_per_wall.rs`
certifies at `T = f64` (`Pair::certify`), and so do the sweep row and the
tour. The shared row's certificate (`certify.rs`, `Resolved::SharedRow`)
is generic over the lane, but nothing runs it at `T = Interval`, where its
brackets have width.

Two pieces of it only matter there:

- The row cover's chord (`certify.rs`, `run_checks`' `read`) zeroes a
  component whose two end brackets overlap, rather than taking the
  midpoints' difference. On the `f64` lane every bracket is a point, so
  the two readings agree, and the mutation that drops the guard turns no
  row red.
- The `NaN` poisoning in that `read`'s hull and in `Box4::hull`
  (`ssi/four.rs`) reads no `NaN` on any row.

## What would close it

The shared-row rows (at least the certifying, turning and escalating
ones) run at `T = Interval` too, through the same `IntersectionDraft`
door, with the chord guard's mutant red on one of them.

## Also noted: the knot-reader ratchet

`NurbsBoxes::deriv_box_closed` and `first_order_continuous_in`
(`ssi/enclose.rs`) were first written through
`Span::knots_f64_interim` and `knots_u_f64_interim`, which raised the
`knots-f64-interim` gate's count by 3. They now read the cell knot vector
`cell_knots` already holds, and `Knots::crease_values`, and the count is at
its baseline. The first is the same interim `f64` vector read one call
further from the accessor the gate counts, so the ratchet held because
the read moved, not because it went away. The gate counts accessor calls,
not readers of what they return.
