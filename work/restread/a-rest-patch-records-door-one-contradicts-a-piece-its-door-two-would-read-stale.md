---
id: a-rest-patch-records-door-one-contradicts-a-piece-its-door-two-would-read-stale
kind: issue
title: A rest patch record's Door 1 contradicts a piece pair on aligned senses where Door 2 would read the pair stale, so a carry refuses an op that should land the record elsewhere
status: open
opened: 2026-10-19
priority: P3
refs: [a-two-carrier-face-pair-off-its-tangent-line-reads-contradicted-not-stale, the-census-confirm-pass-and-confinement-screen-escalate-a-residual-before-the-region]
---

Found by `contacthold/ff-divided-reads-stale`'s sweep (PR 5390), read
from the code at its merge base; not measured.

## What

`census::confirm_patch` (a face–face record on one carrier or two
planes) reads Door 1, `rest_door_one` → `boolean::rest_pair_reading`,
before Door 2, the region overlap. Door 1 contradicts aligned senses
(`ContactContradicted { Rest }`) and returns. Door 2 reads
`ChartOverlap::Empty` as stale. So a pair whose senses are aligned and
whose regions do not overlap reads contradicted, where it does not
meet at all.

At rest in 3′ both refuse the body. Through a carry they do not: when
an op divides a recorded face, `boolean::records::landings` confirms
the row on each pair of the faces' images
(`census::RecordConfirm::meets`), passes a stale pair over, and refuses
the op `CarryRefused { Found, Unread }` on any other finding. A piece
the op flipped onto the record's plane (a cavity floor, as `B ∖ A`
leaves the face block's floor in
`face_face_touch::a_record_whose_faces_the_op_divides_lands_on_the_pair_holding_its_line`)
that does not overlap its partner therefore refuses an op that should
land the record on the pair that does overlap.

PR 5390 fixed this shape in the two-carrier lane, where the line is now
placed in both faces (`boolean::rest_line_in_both_faces`) before the
`Tangent` table reads the sides.

## Owed

- A fixture that reaches it. No op's found row does: a coplanar rest
  pair glues or merges (plate + pad + a block across the plate's top,
  every op, carries nothing). A mate's rest record on a product body,
  carried through an op that flips a piece of one face, should.
- The fix is not a reorder. Door 2 takes Door 1's verdict (`Definite`
  or `Bridged`) to pick its chart, so Door 2 cannot run first as is.
  One shape: on a Door 1 contradiction from the senses alone (the
  carrier is one), read Door 2 on the structural chart and push stale
  where it reads `Empty`.


