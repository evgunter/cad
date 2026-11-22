---
id: a-moved-body-the-gate-admits-can-hold-an-edge-certification-calls-the-box-limit
kind: issue
title: A moved body the box's fold admits can hold an edge whose re-certification refuses as the box's limit
status: open
opened: 2026-11-22
---


Filed by `oneround/box-certify-ask` (PR 7272's fix pass), from its review's N6. By inspection; no fixture built.

**Finding.** The box reads one region two ways, and they can disagree on a body near the box's edge.
- **The fold** (`crates/topo/src/session_box.rs` `reach_of`, `past_its_carriers`) reads a face whose carrier is bounded by that carrier's own cover alone. `crates/topo/src/cover.rs` `face_cover_unproven`, about line 426, takes `surface.cover()` and reads the boundary only where the carrier is unbounded.
- **Certification** (`crates/geom-brep/src/session_box.rs` `edge_in_box`, check 0 of `certify.rs` `run_checks`) reads an edge by its own cover: its carrier's net over its span, hulled with its ends.

A spline edge lies on its faces, but its own control net need not lie in the faces' nets. A fitted section curve's net can be looser than the surface's. So take a moved body whose spline face's net stands inside the box while an edge's net pokes past it, and whose points all stand inside:
- `FinishedBody::moved` and the placing door (the fold) admit it;
- the edge's next re-certification, at rest (`validate`'s re-certification) or by any op that re-certifies it, refuses it as `CertifyError::SessionBoxUnshown`.

One body, inside the box by its points, gets two verdicts depending on which reader asked. Both are sound: neither admits a body past the box. But the admitted body cannot be re-certified.

**Shape of a fixture.** A NURBS-walled body (a loft) placed so that an edge's net, and not its wall's, reaches past 1 km. Then `moved_by` a translation, and re-certify one edge, or run `validate` at rest.

**Ways out.** These are for the owner to weigh:
- the fold also reads each certified edge's own cover (it already folds edge covers into `reach_of` for unbounded carriers);
- or certification reads the edge's region as the meet of its cover with its faces' covers;
- or this disagreement is accepted as a limit and stated in the README's "The session box".
