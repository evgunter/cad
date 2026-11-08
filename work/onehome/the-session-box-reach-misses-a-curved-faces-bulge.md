---
id: the-session-box-reach-misses-a-curved-faces-bulge
kind: issue
title: The session box reads a body's vertices and edges, so a curved face's far side no edge rides is missed
status: open
opened: 2026-11-02
---


`crates/geom-core/README.md` ("Float rounding") defines a body's reach for the session-box predicate at the finished-body gate as read over its vertices, its edges' spans and its faces' boundaries. PR 6380's `topo::session_box` (rework: through `geom_brep::extent::edge_reach_bounds`) reads exactly that. A curved face's interior can reach farther than any edge on it, and the definition misses that.

**Example (from PR 6380's FULL review; likely, not run).**
- The body is the `bulge_loop` torus (latitude seams at radius R), with r = 400 and R = 450.
- It is turned so its meridian faces the origin, and centred at 250 m.
- The gate reads about 806 m; the true reach is about 1100 m.

So a body that leaves the 1 km box is admitted, and the floor premise of the admission rule is false for it.

**Shape of the fix.** A face's reach comes from its surface over the face's own region, not from its boundary:
- a sphere or torus: the farthest point of the surface patch;
- a NURBS face: its control net clipped to the face.

That reach should come from the one extent home (`work/onehome/reach-names-eight-unrelated-types.md`, the `onehome/one-extent` lane), not from a new walk in topo. The README's sentence then names faces' regions, not their boundaries. That is a change to ratified text, so it goes to Ev.
