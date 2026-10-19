---
id: ssi-payloads-print-round-numbers-in-exponent-form
kind: issue
title: geom-brep: SsiError payloads print round numbers in exponent form ('3.125e0 m step', '2e0 m per chart unit')
status: open
opened: 2026-10-14
priority: P4
cost: E
---

(Filed by the ENCL implementer of `zero-arm-payload-wording-doubles-words-and-hedges-a-definite-verdict`, from its sweep for the "e0 m" shape outside ENCL ground.)

## What

`impl Display for SsiError` (`crates/geom-brep/src/ssi.rs`) prints every number with `{:e}`, so a modelling-scale value reads in exponent form: the pins show it.

- `ssi.rs` test beside `SsiError::TraceUnresolved` pins "has 1 sample at the march's 3.125e0 m step".
- `crates/geom-brep/tests/exhaust_lane_meters.rs` pins "a cell 1.25e-1 chart units (2.5e-1 m) wide" and "floor 2.5e-1 chart units (5e-1 m at a certified chart speed of 2e0 m per chart unit)" (the exhaust lane's refusal, `ssi/exhaust.rs`).

The same Display's step, speed, extent, reach and window numbers (the `{step_meters:e}`, `{speed:e}`, `{half_extent:e}`, `{reach:e}` arms) take the same form.

## Repair shape

Print each number through `geom_core::Readable` (positional for `1e-4 ≤ |x| < 1e16` and zero, scientific outside), as ENCL's payloads now do through `MarginDiag::readable()`: "3.125 m step", "0.25 chart units (0.5 m at a certified chart speed of 2 m per chart unit)". The exhaust test's substring anchors were chosen against exponent form (`"5e-1 m"` inside `"2.5e-1 m"`) and want re-reading under the new form.
