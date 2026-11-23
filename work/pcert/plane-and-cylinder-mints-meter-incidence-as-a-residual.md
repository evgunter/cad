---
id: plane-and-cylinder-mints-meter-incidence-as-a-residual
kind: issue
title: The plane and cylinder mints, the sphere, cone and torus class arms, and the sphere and torus ellipse kind gates end a carrier without reading its incidence first (as ChartResidual's defect, or off by kind)
status: closed
opened: 2026-10-25
closed: 2026-10-28
---


**E1c's item** (`work/pcert/plan.md`). E1b's finding 8, deferred by the
orchestrator's ruling (c), widened by review A on E1d to the class arms of
the sphere, cone and torus mints.

## What happens

A carrier off its face should end as its incidence reading does
(`CarrierOffChart`, or `Escalated { ChartIncidence }`), which E1d ends by
the door that reads it (`IncidenceRead::ending`): a definite miss in the
door's defect (the kernel's at a build, the stored culprit at rest, the
damaged file at import), a read in band in the geometry's lever at every
door. That holds only where the mint runs the incidence test
(`incidence::incident`, `crates/geom-brep/src/pcurve_cache.rs`,
`mod incidence`) before it images the carrier. These mints do not:

- **Plane.** `derive_class`'s `Surface::Plane` arm maps any carrier's
  harmonic coefficients through the affine chart. It reads no incidence.
- **Cylinder.** `derive_class`'s `Surface::Cylinder` arm reads incidence
  only where `pcurve_chart_orientation` is in band (`incidence::general_lane`,
  E1's P1). The meridian arm (`pcurve_chart_radial_moving` Zero, and its
  in-band tie-break) and the decided moving arm (β = ±1) take the carrier
  as given.
- **The sphere, cone and torus CLASS arms** (review A, E1d): the sphere's
  polar circle, the cone's rim and the torus's parallel image a carrier
  their class gates accept (the carrier plane ⊥ the axis, centred on it)
  without reading incidence. The class gates read the plane and the
  centre, not the radius, so a circle of the wrong radius passes them.
- **The sphere and torus ellipse kind gates** (review on PR 5844): an
  `Ellipse` carrier is refused `CarrierOffChart` by kind, with no margin,
  before any incidence read. The kind is no reading of whether the curve
  lies on the face: a tiny ellipse within ε of the sphere (semi-axes
  `a = √(0.2·ε)` and `a/2` in the tangent plane at the pole, set at mid
  sagitta) ends as a definite miss, the kernel's defect, though every
  point of it is on the face within ε. A round-ish ellipse far off the
  face ends right, but only by accident of the kind. (PR 5844 briefly
  read the gate by `semi_u − semi_v`; that was no incidence read either:
  a near-round ellipse 10 m off the sphere then ended in the pose lever.
  It was reverted.)
- **`topo::chart_region`** (~2587) reads a plane chart only, so it is
  this item's plane half's.

The carrier's miss is then read by the certificate:

- by check 3's residual schedule (`check_residual_as` in `run_harmonic_checks`);
- at the edge, by `certify.rs`'s `CertCheck::ChartResidual`, which ends
  `Ending::Residual(Unsized::Defect)`.

So such a carrier ends `ChartResidual`: a definite miss is right only by
accident at a build and at rest, an in-band read ends as a defect, not in
its lever, and at the import door a miss within ε_in is offered setting ε
to ε_in, where the door rule gives the damaged file or the lever.

## Measurement (E1b)

A sketch arc set onto a face it does not lie on, through `set_edge_curve`:
- off a cylinder's wall;
- off a plane side of a box.

Both refuse `ChartResidual` at sample 0 with margin 0.707: the arc's
distance from the face, read as the residual's defect. On a sphere, cone
or torus the same move, off every class, ends `CarrierOffChart`.

## The ruling

Of finding 8's three options, the orchestrator took **(c)**: the mints
read incidence before they image, so an off-face carrier ends as its
incidence read, as on the charts that read it. It is its own unit, E1c,
after E1d2 and before E1e (`plan.md`).

**(a) is rejected**: routing `ChartResidual` itself by what was read.
`ChartResidual` also meters the image's own fidelity, so ending it in a
lever could hide a kernel image bug.

Until E1c lands, D3's incidence clause is not finished on any chart: on
the sphere, cone and torus it holds only on the carriers their class gates
send to the incidence test.

## Overlap with E2

The cylinder half overlaps E2. E2 gives a cylinder line a projected lane,
and it re-reads the cylinder `pcurve_chart_radial_moving` tie-break: the
meridian arm's `Ok(Sign::Zero) | Err(_)` fold, which is where a cylinder
carrier's incidence would be read first. E1c's cylinder incidence test and
E2's tie-break work therefore touch the same arm. Whichever lands second
re-reads it. If E2 is dispatched first, it can absorb the cylinder half of
E1c.

## Done when

- A carrier off a plane or cylinder face, or one a sphere, cone or torus
  class arm accepts while off the face, ends as its incidence read at the
  mint, at both lanes and every ε, never `ChartResidual`.
- The sphere and torus ellipse kind gates read incidence first, as the
  cone's ellipse arm does: an ellipse off the face ends by that read
  (definitely off: the door's defect; in band: the lever), and one not
  shown off is a class reading (a lane choice: "not yet" until its lane is
  written). The `Off` payload quotes the incidence read's margin, never a
  roundness distance as the miss.
- That read ends by the door rule (`IncidenceRead::ending`): a definite
  miss is the kernel's defect at a build, the stored culprit at rest and
  the damaged file at the import door, with no ε_in offer; a read in band
  ends in the geometry's lever at every door, never a defect.
- `ChartResidual` is left metering the image's own fidelity.
- Rows hold the `set_edge_curve` arc off a cylinder wall and off a plane
  side, and a wrong-radius circle on each class arm (sphere polar circle,
  cone rim, torus parallel), each read at a build, at rest and at import.
- Rows hold the review's two ellipse probes (`review/5844-delta`,
  `crates/geom-brep/tests/probe_5844.rs`) in their E1c shape: the tiny
  ellipse within ε of the sphere and of the torus is not refused as off
  (a class reading, "not yet"); the near-round ellipse 10 m off each is
  the definite miss (already pinned in
  `chart_incidence::only_an_incidence_reading_ends_as_one`).

## Landed (E1c, `pcert/mints-read-incidence`)

- `derive_class` opens every analytic arm (plane, cylinder, cone, sphere,
  torus) with the chart's incidence test, and the general lane takes its
  witness (`incidence::general_lane(shown, undecided)`), so no arm images
  or takes the general lane before the read. The spiric's plane cap and
  torus wall read it too (`spiric_chart_pcurve`), so a torus that is
  neither of the oval's two is off it at the mint, not at check 1.
- The sphere and torus kind gates read incidence first: an ellipse or a
  line the test does not show off is the general lane ("not yet"), and the
  Off payload quotes the incidence read (`TORUS_OFF`, the sphere's radius
  why), never a roundness distance.
- `topo::chart_region`'s plane site maps an incidence refusal to
  `ChartRegionError::TrimOffFace`, which the at-rest classifier ends by
  the read (`classify_chart_region`).
- The read is taken in the chart's own frame (`incidence::anchored`):
  the carrier and the chart are moved so the anchor is the origin, and
  the axis is made unit. So the read is the distance from the surface
  the chart denotes, at the carrier's own size. A read where the carrier
  sat read its own rounding far out as distance: an exact rim 1e7 m out
  was off at 1e-12. It levered an axis's length defect by the height: a
  ruling up an axis long by δ, inside the at-rest margin, was off at
  1e-6.
- `ChartResidual` is unchanged and meters the image's own fidelity.
- Rows:
  - `sweep::an_arc_off_a_plane_or_cylinder_face_ends_as_its_incidence_read`:
    `set_edge_curve` arcs off a cylinder wall (meridian and moving arms)
    and off a box's plane sides, at a build, at rest and at import, at
    `f64` and `Interval`.
  - `chart_incidence::a_wrong_radius_circle_on_each_class_arm_ends_as_its_incidence_read`:
    each class arm, plus the plane and cylinder, half a metre off (the
    definite miss) and mid-band off (the lever, never a defect), at every
    door and both lanes.
  - `chart_incidence::a_tiny_ellipse_within_eps_of_the_sphere_or_torus_is_not_yet`:
    the review's tiny ellipse, in its E1c shape.
  - `pcurve_spiric::a_plane_or_torus_off_the_oval_is_read_off_at_the_mint`.
  - `chart_incidence::a_carrier_on_its_face_is_read_on_it_far_out_and_on_a_long_axis`.
- The import door is read by the door rule (`ending(FileCoincidence)`).
  A real STEP file's miss still refuses earlier, at the description
  ladder's meters; that is EXCH's, as E1d recorded.
