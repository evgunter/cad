---
id: a-conic-on-a-cone-reads-no-bounded-margin
kind: issue
title: A conic lying in a cone face reads on it by its samples with no bound between them, so the parting refuses its arc CarrierUnread
status: open
opened: 2026-11-25
priority: P3
cost: M
---


Filed by `heldrecords/parting-division` from PR 7783's review (m1).

## What

`boolean/slit.rs` `conic_or_line_on` reads a circle or an ellipse on
a sphere or a cylinder at five samples. The surface's residual
(`geom_brep::implicit_residual`, a quadratic over `2r`) is then a
trigonometric polynomial of degree 2 along the conic, so its samples
bound it everywhere (`quadric_between`, Lebesgue constant `1.99`), and
the arc's `ArcOnFace` row cites that bound. On a cone the residual is
`ρ cos α − h sin α`, with `ρ` a square root: no polynomial along the
conic, so the samples bound nothing between them. The conic reads on
the cone (`Placed::On(None)`, the degree argument), and a parting of
it refuses `EdgeInFace { gap: CarrierUnread }` rather than cite a
margin it does not have.

## Done when

A bound on a conic's distance from a cone over its span, for example
the squared form `ρ² cos² α − h² sin² α` bounded by its samples and
divided by a lower bound on `ρ cos α + h sin α` along the conic, and
the parting cites it.
