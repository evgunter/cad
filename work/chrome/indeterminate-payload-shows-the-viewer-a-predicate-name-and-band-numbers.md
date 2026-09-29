---
id: indeterminate-payload-shows-the-viewer-a-predicate-name-and-band-numbers
kind: issue
title: chrome: every escalation the viewer draws names its predicate and the band's numbers through IndeterminatePayload; the standard does not say whether that is the user's or the developer's
status: open
opened: 2026-09-28
priority: P1
cost: E
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
