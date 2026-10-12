---
id: the-face-whole-excusal-has-no-fixture-on-an-analytic-chart
kind: issue
title: tier 3's face-whole excusal has no fixture: no uncovered class is reachable through a public strut on an analytic chart
status: open
opened: 2026-10-07
---

Filed by `pcert/torus-villarceau-route`. `topo::pcurves::not_owed`
excuses a face only when every refusal its edges meet is not owed, and
`crates/sweep/tests/at_rest_pcurve_faces.rs` pinned that a face is read
whole before it is excused (`an_uncovered_strut_masks_no_off_chart_strut_in_any_cycle_order`,
and the uncovered half of `an_uncovered_strut_masks_no_refused_certificate`),
with an oblique circle strut on a torus wall as the uncovered class
(`UncoveredClass::TorusGeneralCircle`).

That class is gone: the torus's incidence test reads such a circle off
the torus (`CarrierOffChart`), and a circle on it is a Villarceau circle
(imaged) or grazes it (`CarrierGrazesChart`). The rows were cut to what
still holds; the masking property has no fixture now. The classes
`not_owed` still excuses are not reachable through `Body::mev` at `f64`
on an analytic chart:

- `SplineCarrier`: a spline strut refuses at edge certification first —
  `CertifyError::Unimplemented` under a scaffold or chart description
  (`geom_brep::certify`, the `Curve3::Nurbs` gate before the resolver),
  and an `Intersection` description takes the fitted lane, which `f64`
  holds.
- `NoFittedClass`, `MirrorTorusSpiric`: a strut's own door cannot state
  them, and neither can the `mvfs` → `set_face_surface` → `mef` route
  that builds a whole Villarceau loop
  (`crates/topo/tests/a_whole_villarceau_circle_bounds_a_torus_face.rs`).
  Tried on PR 4227 with a whole spiric oval on its mirror torus:
  described in the torus's chart, the edge does not certify (a curved
  chart's description is its chart image, which is what an uncovered
  class lacks, `CertifyError::ChartImageUnavailable`); described on its
  cutting plane, `mef` mints the loop, but neither a new face on the
  mirror torus (`mef` with `FaceSurface::New`) nor a move onto it
  (`set_face_surface`, `set_face_surfaces_describing`) is admitted,
  since no certified edge names that chart (`RechartUnvouched`). Every
  public door keeps a curved face's edges described in its own chart.
- `FittedLaneUnsupported`: only at a scalar without the fitted door.

A fixture needs either a producer path to one of these (the issue
`uncovered-chart-classes-have-no-incidence-test` names a STEP re-import
of a spiric rim landing a spline on a torus) or a test-only door.


Correction (spline-carrier designers, PR 4261): an `Intersection` spline strut on an analytic face does not take the fitted lane. `analytic_derive` excuses it (`UncoveredClass::SplineCarrier`). `crates/topo/tests/m6_2_fitted_at_rest.rs` and `topo/tests/fixture/mod.rs` build exactly that face through public `mev`. So until the spline route lands, a fixture for the excusal is that face without the hand-attached row (`attach_pcurve`).

## Moved by the projected image (branch `pcert/projected-image`, 2026-10-08)

The correction above no longer holds. `UncoveredClass::SplineCarrier`
is deleted: the m6_2 face mints its projected rows through public `mev`
plus the closing mint, so it is not a fixture for the excusal.
`not_owed` still excuses `NoFittedClass`, `MirrorTorusSpiric` and
`FittedLaneUnsupported`. Only the last is reachable on an analytic
chart: the m6_2 face at the `Dual` scalar, which holds no fitted door,
leaves the face rowless (`the_dual_leaves_the_face_rowless_and_says_why`).
A face-whole masking fixture could stand on that: a dual body with a
spline strut beside an off-chart one. It is not built here.
