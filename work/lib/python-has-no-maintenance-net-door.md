---
id: python-has-no-maintenance-net-door
kind: issue
title: Python has no MaintenanceNet door, so a caller composing a cascade re-derives the net by hand
status: open
opened: 2026-09-25
priority: P3
cost: D
---


`editor_core::MaintenanceNet` (`crates/editor-core/src/edit.rs`,
carried by `pncad::document`) is the one statement of which DM7
maintenance rows survive an action of several edits: a strand whose
carrier a later edit repaired or deleted, an orphan a later edit
consumed again or deleted, a rename a later edit moved on or stranded
are folded out, and a caller pushes each accepted edit (`Applied`) and
finishes against the end document. The viewer's session folds every
action it commits through it (PR #3196).

Python binds `Doc.apply` one edit at a time and nothing that folds, so
a Python caller composing a cascade — which is how the stub tells it
to delete a node with dependents — has to re-derive the net by hand.
The stub says as much: `pncad.pyi`'s `orphaned_declare` paragraph
tells the caller to "read the net effect off the document the walk
ended at, not off the rows". The binding census records this as
`gap: B-MAINT-NET` (`crates/pncad-py/tests/test_binding_census.py`,
`FAMILIES` carries the charter).

Closing it: a Python door that folds each `Doc.apply` result with the
document it produced (a `MaintenanceNet` class, or an `apply_all` that
answers the net), the stub paragraph pointing at it, and one Python
row cascading a declared union's `Declare` away and asserting an empty
net.
