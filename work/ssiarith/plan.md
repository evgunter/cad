# SSIARITH — the plan

Opened 2026-10-08 by SSI's close (`work/README.md`, Track size: the
rest of a closing track goes to successor programs). Nothing
dispatched.

## The slate

**33 budget points** of dispatchable work against a ceiling of 30: over,
with the rows filed since the cut that the table below does not list.

| pri | item | cost | title |
|---|---|---|---|
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
f64-read row (`limb3-carrier-ends-read-at-f64-points`) is the repair
`chart-point-signs-are-f64-evaluations` (closed) made in `rect_box`,
at the carrier's ends. The three limb 2 floor rows are one measurement and go last.

## Review posture

OPEN, for this program's first dispatch, under
`docs/DUAL-REVIEW-PROTOCOL.md` as SSI's was.

## SSIMARCH's slate, merged in 2026-11-02

SSIMARCH merged into SSIARITH on 2026-11-02 (`docs/doc-ledger/ssimarch-leaves-the-tracker.md`). Its plan follows as it stood, headings one level down; its rows are now in `work/ssiarith/`.

Opened 2026-10-08 by SSI's close (`work/README.md`, Track size: the
rest of a closing track goes to successor programs). Nothing
dispatched.

### The slate

**34.5 budget points** of dispatchable work against a ceiling of 30:
over, with the rows filed since the cut that the table below does not
list.

| pri | item | cost | title |
|---|---|---|---|
| P3 | `limb3-ladder-stops-on-a-certified-second-arc` | M | limb 3's ladder probes every rung after a certified second arc within the narrowest rung's reach; such an arc could end the ladder |
| P3 | `ssi-a-coincident-wall-or-side-crossing-refuses-the-cell-budget` | M | ssi: a plane holding a whole wall, or a wall side the locus crosses, refuses CellBudget, whose ending names neither |
| P3 | `ssi-match-exit-picks-the-crossing-nearest-a-chord` | M | ssi/ends: match_exit pairs an uncapped march's exit with the unused crossing nearest where the leaving chord meets the side, a proximity guess; settle the exit onto the side and match by identity |
| P3 | `ssi-refinement-can-spend-hours-on-one-branch-before-the-step-wall` | M | ssi: refinement's rounds are bounded only by the step wall, about 10^4 rounds of O(n) each, so one branch could take hours before it refuses; and the wall's 20 000 has no stated envelope |
| P3 | `ssi-the-residual-test-samples-a-bent-chart-at-eps-to-the-quarter-along-its-whole-bend` | M | ssi/march: the residual test samples a flat wall whose chart bends at an ε^−¼ density along the whole bend, against main's certificate-local density: 3.5–5.3× main's samples at ε 1e-12 |
| P3 | `the-cylinder-chart-ellipse-pcurve-samples-a-fixed-schedule` | M | pcurve: the cylinder-chart ellipse pcurve is fitted through a fixed 129-sample schedule, a density nothing asks the certificate about; whether it meets small eps on metre-scale cuts is unmeasured |
| P3 | `ssi-limb-three-refuses-a-bent-chart-path-line-at-eps-1e-12` | — | ssi: limb 3 refuses the bent-chart-path flat wall's line at mu 0.1 and eps 1e-12, its pieces stopping short of a carrier end |
| P3 | `ssi-short-branch-uncertified-length-can-name-another-branchs-crossing` | — | ssi: ShortBranchUncertified's length can be the distance to another branch's crossing, not the branch's |
| P3 | `ssi-unsupported-certificate-carries-faults-that-are-not-boundaries` | — | geom-brep: SsiError::UnsupportedCertificate is raised for kernel faults and a degenerate operand as well as per-arm boundaries, so those sites end in no way through yet |

### Order

The refusal-text rows
(`ssi-short-branch-uncertified-length-can-name-another-branchs-crossing`,
`ssi-unsupported-certificate-carries-faults-that-are-not-boundaries`,
`ssi-a-coincident-wall-or-side-crossing-refuses-the-cell-budget`) are
small and run together. `ssi-match-exit-picks-the-crossing-nearest-a-chord`
is a repair with its shape written. The budget rows (the ladder, the
refinement rounds, the residual test's density, the ellipse pcurve's
schedule) share one question, what density the certificate asks for.

### Review posture

OPEN, for this program's first dispatch, under
`docs/DUAL-REVIEW-PROTOCOL.md` as SSI's was.
