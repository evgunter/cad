---
id: ordering-asserts-outside-mesh-unswept-for-illegible-domination-messages
kind: issue
title: The domination-message sweep read only crates/mesh in full: 457 message-less and 2129 unspecced ordering asserts elsewhere are unread
status: open
opened: 2026-09-18
---

## Finding

`tess/domination-assert-messages` (PR 2848) swept for one shape: an assert
whose condition is an ordering comparison between a bound/ceiling and a
truth, decidable by the last bits, whose message (a) prints two groups of
numbers without saying which is which, (b) prints an `f64` at fewer than 17
significant digits, or (c) does not exist. That shape cost
`work/tess/nurbs-face-bound-unsound-on-a-random-rational.md` two weeks of
misdiagnosis (its "CORRECTION" section).

**The sweep was complete only inside `crates/mesh`.** There all 200
ordering asserts were read by hand. Outside it, the same extraction over
`crates/`, `tools/`, `demos/`, `benches/` and `interval-transcendentals/`
found 2634 more, in three classes, and only the first was read:

| class | count | read? |
|---|---|---|
| message carries a `{…:.N}` / `{…:.Ne}` spec with N < 17 | 48 | yes — the hits are the two rows below |
| no message at all | 457 | **no** |
| a message with no precision spec (so lossless if it prints numbers — but an unlabelled pair, or a message with no numbers, is in here) | 2129 | **no** |

Counts are as of `17fc3bfda`. The two rows the read class produced:
`work/tint/enclosure-asserts-print-too-few-digits-to-show-a-last-bit-red.md`
and `work/instr/derivation-ceiling-asserts-print-percentages-to-five-decimals.md`.
Closing either of those does not discharge this row.

## What is owed

Read the 457 + 2129 for the shape. Most will be dismissed quickly — a
count pin, a positivity check and a `rel < 1e-12` tolerance are not
dominations — and the expected yield is concentrated where certificates
live: `crates/geom-brep` (`patch_bound`, `props`, the `cert*` suites),
`crates/geom-core`'s interval rows and `interval-transcendentals`.

## How to reproduce the extraction

Every `assert!` / `debug_assert!` whose FIRST argument, after stripping
`->`, `=>`, `<<`, `>>` and turbofish generics, contains `<=`, `>=`, `<` or
`>`; string literals are skipped when balancing parentheses and splitting
arguments.

## What the extraction cannot see, anywhere

`assert_eq!`-family rows; `if … { panic!(…) }`; `.expect(…)`; comparisons
made through `partial_cmp` / `total_cmp` / `.max()` / a helper returning
`bool` (which now includes `mesh`'s own `Domination::holds()` rows);
messages assembled by `format!` into a variable first. And criterion (a),
unlabelled-ness, is a reader's judgement and not a match.
