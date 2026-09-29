---
id: census-undecidable-what-cannot-carry-a-valued-ending
kind: issue
title: topo: CensusUndecidable's what is a &'static str, so a witness decision's margin cannot value its tighten
status: open
opened: 2026-09-29
---


(CONTACT-10 implementer, `contact/10-contain-endings`; the rule is D4 ¶1
(i) in `docs/DESIGN.md`.)

## What

`ValidationError::CensusUndecidable { what: &'static str }`
(`crates/topo/src/validate.rs`) is the renderer of every
`census::Undecided` reason. CONTACT-10 carries the point-in-loop walk's
escalating decision into `Undecided::WitnessTooClose(Some(ContainDecision))`.
The escalation's margin (`Indeterminate`) could ride along too, but a
`&'static str` cannot hold the tolerance it gives, so the reason ends in
the decision's lever alone. The same refusal read through
`classify_contain` ends in the lever plus "if this distance is intended,
tighten the tolerance below m/K m".

## Repair shape

Carry the reason on the variant (`what: Undecided`, rendered by
`Display`) or widen `what` to an owned string. Then
`Undecided::WitnessTooClose` can hold the `Indeterminate` and end through
`ContainDecision::ending(cause, Reading::AtRest)`. The field is read as a
`&str` by a dozen `topo` and `sweep` test files (`what.contains(...)`),
so the change moves those rows.
