---
id: PORT-DIMS-1
kind: unit
title: The dimension refusals at the Python boundary: keep the structure at the load door, and let the name mean what Rust means
status: spec
opened: 2026-09-15
refs: [694, 689, S107]
branch: port/dims-1-load-door-and-name
---


One door, two questions, one unit. `load-path-stringifies-structured-refusals`
asks that a structured kernel refusal keep its structure when it crosses
the load door; `python-dimensionerror-names-the-quantity-check-not-the-dimension-check`
asks what the class it arrives as is called. Neither is answerable alone:
the first row's fix has to name a class, and the class the second row
frees is the candidate. Specced together (`work/port/plan.md`, Order).

## The rows

- `load-path-stringifies-structured-refusals` — **H**, and the one row
  this program gives a full review (`plan.md`, Review posture, triggers
  2 and 3).
- `python-dimensionerror-names-the-quantity-check-not-the-dimension-check`
  — **M**, `S107`'s successor. Ev ruled the naming a defect on
  2026-09-15: *Python should always match Rust where it can.*

## Spec

`docs/PORT-DIMS-1-SPEC.md`, deleted at merge with its `docs/DOC-LEDGER.md`
row in the same PR.

## Territory

`crates/editor-core/src/persist/*` is **EDIT's**; `crates/pncad-py/*` is
**LIB's**. PORT claims no paths; both are announced and either may take
the unit.
