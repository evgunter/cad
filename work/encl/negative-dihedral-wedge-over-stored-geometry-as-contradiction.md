---
id: negative-dihedral-wedge-over-stored-geometry-as-contradiction
kind: issue
title: geom-brep: is a definitely negative dihedral wedge over stored geometry a StoredDefinite::Contradiction?
status: closed
closed: 2026-10-14
branch: encl/definite-arm-one-story
pr: 5023
opened: 2026-10-13
priority: P3
cost: E
---


(Filed by the ENCL implementer of `import-door-appends-a-defect-note-to-a-sub-eps-in-zero-span`, from its Answered section's "Filed separately" list.)

## What

`geom_brep::DIHEDRAL_ARM` (`crates/geom-brep/src/dihedral.rs`) is `StoredDefinite::Contradiction`, so a definitely negative reading read at rest ends in `KERNEL_OR_FILE_DEFECT_ENDING` (`SizedDecision::recourse`, `crates/geom-brep/src/recourse.rs`). Its margin is the wedge the arm meters, `sin θ · arm`. Whether a definitely negative wedge over stored geometry is a contradiction between a stored description and its stored geometry, which no move of the geometry reaches (D4 ¶1 (iv)), or a configuration the lever does reach (`StoredDefinite::Lever`), has not been argued.

## Question

Which `StoredDefinite` does the arm's negative wedge belong to? Pin the answer with a row at rest.

## Evidence

- 2026-10-13 (ENCL implementer, `encl/definite-arm-one-story`): `DIHEDRAL_ARM`'s `no_size` note is not true of a definitely negative reading.
  - The note reads "an edge of no length, or a face curving to a point as a cone does, leaves no angle to measure".
  - The margin is `sin θ · arm` (`wedge_decided`, `crates/geom-brep/src/dihedral.rs`). `sin θ` is a norm quotient, so it is never negative. The arm is `min(extent, curvature arms)` (`folded_lever_arm`). The extent is a chord or diameter, and a cone's arm is a norm (`curvature_lever_arm`, `crates/geom-brep/src/implicit.rs`). So a definitely negative reading needs a stored radius or minor radius below zero. That is a breach of the `radius > 0` convention (`Surface::representability_margins`, `crates/geom/src/surfaces.rs`), not an edge of no length or a cone apex.
  - Since that branch, the note rides on such a reading at a build: on the sign-certain arm (`CertifyError::ArmCollapsed` with `Refused::Negative`, through `SizedDecision::recourse`). It also rides on the gate's rejected Negative routed as undecided, at a build, at rest and at the import door past ε_in (`dihedral::tests::a_gates_rejection_ends_by_its_reading_on_either_arm`, row "a rejected Negative", which pins it). One classification (`MarginDiag::shows`) now classes a reading past the band, wholly on the refused side, as `Shows::Refused`, which ends as `NoSize` does.
  - The same evidence bears on this row's question. A negative arm is a stored description that breaks its own convention, which no move of the geometry reaches. That reads as `StoredDefinite::Contradiction`. Whichever way it is answered, the note needs a clause for that reading, or the reading needs its own ending.

## Answered (ENCL orchestrator, 2026-10-14)

`DIHEDRAL_ARM` stays `StoredDefinite::Contradiction`. A definitely negative wedge needs a stored radius below zero. That breaks the stored convention, and no move of the geometry reaches it (D4 ¶1 (iv)). So at rest, and at the import door past ε_in, the sign-certain arm ends in `KERNEL_OR_FILE_DEFECT_ENDING`.

The decision's `no_size` note must be true of every reading the no-size classes carry for it: a zero, and a wholly negative wedge. PR 5023 implements this:

- The note now reads "an edge of no length or a cone's point leaves no angle to measure; a radius below zero is a defect".
- The negative wedge is pinned whole in `dihedral::tests::a_gates_rejection_ends_by_its_reading_on_either_arm`:
  - routed as undecided, it ends in the lever and the note at a build, at rest and at the import door past ε_in;
  - on the sign-certain arm, it ends in the lever and the note at a build, and in the defect ending at rest and at the door.
