# M10-3 — the E6 subdivision driver (unit spec)

**Status: BINDING at dispatch** (orchestrator-authored; the design
is ratified — `docs/ERROR-DESIGN.md` E6, with E2's tail/containment
amendment and E8's read-only rule — and the driver constants below
are the PR-spec dials E6's "Open after this doc" delegated).
Branch `m10/m10-3-driver`. Sizing **L**. Read
`docs/prompts/implementer-discipline.md` in full first, then
ERROR-DESIGN §E2/E6/E8/E9 and this spec. The disk rules bind:
`CARGO_INCREMENTAL=0`, touched-crate batteries.

## Grounding (substrate facts + logged adjudication inputs)

- `Doc::param_env<T>` embeds every parameter through `from_f64` —
  **the interval parameter door is this unit's first deliverable**;
  nothing reaches `evaluate::<Interval>` with a non-degenerate
  parameter today (M10-P's disclosed limit, by design).
- `VerdictLog` rows are float-free and scalar-independent — exact
  f64-vs-Interval verdict-vector comparison is ALREADY the
  substrate's gift; leaf certification is built on it.
- M10-P's guided replay: profile structure re-verified at `T`;
  its typed abort IS the bisect cue; a structure-flip refusal IS
  `FlipCrossing` evidence for profile decisions. Logged: the
  guided enclosure is node-dependent (an extrude's body widens
  with the box; a loft's section stays f64 by C6/D9) — the leaf
  protocol consumes whatever enclosure each node lawfully
  produces and certifies on PREDICATE VERDICTS, never on widths.
- M10-1's analysis lane: `AnalyzedBox`, `box_mass`/`tail_mass`
  with `MeasureUnavailable(Band)`; the logged "no distribution ⇒
  mass 1 lives in prose" input — this unit gives the fixed-param
  case its typed spelling.
- The SSI exhaustiveness subdivision (`geom-brep/src/ssi/exhaust.rs`)
  is the shape template: receipt identity, typed budget refusal,
  deterministic bisect-widest with a fixed tie-break.
- Known coverage limiter, consumed not fixed: issue 1191 (period-
  fold widening) inflates refused mass on boxes crossing period
  boundaries; cite it where a fixture shows the effect.
- `EvalOptions` carries `profile_lift` and `parallel`; the memo
  `prior` door exists.

## Scope

### 1. The interval parameter door

A typed binding from (doc nominals, a sub-box of the `AnalyzedBox`)
to `ParamEnv<Interval>`: each distributed axis becomes
`[nominal+lo, nominal+hi]` for the leaf's offsets; a parameter
WITHOUT a distribution is **typed Fixed** (the logged input: the
box type distinguishes Fixed from Varying axes, so "mass 1" is a
spelling, not a convention every consumer re-derives). Count
params are not axes. The door lives in `editor_core::analysis`
beside the box machinery; `evaluate` itself gains nothing — the
driver hands the env in through the existing evaluation options
(implementer's call on the exact seam; compile-time, no stringly
config).

### 2. `drive(doc, box, config) -> ParamBoxVerdict` (E6 verbatim)

- **Leaf protocol**: evaluate the doc at `Interval` over the leaf's
  env (lift Guided). Every predicate definite AND the leaf's
  verdict vector equal to the f64 witness build's ⇒ **certified**
  (`Leaf { box, verdict_vector_key, results }`). Definite on a
  DIFFERENT vector ⇒ `FlipCrossing { flipped }`, the flipped
  predicates named from the vector diff (a guided structure-flip
  refusal is this class too, named from its decision). Any
  indeterminate (a `k_stats` escalation, a guided typed abort) ⇒
  **bisect**.
- **Split rule (dial, recorded here)**: max RELATIVE width over
  varying axes, ties to lowest axis index — deterministic, D9.
- **Terminal sliver**: the ratified PR-7 semantics — refuse
  `SliverTerminal { predicate }`, never refine, when the deciding
  enclosure sits wholly inside (ε, Kε).
- **Budgets (dials, recorded here)**: `max_depth = 24` per axis
  and `max_leaves = 65_536` per drive, both in `DriveConfig` (run
  config like K; overridable per request). Exhaustion refuses
  `Budget { .. }` typed and priced — no silent partials; the
  receipt identity (certified + refused leaves cover the box
  exactly) is asserted the SSI-exhaust way.
- **E8 vocabulary**: `Infeasible` and
  `Bifurcation(WitnessBifurcation)` variants EXIST in the refusal
  enum (ratified E6 vocabulary) and are documented unreachable in
  v1 at the type — no machinery invents a way to reach them.
- **Read-only (E8)**: `drive` takes `&Doc` and returns a value;
  nothing in its API can write, rewitness, or edit. State it and
  pin it at the API level.

### 3. Measure accounting (E2)

Per-reason mass under the product measure via `box_mass`, plus
`Unanalyzed` = tail mass, summing to 1 within stated f64 bounds.
**Certification is measure-free; pricing is not**: with any Band
parameter varying, leaves still certify/refuse normally and the
ACCOUNTING columns refuse typed naming the Band params (E2 —
never a uniform default). **Chamber containment** (the E2
amendment): when every leaf touching the analyzed box's boundary
is `FlipCrossing`, report `containment: true` — a free predicate
on the leaf set; the accounting text then calls the budget exact
rather than conservative.

### 4. Determinism, caching shape, parallelism

- The verdict is a pure function of (recipe slice, box, ε, K,
  config): serialize it (the goldening form M10-6 consumes) and
  content-key it on those bits — derived, never persisted (E10).
- rayon idiom 1 over independent leaves behind the existing
  runtime-switch pattern; a differential row proves bit-identical
  verdicts sequential vs parallel, and run-to-run.

### 5. k_stats (the E6/T6 obligation)

Driver-path predicate samples reach the funnel: the mechanism is
the implementer's proposal consistent with the existing
probe-lane machinery (a config-gated Probe replay of certified
leaves is acceptable; inventing a new recording channel is not).
State in the PR what a K-REPORT re-examination run would execute;
wire enough that the k_probe sweep CAN drive it. An in-band
landing here is K-REPORT's stated re-open trigger — the funnel
row is the deliverable, not a K verdict.

### 6. e2e (the worked example's driver half)

Drive the M10-P e2e parameterized document (or an equivalent
two-param document with a fillet) over a genuinely wide box:
a handful of certified leaves after ≥1 bisection, at least one
typed refusal class exercised, accounting summing to 1, and the
whole thing through public doors. A planted-flip fixture (a
parameter box straddling a real predicate flip — a boolean
overlap→disjoint transition is the classic) certifies on the
witness side and refuses `FlipCrossing` on the far side, named.

## Out of scope

Clearance (M10-5); Measure/Stackup consumption of leaves (M10-4);
MC; branch enumeration (v2's recorded door); output densities;
fixing issue 1191; any solver machinery.

## Review claims to falsify

1. Certification honesty: a leaf certifies IFF interval evaluation
   is fully definite AND the verdict vector matches the witness —
   attack with planted flips, near-flip boxes, and a mutated
   comparison.
2. Accounting: sums to 1 within stated bounds against an
   independent oracle; Band pricing refuses typed while
   certification proceeds; tail rides every result.
3. Determinism: bit-identical verdicts across parallel schedules,
   repeated runs, and (for the serialized form) across
   save/compare.
4. Zero impact on ordinary evaluation: no f64 or point-Interval
   behavior change (merge-base differential — unique signal).
5. Refusals: budgets refuse typed and priced (construct
   exhaustion); slivers refuse per the ratified semantics; nothing
   silently keeps partial coverage; the receipt identity holds on
   every drive.
6. Read-only: the API cannot express a document write; the same
   doc value drives twice identically.
7. Containment: fires on a constructed contained case and does
   NOT fire when any boundary leaf certifies.
8. e2e as a consumer; report the friction (this is the first unit
   whose OUTPUT is the product's deliverable — "2.1% of the
   tolerance mass has no valid build" is the detect-problems
   pitch; judge whether the verdict as shipped can say that).

## Acceptance

Hosted CI green on the unit's own head; the interval lane is the
unit's axis (request via `CI-Config: lane=interval` when the draw
would miss it). PR carries: the dials table (split rule, budgets)
as shipped, the receipt-identity statement, the k_stats mechanism,
and the honest statement of what refused-mass rates look like on
the fixtures (1191's effect cited where visible).
