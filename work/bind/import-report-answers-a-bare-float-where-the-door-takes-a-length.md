---
id: import-report-answers-a-bare-float-where-the-door-takes-a-length
kind: issue
title: ImportReport.eps_in answers a bare float while import_step now takes a Length
status: open
opened: 2026-09-15
refs: [2249]
priority: P4
cost: E
---

## From PORT's `python-cannot-set-options-structs` review (PR #2678)

That PR bound `ImportOptions::eps_in` as `import_step`'s `eps_in=`
keyword, taking a `Length` — §L4's boundary rule, and the same shape
`Evaluation.step_string`'s `uncertainty=` already had. The READ-BACK
side of the same quantity, `ImportReport.eps_in`
(`crates/pncad-py/src/py/value.rs`, `#[pyo3(get)] eps_in: f64`), answers
a bare float documented as "in metres".

So one line of that PR's own new test reads:

```python
self.assertEqual(import_step(step, eps_in=1e-6 * m).eps_in, 1e-6)
```

— a typed quantity in, an unlabelled number out, in one expression. The
asymmetry is pre-existing on the report side (LIB-IMPORT-REPORT, #2249)
and this is the change that puts the two halves next to each other,
which is why it is filed now rather than then.

**It is not obviously a defect and that is the row's question.** The
surface has a standing convention that readbacks hand back floats —
`MassProperties.volume`, `.surface_area` and the two pads all do — and
`eps_in` following it is consistent. What is new is that this is now
the one quantity a caller both SETS and READS, and it changes units
between the two.

Two dispositions, and the row is which:

- make the readback a `Length`, which makes the round trip `x == y`
  spellable and breaks the readback convention for one attribute; or
- record the convention explicitly at the attribute — inputs are typed,
  readbacks are metres — so the next reader of that test line is not
  asking this question again.

## Home

`work/lib/` — `crates/pncad-py/*` is LIB's territory. Filed by a PORT
lane.
