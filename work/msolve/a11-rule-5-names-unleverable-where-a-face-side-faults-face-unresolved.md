---
id: a11-rule-5-names-unleverable-where-a-face-side-faults-face-unresolved
kind: issue
title: A11 rule 5 says an unresolved part faults Unleverable; a FromFace side faults FaceUnresolved first
status: open
opened: 2026-09-24
priority: P4
cost: E
---


`crates/editor-core/ASSEMBLY.md` A11 rule 5 says *"A mated part that
does not resolve faults its mate `MateFault::Unleverable` in the
resolver's own voice, carrying the part fault unaltered"*. Since
MSOLVE-9 that holds for a mate whose two sides are authored vectors
only. A side authored `FromFace` is resolved before the lever is asked
(`mate/solve.rs` `admit_mate` and `fold_pair` call `resolve_side`
first, which asks `MateReach::face_pose`), so an unresolved part on a
face side faults `MateFault::FaceUnresolved { refusal:
FaceRefusal::PartUnresolved { fault, .. } }` — the part fault still
unaltered, under a different arm. Pinned by
`msolve9_from_face::an_unresolvable_part_faults_in_the_resolvers_voice`
(`RefusingReach` at the door, `EvalOptions::default()` at evaluation).

The code is what the spec asked for (MSOLVE-9 §2: `RefusingReach`
refuses `PartUnresolved` through the new arm); the page's sentence
reads as universal. The re-wording is one clause (name both arms), but
rule 5 carries ratified text the MSOLVE-9 lane was told not to touch,
so it is the orchestrator's call whether this lands as a consequence
of the approved change or waits for Ev.
