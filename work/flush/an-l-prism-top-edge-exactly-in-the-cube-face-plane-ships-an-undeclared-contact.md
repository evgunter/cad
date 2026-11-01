---
id: an-l-prism-top-edge-exactly-in-the-cube-face-plane-ships-an-undeclared-contact
kind: issue
title: A cube ∪/∖ L-prism whose top edge lies exactly in the cube's face plane ships a body with an undeclared contact (tier 3′ red, pre-existing)
status: closed
branch: flush/corner-ruling-tier3
pr: 4650
opened: 2026-10-05
priority: P1
cost: M
refs: [3790, 6103]
closed: 2026-10-11
---



## What

Found by `join/pinch-uncrossed-residue` re-running PR 4038's review r2
battery (`crates/sweep/examples/r2_pinch_probes.rs` on
`join/pierce-pinch-families-review-r2`) on main `4cf3f8b9`. Identical
on PR 4038's frozen head (`review-r2/H-cube.txt.gz`), so it is older
than this lane and unmoved by it.

`Ltop fib0 edge psi=1.9`: the L-prism's reflex top corner `(1, 1, 1)`
on a cube edge, the cube along Fibonacci direction 0 of 120, so
`m = (0.1288, 0, 0.9917)` has no `y` part and the prism's top edge
`(1, 1, 1)–(1, 2, 1)` lies exactly in the cube's near face plane. Three
runs build with the oracle volume, tier 2, the certificate and a legal
operand, and fail tier 3′ (`validate_pseudomanifold` against the
result's own contacts), definite, not escalated:

- prism ∪ cube and cube ∪ prism: `UndeclaredContact { VertexOnFace }`
  at `(1, 2, 1)` and `UndeclaredContact { EdgeFaceOverlap }` at
  `(1, 1.5, 1)`;
- prism ∖ cube: `StaleContactDeclaration { VertexOnFace }`.

An operand edge lying in the partner's face is an undeclared
coincidence (D10's ground, held): the op should refuse it or carry its
contact, and here it does neither.

## The shape to give

Diagnose which lane let the edge-in-face coincidence through without a
refusal or a contact record, beside
`an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired`.
Repro: `R2P_T3=1 R2P_PICK=fib0 r2_pinch_probes cube Ltop`.

## Measured (2026-10-11, PR 4650)

The probe (`R2P_T3=1 R2P_PICK=fib0 r2_pinch_probes cube Ltop`, the
example copied from `join/pierce-pinch-families-review-r2`, unchanged)
on main's merge base `8c8f3b4c`: the three `edge psi=1.9` runs fail as
above (`pc U`, `cp U` `UndeclaredContact`; `pc S` `StaleContactDeclaration`).
On `flush/corner-ruling-tier3` all three are `SOUND`, and no other line
of the probe moves. On PR 4604's head `29f98d7a` the three fail as on
main: 4604 does not clear them.

The cause is PR 4650's row's
(`a-box-and-a-cylinder-through-its-corner-edge-intersect-past-tier-3-prime`):
the prism's top-edge ends rest on the cube's near face, the section
divides it, and the record was chased by key to the wrong piece or
dropped with it. The op does carry its contact; it carried it to the
wrong face. The `pc S` `StaleContactDeclaration` was this defect too,
not FUSE's.

Pinned: `join_pierce_runs_sweep.rs`
`an_edge_in_the_partners_face_plane_records_its_rests_where_the_result_keeps_them`
(the cube rebuilt exactly, `cube_at(V, frame(fib0, 1.9), (0, −2, 0))`,
plain booleans), red on the merge base in `ab U`, `ab S` and `ba U`.

## Closed (2026-10-11, PR 4650)

Fixed by PR 4650. A vertex-on-face record reads its divided face through `Descendants::holder`: the face's own pieces, asked by `boolean::contain::face_placement`, the one containment ask the census also uses. A piece it cannot read refuses `RestPieceUnread`. The PR had a dual review (holdout pair, `docs/dual-review-log/rows/PR-4650.md`), and its fix pass took the adjudicated union. The pins are in `a_ruling_through_a_box_corner_edge.rs` and `join_pierce_runs_sweep::an_edge_in_the_partners_face_plane_records_its_rests_where_the_result_keeps_them`. The residues are filed:
- FLUSH `an-apex-edge-in-the-partners-face-plane-fails-tier-3-prime-on-one-edge-face-overlap`;
- FUSE `an-edge-or-face-record-follows-its-faces-key-through-a-division`;
- FUSE `a-line-edge-in-a-curved-faces-interior-is-recorded-at-its-ends-only`;
- ORBIT `a-sphere-cut-in-divides-a-face-the-carry-reads-by-its-pre-cut-key`.
