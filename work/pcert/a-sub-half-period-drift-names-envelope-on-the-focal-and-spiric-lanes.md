---
id: a-sub-half-period-drift-names-envelope-on-the-focal-and-spiric-lanes
kind: issue
title: A stored row's azimuth drift below half a period refuses as FidelityU on the harmonic and projected lanes but as the whole Envelope on the focal-section and spiric lanes, whose check 4 is one 3-D number
status: open
opened: 2026-10-19
priority: P4
cost: M
---

Filed by PCERT B2b's second fix pass (G3, from the delta review of
PR 5343).

## What

Every closed-form lane now decides its check-4 verdict through one
helper, `envelope_verdict` (`crates/geom-brep/src/pcurve_cache.rs`),
and names the refusing check from what the lane keeps apart:
- the harmonic and projected lanes keep their terms apart
  (`EnvelopeTerms`), so a stored row whose azimuth drifts 3 rad (under
  half a period) refuses as `EnvelopeTerm(FidelityU)`;
- the focal-section and spiric lanes state check 4 as one 3-D number
  (`focal_section_envelope`; the spiric wall's identity and cap form),
  plus their placement terms. A 3-rad azimuth drift moves the point in
  space, which that one number reads, and no placement term reads it
  (the term starts past half a period). So the same drift refuses as
  `Envelope`, not `FidelityU`.

A whole period or more is labelled alike on every lane
(`FidelityU`/`FidelityV`, the placement terms).

## Why not here

Labelling a sub-half-period drift by channel on those two lanes needs
their 3-D statements split into per-channel terms: the focal section's
`H` coefficients and remainder, and the spiric wall's two drift
channels, each carried as an `EnvelopeTerm`. That is a restatement of
both lanes' envelopes, with their own dominance rows, not a fix-pass
change.
