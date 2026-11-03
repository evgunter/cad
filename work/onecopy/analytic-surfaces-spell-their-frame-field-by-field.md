---
id: analytic-surfaces-spell-their-frame-field-by-field
kind: issue
title: Plane, Cylinder, Cone (and sphere, torus) each spell the placement (point, axis, u_ref) field by field, beside the geom::Frame the conics hold
status: open
opened: 2026-11-02
priority: P1
cost: H
---

Found by the ONEHOME orchestrator while adjudicating PR 6328's style review (2026-11-02).

The conic unit (`circle-and-ellipse-frame-arms-re-spell-conic-of`, branch `onehome/conic-frame`) introduces `geom::Frame`: a rigid placement, i.e. a point and an oriented orthonormal basis (`axis`, `u_ref`, `v_ref = axis × u_ref`), held by `Curve3::Circle` and `Curve3::Ellipse`. The analytic surfaces carry the same triple under their own names: `Surface::Plane { origin, normal, u_ref }`, `Cylinder { origin, axis, u_ref }`, `Cone { apex, axis, u_ref }`, and likely the sphere and torus. Under DESIGN `[one-way]` ("Two types carrying the same fields are a sign of a cleaner arrangement to find…", Ev, PR 6325), each should hold one `Frame`. Readers of the placement alone (azimuth, the seam, transforms, the axis line) would then read it there rather than per kind.

Parked until the conic unit lands and fixes `Frame`'s shape. Then sweep every surface kind and every reader that assembles the triple, including the surfaces' axis-line readings that PR 6328's review found built by hand as `geom::Line { origin, dir: axis }`.

## A further holder (PR 6349 review, 2026-11-02)

`geom::chart_point::ChartInverse { anchor, axis, u_ref, v_ref, arm }` holds the same placement triple in the same crate, plus a stored `v_ref` that `Frame::v_ref` derives. It belongs in this row's sweep: it holds a `geom::Frame`, or its readers read the surface's.

## Slate triage (2026-11-03, ONEHOME cut)

- Moved to ONECOPY (`work/onecopy/`) by `git mv`, id and body unchanged. ONEHOME measured 70 budget points against 35 when its orchestrator ran out of usage; the rows it had not started (no lane, no design fork, no in-flight PR riding them) were cut onto ONECOPY, and what is in flight or staged stayed on ONEHOME.
