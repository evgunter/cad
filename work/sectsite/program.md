---
id: sectsite
kind: program
title: SECTSITE — one-site and on-carrier sections on curved faces, behind the section object
status: ready
opened: 2026-10-28
group: kernel/boolean/join
prefix: sectsite/
tag: (SECTSITE orchestrator)
paths: [crates/topo/src/boolean/join.rs, crates/topo/src/boolean/vtxfac.rs]
keep_out: [cut 2026-10-28 from SECT, through SECTARMS, on Ev's request in chat (SECT measured over budget; SECTARMS alone would have opened at 38/30) - the rows moved by git mv with ids and bodies unchanged, SECT owns the section object and SECTARMS its transverse arms, so a one-site or on-carrier reading here is a reading of geom_brep::pair_section's components and never a per-pair frame, the torus on-carrier sector (U-H1) is held by D10 until INTENT reaches it - read work/intent/d10-one-way-to-say-intent-is-unbuilt.md first, crates/topo/src/boolean/* is shared ground - run scripts/work.py territory and announce the seam]
---
**One-site and on-carrier sections on curved faces.** Two rows SECT held
where a section meets a curved face in a degenerate way:
- a closed edge lying in a cone, cylinder or torus face, where the one-site
  join has no arm to side its conic;
- the C5 table's plane × torus and cone × cylinder residue. That residue
  is its U-H1 touching poses: a torus sector on the plane, the tangent
  circle, and the Villarceau pair, held by D10.

Both are read on the section object SECT built.

Slate: `work/sectsite/plan.md`; narrative in `work/sectsite/log.md`.
