---
id: the-negative-arms-denominator-clause-widens-with-decide-3s-definite-quadratic
kind: issue
title: DECIDE-3's definite_quadratic widens nonneg_poly, and through it rule F's negative-arm denominator clause: a cross-product neither unit measured
status: closed
opened: 2026-09-21
priority: P2
cost: D
refs: [derived-frame-placement-freezes-on-the-symbolic-lane, coefficient-ring-width-is-not-monotone-in-reach]
closed: 2026-10-01
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

## Measured at the merge (LINALG, `props/sign-hull` + `main`, 2026-10-01)

Release, ε = 1e-9. The differential is `f_shut` (rule F's two arms
shut, rule G and the read ON — added to `m10_10_evidence_interval`'s
`rules_named`), because on this tree `no_f` (`without_rule_f`) shuts
rule G and the decision read with F and so measures three rules at once.

**The eight documents: nothing moves.**

- Splits at the nominal, `f_shut` → shipped: identical predicate by
  predicate on seven documents — plate `[811, 0, 140, 462]`, bracket
  `[1105, 21, 144, 783]`, annulus `[328, 0, 140, 209]`, link
  `[545, 0, 108, 509]`, segment boss `[375, 2, 96, 233]`, D-tab literal
  `[571, 0, 84, 422]`, D-tab parameter `[571, 0, 80, 426]`. The pad's
  split was not taken: the shape report's render of it was killed at
  388 s on a 15 GB box.
- Ceilings and the over-band set at ceiling + δ: identical on all
  eight, to every printed digit. Brackets (·ε): plate 2.6306e8 ..
  2.6316e8, bracket 3.8716e2 .. 3.8731e2, annulus 8.4161e8 .. 8.4193e8,
  pad 2.4999e3 .. 2.5008e3, link 4.9313e2 .. 4.9332e2, segment boss
  7.2665e8 .. 7.2694e8, D-tab literal 5.6110e8 .. 5.6131e8, D-tab
  parameter 3.5218e2 .. 3.5232e2.

So `definite_quadratic`'s wider denominator clause opens no `abs` atom
the narrower one declined, on any of the eight.

**The one-sided documents: rule F reaches nothing, because the basis
mints no `copysign`.** `Vec3::orthonormal_basis` on this branch is the
axis-order construction; the `copysign(1, n.z)` the negative arm was
built to read is gone. With `f_shut` as the off dial the ladder
(`sym12_phase1_the_one_sided_documents_ladder`) reads the same receipt
at both dials on every document, both lifts: start cap, `FlipZ`, end
cap, `tiltNZ`, `tilt-v` `604 / 272 gated / 446` (`Pinned`) and
`572 / 400 gated / 510` (`Guided`, certifies); `tiltUV` `604 / 30 /
688` (`Pinned`) and `540 / 0 / 292` refusing `carrier_endpoint_start`
(`Guided`). The start cap and `FlipZ` read the end cap's receipt to the
count, at both lifts, with or without the arm. `Guided` ceilings
(`half`): start cap 4.2034e-3, `FlipZ` and end cap 1.5536e-3, `tiltNZ`
and `tilt-v` 1.4445e-3.

**What does move is the decision read** (`no_reads` as the off dial):
with it shut, `Pinned` reads `876 / 0 / 446` on those five (the
theorems the shipped tier counts as 604 + 272 gated), the start cap's
`Guided` certifies on 972 theorems, and `FlipZ`, the end cap, `tiltNZ`
and `tilt-v` refuse `Guided` on `newell_plane_residual` — the read is
what carries them past the Newell wall, and also what relabels
theorems as `sign_gated`
(`work/decide/the-decision-read-answers-theorems-the-must-carry-stations-would-prove`).
Rule G (`no_g`) costs `tiltUV`'s `Pinned` 92 numeric decisions
(596 → 688).

**The gating row, as it stood at the measurement.** The gating row
`m10_the_start_cap_and_flip_z_read_the_end_cap_under_the_negative_arm`
reds: it pins the Duff-era mechanism — F off refuses
`carrier_endpoint_end` `28/0/0/1`, F on takes it to `33/0/0/0` and
stops on the Newell straddle `32/0/0/1`. On the merged tree F off
(`without_rule_f`) reads `25/0/0/1` (start cap) and `24/0/0/1`
(`FlipZ`), and the shipped tier certifies both with
`carrier_endpoint_end` `[32, 16, 0, 0]` and `newell_plane_residual`
`[24, 24, 0, 0]`. `sym12_a_negative_nz_the_arm_folds_and_does_not_reach`
(evidence) likewise: `FlipX` at `half = 2e-3` refuses on the shipped
tier at a wall join, not the Newell straddle. Nothing went down — the
documents certify where `main` refused — but the row's subject (the
arm reaching the start cap) no longer exists.

## Closed (2026-10-01)

The measurement this item owed is clean. Nothing went down anywhere,
and the widened `D` clause opens nothing:

- The eight documents' ceilings and over-band sets are identical under
  `f_shut` and shipped, to every printed digit.
- Seven of the eight splits are identical. The pad's split was not
  taken: its shape report was killed at 388 s on a 15 GB box. The pad's
  whole-box receipt at its `certifies_at · ε` box is identical under
  both rule sets (893 / 34 / 150 / 1002), as are the other four
  `m10_9` studies'.

The LINALG orchestrator ruled that the gating row's claim stands and
only its mechanism is gone. It is re-aimed as
`m10_the_start_cap_and_flip_z_certify_as_the_end_cap_does_and_rule_f_is_inert`:

- under shipped rules the start cap and `FlipZ` certify;
- their receipt and their `carrier_endpoint_end` /
  `newell_plane_residual` splits equal the end cap's, computed in the
  row;
- rule F's arms shut alone moves nothing on all three.

`FlipX` and `FlipV` are carried the same way by
`sym12_rule_f_is_inert_on_the_reviews_negative_nz_documents`.

That the negative arm now has no document consumer is filed as
`the-negative-arm-lost-its-document-consumer`.
