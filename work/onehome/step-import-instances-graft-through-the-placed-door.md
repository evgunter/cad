---
id: step-import-instances-graft-through-the-placed-door
kind: issue
title: step-import's instance loop moves and grafts each part by hand instead of through graft_placed
status: open
opened: 2026-11-18
priority: P2
cost: E
---


`topo::boolean::graft_placed` (`crates/topo/src/boolean/combine.rs`) is the one door that grafts a placed copy into an aggregate. It moves the copy's data through its motion, asks the session box of the image through the one move-and-ask (`finish::moved_and_asked`), then grafts. Every gather in `editor-core` goes through it: the `⊔`, distribute's assembly and the product (PR 7099).

step-import's instance loop (`crates/step-import/src/lib.rs:900-932`) does not. It moves each part itself, in one of two ways:
- `FinishedBody::moved` of a finished source;
- `topo::transform_rigid` of a built solid, then the per-part gate on the image when `topo::per_part_gate_owed`.

It then calls `topo::graft_disjoint`.

**The per-part gate reads the part, not the image.** Tier 3 is up to isometry apart from the session box, which the door asks of the image itself. So the gate can run on the built source before the graft, as the finished-source arm already does (finish the source, then move it), and the loop can graft through `graft_placed`.

What the fold must keep:
- the `StepImportError::Placement` and `TierInvalid { solid }` refusals, naming the solid;
- `finished`'s reuse of a source finished once for its later instances;
- `per_part_gate_owed`'s choice of whether a per-part gate runs at all.

A follow-up PR after 7099 (ONEHOME orchestrator, 2026-11-18).
