---
id: a-spline-collapsed-sides-undecided-vertex-refuses-under-a-variant-named-for-a-sphere
kind: issue
title: an undecided vertex on a spline chart's collapsed side refuses SphereTrimShape { SingularUndecided }, a variant named for a sphere
status: open
opened: 2026-11-23
priority: P4
cost: C
---


Filed by TESSLANE's `tesslane/collapsed-side` lane (PR 7517). The trimmed lane
decides a loop vertex on a spline chart's collapsed side at the run's band
(`crates/mesh/src/trimmed.rs`, `CollapsedSides::holds`, through
`chart_table::at_a_singular_point`). A vertex the band cannot place refuses
through the same `singular_vertices` the analytic arms use
(`crates/mesh/src/trimmed.rs` `singular_vertices`, the raise at ~2638:
`cause: SphereTrimCause::SingularUndecided`), as
`TessellateError::SphereTrimShape { face, edge, cause: SingularUndecided }`.

The refusal is right in kind: a classification of a vertex against a singular
point that the band cannot make. Its name is not. The variant
(`crates/mesh/src/types.rs` `TessellateError::SphereTrimShape`, ~671) says it
is "named for the chart it was first raised on" and lists "a sphere's poles, a
cone's apex, a spindle sheet's apexes"; its cause enum
(`crates/mesh/src/types.rs` `SphereTrimCause`, ~725) is a sphere's by name
too. Neither doc names a collapsed side. `chart_table.rs` (~555) cites the
cause in `AnalyticRow::is_singular`'s doc.

**What a rename touches** (measured with `grep -rn
"SphereTrimShape\|SphereTrimCause"` over `crates`, `demos` and `tools`):
`crates/mesh/src/` types.rs 16, trimmed.rs 27, chords.rs 5, chart_table.rs 1;
`crates/mesh/tests/` errors.rs 3, split_rulings.rs 4;
`crates/sweep/tests/` spheres_crossing_off_every_edge.rs 3,
tilted_sphere_pair.rs 2; `crates/geom-brep/src/pcurve_cache/speed_rows.rs` 1
(a doc citation); and the Python binding's public tag `"sphere_trim_shape"`
(`crates/pncad-py/src/tags.rs` ~2533, pinned in `crates/pncad-py/src/tests.rs`
~6119, and `crates/pncad-py/tests/test_binding_census.py`), which is a public
name and moves with the variant's. The cause words (`SphereTrimCause::name`,
`"singular_undecided"`) need not move.

Closing it: name the variant and its cause for what they cover (a trimmed
face's shape about a chart's singular set: poles, apexes, collapsed sides),
re-spell the sites above and the binding tag, and say the tag moved in the PR.
