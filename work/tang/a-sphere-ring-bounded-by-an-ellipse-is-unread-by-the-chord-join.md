---
id: a-sphere-ring-bounded-by-an-ellipse-is-unread-by-the-chord-join
kind: issue
title: The chord join reads a sphere face's loops as circles only, so a ring bounded by an in-band ellipse, which now certifies on the sphere, ends RingIslandUnread
status: open
opened: 2026-11-04
priority: P3
cost: E
---


Filed by PCERT (`pcert/ellipse-general-lane`, the ellipse's general
lane).

## What

An ellipse within the band of a sphere now certifies on the sphere's
chart through the projected image (D3's lane choice; before, it refused
"not yet" at the mint, so no such edge reached a body). The chord
join's loop read, `topo::chord_join::quadric_loop_arc`, still reads a
sphere face's loops as circles only: `(Quadric::Sphere { .. },
Curve3::Circle(..))` takes `LoopArc::of`, and every other carrier falls
to `_ => None`, which `run_loop_arcs` ends as
`SplitJoinError::RingIslandUnread`. Its doc names "on a sphere a line,
an ellipse or a spiric" as carriers the sphere's loops do not carry.

So a ring on a sphere face bounded by such an ellipse, from an import
or a producer that stores one, ends `RingIslandUnread`, typed, where a
circle's ring is read. Not reached by any constructor today: no kernel
producer mints an in-band ellipse on a sphere.

## Fix

Read an ellipse on the sphere as the cylinder and cone arms do
(`LoopArc::of`), if the ring lane's island winding holds for a
non-circular loop on that chart; otherwise keep the typed refusal and
reword the doc to say why.
