---
id: no-fused-verb-takes-a-departure-tangent-arc-toward-a-point
kind: issue
title: paths: tangent_arc_to is a verb outside ArcData, so no fused verb takes a departure-tangent arc toward an authored point (sharp→filleted is not an edit at the same call site)
status: open
opened: 2026-10-19
priority: P2
cost: M
design: true
---


Found while weighing the arrival-constructed arc (`arc-closer-constructed-from-arrival-tangent`), by both designers independently; a sibling of that question, not part of it.

**The gap.** `tangent_arc_to(p)` is a verb (`crates/profile/src/path/program.rs:1297`, `verb TangentArcTo`), not an `ArcData` mode, so the fused fillet verbs, whose incoming spec ranges over `ArcData`, cannot take it. An arc that departs tangent to the bound direction and ends at an authored point has no `arc_fillet(<tangent to p>, r)` spelling, and `.tangent_arc_to(p).fillet(r)` is not that fillet: bare `fillet(r)` after a leg is ray extension off the arc's end (§2c round 10), not a fillet on the arc's carrier.

**What it breaks.** `docs/PATHS-DESIGN.md` §2c's family list (round 7): "Plain `arc_to(spec)` remains for SHARP-cornered arcs; converting sharp→filleted is an edit at the same call site." That holds for every `arc_to` mode and fails for the tangent-departing endpoint-full arc, which §2 Legs keeps as a verb on purpose ("`tangent_arc_to(p)` stays the tangent-departing endpoint-full form").

**Shape of the question.** Whether the tangent-departing endpoint-full arc becomes an `ArcData` mode from a Directed tip (it is DOF-complete there, as `Sweep`/`ArcLen` are, so round 9 would ship its fused rows), or the round-7 property is restated to exclude it. The arrival `[ev]` PR's `arc_onto(target)` (§2a item 6) shares the gap, since an `arc_onto` arc filleted at its own end has no spelling either; it is not a mode, and a fused arrival direction would be met mid-arc, so it stays outside whatever this row decides. A vocabulary change to ratified §2/§2c text, so it is an `[ev]` question once designers have weighed it.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILEREST (`work/profilerest/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam (PROFILE, PROFILEREST, PROFILETAIL, PROFILEPROSE; `docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.
