# M10-5 — clearance and self-intersection (E7)

STATUS: BINDING (dispatched 2026-08-30). Unit branch
`m10/m10-5-clearance`. Program plan `work/m10/plan.md`; design
record `docs/ERROR-DESIGN.md` E7/E8/E9 (read all three in full),
with E6/E2 as consumed substrate.

## Grounding (substrate facts; verify each at the site)

- **The E6 driver is the outer loop** (M10-3, merged): certified
  leaves from `analysis::drive`, additive accounting, the receipt
  identity, flip naming through `resolve::vdiff`. This unit runs
  INSIDE a certified leaf; it never re-litigates leaf certification
  and never touches mass accounting except to consume it.
- **`crates/bvh` is `T: Bounds`-ready at the `Aabb`** (the plan's
  substrate note) and the **interval lift of `crates/bvh` is THIS
  unit's claimed territory** — the S-CERT territory registration on
  PR #1200 (2026-08-29) claims it for M10-5; build it here, don't
  wait on anyone.
- **Interval geometry evaluation exists for the closed-form
  carriers** (planes, cylinders, spheres, lines, the M10-P-lifted
  profile geometry); carriers WITHOUT interval evaluators are the
  refusal set — v1 scope is exactly the carriers the kernel can
  evaluate at `Interval`, per E7 ("refuse typed, never downgrade to
  sampling").
- **The W2 sketch solver was never built** (plan Q1: solver OUT),
  so E8's solver walls are vacuous in v1: `Infeasible` and
  `Bifurcation` stay unreachable exactly as M10-3 documented them
  at the type. Change nothing there; state it.
- **M10-2 deliberately shipped no `min_clearance`** ("no variant,
  no placeholder, no refusing stub"). This unit does NOT add the
  persisted Measure kind either (§Out of scope) — the query door
  here is analysis-lane API, schema untouched.
- **The tier-3′ census** is the local/static self-intersection
  instrument this unit makes global and parametric; read its
  predicate names before minting any (the funnel rule applies to
  every new margined compare).
- **VERBS' registered demand** (on the plan PR): the curved-neck
  shell wall-clearance case, with ready fixtures in-tree — the
  ordinal-101/103 probe suites' dumbbell/hexagon families; #1019's
  perf box names the shell body as a measurement fixture. #1055 is
  the named consumer issue.

## Scope

### 1. The interval BVH

Lift `crates/bvh` construction and traversal to `T: Bounds`
(conservative boxes at `Interval` — every real configuration in the
leaf's box is inside the interval `Aabb`s). Deterministic (D9) —
same tree, same traversal order, at every scalar and schedule. The
BVH's job here is candidate PRUNING only: no discharge decision
rides a BVH test alone; a pair the BVH cannot exclude goes to §2.
Keep the lift additive — the f64 BVH's behavior and its GUI
consumers bit-identical (merge-base differential is the review's
signal).

### 2. The clearance engine — the inner subdivision

`analysis::clearance(doc, leaf, selection_a, selection_b, c, tol)`
→ the E7 trichotomy, over ONE certified leaf (the caller iterates
leaves; a driver-level convenience that folds over a
`ParamBoxVerdict`'s certified leaves and prices refused/tail mass
via M10-3's accounting is in scope, thin).

- Candidate face pairs from the interval BVH; per pair,
  geometry-domain subdivision with interval exclusion: a cell pair
  DISCHARGES when the distance enclosure is definitely ≥ c, SPLITS
  when indeterminate, and reports a definite violation when the
  enclosure sits definitely < c.
- **The margin `d − c` is a margined predicate like any other**
  (E7's sentence): the discharge/violation compare goes through the
  funnel with its own ledger row (the F16/`assert_bound` precedent
  — argue the row honestly, `LEDGER_FLAGGED_SITES` bumped). No raw
  float compare anywhere in the engine.
- **Trichotomy, never silence**: `Holds`;
  `Violated { param_witness, geometry_witness }` with the witness
  VERIFIED DEFINITE at f64 (a concrete parameter point + closest
  point pair — "here, at these parameter values"); `Refused
  { sliver | budget }`, typed. Sliver takes M10-3's rule shape
  (the deciding enclosure wholly inside the funnel's band); budget
  takes named dials.
- **Receipt identity, reused**: discharged + violated + refused
  cell-pairs account for every candidate pair (the SSI-exhaust /
  M10-3 template — state the identity, ride it on the result,
  tripwire it).
- Probability never enters inside a leaf — no sampling, no
  "probably clear".

### 3. Global self-intersection

The census made global and parametric: over a certified leaf, every
NON-ADJACENT face pair of the selection's body (adjacent pairs are
the wedge predicates' business — their distance is legitimately 0)
certified strictly positive distance through the same §2 engine
with c = 0⁺ (the exact spelling of "strictly positive" is the
implementer's, but it must be a funneled compare, not an ε).
Carriers without interval evaluators refuse `Unsupported` naming
the carrier kind — never skip, never sample.

### 4. Monotonicity pruning — accelerator only

Over a leaf, a sign-definite `Dual<Interval>` enclosure of ∂d/∂pᵢ
(through M10-4's seed door composed with the leaf's box) restricts
the check to a box facet. E9 governs: a degraded tangent forfeits
the pruning and NOTHING else; correctness must never depend on the
accelerator (pin: engine results with pruning forced OFF are
identical, on a fixture where pruning fires). If M10-4's door has
not merged when this lane needs it, build behind a small internal
seam and land the composition when it has — the accelerator is the
LAST section for a reason; everything before it is
interval-value-channel work only.

### 5. e2e — the VERBS acceptance and the honest limit

- The dumbbell/hexagon shell families (in-tree fixtures): a
  wall-clearance certificate over a real parameter box — `Holds` at
  a generous c, `Violated` with an f64-verified witness at a c the
  geometry actually breaks, `Refused` priced at a budget-starved
  config; receipts hold on all three.
- The **#1055 arm is a STRETCH** (Ev's Q5 ruling, follow-up-unit
  valve): the curved-neck shell wall-clearance window closed by
  this certificate. Attempt it after everything above is green; if
  it does not fit, the valve is a named follow-up with the blocker
  stated — never a rushed landing.
- State the honest limit the way M10-3 did: the widths at which
  cells discharge (measure them), and what a macroscopic clearance
  question can and cannot get today; the widening class's home is
  issue 1191 — cite, don't re-file.

## Out of scope

The persisted `min_clearance` Measure kind and its assertion wiring
(a schema move; it rides M10-6's reporting unit, which will cite
this engine as its evaluator — leave the engine's API shaped so
that door can call it). Any schema/node change at all. Auto-fix or
ReWitness (E8's read-only rule — the analysis lane never writes).
Solver walls (Q1). Sampling of any kind (E11.1 is M10-6's advisory
lane). The GUI surface. Curved-carrier interval evaluators beyond
what exists (refusal is the v1 answer).

## Review claims to falsify

1. Zero impact: the bvh lift leaves every f64 consumer
   bit-identical (merge-base differential); no existing evaluation,
   key, or persisted byte moves.
2. Conservativeness: the interval BVH never excludes a pair that a
   denser interval evaluation shows within c (adversarial
   near-touching fixtures); no discharge rides a BVH test alone.
3. Funnel discipline: the `d − c` compare and the strictly-positive
   compare are funneled with ledger rows; no raw float compare and
   no new ε anywhere in the engine (grep + behavioral).
4. Trichotomy totality: no input reaches silence — every
   (leaf × pair) lands in exactly one of the three arms; the
   receipt identity holds on every run you can construct, budget
   exhaustion included.
5. Witness honesty: every `Violated` witness re-verifies definite
   at f64 independently (recompute the distance at the witness
   point yourself); no witness is an interval midpoint that f64
   contradicts.
6. Self-intersection scope honesty: adjacent pairs excluded exactly
   per the wedge rule; unsupported carriers refuse naming the kind;
   nothing samples.
7. The accelerator is removable: pruning OFF ⇒ identical verdicts
   everywhere; a degraded tangent forfeits pruning only (E9 live).
8. D9: deterministic across repeats and schedules on a
   multi-thousand-cell run of your own construction.

## Acceptance

Hosted CI green on the drawn point (trailer-pin
`lane=interval`/`eps=1e-12` if the draw misses interval — the whole
unit is interval work); suites named per basename conventions;
every deviation reported in the PR body; the VERBS acceptance
fixtures exercised; the #1055 disposition stated either way
(landed, or the valve with the blocker named).
