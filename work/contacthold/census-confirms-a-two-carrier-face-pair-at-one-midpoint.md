---
id: census-confirms-a-two-carrier-face-pair-at-one-midpoint
kind: issue
title: The census confirms a two-carrier face–face record by placing the tangent line's overlap midpoint in both faces, one point: a trim the line enters only elsewhere reads stale
status: closed
branch: contacthold/one-tangent-interiors-reading
pr: 5006
opened: 2026-10-12
priority: P2
closed: 2026-10-19
---


Found building the face–face row (`contacthold/face-face-touch-row`).

## What

`census::confirm_face_pair` (`crates/topo/src/census.rs`), for two
faces on distinct carriers, derives the tangent line and both faces'
extents along it (`boolean::tangent_pair_at_rest`), then places ONE
point, the middle of that overlap, in each face by its containment
door. In both, the record is confirmed; off either, it is pushed
`StaleContactDeclaration`. Its own doc says so: "A trim the line enters
only elsewhere is read stale rather than blessed."

The extents are the faces' boundary vertices projected on the line, so
the overlap is a box, not the set of parameters where the line is in
both faces. A face with a hole, a notch or a non-convex trim can hold
the line over part of the overlap and not at its middle. The record is
then live and the census calls it stale. The boolean's own read of the
same question (`boolean::locus_through_both_faces`, `boolean/mod.rs`)
samples the certificate schedule over the overlap and mints the row if
any sample lies inside both faces, so the op can mint a record that 3′
then reads stale, and the carry (`records::landings`, through
`census::RecordConfirm::meets`) can consume or refuse one it should
land.

Not measured on a fixture; read from the code.

## Owed

The confirm reads the line over the overlap on the same schedule as the
op's read, and a witness: a face whose trim holds the line off the
overlap's middle (a plate face with a slot across the line's midpoint),
carrying a face–face record, confirmed at rest and landed by a carry.
