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
(imaged) or, since PCERT E1, a lane choice with no general lane on the
torus, which refuses lever-only "not yet" (`NoGeneralLane` or
`Escalated { ChartClass }`, unit E4). Before E1 it refused
`CarrierGrazesChart`, which is deleted. The rows were cut to what
still holds; the masking property has no fixture now. The classes
`not_owed` still excuses are not reachable through `Body::mev` at `f64`
on an analytic chart:

- `SplineCarrier`: a spline strut refuses at edge certification first —
  `CertifyError::Unimplemented` under a scaffold or chart description
  (`geom_brep::certify`, the `Curve3::Nurbs` gate before the resolver),
  and an `Intersection` description takes the fitted lane, which `f64`
  holds.
- `MirrorTorusSpiric` (and `NoFittedClass`, deleted on
  `pcert/fitted-row-reads-carrier-and-chart`): a strut's own door
  cannot state them, and neither can the `mvfs` → `set_face_surface` → `mef` route
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
`not_owed` still excuses `MirrorTorusSpiric` and
`FittedLaneUnsupported` (`NoFittedClass` has since been deleted). Only the last is reachable on an analytic
chart: the m6_2 face at the `Dual` scalar, which holds no fitted door,
leaves the face rowless (`the_dual_leaves_the_face_rowless_and_says_why`).
A face-whole masking fixture could stand on that: a dual body with a
spline strut beside an off-chart one. It is not built here.

## The dual's m6_2 face is gone too (PR 4791, 2026-10-12)

A lane-free door now refuses an analytic rung-3 edge
(`CertifyError::NurbsLaneNotSupplied`), so the dual's mev refuses the
m6_2 fixture's edge naming the scalar
(`the_dual_holds_neither_lane_and_each_refusal_names_its_scalar`), and
no face is left rowless. With `not_owed`'s `FittedLaneUnsupported` arm
instrumented, the topo, sweep, editor-core, step-import and mesh `ci`
suites reached it at no scalar. The arm stays: it is reachable by
construction. An iso-curve spline edge on a spline chart certifies with
no lane at the dual, and an analytic face beside it is owed a fitted or
projected row for that carrier. It has no fixture.

## `MirrorTorusSpiric` is gone too (`pcert/mint-routes-spiric-nofitted`, 2026-10-12)

The class now has its route: a mirror wall image. The whole oval that
PR 4227 tried is now a fixture for a MINTED face
(`crates/topo/tests/a_whole_spiric_oval_bounds_its_mirror_torus.rs`).
`not_owed`'s first arm excuses `NoFittedClass` alone, and that class is
reachable only on a spline chart (`stated_general_image_mint`), never on
an analytic one.

## The class arm is gone (`pcert/fitted-row-reads-carrier-and-chart`, PR 4900)

`NoFittedClass`, the last uncovered class, is deleted, and with it
`UncoveredClass`, `PcurveCertifyError::UnsupportedCarrier` and
`not_owed`'s class arm. `not_owed` keeps one arm: a scalar with no
fitted door (`FittedLaneUnsupported`). It has a fixture on a spline
chart now: `stated_general_image_mint::at_dual64` reaches it, and those
two rows were the only excusals a census of the topo, sweep,
step-import, mesh and editor-core `ci` suites found. It still has no
fixture on an analytic chart, which is what this item asks for.
