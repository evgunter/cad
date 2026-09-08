---
id: tess-lint-twinned-csv-fixture
kind: issue
title: tess-lint's two test-CSV fixtures are twinned by hand across the crate/integration boundary
status: open
opened: 2026-09-07
---

Filed by the style-review fix pass on `meter/join-gated-voice` (PR 2111).
Pre-existing, outside that unit's diff, so reported rather than swept there.

`tools/tess-lint/src/lib.rs:1441` (`tests::csv`) and
`tools/tess-lint/tests/cli_contract.rs:33` (`scene`) build the SAME
two-face fixture — one plane at ordinal 0, one NURBS wall at ordinal 1,
parameterised by `tris` and `span_opt` — from two copies of one
28-column format string. Both comment sites say so and both end with
**"Keep them in step."**, which is an invariant nothing enforces: the
copies differ already (the lib copy calls its helper `csv` and reads
`IDENTITY_FIRST` from the crate; the integration copy calls it `scene`
and re-derives the same index by `position(|c| c == "u0")`), and
nothing reds if a future column lands in one and not the other.

The stated reason for the split is real — an integration test cannot
see a `#[cfg(test)]` item — but it is a reason not to share THAT item,
not a reason to have no shared one. Cures, cheapest first:

- an `include!`d fixture file both sides pull in;
- a `#[doc(hidden)] pub mod fixtures` behind a `test-support` feature;
- one test that asserts the two strings are equal for a couple of
  `(tris, span_opt)` points, which enforces "in step" without moving
  either copy.

Sweep note: the same `Keep them in step` / `The twin of` vocabulary
turns up nowhere else under `tools/tess-lint`; what that grep cannot
match is an undisclosed copy, and the constants sweep for one is the
28-column row literal itself, which has exactly these two occurrences.
