---
id: pierced-torus-convention-arms-tell-two-stories
kind: issue
title: topo: a pierced torus's tube and ring decisions end 'reshape the torus' in band and 'not supported yet' when definite
status: closed
opened: 2026-09-30
priority: P2
cost: E
branch: topo/torus-and-merge-one-story
pr: 3506
closed: 2026-09-30
---


(TOPO, the §5 sweep of PR 3493's fix pass: each escalated decision's
definite sibling, checked for D4 ¶1 (iv).)

## What

`face_normal::face_outward_normal_at` (`crates/topo/src/face_normal.rs`,
the `Surface::Torus` guard arm) reads `geom::torus_tube` and
`geom::ring_torus` before it differentiates a pierced torus.

- In band, each escalates, and the Boolean
  (`boolean::vtxfac::classify_vertex_on_face`) renders
  `BooleanDecision::TorusTube` / `TorusRing`: "Recourse: reshape the
  torus so its tube is clearly thicker than the tolerance" (or "…stays
  clearly off its axis"), with the valued tolerance on a positive
  margin.
- Definitely non-positive, the arm returns `Ok(None)`, and the Boolean
  renders `BooleanError::CurvedBooleanUnsupported`: "that pairing of
  faces is not supported yet", with `meeting_recourse`'s "reshape the
  parts so they meet only where a plane face meets a plane, cylinder or
  sphere face, or move them so the torus face stays clear of the other
  solid".

A spindle torus is geometry a user reaches by moving the torus's radii,
so D4 ¶1 (iv) wants its definite arm to tell the in-band arm's story
with the same one recourse, and the two arms here tell two.

## Repair shape

Decide which story the decision has. If a spindle or horn torus is a
shape the kernel will never pierce, the definite arm is a refusal of
the torus's own shape and ends in the same lever as the in-band arm
(`BooleanDecision::TorusRing`'s). If it is coverage not built yet, the
in-band arm is the same gap and should say so rather than offer a
tolerance.

## Close (PR 3506)

The spindle and horn torus are shapes the kernel never represents
(D1), so the definite arm refuses the torus's own shape and ends in the
in-band arm's lever, from one closed type both arms read,
`geom_brep::TorusConvention` (`crates/geom-brep/src/torus_convention.rs`),
which tier 3's ring half reads too. The story is told at the pierce
normal (`face_normal::face_outward_normal_at`, routed by
`BooleanError::of_pierced_normal`): the pierce lane reaches it for a
point already placed in a torus face's trim. No public door reaches it
with a degenerate torus, because that placement reads the convention
first (`boolean::solid_contain::point_on_torus_in_face`'s
`require_ring_torus`): through `union`/`subtract` such a torus stops at
`CurvedPierceUnsupported`'s declare menu or `Containment`'s "the solid
itself is fine", as on main. Those two front-door stories are filed on
CONTACT's slate:
`work/contact/degenerate-torus-operand-meets-the-declare-menu-and-a-false-solid-is-fine.md`.

## Closed (2026-09-30, PR 3506)

Built to D4 ¶1 (i)/(iv), ratified by Ev in PR 3352.
- **Torus.** A spindle or horn torus is a shape the kernel never
  represents, per D1 ("spindle tori have no representation"), the type's
  own R > r > 0 convention, and the fact that no constructor mints one.
  The closed `geom_brep::TorusConvention {Tube, Ring}` is shared by
  tier 3 and the Boolean's pierce door. It ends both arms in one lever,
  with the valued tolerance only on the band-decided arm.
  - Through the Boolean's front door, a degenerate torus operand still
    stops earlier, at the trim placement. That is filed on CONTACT as
    `degenerate-torus-operand-meets-the-declare-menu-and-a-false-solid-is-fine`.
- **Plane ladder.** A closed `PlaneRung` is routed by `PlaneDoor`
  (Undeclared, Declared, Neighbours).
  - The Boolean's cross-operand orientation (`PLANE_ORIENTATION`,
    `NonZero`) and the merge's declared orientation
    (`DECLARED_ORIENTATION`, Positive) are separate decisions, because
    their doors accept different outcomes.
  - `DeclaredOppositeOrientation` is the merge decision's sign-certain
    arm.
  - Both levers name the shared edge's chord, which is what the margin
    measures.
  - No declaration is offered on a declared pair or on a same-operand
    pair.

The single review found the first head shared one orientation table
across the two doors (a fresh D4 (iv) fork). The fix pass split it.

Filed from the unit:
- the closed-edge chord arm;
- the plane-offset `INVALID` encoding;
- F7's same-operand coincidence;
- revolve's axis clearance (CARVE);
- the shape-guard stage label (TINT);
- rows on GERM, OFFSET and CONTACT.
