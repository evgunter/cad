---
id: rim-arms-holds-a-level-arm-a-cone-never-reads
kind: issue
title: props' RimArms holds a level arm for a cone, which reads none: the cone is handed in to fill it with poison
status: open
opened: 2026-11-23
priority: P3
cost: C
---


Found by `oneround/lever-sites-brep` (PR 7219), whose review raised it as
a style finding and whose delta review asked that the deferral have a
row.

## What

`geom_brep::props::curved`'s `RimArms<T>` holds two fields for every
kind: `level`, the lever a `RimLevel::Unit` difference turns about, and
`azimuth`. A cone's rim levels are `RimLevel::Length` (a slant length,
already the point deviation), so its `level` is never consumed; the
field's own doc says so ("Never consumed for a `RimLevel::Length`
kind").

The field still has to hold something. `cone_boundary` therefore takes
the cone surface as an argument for no reading of it but
`crate::uniform_curvature_arm(surface)`, which is poison for a cone, and
passes that to `RimArms::uniform`. Poison is the right value for an arm
nothing reads (a reader that appears escalates), and before PR 7219 the
site handed `curvature_radius(|v|·sin α)`, a number vouched for, which
is what that PR removed. What is left is the shape: a parameter and a
field that exist to carry a value no code reads.

## What would close it

`RimArms` distinguishes the kinds whose levels are lengths: for instance
an enum with a variant that holds the azimuthal rate alone, or `level:
Option<Lever<T>>` read by the `Unit` arm only, so that a `Length` kind
constructs no level arm and `cone_boundary` stops taking the surface.
The cylinder's levels are lengths too (`RimLevel::Length`), so the same
variant serves it. Changing what the struct holds was outside PR 7219's
fence.
