---
id: indeterminate-payload-shows-the-viewer-a-predicate-name-and-band-numbers
kind: issue
title: chrome: every escalation the viewer draws names its predicate and the band's numbers through IndeterminatePayload; the standard does not say whether that is the user's or the developer's
status: closed
opened: 2026-09-28
priority: P1
cost: E
closed: 2026-09-29
pr: PRNUM
---

(ENCL implementer, raised by the review of PR 3347.)

## What

`geom_core::predicate::IndeterminatePayload`'s `Display`
(`crates/geom-core/src/predicate.rs`) renders "predicate
'offset_normal_floor' indeterminate: margin 5e-9 lies inside the
ambiguity band (1e-9, 1e-8)". Every escalation the viewer draws carries
that clause, whether it forwards `Indeterminate` whole or composes
`payload()` with its own recourse. Examples: `MeterError::Escalated`
(`crates/geom-brep/src/offset_meters.rs`), the routed
`BlendError::Escalated`, `PathError::Escalated`, and `pcurve_cache.rs`'s
`FittedEscalated`.

The refusal standard ("The standard a refusal is rewritten to" in
`work/chrome/error-and-check-text-overflows-its-region.md`) lists
"predicate routing" and "dispatch tables" as developer detail. It says
nothing on whether a predicate's static name, or the band's two
thresholds, belong to the user's sentence or to the payload `Debug`
carries. So each rewrite takes whatever the payload renders.

## The question

Decide it once, at `IndeterminatePayload`, which every one of these
sites reads. For example, the payload's `Display` could keep the margin
and band in words, or drop them, while `Debug` keeps the name. Either
way the change is one edit in `geom-core`, followed by a re-baseline
of the rows that pin the text.

## Closed 2026-09-29 (`chrome/refusal-residue`)

**Decided at `IndeterminatePayload`: the name is the developer's, the
numbers are the user's.** A predicate's static name is routing, which
the standard lists as developer detail, and nothing the person holding
the mouse can act on. The margin and the band stay: they say how close
the call was, and the tolerance a recourse quotes (`below 5e-10 m`) is
read off them.

- `geom_core::IndeterminatePayload`'s `Display` no longer opens with
  "predicate 'x' indeterminate: " or "sign indeterminate: ". It reads
  "margin 5e-9 lies inside the ambiguity band (1e-9, 1e-8)"; the name
  stays on `Indeterminate::predicate` and in `Debug`.
- `topo::BooleanError`'s coincidence arm dropped the same name from its
  definite-zero sentence ("predicate 'x' definite: …").
- `geom_core::MissingRecourse` still names its predicate. That is the
  sentence's subject (which decision has no routed recourse), not a
  label on someone else's sentence, and it is reached only on a
  routing gap.

The tests that told escalations apart by the name in the sentence now
read it off the typed refusal's `Debug`: `step-import`'s `tier_gate`
(a new `Escalated(predicate)` disposition) and
`cert1_r1_import_probes`, `profile`'s `bool11_probes`, `bool12*` and
`rejections`, `sweep`'s `bool3_torus_doors`, `m5_pr5_tilted_cut`,
`review_m5_pr9_inband_at_rest` and `review_recourse_roster_r2_probes`,
`topo`'s `review_ssiflat_r1_probes` and `chord_join`, and
`editor-core`'s interval replays (their failure strings now carry the
`Debug` after the sentence). `dsc_checks` and `review_m0_pr3` now
assert the name is absent from the sentence.
