---
id: the-negative-arms-denominator-clause-widens-with-decide-3s-definite-quadratic
kind: issue
title: DECIDE-3's definite_quadratic widens nonneg_poly, and through it rule F's negative-arm denominator clause: a cross-product neither unit measured
status: open
opened: 2026-09-21
priority: P2
cost: D
refs: [derived-frame-placement-freezes-on-the-symbolic-lane, coefficient-ring-width-is-not-monotone-in-reach]
---


## What

Filed by SYM-12's fix pass (both reviews named the seam; R1 n5, R2
claim 12). Two units change `geom_core::sym::manifest` on branches
that have not met:

- **SYM-12** (`sym/12-negative-arm`, PR #3046) adds rule F's negative
  arm: `manifest::negative(N/D)` is `positive((−N)/D)`, and `positive`
  reads `nonneg_poly` for the denominator `D`.
- **DECIDE-3** (`props/sign-hull`, PR #3039) widens `nonneg_poly` with
  `definite_quadratic` — a quadratic the syntax shows non-negative by
  its discriminant, not term by term.

Because `negative` is defined as `positive` of the negated numerator
and inherits `positive`'s denominator clause whole, DECIDE-3's widening
of `nonneg_poly` widens the NEGATIVE arm's denominator clause
automatically, on the day the two branches meet — a cross-product
neither unit measured: SYM-12 measured its arm against `main`'s
`nonneg_poly`, DECIDE-3 measured its widening against `main`'s rule F,
which had one arm. The widening is sound by the same argument
(`D ≠ 0` wherever the form has a value, clause 1's), so this is not a
soundness row; it is a measurement owed and a merge that will not be
clean.

## The hunks that will conflict (textual, at the time of filing)

- `crates/geom-core/src/sym/manifest.rs`: the module header (SYM-12
  rewrites the invariant, the "where they come from" list and the
  predicate sections; DECIDE-3 touches the non-negativity section and
  its argument) and the `positive` / `fold_abs` / `magnitude` block
  (SYM-12 adds `negative`, routes `magnitude` through `fold_abs`;
  DECIDE-3 changes `nonneg_poly`'s neighbourhood).
- `crates/geom-core/src/sym.rs`: the rule-F header section's reach
  paragraph, the `manifest_sign` dial doc, the rule table's rows, and
  `combine`'s `Copysign` arm (SYM-12 adds the negative branch;
  DECIDE-3 re-shapes `combine`'s kids).
- `crates/editor-core/tests/m10_derived_frame_tilted_interval.rs`:
  the `Base`/`Place` docs and the row list (SYM-12 adds `FlipV`,
  `FlipX`, three rows; DECIDE-3 its own).

## What the merge owes

Whoever merges `props/sign-hull` into `main` after SYM-12 (or the
reverse) re-takes, with BOTH units in:

- the one-sided ladder (`sym12_phase1_the_one_sided_documents_ladder`)
  and the gating row
  `m10_the_start_cap_and_flip_z_read_the_end_cap_under_the_negative_arm`
  — the arm must still read the end cap's state by name and by count
  on the start cap, `FlipZ` and `FlipX`, and move nothing on the end
  cap, `tiltUV`, `tiltNZ`, tilt-`v` and `FlipV`;
- the eight measured documents' splits at the nominal and their
  whole-certifying brackets with the over-band sets at ceiling + δ
  (`m10_10_splits_at_the_nominal_under_a_rule_set`,
  `m10_10_ceilings_and_the_over_band_set`, both arm states are now one
  dial: `no_f` against shipped), against the ring item's acceptance —
  no split or ceiling down anywhere — since a wider `D` clause lets
  the negative arm open an `abs` atom over a quadratic denominator the
  narrower one declined, which is exactly the class
  `work/sym/coefficient-ring-width-is-not-monotone-in-reach` records.

## Home

SYM. Filed at SYM-12's fix pass (2026-09-21).
