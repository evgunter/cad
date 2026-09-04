---
id: bit-identity-debug-only-gate-ends-an-item-at-a-semicolon
kind: issue
title: scripts/gates/bit-identity-debug-only.sh ends a gated item's read at the first ';' before its brace, so a correctly gated fn with an array type in its signature is reported ungated
status: open
opened: 2026-09-04
---

## What

`scripts/gates/bit-identity-debug-only.sh` (`debug_only_report`, the awk
at lines 64–103) decides whether an `eq_bits` use sits inside a
`cfg(debug_assertions)` item by walking the code-only text delimiter by
delimiter: after the attribute it sets `gated = 1; seen = 0`, and at the
next `{` marks the item entered (`seen = 1`). But its `;` branch —
`else if (gated == 1 && seen == 0) gated = 0` (line 98) — reads ANY
semicolon before the item's first brace as "the attribute's item ended
without a body", so a gated `fn` whose SIGNATURE carries a `;` is
reported ungated: an array-typed parameter (`pairs: [(T, T); N]`), a
const-generic default, or a `where` clause with a semicolon-bearing
type.

## Where it fired

DOCM-2 (PR #1860, run 33911339387, the `discipline (evaluation-code)`
job): `crates/topo/src/source.rs`'s shared fold
`fn bits_witness<T: geom_core::Real, const N: usize>(pairs: [(T, T); N])`,
correctly under `#[cfg(debug_assertions)]`, was reported as
`crates/topo/src/source.rs:209 … uses the bit channel above outside any
cfg(debug_assertions) item`. Every test and gate job on that run was
green; the read was the only red.

## Worked around, not fixed

`b59b2203` made the fold take a slice (`pairs: &[(T, T)]`) so the
signature carries no `;`, and the site's comment cites this issue. The
gate's reader is the defect: the `;` branch should end an attribute's
item only where a `;` can end one (a `use`, a `type` alias — at
parenthesis depth zero and before any `<` … `>` or `[` … `]` of a
signature), or the reader should skip to the first `{` or `;` outside
brackets. Its selftest has no fixture with a `;` inside a gated
signature; one belongs beside `plant_after_the_gated_item`.

## Home

CIW's (the gates are its territory); named here for placement, not
routed.
