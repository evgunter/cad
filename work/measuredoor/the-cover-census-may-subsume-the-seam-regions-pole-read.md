---
id: the-cover-census-may-subsume-the-seam-regions-pole-read
kind: issue
title: The cover census catches every seam-region flip the pole read does on the flip corpus; retiring the pole read is Ev's, since it rewrites C12 (7)
status: open
opened: 2026-10-20
priority: P4
cost: E
design: true
refs: [5516]
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
`poles_inside_a_seam_region`, and both PR 5516 reviews' probes), the
census reads every seam region of two faces or more: 5653 region reads
at 1e-9, 4441 at 1e-12 and 4451 at 1e-6 (2026-10-21, a probe on
`flux/seam-region-unread`). Wherever the pole read refused a region
whose members all carry one bit, the census refused it too, except
where a member carries rings: 6, 6 and 13 such reads. There a flipped
member's claim is a negative "area" and the census reads the region
once (its stated exemption), and the ringed member's own read refuses
it. So the two reads differ only where the census does not read:
- members whose bits disagree, check 4's lamina;
- a ringed member's flip, which that member's own read refuses.

## The question

Retiring the pole read would leave check 6 one region reading and no
pole rule for regions. It rewrites C12 (7)'s ratified paragraph
(`crates/geom-brep/README.md`, "a pole on an edge interior to the region
lies inside it"), so the choice is Ev's: an `[ev]` PR, weighed first by
a designer pair. One loss to weigh: the pole read names each flipped
member (`CurvedSenseInverted`), and the census names only the region.
