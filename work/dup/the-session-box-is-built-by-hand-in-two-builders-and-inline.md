---
id: the-session-box-is-built-by-hand-in-two-builders-and-inline
kind: issue
title: A box through the session (rectangle template, then extrude) has two private builders and more inline copies
status: open
opened: 2026-09-25
priority: P4
cost: E
---


## Finding

- **Where**, measured at `5fe3dff36` plus the insert-door fix pass:
  a box authored through the SESSION — `SessionOp::AddProfile` with
  the chrome's `ProfileShape::Rectangle` template, then
  `SessionOp::AddExtrude` over it — is a named builder twice and
  inline elsewhere:
  - private builders: `combine_ops.rs` `boxed(session, [w, d, h])` and
    `blend_authoring.rs` `boxed(session, side)` (the cube case of the
    first), each preceded by `common::xy_frame_in`;
  - `story_assembly.rs` `author_box_part` (the same pair, then saved
    as a part);
  - inline, the template followed by an extrude within nine to
    eighteen lines: `combine_ops.rs`
    (`a_duplicate_is_not_measured_off_a_picture_older_than_the_document`),
    `creation_ops.rs` (`a_bracket_block_authors_saves_reloads_and_undoes`,
    `the_op_vocabulary_exceeds_the_chrome_templates_and_that_works`,
    `a_boss_is_authored_on_a_picked_face`), `docm1_face_frame.rs`
    (`a_pick_whose_node_an_undo_took_away_is_refused_as_gone`) and
    `story_authoring.rs`'s rook, four times.
  The home would be `common` beside `xy_frame_in` / `instance_in`: a
  `box_in(session, plane, [w, d, h]) -> (profile, extrude)`. The
  story rows may reasonably stay longhand — a story suite reads as the
  user's sequence of gestures, and whether a door keeps that readable
  is this row's per-site question.
- **Not members**: the eight other template sites draw a profile and
  stop (seat-kind rows, refusal rows, `creation_ops`' template-shape
  oracle, `profile_edit`'s `authored`).
- **Importance**: low. No oracle rides on the builder.
- **Instrument, and its blind spot**: `git grep -n
  'ProfileShape::Rectangle' -- crates/viewer/tests` (19 sites), each
  checked for an `AddExtrude` within the next twenty lines. A box
  whose extrude is further away, whose profile is a `Path` or
  document-door rectangle, or that is authored through the document
  door (`common::rectangle` + `Node::Extrude`, e.g. `common::asm`'s
  `box_part`) is outside this instrument; the document-door boxes are
  the brick rows' class, not this one.
- **Raised by**: the S-DUP insert-door unit's fix pass, 2026-09-24,
  naming the rectangle census's blind spots.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` with no single ground-owner; the class is one construction
spelled about a dozen times, which is this program's charter.
