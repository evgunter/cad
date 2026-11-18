---
id: a-finished-body-holds-its-contact-records-at-every-door
kind: unit
title: AtRestBody holds its contact records; every door carries its operand's records through its own lineage and gates each finished body it returns at tier 3′
status: open
opened: 2026-10-28
priority: P1
cost: X
points: 16
---


Ratified by Ev on PR 5967 (2026-10-28), decision 1. The text is in `crates/topo/README.md` tier 3, finished-body paragraph.

## What

`AtRestBody` becomes a body together with its contact records, minted only by a gate that ran tier 3′ against those records. Every door that takes a finished body does three things itself:
- carries its operand's records through its own lineage;
- adds the records of the coincidences it decided;
- gates each finished body it returns.

What retires:
- Caller-side `carry_contacts`: `SplitResult::carry_contacts`, `Shelled::carry_contacts`, `MergeCoplanarOutcome::carry_contacts`, and replace-face's and blend's carries.
- `BooleanInputs` and `BooleanBody::contacts`, which fold into the operands and the result.
- Record drops that become unrepresentable:
  - the shorthand `union`/`subtract`/`intersect(a, b, tol)`, which today passes default inputs;
  - `wire_split`'s empty `Carry.contacts` (WIRE's `a-split-in-the-document-drops-its-operands-contact-records`).

The Boolean cannot hold a tier-3′ verdict until REACHHOLD's cross-solid curved lane lands (`work/reachhold/boolean-door-runs-the-census-over-its-result.md`). Until then its result names the bar it paid (tier 3, or 3′) rather than letting one verdict mean both.

## Price

X, 16 points. It touches every door and editor-core, roughly two H rows. Review tier: dual. It changes a type every door reads, and changing it later would be hard.

## Done when

- Every door that returns a finished body returns one that holds its records.
- No caller-side carry remains.
- The split's door gates each side at tier 3′ against what it carried, measured on the A-cause tests in the tier-3′ row's measurement (`divided_cell_carry`, `one_record_carry`, `review_4730_divided_carry_probes`, `census::split_wedge_tests`, `a_carried_record_no_reading_covers`).

The gate stays at tier 3 for the split's other causes until this track's later rows land.

## Note from WIRE (2026-11-01, PR 6268)

The document's interim split carry (`wire_split`, through
`topo::SplitResult::carry_contacts_across`) retires a record the cut
parts onto the two sides, and only where each of its cells has an image
on the cut (a face its side holds of the plane, or a cell bounding one);
a parted record off the cut refuses `Parted`. The default
`carry_contacts` still refuses every parted record. When the split's
door carries its own records, it adopts this policy or replaces it
deliberately; it is not settled by this unit's text.

## Built and measured (2026-11-18, `embed/records`, for EMBED's gate)

**What landed.**
- `AtRestBody` holds body, records and verdict, fields private;
  `Replay` (a dual's finished type, a box scalar's leaf) holds its
  records too. `FinishedBody::records` is the read.
- `finish` takes `Built` (a body and the records its door decided; a
  bare body converts, touching nothing). A `⊔`'s gate is
  `finish_assembled(Assembled)`: tier 3, then the census as the
  deciding operation between arrivals, the records it mints held on
  the finished body, its rows and findings handed back (`Censused`).
  Both end in the one private constructor pair in `validate.rs`.
- The gate certifies every record it is handed: stale or contradicted
  refuses at that door. It raises no unbacked meeting, and runs no
  census where there is no record. `AtRestBody::bar` names which half
  was paid (`Bar::RecordsConfirmed`, a `⊔`'s `Bar::ArrivalsDecided`).
- Doors: the Boolean's gate hands the records it decided; the plain
  `union`/`subtract`/`intersect` carry their operands' own in
  (`BooleanInputs::of`); split, shell and blend carry
  `operand.records()` through their own lineage (new
  `SplitFinishError::Carry`, `ShellError::Carry`, `BlendError::Carry`;
  the evaluator maps all three to `ContactCarry`). The split adopts the
  document's policy (`carry_contacts_across`: a record the cut parts,
  each cell with an image on the cut, retires).
- Retired: `SplitResult::carry_contacts[_across]`, `SplitContacts`,
  `Shelled::carry_contacts`, `Blended::carry_contacts`, the
  `BooleanBody.contacts` field (a method reading the body),
  `StepImport::Solid.contacts`, `Product.contacts` (a method), the
  Python `Body`'s field, and the evaluator's `Carry.contacts`: `Carry`
  holds `Citing` (how the body's records cite at this value) and no
  cell of any record.
- The assemble node's census is eager, in the graft's finish; the lazy
  `product::Census` reads that body and its rows and no longer
  re-grafts or re-finishes. The laziness was a lane's choice
  (`520e0c8c72`, INTENT's D′3 fix pass, 2026-10-26), not a ruling.

**Against this row's text, not done.**
- `MergeCoplanarOutcome::carry_contacts` and the face-offset verbs'
  carries stay caller-side: those doors return construction state, not
  a finished body, so there is no gate for a door-side carry to feed.
- `BooleanInputs` and `MemberInputs` still carry operand records into
  the Boolean beside the operand. The plain doors fill them from the
  body, so the shorthand's drop is gone; `boolean_op_with` handed fewer
  records than its operand holds still drops them.
- The split's side gate confirms what it carried and does not yet
  raise an unbacked meeting: the flip is
  `the-gate-raises-no-unbacked-meeting-until-the-census-reads-what-doors-build`.
  The A-cause tests (`divided_cell_carry`, `one_record_carry`,
  `review_4730_divided_carry_probes`, `census::split_wedge_tests`,
  `a_carried_record_no_reading_covers`) pass with the door's carry.

**The price** (one box, CI's build, `ci` profile, default ε; 17,185
tests before, 17,197 after).

| | wall | test time | census calls | census s |
|---|---|---|---|---|
| main `763d6fe0` | 144.9 s | 576 s | 6,835 | 12.5 |
| full tier 3′ in every finish (not built) | 159.7 s | 663 s | 58,711 | 54.9 |
| records confirmed + unbacked in-solid refused (not built) | 162.0 s | 657 s | 59,672 | 59.6 |
| as landed | 142.4 s | 567 s | 9,341 | 21.5 |

As landed, by finish site (calls / s): assemble graft 2,679 / 10.2;
Boolean gate 847 / 3.2; product 2,450 / 0.56 (was 3,561 / 4.5: it
reads the graft's body); import aggregate 370 / 0.28; distribute graft
22 / 0.11; split 29 / 0.10; blend 15 / 0.10; shell 5 / 0.06; sweep
leaves 10 / 0.03; bare finishes 6 / 0.02; direct tier-3′ calls by
tests 2,908 / 6.8. Wall and test time are within the run-to-run spread
of this box (a later unmetered run of the same tree: 165 s, 658 s).
The eager graft census is the one new cost: 10.2 s over 2,679 calls.

**Verdicts that moved**, each with its test:
- A record with no touch behind it refuses at the gate:
  `one_record_carry::a_record_lands_by_lineage_and_is_stale_only_on_a_divided_cell`,
  `…::a_carried_record_with_no_touch_behind_it_refuses_at_the_booleans_gate`
  (was `…lands_unread`: the union built holding it),
  `surgery_carries_records::a_record_between_faces_that_do_not_meet_refuses_at_the_gate`,
  `validate::door_gates_read_carried_records` (split, shell), and
  sweep's `contained_flush_cylinder`, `blend_carries_records`,
  `review_4996_blend_cap` rows.
- `face_face_touch::a_bore_tangent_from_inside_refuses_on_the_tangent_record_its_cut_wrote`
  and `the_void_and_material_pinches_read_alike_at_every_pose`: the
  plate less a rod tangent from inside built and shipped missing its
  cusp edges; it refuses `ContactContradicted` on the record the cut
  wrote
  (`work/touchrec/a-cut-by-a-rod-tangent-from-inside-records-its-material-crescent-as-a-tangent-contact.md`).
  `a_solid_missing_its_cusp_edges_leaves_a_solid_in_its_bore_cleared`
  is deleted (its fixture is that cut).
- `split_along_a_pinch_face`: a plane along a face through a union
  pinch refuses `Carry { Parted }` at the kernel door, where the kernel
  dropped the record silently and the document already refused
  (`work/wirehold/a-record-the-split-parts-has-no-consumption-rule.md`).
- `a_vertex_read_by_two_sector_passes::a_touching_vertex_along_two_arches_shared_ray_*`:
  tier 3′ holds in 4 results of the first pyramid, was 1 (the plain
  doors carry the arches' records).
- `product_finishes_once`: a gather over mapped sources runs 2
  finishes, was 3; `corpus_finish_count`: 655, was 662.

**The three ATREST rows.**
- `a-boolean-result-reused-without-its-records-…`: answered; the
  records ride the operand and the plain doors carry them.
- `a-boolean-reading-two-bodies-of-one-arrival-…`: remains (the record
  is on the `⊔`'s body, not on either pick).
- `a-boolean-over-an-instance-drops-its-parts-census-findings`:
  remains whole (findings, not records).
