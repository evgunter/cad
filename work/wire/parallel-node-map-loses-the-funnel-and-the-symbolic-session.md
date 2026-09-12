---
id: parallel-node-map-loses-the-funnel-and-the-symbolic-session
kind: issue
title: the evaluator's parallel node map loses the K-funnel's recordings and the symbolic session
status: open
opened: 2026-09-12
---



## The finding

`evaluate`'s level-synchronous schedule runs a level's nodes as a rayon
indexed parallel map when `EvalOptions::parallel` is set
(`crates/editor-core/src/eval/mod.rs`, the `par_iter().map(eval_node)`
in the scheduled arm). Two pieces of state a node reads are
THREAD-LOCAL and neither crosses to the worker.

**1. The K-funnel's recordings are lost.** `geom_core::k_stats` records
verdicts and escalations into the open `Bracket`'s frame and, under
`probe`, samples into the installed sink — both thread-local. A node
decided on a worker records into that worker's stack and sink. The
evaluator brackets every node evaluation (NAMING-DESIGN N5, so the
verdict-diff engine can attribute flips), so what is lost here is
exactly the artefact that machinery reads. PERF-8 added the door that
composes them back (`k_stats::detached` / `k_stats::splice`); the same
class on this repo's other rayon maps is
`work/perf/rayon-maps-outside-props-lose-the-funnels-recordings.md`.

**2. A symbolic session does not cross, and that changes DECISIONS.**
`geom_core::sym`'s session is thread-local and per-call: `with_session_
rules` installs it on the calling thread, and `Sym`'s `sign_within`
consults it — with no session installed `discharge` answers `None`, the
identity tier discharges nothing, and the decision is the plain numeric
one. `read_leaf`'s `LeafLane::Symbolic` arm opens a session and then
calls `evaluate::<Sym<Interval>>` with the caller's `opts`, so a node
run on a worker under that arm decides WITHOUT the tier while its
siblings on the calling thread decide with it — a schedule-dependent
verdict, which D9 forbids outright, plus a `SymCounts` receipt short by
those nodes' decisions. The session's hash-consing table cannot simply
be shared either: node ids are minted into it, so a shared table would
make them schedule-dependent.

`drive.rs`'s leaf map does not have this defect — it opens the session
INSIDE the leaf, on the worker that runs it, which is the shape a fix
here has to reach.

**Latent, not live.** `EvalOptions::default()` sets `parallel: false`
and every shipping caller takes the default; `parallel: true` appears in
tests. So no run takes a wrong decision today. What is filed is that the
switch cannot be turned on as it stands — which is what `plan.md` §2.2
invites a lane to do.

## What a fix is

For (1), `k_stats::detached` per node on the worker and `splice` in the
sequential fold that already walks the results in level order, so the
verdict log is the serial schedule's at any thread count.

For (2), either open the session per NODE on the worker (drive.rs's
shape, but a per-node table is not a per-leaf table and the counts and
ids change), or refuse the parallel schedule while a session is
installed — the guard `topo::props`' face walk uses
(`decisions_are_thread_portable`), which reads
`geom_core::sym::session_counts()`. Which one is right is a design
question about what a session's table is scoped to, not a code change to
pick off.
