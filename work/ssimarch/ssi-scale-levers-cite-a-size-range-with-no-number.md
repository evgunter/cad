---
id: ssi-scale-levers-cite-a-size-range-with-no-number
kind: issue
title: Four SSI scale levers send the user 'within the model's size range', which names no number and may not be the lever for geometry already inside the 1 km box
status: open
opened: 2026-10-12
---

(Filed by the ENCL lane on `encl/eps-floor` from its sweep for text that claims a size range with no number.)

## What

Four sized decisions in geom-brep's SSI end their recourse with "the model's size range". D4 ¶4 now gives that range a number: a 1 km session box, with a resolution floor `geom_core::tolerance::EPS_MIN` ≈ 2.2e-13 m. The levers name neither:

- `crates/geom-brep/src/ssi.rs`, `STEP_SCALE`: "bring the operands within the model's size range, or name a domain that holds more of the intersection".
- `crates/geom-brep/src/ssi/exhaust.rs`, `R3_SCALE`: "move the geometry nearer the origin, within the model's size range, where its coordinates resolve the tolerance".
- the same file, `CHART_SCALE`: "bring the spline face within the model's size range, or its parameter domain nearer zero, ...".
- the same file, `CHART_TOO_SLOW`: "bring the spline face within the model's size range; it is too small for a length in metres to be stated in its parameters".

## Why it is not a citation swap

- Three of the four fire on geometry that may already be inside the 1 km box. For `R3_SCALE` at ε near ε_min, a coordinate near 1 km is inside the box and still resolves nothing finer than its own spacing. "Within the range" would then send the user nowhere, and the honest lever may be the origin, or the tolerance.
- `CHART_TOO_SLOW` is about a face too *small*. The box has no lower bound, so no size range answers it.
- D4 ¶4's rejection at construction is not built (`work/scalar/no-door-rejects-geometry-past-eps-f64-resolution.md`). Until it is, nothing upstream refuses what these levers ask the user to avoid.

`crates/geom-brep/tests/m5_pr7_ssi.rs` pins the `CHART_*` wording ("Recourse: bring the spline face within the model's size range") and moves with any rewording.
