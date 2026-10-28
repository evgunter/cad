---
id: offset
kind: program
title: OFFSET — the offset lane: the carriers it cannot hold and the folds it calls bounds
status: ready
opened: 2026-09-20
group: kernel/shell-offset
prefix: offset/
tag: (OFFSET orchestrator)
ab_band: 8100-8199
paths: [crates/geom-brep/src/offset.rs, crates/geom-brep/src/offset_meters.rs, crates/topo/src/transform.rs, crates/topo/src/offset_axial.rs]
keep_out: [opened by SHELL's 2026-09-20 priority-seam cut (Ev, in chat) per work/README.md Track size - SHELL measured 65.5 budget points and was cut into tracks meant to run in PARALLEL (Ev, in chat: for these high priority tracks it is ideal to have several components that can be worked on in parallel), the siblings are SHELL CLEAR OFFSET and they share their parent's territory by design - shared ground is legitimate by the README's 2026-09-20 rule and what is owed is awareness while a lane is LIVE, so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled rather than drawing a fence, TOPO and its cut siblings own topo/src, SYM owns geom_core::sym and the symbolic-tier row crosses to it by announced seam]
---
The offset lane, split out because its open rows are about the CARRIER
algebra rather than about the shell verb that consumes it: the carriers
the offset doors cannot hold (a partial-revolve cone's moved cap meets
its wall in a hyperbola the kernel has no kind for), the cost of the
doors themselves, and the decisions on this ground that fold or skip a
bound.

Charter and order: `work/offset/plan.md`; narrative in `work/offset/log.md`.
