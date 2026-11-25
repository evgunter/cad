---
id: coneslope
kind: program
title: CONESLOPE — how Surface::Cone stores its opening: its slope k = tan α instead of its half-angle, and the one convention ladder every cone reader decides
status: ready
opened: 2026-11-24
group: kernel/boolean/join
prefix: coneslope/
tag: (CONESLOPE orchestrator)
paths: [crates/geom/src/surfaces*, crates/geom-brep/src/pair_section/cones.rs, crates/geom-brep/src/intersect.rs]
keep_out: [cut 2026-11-24 from SECTCONE (over budget at its hand-back) - the rows moved by git mv with ids and bodies unchanged, the slope unit touches every Surface::Cone reader across the workspace including STEP import and export and the representability margins so announce the seam with every open lane that reads a cone, SECTCONE keeps the cone pairs' section join march and germ-frame rows]
---
**How the cone stores its opening.** `Surface::Cone` stores its half-angle
`α`. Every certified reader takes `sin α` and `cos α` back out of it, a
transcendental of a stored datum, which C9's "every enclosure certification
needs is transcendental-free" does not admit. The design forks weighed for
`the-cone-has-no-implicit-enclosure-or-metres-conversion` (SECTCONE)
recommended storing the slope `k = tan α`. Ev agreed on PR 7326 ("sure
sounds good"). `the-cone-stores-its-slope` is that change.
`the-cone-convention-ladder-is-spelled-five-times` is the ladder every cone
reader decides about the opening. The slope change rewrites each of its
sites, so the two rows share this track.

Cut from SECTCONE at its hand-back on 2026-11-24, along the seam between
the cone's representation and the cone pairs' section and join reading.
The SECTCONE log (`work/sectcone/log.md`) has the history behind the
slope ruling.
