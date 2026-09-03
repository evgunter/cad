# M10-7 — parameter-aware certification: the symbolic identity tier (E12), the extent lever (E3)

STATUS: BINDING (dispatched 2026-09-03; ERROR-DESIGN revision E12
ratified by Ev on PR #1712 the same day; every cited name re-verified
against main at dispatch).
Unit branch `m10/m10-7-symbolic`. Program plan `work/m10/plan.md`;
design record `docs/ERROR-DESIGN.md` E12 (read in full, twice) with
E3's amendments, E6 as the consumed substrate, and the worked example.
Ev's ruling that opened this unit, verbatim: "we need to make this
parameter-aware so it's usable; that's the whole point of this
machinery so I don't think we can close the program until then."

## Grounding (substrate facts; verify each at the site)

- **The defect, pinned.** `crates/editor-core/tests/m10_3_driver_interval.rs`
  — `a_macroscopic_box_refuses_all_of_its_mass_as_budget_today` and
  `the_certification_width_is_a_small_fraction_of_epsilon`. The module
  docs there state the mechanism precisely (the dependency problem, NOT
  the closed period-fold class of issue 1191): a certification identity
  mentions its parameter several times, the two sides arrive at the
  funnel site as two separately evaluated intervals, and the enclosure
  of their difference is `[0, c·w]`, c ≈ 2–4. Three predicates OBSERVED
  widening: `carrier_endpoint_start/end`
  (`crates/geom-brep/src/certify.rs`, "Check 3: endpoint pinning",
  `Margin::of(spec.carrier.eval(t0).distance(start))`) and
  `side_planes_cosurface` (`crates/sweep/src/extrude.rs`'s
  `SIDE_COSURFACE`, decided in `crates/sweep/src/swept.rs`'s
  `cosurface`). PR #1231's sweep lists 57 identity-shaped names; its two
  grep commands are in that PR body and are re-run here (§4).
- **The funnel.** `geom_core::k_stats::decide(name, Margin<T>, band)`
  → the private `classify` → `T::sign_within(band)` — the ONE
  classification seam (`crates/geom-core/src/k_stats.rs`). `Decide:
  SpanLocate` (sealed, `crates/geom-core/src/spline/locate.rs`), so a
  new deciding scalar lives IN `geom-core`. `Interval`'s `sign_within`
  (`crates/geom-core/src/interval.rs:588`) is the numeric protocol this
  unit puts a symbolic step in front of. `Margin<T>` is
  `#[repr(transparent)]` with blessed construction doors only
  (`crates/geom-core/src/predicate.rs`).
- **The scalar set.** `Real` (`crates/geom-core/src/real.rs:95`: `Copy
  + Add/Sub/Mul/Div/Neg + Send + Sync + 'static`, plus `from_f64`,
  `zero/one/pi/tau`, `sqrt/abs/powi/sin_cos/tan/asin/acos/atan/atan2`,
  `min/max/floor/copysign`, `is_poison`, the periodic reducers).
  Implementors: `f64`, `Interval`, `Dual<T>`, `Probe`. What evaluation
  requires is `editor_core::eval::EvalScalar`
  (`crates/editor-core/src/eval/mod.rs:1324`): `Decide + ContentBits +
  Bounds + Send + Sync + topo::AtRestPolicy + AxisScalar + SeedScalar +
  MinClearanceLane`.
- **Where parameters enter.** `analysis::param_env_over::<T>(doc, box)`
  (`crates/editor-core/src/analysis.rs:865`) binds each continuous
  parameter through `AxisScalar::axis(lo, hi) -> Option<Self>` — the
  door has NO parameter name today. The driver's leaf protocol is
  `drive::classify` (`crates/editor-core/src/drive.rs:1106`): one
  `evaluate::<Interval>` over the leaf box, then definiteness and the
  exact `VerdictVector` comparison against the f64 witness.
- **K telemetry.** `MarginSample { predicate, margin, band_zero,
  band_escalate, outcome: SampleOutcome::{Definite, Indeterminate,
  Invalid} }` behind `probe` (`crates/geom-core/src/k_stats.rs:300+`);
  the driver's K row replays certified leaves at `Probe`
  (`KProbe::CertifiedMidpoints`); the hosted row is M10-6's (rule 1
  gates, `docs/K-REPORT.md`'s M10 addendum).
- **The lever sites.** `crates/editor-core/src/eval/measure.rs` —
  module docs "That arm is `max(separation, 1 m)`" and `fn arm` (the
  comment there is the design statement this unit executes); the
  sibling at `crates/editor-core/src/mate.rs:258`. The E3 amendment
  (ratified) names the replacement: an UPPER bound on the operands'
  extent, no floor.
- **Exact arithmetic available.** `num-rational`, `num-bigint` and
  `num-traits` are already in `Cargo.lock`; whether `geom-core` may
  take them is the dependency policy's call
  (`memories/review-and-dependency-policy.md`) — the fallback is an
  in-tree `i128` rational whose overflow FREEZES the form (§1.3),
  which is sound by construction.
- **Rulings that bind here**: E12 verbatim (a symbolic `Zero` is a
  theorem, never a heuristic; no funnel site is edited; the numeric
  channel's soundness argument is untouched; the frontier is S-CERT's
  `work/cert/param-box-certification-of-implicit-quantities.md`);
  no-flips v1; D9 (bit-identical across repeats and rayon schedules).

## Scope

### 1. `Sym<T>` — the symbolic tier scalar (geom-core)

1.1 **The type.** `geom_core::sym::Sym<T>`: the lane value `T` plus a
DAG node handle. `Copy` (the handle is an id). Every `Real` op computes
the value at `T` verbatim and mints one hash-consed node; `Sym<T>`
implements `Real`, `Decide`, `SpanLocate`, `Bounds`,
`CertifiedEnclosure` by DELEGATION to `T` for every channel but the
sign decision (1.4). Instantiated in this unit at `T = Interval`
(the driver's replay) and `T = Probe` (the K row, 3.3); generic over
`T: Real` where the code allows so the instantiation set stays a
closed enum by policy, not by accident.

1.2 **The DAG.** Nodes: `Param(symbol)`, `Lit(f64 bits)`, `Pi`,
`Add/Sub/Mul/Neg`, `Div` (as `Mul(a, Inv(b))`), and OPAQUE unary/
binary atoms for every other `Real` op (`sqrt`, `abs`, `powi(n)`,
`sin/cos/tan/asin/acos/atan/atan2`, `min/max/floor/copysign`, the
periodic reducers) keyed by their children. Node ids are CONTENT
HASHES (a structural 128-bit hash of `(op, children ids, payload
bits)`), never sequence numbers, so an id is the same under every
rayon schedule and every insertion order — D9 for free. One
hash-consing table PER LEAF REPLAY, dropped with the leaf (nothing is
shared across leaves; nothing persists). A poisoned `T` value keeps
its node — poison is the numeric channel's business.

1.3 **The identity test.** `is_identically_zero(node) -> bool`: the
node's POLYNOMIAL NORMAL FORM over the parameter symbols with EXACT
RATIONAL coefficients (every `f64` literal is a dyadic rational,
embedded exactly; `Pi` and every opaque atom are indeterminates keyed
by the normal form of their arguments, `Inv(b)` an atom keyed by
`NF(b)`), computed LAZILY, memoized per node id, and only ever asked
by `sign_within` (1.4). The form is zero iff every coefficient is
zero. **Freezing budget**: a form whose term count or total degree
exceeds the dial (§5) — or whose coefficient arithmetic overflows the
chosen rational — is FROZEN: the node becomes an indeterminate of its
own, cancellation through it is lost, soundness is not. Freezing is
counted (3.2). Nothing in the normal form ever reads a value.

1.4 **The decide protocol.** `impl<T: Decide> Decide for Sym<T>`:
`sign_within(band)` first asks `is_identically_zero(self.node)`; if
true, returns `Ok(Sign::Zero)` WITHOUT consulting the enclosure and
records the outcome as symbolic (3.2/3.3); otherwise returns
`self.value.sign_within(band)` verbatim. That is the whole change to
decision-making, and it happens INSIDE the scalar, so `decide`,
`classify` and every funnel site are untouched by construction (E12).
A symbolic `Zero` is a theorem about real arithmetic; state the
argument at the impl and pin it (§4 claim 2).

1.5 **The evaluation seam.** `Sym<Interval>` satisfies `EvalScalar`:
`ContentBits` feeds the VALUE's bits only (provenance is not content —
two builds with the same numbers memo the same); `AtRestPolicy`,
`SeedScalar`, `MinClearanceLane` delegate to `T` — `MinClearanceLane`
STRIPS to `Interval`, runs the engine there and re-wraps the bracket as
atoms (the clearance search decides interval-valued window margins,
not identities; tracking it buys nothing and costs a DAG per cell). One
seam change: `AxisScalar` gains `fn axis_named(name: &ParamName, lo,
hi) -> Option<Self>` with a default delegating to `axis`, and
`param_env_over` calls it — `Sym<T>` mints `Param(symbol(name))` there
and `T::axis` for the value. Every existing scalar is unaffected
(claim 1).

### 2. The driver replays at `Sym<Interval>`

2.1 `drive::classify` evaluates the leaf at `Sym<Interval>` instead
of `Interval`; the f64 witness pass stays plain `f64` (a point
residual is tight — the witness still catches a constructor that does
not build what it claims). The exact `VerdictVector` comparison is
unchanged in shape: a symbolic `Zero` is a definite `Zero` row.

2.2 **The ceiling, re-measured.** The M10-3 limit row
(`a_macroscopic_box_refuses_all_of_its_mass_as_budget_today`) FAILS
by design when the widening closes: re-cut it as the positive pin —
the ±0.05 band on the 1.0 nominal certifies, with every refusal a
`FlipCrossing`, `SliverTerminal` or the tail, never `Budget` at the
depth floor. Then MEASURE the new ceiling on the two-hole plate
(`demos/tour/src/tolerance.rs`'s document, through the public doors):
the widest box that certifies fully, what refuses first beyond it and
why (the E6 worked example predicts a coplanarity predicate at small
w). The number is the deliverable, whatever it is.

2.3 **The acceptance study.** The tour's stop 1 — ±0.05 mm on the
spacing, σ = 0.01 mm on the radii — returns certified leaves, and its
stackup gates on a certified worst case; the captions and the cell's
test move with it (the "NothingCertified is the answer" stop becomes
the certified study; the ε-scale stop 2 stays as the divergence
exhibit or is retired with the argument stated). The E10 row-1 budgets
for the registered documents are re-recorded from the new drives.

### 3. Honesty instruments

3.1 **The census (§4 claim 4).** Re-run PR #1231's two sweep commands
on the unit's head; classify every name as EXPLICIT (its margin has an
expression in the parameters on the M10 corpus + the tour documents,
decides symbolically where it is an identity) or IMPLICIT (an iterated
quantity — S-CERT's frontier item) or NOT-A-PREDICATE (the sweep's
known false positives), with the evidence per row (the symbolic-zero
count from 3.2 or the site's construction). The table lives in
`geom_core::sym`'s module docs and the PR body; the implicit rows are
appended to S-CERT's item by name.

3.2 **The receipt.** `ParamBoxVerdict` gains per-leaf and total counts
`symbolic_zero`, `numeric` and `frozen` decisions, riding `serialize()`
/ `content_key()` / `render()` (M10-6's three doors) — the E12 evidence
a reader can see. The goldens that move, move once, with the
re-bless procedure stated.

3.3 **The K row.** `SampleOutcome::SymbolicZero` behind `probe`;
`Sym<Probe>` records it through the existing sink; the driver's
`CertifiedMidpoints` replay runs at `Sym<Probe>` so the hosted driver K
row (M10-6's) reports the symbolic/numeric split per `docs/K-REPORT.md`
(a new column in the M10 addendum; rule 1 still gates; a symbolic Zero
is never a rule-1 sample because no margin was classified).

### 4. The extent lever (E3 amendment, ratified)

`eval/measure.rs`'s `arm` becomes an UPPER bound on the extent of the
two operands together — the carrier windows' diameter where the
operands are faces (M10-5's BVH boxes carry it), the edge's/axis
segment's length where they are curves — with NO floor; the module
docs' "that arm is `max(separation, 1 m)`" paragraph and the `arm`
comment are rewritten to state the shipped lever. The `mate.rs:258`
sibling takes the same lever. The K population for
`bool_plane_parallel` / `carrier_cyl_axis_parallel` MOVES: report the
before/after distribution per the K-REPORT runbook (a demotion-class
change needs its addendum), never re-tune ε or the band. A pair with a
genuinely zero extent (a degenerate operand) refuses TYPED at the
measure — it cannot occur for a validated face, so the arm is the
row that proves it.

### 5. Dials, as shipped

`DriveConfig` gains `symbolic: SymbolicDials { max_terms, max_degree,
enabled }` with defaults stated and argued (start at 4096 terms /
degree 16; measure on the corpus and the plate; the freezing count of
3.2 is the evidence); `enabled = false` reproduces today's numeric-only
replay bit for bit (claim 1's differential).

## Out of scope

Affine forms (rejected at E12); per-site re-association (rejected);
provenance tokens (E12's reserve — only if the census names a family
the tier misses, and then as a DISCLOSED deviation with the family);
implicit quantities over a box (S-CERT's item); branch enumeration;
symbolic simplification beyond the polynomial normal form (no
factoring, no `Inv(b)·b` cancellation, no trigonometric identities —
each is an opaque atom; a documented limit, not a bug); persisting any
DAG; the GUI.

## Review claims to falsify

1. **Zero impact with the tier off**: `symbolic.enabled = false`
   drives, keys and prices every M10 fixture and the corpus
   bit-identically to the merge base (differential over serialized
   verdicts and receipts); every scalar other than `Sym` is unchanged
   by the `AxisScalar` seam.
2. **A symbolic `Zero` is a theorem**: construct margins that are
   identities by different routes (`P + t·d − P` against `t·d`;
   `(a+b)−b−a`; a cross product `d × d`; the endpoint pin on a real
   extrude) and margins that are COINCIDENCES at the nominal (two
   segments collinear at p₀ only) — the former decide symbolic Zero at
   every width, the latter never do and widen with the box; plant a
   wrong constructor (an endpoint off its carrier by 3ε) and show the
   f64 witness pass catches it.
3. **Freezing is sound**: a form over the term budget decides
   numerically, never falsely zero; the frozen count is reported; a
   budget of 0 reproduces claim 1.
4. **The census is complete and honest**: every one of the 57 names
   is in exactly one bucket with evidence; the three observed
   predicates decide symbolically on the fixtures that pinned them.
5. **D9**: the DAG's node ids, the receipts and the serialized verdicts
   are bit-identical across repeats and across the rayon schedule; the
   per-leaf table shares nothing.
6. **The ceiling moved and is measured**: the M10-3 limit row is
   re-cut and green; the plate's real study certifies; the new ceiling
   is a number in the PR body with the first refusal beyond it named.
7. **The K instrument sees the tier**: `SymbolicZero` rows appear in
   the driver population on the hosted k-lint axis; rule 1 still
   gates; the split is reported.
8. **The lever**: two walls 10 mm apart tilted 1e-8 rad now read
   parallel (deviation across their extent 1e-10 m); two planes
   crossing within ε of the reference point at 45° do NOT; no floor
   constant remains at either site; the K distribution's move is
   reported, not tuned away.
9. Every deviation from this spec is in the PR body's deviations
   table with the argument (the spec's shapes are binding, its
   numbers are starting points).

## Acceptance

Hosted CI green on the drawn point plus the interval lane at 1e-12 and
the k-lint axis pinned by trailer on the final head (the driver K row
must EXECUTE — read step conclusions); the re-cut M10-3 row green; the
plate's real study certifying; the census table; the receipts and K
split reported; every deviation in the PR body. After merge the
orchestrator re-cuts `docs/M10-EXIT-WALK.md` (#1700) for ratification.
