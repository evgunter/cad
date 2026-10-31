---
id: geometrywitness-eq-ignores-the-two-chart-axes-its-uv-fields-are-stated-in
kind: issue
title: GeometryWitness's hand-listed PartialEq leaves a_chart_axis and b_chart_axis outside equality, and nothing holds the omission a decision
status: open
opened: 2026-09-15
priority: P3
cost: E
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
read, so two witnesses are equal whenever their seven agree. It is a
hand-written `eq` with no destructure, so the omission is not a
decision the compiler holds.

`a_uv`'s own doc says its parameters are *"IN THE CHART `a_chart_axis`
names — which for a planar face is the engine's own re-chart, not the
stored one"*, and `a_chart_axis` is *"Which world axis the planar
re-chart crossed the normal with, so a consumer can rebuild the same
chart ([`chart_frame`]) and evaluate at `a_uv`"*. So `a_uv` has no
meaning except relative to `a_chart_axis`, and a consumer that
round-trips a witness through `chart_frame` reads a field this `eq`
does not.

**What that is, stated precisely: a missing TIE, not a live wrong
answer.** The scenario of two witnesses with the same `(u, v)` numbers
in different charts does not survive the comparison — `eq` compares
`a_point` and `b_point` coordinate by coordinate
(`crates/editor-core/src/clearance.rs`, `:546`), and a point is what the
chart and the parameters together name, so two witnesses whose charts
differ at that `(u, v)` have different points and already compare
unequal. What remains is narrow: two witnesses over the same faces,
carrying the same `uv` numbers and the same points, whose `chart_axis`
fields differ — one spelling the stored chart as `None` where the other
names the axis its re-chart crossed. Those compare equal, and for that
pair they name the same point, so equality is arguably right. The
defect is that **nothing holds it right**: the two fields are outside
the comparison by omission rather than by decision, the next field added
to the nine lands outside it the same way, and a change to
`chart_frame` that made two charts disagree at a shared `(u, v)` would
turn the omission into a wrong answer with no test moving.

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
