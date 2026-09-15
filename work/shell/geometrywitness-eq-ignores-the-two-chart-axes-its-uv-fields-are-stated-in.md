---
id: geometrywitness-eq-ignores-the-two-chart-axes-its-uv-fields-are-stated-in
kind: issue
title: GeometryWitness's hand-listed PartialEq compares uv pairs across charts: a_chart_axis and b_chart_axis are outside equality
status: open
opened: 2026-09-15
---


Found by CENSUS-DEBUG (`work/census/hand-listed-debug-censuses-in-geom-core-geom-and-topo.md`)
while sweeping hand-listed `Debug`/`PartialEq` walks. Filed rather than
fixed: the unit's fence is the types whose `Debug` it touched, and
`GeometryWitness` carries a derived `Debug` and a hand-written `eq`.

## The defect

`GeometryWitness` (`crates/editor-core/src/clearance.rs`, `:508`)
declares **nine** fields: `a`, `a_uv`, `a_chart_axis`, `a_point`, `b`,
`b_uv`, `b_chart_axis`, `b_point`, `distance`.

`impl PartialEq for GeometryWitness` (same file, `:546`) compares
**seven** of them by hand. `a_chart_axis` and `b_chart_axis` are not
read, so two witnesses are equal whenever their seven agree.

**That is not merely a missing field — it makes the comparison read the
wrong thing.** `a_uv`'s own doc says its parameters are *"IN THE CHART
`a_chart_axis` names — which for a planar face is the engine's own
re-chart, not the stored one"*, and `a_chart_axis` is *"Which world axis
the planar re-chart crossed the normal with, so a consumer can rebuild
the same chart ([`chart_frame`]) and evaluate at `a_uv`"*. So `a_uv` has
no meaning except relative to `a_chart_axis`: two witnesses carrying the
same `(u, v)` numbers in **different** charts name different points of
the same surface, and this `eq` calls them the same witness.

`a_point` and `b_point` do agree in that case, which is why nothing has
caught it — the numeric coordinates are chart-free. What differs is the
carrier parameters, which are the fields a consumer round-trips through
`chart_frame`.

## Two ways to be right, and the choice is shell's

- **Compare the axes**, so a witness is equal only to one stating the
  same parameters in the same chart. The wider answer, and it changes
  what `==` says about witnesses that differ only in re-chart.
- **Compare neither the axes nor the uv pairs**, on the argument that
  the points are the witness and the parameters are a convenience. The
  narrower answer, and it changes what `==` says about a witness whose
  points agree to the bit and whose parameters do not.

Either way the impl destructures `Self` exhaustively, per
`crates/test-utils/tests/hand_written_impl_census.rs`'s invariant — a
field the comparison deliberately does not read binds to `_`, which is
what makes the decision visible and keeps the next field from landing
outside equality silently.

## Where it is held in the meantime

`KNOWN_HAND_LISTED` in
`crates/test-utils/tests/hand_written_impl_census.rs` names
`crates/editor-core/src/clearance.rs`'s `PartialEq` and points here. The
entry is a second finding stacked on the first, not an exemption, and a
sibling row reds if the walk ever stops finding it.
