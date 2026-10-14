---
id: revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet
kind: issue
title: Body::revert refuses a body with coincident edges on one line: build the re-pairing the complement's material sectors decide
status: open
opened: 2026-10-14
priority: P2
cost: H
blocked_on: [4795]
refs: [reverting-a-slit-body-overlaps-its-coincident-edges-wedges-and-passes-every-tier]
---

## What

`Body::revert` (`crates/topo/src/revert.rs`, `coincident_edges`)
refuses `RevertError::CoincidentEdges` on any body where two distinct
edges run between the same two point keys on one line or one stored
curve. ∖ passes the refusal on as `BooleanError::Revert` (the seamed
finish, `boolean/finish.rs`; the single-operand fallback,
`boolean/ops.rs` `finish_fallback`; the void door,
`boolean/voids.rs` `insert_hollow_voids`, as
`VoidInsertError::Revert`). That is a construction gap on valid input:
the split's one-end slit is a legal tier-3′ body, and subtracting it
where its slit is kept refuses.

## Owed

The re-pairing, as an Euler-operator construction inside `revert`:
where 2k sheets meet on a line, pair them by the complement's material
sectors. The slit (PR 4622's one-end form, pinned in
`crates/topo/tests/a_doubled_line_does_not_revert.rs`) worked in
closed form:

- Sheets round the keel: floor either side, wall a, wall b. The slit
  body's material sectors are (floor, wall a) and (wall b, floor),
  each `atan2(2, 0.6)` ≈ 73.3°; its two edges a and b bound them.
- The complement's sectors are (wall a, wall b), `2·atan2(0.6, 2)` ≈
  33.4°, and the half-space under the floor, π. So the complement has
  one crease edge wall a | wall b and the floor running flat across
  the line: the glued tip, an edge lying in a face.
- The construction: drop the slit from the floor's loop, join a and b
  into one edge between the walls, and join the open end's two vertex
  copies (one point key) into one vertex, whose link then crosses the
  keel's line twice — once along the new edge, once through the floor.
- The result carries an edge-on-face contact (the new edge in the
  floor), which `revert`'s signature cannot hand back today: it returns
  a body and no records. The door that reverts owes the record.

Two kissing cusps (D1's doubled cusp) re-pair as two seams; see the
`[ev]` PR 4795 body, report A of round 2, and its question (A) on D1's
"legal together or not at all".

Blocked on 4795: the pairing rule is that PR's principle, and a ruling
for radial edges would change what the complement is.

## What today's refusal does not read

The refusal reads point keys and curve keys only, so these reach the
naive map unrefused (none measured):
- coincident edges whose ends are distinct point keys a vertex–vertex
  record joins, and partial overlaps backed by vertex–edge records:
  `revert` sees no records;
- coincident curved edges on two curve keys, e.g. PR 4622's curved
  keel slit if the split mints its two arcs on two keys;
- an edge lying in a face and a face–face line touch (measured for the
  edge in a face: `subtracting-a-glued-tip-reverts-its-edge-on-face-line-into-the-rider-form`).

The re-pairing retires the refusal and these with it.
