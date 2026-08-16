# M4 PR 2 binding spec — the evaluation service

Status: **LANDED — PR #83, merged 2026-07-24** (historical record;
review outcomes and rulings in docs/M4-LOG.md).

Orchestrator-authored binding spec (2026-07-23). Deviations REPORTED,
never improvised. Charter: M4-PLAN PR 2 under ratified forks F2
(result-DAG shape), F4 (node vocabulary); plus the two obligations
banked at PR 1's merge (M4-LOG): wrap `EvalError::NonFiniteResult`
with node/slot context; instantiate `Doc<P>` with the real profile
payload. Branch: `ev/m4-2-eval`.

## Scope

IN: editor-core's evaluation service — `evaluate(doc) →
Evaluation<T>` per F2; wiring the F4 node vocabulary to the existing
kernel ops; memoized incremental recompute on content keys;
cancelation epochs + cooperative yield points; the D9-addendum
rayon idiom-1 deployment over independent nodes (advisory —
correctness first, parallelism cited). NO naming layer (PR 3), NO
persistence (PR 6), NO solver (M6 — the witness slot stays empty).

## D1 — Dependency direction and Doc<P> instantiation (binding)

editor-core now depends on the op crates it evaluates (topo, sweep,
profile, geom-brep as needed) — G1 layering: editor-core is ABOVE
the kernel. `Doc<P>` instantiates with the profile crate's public
description type as the canonical `P` (a type alias `Doc = 
DocOf<ProfileDesc>`-style; PR 1's genericity stays for tests).
Kernel crates gain NO editor-core dependency (enforce by
workspace-dep direction; state it in rustdoc).

## D2 — Evaluation shape (binding, F2 verbatim)

```
Evaluation<T> = { order: Vec<RecipeNodeId>, nodes: Map<RecipeNodeId, NodeResult<T>> }
NodeResult<T> = Ok(NodeValue<T>) | Failed(NodeError) | Poisoned { through: RecipeNodeId }
NodeValue<T>  = { bodies: Vec<Body<T>>-shaped output (op-appropriate),
                  contacts where boolean (the BooleanBody contract),
                  name_table: RESERVED empty slot (PR 3 fills; type stub now),
                  witness: RESERVED empty slot (M6; type stub now),
                  content_key: the node's input-content hash (D4) }
```

- `order` = deterministic topological order (D9: a pure function of
  the DAG — document the tiebreak: RecipeNodeId ascending).
- Failure poisons DESCENDANTS ONLY (GQ2 ratified); independent
  subgraphs complete. `Poisoned.through` names the nearest failed
  ancestor (not the root cause — the chain is walkable).
- `NodeError` wraps the kernel's typed errors UNALTERED (no
  stringification) + the node/slot context — this is where PR 1's
  `NonFiniteResult` obligation lands: every expression evaluated
  during node evaluation is wrapped with (node, slot).
- Scalar-generic: `evaluate<T: Decide>` compiles and runs at f64 AND
  Interval (pin an Interval evaluation of a boolean-bearing doc).

## D3 — Node semantics v1 (binding; wire, don't invent)

Each F4 node maps to EXISTING public kernel ops — this PR invents NO
kernel behavior: Datum (evaluated frames/axes as values), Profile
(the wrapped description → profile crate validation), Extrude/
Revolve (sweep crate), Split (topo::split; BOTH parts are the node's
output, role-tagged), Union/Intersect/Subtract (topo booleans;
BooleanResult::Empty is a legal NodeValue — F8 typed success),
Transform (rigid placement applied as the existing machinery allows
— if no public rigid-transform op exists on Body, REPORT and land
the minimal one in topo as part of this PR with its own tests, or
propose deferral; do not bolt geometry hacks into editor-core),
Pattern (structural Count → N instances composed with Transform +
the downstream consumer, evaluated as data — patterns do NOT
implicitly union; the recipe says what consumes instances), Declare
(resolved to contact declarations ONLY as pass-through data in v1 —
threading into booleans is PR 5; evaluating a Declare node is a
no-op value carrying its pairs).

## D4 — Memoized incremental recompute (binding)

- Content keys per the banked principle: `content_key(node) =
  hash(op kind, structural params, evaluated expression values AS
  BITS, upstream nodes' content_keys)` — bit-exact floats in, so
  same key ⇒ same inputs ⇒ (D9) same output; the key IS the
  correctness proof. Hash choice: a stable, documented, seeded-fixed
  hasher (NOT the std random-seeded default — D9).
- The evaluator takes an optional prior `Evaluation<T>` (the memo):
  nodes whose content_key matches reuse the prior NodeValue without
  re-running the op; only downstream of changed keys re-evaluates.
- ACCEPTANCE (the M4-PLAN PR 2 criterion): edit one parameter
  mid-DAG on the die document → COUNT re-evaluated nodes (expose the
  count in Evaluation for tests) — only the downstream cone runs;
  the resulting body bit-matches a from-scratch evaluation (D9
  cross-check test).

## D5 — Cancelation + epochs (binding shape, minimal v1)

`evaluate` takes a cancel token checked BETWEEN nodes (cooperative
yield points at node granularity in v1 — intra-op yield is future
work, do not thread tokens into kernel ops); a canceled evaluation
returns a typed partial result (completed prefix + Canceled marker),
never a panic. Epoch = the evaluation's identity token carried in
`Evaluation` (GQ2's stale-result discrimination hook for
editor-core's future callers).

## D6 — Parallelism (advisory, cited)

Independent DAG nodes MAY run under rayon idiom 1 (indexed parallel
map into the per-node slots — the D9 addendum's sanctioned idiom;
cite it in the code comment). Determinism proof obligation: results
land by node ID, order is data not schedule; the D9 cross-check
test (D4) must pass with parallelism on and off (feature-gate or
thread-count env is fine). If sequencing pressure appears, ship
sequential v1 and REPORT — parallelism is not a gate.

## D7 — The die, evaluated (acceptance)

`crates/editor-core/tests/m4_pr2_eval.rs` (+ siblings): the PR 1 die
DOCUMENT now EVALUATES — final body volume exactly 7.8359375 (the
M3 oracle), tier-1/2 valid, at f64 (all ε rows via gate) and one
Interval evaluation; the incremental-recompute count test (D4); the
poisoning test (break a mid-DAG expression → descendants Poisoned,
independent subgraph completes); cancelation test; Empty-result
node test (disjoint subtract to ∅ as a typed success value);
Split-node both-parts test. Plus the PR 1 suites stay green
unchanged.

## D8 — Process (binding)

OUTPUT DISCIPLINE per convention (≤~150 lines/call; skeleton first;
report ≤120 lines). Branch `ev/m4-2-eval` from origin/main (must
include #81+#82, main ≥ `af5a94b`). Push after every commit. Gate:
`local-scripts/gate.sh <merged sha>` as ONE synchronous foreground call
(600000ms timeout; on harness cutoff READ the output file, do not
relaunch). NEVER export RUSTFLAGS (RUSTC_WRAPPER=sccache from first
build is fine). Open the PR ("M4 PR 2: evaluation service — result
DAG, incremental recompute, cancelation"); do NOT merge;
adversarial review follows.
