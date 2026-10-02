---
id: drawing-on-a-picked-face-is-a-two-form-trip
kind: issue
title: Drawing a profile on a picked face takes two forms: mint the face frame in add-datum, then pick it in add-profile
status: open
opened: 2026-10-02
priority: P2
cost: M
---

Found at AUTHOR's exit sweep: a residue whose carrier closed without
carrying it.

**What happens.** To draw a profile on a face of a body, a person
opens the add-datum form, picks the face, mints a `Datum::FaceFrame`,
then opens the add-profile form and picks that frame. The add-profile
form's plane picker (`ProfilePlane`, `crates/viewer/src/session/author.rs`
~:126) offers `Existing(node)` and `NewXy` — a world xy frame minted in
the same committed action, one submit and one undo — and nothing that
mints a face's frame the same way. So the most ordinary placement in
a CAD tool is two forms and two undos where the world plane is one.

**Why it is unscheduled.** AUTH-1 (PR 2955) built the face-frame seat
and named the two-form trip as residue carried by
`add-profile-mints-no-frame`. That row closed with AUTH-3 (PR 3023),
which added `NewXy` and the frame labels but not a face-frame mint in
the add-profile form, and its Closed section does not mention the
trip. Two code comments still say a closed row carries it:
`crates/viewer/src/pane/create.rs` (`add_profile_ui`'s doc, "Drawing on
a picked FACE is still two gestures") and
`crates/viewer/tests/creation_ops.rs` (`a_boss_is_authored_on_a_picked_face`'s
doc); both now point here.

**What would close it.** A third `ProfilePlane` arm — a face frame
minted by the same action from a held face pick — so the add-profile
form commits the `FaceFrame` and the profile in one `commit_action`
(one undo), the way `NewXy` does. The add-datum form's planarity gate
and `DatumSpec` face-frame seat are the one home for what a face frame
needs; the new arm reads them rather than restating them (every AUTHOR
unit minted such a copy). AUTH-1's spec weighed and kept the two forms
apart on the old "one submit, one committed edit" premise, which
`commit_action` retired ("one submit, one undo" is the invariant);
recover it with `git show <recovery-sha>:docs/AUTH-1-SPEC.md`.
