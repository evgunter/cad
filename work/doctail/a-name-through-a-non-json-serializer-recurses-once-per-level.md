---
id: a-name-through-a-non-json-serializer-recurses-once-per-level
kind: issue
title: "names: a StableName serialized outside the JSON doors (meta::to_value, a caller's own serializer) takes the derived form's recursion, once per nesting level"
status: open
opened: 2026-09-30
priority: P3
cost: E
---

(EDIT, found by the name-nesting row's sweep, `edit/name-nesting-stack-safe`.)

## What

A `StableName` writes and reads its JSON one nesting level at a time
only inside a JSON door (`names::nest::json_door`: `persist`'s save,
load and canonical bytes, `StableName::to_json` / `from_json`, and the
Python binding's name text). Outside one, its serde impls are the
derived form (`nest.rs`, `WireOut` / `WireIn`), which serde's data model
can only walk recursively: each held name is serialized inside its
holder's `serialize_field`.

So a name handed to any other serializer or deserializer recurses once
per level. In the tree that is `meta::to_value` / `meta::from_value`
(`crates/editor-core/src/meta/ser.rs`), a public door that takes any
`T: Serialize`, and a caller's own `serde_json::to_string(&name)`
outside `to_json`. No shipped door passes a name through either; a
caller that does, with a name a few thousand levels deep, overflows a
small stack. `MetaValue`'s own nesting has no bound either.

## What would close it

Either say at `meta::to_value`'s door that a name is not metadata
(refused typed), or give the derived path a depth it refuses past. A
row that puts a deep name through `meta::to_value` on the smallest
stack.

## Partly closed by PR 3909

`meta::to_value` now bounds what it builds and how deep it reads a
producer (`meta::MAX_NESTING`, `meta::MAX_PRODUCER_NESTING`), so a
name 10 000 levels deep through it refuses typed rather than
overflowing, and "`MetaValue`'s own nesting has no bound either" above
no longer holds. Measured on the 1 MiB stack, a name nested by
`RoleSeg::Merged`: with a node id within `i64`, it refuses as
`MetaError::NestedTooDeep` from 32 levels (40 and 10 000 too); with a
node id above `i64::MAX` (a minted id can be), it refuses as
`IntOutOfRange` at any depth, one level included, because a
`RecipeNodeId` serializes as a `u64`. So `to_value` refuses a deep name
as a deep value, not as a name, and a shallow one by accident of its
id. A caller's own serializer outside `to_json` is unchanged.
