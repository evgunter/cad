---
id: debug-only-counters-have-no-gate
kind: issue
title: The debug-only gather counter has no CI gate; the one debug-only gate names a single file by path
status: open
opened: 2026-09-04
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
