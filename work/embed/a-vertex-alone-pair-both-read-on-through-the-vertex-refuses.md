---
id: a-vertex-alone-pair-both-read-on-through-the-vertex-refuses
kind: issue
title: A pair sharing a vertex alone whose faces are both read on through it along their conics refuses, and a past curve there is read under the graph separator only
status: open
opened: 2026-11-18
priority: P3
cost: M
---

Filed by lane `embed/conic-vertex` (2026-11-18), as what its reading
leaves out. No measured body reaches either shape today.

`Reading::Conic` reads a corner of two arcs of one conic at a vertex the
pair shares alone (`corner_of`'s `alone`), its zero the conic run on
through the vertex (`Curved::past`). Such a zero is read only against a
partner whose zero set is on the shared cell (`past_read`), and only
under the graph separator built over that conic at the vertex
(`graph_of`). So:

- **Both faces read on through the vertex** (two faces each bounded at
  the vertex by two arcs of a conic, touching there alone) is refused at
  once (`stuck`): neither is read against the other. A reading would
  need the two runs decided apart in `R` off the vertex, which is the
  same question as the shared-edge case's
  `two_faces_run_on_along_one_circle_from_both_sides_do_not_certify`.
- **No plane reads a past curve at a vertex alone.** `planes_through`
  builds a plane through the vertex listing no edge, so it vanishes on
  no curve. The conic's own plane, listing both arcs, would hold the
  whole run; on the tee it does not part the pair (the cap's line edge
  lies in it), so nothing measured needs it.
- **Charts whose image of the conic no presentation reads** (a sphere's,
  a torus's or a cone's image of an off-axis conic) keep the corner
  `Structural`, as the shared-edge reading does (the two-arc lune).
