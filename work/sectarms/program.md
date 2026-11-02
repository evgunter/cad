---
id: sectarms
kind: program
title: SECTARMS — the section object's remaining arms, led by NURBS against a curved face
status: active
opened: 2026-10-28
group: kernel/boolean/join
prefix: sectarms/
tag: (SECTARMS orchestrator)
paths: [crates/geom-brep/src/pair_section.rs, crates/topo/src/boolean/join.rs]
keep_out: [cut 2026-10-28 from SECT on Ev's request in chat (SECT was over its budget), the rows moved by git mv with ids and bodies unchanged - SECT keeps the torus-pair thread (teapot wall 2) and owns the section object's shape, so an arm built here is a new arm of geom_brep::pair_section and of the carried lane SECT step 1b built (GermLane::Carried, RingClosure::Chord), never a per-pair frame or lane beside them, SECTSITE holds the one-site and on-carrier rows and SECTRUNG holds the C5 dispatch and rung-3 residue at P2/P3, crates/topo/src/boolean/* is shared ground with FLUSH CLEAVE BOXES and others - run scripts/work.py territory on your branch and announce the seam in the PR]
---
**The section object's remaining arms.** SECT built the section-first
join's object (`geom_brep::pair_section`, `topo::boolean::section_table`;
PRs 5872, 5894) and its first instances: cylinder × sphere chords and
islands, and the torus poses. This track builds the arms it still lacks
on that object. It is led by the P0 rung-3 arm for a NURBS face against
a cone or sphere (klein wall 3, teapot wall 3, SPLICE's consumers),
then the cone frames, skew cylinders and the walk order. SECTSITE holds
the one-site and on-carrier rows.

Slate: `work/sectarms/plan.md`; narrative in `work/sectarms/log.md`.
