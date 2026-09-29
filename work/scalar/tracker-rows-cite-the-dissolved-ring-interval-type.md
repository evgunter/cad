---
id: tracker-rows-cite-the-dissolved-ring-interval-type
kind: issue
title: Open rows on twelve other slates still name RingInterval, which RING-3 dissolved into Interval
status: open
opened: 2026-09-29
priority: P4
cost: E
refs: [tracker-rows-cite-the-deleted-pcurve-fitted-lane-trait, ring-3-residue-outside-its-fence]
---


## What

The sibling of `tracker-rows-cite-the-deleted-pcurve-fitted-lane-trait`
for RING-3's deletion. `grep -rl RingInterval work/`, less `log.md`,
`STATUS.md`, `work/scalar/` and closed rows, at 2026-09-29:

- NURBS `coefficient-vector-pairing-survivors`
- BAND `S90-impl`
- TESS `chords-m-bound-zero-arm-is-dead-because-the-curve-collapse-has-no-exact-zero-case`
- SHELL `check-rigid-squares-a-column-by-multiplying-two-copies-of-it`,
  `plain-transform-rigid-still-refuses-the-m7-8-class` (lists
  `RingInterval` among the `CertifiedEnclosure` impls)
- PRED `S18`
- TINT `decoration-seam-header-names-no-pin-for-enclose`
- QUAD `C3`
- HELPER `D384`
- COMB `S35`
- GUARD `S41`
- PROPS `ring-refusal-readers-are-spelled-by-hand-at-every-site`,
  `net-refinement-copies-the-differencing-skeleton-and-the-plan-ratio-is-optional`,
  `f64-refinement-inside-an-enclosure-has-five-more-sites`
- SSI `plane-nurbs-ssi-misblames-control-net`,
  `ssi-certify-stretch-divides-by-a-norm-with-no-sqrt-up`

Not read one by one: some are quotations or dated measurements, which
stay as history. The CURVED and CONTACT rows SCALAR-HYGIENE filed name
the type on purpose, as the thing retired.

## Proposed

The fitted-lane row's treatment: re-state each current claim as
`Interval` with its certification doors
(`geom_core::interval::certification::Certification`), `poison` as
`refused`, `is_poison` as `!is_certified()`; where the rename makes a
claim false or moot, a `## Note from SCALAR` line instead.

## Found by

SCALAR-HYGIENE, while carrying the fitted-lane row, 2026-09-29.
