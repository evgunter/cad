---
id: face-pick-cannot-name-which-face
kind: issue
title: The add-datum form's face pick names a body scope, and no label in the tree can name WHICH face
status: open
opened: 2026-09-21
priority: P1
cost: D
refs: [add-profile-mints-no-frame, 2955]
---

## What

`add_profile_mints_no_frame`'s half 2 has three sites. AUTH-3 closed
the first two — the add-profile form's frame picker and the feature
tree's rows now say which frame a node is, from one home
(`tree::frame_pose`) — and did **not** close the third, because the
third is not the same defect and does not have the same answer.

The third is the add-datum form's held FACE pick
(`crates/viewer/src/pane/create.rs`, `datum_face_frame_rows`), which
renders as `BlendTarget::of_face(face).to_string()` and therefore reads
the same for **all six faces of a box**. AUTH-1's reviewer filed that
as NOTE-6.

## Why AUTH-3 did not fix it at the home the spec named

`docs/AUTH-3-SPEC.md` says the third site "now has ONE home — fix that
home and this site and the blend sites move together." That is true of
the NODE half of the sentence and false of the face half, and the
face half is the whole complaint.

- `BlendTarget` is deliberately a BODY SCOPE and nothing else — two
  fields, `node` and `body`, with a doc saying so and a destructure in
  its `Display` whose comment exists to make a third field a compile
  error. A blend's accumulator opens on a body; it does not want a
  face and could not use one.
- So putting face identity into `Display for BlendTarget` would give
  every blend refusal a sentence about something the blend is not
  about, to fix one caller. AUTH-3 routed only the NODE half through
  the crate's one spelling (`tree::node_number`), which is what the
  blend sites genuinely share.
- The face's own identity is its role path, and the tree has no
  renderable form of one: `RoleSeg` carries no `Display` at all
  (`crates/viewer/src/idpass.rs` says so in as many words — "the path
  rides as `Debug` because `RoleSeg` has no `Display`"), and
  `Display for StableName` deliberately renders the kind and the
  minting node and stops, with a comment ruling that "the role path is
  a derivation, not something a person reads mid-sentence, so prose
  never renders it".

## The two candidate answers, neither cheap

1. **A renderable face descriptor in the names layer** — a prose noun
   for a `RoleSeg` (`top cap`, `side wall 3`) beside the typed path.
   That reverses the `Display for StableName` comment above, which is
   a decision in `editor-core` and not a viewer lane's to take alone.

2. **Read the pose off the landed evaluation.** A face PICK is unlike
   a frame NODE here: the pick was made against the landed run's
   picture, so reading that same run is not a stale guess — it is the
   only non-guess. `names::interrogate::face_frame` already answers
   "where is this face", and `add_datum_ui` already resolves the pick
   once through `face_frame_seat` for the button's gate. What it
   costs: the outward normal has exactly one mint,
   `geom_brep::OutwardNormal::from_chart`, which the `pncad` facade
   does not re-export, and hand-negating `Pose::axis` by `Pose::sense`
   at the form would be a second spelling of the type that exists to
   prevent it. So this answer starts with a facade re-export.

Answer 2 is the smaller of the two and does not reverse anyone's
ruling. It is filed rather than taken because it needs a facade
change and a decision about what a pick readout may read, which is
the same question AUTH-3 answered for NODE labels and deliberately
did not answer for PICK readouts.

## The other half of NOTE-6

The reviewer also noted the pick "stays on screen after the selection
is cleared, so an author can commit against a pick nothing in the
viewport is showing". The latch is deliberate and documented
(`datum_face_frame_rows`: it is what lets an author pick a face, type
a spin and click the tree without losing the pick), so the fix is not
to drop it — it is to make the held pick VISIBLE, which is viewport
marks rather than form text. Recorded here so it is not lost with the
half above.
