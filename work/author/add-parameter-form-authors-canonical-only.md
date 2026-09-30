---
id: add-parameter-form-authors-canonical-only
kind: issue
title: The add-parameter form authors only the canonical unit, though the kernel's written_length/written_angle doors are total
status: closed
opened: 2026-09-04
refs: [1776]
priority: P0
cost: E
branch: author/param-notation
rides_with: parameter-row-field-has-no-text-door
pr: 2957
closed: 2026-09-21
---


## Finding

**The add-parameter form can only declare a parameter in the canonical
unit, and it need not be.** A user who wants `base_r = 50 mm` gets
`base_r = 0.05 m` and no way to say otherwise — which is the same
defect CHROME unit 8 fixed for the panel ROW, still standing at the
one door that MINTS a declaration.

Where it stands:

- The form holds `new_param_dimension` and `new_param_value` and no
  unit at all — the drafts are `Drafts::new_param_dimension` /
  `Drafts::new_param_value` in `crates/viewer/src/drafts.rs`, and the
  form is `ViewerBehavior::add_param_ui` in
  `crates/viewer/src/pane/properties.rs`.
- Create mints through `props::doc_param` (called from
  `add_param_ui`'s `create.clicked()` arm), which routes every
  continuous value to `DocParam::continuous`
  (`crates/viewer/src/props.rs`) — the constructor whose
  `display_unit` is `UnitSym::canonical_for(dim)`
  (`crates/editor-core/src/doc.rs`).

**Struck 2026-09-15 — this bullet was FALSE**: *"The field's own drag
tick is the canonical one for the dimension, and says so
(`app.rs:3027-3041`)"*. It named a hand-picked constant as the form's
tick. `add_param_ui` derives the tick —
`FieldWriting::of(dimension, None).tick`, the panel's own rule — and
names `FIELD_DRAG_SPEED` only as the placeholder for "no dimension
picked yet", when Create is refused anyway. The bullet is struck
rather than re-pointed because there is no surviving subject to point
at: the defect it described was fixed, not moved. (It was never load-
bearing for this item, which is about the declared UNIT, not the tick;
`work/forms/drag-tick-has-three-homes.md` (CHROME's, and still there) is where the tick question
lives, and it now records `add_param_ui` as one of the two converted
sites.)

**No kernel change is needed.** `DocParam::written_length` and
`DocParam::written_angle` (`crates/editor-core/src/doc.rs`) are TOTAL
authoring doors that take a `WrittenLength` /
`WrittenAngle` and produce a declaration whose unit measures its
dimension by construction. They are already the doors the panel's
sibling affordances use. This is the one authoring affordance
available before
`work/edit/doc-param-unit-edit-has-no-door.md` lands, and it is
independent of it: minting a declaration in millimetres is a door that
exists; CHANGING one afterwards is the door that does not.

## What it costs

1. A unit picker beside the value field — `widgets::length_picker` /
   `widgets::angle_picker` (`crates/viewer/src/widgets.rs`, the
   creation forms' control) already exist and already carry the rule
   that the unit is the picker's to say.
2. **One design call**, and this is the real content of the item.
   `props`' module contract is "every value that CROSSES this module
   is canonical" (`crates/viewer/src/props.rs`, the module docs'
   opening section "Canonical inside, written units outside"), and
   `props::doc_param` takes a `SlotValue`, which is canonical by that
   rule. A form authoring in millimetres has to get the notation to
   the declaration somehow: either `doc_param` grows a
   `unit: Option<UnitDef>` parameter (the shape `props::slot_edit`
   already has, which is the precedent and probably the answer), or
   the form calls `DocParam::written_length` itself and bypasses
   `props`. The first keeps one door; the second puts a
   second declaration-minting spelling in the crate. Decide before
   implementing.

Note the `Scalar` case: `props::unit_options` returns an empty `Vec`
for `Scalar` and `Count` alike, so there is no picker to draw for those
two and the canonical declaration stays right for them.

## Why it is filed rather than taken

Disclosed as unasked scope by CHROME unit 8 (PR 1776,
`work/chrome/doc-params-carry-no-display-unit.md`), which took the
panel row and left the form.

## Home

CHROME. Everything above is `crates/viewer/src/*` — the form in
`pane/properties.rs`, the drafts in `drafts.rs`, the pickers in
`widgets.rs`, the canonical-only mint in `props.rs`; the kernel doors
it calls already exist.

## Un-parked — the trigger fired (2026-09-04)

`viewer-session-god-module-split` closed on 2026-09-04, so this row's
only blocker is gone and the row is dispatchable. Un-parked here, from
VIEW's PR #1857, rather than by CHROME: on Ev's ruling there, `work.py
lint` now REFUSES a `parked` row whose every blocker is closed, and a
program cannot un-park another program's rows in the PR that closes
their trigger — `work/README.md`'s one-file-one-item rule makes that a
merge conflict by design.

## Re-pointed by subject, one bullet struck (2026-09-15, `chrome/citation-repoint`)

Every `app.rs` citation in this row was a pre-#1830 address. All of
them are re-derived above by subject name, with no line number written
(`docs/prompts/implementer-discipline.md` §7); the `props.rs` and
`doc.rs` numbers are dropped for the same reason rather than refreshed.

**One supporting bullet was struck, not re-pointed** — the drag-tick
one, struck in place above with the reason. The item's HEAD claim is
unaffected and still true: `add_param_ui` mints through
`props::doc_param` → `DocParam::continuous`, whose `display_unit` is
`UnitSym::canonical_for(dim)`, so the form can still declare only in
the canonical unit. Read at `385c01b3`.

`work/issues/doc-param-unit-edit-has-no-door.md` re-pointed to
`work/edit/…` — EDIT claimed the item; it is open.

## Dispatched 2026-09-21 — riding AUTH-2

`docs/AUTH-2-SPEC.md`, branch `author/param-notation`, as the CREATE
half of one notation unit; `parameter-row-field-has-no-text-door` is
the carrier and the EDIT half. Filed as a rides-along rather than
folded in: this row's own finding — the form mints through
`props::doc_param` into `DocParam::continuous` where
`written_length`/`written_angle` are total — is a separate fact with
its own evidence, and closing the carrier is not closing it.

## Closed 2026-09-21 — PR 2957 merged (`8352822c2`), riding AUTH-2

**The add-parameter form authors the unit.** `props::doc_param` takes
an `Option<UnitDef>` and mints through `DocParam::written_length` /
`written_angle` rather than `DocParam::continuous`, so `base_r = 50 mm`
is a declaration whose `display_unit` is `mm` — asserted through a
save/load round trip in `doc_io`, because a notation that does not
survive the wire is not a notation.

**No double scaling**, which was the hazard this row shared with its
carrier: `WrittenLength::canonical_in` multiplies by nothing, and the
assertions are written in mm against a 1000× factor so a second
multiply would read `5e-5` rather than `0.05`.

**A latent collision closed in passing**: `widgets::pick_unit`
hard-coded its id salt as `"creation_unit"`, so a parameter named
`add_param` would have collided with the add-parameter form's own
length picker. `pick_unit` now takes a prefix and the collision is
impossible. Found because this row's fix routed `param_unit_ui`
through that function instead of hand-rolling a third copy of it.

Closed as a rides-along with its carrier
(`parameter-row-field-has-no-text-door`), whose closure section
carries the notation design's full history.
