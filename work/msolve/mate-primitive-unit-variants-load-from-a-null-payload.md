---
id: mate-primitive-unit-variants-load-from-a-null-payload
kind: issue
title: A unit MatePrimitive variant loads from a second spelling, {"coaxial": null}, before and after deny_unknown_fields
status: open
opened: 2026-09-19
priority: P0
cost: E
---


(MSOLVE-7 fix pass, 2026-09-19; the correctness arm's `c4c` probe.)
`MatePrimitive` (`crates/editor-core/src/mate.rs`) is an externally
tagged enum whose unit variants save as bare strings — `"primitive":
"coaxial"`. Serde's unit-variant reading also accepts the map form
with a `null` payload: `{"coaxial": null}`, `{"frame_coincidence":
null}` and `{"clocking": null}` each LOAD through `persist::load` as
the variant, both before and after `deny_unknown_fields` landed
(measured on a saved mate-bearing document with the spelling
substituted; `{"coaxial": {}}`, `{"coaxial": {"x": 1}}` and
`{"coaxial": []}` refuse `Unreadable`). So the wire has two accepted
spellings of one unit variant, one of which no build writes.

A wire fact for the owner to rule on: the persist module docs rule
against silently DROPPING data and say nothing about a second
spelling that loses none. If one spelling is the contract, the load
door owes a refusal of the other (a custom `Deserialize` for the
enum, or a validator pass over the raw text); if both are fine, the
docs should say a unit variant has two readings so a golden that
spells `null` is not mistaken for a format drift. Not MSOLVE-7's: the
unit's attribute changes what a field-bearing variant accepts, not
how a unit one is spelled.
