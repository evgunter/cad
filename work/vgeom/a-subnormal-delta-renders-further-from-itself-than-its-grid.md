---
id: a-subnormal-delta-renders-further-from-itself-than-its-grid
kind: issue
title: display_budget: a subnormal δ (4.94e-321 mm) renders as 5e-321, further from it than the render's own grid; red on main
status: open
opened: 2026-11-25
priority: P2
cost: E
---


`crates/viewer/tests/display_budget.rs`,
`no_delta_renders_as_a_number_a_delta_cannot_be` (slow set), fails on
main (reproduced on 53116fdf43 by the PR 7764 review, and on the lane's
own main checkout): the `for end in [5.0e-324, MIN_POSITIVE, 1e3]` end of
its sweep panics with "δ 4.94e-321 mm renders as 5e-321, further from it
than the render's own grid". It reads only `DisplayTolerance::render_in`.
The cause is not established; a candidate is 76c79d89a5 (2026-11-23, "the
deciding figures are positional"). It reds the gate of every PR whose
diff edits `display_budget.rs`, and the nightly.

Assigned to `vgeom/field-seed`, which is on the δ field's render; it
says whether the render or the test's grid claim is wrong at a
subnormal.
