---
id: a-coplanar-sector-offers-a-rest-the-door-contradicts-across-the-faces
kind: issue
title: a coplanar sector's in-band refusal offers a Rest declaration that the door then contradicts, once the door levers the tilt across the faces
status: open
opened: 2026-10-02
---

## What

The coplanar-sector refusal (`reduce.rs`, the `Coincide::Sectors`
decision) reads a sector pair's normals at the CORNER's arm, finds them
in band, and offers "declare the coincidence" with `Rest` as the class
the door admits on a planar pierced face. The `Rest` door
(`rest::carrier_pair_verdict`) reads the declared pair as one
displacement over its consumed extent — a ball around BOTH faces, and
their vertices (`carrier_eq::declared_reading`, since TANG's
`torus-carrier-axis-margin-is-levered-by-one-not-the-ring`). Where a
face reaches far from the corner, the tilt that read in band at the
corner reads definite across the faces, and the declaration the
refusal offered is contradicted (`ContactContradicted`,
`PlanesNotParallel`).

Pinned by `reduce.rs`'s
`a_coplanar_sectors_in_band_parallelism_offers_a_declaration_the_door_reads_across_the_faces`:
a 5° wedge tilted `5.5ε` on a 3 m × 4.5 m block top, standing on it
(offered `Rest`) and sunk into it (offered a continuation). Undeclared
each offers the declaration; following the offer, the door contradicts
it. Where the tilt is small enough that the door bridges it over both
faces, the lump takes the residue and the union builds
(`a_coplanar_sectors_in_band_residue_builds_through_the_lump_where_the_door_bridges_it`:
standing at `1.2ε`, sunk at `2ε`); standing at `2ε` the door accepts
and the zip refuses `RestZipUnsupported { ChordBetweenIsolatedPierces }`.

## Shape of a fix

The offer is a menu entry; it should be offered only where the door
would take it, i.e. where the pair's in-band reading also holds at the
door's lever (`offer_rows`' `Withdrawn` mechanism is the existing
vocabulary for an offer shown false). Measure which other
sector-level offers read a tilt at a shorter arm than the door.
