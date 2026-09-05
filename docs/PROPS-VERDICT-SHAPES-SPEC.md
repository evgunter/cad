# PROPS verdict-shapes — one module for the two derived verdict forms, one outcome enum, and the pin the split never had

**Binding at dispatch** (PROPS program, `work/props/plan.md` §Verdict
recording; the item is `work/props/three-per-node-verdict-shapes.md`;
difficulty logged at spec: **E/S**). An E rider under the plan's review
posture: single style review, outside the A/B experiment (no ordinal, no
block slot — S-TCOST's and FILLET's precedent for non-dual units). Read
`docs/prompts/implementer-discipline.md` in full before starting. Branch
`props/verdict-shapes`, cut from `main`.

## The decision (the item's three questions, answered)

The tree carries three shapes for "the verdicts a node recorded" over one
substrate: `NodeValue::verdicts` (the raw ordered `Vec<Verdict>`,
`eval/mod.rs`), `NodeVerdicts`/`VerdictSummary` (per-predicate sign
populations, serializable, `resolve/vdiff.rs`) and
`VerdictVector`/`VerdictRow`/`VerdictVectorKey`/`ReplayOutcome` (ordered
rows with an outcome tag, hashable, `drive.rs`). The strict/permutation-
invariant split is deliberate and stays — `drive::VerdictVector`'s own
doc argues it and the argument holds (certification wants the strictest
test; flip naming wants one that survives permutation; neither subsumes
the other). What does NOT hold is the split living in two modules with
two spellings of one outcome enum, and the split's central claim having
no executable pin. So:

1. **`VerdictVector`, `VerdictRow` and `VerdictVectorKey` move to
   `resolve/vdiff.rs`**, beside `NodeVerdicts`/`VerdictSummary` — the
   two derived forms over one substrate in one module, so a third shape
   has a place to be argued against rather than a gap to be minted into.
   `drive.rs` keeps its consumers (`classify`, `classify_replay`,
   `certifying`'s assertion-row drop is a `drive` policy and stays a
   `drive` function or an inherent method whose doc names the policy —
   your call, argued). Public paths: keep every path the surface census
   (`crates/pncad/tests/all.rs`) and `lib.rs` re-export today resolving,
   via `pub use` in `drive` — the census roster is a list of names, and
   the unit's diff shows whether a name moved family; if the roster's
   own doc places these under "the analysis lane's INTERIOR residue"
   and that sentence becomes false, amend the sentence, never the
   count.
2. **`ReplayOutcome` folds into `RunStatus`.** The two enums are built
   from the same `NodeResult` discriminants (`drive.rs:421-430`,
   `vdiff.rs:187-192`); the only informational difference is
   `RunStatus::Absent`, which `ReplayOutcome` folds into `Poisoned` with
   a comment. `VerdictRow.outcome` becomes `RunStatus`, built by
   `vdiff`'s one `status` function; an absent node is `Absent`, not
   `Poisoned` — strictly MORE distinguishing for the strict test, which
   is the direction that test is allowed to move. Key tag bytes: keep
   `Ok = 1`, `Failed = 2`, `Poisoned = 3` so no existing key moves, and
   `Absent = 4`. Any golden `witness_vector` / `certified … key=` hex
   that moves (drive goldens, stackup goldens) moves ONLY because a
   fixture had an absent row folded into poisoned before — say which,
   and re-derive it (discipline §3; never adjust to restore a number).
   `RunStatus` gains no serde change; `VerdictRow` gains none either
   (the vector is derived, never persisted — E10).
3. **`VerdictSummary` is the only persisted shape, and the other two say
   so**: one sentence at `NodeValue::verdicts`'s doc and one at
   `VerdictVector`'s, pointing at `verdict_summary` and the strict
   codecs in `persist/strict.rs`.
4. **The pin the split never had.** The census found the strict-vs-
   population claim asserted in prose only
   (`m10_3_driver_interval.rs:889-892` reverses rows and re-checks the
   witness key, never calling `diff_verdicts`). Add two rows in
   `crates/editor-core/tests/` (a new file or `m4_pr4_diff.rs`): (a) two
   logs that are a permutation of one node's verdicts — `diff_verdicts`
   reports NO flip while the two vectors' keys DIFFER; (b) two logs
   where a pure sign exchange between two predicates in one node cancels
   in the populations — again no flip named, keys differ. Red-first in
   the honest sense: write the rows against the current tree first and
   quote that they already pass (the split is real today); they are the
   contract's executable statement, not a regression.

**Not this unit:** the escalation channel and the thread-local redo
(`k-stats-escalation-channel-and-redo` — the next unit in this lane;
this one must not deepen the dependency on `start_verdict_log` /
`take_verdict_log`, per `k_stats.rs`'s OPEN OBLIGATION paragraph; touch
neither); `FlipEvidence`'s ladder; any change to what `classify` gates on.

**Fence:** `crates/editor-core/src/{drive.rs,resolve/vdiff.rs,resolve/mod.rs,lib.rs,eval/mod.rs (one doc sentence),stackup.rs (an import path)}`,
`crates/editor-core/tests/` (the rows above and the paths the move
forces), `crates/pncad/tests/all.rs` (roster). `drive.rs` is M10's
territory glob, edited here by ANNOUNCED SEAM (`work/m10/log.md`, the
PROPS entry): the type block and its `use` lines only; M10-8
(`origin/m10/m10-8-arc-family`, open) edits `drive.rs` elsewhere
(`SymbolicDials`, the `decisions` report line) — re-merge `main` before
every push and read that branch's diff before you start so your
hunks do not sit on its lines.

## Posture

- No `CI-Config:` trailer; hosted CI is the verification of record.
- ε posture: none — no float moves; every change is structural. Say
  so in one line.
- D2-addendum: `ReplayOutcome` is retired (row 0 — the state "a second
  spelling of the outcome" is gone; no consumer outside `drive.rs`
  matched on it — the census found two match sites, both in `drive.rs`,
  and the `pncad` roster names it; list the roster edit).
- Landing: the item `work/props/three-per-node-verdict-shapes.md` gets
  `pr:` and `status: review` on this branch; the spec is deleted at
  merge per the spec lifecycle (the orchestrator's branch holds it).
  No `Co-Authored-By` trailer in lane commits; push early to
  `props/verdict-shapes`.

## Acceptance

- One module holds both derived forms; one outcome enum; every public
  path still resolves; the roster amended by moved-family only.
- The two pins committed and green in both lanes on hosted CI; every
  moved golden key named with the fixture's absent row as its cause.
- The three docs say which shape persists.
