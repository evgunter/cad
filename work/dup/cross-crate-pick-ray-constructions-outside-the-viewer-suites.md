---
id: cross-crate-pick-ray-constructions-outside-the-viewer-suites
kind: issue
title: Five byte-identical ray helpers across bvh and editor-core, and the axis-aligned pick ray one crate over
status: open
opened: 2026-09-20
---


## Finding

The row `viewer-tests-each-spell-their-own-downward-pick-ray` named
*"any ray in another crate's suites"* as its instrument's blind spot.
This is that blind spot run rather than restated.

- **The hard result — five byte-identical helpers in two crates.**

  ```rust
  fn ray(origin: [f64; 3], dir: [f64; 3]) -> Ray {
      Ray {
          origin: Point3::new(origin[0], origin[1], origin[2]),
          dir: Vec3::new(dir[0], dir[1], dir[2]),
      }
  }
  ```

  Byte-identical at `crates/bvh/tests/ray.rs`,
  `crates/bvh/tests/review_gui1_r1.rs`,
  `crates/editor-core/tests/gui1_pick.rs`,
  `crates/editor-core/tests/gui1_pick_r2.rs` and
  `crates/editor-core/tests/review_gui1_r1.rs`. This is not the viewer
  class at all — it is an array-to-`Ray` adapter, and its home is a
  question about `bvh`'s own test vocabulary, since `editor_core::Ray`
  IS `bvh`'s type.

- **One member of the viewer class, one crate over.**
  `crates/editor-core/tests/edit_pair_apply_names.rs`'s `fn down()` is
  `common::down_from(0.0, 0.0, 5.0)` written longhand.

- **The denominator, so the next lane does not have to re-derive it.**
  `git grep -n 'Ray {' -- crates/` over every tracked file, no path
  argument, minus `crates/viewer/tests/` (closed) and minus
  `PathError::ContinuationTargetOffRay`, which is a different symbol
  the needle matches: **about forty construction sites in thirteen
  files** across `bvh`, `editor-core`, `pncad-py` and `viewer/src`.
  That figure is deliberately approximate and is a denominator, not a
  member count: the members have not been classified, and the two
  results above are what a read of the named helpers established.
  **Classifying the forty is the work**, and it is why this is a row
  and not more edits in the unit that found it.

- **Importance**: low-medium. No oracle on the construction — a wrong
  ray misses and reds the row that aimed it — but `editor-core`'s pick
  suites are the second implementation `viewer`'s rows compare
  against, so a shared home there needs the same per-file test the
  viewer unit ran.
- **Instrument, and its blind spot**: `git grep 'Ray {'` is
  line-shaped; `Ray` and `{` can be separated by a newline, and no
  site in this tree does that today. It also misses a ray built by a
  method or a `From` impl — `git grep 'Ray::'` over every tracked file
  names only `Ray::slab_enter`, a reader, so there is no such
  constructor — and any ray assembled in a loop from a table.
- **Raised by**: the S-DUP lane closing the four viewer-suite door
  rows, in its fix pass, 2026-09-20, measured at `cd9fdfd6b`.

## Why this sits on S-DUP's slate

The ground is three crates' `tests/` trees plus one `src/` test module,
each claimed by several programs, so there is no single ground-owner to
file it with, and one construction spelled more than once is S-DUP's
charter. Any claimant may take it by `git mv`.

