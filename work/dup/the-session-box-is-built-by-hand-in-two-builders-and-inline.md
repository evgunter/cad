---
id: the-session-box-is-built-by-hand-in-two-builders-and-inline
kind: issue
title: A box through the session (rectangle template, then extrude) has two private builders and more inline copies
status: closed
closed: 2026-09-26
opened: 2026-09-25
priority: P4
cost: E
pr: 3285
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

## Closed

Folded by the S-DUP `viewer-drain` lane, 2026-09-26, cut from
`0c1932667`.

- **Census re-taken**: `git grep -n 'ProfileShape::Rectangle'` with no
  path argument — 19 test sites, all under `crates/viewer/tests/`, plus
  the two `src/` sites that define the template. Each read.
- **The homes**, in `common` beside `xy_frame_in`:
  `rectangle_in(session, plane, w, h)` (the template through
  `SessionOp::AddProfile`), `box_in(session, plane, [w, h, d])` (that,
  then `SessionOp::AddExtrude`; answers profile and extrude) and
  `xy_box_in(session, size)` (on a fresh world xy frame; answers the
  extrude).
- **Routed**: both private builders (`combine_ops::boxed`,
  `blend_authoring::boxed`, 51 calls between them) are gone; the inline
  boxes in `combine_ops`, `creation_ops` (bracket block, boss),
  `docm1_face_frame`, `story_assembly::author_box_part` and all four of
  `story_authoring`'s rook boxes take the doors. The rook already
  authored its discs through a suite helper (`circle_at`), so a box
  door reads as that story's own style.
- **Beyond the row, same construction**: the template profiles drawn
  and not extruded (`combine_ops`' seat-kind and split-seat rows,
  `blend_authoring`'s not-a-body row, `creation_ops`'
  `new_document_derives_its_id_and_clears_the_session`) route to
  `rectangle_in`.
- **Kept longhand, with the reason**: `creation_ops`'
  `the_rectangle_template_is_the_centred_polygon` — its subject is the
  template's lowering, compared against a polygon spelled beside it;
  the multi-loop plate in
  `the_op_vocabulary_exceeds_the_chrome_templates_and_that_works`
  (a rectangle with two circles is not this construction);
  `profile_edit`'s template list and `creation_ops`' non-finite
  template, which never reach the session; `path_authoring`'s square,
  which is a `Path` profile and that suite's subject.
- The plants and their per-row red lists are in the PR body.
