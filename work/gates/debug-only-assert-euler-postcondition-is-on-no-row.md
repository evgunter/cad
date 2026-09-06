---
id: debug-only-assert-euler-postcondition-is-on-no-row
kind: issue
title: Fifteen gated statements in topo name no pinned spelling, so the debug-only gate says nothing about them
status: open
opened: 2026-09-06
refs: [debug-only-reader-cannot-place-a-statement-attribute-over-a-braced-call]
---


## Finding

Found by the `gates/statement-attribute-items` lane while measuring the
`ArenaDelta` rows it added to
`scripts/gates/bit-identity-debug-only.sh`, and widened to its class by
that PR's style review.

**The class.** A row pins the uses of its SPELLINGS, so a
`#[cfg(debug_assertions)]` statement whose text names no spelling on any
row is invisible to the gate however debug-only the mechanism it belongs
to is. The `topo` arena-delta rows pin `ArenaDelta`; fifteen gated
statements in the same files, in the same mechanism, name it nowhere.

Every one of them is a live hazard on the same terms the rows exist for:
this workspace's `[profile.release]` keeps debug assertions on, so
dropping any of these attributes compiles and passes every test here, and
the first build that refuses it is a consumer's after publish.

**One site, the postcondition call.** `assert_euler_postcondition`
(`crates/topo/src/euler.rs:2238`, itself a `#[cfg(debug_assertions)]`
`pub(crate) fn`) is the second spelling of the arena-delta mechanism and
is on no row. Sixteen of its seventeen uses cost nothing, because the
statement naming it also names `ArenaDelta` and the `ArenaDelta` row
places it. One does not:

- `crates/topo/src/boolean/voids.rs:314` —
  `dst.assert_euler_postcondition(before, transplant, "insert_void");`
  passes a binding, so the call names no `ArenaDelta` and the
  `boolean/voids.rs` row (which pins the struct literal at `:291`) says
  nothing about it.

**Fourteen sites, the counts taken before the mutation.** Each operator
opens with a gated `let before = self.arena_counts();` whose text names
neither spelling:

- `crates/topo/src/euler.rs:1054`, `:1219`, `:1364`
- `crates/topo/src/euler_ring.rs:406`, `:612`, `:744`, `:927`
- `crates/topo/src/euler_kill.rs:417`, `:558`, `:765`, `:1034`
- `crates/topo/src/null.rs:209`
- `crates/topo/src/split.rs:152`
- `crates/topo/src/movefac.rs:67`

`arena_counts` is not incidentally debug-only either: it lives in
`test_support_impl`, which `crates/topo/src/lib.rs:228` declares under
`#[cfg(any(debug_assertions, test, feature = "test-support"))]`. Drop one
of these fourteen attributes and a consumer's release build — no
`debug_assertions`, no `test`, no `test-support` — stops compiling, which
is exactly what the rows are for.

Per-file `assert_euler_postcondition` use counts as measured on this tree
(the code-only view, at identifier boundaries): `euler.rs` 4,
`euler_ring.rs` 4, `euler_kill.rs` 4, `movefac.rs` 2, `null.rs` 1,
`split.rs` 1, `boolean/voids.rs` 1.

## The candidate fix

**For the class, not for the one site.** Adding
`assert_euler_postcondition` alone is a half-fix: it closes
`voids.rs:314` and leaves the fourteen `arena_counts` statements exactly
as blind as they are now. Both spellings go on each of the seven `topo`
rows, and the pins are re-taken over the pair.

## What it costs

The self-test is **quadratic in the subject count**: each subject
contributes a fixed set of cases, and every case runs the gate as a
subprocess over EVERY subject's planted file, so wall clock goes as
(cases per subject × subjects) × subjects. The seven `ArenaDelta` rows
alone took it from 15 s to 72 s. This row proposes seven more spellings
on top of that — spellings add cases linearly rather than quadratically,
so the marginal cost is small beside the row cost already paid, but it is
paid on a base that is already the largest in the directory. One reading,
on one box; nothing re-takes it.

## What the sweep could not match

Eight sites name `assert_euler_postcondition` in prose or in a string
literal rather than calling it, and every one is inside a `cfg(test)`
module, so no row would reach them and none is a use:

- `crates/topo/src/review_d18_probes.rs:258`, `:305`
- `crates/topo/src/review_m1_pr2/release_corruption.rs:277`
- `crates/topo/src/review_m1_pr5_internal.rs:336`, `:369`, `:383`, `:396`
- `crates/topo/src/source_walk.rs:347`

A `topo` file that STARTS calling either spelling is caught by nothing —
that is the gate's KNOWN GAP 7 (a subject is a file), not this row.
