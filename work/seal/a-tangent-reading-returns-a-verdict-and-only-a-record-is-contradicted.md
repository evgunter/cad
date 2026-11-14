---
id: a-tangent-reading-returns-a-verdict-and-only-a-record-is-contradicted
kind: unit
title: the tangent table and the Rest door return typed verdicts; only a record read at rest is contradicted; the glue door decides a tangency once
status: closed
opened: 2026-11-09
priority: P1
cost: H
closed: 2026-11-09
pr: 6673
parent: glue-disagreements-and-held-contradictions-after-stage-4
---


Step 3 of the parent's design (fork `glue-disagreements-carry`, 2026-11-09).

## What

"Contradicted" is vocabulary from before stage 4. Once declarations were gone, the glue door claims nothing, so a reading's definite "no" is a verdict. A contradiction exists only where a record (a coincidence or contact row a body carries) claims what the reading rules out.

- **The tangent table's verdict.** `tangent_table` returns `Touch(margin) | Ruled { row: TangentRow, by: Decided } | SidesMixed { at: [Decided; 2] } | InBand(Indeterminate) | NotCertifiable`. `TangentRow` is a closed enum: Opposed, Independent, Parallel, Overlap, Gap, Off.
  - The Rest door (`rest_pair_verdict`) returns its relation with `Option<Decided>`, and rows `SensesAligned` and `TwoCarriers`.
  - Display names come from the row (`row.predicate()`). `Decided` gains no predicate field.
- **Reorder `tangent_pass`:** the first-order parallel check runs before the sides comparison, so a sample that is not tangent ends as "not parallel".
  - `SidesMixed` then means two samples each read tangent, with opposite senses: a definite crossing between them.
  - It stays a typed verdict, not an `unreachable!`. The lane admits a Villarceau rim (`tangent_certificate_lane` tests surface kinds only, and `shared_rim` asks only whether two circles are one), where n₁·n₂ varies.
  - topo README notes that rulings and lines of curvature cannot reach it.
- **The census matches the row.** `same_solid_tangent_pair`'s dispatch on `diag.predicate` strings goes (D4 ¶1 (ii)).
  - With no record, `Overlap` stays `CuspEdgesMissing`.
  - Against a record, `Ruled` or `SidesMixed` is `ValidationError::ContactContradicted { record, verdict }`, with the defect ending (D4 ¶1 (iv), `[verdicts.consumers]`).
- **The glue door decides a tangency once.** `GluedPair` carries `OneCarrier { relation, margin } | Tangent(Tangency)` per class, and `verify_glued_contacts` records what the door certified instead of re-running `glued_tangency`.
  - These go: `BooleanError::ContactContradicted`, `tangency_contradicted`, `TangencyFailure::Contradicted`, `verify_tangency`'s conformal screen and `Contradiction::OneCarrier`. All of them are debris of deciding twice.
  - `tangency_reading`'s discarded `Contradicted` becomes the pair's verdict: not glued.
- topo README C4's `ContactRefusal::{Contradicted, …}` sentence is re-worded with the code (agent text, `c7571919b0`).

## Step 1's leftover (PR 6634)

- **The Rest door's two contradictions still mint by hand.** These are I1 (`contact_rest_senses_opposed`) and I2 (`contact_rest_one_carrier`), in `rest_pair_verdict`: `Contradicted { diag: Indeterminate { INVALID, … } }`.
  - **Their evidence is `None`** (fork `glue-disagreements-carry`). A plane's sense is a cosine of about ±1, a curved kind's sense is a bit, and two kinds decide no datum.
  - I2 on a datum decided apart is the exception. It carries `CarrierVerdict::Apart(Decided)`, which step 1 now hands the door.
- **Why they wait here.** `ContactRefusal::Contradicted { diag: Indeterminate }` has no spelling for "no evidence", and the Rest verdict this step builds (`Option<Decided>`, rows `SensesAligned` and `TwoCarriers`) is that spelling. Step 1 would otherwise have built this step's type.

## Done when

No reading on this ground builds a `Contradicted` with an `Indeterminate`. The Boolean mints no contradiction. The census has no predicate-string dispatch, and the probe below has been run:

- **The probe:** two tori sharing a Villarceau circle, with the parallel check first. Does any pair of samples read tangent with opposite senses? Record the result beside `SidesMixed`.

## Dispatch (2026-11-09)

Branch `seal/tangent-verdict`. Review tier: **single, FULL**. Fork `glue-disagreements-carry` settled the types this step builds and the architecture they belong to. What is at risk is correctness: deciding a tangency once, reordering `tangent_pass`, and moving the census off strings. A FULL review's claims target exactly that, so the step does not take the dual tier.

## Closed (2026-11-09, PR 6673)

The tangent table and the `Rest` door return verdicts, and only a record read at rest is contradicted. The PR body has the per-site table (each ending before and after), the probe, the sweep and the premise corrections.

**Done-when is not met to the letter.** The census still dispatches on predicate strings in one place: `census::is_span_precision` (read at the `CensusEscalated` site and by `boolean::ops::collinear_escalated` and `ValidationError`'s `Display`) matches three row names. That is an escalation's decision, not a verdict, and `three-escalation-endings-route-by-the-predicate-name` schedules it with the other sites that pick a decision by name. `same_solid_tangent_pair`'s dispatch, the one the row named, is gone.

- **The verdicts** live in `crate::contact`, so the census names them without naming the engine:
  - `TangentVerdict::{Touch, Ruled(TangentRuling), InBand, NotCertifiable(Uncertified)}`, with `TangentRuling::{Row { row: TangentRow, by }, SidesMixed { at }}`;
  - `RestVerdict::{Rest(margin), Ruled(RestRuling { row: RestRow, by: Option<Decided> })}`.
  The `Rest` door's in-band and outside-the-inventory readings stay `ContactRefusal::{Escalated, NotCertifiable}`, and `ContactRefusal::Contradicted` is gone. The ruling types replace the row's flat `Ruled { row, by } | SidesMixed { at }` arms, so that a record's verdict (`ContactVerdict::{Rest(RestRuling), NotTangent { apart, by }, Tangent(TangentRuling)}`) carries the doors' rulings as they return them, with no parallel enum.
  - `Uncertified::SagUnreached` keeps the table's old `SagUnreached`, which the glue door refuses and every other reading calls not certifiable.
  - I1 is `SensesAligned` on no datum. I2 is `TwoCarriers`, on `CarrierVerdict::Apart`'s `Decided` where the carriers were decided apart, and on none for two kinds.
- **`tangent_pass`** reads the senses (`n₁·n₂`) first, because the touch needs their orientation. The perpendicular arm, `Independent`, ends there. The senses are compared (a contact's `SensesAligned`, a tangency's `SidesMixed`) only once the sample reads parallel. The gap row's sign is apart or crossing where the senses oppose, and which sheet stands over the other where they agree, so its sentence reads no sign.
- **The probe** (`contact_verify::tests`):
  - a torus and its mirror in a bitangent plane share a Villarceau circle, and are tangent along it at four isolated points: two aligned, where the mirror makes `κ_rel` zero, and two opposed;
  - a nine-station schedule over the whole circle, from any of 64 starts, never reads two tangent samples with opposite senses: its first station off the four ends the pass not parallel;
  - a span whose step runs from an opposed point to an aligned one does, and reads `SidesMixed`.
  The result is recorded beside the variant and in topo README C4.
- **The census** matches the verdict:
  - against a record, a ruling is `ValidationError::ContactContradicted { record, verdict }`, ending in `KERNEL_OR_FILE_DEFECT_ENDING`;
  - with no record, a pair the line shows apart or crossing is no touch;
  - with no record, a touch with its sides wrong is the body's defect: `Overlap` stays `CuspEdgesMissing`, and agreeing or mixed senses are the new `ValidationError::MaterialOnBothSides { a, b, witness, ruling }`, the defect ending (main's census-refusal ending for them, which spoke of a record, was the defect);
  - `census_rest_decision` answers `None` on a ruling.
- **The glue door decides a tangency once.**
  - A `GluedPair` holds one origin: the door's `Glued::{OneCarrier { relation, margin }, Tangent(Tangency)}`, a fold step's given decision under its class, or (in test builds) a bare index row. Its class is derived from that origin.
  - `verify_glued_contacts` records the door's decision. It shares `record_touch` with the fold step's `given_verdicts`, and the interiors placement moved from the door's reading to the op, its only reader.
  - Gone: `BooleanError::ContactContradicted`, `tangency_contradicted`, `TangencyFailure::Contradicted` (now `NotTangent`, not glued), the conformal screen with its `PlaneDoor::Screen`, `Coincide::Planes` and `Ending::Frontier`, `Contradiction`, `CONTRADICTION_REASON`, `CONTRADICTION_RECOURSE`.
- **Not done here:** step 4's ground (`vtxfac`/`recl` `carrier_eq_reading`, `unglued_coincidence`) is untouched.
- **Filed:**
  - `work/seal/three-escalation-endings-route-by-the-predicate-name`
  - `work/bind/contact-verdict-is-a-rung-under-contact-contradicted`
  - evidence on `work/contacthold/a-touch-only-rim-with-no-derivable-locus-refuses-every-op-as-a-cusp` (`RimCuspArmUnbuilt` is minted nowhere), `work/bind/boolean-decision-and-contradiction-are-rungs-under-boolean-error` (`Contradiction` retired), and the stale evidence lines of five open rows (TOPO, HONEWORD, TANGTAIL, CENSUS, CONTACTHOLD).
