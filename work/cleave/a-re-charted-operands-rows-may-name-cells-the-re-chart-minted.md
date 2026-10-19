---
id: a-re-charted-operands-rows-may-name-cells-the-re-chart-minted
kind: issue
title: A boolean over re-charted sphere operands may emit rows naming cells the re-chart minted, which no input holds
status: open
opened: 2026-10-19
priority: P3
cost: E
---


Filed by WIRE's `wire-citerows` lane (branch
`wire/boolean-rows-every-decision`).

## What

`boolean/ops.rs` `through_the_join` re-runs a sphere boolean over
re-cut operands: `apply_recuts` re-charts some shells (carved and
grafted) and `apply_cut_ins` cuts faces along chart meridians. Every
row a boolean emits names its INPUTS' cells (`crate::coincidence`), and
that unit maps a cut-in's cells back (`ChartCuts::input_cell`: a
division to the face it was cut from, a meridian to the face it runs
in, an arc split to its arc). Nothing maps a re-chart's: the comment
beside the call says `carve` keeps every kept entity's key, but a
grafted shell's cells are new keys. Once every margined decision is a
row, a decision on a re-charted shell's cell reaches the document as a
cell no input names, and `editor-core` `coincide::name_rows` refuses the
node ("a coincidence row names a cell no input of its node names"), as
the cut-in did on `emit_sphere_cut_in_cap` before the mapping.

Unmeasured: no test in `topo` or `editor-core` failed on it. Find a
re-chart fixture with a Zero decision on the grafted shell, and map its
cells or refuse typed.

Signed (WIRE implementer, wire-citerows).
