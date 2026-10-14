---
id: the-unnamed-slot-crosses-as-a-hit-test-error
kind: issue
title: NodePick.patch_names' unnamed slot crosses to Python as a HitTestError value while the kernel's lane is UnnamedEntity
status: open
opened: 2026-09-29
priority: P4
cost: E
---

Filed by `edit/unnamed-patch-is-a-lookup`
(`work/edit/an-unnamed-patch-is-reported-as-a-hit-test.md`).

## The finding

The kernel's `NodePick::patch_names` and `boundary_names`
(`crates/editor-core/src/resolve/pick.rs`) fill each slot with a
`StableName` or `UnnamedEntity` (`crates/editor-core/src/resolve/hit.rs`),
the name lookup's one refusal, whose `Display` opens *"name lookup:"*
and names no hit test. The Python binding's slot value
(`crates/pncad-py/src/py/pick.rs`, `slot_name` → `hit_test_value`)
is a `HitTestError` exception value built through the kernel's own
`From<UnnamedEntity> for HitTestError`, so a Python caller reading a
slot gets the class the stub promises (`list[str | HitTestError]`,
`crates/pncad-py/pncad.pyi`), variant `unnamed`, `node`, `kind` and
`body` — and a message that opens *"hit test: name lookup: …"*, a
hit-test prefix on a door that ran none. The census disposition is
`"UnnamedEntity": "HitTestError"` in
`crates/pncad-py/tests/test_binding_census.py` (`BOUND_AS`).

The crossing was kept at one spelling because the Python class of a
slot value is LIB's surface, not EDIT's: changing
`list[str | HitTestError]` is an API decision.

## What a fix would be

Either Python gets its own value class for the slot (an
`UnnamedEntity`-shaped exception whose message is the kernel's lookup
sentence, and the stub's `list[str | …]` names it), or the current
crossing is ruled adequate and this row records the reading. Both are
LIB's call.
