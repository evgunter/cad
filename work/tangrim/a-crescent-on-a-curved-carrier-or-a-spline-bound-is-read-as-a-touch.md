---
id: a-crescent-on-a-curved-carrier-or-a-spline-bound-is-read-as-a-touch
kind: issue
title: A tangent crescent between two faces on one curved carrier, or bounded by a spline edge or a bisector, is read as a touch
status: open
opened: 2026-11-02
priority: P2
cost: M
---


Found by the fix pass on PR 6375 (`tangrim/tangent-crescent`), which
reads a tangent crescent as a coplanar overlap
(`a-union-across-a-tangent-crescent-refuses-unbuilt`).

`boolean/sectors.rs` `sector_overlap` reads two coplanar sectors one
order down (`crescent`) only where both faces are planes with opposed
normals and both shared bounds have a bend to read (`bound_bend`).
Apart from aligned plane faces
(`a-crescent-between-aligned-faces-is-read-as-a-touch`), two crescents
are left unread, and each pair stays `touching` as first order reads
it:

- **Two faces on one curved carrier**, lying on each other with
  opposed normals (two cylinder or sphere patches, one of each
  operand, whose rims are tangent at a vertex pair, one curving inside
  the other). `sector_overlap` skips any non-plane face, and
  `crescent`'s `parting` reads the bounds' bends in a plane, not their
  geodesic curvatures in the carrier.
- **A spline edge or a bisector as a shared bound.** `bound_bend` reads
  no bend for `Reach::Extent` (a fitted edge) or `Reach::Bisector`, nor
  for a conic whose bend is `Unbounded` (a spiric section), so
  `crescent` reads no crescent there.

Neither is measured: no pose of either is built yet. Read as a touch,
a union over such a crescent may keep both faces whole across it, as
the plane case did before PR 5687 refused it. The first step is the
pose, every op in both orders, to see what each builds or refuses
today.
