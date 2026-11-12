---
id: a-curved-carriers-degree-one-general-image-ends-as-a-kernel-bug
kind: issue
title: The mint's degree-1 General image of a curved carrier on a spline chart refuses its map residual with a tolerance offer and a kernel-bug ending
status: closed
closed: 2026-11-04
opened: 2026-11-03
priority: P1
cost: M
---

Filed by PCERT unit E5 (PR 6484, review T2). It is the same class as
E4's fix U2 (`pcert/circle-general-lane`): a carrier that takes its lane
must never end in a tolerance offer. E4's lane filed
`an-ellipses-general-lane-is-written-on-no-chart` (the analytic charts'
projected lane), not this spline-chart producer.

## What

`topo::pcurves` `nurbs_iso_derive`'s fall-through derives a curve no
exact class holds as a `General` image through `derive_general_image`
(→ `geom_brep::FittedLane::general_image`), at `PXN_IMAGE_DEGREE = 1`
(`crates/geom-brep/src/edge_nurbs.rs` near :3061). For a curved carrier
the degree-1 image bends off the foot path between its 33 feet. The
mint's certification then refuses at the map residual, as a defect:

    pcurve certification: MapResidual at sample 1 definitely exceeds the
    tolerance band … Recourse: loosen the tolerance, as a last resort;
    this refusal may indicate a kernel bug worth reporting

Measured by E5's review probe (`review/6484-r1`, `20db438c8`). A circle
on a rational ring wall, its row bent `1e-3` past its second sub-arc's
probe, fell to `General` before E5 and refused
`ResidualExceeded { MapResidual }`. The circle lies on the wall wherever
the bend does not move it. The failure is the image's own lane gap, a
degree-1 image of a curved path, and is not the carrier's or the
kernel's.

E5 took circles off this path: a circle no row holds now refuses
`NoGeneralLane` "not yet" in every description form. An **ellipse**
(and any other curved non-spline carrier) on a spline chart still takes
it. The general row type itself certifies such carriers when its image
is fine enough (`crates/geom-brep/tests/pcurve_general.rs`
`a_circle_and_an_ellipse_certify_on_a_spline_chart`, a degree-5 image
through 257 feet), so the gap is the mint's producer, not the row.

## Fix

Either the mint's producer images a curved carrier finely enough for
the row to certify it (raise the degree, or refine where the hull
refuses), after which circles can take `General` too and E5's
`NoGeneralLane` for them goes; or a curved carrier's `General` refusal
ends "not yet" with the image as its lever (D3), never a tolerance
offer or a kernel-bug ending. A row needs an ellipse on a spline wall
that the mint images.

## Closed

`pcert/curved-general-image`. The mint's producer images a bending
carrier (curved, no net: a circle, an ellipse) finely enough for the
`General` row to certify it: `edge_nurbs::curved_chart_image`, degree 5
through 33, 65, 129 or 257 feet of the uniform schedule, stopping at the
first rung whose miss sampled between feet is within `band.zero()/16`,
and broken at full multiplicity where the foot path crosses one of the
wall's interior knot lines. A line's and a spline's image is today's
degree-1 one. Where the certificate still refuses a fresh image of a
bending carrier (a band-scale pose, the finest rung, `Interval` at ε
1e-12), `geom_brep::minted_refusal` relabels it in
`general_lane_refusal` as the lane's gap, "not yet"; a stored row at
rest keeps its term. Circles take the general lane too: the arc rim's
fall-through is the same `general` closure `iso_schedule` ends in, and
E5's `NoGeneralLane` for a circle no row holds is gone.

Rows: `topo/tests/a_bending_carrier_on_a_spline_wall_takes_the_general_lane.rs`
and `geom-brep/tests/pcurve_general.rs`
`the_derived_route_certifies_a_line_and_a_bending_carrier_*`.

Filed:
- `work/pcert/an-ellipse-stated-as-another-charts-image-on-a-spline-chart-takes-no-general-lane.md`;
- `work/pcert/a-curved-spline-carrier-or-a-non-affine-line-ends-its-general-image-as-a-kernel-bug.md` (P1);
- `work/pcert/two-homes-cut-a-chart-path-at-the-walls-knot-lines.md` (P3);
- `work/quad/a-rational-spline-face-with-a-general-row-has-no-volume-lane.md`;
- evidence on `work/ssiedge/plane-nurbs-limb-two-refuses-a-non-row-section.md`
  (the edge certificates' producers), `work/isotail/curved-trim-e2e-fixture-waits-for-a-producer.md`
  and `work/nurbs/nurbs-curve-interval-eval-loses-the-partition-of-unity-over-a-wide-t.md`;
- evidence on `work/pcert/a-minted-refusals-provenance-is-guarded-only-by-its-doc.md`
  (a stated image now keeps its term, `ChartImage::Stated`; the row stays open).
