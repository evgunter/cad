---
id: the-fit-rows-check-an-f64-chain-inside-an-enclosure-with-slack
kind: issue
title: The S5c fit rows check a member's f64 solve inside the enclosure with a 1e-12 relative slack, not structurally
status: open
opened: 2026-10-19
priority: P3
---


Found by reviewer A of C6 S5c (PR 5332), S4. Not taken in that PR; the INLANE orchestrator ruled it to its own item.

## Finding

Two S5c rows in `crates/geom/src/curves/fit.rs` (tests module) assert that each member's own `f64` solve lies inside the `Interval` enclosure:
- `over_a_wide_parameter_the_fit_encloses_every_members`;
- `a_wide_parameter_over_point_knots_takes_the_enclosure_row`.

The `f64` solve is a long chain of rounded operations. The enclosure is guaranteed to hold the real solution, not the member's rounded one. `crates/geom-core/src/interval.rs`'s module rule allows comparing an `f64` result against an enclosure only for a single exact operation.

The rows paper over the gap with a slack of `1e-12 · (1 + |x|)`. That is safe at these widths and condition numbers, since the enclosures are far wider than the rounding. On a tight case it would be fragile: a red that is no defect, or a slack that hides a real escape of the same size.

## Owed

A structural check. Candidates:
- solve each member at `Interval` over point inputs (the member's own enclosure of its real solution), and check that it is contained in the family's enclosure;
- or bound the member's `f64` solve's rounding (the banded LU's backward error, per de Boor–Pinkus) and widen by that bound, not by a constant.

Then drop the constant slack.
