---
id: param-span-reversed-at-a-build-tells-two-stories
kind: issue
title: geom-brep: a reversed ParamSpan at a build ends in the lever alone past the band and in the lever plus the defect note inside it
status: open
opened: 2026-10-13
priority: P3
cost: E
---


(Filed by the ENCL implementer of `import-door-appends-a-defect-note-to-a-sub-eps-in-zero-span`, from its Answered section's "Filed separately" list.)

## What

`CertCheck::ParamSpan` (`crates/geom-brep/src/certify.rs`, `CertCheck::ending`) is `StoredDefinite::Contradiction` with a no-size note. At a build (`SizedDecision::recourse` in `crates/geom-brep/src/recourse.rs`, its `RefusedArm::SignCertain` arm):

- past the band, a reversed span ends in the lever alone;
- inside the band, a reversed span reads as NoSize (a point on the refused side) and ends in the lever plus "an edge of no length, or a reversed one, is one no kernel construction mints, so this is a kernel defect worth reporting".

So one user situation (a reversed edge) tells two stories by distance from zero. D4 ¶1 (iv) asks the definite arm to tell the same story as its in-band sibling.

## Question

Should the build's sign-certain arm carry the decision's note too where the decision says no construction mints the refused side? Or should the note stop claiming "a reversed one"?
