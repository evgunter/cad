---
id: a-join-reminting-a-carried-row-refuses-as-a-kernel-defect
kind: issue
title: A row the join's re-mint carries at its old interval on a face no door owes refuses as JoinRefusal::Kernel, where the route is unbuilt
status: open
opened: 2026-10-08
---


## The finding

The join door re-derives the rows its kills moved
(`crates/topo/src/boolean/edge_join.rs:601`). A row the whole-body
pass carries rather than derives, on a face no door owes a fresh row,
is re-certified at its old interval; where that refuses, the refusal
reaches a door that is not the boolean as `BooleanError::Pcurves`.
`JoinRefusal::of` (`edge_join.rs:497`) folds every arm but the
predicate's and the carrier's into `JoinRefusal::Kernel`, which
renders the kernel-or-file-defect ending (`edge_join.rs:511`). But
the case is not a defect: it is a route the join does not build yet
(a carried row restated over the joined edge's interval), and its
honest ending is "there is no way through this yet", naming this row.

Found by the delta review of PR 4302. No `ci` row reaches it.

## What it needs

Either the join restates a carried row over the joined edge's interval
rather than leaving it to the pass, or `JoinRefusal` gains an arm for
the re-mint's refusal that names this row and says the route is
unbuilt.
