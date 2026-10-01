---
id: plane-section-reports-hole-polygons-clockwise-with-no-role
kind: issue
title: plane_section documents every polygon counter-clockwise, but a hole's polygon comes back clockwise and nothing says which polygons are holes
status: open
opened: 2026-10-01
priority: P2
cost: E
---


Found by the section-rings lane (`cleave/section-rings`), sweeping the
other consumer of the split's completed section polygons.

**Repro.** The 4³ block less the U-cutter
(`crates/sweep/tests/split_section_rings.rs`, `u_cut`), sliced by
`plane_section` through `(3, 0, 0)` with normal `+x`. The three
polygons' shoelace areas over their `uv` are `−1`, `−1` and `16`: the
two prongs' holes wind clockwise, the outline counter-clockwise.

**What it says.** `plane_section`'s docs
(`crates/topo/src/splitting/section.rs`, the `# Winding contract`
section) promise every polygon positive signed area, "consumers
computing signed areas or offsets may rely on this orientation". A
section with a hole breaks that, and `SectionPolygon` carries no role,
so a consumer cannot tell a hole from an island except by re-deriving
containment. The split itself now nests hole polygons as rings of the
face around them (`finish.rs`, `nest_hole_sections`); the slicing
query reads the same polygons before that step.

**Fix shape.** Either state the contract as "outlines CCW, holes CW"
(the sign then IS the role), or group the polygons as the split's
finish does (outline plus its holes) and say so.
