---
id: at-rest-opposed-decides-a-unitless-dot-as-a-length
kind: issue
title: The at-rest quieting rule's opposed test decides a unitless dot product as a length margin, and reads curved faces' senses in one fixed role order
status: closed
opened: 2026-10-26
priority: P3
cost: E
refs: [the-at-rest-census-is-a-check, material-and-convexity-are-read-three-ways]
closed: 2026-10-26
---


`crates/editor-core/src/checks/at_rest.rs` `opposed` is what stage 5 B's interference quieting reads to require "an opposed pair of faces" (`[assertions.quiet]`). It has two defects:

- **Plane arm:** `decide(AT_REST_QUIET_OPPOSED, Margin::of(wo.dot(wi)), band)` decides a dimensionless cosine against the length band, with no lever arm. `[tolerance.margins]` asks that every margin be a length. The test should ask the question the rest of the kernel asks at a contact: the material pairing (`geom_brep::classify_material_pairing`, the door the census, `boolean/contact_verify.rs`, `splitting/finish.rs` and validate check 4 all use).
- **Cylinder and sphere arms:** these compare the two faces' `sense` bits in a fixed role order (outer concave, inner convex). They do not decide that the two faces are opposed where they meet, and they read the same fact that `Carrier`'s `convex` now carries.

Found by designer B on `material-and-convexity-are-read-three-ways` (2026-10-26), and confirmed by reading the code. Nothing measured shows a wrong quiet: a near-perpendicular pair reads not-opposed and stays loud, which is the safe direction. The file is D′3's until #5636 merges, so the fix rides stage 5 C (`the-at-rest-census-is-a-check`), which moves this rule. If the material-reader design decides otherwise, it rides that design's build instead.

## Closed

2026-10-26, fixed by stage 5 C (`the-at-rest-census-is-a-check`):
`checks::at_rest::opposed` asks the material pairing door
(`geom_brep::classify_material_pairing_as` under `at_rest_quiet_opposed`)
at a point of the outer face, levered by `geom_brep::folded_lever_arm`
over the two faces' reach from it (`eval::measure::reach_of`), for every
carrier pair; the plane arm's bare dot and the curved arms' fixed role
order are gone. A pairing the door cannot decide is not opposed, the
safe direction.
