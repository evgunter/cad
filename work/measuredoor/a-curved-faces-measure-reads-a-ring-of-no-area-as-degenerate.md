---
id: a-curved-faces-measure-reads-a-ring-of-no-area-as-degenerate
kind: issue
title: A curved face's measure reads a ring of no area, two coincident edges, as a degenerate face and refuses the body's volume
status: open
opened: 2026-10-26
priority: P1
cost: M
---



Found by the CLEAVE lane on `cleave/riders-read-every-edge-in-a-face`
(PR 5793), split off the CLEAVE row
`work/cleave/a-ring-of-no-area-in-a-curved-face-is-not-measured.md`,
which waits on this one. P1 for verb breadth: the geometry is a box cut
from a pipe with its corners touching the wall.

## What

Where an edge lies in a curved face with neither end on the face's
boundary and both its faces going into the face's material, D1 parts it
into two coincident edges carried by the face as a ring of no area
(`topo::boolean::slit::part`, `kfmrh_minting`). Built that way, the
result gate refuses the body `ResultInvalid` (`VolumeUncomputable`,
`DegenerateFace` on the curved face): the curved face's measure parses
its boundary into rims and reads a two-edge ring of no extent as
degenerate (`crates/geom-brep/src/props/curved.rs` `require_extent`,
`rim_side`). A plane face measures the same ring as nothing. Witness:
`sweep/tests/verbs_ga_r2_probes.rs`
`r2_a_box_with_on_carrier_rulings_builds_every_op_undeclared`, `p ∖ b`,
which the Boolean now refuses `EdgeInFace` (`CurvedRing`) before the
surgery so the gate never sees it.

## Fix

Read a ring whose two edges run along each other, opposite ways, as
contributing nothing to the face's flux and area, in every curved
lane, as the plane lane does; then CLEAVE drops the `CurvedRing`
refusal.
