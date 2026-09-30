---
id: an-assertion-row-shows-no-verdict
kind: issue
title: an Assertion row shows that it exists and never its verdict
status: open
opened: 2026-09-30
priority: P0
cost: M
refs: [the-gui-shows-no-measure-value-and-no-clearance, measure-assertion-offers-an-unvalued-tighten-and-drops-its-margin]
---


Found by AUTH-7's sweep for evaluated values the viewer never shows.
An `Assertion` node's row reads `Assertion` and nothing else:
`tree::node_kind` (`crates/viewer/src/tree.rs`, ~:385) is the whole
label, and `tree::reading_of` answers `None` for
`ValuePayload::Assertion`. The verdict the kernel hands back
(`AssertionVerdict<f64>`, `crates/editor-core/src/measure.rs` ~:740)
reaches no surface. A document can record "this web is at least
0.5 mm" (E10), violate it, and show a row the GUI cannot tell from
one that holds.

Not folded into AUTH-7 because it is more than the same few lines.
Three choices are open, and none of them has a home yet:

* **The numbers need a dimension the verdict does not carry.**
  `Holds`/`Violated` carry `measured` and `bound` as bare `f64`. The
  measure's `dim` sits on the measure node's payload, so the row has to
  read another node's value before it can spell its own through
  `props::computed_text`.
* **How loud a `Violated` row is.** E10 v1 makes an assertion
  report-only. Whether a violated one is `Tone::Actionable`, like a
  failed row, or stays advisory is a call about what the tree's one
  loud row means.
* **Where `Unevaluated`'s reason goes.** `UnevaluatedReason` has a
  `Display` of its own. One of its arms, `MeasureUnavailable`, wraps
  the same `MeasureUnavailableAt` that AUTH-7 draws under the measure's
  row, so the same kernel sentence would be drawn twice, one row apart.

`AssertionVerdict::label` (`Holds`/`Violated`/`Unevaluated`) is the
kernel's word for the state. `Reading` in `tree.rs` is the variant
this would extend.
