# M10-2 — Measure nodes and Assertions (unit spec)

**Status: BINDING at dispatch** (orchestrator-authored; the design
is ratified — `docs/ERROR-DESIGN.md` E3/E10 and CONTACT-DESIGN C5 —
and this spec's elaborations are marked where they are mine).
Branch `m10/m10-2-measures`. Sizing **M–L**. Read
`docs/prompts/implementer-discipline.md` in full first, then
ERROR-DESIGN §E3/E10, CONTACT-DESIGN §C5, and this spec.

## Grounding (substrate facts, surveyed 2026-08-29)

- `Expr` (`editor-core/src/expr.rs`): 16 arithmetic `ExprKind`
  variants, dimension lattice {Length, Angle, Count, Scalar},
  private fields + smart constructors, `eval<T: Decide>`. **No
  function-call or entity-reference vocabulary exists.**
- `Node<P>` (`node.rs:471`): fifteen variants; a new kind touches
  the enum, `slots()`/`expr()`, `eval/slots.rs`, `eval/wire.rs`
  dispatch, `ValuePayload`, persistence, and a `SCHEMA_VERSION`
  bump + ledger (clean break per Q4; **claim the number by an
  explicit by-eye read of main's constant at the final re-merge —
  M10-1 took 15 and M10-P's key work has no schema claim, but LIB
  is active: verify, never assume 16**).
- Frozen entity references on a node: the `Node::Fillet.selection:
  Vec<StableName>` precedent; resolution through
  `resolve::resolve`/`resolve_with_prior` with typed `Diagnosis`.
- `Node::Declare` carries `DeclaredPairs`; C5's gap formulas are
  closed forms over the declared pair's carriers.
- Distribution/analysis groundwork (M10-1) is merged; the F1
  lattice has NO Length-power dims and mass-property measures are
  BANKED (plan Q2) — nothing here may grow the lattice.

## Scope

### 1. The `Measure` node (E3 verbatim, elaborated)

ONE dimension-generic sink: `Node::Measure { expr: MeasureExpr,
refs: Vec<StableName> }` — no body output; typed F1 quantity out.
**Elaboration (mine): entity references live on the NODE** (the
Fillet-selection precedent — frozen, canonicalized, rebindable via
the existing `Rebind` edit), and the measure expression references
them by index through primitive leaves, so `Expr`'s closed
param-only world is untouched. `MeasureExpr` = the existing `Expr`
arithmetic OVER a new leaf kind `Primitive(idx)` — implementer's
call whether that is an `Expr` extension behind a constructor door
or a thin wrapper enum; either way dimension checking runs the
existing lattice (distance/gap → Length, angle → Angle) and the
sublanguage stays total and finite.

### 2. v1 measurement primitives (typed functions, closed-form
### scope stated honestly)

- `distance(a, b)` → Length. v1 carrier scope, each arm a closed
  form with its lever named: vertex×vertex; vertex×plane-face;
  parallel plane-face×plane-face; parallel/coaxial
  cylinder-wall×cylinder-wall (axis distance and radii — the
  worked example's web). Any other pair refuses typed
  (`MeasureUnsupported`, naming the pair class) — the general
  pair lands with M10-5's clearance machinery, never sampled here.
  Parallelism/coaxiality are Q1 trilean decides (existing
  predicates; escalation refuses typed, never guesses).
- `angle(a, b)` → Angle: between two plane-face normals or two
  line-edge directions; same trilean discipline; other kinds
  refuse typed.
- `gap(declaration)` → Length (signed): CONTACT-DESIGN C5's
  formulas verbatim over a named `Declare` pair — parallel planes,
  concentric spheres, coaxial cylinders — with C5's sign
  convention binding (g > 0 clearance, 0 contact, < 0
  interference) and skew axes refusing typed.
- `min_clearance(sel)` is **M10-5's addition** to this vocabulary
  (needs the E7 machinery); it does not exist in this unit — no
  placeholder variant, no refusing stub.

### 3. Evaluation

Measure nodes evaluate at every `T` through the existing service
like any node: resolve refs against the upstream evaluation
(N-machinery; a dangling ref is the typed resolution `Diagnosis`,
never silence), read the referenced entities' carriers, compute
the closed form at `T`, wrap as a new
`ValuePayload::Measure(quantity + dim)`. Failures poison
descendants only (F2 — sinks have none). Content-keyed like every
node (slot values + refs' naming contribution per the existing
key discipline — state in the PR exactly what feeds the key).

### 4. `Assertion` (E10's persisted half)

`Node::Assertion { measure: RecipeNodeId, bound: Expr, dir:
AtLeast | AtMost }` — **elaboration (mine): a node, not a Doc
field**, for the same reasons E3 gives Measure nodes (persisted,
stable-named, diffable, cache-keyed); it references its Measure by
node id (a DAG edge, so poisoning composes) and its bound is an
ordinary dimensioned `Expr` (must type-check against the measure's
dim; mismatch is a typed document error). **Report-only** (E10 v1:
a failing assertion NEVER gates `build()`/evaluation): its
evaluated value is a typed verdict payload
(`Holds | Violated { measured, bound } | Unevaluated(reason)`),
consumed by reports; nothing downstream keys on it.

### 5. Persistence, equality, diff

Schema step (clean break, ledger paragraph, populated goldens with
a Measure + Assertion, prior-version refusal fixture, ledger-test
rows); `bit_eq`/`diff` clauses for both nodes; the binding census
fires for the Python surface — author the census disposition
honestly (full Python authoring surface may be chartered rather
than shipped, but reading a measure/assertion verdict from an
evaluation SHOULD ship — the friction R-series reviews keep
finding is unreadable results).

### 6. e2e (the worked example's first half)

A tour-adjacent test (not yet the demo cell — that is M10-6):
author the two-hole-plate document through `pncad` — two
cylindrical holes, a `Measure { distance(wall1, wall2) }` web and
an `Assertion { web >= 0.5mm }` — evaluate at f64, read the
measured web and the verdict; flip the assertion by a parameter
edit and see `Violated` with both numbers.

## Out of scope

`min_clearance` (M10-5); mass properties + lattice growth (banked,
Q2); the driver and interval boxes (M10-3); sensitivities (M10-4);
assertion gating of build (E10's open sub-question, post-v1); the
Stackup report (M10-4); GUI surface beyond what the census
records.

## Review claims to falsify

1. Zero impact on documents without Measure/Assertion nodes:
   memo keys, evaluation, persistence bytes all unchanged
   (merge-base differential — reviewer's unique signal).
2. No new ε and no new metered predicate WITHOUT a funnel row:
   every trilean this unit consumes is an existing predicate;
   if any new margined compare is minted, it must land in the
   k_stats funnel with a named predicate — grep and behavioral.
3. Refusal completeness: dangling refs, wrong-kind refs,
   dimension-mismatched bounds, unsupported carrier pairs, skew
   gap axes — all typed at both construction and load doors.
4. Closed-form honesty: each distance/angle/gap arm against an
   independent oracle (construct geometry with known answers);
   the C5 sign convention verified in all three regimes.
5. Scalar genericity: measures evaluate at Interval with
   containment of the f64 value (and at Dual64 through the open
   door — the value channel bit-identical; tangents zero unseeded).
6. E10 report-only: a Violated assertion changes NO downstream
   evaluation, gate, or product outcome — construct the document
   that would detect it.
7. e2e as a first-time user; report the friction.

## Acceptance

Hosted CI green on the unit's own head (state the drawn point; the
schema suites and editor-core battery are the likely load). PR
carries the sweep dispositions for every compiler-forced site of
the new node kinds and the primitive-scope table verbatim.
