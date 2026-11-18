---
id: a-first-round-trips-geometry-drift-is-read-at-eps
kind: issue
title: The first round trip's geometric drift (E1 vs E2) is read at ε
status: open
opened: 2026-11-18
---


Found by `importgate/build` (PR 7128), the fix pass's ruling m5.

## What

The byte fixed-point rows (`roundtrip::fixed_point`,
`freecad::cross_dialect_fixed_point`,
`wild::wild_bodies_are_a_fixed_point_of_our_own_dialect`) compare the
second round trip's export with the first's (E2 against E3), since the
first export may differ from the committed file by the rounding a
construction seeded from its digits leaves. Censuses and certified
volumes are compared across every pass. So the first hop's geometric
drift, a move that changes neither a volume past its pad nor a census,
is seen by no row: E1 against E2 is compared only through those.

## What would close it

A row comparing E1's and E2's geometry at ε: every vertex within ε of
its counterpart, and every edge's carrier holding the other's samples
at ε, matched by the writer's traversal order.
