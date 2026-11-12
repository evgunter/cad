---
id: boolean-reads-face-pair-facing-at-a-point-four-more-ways
kind: issue
title: The boolean reads two faces' facing at a point four more ways, each its own cosine sign beside a reading that already decided the pair
status: open
opened: 2026-11-12
priority: P2
---


Found by the face-pair unit's sweep (`face-pair-contact-pairing-has-four-spellings`). That unit moved the census, the link, `at_rest` and the contact row onto the reading each one's question is about. Four more readers in `crates/topo/src/boolean/` ask whether two faces' outward normals point one way or opposite ways at a point. Each decides its own cosine sign under its own K name and lever:

- `overlap.rs` `one_sense` (`bool_overlap_senses`): two faces on one carrier, read at the middle of the first face's first edge over an unsourced unit lever. Whether two faces on one carrier face one way is what the carrier relation (`carrier_pair_relation`: `SameOriented` / `SameOpposite`) decides once per pair.
- `join.rs` `stands_on` (`bool_rim_face_opposed`): a rim face's plane normal against the other operand's plane normal `n_f`, levered by `wedge_reach`.
- `node.rs` `wall_sense` (`bool_node_wall_sense`): a tangent wall's outward normal against its partner's, after tangency was read. It is a material pairing of a tangency, which is `geom_brep::Tangency::pairing`'s question, or `GradientAlignment::turned` where no token is held.
- `sectors.rs` `tangent_lump`, its exact-zero osculating residue (`tangent_lump_senses`): a tangent sector's outward normal against a reference normal, read to pick the `PlaneRelation`.

Wanted: for each, find the reading that already decided the pair parallel or one carrier, and read the facing off it, as the face-pair unit did. Where none exists, read through `geom_brep::gradient_alignment`, which returns `Perpendicular` only past a lever decided positive. Not executed; the four were found by reading.
