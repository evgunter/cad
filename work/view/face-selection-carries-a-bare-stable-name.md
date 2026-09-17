---
id: face-selection-carries-a-bare-stable-name
kind: issue
title: FaceSelection::name is a bare StableName, so the mate tool re-checks what its door decided
status: open
opened: 2026-09-17
---


Filed by EDIT's `edit/mate-head-kind` unit, which made a mate head's
entity kind a TYPE: a head is `editor_core::SitedFace` over
`editor_core::FaceName`, whose one constructor is the only way a face
name is made.

**The viewer's picked face is not that type.**
`crates/viewer/src/session/select.rs`'s `FaceSelection` carries
`pub name: StableName` — a face by its door's own rule (the picking
door refuses `SelectionRefusal::NotAFace` before a selection exists)
and not by its type. So `matetool.rs`'s `picked_member` now calls
`FaceName::new` on a name the picker already proved is a face, and has
to say what it does when the constructor refuses: it asserts in debug
and answers `NotAnInstancePick` in release, which is the arm's word
stretched to cover a case it was not written for.

**The fix is the same move one layer out**: `FaceSelection::name:
FaceName`, made where the pick is made (the picking door has the
kind in hand and already refuses the alternative), and every consumer
reading it through `Deref`/`AsRef`. `picked_member` then takes the
name it is given, and the stretched arm goes away rather than being
re-worded.

Cited: `crates/viewer/src/matetool.rs`'s `picked_member` (the
`debug_assert!` beside `FaceName::new`), `crates/viewer/src/session/select.rs`'s
`FaceSelection`.
