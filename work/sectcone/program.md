---
id: sectcone
kind: program
title: SECTCONE — the cone on the section value: its implicit enclosure and metres conversion, and the cone pairs' section and join reading
status: ready
opened: 2026-11-03
group: kernel/boolean/join
prefix: sectcone/
tag: (SECTCONE orchestrator)
paths: [crates/geom-brep/src/pair_section.rs, crates/geom-brep/src/ssi/residual.rs, crates/topo/src/boolean/join.rs]
keep_out: [cut 2026-11-03 from SECT and SECTARMS on Ev's request in chat (both were over budget) - the rows moved by git mv with ids and bodies unchanged, SECTARMS keeps the NURBS x implicit arm and the cylinder pairs and builds the NURBS x cone stage on this track's cone enclosure, SECTTORUS holds the torus thread, the section object's shape (geom_brep::pair_section, the carried lane, PairSection node listing per C5) is shared and an arm built here is an instance of it and never a per-pair frame or lane beside it, the ruling sweep (pair_section::ruling) is SECTARMS-built and shared with the cone by design, crates/geom-brep/src/ssi* is SSIARITH SSIEDGE SSIMARCH ground, GERM holds the cone's operand root lanes (line x cone, the apex refusal zone), crates/topo/src/boolean/* is shared ground - run scripts/work.py territory on your branch and announce the seam in the PR]
---
**The cone on the section value.** Every rung-3 pair with a cone
refuses today, because the cone has no implicit enclosure and no
metres conversion (`the-cone-has-no-implicit-enclosure-or-metres-conversion`,
P0). That row gates cone × torus, cone × sphere and cone × cone, and
SECTARMS's NURBS × cone stage too. On top of it, the cone pairs
against a curved face read their pose on the section value (C5's
ruling sweep covers a cone side), and the conic germ pairs still on
the frame lane move to the value's foot parameter.

The design is ratified: the section object and its carried lane (SECT,
PRs 5872/5894), C3's domain and trace rules and C5's pose, ruling-sweep
and node rules (`[ev]` PRs 6121, 6127, 6137, 6398), all in
`crates/geom-brep/README.md`. An arm built here is an instance of that
object.

Narrative goes in `work/sectcone/log.md` once the track is picked up.
