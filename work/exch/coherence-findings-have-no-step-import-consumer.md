---
id: coherence-findings-have-no-step-import-consumer
kind: issue
title: the step-import diagnostics half of the coherence consumer: examine_chart_coherence at the door where defective source coordinates actually arrive
status: open
opened: 2026-09-12
refs: [2408, 1585]
---



(FIX orchestrator, 2026-09-12) **The split half of FIX's
`coherence-findings-have-no-consumer`**, filed here because that item
closes with PR 2408 and this half would otherwise survive only as a
sentence.

`work/exch/plan.md:57-59` already names it as block E work — *"the
step-import diagnostics half of FIX's
`coherence-findings-have-no-consumer`"* — and that is the whole of its
tracker existence today. `work/README.md` is explicit that **the
re-homing sweep sees items, not sentences**: a rider in a plan is
invisible to it and dies with the directory. One-file-one-item also
means FIX's file cannot close while carrying work EXCH owns. Hence a
file, on the slate of the program whose ground it is.

**Nothing is owed to FIX by this.** The `CheckId` half landed in PR
2408 and FIX's row closes on it.

## What the half is

`topo::coherence::examine_chart_coherence(body, tol) -> CoherenceReport`
measures three input-quality conditions that used to be `debug_assert!`s
in `mesh::walk` — loop closure, rim and meridian continuation — each a
gap against a lever arm in metres against ε. MESH-8 landed it with zero
production callers and named two consumers. Step-import is the second,
and the argument for it is that **it is the door where defective source
coordinates actually arrive**: issue 723's half-cap is the recorded
π-rad witness through import. `step-import` already depends on `topo`,
so no new edge.

## What PR 2408 established that this half inherits

Read that PR before scoping this one; it answers questions this row
would otherwise re-ask.

- **`skipped` is CONFIGURATION and `unexamined` is DATA**, and the two
  must reach a user distinguishably. MESH-8's review flagged the shape
  collision (`CoherenceReport { findings, unexamined }` against
  `ChecksReport { findings, skipped }`); 2408 kept them apart with a
  pin asserting the two reports share **not one word**. Whatever
  step-import's diagnostic type is, it inherits that obligation.
- **The door is `f64`-only.** `examine_chart_coherence` takes
  `&Body<f64>` while generic consumers are parameterised by the
  decision lane, which forced a `ChartCoherenceLane` capability trait
  in 2408. Establish early whether step-import's path is monomorphic
  at `f64` — if it is, this half is markedly cheaper than that one.
- **The examination has no shape door**, so an ordinary trimmed
  cylinder puts two loops in `unexamined` as a lane boundary rather
  than a defect (`Unexaminable::NonIsoCarrier`). 2408 shipped its
  resident default-`Off` for that reason. A step-import diagnostic
  that fires on every cylinder is the same hazard — `mesh8_corpus_coherence`'s
  own header states the rule: *"A report that fires everywhere is not
  a report."*
- **The kernel's coherence types have no `Display`**
  (`CoherenceFinding`, `Unexamined`, `Unexaminable`, `StructureRead` —
  MESH's `crates/topo/src/coherence.rs`), so 2408's consumer writes the
  condition phrases itself and reaches a user with `{at:?}` for
  `StructureRead`. Readable today, but a second consumer makes it a
  second hand-written vocabulary — which is the
  `tier-3-prime-findings-render-through-debug` shape. Worth raising
  with MESH rather than copying.

## Refs

FIX's closed row (PR 2408), MESH-8 (PR 1585), issues 868, 723, 1571.
