# BOOL-7 — issue 134: the vdiff shadow-exec rung

**Binding at dispatch** (S-BOOL program, `work/bool/plan.md`; difficulty
logged pre-draw: **M**). Read `docs/prompts/implementer-discipline.md`
in full before starting. The primary specification is Ev's standing
option-(a) ruling (2026-07-29, the PR 8 N5 diagnosis question) as
routed by the Q3 ruling (Ev, in-chat, 2026-09-01: S-BOOL takes it, M10
dormant, keep-outs sharp) and the issue item
`work/bool/vdiff-pruned-pair-shadow-exec-rung.md` (issue 134).

## Situation

`crates/editor-core/src/resolve/vdiff.rs` diffs two evaluations' verdict
logs per node by per-predicate sign POPULATIONS (permutation-invariant;
module docs). A `Vanished` name's diagnosis (`resolve/mod.rs`, the
ladder) diffs the last-good run against the current one and, when the
`FlipSet` names a flip on the derivation path, reports
`Diagnosis::PredicateFlip`. The C10 sweep legitimately PRUNES candidate
pairs, so a pruned pair's predicates record no verdicts at all in the
run that pruned them; when the vanish is caused by such a pair the
`FlipSet` is empty (the population never existed) and the diagnosis
falls to the later rungs — the recorded-qualifier delta or
"cause not in evidence". Option (a): when the engine hits an EMPTY pair
population on a vanish, SHADOW-EXECUTE exactly the vanished pair's
predicates from the prior evaluation's context (both sides as needed)
and diff THOSE — bounded, diagnosis-time-only, recovering the full
`PredicateFlip`. Recording pruned-pair pseudo-verdicts in the log stays
ruled OUT (it would put −51% more entries in the corpus log for a
diagnosis path).

## FIRST, before the build — two measurements, reported

1. **The empty-population case, reproduced.** A document whose edit
   vanishes a name through a pruned pair: build it (the C10 sweep's
   pruning predicate and the pair vocabulary — `name_frag_side_of`
   probes between fragments — are the place to look), show the current
   diagnosis is the qualifier-delta or the cause-not-in-evidence rung,
   and show the `FlipSet` is empty for the node. This is the red-first
   row.
2. **What "the vanished pair's predicates from the prior evaluation's
   context" IS, concretely**: which node, which predicate names, what
   inputs the prior evaluation holds for them (the `Evaluation`'s node
   values, the recipe node's authored data), and whether re-running
   exactly those predicates is possible WITHOUT re-running the op (a
   pure function of the prior context — D9 replay determinism) or needs
   the op's construction replayed up to the pair. Report the bounded
   shape you will build: what is executed, from what, how many
   predicates, and the ceiling. If the honest shape needs M10's Dual
   arms (`product.rs`) or the `AtRestPolicy` seam, STOP and report — the
   Q3 keep-outs are hard.

## Deliverables

1. **The rung**: in the `Vanished` ladder, after the population diff
   reports no flip on the path and before the qualifier-delta rung, a
   shadow-exec rung: re-evaluate the vanished pair's predicates against
   the prior context and the current context (as the measurement
   states), diff the two small populations with the SAME
   `diff_populations` core (built once — no second diff), and report a
   `PredicateFlip` carrying a marker that it was recovered by shadow
   execution (a typed field, so a reader knows the flip was not in the
   log). Diagnosis-time only: nothing is written to any log; the rung
   runs only on a vanish with an empty population; bounded by the
   pair's own predicate count (state the ceiling as a constant with its
   reason).
2. **Rows**: the reproduction from measurement 1 now yields the
   `PredicateFlip` (red-first); a vanish whose population is NOT empty
   never enters the rung (the rung is unreachable there — a row proves
   the ladder order); the shadow-exec reads only the prior/current
   contexts (a row with a poisoned op body shows the rung does not
   replay the op — if it must replay, say so and pin the bound); the
   `SetTolerance` audit path is bitwise unchanged; the corpus's
   diagnosis outputs unchanged except the recovered case.
3. **N5's text**: `resolve/mod.rs`'s "Low-evidence diagnosis" paragraph
   and vdiff.rs's blind-spot paragraph re-record: the front door that
   "does not exist yet" exists; the cancelling-exchange blind spot
   stays (the rung does not address it — say so).
4. **Keep-outs proven**: `git diff --stat` shows `product.rs` and the
   `AtRestPolicy` seam untouched; M10's Dual arms untouched.
5. **D9 / behaviour**: no evaluation result moves (diagnosis is
   read-only over evaluations) — the corpus replay digest identical;
   the Python suite's diagnosis strings unchanged except the recovered
   case (if a string changes, the census row moves with it, stated).
6. **ε posture**: no new comparand — the rung re-runs existing
   predicates at the run's own band; state it.
7. **Class sweep** (discipline §5): other rungs in the ladder that rest
   on "the population never existed" (the `SetTolerance` audit's
   empty-set case; the cross-process `diff_summaries`) — measure,
   report, do not act.

## Acceptance

Both measurements reported first; the pruned-pair vanish diagnoses as a
recovered `PredicateFlip`; the ladder order proven; keep-outs proven by
diff; corpus digest identical; hosted CI green; gate record per head.

## Hard rules

- NO `Co-Authored-By`, no model names; no closing keywords; "issue 134"
  spelled out (the orchestrator closes the item).
- Scope fence: `crates/editor-core/src/resolve/vdiff.rs`,
  `crates/editor-core/src/resolve/mod.rs`'s `Vanished` ladder and
  `Diagnosis` payload, their immediate callers, the resolve test suites,
  `crates/pncad-py` census rows only if a diagnosis string changes.
  NOT: `product.rs` (M10's Dual arms), the `AtRestPolicy` seam, the
  evaluation engine, the verdict LOG format, the C10 sweep's pruning
  itself. `crates/editor-core` is SMELL Track V fence ground — disclose
  any row's file reached.
- Merges on green after the dual (the approach is ruled; the marker
  field's shape is stated, not asked).
- Re-merge main before opening the PR.
