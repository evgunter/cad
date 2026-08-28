# Error-propagation MVP: distributions, sensitivities, certified checks over the parameter box (pre-implementation design doc)

Status: **RATIFIED (Evan, PR #110, merged 2026-07-27 — 👍 on the
round-2 sign-off comment).** E1–E11 are the error-propagation
contract seed; the milestone that builds them is **M10**. Design
history: Round 2 (#110): E1 restated as the *completion* of the
Real-trait vision; E2 truncation → **tail-mass accounting** (Evan).
Round 3 (Evan's careful pass, "broadly looks good"): E3 collapsed to
one dimension-generic Measure sink; E6 adopts **no-flips v1** (Evan's
proposal); E11 MC softened to a labeled advisory lane; E11.6
histogram note. Post-ratification amendment on record (#110 thread,
Evan's one-branch-tails observation, 2026-07-27): chamber containment
added to E2.

Written alongside NAMING-DESIGN (#74) and SOLVER-DESIGN (#79) as the
third pre-M4 design doc. Grounding: DESIGN.md's central commitment
and M10 roadmap entry; the ratified Q1 subdivision-driver posture
("outcome probabilities are the distribution's measure on the
sub-boxes"); SOLVER-DESIGN W1–W9; M4's F1/F3/F4; PR 6's recorded-ε
discipline; K-REPORT's scope honesty. NOT reopened: the `Real`
surface, decoration-as-poison, the Dual kink conventions and
value-part delegation, W1–W9, the F1 lattice,
recorded-ε/SetTolerance. This doc pins what the MVP *is*, so M10
planning starts decided.

## 0. Term hygiene (read first)

- **Parameter box**: an axis-aligned product of per-parameter
  intervals in continuous-parameter space; structural (Count)
  parameters are FIXED — a box varies topology only through
  predicate flips.
- **Chamber**: a connected flip-free region of parameter space (W3's
  discriminant-chamber language). The driver certifies *leaves*
  (sub-boxes), never chambers.
- **Analyzed box vs. support vs. measure**: the kernel sees the
  **analyzed box** — an analysis-time choice (E2), = the support
  when bounded; the probability *measure* never enters kernel
  evaluation — it prices leaves and tail at reporting (E1/E6).
- **Certified / advisory**: certified = interval (or
  Dual<Interval>) containment-true; advisory = first-order f64
  (RSS, linearized contributions), always labeled, never gating.

## E1 — Distributions are document-layer parameter metadata; the Real channel is the per-leaf engine

**Decision**: a continuous `ParamDef` at the document layer gains an
optional `Distribution` (E2). The kernel and geometry lanes never
see a probability. The analysis lane projects a distribution to
exactly three consumables: (a) the **analyzed box** (E2) for
interval/driver work, (b) **seed vectors** for dual passes, (c) a
**measure** pricing leaves and tail in reports. No `Real`
instantiation carries probability; there is no `Distribution` scalar.

**This completes the Real-trait vision; it does not depart from
it.** `Real`'s original purpose was this feature, with Interval
intuited as a quasi-stand-in for a uniform distribution (Evan,
#110). That intuition is correct one level down: Interval turns out
to be the **sound integration kernel for ANY input measure** — each
leaf evaluates through the existing Interval `Real` (three-valued
per-leaf answers via decorations) and the measure integrates over
leaf verdicts; Dual rides the same way, per-leaf/per-chamber. The
scalar channel does all propagation; only pricing lives above it.

- Why measures cannot ride the scalar channel: dependency makes
  distribution arithmetic **wrong, not loose**. Interval dependency
  is sound-but-loose (`x−x` has width but contains truth); pushing
  marginals through operators *forgets correlation*, with no
  conservative direction to hide in ("wider" needs an order).
- **Rejected alternative — p-boxes/credal enclosures** (Fréchet
  bounds under unknown dependence), the rigorous scalar-channel
  analogue: they collapse toward vacuity within a few dependent
  operations, and shared-parameter dependence IS the kernel's
  workload. The measure therefore prices **leaves of INPUT space
  only**, where parameter identities still exist — derived-quantity
  correlation never needs representing at all.
- **Rejected alternative — F1 quantity extension**: making
  distributions typed quantities flowing through expressions invites
  exactly that marginal arithmetic. The F1 lattice stays a value
  lattice; distributions annotate *parameters*, not values.
- Counterargument (honest): you cannot state "this measurement is
  normally distributed" as an input. Correct — measurement
  distributions are *outputs*, and v1 deliberately does not even
  compute output densities (E11.6).
- Forecloses: distribution-valued expressions; per-node distribution
  overrides; probability inside evaluation code.

## E2 — v1 vocabulary: unbounded support welcome; the analysis box is an analysis-time knob; tail mass is accounted, never dropped

```
Distribution = Band            { lo, hi }             // worst-case only, NO measure
             | Uniform         { lo, hi }
             | Normal          { sigma }              // unbounded support
             | TruncatedNormal { sigma, lo, hi }      // sugar: tail_mass ≡ 0
```

Offsets are relative to the parameter's nominal (which stays the
single source of truth for the f64 build), dimensioned per F1 — a
Length parameter's band is Lengths. Bounded forms require
`lo ≤ 0 ≤ hi` (asymmetric legal; nominal outside its own support
is a typed document error).

- **The analyzed box is the analysis's knob, not the distribution's
  property.** Distributions may have unbounded support — no ad-hoc
  cutoff baked into the model. Each run chooses a bounded box
  (request config; default = the symmetric quantile box for a named
  default mass, a recorded policy dial like K). The choice only
  moves mass between analyzed and tail columns — never truth.
- **Tail-mass accounting**: mass outside the analyzed box is an
  explicit additive term in every result — `P(defect) ∈ [computed ±
  analysis bounds] + tail_mass` — reported, never dropped. In E6's
  accounting the tail is `Unanalyzed` mass alongside the refusal
  reasons; one **unresolved-mass budget** (refused + tail) is the
  single honesty gate (E10). Truncation = optional sugar, tail ≡ 0.
- **One-branch tails amendment (Evan, post-ratification 2026-07-27,
  #110 thread)**: the no-flips commitment (E6) is what makes the
  MERGED budget principled, not merely simple — under one branch,
  tail, `FlipCrossing`, and undersubdivided mass all mean the same
  thing ("the branch-valid analysis does not cover this mass"), so
  one budget with a diagnostic breakdown (widen box / subdivide /
  accept) is the honest shape. **Chamber containment**: if every
  leaf touching the analyzed box's boundary is `FlipCrossing`-
  refused, the witness chamber is contained in the box, ALL tail
  mass is provably off-branch (not merely unexamined), the
  unresolved budget becomes exact rather than conservative, and box
  growth has a natural stopping rule (growth can only relabel
  tail → `FlipCrossing`). Detection is a free predicate on the
  existing leaf set; E6's driver SHOULD report containment when it
  holds.
- **Band carries no measure** — pure worst-case. Any report needing
  a measure (RSS, leaf mass) over a Band parameter refuses typed,
  never defaults to uniform: "I know the limits but not the shape"
  is real information; uniform is a different, stronger claim.
- **Independence**: product measure only, one distribution per
  parameter; joint distributions are v1-foreclosed (E11.2; a `Joint`
  form is an additive schema variant later).

## E3 — A measurement is ONE dimension-generic recipe sink node

**Decision**: F4's vocabulary grows exactly one `Measure { expr }`
sink node — typed F1 quantity out, no body output. The quantity
kind rides the measured *expression* through the F1 lattice, never
per-kind node variants: measurement primitives are typed functions
in the F7 extension — `distance(a, b)`, `min_clearance(sel)` →
Length; `angle(a, b)` → Angle; mass properties in their own
dimensions (their Length-powers force the recorded *additive* F1
lattice growth, never a Measure-local type) — over StableName
entity references and node outputs.

Evaluated at every `T` like all nodes; failures poison descendants
only (F2 verbatim; sinks have none). Persisted as an ordinary node;
resolved through name tables with N-machinery's typed failure;
content-key cached like everything.

- **Rejected — per-kind Measure taxonomy** (`Distance`/`Angle`/…
  variants, the round-1 shape): a parallel type vocabulary beside
  F1 for zero expressive gain. **Rejected — lever-arm unification**
  of angle with distance: it requires a chosen length scale — an
  ad-hoc constant, exactly the class this project refuses.

- Rationale: measurements must be persisted, stable-named,
  diffable, scalar-generic, cache-keyed — exactly what recipe nodes
  already are. A side-channel query API would need a parallel
  persistence + naming + genericity story (the banked no-parallel-
  path principle forbids it), and its references would silently
  dangle; Measure nodes fail loudly through N5 diagnosis. And the
  sublanguage is total and finite by charter, so any expression is
  Measure-wrappable and dual/interval-evaluable by construction.
- Counterargument: DAG pollution — dozens of measurement sinks.
  Accepted; sinks are lazily evaluable, the GUI presents them as a
  panel, and the document is the right home for design intent
  (E10's assertions).

## E4 — Sensitivity semantics: forward Dual<f64>, one seed per parameter, chamber-local and marked as such

**Mechanism**: ∂m/∂pᵢ = evaluate the recipe at `Dual<f64>` with pᵢ
seeded, others constant; n parameters ⇒ n independent passes (pure
model; parallel under D9 idiom 1). The dual value channel is
bit-identical to the f64 run (THE Dual contract), so every verdict
— hence the topology — is the f64 build's: the sensitivity is of
the as-built body, guaranteed.

**Semantics honesty (the load-bearing clause)**: ∂m/∂pᵢ is the
derivative of the *fixed-topology program* (per the ratified kink
conventions), valid within the nominal's chamber; it can jump at a
predicate flip. Therefore **a sensitivity is never reported bare**:
it carries either a **chamber certificate** (an E6 leaf certified
over the box asked about) or the explicit **`local_only` marking**
— no third, unmarked state (D4 fail-loud applied to derivatives;
the classic stackup lie is extrapolation across a topology change).

**Certified tier**: `Dual<Interval>` over a leaf yields derivative
*enclosures* (Clarke straddle hulls at kinks, per M0), consumed for
E7's monotonicity pruning and E5's contribution bounds — never for
refusal decisions (E9).

**Not in v1**: reverse mode; vector-forward — n forward passes are
O(n·build), trivially parallel; both are performance additives
(E11.4).

## E5 — Stackup deliverable: a typed per-measurement report; certified worst-case gates, RSS is labeled advisory

```
Stackup {
  measurement:  StableName of the Measure node,
  nominal:      f64 value (the f64 build's),
  per_param:    [ { param, sensitivity (E4-marked), contribution } ], // advisory
  worst_case:   certified enclosure of m over the certified leaves, // gates
  rss:          Advisory<σ_m> | UnavailableBecause(Band params named),
  coverage:     certified mass + refused mass + tail_mass (E2/E6; sums to 1),
}
```

- **`worst_case` is the headline and the only gating number**: the
  hull of interval evaluations of the Measure node over E6's
  certified leaves — NOT the linearized Σ|∂m/∂pᵢ|·Δpᵢ (first-order,
  silently wrong under curvature; it survives only as the advisory
  per-contributor table).
- **`rss`** = √Σ(∂m/∂pᵢ·σᵢ)², linearized, advisory. Available only
  when *every* contributor carries a measure — a Band parameter
  yields `rss: UnavailableBecause(...)`, never a partial RSS (a
  partial RSS is still a lie).
- **`coverage`** keeps the report honest: "99.2% certified; 0.3%
  refused (reasons — flip-crossing mass included, E6); 0.5% tail
  (unanalyzed)". `worst_case` is certified over the analyzed box's
  witness-branch leaves; refused + tail say what it does not cover.
- Counterargument: engineers will read RSS as the answer. Mitigation
  is labeling and ordering, not omission — omit RSS and users
  compute it outside, unlabeled.

## E6 — The subdivision driver and `ParamBoxVerdict`

The Q1-promised driver, built as an analysis-lane service on the
`Real`-generic editor-core evaluation service (no parallel path).

```
drive(doc, box) -> ParamBoxVerdict {
  certified: [ Leaf { box, verdict_vector_key, results } ],
  refused:   [ Leaf { box, reason: SliverTerminal { predicate }
               | FlipCrossing { flipped predicates }  // no-flips v1
               | Bifurcation(WitnessBifurcation) | Infeasible  // E8
               | Budget { depth/work bound hit } } ],
  measure_accounting: per-reason mass under the product measure,
                      + Unanalyzed = tail mass outside `box` (E2),
}
```

**Leaf protocol**: replay the recipe at `T = Interval` over the leaf
box (parameters = intervals; witness data verbatim per E8).

- Every predicate definite AND the verdict vector matching the
  witness branch's → leaf **certified**; lineage-scoped key identity
  (Q1 PR 8) means the leaf shares the nominal build's topology;
  Measure enclosures are containment-true over the whole leaf.
- **No-flips v1 (adopted round 3; Evan's proposal)**: a leaf
  definite on a *different* verdict vector is **refused mass**
  (`FlipCrossing`, flipped predicates named) — no branch
  enumeration, no analysis of the far side. Topology change under
  tolerance is usually the reported defect itself; near a flip,
  subdivision only localizes the boundary to shrink refused mass.
  Branch enumeration = the recorded v2 door (with W3, E8).
- Any `Indeterminate` → **bisect** and recurse. Split rule named,
  deterministic (D9): max relative width, ties to lowest index.
  Leaves are independent — parallel under D9 idiom 1, CPU/rayon per
  the ratified GPU-boundary table.
- **Terminal sliver** (enclosure wholly inside (ε, Kε)) → refuse,
  never refine (ratified PR 7 semantics: a genuine semantic sliver).
- **Budget exhaustion** (depth/work caps, run config like K) →
  refuse with `Budget`, typed and priced. No silent partial answers.

**Probability enters exactly once**: leaf and tail masses under the
product measure, at reporting time — Normal CDF via f64 erf is fine
because *reporting decides no topology and gates no certification*.

**No chamber-connectivity claims**: certified leaves (all on the
witness verdict vector under no-flips) may be *presented* coalesced,
but the semantic unit stays the leaf — leaf results are
self-contained; connectivity proofs would be machinery for zero
certificate content.

**K telemetry (T6/K-REPORT obligation)**: every driver-path
predicate sample lands in the k_stats funnel — margins *driven
toward* zero by refinement are the first genuinely ill-conditioned
population K sees; an in-band landing there is exactly K-REPORT's
stated re-open trigger (#89 CLOSED — K = 10 is the permanent
ratified default).

## E7 — Clearance & self-intersection: a trichotomy over box × domain; duals accelerate, never decide

For a **certified leaf** (fixed topology), the analysis answers two
questions:
global self-intersection-freedom, and `min-clearance ≥ c` for a
named selection (a `min_clearance` Measure + E10 assertion).

**Mechanism**: two nested subdivisions. Outer: the E6 parameter
leaves. Inner: geometry-domain subdivision with interval exclusion
— the pre-M5 "SSI completeness is an interval obligation" posture
run with interval *parameters*: candidate face pairs from a
conservative interval BVH; a cell pair discharges at enclosure ≥ c,
splits when indeterminate, reports on a definite violation.

**The answer is a trichotomy, never silence**:

- `Holds` — clearance ≥ c certified throughout the leaf box × all
  domain pairs;
- `Violated { param_witness, geometry_witness }` — a definite sub-c
  distance at a concrete parameter point and closest-point pair
  (verified definite at f64 — "here, at these parameter values");
- `Refused { sliver | budget }` — the clearance margin `d − c` is a
  margined predicate like any other: terminal slivers and budget
  exhaustion refuse, typed and priced by measure.

Probability never enters *inside* a leaf — no "probabilistically
clear" verdict; mass accounting applies to leaves (E6), full stop.

**Self-intersection scope**: the tier-3′ census made global and
parametric — non-adjacent face pairs certified strictly positive
distance; adjacent pairs are covered locally by the wedge predicates
(their distance is legitimately 0). v1 geometry scope = the
carriers the kernel has interval evaluators for; carriers without
interval evaluation refuse typed (`Unsupported`), never downgrade
to sampling.

**Duals as pruning only**: over a leaf, a sign-definite
`Dual<Interval>` enclosure of ∂d/∂pᵢ makes d monotone in pᵢ — the
check restricts to a box facet, collapsing a dimension. An
accelerator only: correctness never depends on it (E9).

## E8 — Composition with the W-contracts: witnesses are fixed document state; walls become priced refusals

- **The analysis lane is read-only.** The driver NEVER writes the
  document: no auto-ReWitness however clean the certificate (W4's
  ban stands; analysis is not a commit context). Every leaf replay
  consumes the committed witness (W1) verbatim — leaf results stay
  a pure function of (doc, box).
- **Per leaf, sketch nodes run W2 at T = Interval**: the ratified
  contraction-from-f64-witness over the leaf's box; the certificate
  firing proves the *entire leaf* shares the witness's branch (W4's
  certified-same-branch invisibility, point upgraded to box).
- **When a leaf straddles a wall**, the certificate refuses and the
  driver bisects; terminal refusals split by kind, vocabulary
  preserved (W3 layer-2 language, never collapsed into "sliver"):
  - `Infeasible` — no real solution over part of the box (the
    elbow past straightening): typed refusal whose mass is a
    *product-level finding* — "2.1% of the tolerance mass has no
    solution" IS the detect-problems deliverable;
  - `Bifurcation(WitnessBifurcation)` — the box reaches across a
    fold/branch wall: refused with the W3 payload. **Distributions
    do not cross witness walls** — the model is undefined there
    without a recorded ReWitness, so the driver prices the mass and
    refuses; the remedy is user intent, never machinery guessing a
    branch. (E6's no-flips rule generalizes this to every predicate
    flip in v1; solver walls are its sharpest case.) (`solver_branch_margin` samples from driver runs feed
    E6's k_stats obligation.)

## E9 — Tangent poison never refuses

Ratified base: decoration-as-poison lives in the value lane;
`Decide` classifies the value channel only — tangent data does not
decide base-space topology. **Addendum**: in `Dual<Interval>`
work, derivative-channel degradation (Clarke straddle hulls
widening to the whole line, kink-jump enclosures like floor's
`[0, +∞]`) NEVER contributes to leaf refusal — refusal is decided
solely by value-channel predicates and W-certificates. A degraded
tangent forfeits exactly its uses: no monotonicity pruning (E7);
affected `per_param`/`rss` entries report `UnavailableBecause`
(E5); `worst_case` untouched (value-channel interval evaluation,
never linearization).

- Rationale: refusing on tangent poison would let `abs` at a kink
  veto an analysis whose value channel certifies cleanly —
  inverting the ratified hierarchy. A straddle hull containing
  zero IS information ("possibly non-monotone"), consumed as that.
- Counterargument: a stackup whose every tangent degrades is weak.
  True, and honest — it still carries the gating certified
  worst-case; the advisory columns degrade loudly, never lie.

## E10 — Reporting & persistence: distributions and assertions persist; verdicts are derived and CI-able

**Persisted, in-document** (additive F3 migration, one schema step;
fields named now):

- `ParamDef.distribution: Option<Distribution>` (E2 forms, offsets
  dimensioned per F1, shortest-round-trip floats as ratified);
- the `Measure` node (E3) with its StableName references;
- `Assertion { measure: NodeId, bound: Quantity, dir: AtLeast | AtMost }`
  — tolerance *requirements* as recorded design intent (the CAD
  analog of a test suite: "min wall ≥ 0.5 mm" lives in the document,
  versioned and diffable, not in a script beside it).

Unknown-field/version handling per F3 verbatim; the migration chain
gains one explicit version-to-version step.

**Derived, never persisted** (D3's "the recipe IS the save"):
`ParamBoxVerdict`, `Stackup` reports, clearance verdicts —
content-key cached on the bit-content of (recipe slice, box, ε, K;
D9 makes the key the proof), serializable for CI goldening.

**CI rows this MVP adds**: (1) assertion gating — corpus assertions must
certify (`Holds`) with refused + tail mass within the recorded
unresolved-mass budget (E2); `Violated`/`Refused`/budget overrun
fail loudly; (2) goldened refusal- and tail-mass accounting on a
margin-thin fixture (the honesty metric is itself regression-
tested); (3) k_stats funnel rows for driver + solver predicates
(the K re-examination evidence, E6/E8).

**Open sub-question**: should a failing Assertion gate `build()`?
v1 says no — assertions report; a gating mode is additive policy.

## E11 — What the MVP does NOT do (loud)

1. **Monte Carlo never gates** *(softened round 3, per Evan:
   "probably fine" is fair once probabilities are on the table)*.
   Certified intervals remain the ONLY gate; MC joins as a labeled
   advisory estimator lane (the RSS pattern — pure replay makes
   sampling trivial). Label discipline: MC results carry sample
   count + seed (fixed, recorded, D9-deterministic); never
   persisted as Assertions.
2. **No correlated/joint distributions** — product measure only
   (E2); `Joint` is an additive schema variant later.
3. **No distributions on structural (Count) parameters** — typed
   refusal. "Hole count ~ Uniform{3..5}" is design-space
   exploration, not tolerance analysis (D8's explicit regime).
4. **No reverse-mode AD, no vector-forward duals** (E4) —
   performance additions, not semantic ones.
5. **No GD&T semantics.** Stackups are parameter-space facts; ASME
   Y14.5 is a language layer that could later *compile to* Measure
   nodes + assertions — the MVP declines to speak it approximately.
6. **Output densities deferred post-v1** — true pushforward is v2.
   Near-free v1 note: leaf-mass × output-enclosure histograms are
   an ADVISORY visualization (each certified leaf spreads its mass
   over its output interval); zero new soundness claims.
7. **Imported bodies carry no parameters** (D7): nothing to vary;
   they participate in clearance checks as constants.
8. **No optimization/inverse loops** ("resize until clearance
   holds") — consumers of the MVP's reports, not part of it.

## Worked example: the two-hole plate

Plate width w (Uniform ±0.1 mm), hole diameters d₁, d₂ (Normal
σ = 0.02 mm, unbounded); `Measure { distance(hole1_wall,
hole2_wall) }` (the web) and `Assertion { web ≥ 0.5 mm }` in the
document. The analysis takes the default ±3σ quantile box for the
d's — tail mass 1 − 0.9973² ≈ 0.54%, carried additively throughout.

- Driver (E6): the analyzed 3-box certifies in four leaves after
  one bisection in w (a coplanarity predicate goes indeterminate at
  small w); one terminal-sliver leaf refuses — 0.4% of the mass. A
  far-side-definite leaf would be `FlipCrossing` refused mass
  (no-flips v1). Accounting: 99.06% certified, 0.4% refused,
  0.54% tail.
- Stackup (E5): ∂web/∂w = +0.5, ∂web/∂dᵢ = −0.5, chamber-certified;
  certified worst-case web ∈ [0.487, 0.613] mm.
- Verdict: the assertion FAILS with a parameter witness (w = lo,
  dᵢ = box hi) — while RSS says σ_web ≈ 0.017 mm, "3σ fine." Both
  print, the tail rides every line, the certified number gates.
  That divergence — certified worst-case vs. RSS optimism — is the
  MVP's reason to exist.
- Clearance (E7): `Holds` on three leaves; on the fourth,
  monotonicity pruning collapses w to a facet, which certifies.
  Self-intersection-freedom certifies everywhere.

## Open after this doc

- **Driver constants**: split rule, budgets, default analyzed-box
  quantile mass (E2's dial), leaf coalescing — PR-spec, not
  blockers.
- **Assertion gating of `build()`** (E10's flag): report-only vs. a
  refuses-while-violated mode — needs editor-core UX input.
- **Vector-forward duals / reverse mode** — pure performance;
  revisit when the Band 4 corpus prices n-pass sensitivity runs.
- **MC advisory lane concretes** (E11.1): sampler/PRNG choice,
  sample-count defaults, report presentation — PR-spec.
- **Branch enumeration (v2)**: analyzing the far side of a flip;
  composes with WitnessBifurcation (E6's recorded door).
- **Correlated distributions**: real tolerance chains correlate
  (same machining setup); additive schema, but leaf-mass accounting
  must then integrate non-product measures.
- **Clearance `c` as a Band**: should the assertion bound carry its
  own tolerance? v1 says exact; revisit with GD&T-adjacent cases.
- **SetTolerance × distributions**: sliver-refusal mass depends on
  ε, so ε edits move coverage numbers; the SetTolerance diff should
  surface coverage deltas. Wiring is PR-spec.
- **Naming-pillar composition**: Measure verdict vectors should
  join the N-machinery diff reports; confirm at implementation.
