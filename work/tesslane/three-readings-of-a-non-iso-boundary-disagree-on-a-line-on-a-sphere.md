---
id: three-readings-of-a-non-iso-boundary-disagree-on-a-line-on-a-sphere
kind: issue
title: Three readings of a non-iso boundary (topo props' trim routing, props' IsoChart::reads, Pcurve::is_non_iso_form) disagree on a line on a sphere
status: open
opened: 2026-11-17
priority: P3
cost: E
---

## What

Three readings of "a boundary carrier no iso line of the face's chart
takes" exist, and they already disagree on a line on a sphere:

- `topo` props' trim routing: `trims` (`crates/topo/src/props.rs:4998`)
  calls a `Line` or a `Circle` iso on any curved face, so a line on a
  sphere keeps the face on its closed form; `curved_rim`
  (`crates/topo/src/props.rs:4984`) reads the plane's arm by kind alone.
- props' door: `IsoChart::reads` (`crates/geom-brep/src/props/curved.rs:524`)
  reads a `Line` only on a cylinder or a cone, so a line on a sphere is no
  rim or meridian, and since PR 7041's second fix pass is decided off the
  sphere (`OffSurface`) unless it is within band of it.
- the stored image's form: `Pcurve::is_non_iso_form`
  (`crates/geom-brep/src/pcurve_cache.rs:1161`), which the curved-face
  route reads first (`crates/mesh/src/tessellate.rs:310`), reads the form
  the mint stored, not the carrier's kind.

No body this build makes puts a line on a sphere, so none of the three
answers it today; whichever is asked first decides where such a face
goes. Found by PR 7041's second review.

## The shape of a fix

One home of the kind table (`IsoChart` is the candidate: it is already
the one home inside props), read by `topo`'s trim routing, with the image
form's reading stated as what the mint stored when the kinds agree.
