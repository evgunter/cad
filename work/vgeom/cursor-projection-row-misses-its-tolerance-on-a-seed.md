---
id: cursor-projection-row-misses-its-tolerance-on-a-seed
kind: issue
title: the cursor-projection fuzz row misses its f32 tolerance at seed 0xe944f229074a29d0, red on main
status: open
opened: 2026-11-19
priority: P4
cost: E
---


`viewer::all review_gui2_r1::cursor_projection_is_exactly_a_shift_and_scale_in_ndc`
(`crates/viewer/tests/review_gui2_r1.rs:185`) fails at
`CAD_FUZZ_SEED=0xe944f229074a29d0`, case 52:

```
case 52: shifted NDC (-5680.0728, 5.331442) vs independent (-5680.075, 5.329694)
```

(`crates/viewer/tests/review_gui2_r1.rs:231`). It is red on main at
`3bddaf8a6` with the same message, so it is not a carrier-frames change;
it was first seen on PR 7131's CI (test (app)).

The `y` reading misses by 1.75e-3 against a tolerance of
`max(1e-3, 1e-4·|want|)` = 1e-3 (`review_gui2_r1.rs:229`). Both sides are
`f32`, and `want_y` is scaled by `size[1]` (drawn up to 4000), while the
tolerance's floor is absolute. Not diagnosed further: whether the fix is
the tolerance or the shift's arithmetic is this item's question.

Reproduce: `CAD_FUZZ_SEED=0xe944f229074a29d0 cargo nextest run -p viewer -E 'test(cursor_projection_is_exactly_a_shift_and_scale_in_ndc)'`.
