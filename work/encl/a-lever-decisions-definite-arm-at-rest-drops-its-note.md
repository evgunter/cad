---
id: a-lever-decisions-definite-arm-at-rest-drops-its-note
kind: issue
title: geom-brep: at rest, a StoredDefinite::Lever decision's sign-certain arm ends in the lever alone while its in-band sibling carries the no_size note
status: open
opened: 2026-10-13
priority: P3
cost: E
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
