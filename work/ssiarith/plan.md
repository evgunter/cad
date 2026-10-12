# SSIARITH — the plan

Opened 2026-10-08 by SSI's close (`work/README.md`, Track size: the
rest of a closing track goes to successor programs). Nothing
dispatched.

## The slate

**24.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P3 | `chart-point-signs-are-f64-evaluations` | M | the chart lane's point signs are f64 evaluations read as certified, so a zero within rounding of a window corner or cut can take a sign |
| P3 | `limb3-carrier-ends-read-at-f64-points` | E | limb 3's end checks read the carrier's ends as f64 point evaluations, not enclosures of the exact ends |
| P3 | `limb-2-frame-drift-is-a-third-of-its-floor-and-hides-sub-floor-frame-reading-at-eps-1e-12` | M | plane × NURBS limb 2 moves ~1e-14 m under a rotation (a third of its 3.2e-14 floor, every eps), so at eps 1e-12 the rigid-map row passes a frame-reading defect under ~3% of the bound; limb 1 resolves ~0.15% there |
| P3 | `plane-nurbs-limb-2-floor-has-an-absolute-part-that-does-not-shrink-with-the-model` | M | plane × NURBS limb 2's delta = 0 floor reads 5.0e-14 m on a 1 mm model, about 1500x what scaling the 1 m reading gives, so the bound carries an absolute part that does not shrink with the part |
| P3 | `plane-nurbs-limbs-under-translation-drift-with-the-coordinates` | M | under a translation the plane × NURBS limbs drift with the coordinates' magnitude (limb 2 at 5.6-8.2 floors under 1 km), so the rigid-map row covers rotations about the origin only |
| P3 | `ssi-closure-pair-reads-both-charts-as-metres` | E | ssi: the march's closure pair reads the ℝ⁴ state, both charts, where its margins claim the carrier in metres |
| P3 | `ssi-a-chart-rung-can-be-narrower-than-eps-in-metres` | — | ssi: a chart tube rung's metre width is the rung times local over sup speed, so the tube can be narrower than eps in metres |
| P3 | `ssi-boundary-strip-sine-divides-by-the-whole-walls-speed` | — | ssi: the boundary strip's sine divides by the whole wall's sup speed, not the strip's own |
| P3 | `ssi-limb-three-chart-sine-reads-the-chart-perpendicular` | — | ssi: limb 3's chart sine is read along the chart-perpendicular direction, not the surface-perpendicular one |
| P3 | `ssi-window-check-reads-the-walls-control-hull` | M | ssi: plane x NURBS refuses WindowShortOfWall from the wall's control hull, which can reach four times past the wall, and says the wall reaches that far |
| P3 | `the-march-domain-diagonal-mixes-state-units-on-the-chart-lane` | M | ssi/march: MarchContext::diagonal is a length in mixed state units on the R4 lane (plane-chart metres and wall parameters), and it is now the binding step cap on straight through_seed branches |

## Order

The units rows (`ssi-closure-pair-reads-both-charts-as-metres`,
`ssi-a-chart-rung-can-be-narrower-than-eps-in-metres`,
`the-march-domain-diagonal-mixes-state-units-on-the-chart-lane`) share
one question, what a chart length is in metres, and run together. The
f64-read rows (`chart-point-signs-are-f64-evaluations`,
`limb3-carrier-ends-read-at-f64-points`) are the same repair in two
places. The three limb 2 floor rows are one measurement and go last.

## Review posture

OPEN, for this program's first dispatch, under
`docs/DUAL-REVIEW-PROTOCOL.md` as SSI's was.
