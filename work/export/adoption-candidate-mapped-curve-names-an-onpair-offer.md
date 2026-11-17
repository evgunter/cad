---
id: adoption-candidate-mapped-curve-names-an-onpair-offer
kind: issue
title: the adoption ladder's MappedCurve candidate labels the pair the must-carry rung offers, so the candidate vocabulary no longer names what is offered
status: open
opened: 2026-11-17
priority: P4
cost: E
refs: [step-import-adopts-a-smooth-join-without-the-must-carry-rule]
---


## Finding

Found by the review of PR 7081 (S7). `crates/step-import/src/error.rs`'s
`AdoptionCandidate` names the ladder's interpretations as data for
remedy flows, and its `MappedCurve` variant reads "a conventional
mapped-curve self-description". `crates/step-import/src/adopt.rs`'s
`adopt_edges` now pushes three different descriptions under it: an
image in one surface's chart (one record on both sides), an image in
the plane's chart (a plane against a NURBS wall), and the pair
`must_carry_rung` offers between two analytic records
(`EdgeDescriptionSpec::OnPair`, possibly undeclared). An `Adoption`
refusal's attempt list therefore names the pair "mapped curve", and a
remedy flow reading the candidate cannot tell a chart image from a
pair. The vocabulary wants a candidate per description the ladder
offers (`Chart`, `OnPair`), or one named for the conventional rung
rather than for the declaration that may ride it.

P4: the refusal's rendered refusal text names the right decision either
way; only the label is stale.
