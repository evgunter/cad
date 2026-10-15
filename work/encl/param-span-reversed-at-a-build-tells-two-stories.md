---
id: param-span-reversed-at-a-build-tells-two-stories
kind: issue
title: geom-brep: a reversed ParamSpan at a build ends in the lever alone past the band and in the lever plus the defect note inside it
status: closed
pr: 5023
opened: 2026-10-13
priority: P3
cost: E
branch: encl/definite-arm-one-story
closed: 2026-10-14
---


(Filed by the ENCL implementer of `import-door-appends-a-defect-note-to-a-sub-eps-in-zero-span`, from its Answered section's "Filed separately" list.)

## What

`CertCheck::ParamSpan` (`crates/geom-brep/src/certify.rs`, `CertCheck::ending`) is `StoredDefinite::Contradiction` with a no-size note. At a build, a reversed span ends two ways (`SizedDecision::recourse` in `crates/geom-brep/src/recourse.rs`):

- past the band, it reaches the sign-certain arm (`RefusedArm::SignCertain`) and ends in the lever alone;
- inside the band, it reaches the Zero or Undecided arm, reads as NoSize (a point on the refused side), and ends in the lever plus "an edge of no length, or a reversed one, is one no kernel construction mints, so this is a kernel defect worth reporting".

So one user situation (a reversed edge) tells two stories by distance from zero. D4 ¶1 (iv) asks the definite arm to tell the same story as its in-band sibling.

## Question

Should the build's sign-certain arm carry the decision's note too where the decision says no construction mints the refused side? Or should the note stop claiming "a reversed one"?

## Ruled (ENCL orchestrator, 2026-10-13)

D4 ¶1 (iv) settles it: a definite outcome tells the same one story as its in-band sibling. At a build, a sized decision's sign-certain refused arm ends exactly as its in-band NoSize sibling does: the lever plus the decision's `no_size` note. `MarginDiag::shows` already classes a reading wholly on the refused side as NoSize, so past the band is that class too. The note is not narrowed. Check every `no_size: Some` decision's note for truth on a definite refused reading. `DIHEDRAL_ARM` overlaps `negative-dihedral-wedge-over-stored-geometry-as-contradiction`: coordinate with that row, don't fold it in silently.
