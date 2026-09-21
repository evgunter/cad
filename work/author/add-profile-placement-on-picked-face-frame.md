---
id: add-profile-placement-on-picked-face-frame
kind: issue
title: Nothing in the viewer can mint a Datum::FaceFrame, so a profile still cannot be placed on a picked face
status: dispatched
opened: 2026-08-31
github: 1374
priority: P0
cost: H
branch: author/face-frame-seat
---

## Premise re-cut (2026-09-15, `chrome/citation-repoint`)

This row was migrated from GitHub issue 1374 (opened 2026-08-31, 0
comments) and its body stated two things the tree has since falsified.
Both are corrected here; the row stays **open**, because the affordance
it asks for still does not exist — it is just missing for a different
reason than the one it was filed with.

**Falsified 1: "the shipped tool authors on world XY only."** It does
not. `SessionOp::AddProfile` carries `plane: RecipeNodeId` — a PICK of
a frame node that already exists, not a frozen `SketchPlane<f64>` the
form fills in — and `ViewerBehavior::add_profile_ui`
(`crates/viewer/src/pane/create.rs`) draws a ComboBox over
`sketch::frames`, which admits **`Datum::Frame` and `Datum::FaceFrame`
alike** (`crates/viewer/src/sketch.rs`). `session::refuse`'s
frame-kind check admits both the same way. So the whole placement path
— op, form, refusal — already takes a face frame the moment one exists
in the document.

**Falsified 2: "the interrogation vocabulary answers no 'is this face
planar' question."** It does.
`editor_core::names::interrogate::face_carrier_kind` answers a face's
`SurfaceKind` (over `topo::readback::face_carrier_kind`, which is where
the tag read itself lives), and it is re-exported through
`pncad::select`, the module the viewer already imports `face_frame`
from in `crates/viewer/src/matetool.rs`. The door is reachable from the
viewer today with no kernel change and no design revision. The two
"what remains" shapes the old body offered — *"a deliberate revision of
the interrogation posture"* or *"offer the frame for ANY picked face"*
— were both predicated on that door's absence and neither is the
question any more.

## What is ACTUALLY missing

**Nothing in the viewer can mint a `Datum::FaceFrame`.** The authoring
vocabulary is `DatumSpec` in `crates/viewer/src/session/author.rs`,
which has five seats — `Plane`, `Axis`, `Point`, `Frame`,
`AxisInPlane` — and `session::author::datum_node` lowers exactly those
five to `Datum`. There is no `FaceFrame` arm. The form above it,
`ViewerBehavior::add_datum_ui` (`crates/viewer/src/pane/create.rs`),
offers `DatumKindChoice` = `Plane` / `Frame` / `Axis` /
`AxisInPlane` / `Point`, and `forms.rs`'s `partial_mirror!` between
the two does not mention `FaceFrame` at all, because there is no
variant to mirror.

So a face frame is a node the DOCUMENT understands, the tree labels
(`tree.rs`: `"Datum frame (on face)"`), the profile form would accept
and the refusal path admits — and that no chrome can create. The whole
gap is one seat in `DatumSpec` plus the form arm that fills it.

**What such a seat has to carry**, from `Datum::FaceFrame` in
`crates/editor-core/src/node.rs`: `at: RecipeNodeId` (the body-denoting
node the face is read out of, a DAG input exactly as
`Datum::AxisInPlane::plane` is), `face: StableName` (a frozen name
resolved through `at`'s value), and `spin: Expr` (`SlotId::Spin` — the
rotation of sketch +x about the outward normal, which is the whole of
what an author chooses, since origin and normal are read off the face).
That is two PICKS and one field, so it is the `AxisInPlane` shape and
not the `Plane` one. The add-datum form authors `AxisInPlane` with a
frame picker inside the form (`frame_picker` in
`crates/viewer/src/pane/create.rs`); a face frame's body and face
picks are the same decision one step wider, since a face is not a node
a combo can list.

**The planarity gate is a tag read, not a verdict.** `face_frame`
answers a pose for any analytic carrier and a cylinder's pose is its
axis frame, so the chrome's gate is
`face_carrier_kind(..)? == SurfaceKind::Plane`; `Datum::FaceFrame`
itself refuses a non-planar face at evaluation (DM1b), so the chrome
gate is an affordance, not the safety.

## Two nodes in one gesture is NOT a constraint — the door ships

Stated here because both this row and its sibling have argued the
opposite, and because a RATIFIED design page settles it and names both
rows by id. `crates/editor-core/REFERENCES.md`, DM1's
chrome-consequence bullet:

> The chrome consequences are CHROME's builds
> (`add-profile-mints-no-frame`,
> `add-profile-placement-on-picked-face-frame`): "on a new XY frame"
> is two inserts in one committed action (`commit_action`); "on this
> face" mints one `FaceFrame` and one profile the same way.

And the door is shipped: `DocSession::commit_action`
(`crates/viewer/src/session.rs`) takes a `Vec<DocEdit>`, applies each
to the value the last produced, records the whole run as one history
state — *"one user action is one undo"* — and is **all-or-nothing**, a
refusal anywhere leaving the session on the document it started from.
`delete_node`'s cascade already goes through it.

So "one submit, one committed edit" is not the invariant; **one submit,
one undo** is, and `commit_action` is how a multi-node gesture keeps
it. Nothing about the face-frame arm is blocked on a commit-door
decision. What is left is the missing seat above, and nothing else.

## Relation to the sibling row

`work/author/add-profile-mints-no-frame.md` is the nearer of the two
halves and the further of the other. Its half 1 — an empty document
cannot draw a sketch, because minting a world-XY frame means leaving
the form — wants a node kind the chrome **can already mint**
(`add_datum_ui` offers `Frame`); it is an affordance-routing problem,
and with `commit_action` above it is close to mechanical. This row
wants a node kind **no chrome can mint at all**, which needs a new
`DatumSpec` seat, a two-pick form shape and a planarity gate. Removing
the false one-submit constraint pushes the two apart rather than
together: it was the only thing making them look like one problem.

Where they DO converge is that row's half 2, the picker and tree
labels. `tree.rs` distinguishes the two frame KINDS (`"Datum frame"`
against `"Datum frame (on face)"`) but neither label says WHICH frame,
and `add_profile_ui`'s ComboBox reads `format!("feature {}", id.0)`
for every frame regardless of kind. A face frame's honest label is the
face it sits on, so this row's form would need exactly the label work
that row asks for. Whoever takes either should read the other for that
reason, not for the commit door.

## Home

AUTHOR (this row sits on AUTHOR's slate; it came here from CHROME by
the 2026-09-20 priority-seam cut, `work/author/log.md`, and the `##
Home` section it was migrated with said `work/issues/`, which stopped
being true when CHROME claimed it). Everything above is `crates/viewer/src/`: the seat in
`session/author.rs`, the choice in `forms.rs`, the form in
`pane/create.rs`. The kernel doors — `Datum::FaceFrame`,
`face_carrier_kind`, `face_frame` — all exist.

## Provenance

GAUTH-1 spec, unit item 3 of GAUTH's plan (GAUTH is closed and its plan
left the tracker with the program, recoverable at the SHA
`docs/DOC-LEDGER.md` sweep 5 names), wanted the add-profile tool to
offer, when the current selection is a planar face, placing the new
profile on that face's frame. The spec's fallback was taken and this is
the scheduled follow-up (protocol v5 — a filed issue, not a silent
narrowing).

## Dispatched 2026-09-21 — AUTH-1

`docs/AUTH-1-SPEC.md`, branch `author/face-frame-seat`. The spec takes
this row's scope EXACTLY as the body above cuts it — the sixth
`DatumSpec` seat, its `DatumKindChoice` offering and mirror entry, the
draft seats, a planarity gate sited where a test can reach it, and one
`SessionOp::AddDatum`. The sibling's picker labels and its
`commit_action` gesture stay with
`work/author/add-profile-mints-no-frame.md`, which runs next; after
AUTH-1 a face frame is authored in the add-datum form and drawn on in
the add-profile form, which is a two-form trip and the known residue.
