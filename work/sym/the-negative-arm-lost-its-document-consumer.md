---
id: the-negative-arm-lost-its-document-consumer
kind: issue
title: Rule F's negative arm has no document consumer since the axis-order basis stopped minting copysign(1, n.z)
status: open
opened: 2026-10-01
priority: P2
cost: M
design: true
refs: [the-negative-arms-denominator-clause-widens-with-decide-3s-definite-quadratic, interval-orthonormal-basis-sign-hull, SYM-12]
---

## What was measured (LINALG's merge of `main` into `props/sign-hull`, 2026-10-01)

SYM-12 built the negative arm (`geom_core::sym::manifest::negative`,
read from `combine`'s `Copysign`/`Abs` folds) for one document class:
a `FaceFrame` whose normal has `n.z = −1/sqrt(P(t))` (the tilt-`u`
cube's start cap, `FlipZ`, `FlipX`), where Duff's
`Vec3::orthonormal_basis` minted `copysign(1, n.z)` and `abs(n.z)`.
`props/sign-hull` replaces that basis with the axis-order construction,
which transfers no sign and so mints no `copysign`.

Release build at ε = 1e-9. The rule set with rule F's two arms shut and
every other rule as shipped (`m10_10_evidence_interval`'s `f_shut`)
reads exactly what shipped reads on every document that exercises
rule F end to end:

- the five `m10_9` studies at their `certifies_at · ε` boxes (plate,
  annulus, link, bracket, pad): identical receipts;
- the eight `m10_10` documents: identical splits at the nominal (seven;
  the pad's shape report was not taken) and identical ceilings and
  over-band sets;
- the one-sided ladder (start cap, `FlipZ`, end cap, `tiltUV`,
  `tiltNZ`, `tilt-v`) at both lifts, and `FlipX` and `FlipV`
  (`sym12_rule_f_is_inert_on_the_reviews_negative_nz_documents`).

The gating row is now
`m10_the_start_cap_and_flip_z_certify_as_the_end_cap_does_and_rule_f_is_inert`.
It asserts that rule F is inert, so the arm's only document-level row
pins that the arm does NOTHING. What still exercises
`manifest::negative` is unit-level only: `sym_rule_f_rows` and
`sym_rule_f_interval_rows` in `geom-core`. Those are not a consumer.

By grep (`without_rule_f`, `manifest_sign`, `manifest::negative`)
the other document rows that toggle rule F are `m10_9_the_pads_four_at_both_dials`
(evidence, `#[ignore]`d), which toggles `without_rule_f` (rule F
together with rule G and the read), and `sym_9`'s retry mask. With
rule F's arms alone shut, the pad reads the same as shipped, so the
stored claim in that row's doc (rule F moved four of the pad's
theorems into the door) no longer describes rule F on this tree.

The positive arm is inert on the same documents too. So rule F as a
whole has no measured document consumer. The `abs` atom the new basis
still mints (`|n.z|` in its axis comparison) is evidently not one the
arms need to open on these documents.

## The question

Does the negative arm (and rule F) still earn its place? It costs one
coefficient-sign scan per declined node, and SYM-12 measured that at
noise. Its standing argument (`manifest.rs`'s header: "an `abs` or
`copysign` atom over a form the syntax shows signed is the number the
form already denotes") is about the functions, not about any
construction. Keeping it is cheap. Retiring it removes a dial and its
rows. A document that needs it again would show up as a
`f_shut`-vs-shipped difference.

## Home

SYM: the rule and its dial are `geom_core::sym`'s.
