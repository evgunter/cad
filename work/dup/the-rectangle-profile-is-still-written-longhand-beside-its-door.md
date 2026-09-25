---
id: the-rectangle-profile-is-still-written-longhand-beside-its-door
kind: issue
title: Seven longhand axis-aligned rectangle polygons beside common::rectangle, two of them in viewer/src
status: closed
branch: dup/viewer-insert-doors
opened: 2026-09-20
closed: 2026-09-24
pr: 3150
priority: P4
cost: E
---


## Finding

- **Where**, at `ab086f8c1`, after `common::rectangle` was minted and
  four spellings folded onto it — **six** axis-aligned rectangle
  polygons remain, written out corner by corner:
  - `crates/viewer/src/scene.rs:737`, inside `plate_with_hole`, and it
    is **byte-identical** to the line deleted from
    `crates/viewer/tests/common/asm.rs` by that fold. `src` cannot
    reach a `tests/` helper, so this one is a statement about where
    the home belongs, not a substitution.
  - `crates/viewer/tests/combine_ops.rs:1844` — `(0.0,0.0) (0.02,0.0)
    (0.02,0.02) (0.0,0.02)`, which is exactly `common::square(plane,
    0.02)` — written **fifteen lines below a literal
    `common::square(sketch_frame, 0.02)` call in the same function**
    (`:1829` against `:1844`), in a file that calls that door again at
    `:1985`.
  - `crates/viewer/tests/creation_ops.rs:547` (centred, ±0.02 × ±0.01),
    `doc_io.rs:328` (the unit square), `docm1_face_frame.rs:89`
    (centred ±0.005), `docm9_range_vs_probe.rs:82` (the unit square).
    The two centred ones need an `origin` the current door already
    takes; the two unit squares are `square(plane, 1.0)`.
- **Importance**: low-medium, and the `src` member is the interesting
  one: it says the shared rectangle may belong beside `PLATE_EXTENT`
  in `viewer::scene` rather than in `tests/common`, which is a
  home question this unit did not have the standing to settle.
- **Confidence**: sure about the six; the home question is open.
- **Instrument, and its blind spot**: a structural scan over every
  tracked `crates/viewer/**.rs` for `(LoopProgram|ProfileLoop)::polygon`
  with exactly four corner tuples spanning exactly two distinct x and
  two distinct y expressions — shape, not text, so it catches the
  centred and variable-cornered members a literal grep misses. It is
  blind to a rectangle assembled from a `Vec` or a loop, to one with
  redundant collinear corners, and to every crate but `viewer`.
- **Raised by**: the S-DUP lane's fix pass on
  `viewer-review-suite-fixtures-have-no-oracle-role`, 2026-09-20.

## Why this sits on S-DUP's slate

Five of the six are under `crates/viewer/tests/`, claimed by `chrome`,
`tcost`, `tint`, `vdoc` and `view` with no single ground-owner; the
sixth is `crates/viewer/src/scene.rs`, which is `view`'s. It sits here
because the class is one construction spelled six times and because the
unit that minted the door found it. `view` is the right claimant if the
home moves into `src`, by `git mv`.

## Disposition (2026-09-24, `dup/viewer-insert-doors`)

- **Census re-taken at `6db5b87f2`**: seven, not six. A balanced-paren
  scan of every `polygon(` call in `git ls-files crates/viewer` (20
  calls) for four corner tuples over exactly two distinct x and two
  distinct y expressions found the row's six plus
  `crates/viewer/src/widgets.rs`'s `value_field_tests` (a 0.04 m
  square) and the door itself. `scene.rs`'s member has moved to
  `plate_with_hole`'s first line.
- **Folded**: `combine_ops` (`square(lifted, 0.02)`), `doc_io`
  (`square(plane, 1.0)`), `docm9_range_vs_probe` (`square(f, 1.0)`)
  and `docm1_face_frame`, whose rectangle is a
  `SessionOp::AddProfile` loop rather than a node: the door is split
  into `common::rectangle_loop` and `rectangle`, which draws it.
- **Kept, `creation_ops`**: `the_rectangle_template_is_the_centred_polygon`
  asserts the chrome's template lowers to `(±w/2, ±h/2)` counter-
  clockwise from lower-left. The literal corners ARE that row's
  oracle; routing them through a test door would make the expectation
  a second computation of the thing under test.
- **Kept, `scene.rs`**: `plate_with_hole` is authored from a user's
  seat through the public doors, and the public door for a rectangle
  is `LoopProgram::polygon`. A `src` home would be a helper minted for
  one caller, or public API minted for tests; neither earns it. The
  library-side question (a rectangle constructor on `LoopProgram`) is
  the vocabulary's, beside the polygon/circle asymmetry the site
  already records.
- **Kept, `widgets.rs`**: a `#[cfg(test)]` module in `src` cannot reach
  `tests/common`; added as evidence to
  `viewer-src-test-modules-restate-the-literal-doors`, which holds the
  same home question.
- **Re-taken after merging `origin/main` (`5fe3dff36`)**: main had
  added `edit_maintenance.rs` with two more (`block`'s unit square at
  `x0`, and the 2 m square under the hole row); both folded onto
  `rectangle_loop`. `rectangle_loop` now asserts `w, h > 0`, the
  condition under which its "lower-left, counter-clockwise" holds.
- **Blind spots, named** (none of them a `polygon(` call, so none in
  the count above): rectangles written as `Step` chains
  (`profile_edit.rs`' and `path_authoring.rs`' session profiles,
  `src/session.rs`'s test module); the byte-identical private
  `fn polygon` step builder in `profile_edit.rs` and
  `profile_edit_order.rs`, **folded** onto `common::polygon_steps`;
  and rectangles drawn through the chrome's
  `ProfileShape::Rectangle` template, which is where the two session
  box builders live — filed as
  `the-session-box-is-built-by-hand-in-two-builders-and-inline`.
- **Outside `viewer`**: the same scan with no path argument finds 256
  such polygons over 11 roots. They cannot reach this door; the class
  is `the-box-extrusion-written-inline-inside-test-bodies`'s ground.
