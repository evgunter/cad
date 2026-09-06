---
id: debug-only-counters-have-no-gate
kind: issue
title: The debug-only gather counter has no CI gate; the one debug-only gate names a single file by path
status: review
opened: 2026-09-04
track: K
branch: gates/debug-only-subjects
---



## What

DOCM-5 (PR 1871) added a debug-only gather counter to
`crates/editor-core/src/product.rs`: a `thread_local!` cell, an
increment at the top of `product_recorded`, and a
`gathers_on_this_thread` reader, each behind
`#[cfg(debug_assertions)]`. It is the witness the unit's
one-gather-per-landing rows read.

Nothing in CI holds it to that shape. A fourth site added without the
attribute — or the attribute dropped from one of the three — compiles,
passes every test, and puts a counter in the shipped kernel. The only
thing standing there is a source-text row in the unit's own suite
(`crates/viewer/tests/landing_gathers.rs`,
`every_site_of_the_gather_counter_carries_the_debug_gate`), which is a
test asserting about a file in another crate rather than a gate.

The comparison that makes this worth a file: `topo::source`'s
bit-identity witnesses have exactly this obligation and DO have a
gate — `scripts/gates/bit-identity-debug-only.sh`, wired into ci.yml
and `local-scripts/ci-local.sh`, with its own self-test battery. Two
debug-only mechanisms, one guarded and one not.

## Why extending the existing gate is not a one-liner

`bit-identity-debug-only.sh` is single-subject by construction, not by
accident:

- `SUBJECT=crates/topo/src/source.rs` is one path, and
  `gate_require_file "$SUBJECT"` exists because a MISSING subject once
  made the gate green (its own header records that).
- Every one of its eleven self-test fixtures plants that exact path
  (`plant`, `plant_one_gated_one_leaked`, `plant_after_the_gated_item`,
  … each `mkdir -p "$1/crates/topo/src"`).
- `GATE_SCAN_NOUN='bit-channel use'` and the awk scan look for
  `bit_identity::|eq_bits`, not for an arbitrary debug-only symbol.

So the change is: a subject LIST, a per-subject symbol pattern, the
fixtures parameterised, and the self-test cases run per subject. That
is a gate rewrite with its own self-test obligations, not an added
line — which is why DOCM-5's fix pass filed this rather than doing it.

## Shape of a fix

Two candidates:

1. **Generalise the existing gate** to (subject, symbol) pairs, keeping
   its enclosure analysis (brace-depth `cfg` items plus `debug_assert!`
   statements) — that analysis is the valuable part and is worth
   exactly one implementation. The self-test battery parameterises with
   it.
2. **A second gate** over a declared roster of debug-only symbols
   anywhere under `crates/*/src`, with the roster in the gate. Wider
   reach, but it duplicates the enclosure analysis, which is the thing
   the first gate's header argues hardest about getting right.

Either way `scripts/gates/gate-roster.sh` must see the result wired
into ci.yml and `local-scripts/ci-local.sh`.

## Citations

- `crates/editor-core/src/product.rs` — the counter's three gated
  sites.
- `scripts/gates/bit-identity-debug-only.sh` — the gate that exists,
  and its single-subject shape.
- `crates/viewer/tests/landing_gathers.rs` — the source-text row
  standing in for a gate, which says at itself that it pins source and
  not an artifact.

## A thing to know before writing the fix

`[profile.release]` in the workspace `Cargo.toml` sets
`debug-assertions = true` until publish (and `demos/tour` the same), so
`cfg(debug_assertions)` is NOT "absent from a release build" here
today. The gate would be pinning the SOURCE shape — which is the right
subject, and is what the existing one pins too.

No program obviously owns this: it is CI-discipline work over a kernel
crate's source shape. Filed unowned by DOCM-5's fix pass on its dual
review.

## Re-homed (2026-09-05)

Arrived in `work/issues/` after the 2026-09-04 re-home sweep had run
and was routed at that sweep's merge with main. Moved to
`work/code-quality/` with `track: K`: the change it asks for is a
rewrite of `scripts/gates/bit-identity-debug-only.sh` into a
subject-list gate, and `scripts/gates/` less `gate-roster.sh` and
`probe-suite-census.sh` is Track K's fence — CIW's `keep_out` says the
same from the other side. It is the second row on that one script;
`bit-identity-debug-only-gate-ends-an-item-at-a-semicolon` is the
first, and the two want the same file open at once, so they are one
lane and not two.

## Claimed by GATES (2026-09-06)

Moved from `work/code-quality/` to `work/gates/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, `track:` letter and body unchanged. Track K; one lane with `bit-identity-debug-only-gate-ends-an-item-at-a-semicolon`.

## Fixed (2026-09-06, `gates/debug-only-subjects`)

Candidate 1 of "Shape of a fix": `bit-identity-debug-only.sh` now
carries a `SUBJECTS` list — path, symbol ERE, the bare spelling and the
use expression the fixtures write, and the noun the diagnosis names the
mechanism by — with the enclosure analysis (brace depth for `cfg` items,
statement extent for `debug_assert!`) implemented exactly once and every
row run through it. `crates/editor-core/src/product.rs` is the second
row, pattern `GATHERS|gathers_on_this_thread`, which is the cell, the
increment in `product_recorded` and the reader.

`gate_require_file` is per subject and every row is proved present
before any row is scanned, so a subject that moved cannot hide behind
one that stayed; the subject-gone case runs once per row. Every other
self-test case is parameterised on (path, symbol, use expression) and
runs once per row too. The gate reads every subject before failing, so
one red run names all of them. The script's NAME and command line are
unchanged, so `ci.yml`, `local-scripts/ci-local.sh` and
`gate-roster.sh` needed no edit and `gate-roster.sh` stays green.

Not this lane's to change, reported on the PR: `ci.yml`'s step name
still reads "bit-identity debug-only guard (topo/source.rs)", which now
understates the subject list (CIW's file); and
`crates/viewer/tests/landing_gathers.rs`'s
`every_site_of_the_gather_counter_carries_the_debug_gate` — the
source-text row this issue names as standing in for a gate — now has
the gate it was standing in for, so whether it retires (with its
`crates/test-utils/tests/reader_census.rs` ledger line) is S-TCOST's
and VIEW's call.

