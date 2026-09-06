---
id: debug-only-assert-euler-postcondition-is-on-no-row
kind: issue
title: assert_euler_postcondition is a second spelling of the arena-delta mechanism and is on no subject row
status: open
opened: 2026-09-06
refs: [debug-only-reader-cannot-place-a-statement-attribute-over-a-braced-call]
---


## Finding

Found by the `gates/statement-attribute-items` lane while measuring the
seven `ArenaDelta` rows it added to
`scripts/gates/bit-identity-debug-only.sh`.

The debug-only arena-delta mechanism has TWO spellings, not one. The rows
that landed pin `ArenaDelta`; the other half of the mechanism is
`crates/topo/src/euler.rs:2238`'s `assert_euler_postcondition`, itself a
`#[cfg(debug_assertions)]` `pub(crate) fn`, and it is on no row's spelling
list.

For sixteen of its seventeen uses that costs nothing, because the
statement naming `assert_euler_postcondition` also names `ArenaDelta` and
the `ArenaDelta` row already places it. One does not:

- `crates/topo/src/boolean/voids.rs:314` —
  `dst.assert_euler_postcondition(before, transplant, "insert_void");`
  passes a binding, so the call names no `ArenaDelta` and the
  `boolean/voids.rs` row (which pins the struct literal at `:291`) says
  nothing about it. Drop its `#[cfg(debug_assertions)]` and the gate stays
  green while a release build of the crate stops compiling for a consumer.

Per-file use counts as measured on this tree (the code-only view, at
identifier boundaries): `euler.rs` 4, `euler_ring.rs` 4, `euler_kill.rs`
4, `movefac.rs` 2, `null.rs` 1, `split.rs` 1, `boolean/voids.rs` 1.

## The candidate fix

Add `assert_euler_postcondition` to each of the seven `topo` rows'
spelling lists and re-take their pins (10→14, 10→14, 9→13, 4→6, 3→4, 3→4,
1→2; total 40→57). It is one edit inside this program's fence.

**What it costs, and why this lane did not take it.** The self-test plants
a leak per spelling per subject and runs each case as a subprocess over
every subject, so its wall clock is already 15 s → 72 s from the seven
rows alone; seven more spellings add to that. The lane's brief scoped it
to `ArenaDelta`, and widening the symbol set past the brief is the
orchestrator's call, not a lane's.

## What the sweep could not match

`crates/topo/src/review_m1_pr5_internal.rs:383` and
`crates/topo/src/source_walk.rs:347` name `assert_euler_postcondition`
inside a string literal and a doc comment respectively; neither is a use,
and neither file would be a row. A file that starts calling it is caught
by nothing — that is the gate's KNOWN GAP 7 (a subject is a file), not
this row.
