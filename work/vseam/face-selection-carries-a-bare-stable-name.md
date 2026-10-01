---
id: face-selection-carries-a-bare-stable-name
kind: issue
title: FaceSelection::name is a bare StableName, so the mate tool re-checks what its door decided
status: open
opened: 2026-09-17
priority: P1
cost: D
---


Filed by EDIT's `edit/mate-head-kind` unit, which made a mate head's
entity kind a TYPE: a head is `editor_core::SitedFace` over
`editor_core::FaceName`, whose one constructor is the only way a face
name is made.

**The viewer's picked face is not that type.**
`crates/viewer/src/session/select.rs`'s `FaceSelection` carries
`pub name: StableName` — a face by its door's own rule (the picking
door refuses `SelectionRefusal::NotAFace` before a selection exists)
and not by its type, with every field public. So `matetool.rs`'s
`picked_member` calls `FaceName::new` on a name the picker already
proved is a face, and has to say what it does when the constructor
refuses.

**What it does today**: it answers a typed arm,
`MateToolError::PickIsNotAFace { side, refusal }`, in every build —
the head constructor's own `NotAFaceName` carried rather than
restated, so the sentence a user reads names the kind. It is not a
`debug_assert`: the rule lives in the picking DOOR and the value is a
`pub`-field struct any caller of `MateTool::pick` can build, so a
release build must refuse it rather than reach a head the type
forbids. Rows:
`crates/viewer/tests/rv_matehead_probes.rs`'s two.

**The fix is the same move one layer out**: `FaceSelection::name:
FaceName`, made where the pick is made (the picking door has the
kind in hand and already refuses the alternative), and every consumer
reading it through `Deref`/`AsRef`. `picked_member` then takes the
name it is given.

**What taking it removes**: the `FaceName::new` call in
`picked_member`, the `MateToolError::PickIsNotAFace` arm and its
rendering, and the two rows above — one refusal arm fewer in the
tool's closed enum, because the state it answers stops being
constructible instead of being answered.

Cited: `crates/viewer/src/matetool.rs`'s `picked_member` and
`MateToolError::PickIsNotAFace`,
`crates/viewer/src/session/select.rs`'s `FaceSelection`.
