---
id: lofted-section-params-read-nan-for-a-wide-parameter
kind: issue
title: Lofted::section_params is a pub Vec<f64> that reads NaN for a section parameter wider than a point at an enclosure scalar, a value hidden in a float sentinel
status: open
opened: 2026-10-16
priority: P4
cost: E
refs: [the-skin-at-interval-encloses-the-nominal-skin]
---


Raised in PR 5233's review (style lane, unilateral). Not taken there.

`sweep::Lofted::section_params` (`crates/sweep/src/loft.rs`) is a
`Vec<f64>` of each section's v-parameter, read off the value channel in
`LoftBuild::in_t`. At an enclosure scalar, a loft over a wide section
with `k ≤ v_degree + 1` has interior parameters wider than a point, and
those entries read NaN. The doc on the field says so. Still, a caller
reading the field gets a float sentinel, not a typed "no single point".

Repair shape: hold the parameters in `T` (`Vec<T>`), as
`LoftGeometry::section_params` already does, so an enclosure reads as
its bracket. That changes a public field's type, and so moves its
callers (`crates/sweep/tests/turning_orientation.rs`,
`loft_v_is_the_section_set.rs`, `cert5_offgrid_knot_rational.rs`).
