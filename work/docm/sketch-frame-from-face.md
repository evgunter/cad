---
id: sketch-frame-from-face
kind: issue
title: A sketch frame derived from a picked face: frozen snapshot or live reference?
status: closed
opened: 2026-09-03
github: 1374
closed: 2026-09-04
---

## What

**GitHub #1374's successor, and a different question than #1374 asked.**

GAUTH-1's spec wanted the add-profile tool to offer, when the selection
is a planar face, placing the new profile on that face's frame — the
`select::face_frame` door the mate tool uses, "frozen f64 in the
program's placement struct, stated in the form as a snapshot". The
spec's fallback was taken and #1374 filed.

**A profile's plane is a document node now, so the snapshot half of
that sentence no longer describes anything that exists.** #1374 says
"`SessionOp::AddProfile` carries `plane: SketchPlane`, so the
vocabulary already expresses arbitrary placement — the missing piece is
only the chrome affordance and the planarity gate." That is stale: the
op carries a `RecipeNodeId` naming a `Datum::Frame`, and the twelve
placement floats are gone.

So the arm is no longer "read a pose and freeze it into the sketch". It
is "mint a frame from a face", and there are two of those, which is the
fork below.

## The fork (Ev's)

**(a) A frozen frame.** Read `face_frame`'s pose at authoring time,
write it into a `Datum::Frame`'s twelve `Expr` literals, insert it, and
let the profile name it. One node, no new vocabulary, and it works
today.

The cost is that it re-introduces exactly the dead snapshot the sketch
frame deleted, one node further out. Change the body and the sketch
"on its top face" does not move — and now it does not move *visibly*,
because the frame is a feature in the tree that says it is at
z = 0.6 and is lying about why.

**(b) A derived frame.** A `Datum::Frame` variant whose pose is
computed at evaluation from a `StableName`. Live: the face's body is a
DAG input, the frame moves when the face moves, and it participates in
the memo and the content key like every other node.

The cost is a vocabulary-level design change — a second way for a
`Datum::Frame` to come into being, a naming reference inside a datum
(which no datum carries today), and the failure mode of a name that
stops resolving. It is the honest end state and it is not a GUI unit.

**Nothing should be built until this is answered**, because (a) and (b)
are not a smaller and larger version of one thing: (a) is a chrome
affordance and (b) is vocabulary, and shipping (a) first would put a
frozen-frame door in the GUI that (b) then has to deprecate.

## The other half #1374 named, unchanged

The offer is conditioned on the picked face being PLANAR. When #1374
was filed the interrogation vocabulary answered no such question; DM2
has since ruled a carrier-kind read a VALUE (a stored tag copied out,
not a numeric verdict — rule 1 now says NUMERIC predicates are what no
door decides), and DOCM-1 built the door:
`names::interrogate::face_carrier_kind` answers a face's `SurfaceKind`,
and `Datum::FaceFrame` refuses a non-planar face typed at evaluation
(DM1b). `face_frame` still answers a pose for any analytic carrier, and
a cylinder's pose is still its axis frame, so the chrome gates on the
tag read, not the pose. The second decision — offer the frame only for
a planar face, or for ANY face with wording that says which frame it
is — is the chrome's, and it survives the fork above whichever way it
goes.

## Where it stands

`gauth` is closed and owns nothing; this is unowned residue until a
program claims it. The fork wants an `[ev]` PR before any unit exists.

## Closed (2026-09-04)

Ruled derived, not frozen: `docs/DOCM-REFERENCES-DESIGN.md` DM1, with
the sense read beside the pose (DM1a), the typed non-planar refusal
(DM1b) and the carrier-kind read (DM2). The build is `DOCM-1`; the
chrome is CHROME's (`add-profile-mints-no-frame`,
`add-profile-placement-on-picked-face-frame`).
