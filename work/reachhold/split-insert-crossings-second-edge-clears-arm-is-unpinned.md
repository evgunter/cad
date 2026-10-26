---
id: split-insert-crossings-second-edge-clears-arm-is-unpinned
kind: issue
title: the split's insert_crossings re-checks edge_clears for an unlaned carrier behind gate_operand's identical check, and no row pins the copy
status: open
opened: 2026-10-03
priority: P4
cost: E
---


Filed from PR 3984's dual review (r2 mutant M5, r2 NOTE 6;
pre-existing).

## Finding

`crates/topo/src/splitting/classify.rs` `insert_crossings` passes an
unlaned (spiric or spline) carrier only behind `edge_clears`, and
refuses `SplitReduceError::CurvedEdgeUnsupported` otherwise. The split's
own `gate_operand` (same file, the edge loop) runs the identical check
first on every operand edge, so the arm in `insert_crossings` is a
second copy no input reaches: r2's mutant M5 (the arm skips
`edge_clears`) survives every topo and sweep split, loft and spiric
row.

## Fix

Either pin it (a row that drives `insert_crossings` directly with a
spline edge the plane meets, red when the arm passes it) or delete the
copy and state at the arm that `gate_operand` decided the edge clears.
