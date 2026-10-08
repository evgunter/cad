---
id: the-pre-zip-pinch-weld-retires-once-coincident-pierces-split-per-cone
kind: issue
title: The pre-zip pinch weld (finish::weld_pinches) stays as the repair of an operand's coincident pierces; it retires once those split per cone (D10 ground)
status: open
opened: 2026-10-06
priority: P1
cost: H
refs: [a-pinch-no-kept-face-can-cross-refuses, a-hole-weld-cannot-tell-a-figure-eight-hole-from-an-island-face]
blocked_on: [intent-stage4-is-built]
---


## What

Found building `a-pinch-no-kept-face-can-cross-refuses` (branch
`join/pinch-one-vertex-per-cone-build`). The ruling (Ev, PR 4057) retires
the pinch welds. The post-zip pierce weld (`weld_pierce_copies`) is
retired there. The pre-zip one, `finish::weld_pinches`, cannot go yet.

It fires where two coincident edges of one operand pierce the other's
face at one point. That operand carries a contact of its own: a pinch
line where two of its lumps touch. Each pierce mints its own ring vertex
in the pierced face, so before any zip that face holds two vertices on
one point whose corners overlap: the operand's own fragment crosses
there. `weld_pinches` repairs it by fusing the two into one vertex, its
chord dividing a bow-tie into two faces. That vertex then goes through
`zip::split_cones` with the rest. The split can separate a vertex per
cone, but it cannot uncross two.

Measured with the weld removed, on that branch:
- **Declared ground (D10).** `topo::all union_flush_onto_edge_contact`'s
  three rows go from building to tier-3′ `UndeclaredContact`
  (`VertexVertex` and `EdgeEdgeOverlap` between the two pierce copies).
  The copies sit on two point keys, and the contact records reach them
  only through the weld rows (`ops::Descendants`).
- **Undeclared.** `editor-core::all union_pinch_member_order`'s four
  rows build, then the mesher refuses `PinchWedge`: one face's corners
  cross at the pinch.

## The shape to give

Mint an operand's coincident pierces on one point key with their corners
already per cone (each pierce's ring vertex holding the corners of its
own cone), so no repair weld is needed. Then retire `weld_pinches`,
`pinch_site` and `Joint::Hole`. The declared rows move only when D10
lifts, and their contact records then have to name both copies.

## 2026-10-06 — the weld's site must nest (PR 4139's review r1, MAJOR-1)

A pinched operand (two reflex corners touching only at `v`) against a
face through `v`, one corner crossing the face in two sectors and the
other in one between them (r1's set `dbl`). The first corner leaves a
copy of its pierce vertex per pair of sector edges, each with a corner
of the face, and the second's pierce lies inside one copy's corner
only. `pinch_site` asks only for a lineage face both vertices have a
corner of, so the weld chorded the second pierce to the copy it met
first. Its fan then spliced out of angular order, `split_cones` read
two cones where the link holds one, and 16 lines refused
`LoopRoleInverted` (12 of them `SOUND` on main).

A weld now joins two pierces only where their corners of the site face
nest: every edge leaving either vertex runs inside the other's corner
(`finish::corners_nest`, read on `sectors::orbit_corners`). Against the
PR's previous head `7863f27b`, over `dbl`
(4320 lines) 108 lines move, every one refusal → built (99 `SOUND`, 9
failing tier 3′ only, on the undeclared contact at `v`), none to a
wrong vertex count; pinned by
`a_pinched_operands_pierces_weld_where_their_corners_nest`
(`crates/sweep/tests/join_pierce_runs_sweep.rs`). The shape to give
above stands: minting the coincident pierces on one point key with
their corners already per cone would need no nesting read at all.

## 2026-10-07 — parked on D10

The weld cannot retire without its declared rows (`union_flush_onto_edge_contact`)
regressing to `UndeclaredContact`, and fixing those means contact records
naming both copies: declared contact, held ground under D10. The per-cone
minting itself is taken up undeclared by
`a-vertex-two-crossing-pairs-cut-is-the-in-end-of-one-null-edge-and-the-out-end-of-another`
(a split per cone at insertion); this row resumes from there when D10 lifts.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: the weld cannot retire until the declared union_flush_onto_edge_contact rows' contact records name both copies, which stage 4 rewrites at the one door. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)
