---
id: circle-crossings-take-a-callers-radius-undecided
kind: issue
title: trace::circle_crossings and circle_crossings_to hand a caller's radius straight to Circle::new
status: open
opened: 2026-11-23
priority: P2
cost: E
---


Filed by GERM's `germ/conic-constructions-audit` lane (2026-11-23), from
`work/germ/conic-constructions-from-computed-sizes-unaudited.md`.

## Measured

`topo::boolean::trace::circle_crossings` and `circle_crossings_to` are
public, take the circle as a tuple `(center, axis, radius, u_ref)`, and
build `geom::Circle::new(frame, radius)` as their second statement.
`Circle::new` panics on a radius that is negative or not finite, and its
docs say a door taking a caller's radius decides its sign before it
builds one. Neither door decides anything about it, so a radius of
`−1.0`, NaN or `inf` panics before any `TraceFault`.

The one production caller is `editor-core`'s `trace::arc_to`, which
passes `(pen − centre).direction_and_length().1`: a length, so never
negative. Whether a sketch expression can hand it a pen or a centre
that is not finite was not established; if one can, that is a panic on
a document.

## What a fix has to supply

The radius decided before the circle is built, and a `TraceFault` for a
radius that is not a positive finite length.
