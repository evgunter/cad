---
id: a-section-loop-inside-two-one-band-curved-faces-refuses-at-the-section-certificate
kind: issue
title: A section loop inside two one-band curved faces (ball and donut) refuses at the no-crossings path's section certificate (R-loop), where a π meridian used to cross it
status: open
opened: 2026-11-23
priority: P1
cost: H
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
---


Found by the LATHE mesh-budget lane (2026-11-23), measured on
`lathe/mesh-budget-one-band` (base `lathe/full-revolve-one-band` at
`03e5b2264`), and again with PR 7440's head (`023c8d7b19`,
`lathe/interior-loop-one-band`) merged in: it refuses the same there.

## What

`quadric_pair_sections`' ball `ball(0.5)` against the donut
`donut(0.6, 0.2)` moved by `(0.1, 0.35, 0.2)` and turned `0.7` about `x`
(`crates/mesh/tests/quadric_pair_sections.rs`, the row this lane renamed
`a_rod_through_a_donut_builds_and_a_ball_against_it_refuses_its_interior_loop`):
every op, both orders of the difference, refuses
`FallbackExtentUnsupported` with `section_cert::Refusal::Loop`'s sentence
("a closed loop interior to both faces … the loop is certified, and
cutting it in is not built",
`crates/topo/src/boolean/section_cert.rs`, `Refusal::what`), naming the
donut's one torus face. On main, with two π-bands on each operand, the
same ops build: a seam meridian at π crossed the section loops, so the
crossing layers saw events.

This is the class of
`a-section-loop-inside-a-one-band-curved-face-refuses-the-interior-loop-guard`
(on `lathe/interior-loop-one-band`, PR 7440) on another route. PR 7440
cuts an interior loop in on the crossings path's interior-loop guard
(a plane against a curved face). This pair is two curved faces on the
no-crossings path, whose section certificate counts and certifies the
loop and then refuses because no cut-in is built there.

## Ground

`crates/topo/src/boolean/section_cert.rs` (`Refusal::Loop`) and the
no-crossings path that reads it (`crates/topo/src/boolean/ops.rs`);
topo's boolean territory, as its sibling. The march lists the loop's
carrier (a periodic cubic), so a cut-in here inserts a ring of a spline
edge on each face.

