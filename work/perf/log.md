# PERF log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/perf/plan.md`. A/B band 3400–3499
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening for work (2026-09-10)

PERF has been a register since 2026-07-21 (no orchestrator, no units).
Ev opened it for work in-chat on 2026-09-10 with these rulings, which
this entry is the record of:

- **Territory.** Few other orchestrators are running, so PERF lands
  fixes in whichever program's territory holds the cost and records the
  seam here; the owner's dispatched/review rows are checked first, and
  very new code is deprioritized (it will change anyway, and a perf
  change on it is work done twice).
- **Developer-side pain is in scope** (debug profile, CI, test wall
  time), not only the GUI and kernel-API seats.
- **"Stop doing this" beats "do this faster"** whenever both are
  viable — but a change of that kind that removes work a ratified
  clause asks for (D1's per-op tier-1 postcondition is the standing
  example) is an `[ev]` PR before it lands, not after.
- **Review posture: full v6 dual** on every kernel unit unless the
  orchestrator judges a unit unusually low-risk and says so in its row;
  measurement-only and instrument-only units record no row.
- **Merging.** The orchestrator merges its own units after the dual
  concludes (Ev, in-chat).
- **Ev's own pain point:** the GUI lags after edits, location unknown.
  That is the first thing the exploration wave measures.
- **Off-box measurement is allowed**: temporary GitHub Actions
  workflows may be used to take quiet timings while this 4-core box is
  running several lanes.

Opening commit: `prefix: perf/`, `tag`, band 3400–3499 claimed in
`docs/MODEL-AB-LOG.md`, this log. Exploration wave dispatched next
(Opus lanes, per Ev's budget steer): a GUI-seat lane on the edit→repaint
path, a kernel-seat lane over the demos and the Python binding, a
developer-seat lane over the debug/CI profile, and a static lane
re-verifying `plan.md` §1.3 against today's tree and the tracker's
already-filed perf findings. Each finding becomes one item file here.

## PERF-8: a composing door on the K-funnel (2026-09-12)

**Announcing a `geom-core` edit in PROPS' territory** (`work/README.md`:
the owner is told where the change lands, and this is that telling).
`crates/geom-core/src/k_stats.rs` gains two doors and the type between
them — `detached`, `splice`, `Detached` — and nothing else in that file
changes behaviour. `Bracket`'s semantics, nesting included, are
untouched; `detached` is built out of `Bracket` rather than beside it.

Why it had to be there: the funnel's recording is thread-local
(`FRAMES`, and `SINK` under `probe`), so a face decided on a rayon
worker records into that worker's frame and sink. `detached` runs a
closure on the current thread under a frame and sink of its own and
hands back what it recorded; `splice` appends such a recording to the
current thread's open frame and sink. A walk that maps faces onto
workers and splices in arena order therefore produces the serial walk's
verdict log, escalation log and sample population, element for element,
at any thread count.

`crates/topo/src/props.rs` is the first consumer (`mass_properties` and
`sign_certified`); `topo` gains the workspace `rayon`. The same door is
what the other four rayon maps in the tree need —
`work/perf/rayon-maps-outside-props-lose-the-funnels-recordings.md` and
`work/wire/parallel-node-map-loses-the-funnel-and-the-symbolic-session.md`.
