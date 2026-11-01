# NEARPOINT — the plan

Opened 2026-10-27 by the re-cut of FLUX and FLUXTAIL. Together they
held 95.5 budget points against 30, and both shared
`props/curved.rs` and `props/spiric.rs`, so their orchestrators
could not run side by side. The rows were re-cut by logical thread
instead of by priority, so that the five successors can run in
parallel: each mostly owns its own files, and where two share a file
they work on different functions of it
(`docs/doc-ledger/flux-leaves-the-tracker.md`). Nothing dispatched.

## The slate

**24 budget points** of dispatchable work against a ceiling of 30, in 12 rows (2026-10-27).

| pri | item | cost | title |
|---|---|---|---|
| P1 | `a-cone-wall-far-from-its-apex-measures-its-rounding-at-the-apex-lever` | M | A cone wall far from its apex sums its vector area about the apex, so a rigid turn reads the rounding at the apex's lever: a drafted revolve measures 2.6% wrong with pad 0 |
| P1 | `a-cylinder-wall-reads-its-vector-area-at-the-axis-origins-radial-lever` | M | A cylinder wall's closed form sums its vector area about the axis origin, a radius from the wall, so a turned wall of a large bulge misses with pad 0 |
| P1 | `a-planar-face-sums-its-area-about-a-far-carrier-origin` | M | A planar face's closed form sums its vector area about the carrier plane's stored origin, so a small face far from that origin loses its area to cancellation |
| P1 | `a-sphere-faces-flux-cancels-r-times-its-area-against-its-vector-area` | M | A sphere face's closed-form flux cancels R·Area against c·A⃗ by h/R, and its area's level difference cancels near a pole, so a thin cap's volume reads wrong and its certified enclosure straddles zero |
| P2 | `a-kept-rim-off-its-sphere-is-read-on-its-own-sphere-so-member-orders-diverge` | M | The fan form reads a kept rim that stands off its sphere on the circle's own sphere, so the tube-dome union's member orders answer differently at eps 1e-12 |
| P3 | `a-cylinder-rims-level-is-recovered-by-a-dot-product-that-loses-a-short-faces-height` | E | A cylinder face's rim levels are recovered as (centre − origin)·axis, which on a tilted frame far from its origin costs a short face up to 1.8e-8 of its area |
| P3 | `cylinder-and-torus-edge-misses-are-read-back-about-the-axis` | E | The cylinder's and torus's per-edge whole-turn misses are read back about the axis, so their Interval width is charged as disagreement on a far generic-pose body |
| P3 | `planar-face-reads-arc-spans-undecided` | E | props: planar_face integrates every circle and ellipse arc's stored span with no span decide, so a hand-built arc past a period counts the extra turn |
| P3 | `props-rim-incidence-decides-axis-and-centre-one-at-a-time` | E | props' require_rim_incidence decides the rim's axis and centre one at a time, not their sum |
| P3 | `props-sphere-side-in-arc-is-levered-by-arc-length-not-the-roots-slope` | M | props: the sphere side's in-arc reading is levered by arc length, so near a graze a root within its own error of a span end is read decided |
| P3 | `sphere-rim-level-cosine-cancels-near-a-pole` | M | A sphere rim level's cosine is recovered as √(1 − s²) and cancels near a pole |
| P4 | `props-reads-vector-norm-bounds-off-per-coordinate-hulls` | M | props: the area pads and the curve second-derivative bound read a vector norm off per-coordinate hulls, so they depend on the frame |

## Order

The four P1 rows are one fix: each arm sums its vector area about a point that is far from the face. They are `a-planar-face-sums-its-area-about-a-far-carrier-origin`, `a-cylinder-wall-reads-its-vector-area-at-the-axis-origins-radial-lever`, `a-cone-wall-far-from-its-apex-measures-its-rounding-at-the-apex-lever` and `a-sphere-faces-flux-cancels-r-times-its-area-against-its-vector-area`. Decide the reference point once, measure what each arm loses today, and land it arm by arm or together. The rim-level rows (`a-cylinder-rims-level-…`, `sphere-rim-level-cosine-cancels-near-a-pole`) and `cylinder-and-torus-edge-misses-are-read-back-about-the-axis` are the same class one level down, and come next. The sphere side and incidence rows, the vector-norm hulls and `planar-face-reads-arc-spans-undecided` batch by file.

The cone-apex row edits `cone_face_closed_form`, which CONEARM's region unit edits too. Whichever program reaches that function second brings the other's change in first.

## Review posture

FLUX's, inherited: the review tiers of `memories/orchestration-model.md`
(orchestrator's read, single style or full review, or a dual under
`docs/DUAL-REVIEW-PROTOCOL.md`, logged in `docs/DUAL-REVIEW-LOG.md`),
named with its reason at dispatch. Expect a dual on a row that changes
what the kernel will certify over a family: a wrong closed form is the
error that reads as an improvement.

What PROPS and FLUX learned, stated so that briefs carry it:

- **A spec that asserts a premise hands the unit a defect.** One
  premise about `du_of_rims` failed three times running in PROPS, the
  last time on main. A spec here requires a decide.
- **A re-baselined golden is not a re-pointed row.** Check what a row
  witnesses before moving its constants: its subject can move under a
  change while its numbers stay movable.
- **A narrower bound is the expected outcome of a width fix, and
  therefore not evidence that the fix is correct.** Containment is.
  Test against exact arithmetic, with cases chosen to break it.

## Exit

The slate is closed or re-homed. This plan sets no `## Exit criteria`,
so the program closes without an exit walk.
