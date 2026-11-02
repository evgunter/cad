---
id: needs-enclosures-recourse-sends-the-reader-to-a-parameter-box-and-a-type-path
kind: issue
title: MeasureUnavailableAt's recourse sends a reader to 'a parameter box' and names a type path, and its two arms name two doors
status: open
opened: 2026-11-01
priority: P4
cost: E
---


Found by the clearance-consumer designers (DOORS fork, `[ev]` PR 6261, 2026-11-01).

`MeasureUnavailableAt`'s two arms (`crates/editor-core/src/eval/wire.rs`, the `min_clearance` arm) disagree on which door answers:
- `NeedsEnclosure` names `topo::ClearanceLane`;
- `NoUpperBound` names `topo::clearance::min_separation`.

The recourse also says to evaluate "over a parameter box". At nominal no box is needed: a point box suffices, and that is the reading the GUI consumer will take.

This is reader-facing text, so it should name the action and not a type path. If PR 6261's design lands, the action is the certified reading, which the GUI then performs on its own. That leaves the sentence for API readers. (DOORS orchestrator)
