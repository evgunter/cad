---
id: a-carried-edge-face-record-the-census-cannot-read-refuses-every-later-op
kind: issue
title: A union ships edge–face records its census cannot read, and the one carry refuses the same records in every later op, even one touching neither cell
status: closed
opened: 2026-10-13
priority: P1
design: true
branch: contacthold/carry-by-lineage
pr: 4996
closed: 2026-10-19
---


Found on PR 4921's review (`contacthold/face-face-touch-row`), MINOR 2.
That PR measured it and stopped: under the ratified text the two
obvious fixes each contradict a clause (below), so this is a design
question, not a fix.

## What

A rod (r 0.5) resting on a plate's side face along a ruling through
both faces' interiors, its vertices off the ruling. `plate ∪ rod` ships
two solids with three records: the face–face record of the plate's face
and the rod's wall, and an edge–face record for each cap circle, which
touches the plate's face at one point inside its arc. Those two are
the records C7 (`crates/topo/README.md`) bounds the face–face record
by. The union's own 3′ cannot read them (`CensusUnsupported` on the two
cap edges; RESTREAD's
`an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane`),
and the union ships them all the same.

The one carry (`records::landings`) puts every pair drawn from a
record's cells' images to the census's record confirm, kept cells
included, so the next op refuses on those records:

| op on `plate ∪ rod`, carrying | result |
|---|---|
| ∪ a far brick (`[10, 11]² × [0, 1]`), `carried()` | `CarryRefused { Carried(A), Unread { record: 0, cells: [cap edge, plate face] } }` |
| ∖ a hole in the plate (`[-1, -0.5]² × [-1, 2]`), `carried()` | the same |
| either, the face–face record only | answers (3 / 2 solids), the record landed; 3′ `CensusUndecidable` (RESTREAD's cross-solid row) |

Neither op touches the cap edges or the plate's y = 2 face. editor-core
hands every record to the next boolean (`eval/wire.rs`,
`ContactRecords::carried`), so in the editor every later boolean on
such a body dead-ends: `editor-core`
`face_face_touch_carry::a_later_op_on_a_rod_resting_on_a_plate_refuses_on_its_unread_cap_record`
pins it.

The same disagreement shows inside one op, on the reduction's own
records (row `Found`): with a block crossing the rod and another
crossing the plate's face, each op whose result divides a cap record's
cells refuses `CarryRefused { Found, Unread }` on it (∪ both orders,
B∖A; inside form: A∖B, ∩ both orders), and each op that leaves them
whole ships them unread (PR 4921's review probe
`review_4921_both_faces_divided`, branch
`contacthold/face-face-touch-row-review`).

## The fork

- **The minting op refuses** a record its census cannot confirm. D1
  says the result carries "machine-checkable contact records". But C7
  says the union is two solids carrying the face–face record bounded by
  these records, so the rod on a plate refuses again, as it did before
  PR 4921 (`TangentSlitArmUnbuilt`).
- **The carry lands a record whose cells it kept, by lineage alone.**
  Sound: kept cells have unchanged geometry, so the meeting the record's
  decision backed still holds. But D1's carry clause puts "each pair
  drawn from a record's two cells' images to the census's record
  confirm", and this would exempt kept pairs. It also leaves the
  divided-cell case (row `Found`) refusing.
- **Build the census lane** for a circle (or any curve) touching a face
  at a point, so the confirm reads these records. Both clauses then
  hold as written. RESTREAD's row already holds this, and it calls what
  the lane certifies "a design question".

## Weighed (2026-10-13)

A designer pair (fork log row 132) converged in two rounds: a point-touch reading in the census for any cell pair (C3); every door that mints a record reads it with the census's record confirm and refuses typed one no reading covers, the search for unrecorded touches staying parked (D1 tier 3); the carry lands a record whose cells the op kept or substituted by lineage, unread, and confirms only pairs drawn from a divided cell (D1's carry clause). `[ev]` PR for the D1/C3 text. Sequencing either way: the point-touch reading lands before the mint-time confirm, so the rod-on-plate union never refuses in between.

Ev on `[ev]` PR 4951 (2026-10-13): the point-touch reading and the carry by lineage, confirming only on division, as recommended; the mint-time per-record confirm is not built now but folds into un-parking the door's result gate (`work/reachhold/boolean-door-runs-the-census-over-its-result.md`), since its job there is "escalate or not". Owed: the lineage carry (D1's carry clause as PR 4951 left it). The point-touch reading is PR 4956.

## 2026-10-13 — an n-ary union's next fold step refuses it too (CONTACTHOLD)

Measured on `contacthold/fold-step-face-face-row`: the rod of
`face_face_touch` (z 0.25 to 0.75) on the plate, unioned through
`editor_core` with a block apart from both, members in the order plate,
rod, block, refuses `UnionFoldStep { member: 2, refusal:
CarryRefused { row: Carried(A), refusal: Unread { cells: [Edge, Face] } } }`:
the step folding in the block carries the first step's cap-circle
edge–face record and cannot read it. The fold-step witness of that
branch uses a rod of the plate's height instead, whose ends are
edge–edge records the carry reads.

## 2026-10-13 — the point-touch reading lands (`contacthold/point-touch-reading`)

The census now reads the cap circles' edge–face records as point
touches (`census::point`), so the table's first two rows answer: ∪ a
far brick and ∖ a hole in the plate, carrying `carried()`, keep all
three records (`editor-core`
`face_face_touch_carry::a_later_op_on_a_rod_resting_on_a_plate_carries_its_cap_records`);
the bridge carrying every record answers one solid, its face–face and
bottom cap records landed, 3′ Ok; a step dividing the bottom cap's
circle lands the record on the piece holding the touch point. 3′ still
reads the far ops' bodies `CensusUndecidable` (the cross-solid curved
search, RESTREAD's row), as with the face–face record alone.

The `Found` row case moved rather than cleared: the four ops that
refused on a cap record now refuse `CarryRefused { Found, Unread }` on a
face–face pair the two-carrier confirm reads `Contradicted`
(`a-two-carrier-face-pair-off-its-tangent-line-reads-contradicted-not-stale`).
The mint-time confirm and the lineage carry are not this unit (Ev on
`[ev]` PR 4951, above).
