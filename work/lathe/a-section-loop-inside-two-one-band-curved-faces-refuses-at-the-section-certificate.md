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


## 2026-11-25: past the certificate, the ring homing refuses

With PR 7575 on the stack (its cut-in for a section loop inside a
one-band sphere face and a curved face), the class gets past the
section certificate. One sibling now stops later:
`sweep` `germ_sphere_torus_placements::a_ball_against_a_donut_carries_onto_the_torus_and_builds`,
placement "a spline edge in a torus ring's run" (`donut0 ∪ moved`), refuses
`Join(BooleanJoin(RingHomingAmbiguous { ring: LoopKey(6v1) }))` at ε 1e-9,
1e-6 and 1e-12, on `lathe/full-revolve-one-band` at a8a9d3a9b6. The
other three placements build. Base 84588ee6fa (two π-bands) builds all
four. The row is pinned red at its stop (`pinned_red::row`,
`EVERY_ROW`), citing this row.

- 2026-11-26 — PR 7575 (in the one-band stack, landed with PR 7414) builds the item's own poses through the no-crossings cut-in. The row stays open: `sweep germ_sphere_torus_placements::a_ball_against_a_donut_carries_onto_the_torus_and_builds` is pinned red at every ε on it (`RingHomingAmbiguous`, a spline edge in a torus ring's run, once the cut-in is built). That is this row's residue.
- 2026-11-27 — Not part of `close-cuts-is-not-total-on-crossings-path-cut-ins` (LATHE, `lathe/close-cuts-total`). The residue gets past the cut-in and the re-entry, and stops in the join at `RingHomingAmbiguous` (a spline edge in a torus ring's run). The closing, which runs in the output stage after the join, is never reached. The priority stands.
- 2026-11-28 — `lathe/strut-ring-homing` defers a strut ring no path decides (`Undecided`), as an `OnRun` one on a plane is, and holds the deferral to a checked invariant. Ring 6v1 is deferred. The residue moves to 7v1, whose own polygon completes inside it: `RingHomingAmbiguous { ring: 7v1 }`, donut first only. The ball-first ops build, with tier 3′ and inclusion–exclusion held. The rest is `a-strut-rings-side-is-read-from-its-own-germ` (P1), and this row stays open on it.
