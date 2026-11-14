---
id: measure-spells-the-cylinder-axis-name-five-times
kind: issue
title: measure.rs spells carrier_cyl_axis_parallel as a literal beside its own AXES_PARALLEL const, under topo's carrier_eq name
status: open
opened: 2026-11-09
priority: P3
cost: E
---


Found by ONECOPY's topo roster lane (`topo-has-no-predicate-name-roster`).

`crates/editor-core/src/eval/measure.rs` declares `const AXES_PARALLEL: &str = "carrier_cyl_axis_parallel"` ("the site two axes are decided parallel at, the cylinder pair's own"). The same file then spells the literal four more times: in `distance`'s cylinder arm (`parallel("carrier_cyl_axis_parallel", …)` and `not_parallel("distance", "carrier_cyl_axis_parallel", …)`), and in `gap`'s cylinder arm (the same two). The literal is also topo's name: `topo::boolean::carrier_eq`'s cylinder datum `CARRIER_CYL_AXIS_PARALLEL`, which `topo::decision_words` renders. So a measure decision and a boolean decision are metered as one K row, under two independent spellings.

Fix: route the four literals through `AXES_PARALLEL`. Then decide whether measure's parallelism should share topo's row at all, and if it should, take the name from topo's const (made `pub`) rather than restating it.
