---
id: editor-core-src-pick-tests-restate-the-fixture-pick-doors
kind: issue
title: editor-core's resolve::pick unit tests restate near_tangent, aimed, down_from and the uncertified conditioning that tests/fixture/pick holds
status: open
opened: 2026-09-26
priority: P4
cost: E
---



## Finding

- **Where**: the `#[cfg(test)]` module of
  `crates/editor-core/src/resolve/pick.rs`:
  - `near_tangent` — the same construction as
    `tests/fixture/pick.rs`'s `near_tangent`, which batch 6 made the
    one home of the two integration-suite copies (both of which
    declared themselves "`pick.rs`'s `near_tangent` fixture").
  - `ray_through(target, dir, reach)` — `fixture::pick::aimed`,
    body for body. Batch 6 routed the module's own three longhand
    `origin: x - dir` rays through it, so the module holds one
    spelling of the door rather than four.
  - `conditioning(ray, tri)` — the second half of
    `fixture::pick::det_and_conditioning`.
  - one longhand downward ray (`Ray { origin: Point3::new(midpoint.x,
    midpoint.y, 2.0), dir: Vec3::new(0.0, 0.0, -1.0) }`) —
    `fixture::pick::down_from`.
- **Why it is a row and not a fold**: a unit-test module cannot
  import a `tests/` tree, and `editor-core` has no `src` test-support
  home. The fix batch 6 used for `viewer`
  (`crates/viewer/src/test_support.rs`: a `#[cfg(test)]` module that
  speaks only through external crates, mounted by the tests tree with
  `#[path]` and re-exported) applies unchanged — the three doors
  speak only through `bvh` and `geom_core`, and `listed`, which names
  `editor_core` types, would stay in `tests/fixture/pick.rs`. It was
  not done in the same batch because that shape lands there for the
  first time, unreviewed; a second instance on another crate's `src`
  should follow a read of the first.
- **Importance**: low. No oracle rides on the rays; `conditioning` is
  a reference the unit rows read `crossing` against, and it shares no
  code with it.
- **Instrument**: a read of the module against
  `tests/fixture/pick.rs`, door by door, after `git grep -n
  'fn near_tangent\|fn ray_through\|fn conditioning\|fn
  det_and_conditioning'` over every tracked file.
- **Raised by**: the S-DUP lane folding
  `cross-crate-pick-ray-constructions-outside-the-viewer-suites`,
  2026-09-26.

## Why this sits on S-DUP's slate

The copies are test-side vocabulary spelled twice — S-DUP's charter.
The ground is `editor-core/src`; any claimant may take it by `git mv`.
