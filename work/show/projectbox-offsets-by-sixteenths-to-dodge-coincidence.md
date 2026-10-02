---
id: projectbox-offsets-by-sixteenths-to-dodge-coincidence
kind: issue
title: projectbox offsets every feature by 1/16 so no operand planes coincide; the declared door makes the flush enclosure authorable
status: open
opened: 2026-10-02
priority: P3
cost: M
---

## What

Found in the sweep for `letterforms-flush-declared`, which retired the
letterforms' 1/16 decoupling and with it the module doc that stated
"#91's design rule" (operands never share coincident planes).
`demos/tour/src/projectbox.rs` still follows that rule — its module doc
says "no two operand planes coincide anywhere in the chain (all features
offset in 1/16 steps)", its bosses union "with a 1/16" overlap into the
floor, and the `projectbox` caption repeats "the #91 design rule" — and
its Python mirror (`crates/pncad-py/tests/test_north_star.py`,
`TestProjectbox`) says the rule "is exactly what makes it authorable
without a declaration door". The declaration door exists now
(`pncad::topo::{union_with, subtract_with}` fed by `booleans::flush_declarations`, and
`Doc.declare_all` in Python), so the offsets are a dodge: a real
enclosure's bosses stand on its floor.

`heatsink`'s 1/16 fin overlap is the same shape and is already tracked
(`heatsink-placedunion-base-union-unfinished`, #1344); `twopeg` cites
the rule only as the reason a dodge was removed.

## Do

Re-author `projectbox` flush with its contacts declared, re-derive its
exact oracle, and retire the citation of the rule; measure the 15-op
chain first and pin any refusal with `walls::wall`.
