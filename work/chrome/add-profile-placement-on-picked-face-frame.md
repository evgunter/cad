---
id: add-profile-placement-on-picked-face-frame
kind: issue
title: GAUTH-1 residue - add-profile placement on a picked planar face's frame
status: open
opened: 2026-08-31
github: 1374
---

## From GitHub issue 1374

Opened 2026-08-31; 0 comments.

The GAUTH-1 spec (unit item 3 of GAUTH's plan — GAUTH is closed and its plan left the tracker with the program, recoverable at `docs/DOC-LEDGER.md` sweep 5's SHA) wants the add-profile tool to offer, when the current selection is a planar face, placing the new profile on that face's frame — the `select::face_frame` door the mate tool uses, frozen f64 in the program's placement struct, stated in the form as a snapshot.

The spec's own fallback was taken: the shipped tool authors on world XY only, and this issue is the scheduled follow-up (protocol v5 — a filed issue, not a silent narrowing).

Why the arm proved deep rather than a form field: the offer is conditioned on the picked face being PLANAR, and the interrogation vocabulary deliberately answers no such verdict — `editor-core::names::interrogate`'s module docs state "values never verdicts (no door answers 'is this face planar')". `face_frame` happily answers a pose for any analytic carrier, but a cylinder's pose is its axis frame, and placing a sketch on that silently would be exactly the wrong-surprise this codebase refuses elsewhere. So the honest implementation needs either

1. a deliberate revision of the interrogation posture (a carrier-kind or planarity door at the layer 2/3 boundary), discussed as a design change rather than slipped in from a GUI unit, or
2. offering the frame for ANY picked face with wording that says what frame it actually is — a different UX decision than the spec's, which somebody should make on purpose.

What already exists on the op side: `SessionOp::AddProfile` carries `plane: SketchPlane<f64>` (the program's own frozen-f64 placement struct), so the vocabulary already expresses arbitrary placement — the missing piece is only the chrome affordance and the planarity gate. A headless consumer can author on any plane today.

## Home

GAUTH's closing entry names this issue as its residue, and both GAUTH and GUI are closed programs that may hold only closed items, so it lands in `work/issues/`.
