---
id: an-assertion-row-shows-no-verdict
kind: issue
title: an Assertion row shows that it exists and never its verdict
status: closed
branch: author/assertion-verdict
pr: 3535
opened: 2026-09-30
priority: P0
cost: M
refs: [the-gui-shows-no-measure-value-and-no-clearance, measure-assertion-offers-an-unvalued-tighten-and-drops-its-margin]
closed: 2026-09-30
---


Found by AUTH-7's sweep for evaluated values the viewer never shows.
An `Assertion` node's row reads `Assertion` and nothing else:
`tree::node_kind` (`crates/viewer/src/tree.rs`, ~:385) is the whole
label, and `tree::measured_of` answered `None` for
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
kernel's word for the state. `Measured` in `tree.rs` (AUTH-7 renamed it from `Reading`) is the
enum this would extend.

Dispatched 2026-09-30 as **AUTH-8** (`docs/AUTH-8-SPEC.md`, branch `author/assertion-verdict`). Checked first: `UnevaluatedReason` has more arms than listed above (`Indeterminate`, `MeasureUnavailable`, a carrier-endpoint arm), and the assertion names its measure by id, so the verdict's numbers can take the measure's `dim`.

## Closed 2026-09-30 — PR 3535 merged (`237e1be9`)

**An assertion row shows its verdict.** It shows the kernel's own label (`Holds` / `Violated` / `Unevaluated`) and, where there are numbers, `measured <relation> bound`. The relation uses the kernel's `AssertionDir::symbol`, and the numbers are written in the measure's dimension, read off its landed payload. `Unevaluated` gives its reason in the kernel's own words. For `MeasureUnavailable` the row links to the measure's row ("see feature N") rather than repeating the sentence already drawn there.

**Tone:** `Violated` is `Actionable`; `Holds` and `Unevaluated` are quiet. `Tone::Actionable` is "a verdict a reader may need to act on", and E10's report-only rule concerns what consumes a verdict, not how loud it is drawn. The correctness review agreed that this was not close. A violated assertion does not make a building document read as broken (`has_faults` is unchanged).

**Correctness** swept 44 bound × direction × dimension cases and found no row showing a verdict the kernel did not reach, and no pair of numbers swapped. Inside ε of the bound, a `Holds` row can print a comparison that is literally false (`Sign::Zero` → Holds). That is a notation matter, recorded on CHROME's units row.
