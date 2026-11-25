---
id: main-red-a-spline-capped-boxs-moved-cap-is-left-unminted
kind: issue
title: Main red: shell_general_door's spline-capped box leaves its moved cap's pcurves unminted at 1e-9
status: open
opened: 2026-11-25
---


Found by the `sectarms/one-image-function` lane (PR 7537), whose diff
edits `crates/sweep/tests/shell_general_door.rs` and so runs its slow
rows per PR.

## What

`sweep::all shell_general_door::a_spline_capped_box_moves_every_chart_onto_its_closed_form`
is red on `origin/main` at `bc34e6f933`, at the default ε (run alone in
a worktree of that commit, 2026-11-25):

```
eps 1e-9: expected only the fitted cap's quadrature refusal, got
[Pcurve { finding: Unminted { face: FaceKey(1v1) } }]
```

(`crates/sweep/tests/shell_general_door.rs:110`.) The shell is built,
its eight corners and four sections read as the row expects, and
`validate_geometric` then finds the moved cap's pcurves unminted, where
the row expects only the fitted cap's quadrature refusal. PR 7537's head
reads the same finding, on the hosted run and locally; it is not that
diff's.

The row is in the slow set, so a PR that does not edit its file or spell
its name does not run it.

## Done when

The row is green on main, or re-pinned with the reason the cap's rows
are unminted.
