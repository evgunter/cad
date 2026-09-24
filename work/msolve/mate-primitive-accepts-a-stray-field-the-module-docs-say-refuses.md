---
id: mate-primitive-accepts-a-stray-field-the-module-docs-say-refuses
kind: issue
title: MatePrimitive carries no deny_unknown_fields, so a struct-variant field this build lacks is silently dropped
status: closed
opened: 2026-09-15
closed: 2026-09-19
pr: 2885
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

## Closed (2026-09-19, PR 2885)

Fixed by MSOLVE-7: `MatePrimitive` carries
`#[serde(deny_unknown_fields)]`; the census's `ATTRIBUTE_SITES_TODAY`
entry for `mate.rs` re-baselined 2 → 3, its sibling row (`every_deny_
unknown_fields_attribute_has_a_named_field_to_deny`) green on the
attribute. Pinned through the load door by `msolve7_member_residue::
a4_a_stray_key_on_a_planar_rest_refuses_at_the_load_door`
(`persist::load` refuses `PersistError::Unreadable` naming the field)
and `a4_the_same_alignment_without_the_key_loads` (the stray key
before `offset`, after it, and spelled with the alignment's own
`clocking`); established red without the attribute before it landed.
Every checked-in `.pncad` still loads
(`a4_every_checked_in_document_loads_and_none_carries_a_mate` — none
of the four carries a mate, so the corpus proves nothing is refused,
not that the attribute is reached) and re-saves identically
(`msolve6_part_extent::c5_every_checked_in_document_loads_with_no_store_and_re_saves_identically`).
The complement sweep across other crates stays undone, as the row
disclosed and the spec keeps out of scope.
