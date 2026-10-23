---
id: a-symmetric-parts-seam-is-not-walked-as-one-construction
kind: issue
title: The coincidence walk stops at a mirror, so a symmetric part's seam is glued by value where D10 says it is one construction read twice
status: open
opened: 2026-10-22
priority: P1
cost: M
design: true
refs: [a-mirror-is-a-construction]
---


Found by `a-mirror-is-a-construction` (2026-10-22). D10 `[repetition]` says a symmetric part, `union(body, Mirror { body, plane })`, is "one construction whose seam is the plane's face read twice". Today the union glues the seam by value: DM4's pairwise pass meets two coplanar faces and fuses them (`crates/editor-core/tests/a_mirror_is_a_construction.rs`, `a_body_and_its_mirror_across_its_own_face_union_to_one_solid`; the `symmetric_plate` corpus document). The coincidence door's walk does not reach it: `coincide::construction` (`crates/editor-core/src/coincide.rs`) follows `Part` and `Split` from a name to its minter, and stops at a mirror (`Unwalked::Through`), since a mirror carries its source's names but reflects their geometry (`names::VerbatimEdge::Reflected`).

The open question is what the walk should say through a mirror. The image of a face lying in the mirror plane is that face with its normal reversed, so a mirror across a face's own plane carries that face onto itself, and the two reads meet as one construction. Every other face's image is a new construction. Weigh it with COINCIDE, whose door it is.
