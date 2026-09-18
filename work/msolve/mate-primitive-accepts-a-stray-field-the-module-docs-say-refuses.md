---
id: mate-primitive-accepts-a-stray-field-the-module-docs-say-refuses
kind: issue
title: MatePrimitive carries no deny_unknown_fields, so a struct-variant field this build lacks is silently dropped
status: open
opened: 2026-09-15
---


Found by CENSUS-INERT-DENY's sweep of `#[serde(deny_unknown_fields)]`
(2026-09-15), from the other side: the sweep's question is which
attributes have no field to deny, and the complement is which
field-bearing wire types have no attribute.

`MatePrimitive` (`crates/editor-core/src/mate.rs`, ~`:162`) derives
`Deserialize` and carries `#[serde(rename_all = "snake_case")]` and no
`deny_unknown_fields`. Its `PlanarRest { offset }` variant is a struct
variant, so it has a named field to deny — and nothing denies it. Its
container `Alignment` (~`:237`) and its sibling `MateFrame` (~`:113`)
both carry the attribute, so this is the one hole in the mate wire
rather than a policy.

**Established by execution**, not by reading:

```rust
let p: MatePrimitive =
    serde_json::from_str(r#"{"planar_rest":{"offset":1.0,"stray":2.0}}"#).unwrap();
// PlanarRest { offset: 1.0 }
```

The stray key loads and is dropped. What makes that a defect rather
than a taste question is that `crates/editor-core/src/persist/mod.rs`'s
module docs rule the opposite for this whole format: *"A NEWER document
carrying a field this build lacks refuses … a stale reader must not
silently drop data."* A document written by a build whose `PlanarRest`
grew a second field loads here, silently, with that field gone — and
then saves back without it.

The fix is one line at the declaration. It is filed rather than taken
because it CHANGES WHAT A DOCUMENT ACCEPTS, which the sweep that found
it is explicitly not allowed to do (`docs/CENSUS-INERT-DENY-SPEC.md`,
§What this unit is NOT), and because the mate wire is msolve's.

**Scope of the scan that found it.** Every `struct`/`enum` under
`crates/editor-core/src/` deriving `Deserialize`, classified by whether
it has a named field anywhere and whether the attribute is on it. Six
candidates came back; five are not defects (`Doc` carries the attribute
above an interleaved `#[serde(bound(…))]`; `Annotation` and `Marker`
are stand-in producer types inside `meta`'s own `mod tests`; `Loaded`
and `Vis` derive no `Deserialize` at all). `MatePrimitive` is the
residue. **Crates other than `editor-core` were not scanned for this
complement**, so this is one confirmed instance and not a swept class.
