---
id: union-with-a-tilted-cylinder-boss-refuses-as-classification-invariant
kind: issue
title: boolean: a union with a definitely tilted cylinder boss refuses ClassificationInvariant (the volume backstop) where a frontier refusal is due
status: open
opened: 2026-09-08
---


## Finding (BLEND unit 1's style review, PR 2123; reported to S-BOOL by the BLEND orchestrator)

A boss extruded from a `SketchPlane` tilted by a DEFINITE angle
(measured at 4e-3 rad and at 0.2 rad), standing on or penetrating a
box, refuses through the union as
`ClassificationInvariant { "volume backstop: mass properties refused on
a tier-valid planar body" }` — `crates/topo/src/boolean/ops.rs:744`–`:758`
maps `mass_properties_closed_form`'s refusal to `corrupt()`. The
untilted penetrating boss builds. So a tilted cylinder (its stored axis
not the world normal — the one public route to that is a tilted sketch
plane) reaches the boolean's row-4 "kernel invariant" message for what
reads as a row-2 frontier (a support-pair configuration the rebuild
does not carry): the sentence blames the kernel for a shape it does
not admit. In-band tilts are refused earlier and honestly at
`split_conic_plane_parallel` (margin 5e-9 on a 5ε tilt); the definite
tilt is the case with the wrong sentence.

Recorded by
`crates/sweep/tests/review_blend1_r1_probes.rs::r1_a_boss_on_an_in_band_tilted_sketch_plane_through_the_union`
(the in-band door) and the reviewer's report for the definite tilts;
a row for the definite case is owed here.

## Home

`work/bool/` — `crates/topo/src/boolean/*` is S-BOOL's; filed by the
BLEND orchestrator per `docs/prompts/implementer-discipline.md` §6.
