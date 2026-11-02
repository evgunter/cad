---
id: a-profile-cannot-be-picked-in-the-viewer
kind: issue
title: A profile cannot be selected by clicking it in the viewer
status: closed
opened: 2026-11-02
closed: 2026-11-02
branch: vseam/a-profile-is-picked-in-the-viewer
pr: 6299
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

## Closed (2026-11-02, PR 6299)

Reproduced on main by reading the pick path: `PickIndex::hovered_for`
answered `None` whenever the ray met no body, and the index held no
profile geometry, so a click on a drawn outline selected nothing. The
two new rows below fail with the profile pass disabled.

The index now holds each committed profile's drawn outline
(`crates/viewer/src/pickindex.rs`, `Outline`, built in
`PickIndex::assemble` from `sketch::committed` at the picture's δ),
under the same `PictureKey` as the bodies. `hovered_for` weighs an
outline as an edge: within `EDGE_PICK_RADIUS_PX`, not hidden by the
solid, the nearer of it and the nearest body edge wins, the body edge
at a tie; under the bare cursor's `PickKinds` only. A hit hovers
`Hovered::Profile` and a click selects `Selection::Node(profile)`;
`marks::edge_overlay` lights the outline when it is hovered or
selected. GQ7 in `crates/viewer/GUI-DESIGN.md` states the rule.

Rows: `app::properties_pane_tests::a_click_on_a_profiles_outline_selects_it_and_beside_it_nothing`
and `a_profile_on_a_face_is_picked_at_its_outline_and_the_face_inside_it`.

Residue, each its own file:
`a-profile-pick-names-no-profile-piece` (a click names no
`ProfileEdgeRef`) and
`a-profile-edited-in-place-is-picked-by-its-undrawn-outline`.
