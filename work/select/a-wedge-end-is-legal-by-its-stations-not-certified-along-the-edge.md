---
id: a-wedge-end-is-legal-by-its-stations-not-certified-along-the-edge
kind: issue
title: A wedge end is legal by its stations, not certified jet-determinate along the edge: MATE-7's condition read pointwise until the drift bound can certify real cusps
status: open
opened: 2026-11-24
priority: P2
blocked_on: [a-band-seam-the-certificate-certifies-reads-under-determined-on-a-loose-drift-bound]
---


Found by PR 7471's delta review (R1, probe arc on
`heldrecords/review-elbow-seams-7d2e04`, `r1d_opposed_converse_arc_reads_one_end`),
filed by HELDRECORDS' elbow-seams fix pass on the orchestrator's ruling.

## What

MATE-7 (Ev, 2026-09-01) makes a 0/2π wedge (a cusp or a slit) legal only
where it is jet-determinate. The whole-edge fold
(`geom_brep::edge_wedge::material_outcome`) hands out
`WedgeVerdict::End` wherever the opposed faces' jets separate at every
station, and tier 3's check 4 (`topo::validate::wedge_reading`) accepts
it. Since PR 7471 the end carries a `bend`: `Determinate` only where
`|κ_rel|` is certified bounded away from zero along the edge. So an end
can be legal at the stations and uncertified between them. R1's arc
(a pipe against an elbow's torus over the converse arc, faces opposed,
both neutral points in the end gaps) reads one end at every station, its
`κ_rel` changing sign in the gaps, and tier 3 accepts it. Pinned at
today's reading:
`geom_brep::dihedral::bend_rows::an_opposed_bend_crossing_zero_off_its_stations_reads_one_uncertified_end`.

## Why it is not refused today

Refusing `End { bend: UnderDetermined }` was built and measured on PR
7471. It moved valid bodies to refusing:
`sweep::the_football_revolves::a_horns_cusp_revolved_part_way_is_its_lune`
and `a_horns_cusp_turns_on_the_faces_side_from_above_and_below` (a horn
torus's needle cusp, which main certifies, watertight at its closed
form). Their tube fails on `kappa_drift`, not on the cusp. The
gap is older than PR 7471, and no body reaches it today.

## Owed

Once the drift bound certifies real cusps
(`a-band-seam-the-certificate-certifies-reads-under-determined-on-a-loose-drift-bound`),
an end's legality reads the certified walk: tier 3 refuses
`End { bend: UnderDetermined }` typed, and the pinned row flips.
