---
id: stored-contradiction-at-a-build-still-tells-the-lever-story
kind: issue
title: RefusedSide::StoredContradiction still ends a definite reading at a build in the lever, where D4 ¶1 (iv) now makes a stored contradiction the kernel's defect at a build
status: open
opened: 2026-10-27
priority: P2
cost: M
refs: [who-placed-it-is-read-off-the-description-not-the-door]
---


## Finding

Filed by PCERT E1d, out of that unit's scope: E1d moved incidence and
nothing else.

Ev's ruling on PR 5838 changed DESIGN D4 ¶1 (iv). Before, it said "A
contradiction between a stored description and its stored geometry
tells the lever story at a build". Now it says that contradiction "is
the kernel's defect at a build, since every build door that holds a
carrier and a face is the kernel's own: a verb that takes a user's
curve refuses it at its own door, with its lever".

`geom_brep::recourse::RefusedSide::StoredContradiction` still documents
and renders the old sentence: "At a build it is the user's geometry,
which the lever moves: the lever and the note". These sized decisions
take that side:
- `CertCheck::Transversality` and `CertCheck::TangentSecondOrder`
  (`crates/geom-brep/src/certify.rs`, `CertCheck::ending`);
- `CertCheck::ImageSpan` (`span_decision`);
- the pcurve span checks `AzimuthPeriod` and `TubePeriod`
  (`PcurveCheck::routing`, `SpanDecision::Winding.on(StoredContradiction)`).

Whether each of those is "a contradiction between a stored description
and its stored geometry" in the new sentence's sense is the question.
- A transversality reading at a Boolean build is the user's operands'
  pose. That is a sized decision the user may intend, so its lever is
  arguably right.
- A span or winding past its gate on a row the kernel minted is the
  kernel's.

So the class may need splitting rather than flipping. E1d did not touch
it, and every one of these still ends in the lever at a build.
