---
id: the-profile-editors-naming-and-offer-doors-wait-on-held-edits
kind: issue
title: The profile editor's naming and offer doors are withheld while the draft holds edits
status: open
opened: 2026-11-02
---


Found by the held-expression unit's sweep (branch
`forms/held-expression`), which moved an expression typed into a
profile editor field into the draft, so it no longer waits on held
edits.

The other two doors under a held field still wait on them, and the
way they wait differs from the refusal that unit removed: they are not
drawn at all. `ViewerBehavior::held_slot_doors_ui`
(`crates/viewer/src/pane/properties.rs`) draws the `name` buttons and
the offer only while the draft is `untouched`, because each emits a
session op that moves the program the draft was loaded from:

- **Accepting an offer of a named variable**
  (`ViewerBehavior::offer_ui`, `SessionOp::SetSlotVariable` with no
  name) makes the slot read that variable. That is the same shape as
  the expression: a formula reading the variable, written into the
  held step (`ProfileEdit::write_text`'s path, through
  `HeldStep::with_arg`), would carry it into the draft.
- **Naming the slot's unnamed variable** (`name_button` →
  `SessionOp::RenameVar`, or `SetSlotVariable` with a name from an
  offer) declares a name in the document. The draft holds a program,
  not declarations, so this is not that shape: it needs either a
  document edit beside the program edit, or a ruling that naming
  waits.

A person who has moved one corner therefore cannot name another
corner's value, or share it with an equal one, without committing or
reverting first; and the doors vanish rather than saying why.

This touches the profile unit (the editor commits each change live,
Ev on `[ev]` PR 6388): with no held state between changes, the
`untouched` gate may stop mattering, which may settle the row.

## 2026-11-02 — the editor commits live (profile unit)

Evidence from branch `forms/profile-unit`. Each change now lands as it
is made whenever the edited program passes the edit door
(`pane::profile::edit_door_ui`), and the draft reloads untouched from
the program that landed. So `untouched` is false only while the editor
holds a draft the door refused (`ProfileEdit::refused`), or for the
frames of a drag before its release. Between ordinary changes, the
naming and offer doors are drawn. What remains of this row is the
refused-and-held case: a person whose chain is half-closed still
cannot name another corner's value until the draft lands or is
discarded (`DISCARD`, "Discard changes"), and the doors vanish rather
than saying why.
