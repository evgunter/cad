---
id: topo-tests-has-two-vocabulary-homes-with-no-stated-boundary
kind: issue
title: topo/tests has two tests-only vocabulary homes and neither names the other
status: open
opened: 2026-09-18
priority: P3
cost: E
---

## Finding

- **Where**: `crates/topo/tests/common/mod.rs` against
  `crates/topo/tests/fixture/mod.rs`.
- **Importance**: low-medium
- **Confidence**: sure that there are two homes and that neither
  mentions the other; the remedy is a boundary decision, not a merge
- **Raised by**: the full review of PR #2812 (S-DUP), 2026-09-18

`topo`'s test binary has **two** `tests/`-only vocabulary modules, both
declared as roots of `tests/all.rs`:

| | `common/mod.rs` | `fixture/mod.rs` |
| --- | --- | --- |
| size | ~510 lines | 393 lines |
| public items | the cube/prism family, `line`, `plane`, `describe_as_intersections`, the straddle seat, `flush_declarations`, two assertion helpers | `Built<T>`, `build`, `foreign_cache`, `certify_at_dual` |
| consumers | most suites in the crate | `m6_2_fitted_at_rest.rs`, `review_m6_2_probes.rs` |
| header allows | `dead_code` + `unwrap/expect/panic` + `unreachable_pub` | the same four, same order |
| the comment on `dead_code` | *"one instance per binary; no single consumer uses all of it"* | *"one instance per binary; no single consumer uses all of it"* — verbatim |

**Nothing today is duplicated between them**, and that is why this is a
low row rather than a fix. `fixture` is the M6-2 cylinder×sphere
acceptance rung: an SSI trace, a fitted chart image and a lift to the
caller's scalar. `common` is the polyhedral vocabulary. They do not
overlap.

## Why it is still worth a row

**`common/mod.rs`'s header presents itself as *the* home** and cites
the crate's three-homes rule as if the question were settled:

> Compiled into the test binary, not the library: the cheapest of this
> crate's three homes for test vocabulary, and the right one whenever
> the library itself never names the item. `topo`'s
> `src/test_support_impl.rs` docs give the rule for all three.

The three-homes rule distinguishes `mod tests`, `test_support_impl.rs`
and "`tests/common/mod.rs`" — it does not have a slot for a **second**
`tests/`-only module, so a reader following the cited rule to decide
where a new fixture goes will not learn that `fixture/` exists. The
duplicated `#![allow]` header and the verbatim `dead_code` comment say
the second home was created by copying the first's preamble, which is
the usual way two homes with no boundary come about.

**Two homes with no stated boundary is the shape that drifts**, and
this program's own charter is that class. The next person adding a
non-polyhedral fixture picks by coin flip; the one after that adds a
helper both want.

## What link 3 has to decide anyway

`work/dup/brick-has-two-constructions-and-two-homes.md`'s third link
moves the cube/prism family from `tests/common/mod.rs` down into
`crates/topo/src/test_support_impl.rs`. That move **has to answer this
question in passing**, because it changes what `common/mod.rs` is for:

- if `common` empties out into `src/`, does `fixture` follow it, stay,
  or absorb what is left?
- `fixture::build` is `Option`-returning and traces at `f64` before
  lifting — it is not obviously a `src/` citizen under the same
  argument that carries `prism_z` down (every consumer of `prism_z` is
  at or above `topo`; `fixture`'s two consumers are both inside
  `topo/tests`).
- whichever way it goes, `common/mod.rs`'s header sentence about "the
  three homes" needs a fourth line or a correction, because it is the
  document a reader is pointed at.

So the cheapest time to settle it is with that move, and the useful
output of this row is one sentence in each header saying what belongs
in which. **A boundary stated is the whole remedy**; no code needs to
move for it.

## What was not checked

- Whether `fixture`'s four public items have counterparts anywhere else
  in the tree. The reflex-L census run alongside this row keys on
  profile literals and would not see an SSI fixture.
- The other crates. `mesh`, `sweep`, `editor-core` and `profile` each
  have a `tests/common/`, and `editor-core` also has a
  `tests/fixture/mod.rs`; whether the same two-home shape is there was
  not looked at.

## The second home mints tolerances too (measured 2026-09-18, S-DUP link 2)

`crates/topo/tests/fixture/mod.rs` holds **4** `Tol::witness()` calls.
`crates/topo/tests/common/mod.rs` held 10 and now holds **0**: link 2
threaded `tol: Tol` through every door of the prism/cube family, because
`scripts/gates/witness-not-ambient.sh` reads
`crates/topo/src/test_support_impl.rs` as production code (re-confirmed
by planting a violation and watching the gate name the line).

This is a second axis on which the two homes are not interchangeable,
and it bears on the boundary question directly: whichever home moves
into `src/` pays that cost, and `tests/fixture/mod.rs` has not paid it.
A decision that moves `common/` and leaves `fixture/` where it is has
nothing to do; one that moves `fixture/` instead, or as well, owes the
same threading over its four sites first.
