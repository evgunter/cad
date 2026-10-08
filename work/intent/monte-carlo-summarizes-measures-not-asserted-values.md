---
id: monte-carlo-summarizes-measures-not-asserted-values
kind: issue
title: Monte Carlo summarizes each Measure node, so a measurement with arithmetic has no population row
status: open
opened: 2026-10-08
---


Since stage 2 PR D a `Measure` holds one primitive, and a measurement with
arithmetic is measures plus a definition over their outputs, read by an
assertion. `mc::monte_carlo` (`crates/editor-core/src/mc.rs`, the
`is_measure` branch reading `ValuePayload::Measure`) still keeps one
`McMeasure` row per `Measure` node, so a web `distance − r_a − r_b` has
a row for the distance and none for the web; the assertion's row counts
verdicts only.

Found migrating the tour: `demos/tour/src/mcplate.rs` now checks its
replay against the separation row bit for bit and reads the web itself
per sample through `Evaluation::reading`, which no lane offers in bulk.

The natural shape is a row per observed variable an assertion reads (the
stackup already takes any scalar `VarId`), or the asserted value's
statistics on `McAssertion`. Either is an API change to `McReport` and
Python's `McMeasure`.
