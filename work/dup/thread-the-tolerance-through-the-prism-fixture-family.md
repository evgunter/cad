---
id: thread-the-tolerance-through-the-prism-fixture-family
kind: issue
title: Link 2: thread tol through the prism fixture family, which witness-not-ambient forces before the move
status: closed
opened: 2026-09-18
refs: [brick-has-two-constructions-and-two-homes]
closed: 2026-09-18
pr: 2839
---


## Finding

- **Where**: `crates/topo/tests/common/mod.rs` — the prism/cube fixture
  family, which calls `Tol::witness()` internally rather than taking a
  tolerance.
- **Importance**: medium — it gates link 3
- **Confidence**: sure, and the gate was **measured** rather than read
- **Raised by**: the `dup-brick-measure` lane (S-DUP), 2026-09-16

**Link 2 of `work/dup/brick-has-two-constructions-and-two-homes.md`.**
The family cannot move into `crates/topo/src/test_support_impl.rs`
until it stops reaching for an ambient tolerance, because
`scripts/gates/witness-not-ambient.sh` forbids `Tol::witness()` under
`crates/*/src` and exempts only `#[cfg(test)]` —
`gate_test_only_mounts`' `GATE_CFG_TEST_NOT_RE` explicitly excludes an
`any(...)` cfg from that narrowing, so
`test_support_impl.rs`'s
`#[cfg(any(debug_assertions, test, feature = "test-support"))]` mount
puts it squarely in the gate's production set.

**Established by planting one violation there, watching the gate fire
and name the line, then reverting and confirming green over 434 files.**
Method item 4 applied to a gate rather than a constant.

## Why it is worth doing on its own merits

A fixture should not reach for an ambient tolerance — that is what the
gate exists to say. And threading `tol: Tol` lands `prism_ops`'
signature on `sweep::test_support::brick`'s, which is convergent
evidence that the two are one door (the measurement on the brick-homes
row proved the bodies equal; matching signatures is the shape following
the fact).

Also in this unit: the file-level
`#![allow(unwrap_used, expect_used, panic)]` that is unremarkable in
`tests/` lands inside the library tree on the move, and needs a
decision rather than a copy.

## Count

The family called `Tol::witness()` **24 times** when measured on
2026-09-16; `crates/topo/tests/common/mod.rs` alone held 17. **Re-take
it** — this program's standing result is that no count survives contact
with the tree, and PR #2812 has since unified two builders into one,
which will have moved it.

## Done (2026-09-18, branch `dup/thread-the-tol`)

### The premise, re-confirmed rather than read

One `Tol::witness()` planted in `crates/topo/src/test_support_impl.rs`;
`scripts/gates/witness-not-ambient.sh` fired and named the planted line
by file and line number; reverted; green again over **436** source files
(the 2026-09-16 measurement said 434, so the production set has grown by
two). The `#[cfg(any(debug_assertions, test, feature = "test-support"))]`
mount is still in the gate's production set.

### The count, re-taken — and the 24 and the 17 are the same file twice

**10**, all of them in `crates/topo/tests/common/mod.rs`. Nothing in the
family lives anywhere else: `git grep` with no path argument for each
member's DEFINITION finds every one of them in that file and nowhere
else.

The row's "24 across the family with 17 in this file" was never a family
total against a file subtotal. It is **one file at two commits**, read
at two moments and written down as though they were two scopes:

| commit | `Tol::witness()` in `crates/topo/tests/common/mod.rs` |
| --- | --- |
| `01ca2ead7` (before S-DUP touched it) | 24 |
| `6b092272a` (link 1, one Euler cube sequence) | 17 |
| `244a6bb83` (link 1b, PR #2812, one `prism_ops`) | 10 |
| `bcc2e6c6f` (this unit's merge base) | 10 |

So the unification did the larger half of this unit's work before it
started, and the row's own "re-take it" instruction is what caught it.

### What was threaded

`tol: Tol` LAST, which is the tree's convention at every door the family
calls (`mev`, `mef`, `set_edge_curve`, `graft_disjoint_all_keyed`,
`find_flush_candidates`) and the signature this converges on
(`sweep::test_support::brick(x, y, z, tol)`,
`topo::fixtures::prism(n, tol)`).

Threaded: `plane`, `prism_ops`, `geometric_cube`, `straddle_seat`,
`prism`, `prism_z`, `brick`, `describe_as_intersections`, `mapped_cube`,
`cube_into`, `flush_declarations`.

**Not threaded, and why.** `line` mints nothing — it is
`EdgeCurveSpec::line_between`, which takes no tolerance — so a `tol`
parameter there would be a parameter the body never reads, which is the
camouflage the gate's own header warns about. `assert_every_chord_named_by_both_rules`
is an assertion helper over an already-built body: no tolerance, nothing
to thread.

**In the family by judgement, though neither is a prism.**
`straddle_seat` builds two `prism_z`es and grafts them, so it is a
caller; `flush_declarations` is not a prism fixture at all, but it minted
one of the ten witnesses in the file link 3 moves wholesale, and leaving it
would have left link 3 blocked by exactly the gate this unit exists to
clear. Both are one-line changes that cannot move a body.

### Evidence the bodies did not move

1. **`Tol` has one inhabitant.** It is a ZST whose only constructor is
   `witness()`, so `tol == Tol::witness()` at every threaded site by
   construction. Passing the value in is not merely equivalent to
   minting it — it is the same value.
2. **`crates/topo/tests/cube_doors_agree.rs`**, the guard link 1b built
   and its fix pass strengthened to read every face's outward normal:
   all three rows green (`every_box_door_builds_one_body`,
   `every_door_builds_the_prism_its_inputs_name`,
   `geometric_cube_is_the_one_door_that_keeps_its_scaffolding`).
3. **A whitespace-normalised differ over all 75 changed files.** Strip
   whitespace, delete every `Tol::witness()` / `geom_core::Tol::witness()`
   and every comma, and **71 of the 75 files are byte-identical to the
   merge base.** The four that are not are `common/mod.rs` (the threaded
   signatures) and three files where `cargo fmt` added or removed a
   redundant closure brace (`m3_pr6_tier3prime.rs`, `review_m2_pr7.rs`,
   `review_m3_pr6.rs`). No other token anywhere changed.

### The `#![allow]` decision: it moves with the family, and the tree has already decided it twice

`crates/topo/tests/common/mod.rs`'s
`#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]` is
**not a `tests/`-only indulgence and does not need a new argument.** Two
files in this tree already hold fixture vocabulary inside `src/` and both
carry that exact line at file scope:

- `crates/sweep/src/test_support.rs:110`
- `crates/topo/src/fixtures.rs:44`, with the argument written out:
  *"Test-support code: panicking is a test's failure mechanism (L5), and
  fixture unwraps are on keys the fixture itself just minted."*

So the decision is: **carry it, with `fixtures.rs`' justification
comment, at the scope of the file the family lands in — and that file
should not be `test_support_impl.rs` itself.** An inner `#![allow]` in a
module file scopes to that module, so putting the family into
`test_support_impl.rs` would blanket `ArenaCounts`, which compiles clean
without it today and is the one item there with a non-test consumer (the
D1 debug postcondition). A sibling module file re-exported through
`test_support` keeps the allow exactly as wide as the code that earns it,
and matches both precedents.

The other two allows need no decision from link 2: `unreachable_pub` is
already handled per-item in `test_support_impl.rs`, and `dead_code` is
handled per-item in `fixtures.rs` (`// key bundles expose every minted
key; tests pick what they need`) — the same shape works for `PrismOps`,
`Prism` and `GeoCube`.
