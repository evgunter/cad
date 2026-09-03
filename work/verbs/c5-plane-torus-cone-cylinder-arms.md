---
id: c5-plane-torus-cone-cylinder-arms
kind: issue
title: C5 table - plane x torus and cone x cylinder section arms (blocking the Klein wall-pair debt)
status: open
opened: 2026-08-27
github: 1057
refs: [VERBS-C5ARMS, 1048]
---

## From GitHub issue 1057

Opened 2026-08-27; 0 comments.

**Two analytic section arms the C5 table is missing, and the demo debt they block.** Raised by OFF-D PR-2 (#1048) — see that PR's Klein deviation row and banked finding 5, and the pinned row `verbs_shell::the_klein_wall_pair_waits_on_a_plane_torus_route`.

**`plane × torus`.** `geom_brep::intersect::route(Plane, Torus).implemented` is false, so `topo::replace_face_offset` refuses `NeighborPairUnroutable { Plane, Torus }` and `topo::shell` carries that up. The configuration actually needed is the easy one: **a plane CONTAINING the torus axis cuts it in two circles** — closed form, no marching, the same shape as the existing `plane_cylinder_section` / `plane_cone_section` doors. (The general tilted case is a quartic and is not what any current consumer needs.)

**`cone × cylinder`.** Same table, same shape of gap. Pinned by `verbs_offd::an_undescribable_neighbor_pair_refuses_typed`.

**What they block.** The Klein bottle's walls are all revolved, so every one of its hand-built `r ± t/2` wall pairs — the demo's own findings list calls this "paid once per wall" — waits on these two arms before a `shell` call can replace it. A partial revolve of a disc gives a torus wall and two planar meridian caps, so every rim is `plane × torus`; the flare adds `cone × cylinder`. Until both land, the debt cannot start retiring and the demo keeps spelling the wall thickness into two call sites per elbow.

The contract for the eventual re-authoring is recorded in #1048: **naturalness, not byte-identity**, per the demo rule. The pinned row states the comparison it will make when the arms land (topology exactly equal; stored radii within one ulp, since the two spellings reach the inner radius by different float routes; volume within 1e-12).

## Home

VERBS' charter names the C5 section arms; the unit VERBS-C5ARMS already carries this issue's two arms (PR-1 merged, PR-2 remaining).
