---
id: a-box-and-a-cylinder-through-its-corner-edge-intersect-past-tier-3-prime
kind: issue
title: A box intersected with a cylinder whose wall passes through the box's corner edge builds at the right volume and fails tier 3′
status: closed
branch: flush/corner-ruling-tier3
pr: 4650
priority: P0
cost: M
opened: 2026-10-09
refs: [4399]
closed: 2026-10-11
---

Found by the dual review of the tube-on-a-ball unit (PR 4399, review r1, its N4), and present on base `b1a15ad0` before that PR.

## What

The box is `[0, 2]² × [0, 1]` (`sweep::test_support::brick`). The cylinder is the extrude of the circle about `(0.5, 0.5)` with radius `√0.5`, over `z ∈ [−1, 2]`. Its wall passes through the box's corner edge `x = y = 0`, a ruling of the wall, and both box faces at that edge cross the wall there.

- `A ∩ B` and `B ∩ A` build at the right volume, `π/4 + 0.5 = 1.285398163`.
- The built body passes tier 2 and the certificate.
- It fails tier 3′: `topo::validate_pseudomanifold` refuses it against its own contact records.

A body the boolean ships fails a tier the door gates on, so this is a live wrong answer.

## Repro

Review r1's `tube_on_ball_r1_family_line_edges`, the pose `"line/crossing faces"`, on branch `join/tube-ending-on-a-ball-review-r1` (`crates/sweep/tests/tube_on_ball_review_r1_probes.rs`). It is an ignored row: run it with `--ignored --nocapture` and read `outcome`'s `t3p=false`.

## What the taker owes

Find which contact or edge the census reads as non-manifold at the corner ruling (a line edge of the box lying in the wall, met by both box faces), and decide whether the body or the census is wrong.

## Measured

On main `19b704d9`, with the plain booleans (`sweep/tests/a_ruling_through_a_box_corner_edge.rs`):
the census reads nothing as non-manifold or undeclared. Tier 3′ fails in
both orders of `∩` on two `StaleContactDeclaration { VertexOnFace }`:
the box's corner edge's two ends, recorded as resting on the wall face,
against a wall face that holds neither (`curved_face_containment` reads
both `Out`). `∪` and both `∖` pass tier 3′, but wall `∖` box is wrong
the other way: its corner vertices lie `In` a live wall face and the
result carries no record (the census cannot see it: same-solid curved
pairs are the constructor's obligation, `census.rs` module docs).

Across the corner ruling along each of the box's twelve edges (all six
ops, 72 cases), 18 fail on main: six edges each fail `w ∩ b`, `b ∩ w`
(stale) and `w ∖ b` (rests dropped). The other six edges pass by key
order alone.

The body is right; its records are wrong. The rest is decided at the
reduction against the whole wall face. The section divides the face,
the division keeps the face's key on one piece, and `carry` chased the
record's face by key alone (`boolean::ops::carry_rows`, the
vertex-on-face loop through `Descendants::live`). Where the key landed
on a piece the vertex is not on, `∩` named that piece (stale) and
wall `∖` box lost the record with the discarded key piece.

## Fix

`Descendants` holds each operand's face divisions (the join's chord-mef
fragments and the pinch welds', in clone keys), and a vertex-on-face
record reads its face through `Descendants::holder`: on a divided face,
the live piece the vertex bounds, else the one whose region holds it,
asked as the census confirms the record (`contfp`, or the chart trim);
no holding piece drops the record. An undivided face reads as before.
The cone sibling refuses `CurvedPierceUnsupported` before any record is
carried: evidence added to ROOTS' `a-line-parallel-to-a-cone-generator-has-no-root-lane`.

## Closed (2026-10-11, PR 4650)

Fixed by PR 4650. A vertex-on-face record reads its divided face through `Descendants::holder`: the face's own pieces, asked by `boolean::contain::face_placement`, the one containment ask the census also uses. A piece it cannot read refuses `RestPieceUnread`. The PR had a dual review (holdout pair, `docs/dual-review-log/rows/PR-4650.md`), and its fix pass took the adjudicated union. The pins are in `a_ruling_through_a_box_corner_edge.rs` and `join_pierce_runs_sweep::an_edge_in_the_partners_face_plane_records_its_rests_where_the_result_keeps_them`. The residues are filed:
- FLUSH `an-apex-edge-in-the-partners-face-plane-fails-tier-3-prime-on-one-edge-face-overlap`;
- FUSE `an-edge-or-face-record-follows-its-faces-key-through-a-division`;
- FUSE `a-line-edge-in-a-curved-faces-interior-is-recorded-at-its-ends-only`;
- ORBIT `a-sphere-cut-in-divides-a-face-the-carry-reads-by-its-pre-cut-key`.
