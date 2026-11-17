---
id: the-impl-census-reds-on-two-arrivals-from-pr-7052
kind: issue
title: hand_written_impl_census reds on main: two Debug impls PR 7052 added
status: open
opened: 2026-11-17
priority: P1
cost: E
refs: [7052]
---



Main is red. PR 7052 (the periodic loop carrier) added two hand-written
`Debug` impls, and `crates/test-utils/tests/hand_written_impl_census.rs`
reds on both:

- `every_hand_written_walk_is_held_to_its_declaration`:
  `impl Debug for PeriodicNurbs3<T>`
  (`crates/geom/src/curves/periodic_nurbs.rs`) reads `self.knots`,
  `self.control`, `self.period` and `self.flipped` by name, so a field
  added to the struct falls outside the dump silently.
- `the_walk_still_sees_every_file_holding_one`: `IMPL_FILES_TODAY` does
  not list `crates/geom/src/curves/periodic_nurbs.rs` or
  `crates/geom-brep/src/certify.rs`. `certify.rs` holds
  `impl Debug for EdgeCurve<T>`, which is hand-written on purpose (it
  leaves the reading out of the three committed editor-core digests)
  and already destructures `Self`.

Owed: destructure `PeriodicNurbs3`'s walk, binding `base` and
`two_turns` to `_`, with the same output; add both files to
`IMPL_FILES_TODAY`.

Reproduce:

    cargo nextest run -p test-utils -E 'test(hand_written_impl_census)' --run-ignored all
