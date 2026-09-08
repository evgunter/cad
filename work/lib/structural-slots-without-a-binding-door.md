---
id: structural-slots-without-a-binding-door
kind: issue
title: two of the four structural slots have no Python door
status: open
opened: 2026-09-08
---



Found by LIB-B-PART's sweep, banked rather than acted on.

## What is true

A slot is STRUCTURAL exactly when its dimension is `Count`
(`crates/editor-core/src/node.rs:498`), and four slots answer that:
`SlotId::{Count, VDegree, Stations, Instance}` (`node.rs:486`).
`DocEdit::SetStructuralParam` accepts any of them.

Python has a door per slot, by the census's own shape decision — a
slot is addressed through its own spelling rather than by crossing
`SlotId` (`test_binding_census.py`'s `NOT_BOUND` docstring, the
`different-shape` paragraph). After LIB-B-PART there are TWO doors:
`DocEdit.bind_count_param` and `DocEdit.bind_instance_param`. So two
slots have no door:

- **`VDegree`** — a loft's / sweep's v-direction interpolation degree.
  Authorable as a LITERAL: `Node.loft(profiles, v_degree)` takes a
  plain `int` (`crates/pncad-py/src/py/doc.rs`, `pncad.pyi`). Not
  bindable to a document parameter, so changing it is a re-authoring
  and each degree is a new document — exactly the state
  `bind_count_param` exists to end for a pattern's count.
- **`Stations`** — a sweep's station count. Unreachable at all: its
  only node is `Node::Sweep`, which has no Python constructor and
  whose kernel arm refuses unconditionally (the SWEEP_FRONTIER lane,
  `docs/guide/north-star-audit.md`'s G2). Nothing is owed here until
  the sweep node exists; it is listed so the count of four is
  accounted for rather than two-of-four being read as an oversight of
  two.

## Why it was not fixed at LIB-B-PART

`bind_instance_param` was in that unit's charter (`SlotId.Instance`);
`VDegree` is not, and a loft's degree is not a projection's index.
Binding it is the same mechanical shape — six lines beside its
sibling, a stub entry, and a test that moves a degree with one
`set_doc_param` — and wants a scene that shows a degree actually
mattering, which is the part worth doing deliberately.

## What was corrected in place instead

`bind_count_param`'s own prose said the narrowing was safe because
"the slot is `Count` — the only structural slot there is". That
sentence was false when written and is now deleted; the doors are
narrow because each one NAMES its slot, which is the reason that
survives a second door.
