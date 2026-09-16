---
id: topo-tests-review-m2-pr7-rederives-the-shared-cube
kind: issue
title: review_m2_pr7 declares a private mapped_cube whose doc says it is common::geometric_cube's op sequence
status: open
opened: 2026-09-16
---

## Finding

- **Where**: `crates/topo/tests/review_m2_pr7.rs:30` —
  `fn mapped_cube(map: impl Fn(Point3<f64>) -> Point3<f64>) -> Body<f64>`,
  against `common::mapped_cube` and `common::geometric_cube`.
- **Importance**: low-medium
- **Confidence**: sure
- **Raised by**: the style review of the `dup-brick` lane's PR (S-DUP),
  2026-09-16

Its own doc opens **"The geometric-cube op sequence of
`common::geometric_cube`, but with every coordinate passed through
`map`"** — a self-declared re-derivation of a shared builder. It also
**shares a name with a different function in the same test tree**:
`common::mapped_cube` takes `impl Fn(f64, f64, f64) -> Point3<f64>`,
this one takes `impl Fn(Point3<f64>) -> Point3<f64>`, so the two cannot
be told apart by the call sites' shape without reading both signatures.
It is called with the identity map at `:186` (`mapped_cube(|p| p)`),
which is the unit cube — the body `common::brick` and
`common::mapped_cube(Point3::new)` both produce, verified by execution
in the `dup-brick` PR.

Whether it should be deleted depends on the row above it
(`topo-tests-geometric-cube-and-cube-into-are-one-sequence-twice`): the
suite's reflection rows want the Newell planes to flip, and whether the
shared door can serve that is the same question.

**How it was found, and why the obvious sweeps miss it.** It is
invisible to a `fn brick` census (wrong name), to a
`fn [a-z_]*brick[a-z_]*` census (wrong name again) and to a `prism_z`
construction sweep (it does not use the builder at all). What finds it
in seconds is the PROSE: `rg 'op sequence of|verbatim' crates/topo/tests/`.
A wider prose census is its own row,
`topo-tests-self-declared-fixture-copies-census`.
