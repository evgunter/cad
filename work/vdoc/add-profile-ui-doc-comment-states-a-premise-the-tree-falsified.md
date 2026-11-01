---
id: add-profile-ui-doc-comment-states-a-premise-the-tree-falsified
kind: issue
title: add_profile_ui's doc comment says the form authors on world XY and that no planarity door exists; both are false
status: closed
opened: 2026-09-15
priority: P4
cost: E
pr: 2955
closed: 2026-09-21
---


## Finding

`ViewerBehavior::add_profile_ui`'s doc comment
(`crates/viewer/src/pane/create.rs`) opens with two sentences the code
ten lines below it contradicts:

> The add-profile form: a template shape with Length fields, one
> [`SessionOp::AddProfile`] on commit — **on the world XY plane, which
> the form says.**
>
> […] the face-frame placement arm is deferred as a filed issue — **the
> interrogation vocabulary deliberately answers no "is this face
> planar" verdict for it to gate on.**

Both are false at `385c01b3`:

1. **The form does not author on world XY and does not say so.** Its
   own body comment, immediately below, says the opposite: *"The form
   used to say 'on the world XY plane' and mean a constant; a
   profile's plane is a node now, so this names one."* The form draws
   a ComboBox over `sketch::frames` and pushes
   `SessionOp::AddProfile { plane, loops }` with
   `plane: RecipeNodeId`. The summary was not updated when the op's
   `plane` stopped being a `SketchPlane<f64>`; the two comments in one
   function now disagree with each other.
2. **The interrogation vocabulary does answer it.**
   `editor_core::names::interrogate::face_carrier_kind` reads a face's
   `SurfaceKind` (over `topo::readback::face_carrier_kind`) and is
   re-exported through `pncad::select`, which this crate already
   imports `face_frame` from in `crates/viewer/src/matetool.rs`.

This matters beyond tidiness because the sentence is a **design
claim** — it tells a reader the kernel deliberately declines to answer
a question it now answers, and it is the reason the follow-up issue
was filed with the premise it had.

## Where the same premise was already corrected

`work/author/add-profile-placement-on-picked-face-frame.md` carried
both falsehoods and was re-cut on 2026-09-15
(`chrome/citation-repoint`). (The row was CHROME's then and is AUTHOR's now, by the
2026-09-20 priority-seam cut.) That unit's fence was `work/chrome/*.md`
and touched no source file, so the doc comment was left for this row.

## Shape

Rewrite the summary to state the invariant
(`docs/prompts/implementer-discipline.md` §4: the invariant, not the
history) — the form names an existing frame node and refuses until one
is picked — and delete the planarity sentence, which is now only an
argument about how the code used to work. The remaining gap, that no
chrome can mint a `Datum::FaceFrame`, is the re-cut row above and is
what the deferral note should point at if it keeps one.

## Home

VDOC — this row sits on VDOC's slate, which is the claim; `## Home`
said CHROME because that is where it was filed before the 2026-09-20
cut sent the prose rows here. One file,
`crates/viewer/src/pane/create.rs`.

## Discharged by AUTHOR's AUTH-1, in review (2026-09-21)

**Both sentences are gone.** AUTH-1
(`work/author/add-profile-placement-on-picked-face-frame.md`, PR 2955,
branch `author/face-frame-seat`) rewrote `add_profile_ui`'s doc
comment as part of building the face-frame seat, because this row's
second falsehood — *"the interrogation vocabulary deliberately answers
no 'is this face planar' verdict"* — is a claim that unit falsifies by
existing, and its first is the same premise re-cut one door over. The
replacement states the invariant per implementer-discipline §4: the
plane is a pick of a frame node that exists, the picker lists both
frame kinds, and the residue is the one-gesture version.

Recorded here rather than left for VDOC to re-derive. **2955 merged
2026-09-21 (`2cf83b500`), so this row is CLOSED** — the condition it
was set to `review` on has fired and both sentences are gone from the
tree.
**The seam is announced on VDOC's log** — AUTHOR touched VDOC's row,
not just its ground.

This row's `## Shape` asked for precisely what landed, including its
last sentence: *"The remaining gap, that no chrome can mint a
`Datum::FaceFrame`, is the re-cut row above and is what the deferral
note should point at if it keeps one."* AUTH-1 is that gap closing, so
the rewritten comment points at the two-form residue instead.
