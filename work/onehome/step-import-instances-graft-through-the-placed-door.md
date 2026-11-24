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

- 2026-11-24 — one more step behind the same hand-built loop (the gather-carries lane's sweep): a repeated part is finished once (`gate`) and each later instance is that finished body moved (`topo::FinishedBody::moved`, its verdict carried), then given back as construction state (`.into_body()`), grafted with `topo::graft_disjoint`, and the whole file's body finished again by the shared gate, so every instance's verdict is derived twice at `f64`. `topo::gather` (the gather-carries PR) is the door that grafts finished placed bodies and carries their verdicts; through it, the import's last gate would derive only what the gather does not carry. No verdict moves at `f64`, so the cost is the finish repeated. (gather-carries lane)
