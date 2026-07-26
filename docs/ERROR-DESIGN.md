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
lattice, recorded-ε/SetTolerance. This doc pins what the MVP *is* —
where distributions live, what a sensitivity means, what the driver
certifies, what is loudly refused — so M6 planning starts decided.

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
  recipe, different scalar" — and probability is not a scalar; it is
  a measure pushed forward over subdivision leaves, *literally the
  ratified Q1 sentence*. Any distribution-*arithmetic* rule (moment
  propagation, convolution on expressions) would be a second,
  inconsistent propagation semantics — wrong under shared-parameter
  dependence and a research tarpit.
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
  box", and the box IS the support; unbounded support makes that
  impossible, and the honest fallback — silently certifying ±kσ —
  buries the k as unrecorded policy. API-boundary sugar
  (`normal_3sigma(σ)`) may write `lo = −3σ, hi = 3σ` *into the
  document*; the persisted form is always truncated, so the
  certificate's domain is explicit and recorded.
- **Band carries no measure** — pure worst-case. Any report needing
  a measure (RSS, leaf mass) over a Band parameter refuses typed,
  never defaults to uniform: "I know the limits but not the shape"
  is real information; uniform is a different, stronger claim.
- **Independence**: product measure only, one distribution per
  parameter; joint distributions are v1-foreclosed (E11.2; a `Joint`
  form is an additive schema variant later).

## E3 — A measurement is a recipe node (proposed)

**Decision**: F4's vocabulary grows a `Measure` node family — sink
nodes producing typed F1 quantities, no body output:

- `Measure::Distance { a: StableName, b: StableName }` (minimum
  distance between the named entities), `Measure::Angle { a, b }`,
- `Measure::MassProperty { body, kind: Volume | CentroidComponent | … }`,
- `Measure::MinClearance { within: body/face-set selection }` (E7),
- plus any F7 expression over node outputs, wrapped as a node.

Evaluated at every `T` like all nodes; failures poison descendants
only (F2 verbatim; measurements have no descendants — they are
sinks). Persisted as ordinary nodes; resolved through name tables
with N-machinery's typed failure; content-key cached like everything.

- Rationale: measurements must be persisted, stable-named, diffable,
  scalar-generic, and cache-keyed — the exact property set recipe
  nodes already have. A side-channel query API would need a parallel
  persistence + naming + genericity story (the banked one-evaluation-
  service principle forbids parallel paths), and its references would
  silently dangle after edits; Measure nodes fail loudly through N5
  diagnosis instead.
- "Measurements only vs. any expression" dissolves: the sublanguage
  is total and finite by charter, so *any* expression is Measure-
  wrappable and every Measure node is dual/interval-evaluable by
  construction. Sensitivities are of node outputs; same thing.
- Counterargument: DAG pollution — dozens of measurement sinks.
  Accepted; sinks are lazily evaluable and the GUI presents them as
  a panel, not tree rows. Clutter is a presentation problem; the
  document is the right home for design intent (E10's assertions).

## E4 — Sensitivity semantics: forward Dual<f64>, one seed per parameter, chamber-local and marked as such (proposed)

**Mechanism**: ∂m/∂pᵢ = evaluate the recipe at `Dual<f64>` with pᵢ
seeded (`variable`), others `constant`; n parameters ⇒ n independent
passes (pure model; parallel under D9 idiom 1). The dual value
channel is bit-identical to the f64 run (THE Dual contract), so
every predicate verdict — hence the topology — is the f64 build's:
the sensitivity is of the as-built body, guaranteed.

**Semantics honesty (the load-bearing clause)**: ∂m/∂pᵢ is the
derivative of the *fixed-topology program* (per the ratified kink
conventions), valid within the nominal's chamber; it can jump at a
predicate flip. Therefore **a sensitivity is never reported bare**:
it carries either a **chamber certificate** (an E6 leaf certified
over the box asked about) or the explicit **`local_only` marking**
(derivative at the point, validity radius unclaimed). No third,
unmarked state — D4 fail-loud applied to derivatives; the classic
stackup lie is a sensitivity extrapolated across a topology change.

**Certified tier**: `Dual<Interval>` over a leaf yields derivative
*enclosures* (Clarke straddle hulls at kinks, per M0 ratification).
Consumed for E7's monotonicity pruning and E5's contribution bounds
— never for refusal decisions (E9).

**Not in v1**: reverse mode; vector-forward (one pass, n tangents).
n forward passes are O(n·build) with trivial parallelism; both
alternatives are pure-performance additives later (E11.4).

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
  certified leaves — NOT the linearized Σ|∂m/∂pᵢ|·Δpᵢ, which is
  first-order and silently wrong under curvature; the linearization
  survives only as the per-contributor *table*, labeled advisory.
- **`rss`** = √Σ(∂m/∂pᵢ·σᵢ)², linearized, advisory. Available only
  when *every* contributing parameter carries a measure — a Band
  parameter yields `rss: UnavailableBecause(...)`, never a partial
  RSS (a partial RSS is still a lie).
- **`coverage`** keeps the report honest under refused residue:
  "certified over 99.7% of the box by measure; 0.3% refused
  (reasons)" — refused mass is part of the answer, not a footnote.
- Counterargument: engineers want one ± number and will read RSS as
  the answer. Mitigation is labeling and ordering, not omission —
  omit RSS and users compute it outside, unlabeled.

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
- Any `Indeterminate` → **bisect** and recurse. Split rule named and
  deterministic (D9): dimension of maximum relative width, ties to
  lowest index. Leaves are independent — parallel under D9 idiom 1,
  CPU/rayon per the ratified GPU-boundary table.
- **Terminal sliver** (enclosure wholly inside (ε, Kε)) → refuse,
  never refine (ratified PR 7 semantics: a genuine semantic sliver).
- **Budget exhaustion** (depth/work caps — named constants, run
  configuration like K) → refuse with `Budget`, typed and priced.
  No silent partial answers.

**Probability enters exactly once**: leaf masses under the product
measure, at reporting time — Normal CDF via f64 erf is fine because
*reporting decides no topology and gates no certification* (the
display-layer argument in D4). Nothing else in M6 consumes a
measure.

**No chamber-connectivity claims**: adjacent certified leaves with
identical verdict-vector keys may be *presented* coalesced, but the
semantic unit stays the leaf — each leaf's results are
self-contained, so connectivity proofs would be new machinery for
zero certificate content.

**K telemetry (T6/K-REPORT obligation)**: every driver-path
predicate sample lands in the k_stats funnel. Margins *driven
toward* zero by box refinement are the first genuinely
ill-conditioned population K sees — exactly what K-REPORT scoped
out. The M6 exit reads the funnel before reaffirming K = 10.

## E7 — Clearance & self-intersection: a trichotomy over box × domain; duals accelerate, never decide (proposed)

For a **certified leaf** (fixed topology), M6 answers two questions:
global self-intersection-freedom, and `min-clearance ≥ c` for a named
selection (`Measure::MinClearance` + an E10 assertion supplies c).

**Mechanism**: two nested subdivisions. Outer: the E6 parameter
leaves. Inner: geometry-domain subdivision with interval exclusion —
the pre-M5 "SSI completeness is an interval obligation" posture,
now run with interval *parameters*: candidate face pairs from a
conservative interval BVH; a domain cell pair is discharged when the
interval distance enclosure proves ≥ c (pruned), split when
indeterminate, and reported when a definite violation is found.

**The answer is a trichotomy, never silence**:

- `Holds` — clearance ≥ c certified throughout the leaf box × all
  domain pairs (every cell pair discharged);
- `Violated { param_witness, geometry_witness }` — a definite
  sub-c distance at a concrete parameter point and closest-point
  pair (an f64 point in the leaf, verified definite at f64 — the
  actionable artifact: "here, at these parameter values");
- `Refused { sliver | budget }` — the clearance margin `d − c` is a
  margined predicate like any other: terminal-sliver enclosures and
  budget exhaustion refuse, typed and priced by measure.

Probability never enters *inside* a leaf: there is no
"probabilistically clear" verdict. Mass accounting applies to leaves
(E6), full stop.

**Self-intersection scope**: the tier-3′ census made global and
parametric — non-adjacent face pairs certified strictly positive
distance; adjacent pairs are covered locally by the wedge predicates
(their distance is legitimately 0). v1 geometry scope = the carriers
M5 leaves interval evaluators for; carriers without interval
evaluation refuse typed (`Unsupported`), never downgrade to
sampling.

**Duals as pruning only**: over a leaf, if the `Dual<Interval>`
enclosure of ∂d/∂pᵢ is sign-definite, d is monotone in pᵢ and the
check restricts to the corresponding box facet — a dimension
collapses. This is an accelerator: correctness never depends on it,
and a degraded tangent (E9) merely forfeits the pruning.

## E8 — Composition with the W-contracts: witnesses are fixed document state; walls become priced refusals (proposed)

- **The analysis lane is read-only.** The driver NEVER writes the
  document: no auto-ReWitness however clean the certificate (W4's
  automation clause is about *commit contexts*; analysis is not
  one). Every leaf replay consumes the committed witness (W1)
  verbatim — leaf results stay a pure function of (doc, box), hence
  cacheable and parallel.
- **Per leaf, sketch nodes run W2 at T = Interval**: the ratified
  contraction-from-f64-witness over the leaf's box. The certificate
  firing proves the *entire leaf* shares the witness's branch —
  W4's certified-same-branch invisibility, upgraded from a point to
  a box: one containment, whole-leaf branch identity.
- **When a leaf straddles a wall**, the certificate refuses and the
  driver bisects; terminal refusals split by kind, vocabulary
  preserved (W3 layer-2 language, never collapsed into "sliver"):
  - `Infeasible` — no real solution over part of the box (the elbow
    past straightening): typed refusal whose mass is a
    *product-level finding* — "for 2.1% of the tolerance mass the
    sketch has no solution" IS the detect-problems deliverable;
  - `Bifurcation(WitnessBifurcation)` — the box reaches across a
    fold/branch wall: refused with the W3 payload. **Distributions
    do not cross witness walls** — the model is undefined on the
    far side without a recorded ReWitness, so M6 prices the mass
    and refuses; the remedy is user intent (tighten the tolerance
    or re-witness), never machinery guessing a branch.
- `solver_branch_margin` samples from driver runs land in k_stats —
  the T6 obligation's data arrives through exactly this lane.

## E9 — Tangent poison never refuses (proposed)

Ratified base: decoration-as-poison lives in the value lane;
`Decide` classifies the value channel only — tangent data does not
decide base-space topology. **M6 addendum**: in `Dual<Interval>`
work, derivative-channel degradation (Clarke straddle hulls widening
to the whole line, kink-jump enclosures like floor's `[0, +∞]`)
NEVER contributes to leaf refusal — refusal is decided solely by
value-channel predicates and W-certificates. A degraded tangent
forfeits exactly its uses: no monotonicity pruning (E7); the
affected `per_param`/`rss` entries report `UnavailableBecause`
(E5); `worst_case` is untouched (it comes from value-channel
interval evaluation, never linearization).

- Rationale: refusing on tangent poison would let `abs` at a kink
  veto an analysis whose value channel certifies cleanly — inverting
  the ratified hierarchy. A straddle hull containing zero IS
  information ("possibly non-monotone") and is consumed as exactly
  that: no pruning, no linearized claims.
- Counterargument: a stackup whose every tangent degrades is a weak
  report. True, and honest — it still carries the gating certified
  worst-case; the advisory columns degrade loudly, never lie.

## E10 — Reporting & persistence: distributions and assertions persist; verdicts are derived and CI-able (proposed)

**Persisted, in-document** (additive F3 migration, one schema step;
fields named now):

- `ParamDef.distribution: Option<Distribution>` (E2 forms, offsets
  dimensioned per F1, shortest-round-trip floats as ratified);
- the `Measure` node family (E3) with StableName references;
- `Assertion { measure: NodeId, bound: Quantity, dir: AtLeast | AtMost }`
  records — tolerance *requirements* as recorded design intent (the
  CAD analog of a test suite: "min wall ≥ 0.5 mm" lives in the
  document, versioned and diffable, not in a script beside it).

Unknown-field/version handling per F3 verbatim: refusal typed, no
best-effort loads; the migration chain gains one explicit
version-to-version step.

**Derived, never persisted** (re-derives from the recipe; the D3
"the recipe IS the save" line holds): `ParamBoxVerdict`, `Stackup`
reports, clearance verdicts. All content-key cached — keyed by the
bit-content of (recipe slice, box, ε, K), so D9 bit-determinism
makes the key the correctness proof, same as everywhere. All
serializable in a stable text form for CI goldening.

**CI rows M6 adds**: (1) assertion gating — corpus documents'
assertions must certify (`Holds`), with `Violated`/`Refused` failing
the row loudly; (2) goldened refusal-mass accounting on a
margin-thin fixture (the honesty metric is itself regression-
tested); (3) the k_stats funnel rows for driver + solver predicates
(the K re-examination evidence, E6/E8).

**Open sub-question (flagged, not decided)**: whether a failing
Assertion should also be able to gate `build()` itself (a document
that refuses to evaluate while violated) — v1 says no, assertions
report; a gating mode is additive policy.

## E11 — What M6 does NOT do (loud) (proposed)

1. **No Monte Carlo lane.** Pure replay keeps MC additive forever;
   v1 refuses it deliberately: MC produces confidence, not
   certificates, and shipping it first would set "probably fine" as
   the product's epistemic register. If it ever arrives, it is a
   labeled advisory lane after the certified lane exists.
2. **No correlated/joint distributions** — product measure only
   (E2); `Joint` is an additive schema variant later.
3. **No distributions on structural (Count) parameters** — typed
   refusal. Structural variation is D8's explicit regime; "hole
   count ~ Uniform{3..5}" is a different feature (design-space
   exploration), not tolerance analysis.
4. **No reverse-mode AD, no vector-forward duals** (E4) —
   performance additions, not semantic ones.
5. **No GD&T semantics.** Stackups are parameter-space facts about
   the recipe; ASME Y14.5 (datum frames, material condition) is a
   language layer that could later *compile to* Measure nodes +
   assertions. M6 does not speak it rather than approximating it.
6. **No output densities** (PDF/histogram of a measurement) — those
   require MC or measure transport, both out. v1 outputs: certified
   enclosure + advisory RSS + leaf-mass accounting.
7. **Imported bodies carry no parameters** (D7, restated): nothing
   to vary; they participate in clearance checks as constants.
8. **No sensitivity-driven optimization/inverse loops** ("resize
   until clearance holds") — consumers of M6's reports, post-M6.

## Worked example: the two-hole plate

Plate width w (Uniform ±0.1 mm), two hole diameters d₁, d₂
(TruncatedNormal σ = 0.02, ±0.06 mm); `hole_x = w/2 − margin` per
F7. Document carries `Measure::Distance(hole1_wall, hole2_wall)`
(the web) and `Assertion { web ≥ 0.5 mm }`.

- Driver (E6): the 3-box certifies in four leaves after one
  bisection in w (a coplanarity predicate goes indeterminate at
  small w); one terminal-sliver leaf refuses — 0.4% of the mass,
  reported, priced.
- Stackup (E5): ∂web/∂w = +0.5, ∂web/∂dᵢ = −0.5, chamber-certified;
  certified worst-case web ∈ [0.487, 0.613] mm.
- Verdict: the assertion FAILS with a parameter witness (w = lo,
  dᵢ = hi) — while RSS says σ_web ≈ 0.017 mm, "3σ fine." The report
  prints both; the certified number gates. That divergence —
  certified worst-case vs. RSS optimism — is the MVP's reason to
  exist.
- Clearance (E7): `Holds` on three leaves; on the fourth,
  monotonicity pruning (∂d/∂w sign-definite by Dual<Interval>)
  collapses w to a facet, which certifies. Self-intersection-
  freedom certifies everywhere.

## Open after this doc

- **Driver constants**: split-rule details, budget defaults,
  leaf-coalescing presentation — M6 PR-spec work, not blockers.
- **Assertion gating of `build()`** (E10's flag): report-only vs. a
  refuses-while-violated mode — needs editor-core UX input.
- **Vector-forward duals / reverse mode** — pure performance;
  revisit when the Band 4 corpus prices n-pass sensitivity runs.
- **The MC advisory lane** (E11.1): whether it ever ships, and how
  it coexists with the certified register.
- **Correlated distributions**: real tolerance chains correlate
  (same machining setup); additive schema, but leaf-mass accounting
  must then integrate non-product measures.
- **Clearance `c` as a Band**: should the assertion bound carry its
  own tolerance? v1 says exact; revisit with GD&T-adjacent cases.
- **SetTolerance × distributions**: bands are Lengths,
  ε-independent — but sliver-refusal mass depends on ε, so ε changes
  move coverage numbers; the SetTolerance diff should surface
  coverage deltas. Wiring is M6 PR-spec.
- **Naming-pillar composition**: Measure nodes' verdict vectors
  should join the N-machinery diff reports (same verdict-diff
  substrate); confirm at implementation.
