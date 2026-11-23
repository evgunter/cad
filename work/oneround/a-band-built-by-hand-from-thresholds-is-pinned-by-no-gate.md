---
id: a-band-built-by-hand-from-thresholds-is-pinned-by-no-gate
kind: issue
title: a band built by hand from the band's own thresholds is pinned by no gate
status: open
opened: 2026-11-22
priority: P2
cost: M
---


Found by both designers of the fork on the ten readings that are not
levers (`docs/DESIGN-FORK-LOG.md` row 303, the `[ev]` PR on
`oneround/ev-readings-not-levers`).

Once `Lever::unsourced` is deleted, the remaining routes for a site's
own number into a decision are `Band::new(any, any)` and
`Margin::of(x * y)`. Neither is gated. Production already builds bands
by hand from the band's own thresholds:

- `topo::chart_region::in_band_premise`: `Band::new(band.zero()/2, band.escalate())`.
- `topo::census::span_clear_as`: `Band::new(escalate − zero, escalate)`.
- `step_import::interpret::file_band` / `band_at`: `Band::new(e, k·e)` at
  a file's ε, which is the hidden `Band::linear_at` written out in
  production. Whether it should call a named import door is a question
  of its own.

Owed:

1. Named constructors in geom-core for the derived bands the fork
   introduces (the band at the tolerance a refusal offers,
   `Indeterminate::offered_band`; the band one K-step finer), so no site
   builds either by hand. These land with the unit that converts the
   ten.
2. A per-file pin on the remaining production `Band::new` references,
   each with its reason, as the lever doors are pinned. Count them with
   the gate's statement view, which drops `cfg(test)` and test-only
   modules: most `Band::new` references under `src/` are in test
   modules, and a line grep overstates them by two orders.
3. `scripts/gates/lever-sources.sh`'s "what review holds" line names a
   tolerance lifted into a product, with a pointer to the from-f64
   gate's rows (27 rows mention a band threshold; four were this fork's).
4. `Band::padded`'s rustdoc points at `k_stats::decide_across` for a pad
   in the run's scalar: the stand-ins implementer did not find that door.

Lands no later than the unit that deletes `Lever::unsourced`, since
that deletion is what makes `Band::new` the open route. A gate is
enforcement of ratified text, so this does not wait for a sign-off.

Also worth one grep in the same pass: `Margin::levered(..).value()`
used as a multiplication (read straight back, never decided). Seen at
`intersect::pair_bound` (three) and `ssi/certify.rs`; `ssi/march.rs`
was named and not confirmed. Neither gate sees it.
