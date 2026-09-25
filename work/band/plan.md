# BAND — the plan

the blend and rim bands: what a roll builds over a rim, and what it may call the result

Opened 2026-09-20 by CARVE's priority-seam cut
(`work/README.md`, Track size).

## The slate

**30 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `blend-slit-name-collides-when-two-rims-share-a-meridian` | H | the blend name emitter refuses a roll of two rims whose bands slit ONE seam meridian (RoleSeg::BandSlit has no discriminator) |
| P0 | `ruled-band-keys-a-d-hole-rim-on-the-caps-outer-cycle` | H | sweep: the ruled band refuses a D-shaped through-hole's crease as BodyNotIntact instead of as a frontier |
| P0 | `subdivided-profile-side-coplanar-walls-gate` | D | sweep/topo: a subdivided profile side lowers to two coplanar walls — one surface key, one GeomSource, or does gate_maximal_faces refuse it? |
| P0 | `sweep-emits-no-contact-record-for-declared-cusps` | D | extrude returns a body whose declared cusps have no contact record, so tier 3 must be re-declared by the caller |
| P1 | `S90-impl` | H | Tighten the blend seam's three doors to CertifiedBounds — the ruled S90 implementation, with #883 parked here |
| P1 | `annulus-rim-phase-keeps-a-second-spelling-of-the-split-provenance` | E | blend: the annulus rim phase keeps a second spelling of the split's provenance |
| P1 | `cap-rim-smooth-arm-decides-by-argument-not-by-the-rule` | D | sweep: the cap-rim smooth arm decides a description by an in-code argument, not by the must-carry rule |
| P1 | `corner-config-recourse-and-policy-assert-a-default-for-any-tag` | D | blend: CornerConfig's recourse and policy tables assert an answer for any tag added later |
| P1 | `every-escalation-carries-the-coincidence-recourse-first` | E | blend: every Escalated renders the coincidence recourse BEFORE the routed one, and no blend door takes a declaration |
| P1 | `in-band-corner-verdicts-route-to-the-corner-configuration-recourse` | E | blend: an in-band corner-independence or cap-transverse verdict is routed to FILLET3_CORNER_RECOURSE, which the refused corner already satisfies |

## Order

Four P0 rows first, three at once where their ground is disjoint:

- `ruled-band-keys-a-d-hole-rim-on-the-caps-outer-cycle` — the ruled
  plan (`blend/open/ruled.rs`).
- `blend-slit-name-collides-when-two-rims-share-a-meridian` — the blend
  name record and its emitter (`blend/naming.rs`,
  `editor-core/src/names/{role,emit_blend}.rs`).
- `subdivided-profile-side-coplanar-walls-gate` — measure first, then
  the lowering or the refusal.
- `sweep-emits-no-contact-record-for-declared-cusps` follows the
  subdivided-side row, because both are `extrude.rs`'s lowering. The
  row offers two closes; the one taken is `Extruded` carrying the
  declarations its profile authored (and `Lofted` beside it): a
  declaration the author wrote that a verb drops is the defect, and
  "the caller re-derives the face pairing" is not an API an outside
  consumer can use.

Then the P1 rows. The three recourse rows
(`corner-config-recourse-and-policy-assert-a-default-for-any-tag`,
`every-escalation-carries-the-coincidence-recourse-first`,
`in-band-corner-verdicts-route-to-the-corner-configuration-recourse`)
are one unit: a table asserting an answer where the question was not
asked, at three sites of `blend/mod.rs`. `annulus-rim-phase-…` and
`cap-rim-smooth-arm-…` are small and independent.

`S90-impl` waits on SCALAR's LANE-4 (PR #3194, `PcurveFittedLane`
folds into a `FittedLane` door value), which is the lane-trait split
the row says the tightening turns on; it is re-read against the tree
when that lands, not before.

## Review posture

Per unit, by the review tiers in `memories/orchestration-model.md`;
the model A/B the opening text inherited is suspended, so there is no
triage question left to answer. Each dispatch names its tier in the
log.
