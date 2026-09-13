---
id: SYM-2
kind: unit
title: the tier's file split: the coefficient tower and the polynomial out of sym.rs, the header distributed with them
status: spec
opened: 2026-09-13
branch: sym/2-split
refs: [sym-rs-is-one-file-with-a-347-line-header]
---


## What

`sym-rs-is-one-file-with-a-347-line-header`'s three moves: the
coefficient tower (`Int`, `Rat`, `gcd_u128`, `isqrt_u128`, `COEFF_BITS`)
to `sym/rational.rs`, the polynomial (`Poly`, `Mono`, `Form`, `within`)
to `sym/form.rs`, and the header distributed with them so each
contract sits beside the code it binds; then the header's archaeology
cut to present-tense invariants, each cut listed for the reviewer.
Pure moves — no logic edit, no rename with an outside caller. Outside
the A/B experiment (hygiene; one style review, no row). Spec:
`docs/SYM-2-SPEC.md`. Lands before SYM-1, which re-sites its counters
on the split tree.
