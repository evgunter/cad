---
id: an-undecided-span-reading-at-rest-on-the-unminted-side-ends-as-a-defect
kind: issue
title: An undecided span reading at rest on the side no construction mints ends as a kernel defect or a damaged file
status: closed
opened: 2026-11-11
priority: P2
cost: E
closed: 2026-11-16
branch: onehome/undecided-unminted
---


A message-ending follow-up from ONEHOME fork 259 (PR 6820), kept out of
that PR by the orchestrator's ruling.

## What

Fork 144 (`crates/topo/src/validate.rs:7370`) and fork 259 rule that an
undecided reading is reported as undecided, never as a definite breach
and never in the "kernel defect … report it" ending. PR 6820 makes the
unsized decisions do so past a build (`Unsized::recourse`,
`crates/geom-brep/src/recourse.rs:299`).

A sized decision does not take that path. `CertCheck::recourse` hands a
sized check to `SizedDecision::recourse`
(`crates/geom-brep/src/certify.rs:1809`,
`crates/geom-brep/src/recourse.rs:665`), and on the side no
construction mints, a span reading at rest still ends in "There is no
way through: this is a kernel defect or a damaged file; report it",
even when the reading is undecided. The "unminted" rows of
`each_decision_family_ends_in_its_routed_sentence`
(`crates/geom-brep/src/certify.rs:8779`) pin that: `ParamSpan` at
`MarginDiag::value(-5e-9)`, in band and undecided, ends at rest in
`KERNEL_OR_FILE_DEFECT_ENDING`.

## Done when

An undecided sized reading at rest, on any side, ends as undecided: over
a box in `BOX_SCALAR_RECOURSE`, at a point in the decision's lever or the
last resort. The unminted rows are re-pinned to say so, and a definite
reading on the unminted side keeps the defect ending.

## Closed (2026-11-16): not a defect, the readings are definite

The row's readings are definite, not undecided. D4 ¶1 (iv)
(`docs/DESIGN.md:578`): "'Definite' means the reading certainly lies on a
refused side, whether the band, a number or a proof decided it … every
definite reading of it ends by that at each door". The unminted rows'
`ParamSpan` at `-5e-9`, at `±0`, and `ParamWinding` over
`[-4e-9, -2e-9]` each lie wholly on the refused side, so
`MarginDiag::shows` classes them `NoSize`/`Refused`, and they end by their
side whichever arm routes them. Every reading the band leaves undecided
(unresolved, an offer, a floor, unreadable) already ends in the lever at
every door, on every side: `Shows::ending`
(`crates/geom-core/src/predicate.rs`) reads `RefusedReading::Defect` on
`NoSize`/`Refused` alone. What the row read as "undecided" was the arm
that routed the reading (`RefusedArm::Undecided`), not its class, and
keying the ending on the arm is what (iv) forbids.

Measured: keying it on the arm (`769ca5668b`, reverted in `79354100de`)
reddens six principle rows in `crates/geom-brep`:
`recourse::tests::one_definite_reading_ends_one_way_whichever_arm_routes_it`,
`recourse::tests::either_route_ends_a_reading_alike`,
`certify::tests::a_hand_built_unminted_refusal_renders_at_every_door`,
`certify::tests::each_sized_decision_ends_past_eps_in_by_its_refused_side`,
`certify::tests::a_stored_arc_past_a_turn_at_a_callers_finer_band_ends_typed`
and `pcurve_cache::tests::each_pcurve_check_names_its_refused_side`.

Checked, the `-5e-9` case at each door, against (iv):
`a_hand_built_unminted_refusal_renders_at_every_door`
(`crates/geom-brep/src/certify.rs:8382`) pins it in the check's lever at a
build, the kernel's or the file's defect at rest and the file's at the
import door, each as the `CertifyError::StatedInterval` a public door
returns for the same reading. That is (iv)'s story for a side no
construction produces; no door ends it otherwise, so nothing is refiled.

