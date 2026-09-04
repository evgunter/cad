---
id: verbs
kind: program
title: VERBS — the modeling-verb breadth program
status: closed
closed: 2026-09-04
opened: 2026-08-21
area: kernel
prefix: verbs/
tag: (VERBS orchestrator)
ab_band: 100-199
paths: [crates/geom-brep/src/intersect.rs, crates/geom-brep/src/ssi.rs, crates/geom-brep/src/ssi/*, crates/topo/src/offset_axial.rs, crates/sweep/src/revolve/*, crates/sweep/tests/verbs_*, crates/topo/tests/verbs_*, docs/KERNEL-VERBS.md]
keep_out: [the boolean reduction and its honest remainder are S-BOOL's (crates/topo/src/boolean), fillet band/surgery and chamfer parity were S-BLEND's and stay ceded, editor-core recipe doors and Verb lowering are LIB's and SEAT's, the shell and offset verbs (topo/{shell,replace_face,transform,offset_together}.rs and geom-brep/offset*.rs) are SHELL's since 2026-09-04 — offset_axial.rs stays here only until VERBS-RIMCAP merges, C7/REST joins and the loft U-turn gate are not verb-gating]
---

Executes `docs/KERNEL-VERBS.md`, the register: the missing modeling verbs
whose prerequisites are ratified, in dependency order, plus the register's
verb-gating defect rows. Waves 1 (plumbing), 3 (offset → shell → the teapot)
and most of 4 (what the consumers measured) have merged; what remains is
Wave 2's curved boolean breadth — the germ lanes over analytic pairs, the C5
section arms, and the cone/torus operand lanes. Design conversations open as
information firms and are Ev-paced. Detail: `plan.md`; live narrative:
`log.md`'s tail.
