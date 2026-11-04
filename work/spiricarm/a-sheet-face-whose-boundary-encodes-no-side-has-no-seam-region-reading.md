---
id: a-sheet-face-whose-boundary-encodes-no-side-has-no-seam-region-reading
kind: issue
title: A spindle sheet face whose boundary encodes no side is exempt from check 6's sense comparison: read_seam_regions reads sphere faces only
status: open
opened: 2026-11-04
priority: P3
cost: M
---


(SPIRICARM, stage 2 of `horn-and-spindle-tori-have-no-representation`.)

## What

The football's two sheet faces are π-bands between the meridians that
end at both apexes. Their boundary parts the closed sheet in two, so
`geom-brep::props::curved::sheet_material_sign` reads it `Unencoded`.
Tier 3's check 6 (`topo::validate`, the curved arm) then looks for the
face's seam region, which would read the side from the region's
boundary. But `topo::props::read_seam_regions` gathers `Surface::Sphere`
faces only, so the sheet face finds no region and the comparison of its
bit against its boundary is exempt.

The bit is not unguarded. Flipping one sheet face's bit refuses at
tier 3, and so does flipping both or `revert()`, through the body's
other checks (`LaminaWedge`, `NegativeVolume`).
`crates/sweep/tests/the_football_revolves.rs`
`a_football_with_a_sheet_faces_bit_flipped_is_refused` pins that. The
sense comparison itself, though, has no reading for a sheet.

Swept with it: the other sphere-only gates in the curved-sense
machinery are `geom-brep::props::sphere_region_material_sign`,
`sphere_region_cover` and `seam_loops`. Each is called only from
`read_seam_regions`' sphere path (`topo::props`). `CurvedSenseInverted`
has no exempt list of its own: it is skipped exactly where the material
sign refuses or where no region reads.

## Done when

A sheet face whose boundary encodes no side is read through its seam
region, as a sphere's lune is, or this exemption is pinned by a row and
named in check 6's not-yet-checked list.
