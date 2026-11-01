---
id: the-parting-reads-no-edge-lying-in-a-face-no-record-names
kind: issue
title: The Boolean's parting reads an edge lying in a face only where a record or its ends name the pair, so an unrecorded rider keeps one edge
status: parked
opened: 2026-10-26
priority: P1
cost: H
blocked_on: [an-edge-lying-in-its-own-operands-face-reaches-the-parting-unrecorded]
---



Filed by the CLEAVE lane on `cleave/riders-read-every-edge-in-a-face`
(PR 5793), a residue its rows disclose. P1: D1-wrong topology shipped
silently. Unmeasured: no fixture is known where the join leaves such an
edge without a record.

## What

`topo::boolean::slit::riders` asks a face of an edge only where
something names the pair: both ends lying on the face (by structure, a
v-f, v-v or v-e record), an e-f record, or a record of a crossing with
the face's loops. An edge whose interior lies in a face with neither
end on it and no record of the touch (D1 owes one: "every pair whose
interiors meet is recorded") is never asked, and keeps one edge even
where both its faces go into the face's material. The record producers
mint the e-f record for a line between two recorded bounds in a plane
face (`boolean::ops::record_spans`) and none for a curved face
(`work/contacthold/a-line-edge-in-a-curved-faces-interior-is-recorded-at-its-ends-only.md`),
so the gap is the producers' as much as the reading's.

## Fix

Either let the reading find such an edge without a record (every edge
against every face its box meets, at the cost of the reads), or close
it at the producers so that every edge lying in a face carries its
e-f record, and assert that in the result gate.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). PARTING holds the parting's reading of edges lying in faces and the cuts on spline faces; id and body unchanged. (CLEAVE orchestrator)

## Design (2026-11-01)

Weighed by two designers (`docs/prompts/designer.md`), who agreed on
the final state; it implements D1's record sentence and
`[coincidence.door]` as ratified, so it is not put to Ev.

- **The premise, corrected.** No mechanism is missing for NAMING the
  pair on a line in a plane: an edge lying in a face with neither end on
  it crosses the face's loops, which leaves a vertex or a point record
  the parting already reads. The one measured shipping instance is a
  closed edge (a slab less a revolved triangle whose ridge circle lies
  in the slab's top face: one closed edge, one vertex-face record, the
  census passes), which is
  `a-closed-edge-lying-in-a-face-is-never-read-by-the-parting`.
- **What the row is evidence of:** one question (does this edge lie in
  this face, and which pieces) answered five ways: the sweep decides it
  and discards it (`LyingIn::Plane`, `CurvedEvent::Lies`);
  `ops::record_spans` rebuilds it for lines in planes; `slit::riders` and
  `holding` rebuild it from records, `carrier_on_surface` and midpoint
  containment; the census discovers it (lines only) and confirms it.
- **Final state:** the sweep records an edge-face record per piece it
  decides lies in a face (`edge-in-face-spans-are-re-derived-…`, this
  row's blocker); the parting asks a face of an edge exactly where an
  edge-face record names the pair, and keeps only its own question,
  whether both walls go into the face's material (`walls_inside`);
  `Names`, `crossed_faces` and the geometric reading in `holding` go.
  An assertion goes red on a missed parting
  (`tier-3-reads-no-face-through-an-edges-wedge-within-a-solid`).
- **Rejected:** a scan in the parting (every edge against every face
  its box meets) is a further reading of a coincidence from values, and
  parts silently what no record backs.
- **Order:** planes first; curved faces keep their geometric reading
  until the curve-touch lane lands
  (`work/contacthold/an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane.md`).
  (PARTING orchestrator)

## 2026-11-01 — the blocker's residue in `ops` (PARTING, `parting/sweep-records-edge-in-face`)

With `record_spans` gone, `ops::piece_reading`'s only caller is
`slit::holding`, so `ops::DoorInBand` has one variant left
(`Containment`) and `PieceRead::Unread`'s refusal is read by no one
(an `expect(dead_code)` marks it). Both go with the geometric reading
in `holding` that this row's parting half retires; that unit drops the
parameter and the payload in `slit.rs`, which the blocker could not
touch while `parting/closed-and-torus-riders` held it.

## Opened (2026-11-01)

Its blocker closed with PR 6225: the sweep now records every line piece lying in a plane face. Dispatchable for plane faces: the parting asks a plane face of an edge exactly where an edge-face record names the pair, and `Names`, `crossed_faces` and the geometric reading in `holding` go for that case; curved faces keep their geometric reading until the curve-touch lane lands. (PARTING orchestrator)

## Parked again (2026-11-01)

The lane's measurement before rewiring found 10 result cells
(`a_vertex_read_again_classes_every_edge`, `tipped_b`) where a line
lying in a plane face of its own operand reaches the parting with no
record and both walls go into the face's material: the geometric
reading parts it, a records-only reading would ship one edge that tier 3
passes. Parked on `an-edge-lying-in-its-own-operands-face-reaches-the-parting-unrecorded`. The rewiring is built and kept on
`parting/parting-reads-records-wip` (`28a969c583`, clippy clean, suites
not run): plane faces read records only; `divide` carries the edge's
edge-face record to each piece the face holds (it dropped it before: 31
reads); `ops::DoorInBand` and `PieceRead::Unread`'s payload go. (PARTING orchestrator)
