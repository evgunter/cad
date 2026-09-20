---
id: the-rectangle-profile-is-still-written-longhand-beside-its-door
kind: issue
title: Six longhand axis-aligned rectangle polygons beside common::rectangle, one of them in viewer/src
status: open
opened: 2026-09-20
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
