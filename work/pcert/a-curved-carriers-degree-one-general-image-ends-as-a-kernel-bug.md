---
id: a-curved-carriers-degree-one-general-image-ends-as-a-kernel-bug
kind: issue
title: The mint's degree-1 General image of a curved carrier on a spline chart refuses its map residual with a tolerance offer and a kernel-bug ending
status: open
opened: 2026-11-03
priority: P2
cost: M
---

Filed by PCERT unit E5 (PR 6484, review T2).

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
