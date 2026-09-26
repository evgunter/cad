---
id: the-array-ray-and-box-adapters-are-one-per-crate-across-bvh-and-editor-core
kind: issue
title: bvh's and editor-core's test trees each hold the array-to-Ray adapter, and neither can reach the other's
status: closed
opened: 2026-09-26
priority: P4
cost: D
closed: 2026-09-26
---



## Finding

- **Where**: `crates/bvh/tests/common/mod.rs` (`boxed`, `ray`) and
  `crates/editor-core/tests/fixture/pick.rs` (`ray`). The two `ray`
  bodies are byte-identical: `Ray { origin: Point3::new(o[0], o[1],
  o[2]), dir: Vec3::new(d[0], d[1], d[2]) }`, and `editor_core::Ray`
  IS `bvh::Ray`.
- **What is left after batch 6**: one definition per crate. Batch 6
  took `ray` from five copies in two crates to one home per crate
  (`bvh`: two, now one; `editor-core`: three, now one) and `boxed`
  from five byte-identical copies in `bvh`'s suites to one.
- **Why it is a row and not a fold — a design question**: the two
  crates' test trees cannot reach each other's home today.
  `test-utils` depends on nothing, so a `test_utils` door would need a
  new `test-utils -> bvh` (or `-> geom-core`) edge; and
  `editor-core`'s tests could `#[path]`-mount `bvh`'s
  `tests/common/mod.rs` (the crate already depends on `bvh`, so it
  would compile), but that is a new cross-crate test mount, the shape
  `viewer`'s `corpus`/`fixture` symlinks carry for one pair of crates
  and nothing has ratified for a second. Either is a new cross-crate
  test dependency, which the lane that found it was told to file.
- **Importance**: low. Two six-line adapters with no oracle.
- **Instrument, and its blind spot**: `git grep -n -E
  '(\b|::)Ray \{' -- '*.rs'` over every tracked file, no path
  argument, each hit read. Line-shaped: a `Ray` literal split after
  the name would be missed (none in the tree today).
- **Raised by**: the S-DUP lane folding
  `cross-crate-pick-ray-constructions-outside-the-viewer-suites`,
  2026-09-26.

## Why this sits on S-DUP's slate

`crates/*/tests/` is S-TCOST's and S-TINT's in every crate, and the
question is one construction spelled once per crate — S-DUP's
charter. Any claimant may take it by `git mv`.

## Closed

Folded in the same PR that filed it (batch 6), on review: this was not
a design question. A `test-support` feature on `bvh` with a
dev-dependency edge from each consumer is the tree's existing shape.
`bvh::test_support` holds `boxed` and `ray`; `bvh`'s suites reach it
through a self dev-dependency, `editor-core`'s `test-support` forwards
`bvh/test-support`, and `viewer` enables it by dev-dependency. The two
per-crate homes (`bvh/tests/common`, `editor-core/tests/fixture/pick.rs`)
are gone.
