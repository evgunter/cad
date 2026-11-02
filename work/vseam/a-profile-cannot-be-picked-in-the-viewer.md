---
id: a-profile-cannot-be-picked-in-the-viewer
kind: issue
title: A profile cannot be selected by clicking it in the viewer
status: open
opened: 2026-11-02
priority: P1
cost: M
---

**Reported against an older build** (Ev, in chat, 2026-11-02: "i had
accidentally built a slightly old version of the gui"). Reproduce on
current main before building anything; if main no longer shows it, close
the row saying so.

Reported by Ev (in chat, 2026-11-02): "profiles should be able to be
selected by clicking in the viewer". Filed by the VSEAM orchestrator.

The viewer draws a committed profile's outline in the viewport
(`sketch::committed`), but a click does not select it. The pick index
(`crates/viewer/src/pickindex.rs`) indexes the tessellated bodies'
faces, edges and vertices. A profile is 2-D and has no tessellated
face, so it can be selected only from the feature tree.

A fix needs:
- the drawn profile outline (its polylines, in the space it is drawn
  in) entered into the pick index, or a pick pass of its own;
- what a hit selects: the profile node, and possibly one profile piece
  by its N1 name (`ProfileEdgeRef`), for tools that take a profile
  edge;
- precedence against a body face behind it.

`pickindex.rs` is shared with VGEOM (its numeric doors). This program
owns the seam the index lives behind.

P1: Ev reported it as a GUI defect, and the extrude and revolve forms
want a profile picked from the picture like every other input.
