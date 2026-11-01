---
id: a-corner-pair-with-an-edge-in-the-partners-face-plane-builds-with-undeclared-contacts
kind: issue
title: A corner pair with an edge in the partner's face plane builds a body tier 3' rejects (UndeclaredContact, StaleContactDeclaration)
status: closed
branch: flush/corner-ruling-tier3
pr: 4650
opened: 2026-10-05
priority: P0
cost: M
closed: 2026-10-11
---


## What

Found by PR 4061's review (NOTE 3). It is pre-existing: the lines are
identical on main.

In `corner_pairs_battery`, 287 runs build a body with the right volume
that `validate_pseudomanifold` rejects. All 287 are at exact-tie turns:
98 at ψ = 0, 34 at π/2 and 155 at π. By pair: w343-w330 93, w300-w270
107, w350-w200 87.

**The failures are definite, not census-escalated.** No error carries
an `Indeterminate`. The kinds, read per run, are:
- `StaleContactDeclaration` alone: 128;
- two `UndeclaredContact` (a `VertexOnFace` and an `EdgeFaceOverlap`):
  91;
- both of the above: 68.

**Each pose holds an exact coincidence: an edge in the partner's face
plane**, not a face in a face plane. At ψ = 0 or π the frame's `u` is
horizontal, and at ψ = π/2 its `w` is. So B's corner edge along that
axis lies exactly in A's top-face plane z = 1, and the witnesses read
z = 1.0 exactly. Example, `w343-w330 i=0 j=1 psi=0 ab U`:
- `UndeclaredContact { VertexOnFace, witness (-0.385, 1.963, 1.0) }`;
- `UndeclaredContact { EdgeFaceOverlap, witness (-0.193, 0.981, 1.0) }`.

The other two ops at that pose pass tier 3′. This is undeclared-
coincidence ground, so the row waits on D10.

## Repro

`cargo test -p sweep --release --test all corner_pairs_battery --
--ignored --nocapture | grep t3p=false`. Read each body's
`topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol)` errors
for the kinds.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: the volume is right; the fault is tier 3′ UndeclaredContact/StaleContactDeclaration on the contact records, which stage 4 replaces with one-door records and unproven-coincidence findings. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Also in `join1_delta_arc_battery` (2026-10-08)

Measured on main `047d10d5` and on `join/battery-hygiene`, unmoved
between them: 6 lines `ARC tri {half,shallow,tri} d=(0.0, 0.5)
zb=(0.5, 1.5) decl={false,true} U` build two solids at the volume,
with tier 2, the certificate and a legal operand. They fail tier 3′
on one `UndeclaredContact { EdgeFaceOverlap }`. The triangle's apex
edge `x = 0, y = 0.5` lies in the partner's face plane `y = 0.5`
(witness `(0.0, 0.5, 1.0)`).

## Released by INTENT stage 4 E (`intent/s4-e-glue-on-zero`) (2026-10-09)

E does not reach this. The coincidence is an EDGE lying in the partner's face plane, not a face pair, and E's glue door records face pairs only (`crates/topo/src/boolean/glue.rs:40`). The `UndeclaredContact { VertexOnFace / EdgeFaceOverlap }` findings come from the census (`crates/topo/src/census.rs:1388` and its siblings), which E leaves as it was. `StaleContactDeclaration` is FUSE's `a-boolean-result-ships-contact-records-its-geometry-no-longer-confirms` (stage 4 F). Not re-measured on E. `join1_delta_arc_battery` already answered `decl=false` and `decl=true` alike, so a declaration was never the lever.

## Measured (2026-10-11, PR 4650)

`corner_pairs_battery`'s outcome lines, sorted and diffed whole
(16380 lines each, beside `join1_delta_arc_battery`'s 7350, which were
diffed in the same pass), on main's merge base `8c8f3b4c` and on
`flush/corner-ruling-tier3`: exactly 287 lines move, all from
`t3p=false` to `OK SOUND`, by pair w300-w270 107, w343-w330 93,
w350-w200 87 (this row's counts), and no other line moves. On PR 4604's
head `29f98d7a` the failing set is main's, line for line: 4604 does not
clear it.

The cause is PR 4650's row's
(`a-box-and-a-cylinder-through-its-corner-edge-intersect-past-tier-3-prime`):
the edge's end rests on the partner's face, the section divides that
face, and `carry_rows` chased the record's face by key to a piece that
does not hold the vertex, which reads `StaleContactDeclaration`, or
lost it with a discarded key piece, which reads `UndeclaredContact
{ VertexOnFace }` plus the `EdgeFaceOverlap` its bounds no longer back.
`Descendants::holder` lands it on the holding piece. So this row's
`StaleContactDeclaration` runs were this defect, not FUSE's
`a-boolean-result-ships-contact-records-its-geometry-no-longer-confirms`.

Pinned: `join_pierce_runs_sweep.rs`
`an_edge_in_the_partners_face_plane_records_its_rests_where_the_result_keeps_them`
(`i=0 j=1 psi=0` of each pair, plain booleans), red on the merge base
in `ab U` of each pair and `ba U` of w350-w200.

The `join1_delta_arc_battery` lines above do not move under PR 4650 or
under 4604: a different cause, split out as
`an-apex-edge-in-the-partners-face-plane-fails-tier-3-prime-on-one-edge-face-overlap`.

## Closed (2026-10-11, PR 4650)

Fixed by PR 4650. A vertex-on-face record reads its divided face through `Descendants::holder`: the face's own pieces, asked by `boolean::contain::face_placement`, the one containment ask the census also uses. A piece it cannot read refuses `RestPieceUnread`. The PR had a dual review (holdout pair, `docs/dual-review-log/rows/PR-4650.md`), and its fix pass took the adjudicated union. The pins are in `a_ruling_through_a_box_corner_edge.rs` and `join_pierce_runs_sweep::an_edge_in_the_partners_face_plane_records_its_rests_where_the_result_keeps_them`. The residues are filed:
- FLUSH `an-apex-edge-in-the-partners-face-plane-fails-tier-3-prime-on-one-edge-face-overlap`;
- FUSE `an-edge-or-face-record-follows-its-faces-key-through-a-division`;
- FUSE `a-line-edge-in-a-curved-faces-interior-is-recorded-at-its-ends-only`;
- ORBIT `a-sphere-cut-in-divides-a-face-the-carry-reads-by-its-pre-cut-key`.
