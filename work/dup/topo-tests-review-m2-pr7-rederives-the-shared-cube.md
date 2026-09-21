---
id: topo-tests-review-m2-pr7-rederives-the-shared-cube
kind: issue
title: review_m2_pr7 declares a private mapped_cube whose doc says it is common::geometric_cube's op sequence
status: closed
opened: 2026-09-16
branch: dup/cube-sequence-reconcile
closed: 2026-09-16
pr: 2727
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

## Closed (2026-09-16, PR #2727)

`review_m2_pr7.rs`'s private `mapped_cube` is deleted. Its six call
sites name `common::mapped_cube`, with each `|p| …` map respelled as
the `(x, y, z)` arity `common`'s door takes.

**Equality proved by execution at four maps, not one.** Both doors were
run and dumped as `format!("{body:#?}")` — `Body`'s DERIVED `Debug`, so
every arena, key, slot version and `free_head` is in the comparison —
and the dumps are byte-identical at the identity, at the `x` reflection,
at the 1e6-scaled reflection and at the `10ε` inverted slab. The
identity case additionally lands in the same six-door group as
`brick`, `prism`, `prism_z`, `mapped_cube` and `cube_into`.

The one thing the private door's doc carried that the shared sequence
does not — that under a reflection the same loop traversals become
inward-CCW, so every Newell plane flips and the body's only defect is
global orientation — moved to the suite's module doc, where it is about
what the suite attacks rather than about a builder.
