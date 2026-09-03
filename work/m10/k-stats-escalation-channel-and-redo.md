---
id: k-stats-escalation-channel-and-redo
kind: issue
title: k_stats: an escalation channel beside the verdict log (and the redo that channel is already owed)
status: open
opened: 2026-08-29
github: 1254
refs: [1231]
---

## From GitHub issue 1254

opened 2026-08-29, 0 comments.

## What

`geom_core::k_stats` has a verdict log — `start_verdict_log` / `take_verdict_log` — that records every DEFINITE outcome through `classify`. It has no counterpart for the INDETERMINATE outcomes, which are exactly what `classify`'s `Err` arm produces and are never recorded anywhere.

The consequence, first hit by M10-3 (PR #1231): the E6 subdivision driver's leaf protocol needs "was every predicate in this leaf definite" as an observable fact. The only way to ask today is to walk the typed error a node failed with and recognise the escalation-carrying arms by hand — and escalations arrive wrapped inside each op's own error enum. `grep -rn 'source: Indeterminate\|cause: Indeterminate\|(Indeterminate)' crates/*/src` finds roughly 40 such variants across `sweep`, `topo`, `profile`, `geom-brep` and `editor-core`.

So the driver recognises the two arms it can prove — `NodeErrorKind::Escalated` and the profile lift's guided `StructureRefusalKind::Indeterminate` — and treats every other node failure as the conservative bisect cue. That is sound (it can never produce a false certificate or a false flip; it costs refinement and lands as `Budget` at the floor) but it is lossy: the ratified PR-7 terminal-sliver semantics can only fire on escalations that surface through those two arms. A sliver reaching the driver as `ExtrudeError::ExtrusionEscalated` is priced `Budget` instead of `SliverTerminal`, which is a worse answer for the same mass.

## Why it is not just "add another thread-local"

`k_stats`' own module docs put the verdict log on notice, in as many words:

> **OPEN OBLIGATION — this mechanism is on notice; see `docs/PERF-SCAN-2026-08.md` §2.** Delivering a production value by thread-local side effect makes the per-node bracket's correctness a comment rather than a type, and it has already failed once … Do not add call sites that deepen the dependency on the current shape.

An escalation log built the same way would be exactly such a call site — and it would inherit the same nesting bug (`start_verdict_log` overwrites an installed log unconditionally, so a nested evaluation destroys its parent's).

So this issue is the escalation channel AND the redo the verdict log is already owed, together: the two are one mechanism and should not be built twice.

## Shape to decide

1. **Verdicts as a returned value** (the obligation's own first option), with escalations riding the same return. Removes the thread-local for both, makes the per-node bracket a type rather than a comment, and fixes the nesting bug on the way.
2. **An RAII bracket** with re-entry refused loudly and thread confinement enforced, carrying both channels.

Either closes the driver's gap; neither should be attempted for escalations alone.

## Acceptance

- A leaf evaluation can answer "did any predicate escalate, and what was the `Indeterminate`" without matching on op error enums.
- `editor_core::drive`'s `classify` uses it, and `RefusalReason::SliverTerminal` fires on escalations wrapped in kernel error enums (a fixture in `m10_3_driver_interval.rs` currently reads `Budget` where `SliverTerminal` is the true class).
- The verdict log's nesting bug is fixed or explicitly re-scoped.

Referenced from `editor-core/src/drive.rs`'s `classify` and from PR #1231's deviation 3.

## Home

`work/m10/` — `crates/editor-core/src/drive.rs` is an M10 territory glob and the gap is the E6 subdivision driver's, raised by M10-3.
