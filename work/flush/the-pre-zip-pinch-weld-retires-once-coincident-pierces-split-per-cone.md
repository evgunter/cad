---
id: the-pre-zip-pinch-weld-retires-once-coincident-pierces-split-per-cone
kind: issue
title: The pre-zip pinch weld (finish::weld_pinches) stays as the repair of an operand's coincident pierces; it retires once those split per cone (D10 ground)
status: closed
opened: 2026-10-06
priority: P1
cost: H
refs: [4139, a-hole-weld-cannot-tell-a-figure-eight-hole-from-an-island-face]
branch: flush/pinch-cluster
pr: 4728
closed: 2026-10-12
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

## Released by INTENT stage 4 E (`intent/s4-e-glue-on-zero`) (2026-10-09)

E does not reach this. `finish::weld_pinches` and `pinch_site` stand (`crates/topo/src/boolean/finish.rs:523`, `:738`). The contact records of `union_flush_onto_edge_contact`'s rows still reach the pierce copies only through the weld rows, because E records face-pair coincidences only (`crates/topo/src/boolean/glue.rs:40`). The shape to give stands: mint the coincident pierces on one point key per cone, and record the copies' v-v contact.

## Measured (2026-10-11, main `bef9a8f1`, plain booleans)

Main does not compile as it stands: `boolean/ops.rs` `piece_holds`
(PR 4650) misses `ContainError::ArcEndClearOfCorners` (PR 4613). This
branch's first commit routes it with the door's other unread refusals.

With `weld_pinches` skipped and nothing else changed (release), 15
tests go red, none of them through the declared surface, which F
retired:
- topo `union_flush_onto_edge_contact`: `a_pinch_line_through_a_face_…`
  refuses tier 3′ `UndeclaredContact { VertexVertex, EdgeEdgeOverlap }`
  between the two pierce copies at (0, 0, 1);
- topo `pierce_strut_at_a_pinch` (all four rows): `RingMeetsOuter`, or
  a vertex count one off;
- editor-core `union_pinch_member_order` (six rows): the mesher's
  `PinchWedge`, or `RingMeetsRing`;
- sweep `join_pierce_runs_sweep`: `a_pinched_operands_pierces_weld_…`
  (`dbl`, `xy U` fails 3′), `a_pinchs_cones_share_one_point_key`,
  `a_pinchs_unions_from_every_root_of_the_ring`,
  `the_join_stage_leaves_a_pinchs_cones_their_vertices`.

## Fix (branch `flush/pinch-cluster`)

The pierces of one face at one point mint one ring
(`vtxfac::mint_ring`). `classify_vertex_on_face` inserts the piercing
side's null edges and returns its half of the ring (`RingPlan`). Each
vertex-on-face pass then groups its pierces by face and point
(`vtxfac::ring_groups`, `bool_contact_vertex`). It mints one ring vertex
per group, with one strut tree over every member's runs
(`null::ring_tree` on their germs together). So the copies are on one
point key, with a vertex per corner of the face round the point, and
`zip::split_cones` splits them per cone as it does one pierce's.
- A tie between two members' germs (two cones touching along a ray in
  the face) is broken so that their runs' chords nest
  (`vtxfac::tie_breaks_after`). A tie inside one pierce still refuses.
- A group of one mints exactly as before, in the same order and with
  the same keys.

`finish::weld_pinches`, `weld_pair`, `pinch_site`, `corners_nest`,
`corner_holds_pierce`, `corner_holds` and `Joint::Hole` are gone, and so
are the weld rows in `FinishOut`, `Descendants` and `BooleanNaming`
(`weld_merges_b`, read by editor-core's `emit_topo`). The seam
correspondence accepts a ring copy that pairs with the runs of several
pierces of its face (`finish::one_ring`), where it accepted the weld's
vertex.

The contact records need no row for the copies: they share one point
key, which tier 3′ reads as structural (`census` `Geo::same_point`).

## Measured (release, ignored batteries, before and after)

The before side is the branch built with a switch that restores the
per-pierce rings and the weld. A group of one mints exactly as before.

| battery | lines | moved |
|---|---|---|
| `dbl_battery` (new) | 2 160 | 1 188 |
| `pinch_runs_battery` | 3 024 | 0 |
| `four_pairs_battery` | 5 334 | 0 |
| `corner_pairs_battery` | 16 381 | 0 |
| `pierce_runs_battery` | 4 537 | 0 |
| `near_tangent_battery` | 7 200 | 0 |
| `eight_crossings_with_a_pinched_cube_battery` | 576 | 0 |
| `review_sixx_pinch` | 207 | 0 |
| `curved_notch_pinch` | 1 513 | 0 |

`dbl_battery`'s moves, and none from `SOUND`:
- 1 126 `OK BAD` (tier 3′ only; 46 of them also a wrong cone count)
  → `SOUND`;
- 26 `ResultInvalid` → `SOUND`;
- 36 `yx S` lines → `ZipCorrespondence` ("a split section face does not
  pair one to one"): 23 were `OK BAD` with three vertices at `v` for
  two cones, 13 `ResultInvalid`. Filed as
  `a-cube-less-a-pinched-operand-piercing-its-face-refuses-the-split-cones-re-pair`.

The default suites of topo, sweep and editor-core move only where a
pin flips to 3′ passing:
- `pierce_strut_at_a_pinch`'s staircase `P − X`;
- `a_pinch_line_through_a_face_…` (FUSE's
  `a-pinch-line-crossing-a-face-interior-drops-the-pinchs-records`);
- `union_pinch_member_order`'s four rows (WIRE's
  `a-boolean-clipping-a-carried-contact-at-a-pinch-records-no-contact-for-the-pierce`);
- `a_pinched_operands_pierces_…`, all twelve lines now `SOUND`.

## Fix pass (PR 4728's dual review)

**The MAJOR: a shared key was a value decision nothing recorded.**
Taken as the orchestrator's option B: one key per pinch (Ev, PR 4057),
plus a record.
- `vtxfac::ring_groups` closes "one face, one point" transitively
  (union-find). It keeps each joining pair's `one_vertex_at` margin.
- `ops::ledger` records each pair whose operand vertices sit on two
  point keys. That record is a `decide_one` row (a `VertexFusion`
  coincidence, its margin kept), with a vertex-vertex record of the
  piercing operand's citing it. The carry then records every pair of
  result cells the two reach, and `cite_rows` emits the row for the
  `unproven-coincidence` lint.
- A pair already on one key is structure, and nothing is recorded.
- `census` `Geo::same_point` now states its second case and why the
  census does not ask for the record.

Not taken:
- A (distinct keys plus a record) revises PR 4057's pinch
  representation, and `edge_join` would then kill a cone's vertex.
  Measured: excluding ring keys from `zip::point_classes` left one
  vertex at `v` on `a_pinchs_cones_share_one_point_key`'s `fib21 xy U`.
- C (structural grouping only) contradicts D10's "glues it all the
  same".

**Rows.**
- editor-core `coincidence_door::a_ring_serving_two_pierces_records_their_fusion_unproven`:
  the lint names a fusion 1e-13 off bitwise, with the one-construction
  recourse. A second op over the same pinch on one key records nothing.
- Each flipped pin asserts the record and the row it rests on.
- `ring_groups_close_transitively_and_escalate_in_band`.
- `germs_of_two_pierces_on_one_ray_nest_and_a_third_refuses`.
- `m3_pr4_boolean`'s guard that a ring of one mints where a per-pierce
  ring did.
- R1's tie probe, now `a_face_holding_the_pinch_line_ties_the_pierces_germs`:
  its 18 lines refuse typed `PinchRaysInAFace` (they had refused
  `JoinDesync`), and slope −0.5 builds.

**Re-measured** (release, merge base `8b3ea728` against the fix head).
`dbl_battery` moves the same 1 188 lines: 1 152 to `SOUND`, and 36
`yx S` to `ZipCorrespondence` (23 `asym asym` `OK BAD`, 13 `Ltop asym`
`ResultInvalid`). No line leaves `SOUND`. The other eight batteries
are byte-identical.

## Closed (2026-10-12, PR 4728)

The pre-zip weld is retired, together with `weld_pinches`, `pinch_site`, `corners_nest`, `Joint::Hole` and the weld rows. The pierces of one face at one point now mint one ring (`vtxfac::ring_groups`, a union-find over `one_vertex_at`). The pinch keeps one point key (PR 4057). Every value-decided grouping that leaves the cones distinct is recorded at the door as a `VertexFusion` row, with its margin and a citing vertex-vertex record, and the `unproven-coincidence` lint reports it (D10, ruling B). Of the `dbl` battery's 2160 lines, 1152 move to SOUND and none leave SOUND. 36 lines that were wrong bodies or `ResultInvalid` now refuse `ZipCorrespondence`; that is the P2 `a-cube-less-a-pinched-operand-piercing-its-face-refuses-the-split-cones-re-pair`. Review: a dual concurrent pair (`docs/dual-review-log/rows/PR-4728.md`); both reviewers raised the same MAJOR, then confirmed the fix pass. A value glue into one vertex records no row; whether it owes one is filed at `work/intent/a-boolean-value-glue-into-one-vertex-records-no-coincidence.md`.
