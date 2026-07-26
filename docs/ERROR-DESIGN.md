# Error-propagation MVP: distributions, sensitivities, certified checks over the parameter box (pre-M6 design doc)

Status: **DRAFT for design conversation — not ratified.**

Third of the pre-M6 docs (NAMING-DESIGN #74, SOLVER-DESIGN #79 are the
pattern). Grounding: DESIGN.md's M6 roadmap entry and central
commitment ("evaluate the same function with a different scalar
type"); the ratified Q1 subdivision-driver posture ("union over
branches, pushing the distribution forward into each: leaves of the
subdivision take definite branch paths; outcome probabilities are the
distribution's measure on the sub-boxes"); SOLVER-DESIGN W1–W9 (M6's
other half); M4's F1 (dimension lattice), F3 (persistence/migrations),
F4 (node vocabulary); PR 6's recorded-ε discipline; K-REPORT's scope
honesty. NOT reopened here: the `Real` surface, decoration-as-poison,
the Dual kink conventions and value-part delegation, W1–W9, the F1
lattice, recorded-ε/SetTolerance. This doc pins what M6's
error-propagation MVP *is* — where distributions live, what a
sensitivity means, what the driver certifies, and what is loudly
refused — so M6 planning starts decided.

## 0. Term hygiene (read first)

- **Parameter box**: an axis-aligned product of per-parameter
  intervals in continuous-parameter space; structural (Count)
  parameters are FIXED throughout — a box never varies topology
  *structurally*, only through predicate flips.
- **Chamber**: a connected flip-free region of parameter space (W3's
  discriminant-chamber language). The driver certifies *leaves*
  (sub-boxes), never chambers — a leaf certificate is self-contained
  and makes no connectivity claim.
- **Support vs. measure**: the box the kernel machinery sees is the
  distribution's *support*; the probability *measure* never enters
  kernel evaluation — it prices leaves at reporting time (E1/E6).
- **Certified / advisory**: certified numbers come from interval
  (or Dual<Interval>) evaluation and are containment-true; advisory
  numbers (RSS, linearized contributions) are first-order f64
  artifacts, always labeled, never gating.

## E1 — Distributions are document-layer parameter metadata; the kernel sees only boxes and seeds (proposed)

**Decision**: a continuous `ParamDef` at the document layer gains an
optional `Distribution` (E2). The kernel and geometry lanes never see
a probability. The analysis lane projects a distribution to exactly
three consumables: (a) its **support box** for interval/driver work,
(b) **seed vectors** for dual passes, (c) a **measure** used only to
price leaves in reports. No `Real` instantiation carries probability;
there is no `Distribution` scalar and never will be one via this doc.

- Rationale: the central commitment makes error propagation "same
  recipe, different scalar" — the scalars are intervals and duals.
  Probability is not a scalar; it is a measure pushed forward over
  subdivision leaves, which is *literally the ratified Q1 sentence*.
  The pushforward through the actual recipe is what the machinery
  computes; any distribution-*arithmetic* rule (moment propagation,
  convolution on expressions) would be a second, inconsistent
  propagation semantics — wrong under shared-parameter dependence and
  a research tarpit. Rejected accordingly.
- **Rejected alternative — F1 quantity extension**: making
  distributions typed quantities flowing through expressions invites
  exactly that arithmetic. The F1 lattice stays a value lattice;
  distributions annotate *parameters*, not values.
- Counterargument (honest): you cannot state "this measurement is
  normally distributed" as an input. Correct — measurement
  distributions are *outputs* of the analysis, and v1 deliberately
  does not even compute output densities (E11.6).
- Forecloses: distribution-valued expressions; per-node distribution
  overrides; probability inside evaluation code.

## E2 — v1 vocabulary: three forms, compact support mandatory (proposed)

```
Distribution = Band            { lo, hi }              // worst-case only, NO measure
             | Uniform         { lo, hi }
             | TruncatedNormal { sigma, lo, hi }
```

Offsets are relative to the parameter's nominal value (which stays
the single source of truth for the f64 build), dimensioned per F1 —
a Length parameter's band is Lengths. Constraint `lo ≤ 0 ≤ hi`
(asymmetric bands like +0/−0.1 legal; nominal outside its own support
is a typed document error).

- **Compact support is mandatory.** An untruncated Normal is a typed
  refusal at the document layer. Every M6 certificate is "over the
  box", and the box IS the support; unbounded support makes "certified
  over the support" impossible, and the honest fallback — silently
  certifying ±kσ — would bury the k as unrecorded policy. Forcing the
  truncation into the document makes the certificate's domain explicit
  and persisted. API-boundary sugar (`normal_3sigma(σ)`) may write
  `lo = −3σ, hi = 3σ` *into the document*; the persisted form is
  always truncated.
- **Band carries no measure.** It is pure worst-case. Any report
  requiring a measure (RSS, leaf mass) over a Band parameter is
  refused with a typed reason, never defaulted to uniform — "I know
  the limits but not the shape" is real information and uniform is a
  different, stronger claim.
- **Independence**: product measure only; one distribution per
  parameter; correlated/joint distributions are v1-foreclosed (E11.2,
  additive later — a `Joint` form is a new schema variant, no
  migration pain).

## E3 — A measurement is a recipe node (proposed)

**Decision**: F4's vocabulary grows a `Measure` node family — sink
nodes producing typed F1 quantities, no body output:

- `Measure::Distance { a: StableName, b: StableName }` (point/edge/
  face pairs; the value is the minimum distance between the named
  entities),
- `Measure::Angle { a, b }`,
- `Measure::MassProperty { body, kind: Volume | CentroidComponent | … }`,
- `Measure::MinClearance { within: body/face-set selection }` (E7's
  subject),
- plus any F7 expression over node outputs, wrapped as a node.

Evaluated at every `T` like all nodes; failures poison descendants
only (F2 verbatim; measurements have no descendants — they are
sinks). Persisted as ordinary nodes; resolved through name tables
with N-machinery's typed failure; content-key cached like everything.

- Rationale: measurements must be persisted, stable-named, diffable,
  scalar-generic, and cache-keyed — the exact property set recipe
  nodes already have. A side-channel query API ("measure this now")
  would need a parallel persistence + naming + genericity story,
  which the banked editor-core principle (one evaluation service,
  generic over `Real`, no parallel path) forbids. And a query API's
  references would silently dangle after edits; Measure nodes fail
  loudly through N5 diagnosis instead.
- "Measurements only vs. any expression" dissolves: the expression
  sublanguage is total and finite by charter, so *any* expression is
  Measure-wrappable and every Measure node is dual/interval-evaluable
  by construction. Sensitivities are of node outputs; the two
  phrasings coincide.
- Counterargument: DAG pollution — a toleranced part may carry dozens
  of measurement sinks. Accepted; they are sinks, cheap to evaluate
  lazily (the evaluation service may skip sinks nobody asked for),
  and the GUI presents them as a separate panel, not feature-tree
  rows. The document is the right home for design intent (see E10's
  assertions), and clutter is a presentation problem.

## E4 — Sensitivity semantics: forward Dual<f64>, one seed per parameter, chamber-local and marked as such (proposed)

**Mechanism**: sensitivity of measurement m to parameter pᵢ =
evaluate the recipe at `Dual<f64>` with pᵢ seeded (`variable`), all
other parameters `constant`; n parameters ⇒ n passes. Passes are
independent (pure model) and parallelize under D9 idiom 1. The dual
value channel is bit-identical to the f64 run (THE Dual contract),
so every predicate verdict, and hence the topology, is the f64
build's — the sensitivity is of the as-built body, guaranteed, not
of some drifted re-evaluation.

**Semantics honesty (the load-bearing clause)**: ∂m/∂pᵢ is the
derivative of the *fixed-topology program* — the program as
evaluated, per the ratified kink conventions. It is valid within the
nominal's chamber and can jump at a predicate flip. Therefore **a
sensitivity is never reported bare**: every reported sensitivity
carries exactly one of

- a **chamber certificate** — the E6 leaf containing the evaluation
  point certified over the box the user asked about, or
- the explicit **`local_only` marking** — derivative at the point,
  validity radius unclaimed.

There is no third, unmarked state. This is D4 fail-loud applied to
derivatives: the classic tolerance-stackup lie is a sensitivity
extrapolated across a topology change.

**Certified tier**: `Dual<Interval>` over a leaf yields derivative
*enclosures* (Clarke straddle hulls at kinks, per M0 ratification).
Consumed for E7's monotonicity pruning and E5's contribution bounds
— never for refusal decisions (E9).

**Not in v1**: reverse mode; vector-forward (one pass, n tangents).
n forward passes are O(n·build) with trivial parallelism; the recipe
count where reverse mode wins (n ≫ measurements) is not the MVP
regime. Vector-forward is a pure-performance additive later — the
`Dual<T>` chain rules generalize mechanically (E11.4).

## E5 — Stackup deliverable: a typed per-measurement report; certified worst-case gates, RSS is labeled advisory (proposed)

```
Stackup {
  measurement:  StableName of the Measure node,
  nominal:      f64 value (the f64 build's),
  per_param:    [ { param, sensitivity (E4-marked),
                    contribution = |∂m/∂pᵢ|·max(|lo|,|hi|) } ],   // advisory
  worst_case:   certified enclosure of m over the certified leaves, // gates
  rss:          Advisory<σ_m> | UnavailableBecause(Band params named),
  coverage:     certified measure fraction + refused-residue bound (E6),
}
```

- **`worst_case` is the headline and the only gating number**: the
  hull of interval evaluations of the Measure node over E6's
  certified leaves. It is NOT the linearized Σ|∂m/∂pᵢ|·Δpᵢ — the
  linearization is first-order and silently wrong under curvature;
  it survives only as the familiar per-contributor *table*
  (`per_param`), labeled advisory.
- **`rss`** = √Σ(∂m/∂pᵢ·σᵢ)², linearized, advisory. Available only
  when *every* contributing parameter carries a measure — a stackup
  with a Band parameter reports `rss: UnavailableBecause(...)`,
  never a partial RSS (a partial RSS is a smaller lie than a
  defaulted-uniform one, but still a lie).
- **`coverage`** keeps the report honest when the driver refused
  residue: "worst_case certified over 99.7% of the box by measure;
  0.3% refused (reasons)" — the refused mass is part of the answer,
  not a footnote.
- Counterargument: engineers want one ± number and will read RSS as
  the answer. Mitigation is labeling and ordering (certified first),
  not omission — RSS is the industry's lingua franca and refusing to
  print it would just push users to compute it outside, unlabeled.

## E6 — The subdivision driver and `ParamBoxVerdict` (the M6 build) (proposed)

M6 builds the Q1-promised propagation driver as an analysis-lane
service on the editor-core evaluation service (which is `Real`-
generic from day one — banked principle; no parallel path).

```
drive(doc, box) -> ParamBoxVerdict {
  certified: [ Leaf { box, verdict_vector_key, results } ],
  refused:   [ Leaf { box, reason: SliverTerminal { predicate }
                              | Bifurcation(WitnessBifurcation)   // E8
                              | Infeasible(typed solve failure)   // E8
                              | Budget { depth/work bound hit } } ],
  measure_accounting: per-reason mass under the product measure,
}
```

**Leaf protocol**: replay the recipe at `T = Interval` over the leaf
box (parameters = intervals; witness data verbatim per E8).

- Every predicate definite → leaf **certified**; lineage-scoped key
  identity (Q1 PR 8) means the leaf shares topology with the f64
  build it brackets, and Measure-node enclosures are containment-true
  over the whole leaf.
- Any `Indeterminate` → **bisect** and recurse. Split rule is named
  and deterministic (D9): split the dimension of maximum relative
  width, ties to lowest parameter index. Leaves are independent —
  parallel under D9 idiom 1, CPU/rayon per the ratified GPU-boundary
  table.
- **Terminal sliver** (enclosure wholly inside (ε, Kε)) → refuse the
  leaf, never refine further (ratified PR 7 semantics: it is a
  genuine semantic sliver, not numeric fuzz).
- **Budget exhaustion** (depth/work caps — named constants, run
  configuration like K) → refuse with `Budget`; the refused mass in
  `measure_accounting` is the honesty line. No silent partial
  answers: a budget refusal is typed and priced.

**Probability enters exactly once**: leaf masses under the product
measure, computed at reporting time — Normal CDF via f64 erf is fine
*because reporting decides no topology and gates no certification*
(the same argument as display-layer comparisons in D4). Nothing else
in M6 consumes a measure.

**No chamber-connectivity claims**: adjacent certified leaves with
identical verdict-vector keys may be *presented* coalesced, but the
semantic unit stays the leaf. Two certified leaves are not claimed to
be one chamber — each leaf's results are self-contained, so the
claim isn't needed, and proving connectivity would be new machinery
for zero certificate content.

**K telemetry obligation (T6/K-REPORT)**: every driver-path predicate
sample lands in the k_stats funnel. This corpus — margins *driven
toward* zero by box refinement — is the first genuinely
ill-conditioned population K will see; the K-REPORT explicitly
scoped its "K rarely binds" evidence away from exactly this. The M6
exit reads the funnel before reaffirming K = 10.
