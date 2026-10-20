---
id: the-cover-census-may-subsume-the-seam-regions-pole-read
kind: issue
title: The cover census catches every seam-region flip the pole read does on the flip corpus; retiring the pole read is Ev's, since it rewrites C12 (7)
status: open
opened: 2026-10-20
priority: P4
cost: E
design: true
refs: [a-seam-region-covering-the-sphere-twice-is-unread, a-seam-region-whose-boundary-carries-both-poles-is-unread]
---


## Measured

Tier 3's check 6 reads a seam region twice (`crates/topo/src/validate.rs`,
the region read in check 6):

- the **pole read** (`crates/geom-brep/src/props/curved.rs`,
  `sphere_region_material_sign`) derives the region's side from a pole on
  an interior edge, or from the area sign of a multi-loop boundary;
- the **cover census** (`sphere_region_cover`, the unit that closed
  `a-seam-region-covering-the-sphere-twice-is-unread` and
  `a-seam-region-whose-boundary-carries-both-poles-is-unread`) refuses a
  region whose members, under their bits, claim it more than once.

Over the seam-region and sphere-flip suites (both witnesses,
`seam_siblings_flipped_together`, `sphere_face_flips_are_refused`,
`tilted_sphere_pair`, `a_flipped_shallow_dimple`, step-import
`poles_inside_a_seam_region`), 344 region reads, the census also caught
every flip the pole read caught. Every region the pole read encoded
against its members' common bit read `c ≥ 1` (2026-10-20, a probe on
`flux/seam-region-unread`). The two reads differ only where the census
does not read:
- members whose bits disagree, check 4's lamina;
- a ringed member's flip, which that member's own read refuses.

## The question

Retiring the pole read would leave check 6 one region reading and no
pole rule for regions. It rewrites C12 (7)'s ratified paragraph
(`crates/geom-brep/README.md`, "a pole on an edge interior to the region
lies inside it"), so the choice is Ev's: an `[ev]` PR, weighed first by
a designer pair. One loss to weigh: the pole read names each flipped
member (`CurvedSenseInverted`), and the census names only the region.
