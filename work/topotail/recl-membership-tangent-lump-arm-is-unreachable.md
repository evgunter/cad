---
id: recl-membership-tangent-lump-arm-is-unreachable
kind: issue
title: topo: recl's membership reads a declared-Tangent flank pair's lump, but the tangent-flank short-circuit has already skipped every such pair
status: closed
opened: 2026-09-30
closed: 2026-10-12
---


(TOPO, PR 3513's third fix pass, found while constructing the
membership tie's declaration read.)

## What

`boolean::recl::resolve_edge_edge` runs the wedge membership only when
no flanking pair is declared `Tangent` (`tangent_flank`, over the four
combinations of `(fa_s | fa_e) × (fb_s | fb_e)`). Inside the membership,
the `SideCode::On` arm reads the pair `(own_secs[own_idx],
other_secs[oi])` and, where that pair is declared `Tangent`, lumps it
through `sectors::tangent_lump`. `own_idx` is one of A's flankers and
`oi` one of B's, so that pair is one of the four `tangent_flank` read:
the `Tangent` lump arm cannot be reached, and every pair reaching the
lump takes `require_same`.

## Repair shape

Remove the arm, or say why the short-circuit and the arm must both
stand (a flanking set the two readings would disagree on), with a row.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: tangent_flank and the lump arm read declared-Tangent flank pairs in recl::resolve_edge_edge, declared-pair machinery stage 4 retires. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Re-homed at TOPO's cut (2026-10-11)

Moved from TOPO to TOPOTAIL when TOPO dropped to P2 and split on its priority seam (`work/topotail/log.md`). TOPO kept the Euler surgery doors; this row is a torn-record, refusal-recourse or code-hygiene row, or a parked boolean or placement row.

## Released by stage 4 F (2026-10-11)

Stage 4 F (`declared-pairs-retire`) retired every boolean declaration: no author can declare a pair, the glue is what the margins decide, and the declare offer, the flush detector and the carried declarations are gone. What this row waited on has fired; re-read it against that tree.

## Closed (2026-10-12)

`contacthold/same-side-tangent-touch` (PR 4781) deletes the
`tangent_flank` short-circuit, so the membership's `Tangent` lump arm
is now the path that decides a verified `Tangent` flank pair at an
edge–edge site. Rows that reach it, measured with a probe in that arm:
`wedge_end_doors::two_discs_kissing_externally_touch_at_every_op`,
`verbs_cylcyl_r1_review_probes::externally_tangent_walls_touch_at_every_op`,
`review_4704_flush_round_oracle::a_same_side_tangent_touch_answers_every_op`
and `review_4781_r1_probes::review_4781_touch_pairs_every_op`.
