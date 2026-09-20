---
id: gated-marker-derivation-is-blind-to-include-str
kind: issue
title: The gated_to! tests/ half derives sibling mod imports only and cannot see include_str!/include_bytes! data dependencies
status: open
opened: 2026-09-19
refs: [gated-markers-name-too-few-src-paths]
---


## Finding

- **Where**: `scripts/ci-filter.py`, `_unnamed_helper_imports`. It
  resolves a gated suite's **sibling `mod` imports** and refuses a
  marker that does not name them. It does not read `include_str!` or
  `include_bytes!`, so a suite can depend on a committed **data file**
  the derivation cannot see, the marker can omit it, and
  `--gated-check` passes.
- **Why it matters**: the omission fails in the silent direction. A PR
  editing only that data file does not select the suite, the suite does
  not run, and the run is green — the same failure mode
  `gated-markers-name-too-few-src-paths` names for the `src/` half,
  reached by a different route. The difference is that the `src/` half
  is **declared** unchecked and this one is **derived and incomplete**,
  which reads as covered.
- **Importance**: medium. The live population is empty as of today (see
  below), and that is exactly when an instrument gap stops being
  visible.
- **Confidence**: sure about the mechanism and the sweep; the
  population is **measured-today, not a standing count**.
- **Raised by**: the S-DUP citation-census unit, 2026-09-19, from its
  own style review.

## The sweep, its method and its result

Every file carrying a `test_utils::gated_to!` marker, scanned for
`include_str!`/`include_bytes!` whose argument resolves outside the
paths that marker names. **At `5b4979ef2` it returned two**, both in
`crates/viewer/tests/`:

- `review_gui2_r1.rs` — `include_str!("gallery_ring.pncad")`
- `review_gui2_r2.rs` — the same

Both are fixed in PR #2886: the two suites now read
`common::gallery_ring_at`, and both markers name
`crates/viewer/tests/gallery_ring.pncad` explicitly, because moving the
`include_str!` into `crates/viewer/tests/common/mod.rs` does **not**
make the dependency derivable — the resolver is equally blind to it
there. **So the population is empty today and the hole is not.**

What the sweep could not match: a data file reached through a path
built at runtime (`std::fs::read` of a computed path) rather than by a
compile-time include. Nothing in the tree does that today, and nothing
would catch it if it did.

## Why this sits on S-TCOST's slate

`scripts/` is S-TCOST's ground (`work.py territory`), and the change
filter is its instrument. `gated-markers-name-too-few-src-paths` is the
sibling row for the `src/` half; this is not a duplicate of it, because
that row's subject is a half nothing derives and this one's is a half
something derives incompletely.
