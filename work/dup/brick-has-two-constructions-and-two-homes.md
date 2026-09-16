---
id: brick-has-two-constructions-and-two-homes
kind: issue
title: The axis-aligned box is built two ways in two homes; the shared home is topo's, not sweep's
status: open
opened: 2026-09-16
refs: [topo-tests-brick-copies]
---


## Finding

- **Where**: `crates/sweep/src/test_support.rs` (`brick`, over `prism_at`
  → `prism_on(sketch_at(z0), …)`, i.e. the extrude machinery) against
  `crates/topo/tests/common/mod.rs` (`prism_z`/`Prism`, hand-built from
  `mvfs`/`mev`/`mef`).
- **Importance**: medium
- **Confidence**: sure that there are two constructions in two homes;
  **unmeasured** whether the two produce equal bodies, which is what
  decides the remedy
- **Raised by**: the S-DUP orchestrator, 2026-09-16, out of
  `work/dup/topo-tests-brick-copies.md`

Two spellings of "the plainest body in the kernel". `sweep`'s is
reachable from `sweep`, `stl` and `step-export` (both of the latter
`pub use sweep::test_support::brick` from their `tests/common`);
`topo`'s is reachable only from `topo`'s own test binary. Neither can
reach the other today.

## Why the shared home is `topo`'s

`topo`'s `src/test_support_impl.rs` module docs state the crate's three
homes for test vocabulary, and `tests/common/mod.rs`'s header cites
them:

| home | reachable from |
| --- | --- |
| in-crate `mod tests` | private sites only |
| `src/test_support_impl.rs`, gated `feature = "test-support"` | `topo`'s `tests/` **and any crate above `topo`** |
| `tests/common/mod.rs` | `topo`'s test binary only |

`prism_z` and `Prism` sit in the third. Moving them to the second makes
them nameable from `sweep`, `mesh`, `stl`, `step-export` and
`editor-core` through the `topo = { path = "…", features =
["test-support"] }` spelling — **no new manifest edge and no
dev-dependency cycle**, because all of those already depend on `topo`.
Every import `prism_z` needs (`geom`, `geom_brep`, `geom_core`) is
already a normal dependency of `topo`, so the move compiles where it
lands.

The rule is the tree's own: an item lives at the narrowest home all of
its consumers can reach. An axis-aligned box **is** a `topo` value;
building one by extruding a rectangle uses a `sweep` operation to
produce a lower layer's value, which is why the shared home was
reached for uphill and found to need a cycle.

This supersedes option 1 of `work/dup/topo-tests-brick-copies.md` (add
`sweep` to `topo`'s `[dev-dependencies]`): that edge is cargo-legal and
has three precedents in `topo`'s own manifest, but it is unnecessary
and it would make `cargo test -p topo` build the extrude/revolve/blend
stack, which nothing does today.

## What this row owes before it is dispatched

**The measurement, first, not the fix.** Build the same box both ways
at the same `(x, y, z)` and compare: face/edge/vertex counts, arena
order, the keys, and `mass_properties` bits. The answer decides the
remedy and the remedy is different in each direction:

- **Equal** → one fixture. `prism_z`/`Prism`/`brick` move down to
  `topo::test_support`; `sweep::test_support::brick` becomes a
  re-export or a delegation, and `stl`/`step-export` follow it.
- **Not equal** → two legitimately different fixtures that must be
  **named for their construction** rather than both called `brick`.
  `sweep`'s suites keep the extruded one wherever extrusion is the
  thing under test; the rest take whichever the suite actually means.
  Renaming is then the fix, and the duplication claim is retired as
  false.

Do not let the lane choose the shape before the measurement is in.
