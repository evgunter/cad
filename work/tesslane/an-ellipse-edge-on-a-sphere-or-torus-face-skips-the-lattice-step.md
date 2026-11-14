---
id: an-ellipse-edge-on-a-sphere-or-torus-face-skips-the-lattice-step
kind: issue
title: The chord pass steps an ellipse edge by its curvature alone, never the sphere or torus lattice's boundary step, though an in-band ellipse now certifies on both charts
status: open
opened: 2026-11-04
priority: P3
cost: E
---


Filed by PCERT (`pcert/ellipse-general-lane`, the ellipse's general
lane).

## What

`mesh::chords`' edge schedule steps a circle edge by its sagitta and
then tightens it to every adjacent face's lattice step
(`chords::tighten`, the sphere's `chords::tighten` among them), so
the trim polygon and the face's lattice are sized from one value. Its
`Curve3::Ellipse` arm takes `ellipse_step` alone, on the stated ground
that "an ellipse never lies on a torus chart of this kernel's
constructions".

An ellipse within the band of a sphere or a torus now certifies on that
chart through the projected image (D3's lane choice), and the sphere
arm's speeds already read its image generically
(`geom_brep::Pcurve::speed_bound`, through `FramedCarrier::Conic`). So a
sphere or torus face bounded by such an ellipse, from an import or a
producer that stores one, is stepped without its lattice's tightening.
No kernel constructor mints one today.

## Fix

Run the adjacent faces' `chords::tighten` on the ellipse arm too (it
reads the face, not the carrier's kind), and drop the comment's
premise.

- 2026-11-14 — moved from TESSSIZE to TESSLANE by `git mv` when TESS and TESSSIZE merged into their successor TESSLANE (`docs/doc-ledger/tess-and-tesssize-leave-the-tracker.md`), id and body unchanged.

## 2026-11-14 — the fix's premise is gone (TESSLANE, `one-chord-rule-over-each-boundary-images-speed-bound`)

The chord pass now tightens every carrier kind by one rule (`chords::tighten`), which reads the face, not the carrier, and the comment premised on an ellipse never lying on a torus chart is gone. What is true now: a sphere or torus face bounded by an ellipse routes to the trimmed lane (`trimmed::has_trim_carrier`), whose sphere arm excludes a face with a trim carrier (`trimmed::takes_sphere_arm`) and whose roster has no torus arm, so such a face lays no lattice the rule reads and refuses at the roster (`trimmed::trim_frontier`) before its boundary's count matters. The row's fix is done as far as the chord pass goes; what remains is the arm, and that is `cone-and-sphere-faces-with-conic-or-spline-trims-have-no-trimmed-arm` and `trimmed-tessellation-lacks-torus-and-plane-arms`.
