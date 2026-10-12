---
id: an-in-band-certification-of-a-minted-edge-other-than-its-face-residual-reads-as-a-defect
kind: issue
title: An in-band certification of a minted edge other than its face residual reads as a kernel defect (describe pass) or as the fit's last resort (section describer)
status: closed
opened: 2026-10-11
priority: P2
cost: M
branch: cleave/minted-edge-inband-cert
closed: 2026-10-12
pr: 4773
---


## What

Found by the sweep of `cleave/folded-graze-join-desync`. That branch
types a minted edge's exact carrier read in band of one of its faces
(`CertCheck::SurfaceResidual` escalated) as
`Escalated { BooleanDecision::EdgeOnFaces }` at `boolean/ops.rs`
`of_attachment`, the one in-band certification the near-tangent census
measured. (`WitnessSurfaceResidual` stays a defect: the witness is an
exact construction, as certify's ending table types it; a fitted
carrier's residual takes the table's own last resort.) Two neighbours
of it are unmeasured:

- **The describe pass's other in-band checks.** `describe_edges` sends
  every other `CertifyError::Escalated` (`Transversality`,
  `TransversalityArm`, `TangentPlanes`, `TangentSecondOrder`,
  `TangentParallel`, `TangentHull`, `TangentTube`, the plane × NURBS
  and rung-3 rows) to `BooleanError::EdgeUndescribed`, which ends in
  the kernel-defect ending. Each is a `Value` margin in band, which D9
  row 1 calls the input's. They are not mapped onto the seam's own
  decisions (`SeamWedge`, `LeverArm(Seam)`, `SeamJet`) because the
  pass sets differ: the describe pass's seam reading passes a decided
  smooth seam, while the certification of an `Intersection` refuses
  one (`NotTransverse`), so the seam's tolerance offer would not hold
  there.
- **The splitting section describer** (`splitting/finish.rs`, the
  `classify_dihedral` … `set_edge_curve(…)?` arm): its
  `EulerOpError::Certification` reaches the user through the
  certification table at a build, where a `SurfaceResidual` in band
  ends in the fit's last resort ("loosen the tolerance … may be a
  kernel bug"). On a chord between vertices the split placed, that is
  the story `EdgeOnFaces` corrects at the Boolean.

## Owed

Measure whether either is reachable (a near-tangent census over the
split door and over seam poses at `|d| ≈ ε`), and type what is with
its own decision and lever.

## Built (branch cleave/minted-edge-inband-cert)

Measured on main `f813abf9`, release, at ε = 1e-9, 1e-6 and 1e-12, with
the attachment refusals of both describers logged.

- **(a) The describe pass's other checks: unreached.** The Boolean census
  (`near_tangent_census_probe`, every prism, op and order, on its own
  tilt grid and on a grid putting the far vertex within a few roundings
  of `|r| = ε`) reaches the describe pass's certification only on
  `SurfaceResidual`: the 12 `EdgeOnFaces` runs at ε = 1e-9, none at
  1e-6 or 1e-12. The curved seam census
  (`seam_near_tangent_census_probe`: a slot whose box sides meet a
  cylinder smoothly, a box face along a ruling, tilted 1e-3 and 1e-6 off
  it, two cylinders touching outside and inside, a ball on a box,
  `δ` from 0.5ε to 1000ε either side, 912 runs a row) reaches it on no
  check at any ε: every in-band pose refuses upstream (`Coincidence` on
  `EdgeOnPlane`, `EdgeOnCurvedFace`, `VertexOnCurvedFace`, `SectorSide`,
  `EdgeOnEdge`; `Containment`; `WallRoots`) or as unsupported, and every
  pose that reaches the pass describes. For a plane pair it cannot: the
  pass reads the seam with `classify_dihedral` at the witness over the
  chord, which is `wedge_decided`, the function check 4 reads at each
  interior sample over `edge_extent`, the chord for a line; a plane's
  gradient does not depend on the point, so a transverse reading cannot
  escalate at a sample. No witness exists, so nothing is pinned and
  `describe_edges` is unchanged.
- **(b) The split's describer: reached at every ε, at three sites.**
  The split census (`split_near_tangent_census_probe`, the census poses
  as splits of the prism by the cube's near face) refuses at the finish,
  all on an edge whose ends the split read on the plane with the far
  vertex at the zero band's edge, its carrier read one rounding past it:
  - the describer's transverse arm, `Certification { Escalated
    { SurfaceResidual } }`, the fit's last resort: 33 / 16 / 5 runs at
    1e-9 / 1e-6 / 1e-12 (margins 1.00000005ε to 1.0001ε);
  - its smooth arm, the image restated in the section chart,
    `Certification { Escalated { ChartResidual } }`, the kernel-defect
    ending: 36 / 25 / 80;
  - the promotion's re-chart onto the section planes,
    `RechartFalsifies { SetFaceSurfacesDescribing, Escalated
    { ChartResidual } }`, the kernel-defect ending: 54 / 26 / 89.

  Each is the input's: the vertex the split read on the plane lies at
  the zero band's edge, and the carrier one rounding past it (PR 4732:
  the certificate bounds the carrier, and the rounding is K-headroom
  noise, not an amplified error). Every one of these carriers is a line
  chord. A circle or ellipse is not covered by that argument: the
  review's dense census of planes nearly along a cylinder's axis
  measured the split's elongated ellipses carrying amplified rounding in
  band (774 runs at ε = 1e-12), which the join refuses before the
  finish today.
- **Typed, with one rule for both doors.**
  `merge_faces::edge_on_faces_reading` holds the rule: an in-band
  `SurfaceResidual` or `ChartResidual` on a line carrier is the edge's
  own reading. Everything else stays the gate's refusal: a circle,
  ellipse, spiric or spline, a definite miss, the witness's residual, an
  endpoint pin, and any other check.
  - The split's `of_attachment` reads it at the describer's two arms and
    at both section re-charts (promotion and nesting,
    `rechart_section`), into `SplitFinishError::EdgeOnFacesEscalated`.
  - The Boolean's `of_attachment` reads it into `EdgeOnFaces`. This
    narrows PR 4625's rule from every non-spline carrier to lines; its
    12 witnesses are lines and do not move, and its ChartResidual arm
    (the stale-line restatement) gains the same reading.
  - The question ("whether … lies on both faces it bounds") and the
    lever are spelt once (`edge_on_faces_subject!`,
    `edge_on_faces_lever!`). The lever adds the angle an edge ending on
    the face needs: "… lies clearly on it or clearly off it, and an edge
    that ends on it leaves it at a clear angle". At the split it composes
    `geom_core::split_plane_recourse!()`.
  - The nesting re-chart has no witness. The hole's edges are restated
    over the same plane at promotion first, so a pose would have to
    differ between the two charts by roundings; the site says so. After the change
  all 123 / 67 / 174 census refusals at the finish type this way, and
  no other finish refusal is left.
- **The lever tells the truth.** For each such run of the census's own
  tilts (81 / 46 / 105), the far vertex moved onto the plane (`d = 0`)
  builds, and the edge tilted 1e-2 rad off on either side builds. Moving
  the far vertex only `3Kε` off leaves about half refusing
  `SliverSector`: the edge still meets the plane at the corner, so it
  needs a clear angle there, and a milliradian is still not one at
  ε = 1e-6. The lever says so.
- **Replay.** The Boolean census's band-edge grid is
  `NT_EDGE=1 NT_FULL=1 near_tangent_census_probe` (a describe-pass
  refusal reads `EdgeUndescribed` or `EdgeOnFaces`). The split census is
  `split_near_tangent_census_probe`, and the seam census is
  `seam_near_tangent_census_probe`.
- **Pins.** `sweep` `split_edge_on_faces_poses`: 7 poses per ε row
  across the three sites and eight prisms, each refusing typed with the
  lever and building at `d = 0` and `d = ±1e-2`, red on main. `topo`
  `splitting::finish::edge_on_faces_rows`: the mapping at both doors,
  and what stays the gate's.
- **Filed.** The join's endpoint pin reads the same story as a kernel
  defect at both doors:
  `a-section-curves-end-read-in-band-of-its-vertex-refuses-as-a-kernel-defect-at-the-join`.
  Its definite neighbour, an elongated ellipse at ε = 1e-12, is added to
  GERM's `conic-section-just-past-its-band-fails-certification-downstream`.

## Closed (PR 4773, 2026-10-12)

- **(a) The describe pass's other in-band checks.** Measured unreached by the planar census and a curved seam census (912 runs per ε), and structurally unreachable for a plane pair.
- **(b) The split's section describer.** Reached at three sites. Those are now typed `EdgeOnFacesEscalated` with a lever that asks for angular clearance.
- **The rule.** An in-band `SurfaceResidual`/`ChartResidual` reads as the input's only on a Line chord. It has one home, `merge_faces::edge_on_faces_reading`, which both the split and the Boolean read. Conics keep the gate's refusal, because amplified ellipse rounding sits in band at 1e-12.
