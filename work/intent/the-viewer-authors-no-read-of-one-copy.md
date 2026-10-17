---
id: the-viewer-authors-no-read-of-one-copy
kind: issue
title: The viewer's authoring forms take an Operand, so they cannot write a read of one copy of an instance: a boolean, fillet or split over an instance refuses in the GUI
status: open
opened: 2026-10-16
priority: P1
refs: [a-placement-is-the-bundle-of-mates]
---


Found by INTENT stage 3 B's viewer migration (PR #5232).

An instance's one output is `copies`, and a seat that reads one body (a union or intersect member, a subtract's `from` or `tool`, a split's target, a fillet's body) reads one copy of it: a keyed read, `BodyRead { read, key: Some(CopyKey::Part { .. }) }`, written through `SlotValue::Copy { of, key }` (`crates/editor-core/src/edit.rs`). A whole read of an instance at a body seat refuses at the door (`EditError::SlotVarKind { found: Bodies, .. }`).

The viewer's authoring forms (`crates/viewer/src/session/author.rs` and the combine and feature forms that call it) build their reads as an `Operand` and never a keyed read. So a boolean, fillet or split picked over an instance refuses, and a feature over an instance places nothing. The kernel and the Python binding can write the read; the GUI cannot.

The refused gestures:
- a union, intersect or subtract whose member, `from` or `tool` is picked on an instance;
- a fillet, chamfer or shell on an instance's body;
- a split of an instance;
- a transform or pattern of one copy of an instance, which places nothing because the form writes the whole read.

Shape of a fix: when a pick lands on an instance's copy, the form writes the keyed read of that copy (the copy the picked face's name lands in, `CopyKey::Part { placement, within }`), as `fixture::copy` does in the tests.
