---
id: the-sphere-cut-in-reads-its-circle-from-the-door-not-the-section-table
kind: issue
title: The sphere cut-in reads its pair's circle from the door, not from the section table that now holds it
status: open
opened: 2026-11-21
---



Filed by the SECTCONE U1 lane (`the-plane-and-sphere-conic-pairs-on-the-section-value`) from its sweep.

## What

Sphere × sphere is now built on the section value (`geom_brep::pair_section`, `PairPose::SphereSphere`), and the glue door's section table holds every overlapping sphere pair. The boolean's sphere cut-in still reads the pair's circle straight from the door:

- `crates/topo/src/boolean/ops.rs`, `sphere_pair_cut` (`geom_brep::sphere_sphere_section(x, y, band)`), the circle a sphere group is re-cut along;
- `crates/topo/src/boolean/ops.rs`, `pair_seam` (the same door with A's sphere first), the seam both groups' re-charts share.

The value reads the same door, so the two cannot disagree on the circle today. They are still a second reading of a pair the table holds, with their own refusal mapping (`SphereQuestion::CutIn`) beside the table's (`faces_section_unbuilt`).

## Open question

The cut-in runs in the extent scan, which may come before the glue door builds the table. If so, the door is the only reading available there, and this row closes as "not a second reading". Otherwise the cut-in should take the circle from `SectionTable::held`.

## Evidence (2026-11-23, ORBIT `orbit/one-sphere-escape-cut`, PR 7602)

`pair_seam` is gone with the re-chart it served. `sphere_pair_cut` is the one remaining reading of the pair's circle from the door, and it now serves closed balls too.
