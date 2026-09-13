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
in the scheduled arm). Two families of thread-local state do not cross to the worker, and
they are not the same kind of defect: one loses a RECORDING, the other
changes a DECISION.

**1. The `probe` sample population and the shape report are lost; the
verdict log is NOT.** `geom_core::k_stats` records verdicts and
escalations into the open `Bracket`'s frame and, under `probe`, samples
into the installed sink — both thread-local. The verdict channel is
safe here because `eval_node` opens its OWN bracket INSIDE the mapped
closure (`eval/mod.rs`'s `let bracket = geom_core::k_stats::Bracket::open();`
at the top of `eval_node`, whose comment already says "runs whole nodes
on one worker each") and returns the `Recorded` on the node, so a node
evaluated on a worker records into a frame of its own on that worker and
hands it back as a value. That is one of the two sound shapes
`k_stats`' module docs now name.

What is NOT handed back is everything that stays in a thread-local: the
`probe` SINK, which `start_recording` installs on the caller's thread
and `take_samples` reads there — so the margin population
`docs/K-REPORT.md` and `tools/k-lint` are computed from shrinks with
the thread count rather than being a property of the document — and
`geom_core::sym::report`'s `ACTIVE`/`SHAPES`/`NAMES`, which the
`Sym` evidence rows install around a run.

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
those nodes' decisions. Sharing the session across workers is not the answer either, and the
reason is NOT the node ids: `intern` is a content hash of the node, so
the DAG a replay builds is the same whatever order it is built in. What
is order-dependent is the rest of the session — `OPAQUE_SEQ`, a
per-replay counter whose determinism rests on the minting ORDER being a
fixed single-threaded walk, and the `SymCounts` receipt, which
`count_decision` mutates in place — and `Session` is a `RefCell` behind
a thread-local, so there is no shared-access story at all.

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
sample population is the serial schedule's at any thread count. The
verdict log needs nothing — `eval_node`'s own bracket already covers it
— and a `detached` frame around it would simply nest, which is defined.

For (2), either open the session per NODE on the worker (drive.rs's
shape, but a per-node table is not a per-leaf table and the counts and
ids change), or refuse the parallel schedule while a session is
installed — the guard `topo::props`' face walk uses
(`decisions_are_thread_portable`), which reads
`geom_core::sym::session_counts()`. Which one is right is a design
question about what a session's table is scoped to, not a code change to
pick off.
