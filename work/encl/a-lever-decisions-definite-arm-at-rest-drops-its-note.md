---
id: a-lever-decisions-definite-arm-at-rest-drops-its-note
kind: issue
title: geom-brep: at rest, a StoredDefinite::Lever decision's sign-certain arm ends in the lever alone while its in-band sibling carries the no_size note
status: closed
pr: 5101
opened: 2026-10-13
priority: P3
cost: E
branch: encl/at-rest-one-story
closed: 2026-10-14
---

(Filed by the ENCL implementer of `param-span-reversed-at-a-build-tells-two-stories`, from its sweep.)

## What

That unit made a sized decision's sign-certain refused arm at a build end as its in-band NoSize sibling does: the lever plus the decision's `no_size` note (`SizedDecision::recourse`, `crates/geom-brep/src/recourse.rs`). It kept the at-rest arms as they were, as briefed.

At rest, a `StoredDefinite::Lever` decision still tells two stories:

- in band, on the refused side, a reading ends in the lever plus the stored note (`MarginDiag::sized_recourse`, class `NoSize`);
- the sign-certain arm, `(Reading::AtRest, StoredDefinite::Lever)`, ends in the lever alone.

Two production decisions have this shape:

- `CertCheck::ParamSpanMeter` (`crates/geom-brep/src/certify.rs`, `CertCheck::ending`). A spline that turns back on itself reads a definitely negative floor (`review_m2_pr3_certify::a_collapsed_spline_meter_refuses_as_the_meter_decision`, the "turns back" net). Read at rest, it ends in the lever alone. A stalled one ends with the note.
- `Meter::NormalFloor` (`crates/geom-brep/src/offset_meters.rs`, `Meter::decision`). Its thinness is never negative (the `MeterError::NormalFloor` docs), so the definite arm there is unreachable in practice.

D4 ¶1 (iv) excuses only a definite *contradiction* read at rest, which ends as a defect. A Lever decision is by definition one the lever reaches, so (iv) would ask for one story.

## Question

Should `(Reading::AtRest, StoredDefinite::Lever)` take the class route too, ending in the lever plus `NoSize::stored`? The answer would be `sign_certain_recourse` over the words at rest.

## Evidence

- 2026-10-14 (ENCL implementer, `encl/definite-arm-one-story`, PR 5023 review): the same branch splits one definite reading at rest by the route that carries it. `MarginDiag::shows` classes a reading past the band, wholly on the refused side, as `Shows::Refused`, which ends as `NoSize` does. So a gate's rejected definite reading routed as `RefusedArm::Undecided` ends in the lever plus `NoSize::stored` at rest (`SizedDecision::recourse`, its `Undecided` arm through `MarginDiag::sized_recourse`). The same reading on `RefusedArm::SignCertain` at rest ends by `SizedDecision::stored`:
  - **Lever decisions** (`CertCheck::ParamSpanMeter`; `Meter::NormalFloor`): Undecided-past ends in the lever plus the note, and SignCertain at rest in the lever alone (`(Reading::AtRest, StoredDefinite::Lever)`).
  - **Contradiction decisions** (`CertCheck::ParamSpan`, `DIHEDRAL_ARM`): Undecided-past ends in the lever plus the note, and SignCertain at rest in `KERNEL_OR_FILE_DEFECT_ENDING` (`(Reading::AtRest, StoredDefinite::Contradiction)`). `dihedral::tests::a_gates_rejection_ends_by_its_reading_on_either_arm` pins both routes for the negative wedge.
  - The import door past ε_in reads as at rest, so it carries the same split.
  - So the question above widens: one definite reading should end one way at rest whichever route carries it. For a Contradiction decision that would make the Undecided-past route end in the defect ending, or the SignCertain route in the lever plus the note.


## Ruled (ENCL orchestrator, 2026-10-14)

One definite reading ends one way at rest whichever arm routes it. The ending reads the reading's class and the decision's `StoredDefinite`, never the route. Per D4 ¶1 (iv), applied as ratified:

- **`StoredDefinite::Lever`:** a definite refused reading at rest (`Shows::Refused`, or the sign-certain arm) ends in the lever plus the decision's `no_size` note, the same as its in-band sibling. A Lever decision is one the lever reaches.
- **`StoredDefinite::Contradiction`:** a definite refused reading at rest ends in `KERNEL_OR_FILE_DEFECT_ENDING` on either route. That includes the import door past ε_in, which reads as at rest. (iv) excuses exactly this case: a contradiction between a stored description and its stored geometry that no move reaches. An in-band reading at rest is not definite, so it keeps the lever plus the note.
- **A build is unchanged:** every sized decision's definite refused arm ends in the lever plus the note, as PR 5023 made it.

Derive the route-free ending from one place, so the two routes cannot drift. Pin both routes for each of the four `no_size: Some` decisions at rest, and at the door past ε_in.
