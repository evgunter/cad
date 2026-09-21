# BAND — the plan

the blend and rim bands: what a roll builds over a rim, and what it may call the result

Opened 2026-09-20 by CARVE's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**27 budget points** of dispatchable work against a ceiling of 30.

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

## Order

`ruled-band-keys-a-d-hole-rim-on-the-caps-outer-cycle` first: a
D-shaped through-hole is an ordinary feature and the ruled band refuses
its crease outright. `blend-slit-name-collides-when-two-rims-share-a-meridian`
is the second refusal and independent of it, so the two can run at
once.

`S90-impl` (tightening the blend seam's three doors to
`CertifiedBounds`) is the ruled row and the largest; it is specified
from Ev's ruling rather than re-argued. The three recourse/policy rows
are one unit: they are the same fact — a table asserting an answer
where the question was not asked — at three sites.

## Review posture

OPEN, for this program's first dispatch. CARVE inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.
