---
id: add-parameter-form-authors-canonical-only
kind: issue
title: The add-parameter form authors only the canonical unit, though the kernel's written_length/written_angle doors are total
status: parked
opened: 2026-09-04
refs: [1776]
blocked_on: [viewer-session-god-module-split]
---


## Finding

**The add-parameter form can only declare a parameter in the canonical
unit, and it need not be.** A user who wants `base_r = 50 mm` gets
`base_r = 0.05 m` and no way to say otherwise — which is the same
defect CHROME unit 8 fixed for the panel ROW, still standing at the
one door that MINTS a declaration.

Where it stands:

- The form holds `new_param_dimension` and `new_param_value` and no
  unit at all (`crates/viewer/src/app.rs:364-372`, the form at `add_param_ui`,
  `:3001-3041`).
- Create mints through `props::doc_param`
  (`crates/viewer/src/app.rs:3075`), which routes every continuous
  value to `DocParam::continuous`
  (`crates/viewer/src/props.rs:587-592`) — the constructor whose
  `display_unit` is `UnitSym::canonical_for(dim)`
  (`crates/editor-core/src/doc.rs:176-183`).
- The field's own drag tick is the canonical one for the dimension,
  and says so (`app.rs:3027-3041`).

**No kernel change is needed.** `DocParam::written_length` and
`DocParam::written_angle` (`crates/editor-core/src/doc.rs:152`,
`:164`) are TOTAL authoring doors that take a `WrittenLength` /
`WrittenAngle` and produce a declaration whose unit measures its
dimension by construction. They are already the doors the panel's
sibling affordances use. This is the one authoring affordance
available before
`work/issues/doc-param-unit-edit-has-no-door.md` lands, and it is
independent of it: minting a declaration in millimetres is a door that
exists; CHANGING one afterwards is the door that does not.

## What it costs

1. A unit picker beside the value field — `length_picker` /
   `angle_picker` (`crates/viewer/src/app.rs`, the creation forms'
   control) already exist and already carry the rule that the unit is
   the picker's to say.
2. **One design call**, and this is the real content of the item.
   `props`' module contract is "every value that CROSSES this module
   is canonical" (`crates/viewer/src/props.rs:6-24` ("Canonical inside, written units outside")), and
   `props::doc_param` takes a `SlotValue`, which is canonical by that
   rule. A form authoring in millimetres has to get the notation to
   the declaration somehow: either `doc_param` grows a
   `unit: Option<UnitDef>` parameter (the shape `slot_edit` already
   has — `props.rs:563-581` — which is the precedent and probably the
   answer), or the form calls `DocParam::written_length` itself and
   bypasses `props`. The first keeps one door; the second puts a
   second declaration-minting spelling in the crate. Decide before
   implementing.

Note the `Scalar` case: `unit_options` is empty for `Scalar` and
`Count` (`props.rs:151-161`), so there is no picker to draw for those
two and the canonical declaration stays right for them.

## Why it is filed rather than taken

Disclosed as unasked scope by CHROME unit 8 (PR 1776,
`work/chrome/doc-params-carry-no-display-unit.md`), which took the
panel row and left the form.

## Home

CHROME. Everything above is `crates/viewer/src/*`; the kernel doors it
calls already exist.
