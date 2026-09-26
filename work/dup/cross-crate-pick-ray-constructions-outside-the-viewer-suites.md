---
id: cross-crate-pick-ray-constructions-outside-the-viewer-suites
kind: issue
title: Five byte-identical ray helpers across bvh and editor-core, and the axis-aligned pick ray one crate over
status: closed
opened: 2026-09-20
priority: P4
cost: D
closed: 2026-09-26
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

- **The denominator, so the next lane does not have to re-derive it:
  41 construction sites in 15 files.** Re-taken at `cd9fdfd6b` over
  every tracked `.rs` with no path argument, for `Ray {` with an
  optional path qualifier, minus `crates/viewer/tests/` (closed),
  minus `PathError::ContinuationTargetOffRay` (a different symbol the
  needle matches), minus the type's own `struct`/`impl` headers, and
  minus the nine `-> Ray {` SIGNATURE lines, which the needle matches
  once per helper on top of that helper's one construction.

  | file | sites |
  | --- | --- |
  | `crates/editor-core/src/resolve/pick.rs` | 10 |
  | `crates/bvh/tests/ray_r2.rs` | 10 |
  | `crates/editor-core/tests/pick3_early_out.rs` | 5 |
  | `crates/editor-core/tests/review_pick3_r2_probes.rs` | 4 |
  | `crates/editor-core/tests/review_pick_r2_probes.rs` | 2 |
  | `crates/bvh/tests/ray.rs`, `bvh/tests/review_gui1_r1.rs`, `editor-core/tests/{gui1_pick, gui1_pick_r2, review_gui1_r1, review_pick3_r1_probes, edit_pair_apply_names}.rs`, `pncad-py/src/py/pick.rs`, `viewer/src/camera.rs`, `viewer/src/pickindex.rs` | 1 each |

  **This is a denominator, not a member count.** The members have not
  been classified; the two results above are what a read of the named
  helpers established. **Classifying the 41 is the work**, and it is
  why this is a row and not more edits in the unit that found it.

  The first version of this row published *"about forty construction
  sites in thirteen files"*. The site count was honest as a rounding;
  the file count was not — thirteen came from a needle that could not
  see a path-qualified `editor_core::Ray {` or `s::Ray {`, which is
  two more files. A rounded number that announces its rounding is
  fine in a row whose purpose is to save a re-derivation; an exact
  one that is wrong is not.

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


## Closed

Folded by the S-DUP lane `dup/b6-b`, 2026-09-26, cut from
`032999ff2`. PR #3302.

- **The denominator re-taken** with the row's instrument (`git grep
  -n -E '(\b|::)Ray \{' -- '*.rs'`, no path argument, minus
  `crates/viewer/tests/`, `ContinuationTargetOffRay`, the type's own
  headers and the NINE signature lines — eight `-> Ray {` and
  `edit_pair_apply_names`'s `-> editor_core::Ray {`): **41 sites in 15
  files**, the row's number, with `pick.rs` at 10 and `ray_r2.rs` at
  10. Classified, every one:
  - **the array adapter**, 15: five byte-identical `fn ray` bodies
    (`bvh` 2, `editor-core` 3), `ray_r2`'s eight longhand literal rays
    and its integer `ray_of` body, and `pick3_early_out`'s longhand skew
    ray → `bvh::test_support::ray` (`ray_of` now delegates to it).
  - **the downward ray**, 5: `edit_pair_apply_names`'s `down()`, three
    in `pick3_early_out`, one in `pick.rs`'s unit module →
    `editor_core::test_support::down_from`, which `viewer`'s
    `tests/common` re-exports.
  - **the aimed ray** `target - dir * reach`, 7: `pick.rs`'s own
    `ray_through` body and its three longhand `x - dir` rays,
    `review_pick_r2_probes`' wide aim, `review_pick3_r2_probes`'
    interior aim and its snapped `aimed` (now `aimed_snapped`,
    delegating) → `editor_core::test_support::aimed`.
  - **`near_tangent`**, 3: the `pick.rs` unit module's and the two
    integration copies that declared themselves copies of it →
    `editor_core::test_support::near_tangent`.
  - **bespoke, not folded**, 8: `ray_r2`'s random ray; `pick.rs`'s two
    table closures, its level ray and its interior ray;
    `review_pick3_r2_probes`' dir-scaled ray; the random-origin rays of
    `review_pick_r2_probes` and `review_pick3_r1_probes`.
  - **production, not members**, 3: `pncad-py`'s `pick.rs`, `viewer`'s
    `camera.rs` and `pickindex.rs`.
- **The homes are the tree's test-support mechanism**: `bvh` gains a
  `test-support` feature and `bvh::test_support` (`boxed`, `ray`);
  `editor-core`'s existing `test-support` feature now gates
  `editor_core::test_support` (`AXES`, `down_from`, `aimed`,
  `near_tangent`, `listed`, `det_and_conditioning`) and forwards
  `bvh/test-support`. Each crate turns its own on through its self
  dev-dependency; `viewer` turns both on through dev-dependencies.
  `resolve::pick`'s unit tests, `editor-core`'s pick suites and
  `viewer`'s corpus pick suites read one definition of each.
- **Beside the class, in the files it had open**: `bvh`'s `boxed`,
  byte-identical in five suites and written longhand six more times in
  `ray_r2` (four literals, a centre ± extent and a two-case closure; its
  integer `box_of` delegates) → `bvh::test_support::boxed`;
  `review_gui1_r1`'s `answered` → `listed`; `pick.rs`'s unit
  `conditioning` → `det_and_conditioning(..).1`; `viewer`'s
  `common::up_at` and `level` → `bvh::test_support::ray`.
- **Gates**: the `gated_to!` markers of the suites that now read a
  test-support module name its file (`gated-suite-paths.sh` passes);
  `test-features-dev-only.sh` passes.
- **Residue rows**: both filed by this unit's first pass are closed in
  the same PR — the mechanism already existed, so neither was a design
  question.
- **Plants** (copy/restore harness, tree clean against HEAD after each;
  `editor-core` scoped to the pick suites and `resolve::pick`, 61
  rows; `viewer` 806; `bvh` 57): a `panic!` in `aimed` reds 10 `viewer`
  rows and 8 `editor-core` rows, six of them `resolve::pick` unit rows;
  `det_and_conditioning` answering 1 reds 1 and 2 (one a unit row);
  `near_tangent` at `k + 64` reds 6 (three unit rows, three in
  `pick3_early_out`); `boxed` with its x bounds swapped reds 18 of
  `bvh`'s 57; `ray` with x negated reds 13 in `bvh`, 6 in `viewer`
  (the level rays) and 17 in `editor-core`.
