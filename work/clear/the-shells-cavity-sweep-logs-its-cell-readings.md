---
id: the-shells-cavity-sweep-logs-its-cell-readings
kind: issue
title: The shell's cavity check runs the clearance sweep outside the unlogged scope the clearance wire keeps, so a shell node at Interval logs cell readings as the drive's
status: closed
opened: 2026-11-11
closed: 2026-11-12
branch: clear/verdict-door
priority: P2
cost: E
refs: [clearance-cell-enclosures-are-typed-as-the-interval-scalar]
---

Found by both designers on fork `clear-enclosure` (2026-11-11), reading the code; not yet run.

`topo::shell`'s `cavity_embedded` (`crates/topo/src/shell.rs:4615`) calls `lane.sweep` (`:4641`) with no `k_stats::unlogged` scope. `editor-core`'s `wire_clearance` wraps the same sweep in one (`crates/editor-core/src/eval/wire.rs:2635`, audit row F30 in `docs/predicate-dimension-audit.md`). F30 gives the reason for the scope: cell readings logged into a node's verdict bracket cue the subdivision driver to bisect a parameter box over cell pairs that only a smaller cell answers. On the dumbbell neck, drives that certified at once ran past ten minutes. Those are also rows the f64 witness build cannot have.

`eval_node` opens a verdict bracket per node (`crates/editor-core/src/eval/mod.rs`). So a shell node evaluated at `Interval` or `Sym<Interval>` would log the cavity sweep's cell readings and in-band escalations as its own.

**Done when:**
- a shell node at `Interval` is measured to log the sweep's cell rows, or shown not to;
- if it does, the cavity sweep runs in the same scope, with a pin that fails when the scope is dropped.

The fork's answer may move where that scope lives, into the engine's door or into a type, so build this after the fork is decided if the decision lands soon.

## Closed (2026-11-12, CLEAR's verdict-door lane)

Measured: a shell at `Interval` did log the sweep's cell rows. With
the scope dropped, the tube's shell logs 1,027 of them and the vessel's
588. The fork settled where the scope lives (decision 3, Ev on PR
6821). `Sweep::run` and `min_separation` now run under
`k_stats::unlogged("F41")` themselves, so the cavity gate keeps no
scope of its own. The pin is `shell_cavity_embedded`'s `holds`, for
the tube and the vessel at `Interval`. It asserts that the build's log
holds none of the engine's cell readings, and it reds when the scope
is dropped.
