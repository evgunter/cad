---
id: boolean-arm-exists-cites-a-retired-approx-composition-rule
kind: issue
title: boolean_arm_exists keeps Approx off the roster on a composition-rule premise that C5 no longer holds
status: open
opened: 2026-10-09
priority: P4
refs: [a-fitted-wall-has-no-section-with-a-moved-cap]
---

Found by SHELL's plane × `Approx` unit. `boolean_arm_exists`'s doc
(`crates/topo/src/boolean/reduce.rs:191`) keeps `Approx` off the
boolean's roster "until a rule for composing the fit's precision claim
with the boolean's certificates is ratified". The C5 ruling that unit
landed (`crates/geom-brep/README.md`, C5; O2's dispatch sentence) is
that nothing is composed: as an operand an `Approx` surface is its fit,
and the fit's distance from its description is the face's claim. The
refusal itself may still be right (the crossing layer for a spline
face does not exist, `reduce.rs`'s NURBS arm), but its stated reason is
the retired premise. Restate the reason, or rule on whether the boolean
reads a fitted operand as its fit.
