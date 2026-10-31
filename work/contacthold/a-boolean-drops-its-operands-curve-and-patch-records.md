---
id: a-boolean-drops-its-operands-curve-and-patch-records
kind: issue
title: A boolean drops its operands' curve and patch records: only vertex and edge granularity carries
status: open
opened: 2026-10-11
priority: P2
cost: M
---

## What

Stage 4 F carries an operand's own records into every pair boolean and
every union fold step (`a-boolean-drops-its-operands-own-contact-records`,
closed). It carries the vertex and edge granularities alone:
`ContactRecords::carried()` (crates/topo/src/boolean/mod.rs:851) builds
`OperandRecords` from `vv`, the vertex-on-face rows, `ve` and `ee`, and
its doc says a face-granularity record "has no carried kind and stays
here". The op remaps only the curves its own reduction discovered
(crates/topo/src/boolean/ops.rs:3715).

So a curve or patch record an operand holds — a tangent
`CurveContact` a union's fold step k decided, or one a pair boolean's
operand carries — does not reach the result once that body re-enters an
op: after step k + 1 the union's records lack it.

Found by stage 4 F's review (M3). No witness is built yet: the corpus
scenes that passed 3′ on F's carry hold vertex and edge contacts only.

## Owed

- A witness: a union of three members where two touch along a tangent
  curve and the third folds in after them, and its 3′ verdict.
- The carried kinds for curve and patch records, under `remap_carried`'s
  drop rules (a curve record survives where its curve's entities do).

## 2026-10-11 — an edge-face span the op now mints, dropped on carry (FLUSH, PR 4737)

PR 4737 makes the boolean record a line edge's span in a plane face
(`boolean::ops::record_spans`). `ContactRecords::carried` drops that
record like any edge–face one, so the next op never sees it. The
review probe battery `sweep` `review_4737_span_probes::review_4737_span_battery`
(ignored, release) unites a triangle prism with a brick whose face
`y = 0.5` holds the prism's apex edge, then cuts the union with a plate
across the contact, carrying its records:

```
CARRY ang=0 whole_in_face plate I x => records=[0, 0, 0, 0, 0, 0]
  t3p=ERR [UnrecordedContact VertexOnEdge (0.0, 0.5, 1.123),
           UnrecordedContact VertexOnEdge (0.0, 0.5, 0.8),
           UnrecordedContact EdgeFaceOverlap (0.0, 0.5, 0.9615), ...]
```

18 of the battery's `plate I x` lines refuse 3′ this way, at all three
turns. The lines are identical on PR 4737's pre-fix head (`dfe5ea10`):
the carry drops the face span, and the cut's new ends on the apex edge
have nothing to inherit. Carrying an edge–face record through the cut's
lineage (onto the pieces whose interiors still meet the face, as D1
rules for an edge–edge one) is this row's owed step for that kind.

## 2026-10-12 — the carried kinds exist; two witnesses still owed (CONTACTHOLD)

`contacthold/face-face-touch-row` builds the carried kinds:
`OperandRecords::ef` and `::ff` (`EfContact`, `FfContact`), filed by
`ContactRecords::carried`, validated at the op door
(`validate_declarations`), carried through the one carry
(`ops::carried_rows`, `records::landings`) and filed back by
`ops::Rows::push`, which no longer refuses an edge–face or face–face
landing. A record whose census confirm has no lane now refuses the op
(`CarryRefused { Unread }`) instead of vanishing.

Measured on that branch:

- A face–face record lands through a later op: `sweep`
  `face_face_touch::a_rod_on_a_plate_records_its_touch_and_a_bridge_lands_it`
  (rod on a plate, then a bridging union carrying the record).
- `review_4737_span_battery`'s `plate I x` lines: the apex edge's span
  record now lands (`records=[0, 0, 0, 0, 1, 0]` where it was all
  zeros), but the same 18 lines still refuse 3′, on the cut's new ends
  on the apex edge (`UnrecordedContact VertexOnEdge`) and on one piece's
  span (`UnrecordedContact EdgeFaceOverlap` at `(0.0, 0.5, 0.9615)`).
  That is the carry implying the records at a cut's new ends from a
  landed span, which this row still owes.

Still owed: that, and the three-member fold witness, which waits on a
fold step minting the face–face row at all
(`work/contacthold/a-union-fold-step-mints-no-face-face-row-for-a-given-tangency.md`).

## 2026-10-13: a carried edge-face or face-face record lands through the one carry (FUSE)

FUSE closed `an-edge-or-face-record-follows-its-faces-key-through-a-division`
as no path for the reduction's own records. Their divided faces already
go through `boolean::ops::place_found` (PR 4820). An operand's carried
edge-face and face-face records still have no carried kind, so that
half stays here. Once this row gives them one, they must land as
`boolean::ops::carried` lands the carried kinds today: through
`operand_fate`, then `records::landings`, which puts a record on every
pair of a divided face's pieces and chords that the census's record
confirm reads as meeting. Do not chase their face by key through
`Descendants::live`: a division keeps the key on one piece, whichever
it is.

## Re-homed to CONTACTHOLD and parked (2026-10-17, WIRE triage)

The title is overtaken: `ContactRecords::carried` now files all six `ContactKind`s, `OperandRecords::ef`/`ff` included. What remains is (a) the records implied at a cut's new ends from a landed span, and the `review_4737` lines refusing tier-3′ `UnrecordedContact`, and (b) the three-member witness, which waits on CONTACTHOLD's `a-union-fold-step-mints-no-face-face-row-for-a-given-tangency`. Every file it touches is topo's (`boolean/records.rs`, `ops.rs`), so it moves to the program holding the carry rows. Stage 5 makes the at-rest census a check, so (a)'s refusals become findings; parked on `the-at-rest-census-is-a-check`. Nothing else here was decided for the receiver.

Signed (WIRE orchestrator).

## 2026-10-26 — opened: its trigger fired, and (a) stands

Stage 5 C (`the-at-rest-census-is-a-check`) made the editor's at-rest
census a check. The refusals in (a) are not the editor's at-rest gate:
`review_4737_span_battery` calls the kernel's tier-3′ door
(`topo::validate_pseudomanifold`) on a boolean's result, which C leaves as
it was. So (a) and (b) both stand, and the row is open.
