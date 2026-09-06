---
id: k-stats-escalation-channel-and-redo
kind: issue
title: k_stats: an escalation channel beside the verdict log (and the redo that channel is already owed)
status: closed
opened: 2026-08-29
github: 1254
refs: [1231]
branch: props/kstats-bracket
pr: 1969
closed: 2026-09-05
---

## From GitHub issue 1254

Opened 2026-08-29; 0 comments.

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

## Closed (2026-09-05, #1969)

**The bracket with a stack, the returned value declined in writing.**
Measured: 530 `decide*` call sites in 261 enclosing functions (104
public) across seven crates, plus `Decide::sign_within`'s five impls —
the sink a returned value would thread. `k_stats::Bracket` replaces
`start_verdict_log` / `take_verdict_log` (deleted; seventeen caller
files converted): a thread-local stack of frames, each carrying a
per-thread unique id its guard remembers, so an out-of-order close is
DEFINED identically in every profile (the outer takes its own frame and
discards the inner's; a stale guard pops nothing and returns empty —
never another bracket's decisions); `!Send` by a `*const ()` phantom,
pinned as `compile_fail,E0277` beside a legal twin; `Drop`-popped, so
an early return or a panic unwinding through an op leaves no frame.
The nesting defect is fixed by construction and pinned: an instantiate
node's log is its own op's decisions (466 on the fixture), the same on
a part-cache hit and a miss, under both schedules, part-in-part (922),
through a node-memo hit and after a cancelled run; the part's own 724
are on the part's nodes.

**The escalation channel.** `classify` records every indeterminate
outcome it produces as an `Escalation { source: Indeterminate }` in the
same frame, in decision order; `NodeValue::escalations` and
`NodeError::escalations` carry it (neither persisted);
`drive::classify_replay` reads a definite box-independent refusal
first, the log second, the error-enum arms third. The named fixture
flipped: the planted flip's in-band strips price `SliverTerminal`
(22.5 %) naming `extrusion_normal_component` where they were `Budget`.

**Acceptance, narrowed where the code narrows it.** "A leaf evaluation
can answer 'did any predicate escalate, and what was the
`Indeterminate`' without matching on op error enums" holds for FUNNEL
predicates only: an op that asks the funnel, gets a definite sign and
mints its own `Indeterminate` (eight `geom-brep` sites), the two raw
`sign_within` calls, and the whole-document mate solve reach a consumer
only through the error enums, whose two arms in `classify_replay` are
therefore load-bearing. Pinned by name (`geom-brep/tests/kstats_escalation_channel.rs`,
`asm_r2a_mate_solve::row7e`) and filed as
`escalation-channel-misses-op-minted-indeterminates`; the arm-deletion
sweep is `indeterminate-error-arms-sweep`; the part's dropped per-node
logs are `part-per-node-logs-dropped-with-nested-evaluation`; the
bracket's scope (`work/issues/bracket-scope-is-run-op-not-the-node`)
and the coincidence zone priced `Budget`
(`work/m10/coincidence-zone-priced-budget-at-the-floor`) are the two
findings outside PROPS. Deviations argued in the PR: `NodeError`
carries the channel; one shielding bracket on the part cache's miss
path; `Ok` nodes with escalations bisect (zero in the corpus); the
M10-6 accounting goldens and M10-7's tier-off copies re-cut for the
class the acceptance moves.
