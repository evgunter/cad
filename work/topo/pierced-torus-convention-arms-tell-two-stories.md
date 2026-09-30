---
id: pierced-torus-convention-arms-tell-two-stories
kind: issue
title: topo: a pierced torus's tube and ring decisions end 'reshape the torus' in band and 'not supported yet' when definite
status: open
opened: 2026-09-30
priority: P2
cost: E
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
