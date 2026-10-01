---
id: linalg-escalations-offer-a-declaration-the-door-cannot-take
kind: issue
title: linalg: a direction or frame escalation offers the declare menu at frame and mate doors, which take no declaration
status: closed
opened: 2026-09-29
priority: P2
cost: E
closed: 2026-10-01
---


(CHROME, from the triage in
`work/chrome/escalations-forwarded-whole-are-untriaged-for-a-declarations-object.md`.)

## What

Two frame-side refusals offer "declare the coincidence" at doors whose
declaration, if they take one, cannot name the decision:

1. `UnitVec3Error::Escalated` (`crates/geom-core/src/linalg/unit_vec.rs`,
   the variant near :143, its `Display` near :165) renders "a direction
   vector's length is indeterminate: {source}", which is the whole
   `Indeterminate`. The viewer never shows it, because editor-core
   re-wraps it (`eval::wire::refusal` near :868, into
   `NodeErrorKind::Escalated`). But `OrthoFrameError`'s `Display`
   (`crates/geom-core/src/linalg/ortho_frame.rs` near :169) forwards it
   whole, and Python raises that text as `FrameError`
   (`crates/pncad-py/src/py/place.rs`, `ortho_frame_err`). A frame
   witness takes no declaration.
2. `FrameError::Degenerate` (`crates/geom-core/src/linalg/frame.rs`,
   `Display` near :331) renders the payload, then
   `Recourse: {COINCIDENCE_RECOURSE}`. The mate solve raises it for an
   in-band aim direction (`crates/editor-core/src/mate/solve.rs` near
   :608), and so do the frame ladder constructors. A mate does carry a
   declaration: `Node::Mate` (`crates/editor-core/src/node.rs` near
   :2417) declares a face pair's contact class. But that declaration
   has no object for a direction's length, and a frame takes none. The field-type greps in the CHROME row
   miss this one: its field is `indeterminate: Option<Indeterminate>`.

## Repair shape

Render the payload with a subject ("whether a direction has any
length", which is the words `NodeErrorKind::Escalated` already uses) and
`geom_core::NO_DECLARATION_RECOURSE` or the site's own lever. See
`sweep::blend::BlendError::Escalated`'s `Display` for the shape.

## Closed (2026-10-01)

Closed by #3710.

`UnitVec3Error::Escalated` and both arms of `FrameError::Degenerate`
now render three things: a subject (the new
`geom_core::DIRECTION_LENGTH_SUBJECT`, one literal where there used to
be four), the payload, and `NO_DECLARATION_RECOURSE`. They no longer
render the declare menu.

The review confirmed that no door taking a declaration renders these
variants.
