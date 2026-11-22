# BOXFINISH M9 — a refusal under its key, and the tour's leaf time outside the node ops (2026-11-22)

**The brief's premise first: the "~81 s outside `run_op`" is not there.** That figure was M8's subtraction of two different
runs (M7's 214.9 s of leaf time, taken with `BOXFINISH_TRACE=1` printing every finish while another job held a core, minus
M8's 134 s of node ops). Measured here in ONE run, with the probe's own time removed: leaf time 177.6 s, node ops 153.7 s,
outside them 24.0 s, of which 23.0 s is the symbolic session's teardown. The error was mine (M8, item 2).

Headline: on the tour every failed run of a node under one content key refuses with the same payload, the same escalations and
the same count delta (127 keys, 127 refusals, per hole node, over 1,023 runs each); the same holds for every node that
builds. Nothing per-replay leaks into what the drive receives from a node under one key.

## Tree, probe, how it was measured
- Same tree and switches as M8 (probe + `origin/main` ae076e0507, box arm off, `BOXFINISH_S1=1`), one more probe commit;
  `BOXFINISH_M8=1` now also prints `[m9-node]`, `[m9-leaf]`, `[m9-replay]`, `[m9-probe]`. Report commit: this file's.
- **A failed node's refusal as the drive receives it** is a `NodeError { node, kind, escalations }`: `eval_node`'s `fail`
  keeps `bracket.finish().escalations` and drops the frame's verdict rows and marks (read from the code). Hashed: the debug
  text of `kind` and of `escalations` (so the whole error value, enclosures and bands included), raw and with `SymId`s
  removed. **A built node's recorded side**: the debug text of its `verdicts`, `escalations` and `marks`.
- **The symbolic counts' delta over the node**: the session's `SymCounts` read at `eval_node`'s entry and after `run_op`
  (symbolic, numeric, retried, registrations refused / contradicted, theorems disputed, checks expanded / frozen). `frozen`
  is not a per-node quantity: the session rewrites it once at the leaf's end (`leaf_need`), so it is left out.
- Registrations and deferred entries: the drive's sessions are not deferred sessions (no entry exists), and a registration
  shows to the leaf only through the two registration columns above (inferred from the code; the registry itself was not
  hashed).
- A process-wide table keeps the first text seen under each (node, key) and prints the first differing bytes of any later
  one (`[m9-diff]`, `[m9-cdiff]`). None was printed in any drive.
- Time: per replay, the leaf's whole time, the session's setup and teardown (`sym::with_session_in`, before and after the
  closure), `evaluate()`, `classify_replay`, per node the time from `eval_node`'s entry to `run_op` (inputs, slots, the key),
  `run_op` and the finishes inside it; the probe's own printing is timed and subtracted.

## 1. One key, one refusal (all measured)
| drive | node | state | runs | distinct keys | distinct (key, refusal or record) | same with ids removed | distinct (key, counts delta) |
|---|---|---|---|---|---|---|---|
| tour | 29 (first hole's extrude) | failed | 1,023 | 127 | **127** | 127 | 127 |
| tour | 37 (second hole's extrude) | failed | 1,023 | 127 | **127** | 127 | 127 |
| tour | 20 (the plate's extrude) | ok | 1,023 | 1 | 1 | 1 | 1 |
| tour | 12, 77 | ok | 1,023 each | 1 each | 1 each | 1 each | 1 each |
| tour | 25, 33 | ok | 1,023 each | 127 each | 127 each | 127 each | 127 each |
| two-parameter | 21 | failed | 31 | 31 | 31 | 31 | 31 |
| two-parameter | 13 | ok | 31 | 7 | 7 | 7 | 7 |
| docm9 | 11 | failed 49, ok 129 | 178 | 178 | 178 | 178 | 178 |
| docm9 | 20 | failed 8, ok 121 | 129 | 129 | 129 | 129 | 129 |
| planted flip (`m10_3 a_planted_flip_refuses_flip_crossing..`) | 11, 20 | failed and ok | 315, 290 | 315, 290 | 315, 290 | same | same |
- **The tour is the only drive that tests the question.** Its hole extrudes repeat each key about 8 times (896 repeats per
  node) and every repeat carries a byte-identical refusal (the `SurfaceDatumUndecided` / `CurveDatumUndecided` findings with
  their enclosures and bands), the same escalation list and the same count delta. No key has two refusal hashes, so there is
  no difference to trace to a counter, a per-session id, a cap or a telemetry field, and nothing to re-hash without.
- In the two-parameter, docm9 and planted-flip drives a failing node never repeats a key (it reads every axis the drive
  splits), so they neither confirm nor contradict: 0 repeated keys among failed runs.
- OK nodes' recorded side: one record and one count delta per key wherever a key repeats (tour: nodes 12, 20, 25, 33, 77,
  5,115 runs over 257 keys; two-parameter node 13, 31 runs over 7 keys).
- Does not establish: a drive that certifies (the tour refuses at check 1 here); a node whose refusal depends on a retry cap
  reached earlier in the replay (`retried` is +0 in every delta here, so no retry ran); the f64 witness lane.

## 2. The tour's leaf time outside the node ops (one run, 1,023 replays, 4 workers, times summed over workers)
| part | time | scales with |
|---|---|---|
| leaf time, probe removed | 177.6 s | |
| `run_op` over all nodes | 153.7 s (finishes 59.5 s) | per node: what a store keyed by content serves |
| outside `run_op` | **24.0 s** (13.5%) | |
| .. the symbolic session's teardown | 23.0 s (22.5 ms per replay, max 75.8 ms) | per replay, and with the session's size (below) |
| .. `evaluate()` outside the nodes (schedule, environment, the box) | 0.5 s | per replay regardless of the box |
| .. nodes before their op (input and slot evaluation, the key, `member_key`) | 0.4 s | per node; needed to compute the key, so a store does not serve it |
| .. the session's setup | under 0.01 s | per replay |
| .. `classify_replay` and the verdict assembly | under 0.01 s | per replay |
| .. unattributed | under 0.1 s | |
- The K probe does not run here: the `probe` feature is not built in the tour or the test binaries (read from the code:
  `#[cfg(feature = "probe")]` in `Sym::decide_sign`).
- The teardown is `sym::with_session_in` after the closure: `publish_to_memo` (this leaf's plain forms, atoms and frozen ids
  into the drive's memo), `leaf_need` (the leaf's `frozen` column), and dropping the session's tables. It was timed whole,
  not split among the three. It is paid once per replay, so a 127-replay tree divides its count by eight; its size per
  replay follows what the session built, so a store that serves a node without re-running it would also shrink it
  (inferred: the three steps walk what the leaf recorded; not measured).
- So of the 177.6 s: 86.5% is node ops, which M8 put at 87% (build) and 91% (finish) on keys already seen; 13% is the
  session's teardown; under 1% is everything else.
- Run-to-run: this run's node ops sum to 153.7 s against M8's 134.0 s on the same binary modulo the trace (4 workers on 4
  cores, the trace's formatting inside the session). The shares are what to read, not the seconds.

## Not measured, and why
- The teardown's three parts apart (one more 14-minute rebuild; say if it matters).
- A certifying tour; a drive in which a retry runs; refusals at the f64 witness.
- The session's registry and form tables as state a served refusal would have to reproduce: only the counts were compared.
