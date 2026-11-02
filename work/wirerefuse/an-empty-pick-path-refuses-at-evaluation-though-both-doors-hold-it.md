---
id: an-empty-pick-path-refuses-at-evaluation-though-both-doors-hold-it
kind: issue
title: An empty pick path refuses at evaluation (NoSuchPiece at 0) though both doors refuse it
status: open
opened: 2026-11-02
priority: P4
cost: E
---



Found by the sweep of `a-loft-of-fewer-than-two-sections-refuses-at-evaluation`
(branch `wirerefuse/loft-count-at-load`), which looked for evaluation
refusals of facts the document doors already hold.

`crates/editor-core/src/eval/wire.rs`, `pick_of`: when `pick.of` is
empty the walk reaches no piece and returns
`NodeErrorKind::NoSuchPiece { pick: var, at: 0 }`. An empty path is
refused at both doors by one walk, `edit::pick_fault` (`!copied ||
of.is_empty()` → `Some(None)`): the edit door as `EditError::PickPath`
and the load door as `SnapshotError::PickPath { at: None }`
(`persist/check.rs`, "Every pick, by the edit door's own walk"). So the
evaluation arm is reachable only if a pick reaches evaluation without
passing either door, and `at: 0` names a step that does not exist.

## What is owed

Check that no in-crate writer (a split's or a reshaping's rewrite of a
pick, `refactor.rs`) can leave a pick's path empty. If none can, make
the arm `unreachable!` and cite `pick_fault` (D9 row 4). If one can,
make that writer refuse, as the loft's section count does at
`edit::loft_structure_fault`.
