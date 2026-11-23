---
id: secttorus
kind: program
title: SECTTORUS — the torus pairs on the section-first join: the plane × torus general rung (lily wall 8), the torus doors' and gaps' charges, and the torus rows' cost
status: ready
opened: 2026-11-03
group: kernel/boolean/join
prefix: secttorus/
tag: (SECTTORUS orchestrator)
paths: [crates/geom-brep/src/pair_section.rs, crates/geom-brep/src/pair_section/torus.rs, crates/topo/src/boolean/join.rs]
keep_out: [cut 2026-11-03 from SECT on Ev's request in chat (SECT was over budget) - the rows moved by git mv with ids and bodies unchanged, SECT keeps the one-site and on-carrier join arms and its residue, SECTCONE holds the cone and the cone x torus arm, SECTARMS holds the NURBS x implicit arm (its NURBS x torus row is parked on this track's plane x torus row, lily wall 8's next stop), the section object's shape is shared and an arm built here is an instance of geom_brep::pair_section and its carried lane and never a per-pair frame beside them, crates/geom-brep/src/ssi* is SSIARITH SSIEDGE SSIMARCH ground, crates/topo/src/boolean/* is shared ground - run scripts/work.py territory on your branch and announce the seam in the PR]
---
**The torus pairs on the section-first join.** SECT built the torus
poses on the section object. What remains is mostly the plane × torus
general rung that lily wall 8 stops at
(`c5-plane-torus-cone-cylinder-arms`, P0, two hard pieces: the tilted
plane's spiric section and U-H1's touching poses), plus the torus
doors' and gaps' charges at a coarse or fine band and the cost of the
torus rows.

The design is ratified (SECT's section-first join, `[ev]` PR 4313;
C3/C5 in `crates/geom-brep/README.md`). An arm built here is an
instance of the section object.

Narrative goes in `work/secttorus/log.md` once the track is picked up.
