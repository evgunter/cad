---
id: patch-image-iso-still-carries-a-point-and-a-slope
kind: issue
title: PatchImage::Iso(Point2, Vec2) still carries the diagonal-capable form of an iso line, read off the image's structure at one site
status: open
opened: 2026-11-23
priority: P4
cost: E
---


Found by the review of BOXFINISH's iso-image unit (PR 7393, 2026-11-23).

`geom_brep::pcurve_cache::patch::PatchImage::Iso(Point2<T>, Vec2<T>)` is
the form the stated door's composite reads an iso line in: a point and
a chart velocity, which can say a diagonal. Since PR 7393 an
`IsoLine` is `{ line: ChartLine, at0, rate }` and cannot. The one
producer, `PatchClass::of` (`patch.rs:82`), builds the pair from the
structure (`line.point(at0)`, `line.vector(rate)`), and the one
reader, `stated.rs`'s `StatedDoor::Patch` arm, hands it to
`iso_net(p0, pl, domain)`.

So the held axis is stated by the image and then dropped into a form
that no longer states it, between two functions of one module. Nothing
reads the dropped fact today (`iso_net` builds a degree-1 net either
way). The form to carry is the image's own `(ChartLine, at0, rate)`,
with `iso_net` reading `line.point(at0 + rate·t)`; then no value of
`PatchImage` can be a slanted iso line.
