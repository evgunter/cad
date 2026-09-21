---
id: c5-plane-torus-cone-cylinder-arms
kind: issue
title: C5 table - plane x torus and cone x cylinder section arms (blocking the Klein wall-pair debt)
status: open
opened: 2026-08-27
github: 1057
refs: [VERBS-C5ARMS, 1048]
priority: P1
cost: H
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

**VERBS closed** (exit walk ratified, PR #1793); re-homed to
`work/issues/` awaiting an owner.

**Adopted by CURVED** at its opening for dispatch (2026-09-04, Ev's
in-chat direction): the plan's lane that carries this item is in
`work/curved/plan.md`.

**Both section arms delivered** (PR-1 #1577 plane×torus; PR-2 #1864
coaxial cone×cylinder, 2026-09-05). What this issue still blocks on is
not a section arm: the Klein wall-pair debt (rows 3/4/8) waits on the
spiric rim carrier — `docs/CURVED-SPIRIC-DESIGN.md`, ratified
2026-09-04, a CURVED unit to cut.

**2026-09-13 (CURVED-SPIRIC PR-1a).** The elbow's rim now mints as
`Curve3::Spiric` (the kernel half of rows 3/4/8's carrier debt), and
the measurement moved the elbow's wall to a door the spec did not
predict: `shell`/`shell_open` on the klein elbow refuse at the
EQUATOR SEAMS' re-author (`offset_axial_reauthor_plane` — a disc's
profile vertices revolve into `RevolvedPoint`-declared chart seams
whose corner the moved cap displaces off the sketch plane;
`torax_axial`, `verbs_shell`, `torax_interval`, `shell7_seam_corner`).
The sectioned torus VESSEL, whose band has no such seam, reaches tier
3 and stops at the props door (`spiric_rim`, the tour's `torusvessel`
wall 1). Two doors now stand between the elbow and its re-authoring:
the seam re-author (an orchestrator question raised by PR-1a) and the
props quadrature lane for a spiric-bounded face (the spiric unit's
PR-2, after PR-1b's pcurve variant and STEP spline).
