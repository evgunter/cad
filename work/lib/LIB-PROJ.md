---
id: LIB-PROJ
kind: unit
title: two projection repairs: the mate accessors exhaustive, the pick indices as attributes
status: closed
opened: 2026-09-08
branch: lib/proj
refs: [mate-fault-accessors-wildcard-into-silence, mesh-index-numbers-cross-as-prose-under-a-projecting-door]
pr: 2236
closed: 2026-09-08
---

Two one-file projection repairs under the ratified rule (A) on
`pncad-py-seven-doors-lack-field-projection`: every arm's payload is
an attribute, present on every arm, `None` where the arm carries none,
from one exhaustive match with no wildcard.

1. `crates/pncad-py/src/py/mate.rs` — the `MateFault` accessors are
   exhaustive, and so is every other accessor in the file.
2. `crates/pncad-py/src/py/pick.rs` — `NodePickError` projects
   `patch`, `triangle` and `index`.

## Delivered

- `crates/pncad-py/src/mate_payload.rs` (new) — `MateFaultPayload`,
  `presence()`, `NONE`, and `mate_payload`, one exhaustive match over
  `MateFault`'s thirteen arms. Sited outside `py/` for
  `edit_payload.rs`'s reason: `crate::py` compiles only under the
  `python` feature, and the drift alarm has to ring on the row that
  runs everywhere.
- `crates/pncad-py/src/pick_payload.rs` (new) — `IndexPayload` and
  `index_payload`, exhaustive over `NodePickError` and over
  `MeshPickError` inside it. Outside `py/` for the same reason plus
  one more: the index arm is unconstructible from Python, so a Rust
  pin is the only way to read its three numbers back at all.
- `crates/pncad-py/src/py/mate.rs` — seventeen `MateFault` accessors
  read fields off the record; `MatePrimitive::offset`, `Subgroup`'s
  three and `ClusterMaintenance`'s seven became exhaustive matches in
  place (few arms, few accessors: a record there adds a layer without
  removing a match). Class docstring names the record and lists the
  seventeen in publication order.
- `crates/pncad-py/src/py/pick.rs` — `node_pick_err` carries the three
  numbers; its docstring says why the payload has no door of its own.
- `crates/pncad-py/src/lib.rs` — the two modules registered.
- `crates/pncad-py/src/py/mod.rs`, `crates/pncad-py/pncad.pyi` —
  `NodePickError`'s three new attributes, in the exception docstring
  and the stub.
- `crates/pncad-py/src/tests.rs` —
  `every_mate_fault_arm_projects_the_payload_it_carries` (nine of
  thirteen arms; the other four hold payload types the façade does not
  re-export) and
  `every_pick_arm_projects_the_index_numbers_it_carries`.
- `crates/pncad-py/tests/test_picking.py` — the three attributes in
  `test_every_arm_carries_every_attribute`, asserted `None` on the
  three reachable arms; the class docstring says where the numbers
  themselves are pinned.
- `crates/pncad-py/tests/ty_fixtures/legal.py` — the three attributes
  typed `int | None`.
- `crates/pncad-py/tests/test_binding_census.py` — the `MeshPickError`
  `BOUND_AS` prose and the B-PICKING gap note, with the measurement
  stated. The mapping itself did not move.

Deviations from the brief, both with their home:

- **The brief's claim "the three pick numbers reachable from Python
  and equal to the numbers the pick was made with" is not provable and
  was not claimed.** The arm reports a mesh violating its own
  invariant; nothing authorable reaches it and nothing in Python
  constructs it, exactly as the item says. The numbers are pinned in
  Rust instead, at three distinct values, and Python owns the
  `None`-shape half.
- **The unit swept its own file past `MateFault`.** `MatePrimitive`,
  `Subgroup` and `ClusterMaintenance` carried the same shape in the
  same file; `py/mate.rs` now has no `_ =>` at all.

Residue, each with its own file:

- `mate-fault-arms-carry-payload-that-does-not-cross` — six kernel
  fields over six `MateFault` arms that no attribute crosses, four of
  them holding types the façade does not re-export.
- `payload-accessor-wildcards-remain-in-checks-and-assembly` — the
  sweep's hit list outside this unit's two files, with dispositions.
