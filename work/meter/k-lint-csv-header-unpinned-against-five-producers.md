---
id: k-lint-csv-header-unpinned-against-five-producers
kind: issue
title: k-lint's EXPECTED_HEADER is hand-copied at five producer sites with no pin in either direction
status: open
opened: 2026-09-07
refs: [k-lint-predicate-roster-unpinned, D204]
---


## Was

unrowed. Raised by `k-lint-predicate-roster-unpinned`'s sibling sweep
(2026-09-07), which the item itself asked for: the same shape one
constant over.

## Finding

`tools/k-lint/src/lib.rs:465` holds

```
const EXPECTED_HEADER: &str = "shape,predicate,margin,band_zero,band_escalate,outcome";
```

and calls it "the only header `lint_csv` accepts". That string is
written by hand at **five** producer sites, none of which can see it
and none of which it can see:

- `crates/sweep/tests/k_report.rs:292`
- `crates/editor-core/tests/m4_pr8_k_probe.rs:81`
- `crates/editor-core/tests/m10_3_driver_k_probe_interval.rs:387`
- `demos/tour/src/probe.rs:121`
- `scripts/rundump-guard-selftest.sh:60`

plus a sixth copy inside k-lint's own suite (`tests/cli_contract.rs:15`,
a `const HEADER` that restates the constant rather than importing it —
`EXPECTED_HEADER` is private, which is why).

`FLOAT_COLUMNS` (`:574`) is pinned to `EXPECTED_HEADER` *inside* k-lint
by `the_policed_block_is_the_headers_float_block`, so the column-block
half is sound; what is unpinned is the boundary the block is stated
across.

**Why this is the class `D204` named and not a fresh one.** The sibling
`ACCEPTED_OUTCOMES` — the other vocabulary this tool shares with the
kernel — IS pinned, variant by variant, in `tests/outcome_vocabulary.rs`,
because `geom_core::k_stats::SampleOutcome` is a type k-lint can import.
That test's own header says what it does not cover: *"It does NOT check
that the sweep's CSV writers call `token()`; that is a separate claim,
and the writers are ordinary test harnesses in the kernel workspace
which this crate cannot see."* This item is that separate claim, one
column wider. `tess-lint`'s `EXPECTED_HEADER` has exactly the pin this
one lacks (`tools/tess-meter/tests/derivations.rs:1027`), from the
producer's side.

**How it fails, and why loud is not the same as seen.** A producer that
adds or renames a column has every row refused as harness breakage, and
that refusal is deliberate. It is also precisely the failure the
`SymbolicZero` drift produced, where the E6 driver population was linted
zero times for a stretch of commits while the CI row still reported
success — `tests/outcome_vocabulary.rs`'s header is the write-up. Loud
in the tool is not loud in the report.

**Not this unit.** The roster-pin lane owns `tools/k-lint` and read this
out of it, but four of the five producers are outside METER's fence:
`crates/*/tests` are the owning crates', `demos/` is Track X's, and
`scripts/` is CIW's. Which side carries the pin is the fence that has to
be drawn before a lane — `D204`'s answer was the producer's side, and
five producers make that answer expensive; the consumer side is one
`include_str!` per producer over a path table, which is what
`tools/k-lint/tests/predicate_roster.rs` now does for the predicate
roster and could be extended to do here.
