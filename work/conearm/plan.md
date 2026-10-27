# CONEARM — the plan

Opened 2026-10-27 by the re-cut of FLUX and FLUXTAIL. Together they
held 95.5 budget points against 30, and both shared
`props/curved.rs` and `props/spiric.rs`, so their orchestrators
could not run side by side. The rows were re-cut by logical thread
instead of by priority, so that the five successors can run in
parallel: each mostly owns its own files, and where two share a file
they work on different functions of it
(`docs/doc-ledger/flux-leaves-the-tracker.md`). Nothing dispatched.

## The slate

**13 budget points** of dispatchable work against a ceiling of 30, in 6 rows (2026-10-27).

| pri | item | cost | title |
|---|---|---|---|
| P2 | `a-cone-face-whose-rings-leave-or-overlap-its-region-measures-silently` | H | A cone face whose rings leave, straddle or overlap its region, or whose loop crosses itself between vertices, measures a wrong area silently |
| P2 | `cone-holes-outside-their-face-are-subtracted-silently` | M | A cone face's hole outside the loops that bound it is subtracted silently, not refused |
| P3 | `frame-read-off-ellipse-fields-in-curved-props` | E | props/curved.rs spells an ellipse's semi-axis frame from Curve3::Ellipse's fields instead of Conic::semi_axis_vectors |
| P3 | `props-cone-ellipse-elevation-over-reads-a-section-ellipse-by-twenty-fold` | M | props_cone_ellipse_on_cone bounds a plane × cone section ellipse's elevation 9–21× above its measured miss, so a half of a scale-100 frustum split refuses tier 3 at ε = 1e-12 |
| P3 | `props-reads-a-cone-points-slant-by-its-own-spelling` | E | props reads a cone point's chart v (its slant) by its own spelling, beside geom's chart reading |
| P4 | `cone-apex-loop-beside-a-winding-rim-refuses-a-region-it-bounds` | E | A cone face with a loop through the apex and a rim that winds the axis refuses, though it bounds one region |

## Order

`a-cone-face-whose-rings-leave-or-overlap-its-region-measures-silently` and `cone-holes-outside-their-face-are-subtracted-silently` are one question: what region a cone face's loops bound. They take one unit, with `cone-apex-loop-beside-a-winding-rim-refuses-a-region-it-bounds` as its third case. The section-ellipse rows (`props-cone-ellipse-elevation-over-reads-…`, `frame-read-off-ellipse-fields-in-curved-props`) share `require_cone_ellipse` and batch together. `props-reads-a-cone-points-slant-by-its-own-spelling` waits on PCERT's question of what a cone point's v is (`work/pcert/the-projected-image-inverts-chart-points-by-a-second-spelling.md`).

NEARPOINT's `a-cone-wall-far-from-its-apex-measures-its-rounding-at-the-apex-lever` edits `cone_face_closed_form` too. Whichever program reaches that function second brings the other's change in first.

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
