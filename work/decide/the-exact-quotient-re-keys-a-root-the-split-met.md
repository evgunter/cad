---
id: the-exact-quotient-re-keys-a-root-the-split-met
kind: issue
title: rule G's exact quotient re-keys a root the split met: sqrt(N/D) with D | N no longer meets sqrt(N)/sqrt(D) — the remedy is a canonical factorisation of a root's argument
status: open
opened: 2026-09-24
priority: P2
cost: H
refs: [DECIDE-4, rule-d-reaches-the-unit-bulge-only]
---


**Found by both DECIDE-4 reviews** on `334bb2aa2`. Rule G's exact
quotient (`SymRules::root_quotient`, `crates/geom-core/src/sym/root.rs`,
`exact_quotient_root`) mints a root over `N/D` with `D | N` as `sqrt(Q)`,
`N = Q·D`, BEFORE the split. That re-keys the root away from the split
spelling `sqrt(N)/sqrt(D)` of the same value, which the dial-off tier
meets.

## The two shapes

Both are pinned in `crates/geom-core/tests/decide_4_root_quotient_rows.rs`,
`the_split_spelling_is_what_the_quotient_trades`. Each is a theorem with
the dial off and refused with it on:

- review r1: `sqrt(N/D) − sqrt(N)/sqrt(D)`, `D = 1 + x²`, `N = (2 + x)·D`;
- review r2: `sqrt(N/D)·sqrt(D) − sqrt(N)`, `Q = 1 + x²`, `D = 1 + y²`.

No measured document moves on either shape. The nominal splits of the
plate, bracket, annulus, link, boss, both `0.4` D-tabs and both `0.5`
controls are unmoved by the rule except for the boss's `arc_span`, which
it takes.

## Why no order of the two steps keeps both meetings

The measurement is DECIDE-4's fix pass (and review r1's mutant A1). Asking
the split FIRST keeps these two shapes, and it still takes the MEASURED
boss: `m10_bulge_the_bosss_split_at_the_nominal` passes under it,
because on that document the split declines on the ring and the quotient
is asked anyway. What it loses is ring-independence. Wherever the ring
lets the split through, `sqrt(5p⁶)/sqrt(p⁴)` keys `|p³|/p²` and not
`|p|`, so the boss's shape on a dyadic chord is refused.

Split-first reds four rows of `decide_4_root_quotient_rows`:
- `the_bosss_shape_folds`, the dyadic twin;
- `a_sign_carrying_quotient_root_stays_a_magnitude`, a square over a
  squared denominator;
- `the_quotient_spelling_with_a_signed_denominator_meets`;
- `the_split_spelling_is_what_the_quotient_trades`.

The reason the quotient stays first is that its key does not depend on
the ring width, and those four rows.

## The remedy

A canonical factorisation of a root's argument: `sqrt(Q·D) =
sqrt(Q)·sqrt(D)` and `|P^k| = |P|^k`, keyed on the square-free parts.
With it, both spellings key the same atoms. That is a multivariate
factorisation, and it is not in DECIDE-4.
