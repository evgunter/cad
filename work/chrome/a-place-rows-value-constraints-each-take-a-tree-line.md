---
id: a-place-rows-value-constraints-each-take-a-tree-line
kind: issue
title: viewer: a Place row in the model tree draws one line per value constraint under it; it should read on one line like every other row
status: open
opened: 2026-11-01
priority: P1
cost: E
refs: [the-tree-view-nests-a-nodes-inputs-under-its-consumer]
---
Reported by Ev (chat, 2026-11-01). Filed P1 at Ev's request. In the
model tree a `Place` node draws its row, then one indented line per
constraint:

```
constraint 1: slide along x value
constraint 2: slide along y value
constraint 3: slide along z value
constraint 4: spin about x value
constraint 5: spin about y value
constraint 6: spin about z value
```

Ev: "Place should display on one line like everything else."

## Where

- `TreeRow::constraints` (`crates/viewer/src/tree.rs:294-298`), built
  by `constraint_rows` (`tree.rs:327`) and worded by `constraint_said`
  (`tree.rs:316`);
- drawn as child lines, each with a remove button, by
  `constraint_lines` (`crates/viewer/src/pane/features.rs:151-164`).

## Fix direction

The tree row is one line. The constraints, and the per-constraint
remove buttons, belong in the Properties pane when the placement is
selected. A summary on the row (e.g. `Place · 6 values` or
`Place · 2 mates`) is fine if it fits. Keep the mate-refusal lines
under the row (`tree.rs:50-84`, carried failures). They are failure
reporting, not constraint listing. Check the PLACING rows on the same
gestures (`work/placing/`) so the remove button keeps a home.
