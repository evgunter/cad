---
id: topo-tests-brick-copies
kind: issue
title: Twenty-odd private brick fixtures in crates/topo/tests, blocked on whether topo may dev-depend on sweep
status: open
opened: 2026-09-15
---

## Finding

- **Where**: `crates/topo/tests/` — 24 `fn brick` declarations across 22 suites
  (`review_m3_pr4.rs`, `m3_pr5_boolean_ops.rs`, `review_s6_probe.rs`,
  `shell_roles.rs`, `merge_skip.rs`, `void_door.rs`, `seat3_flush_detector.rs`,
  … ). Five are byte-identical at the signature
  `fn brick<T: Decide>(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<T>`;
  the rest vary only in the bound (`+ Bounds`, `+ CertifiedBounds +
  PropsQuadLane`) or in the argument spelling (`m4_pr2_transform.rs` takes
  `(x, y, h)`; `geom_origin_rows.rs` takes nothing).
- **Importance**: medium
- **Confidence**: sure that they are copies; unsure that the remedy below is
  the one Ev wants
- **Raised by**: the `S52` lane (SUITE), 2026-09-15

`S52` homed the axis-aligned box as `sweep::test_support::brick<T: Decide>`
and collapsed the copies in `sweep`, `mesh`, `step-export` and `stl` onto it.
`topo`'s are the largest remaining population of the same fixture and were
**not** touched, because reaching the home from `crates/topo/tests/` needs
`topo` to carry `sweep = { path = "../sweep", features = ["test-support"] }`
in `[dev-dependencies]`, and `sweep` depends on `topo`. Cargo permits that
cycle — a dev-dependency edge is not followed for the library build — but
nothing in this tree does it yet, and whether it should is a decision about
the crate graph rather than about a fixture. It is worth stating plainly that
the cost is not only conceptual: it makes `cargo test -p topo` build `sweep`,
which is the whole extrude/revolve/blend stack.

The alternatives, so whoever takes this does not have to re-derive them:

1. **The dev-dependency edge**, above. One manifest line, 24 fixtures gone,
   and the rule "an item lives at the narrowest home all of its consumers can
   reach" is satisfied because `sweep`'s own `src` pins name the family.
2. **A second home in `topo`** — `crates/topo/src/test_support_impl.rs` already
   exists with the right gate, but `topo` cannot extrude, so a `brick` there
   would be hand-built through the Euler operators and would NOT be the same
   body the rest of the tree calls a brick. That is a new spelling, not a
   shared one.
3. **`crates/topo/tests/common/mod.rs`**, which already exists and is reachable
   from every suite in the aggregated binary. Same objection as (2): `topo`'s
   tests cannot call `extrude` without the edge from (1), so this only moves
   the hand-building to one place. That is still 24 → 1 inside `topo` and it
   needs no decision from anyone, which may make it the right first move
   whatever happens to (1).

`S52`'s own sweep and its blind spots are in that PR's body.
