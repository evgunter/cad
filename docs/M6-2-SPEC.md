# M6-2 spec — the SSI generic-T lift (binding)

Mandate (docs/M6-PLAN.md unit 2): lift the SSI enclosure/
certification stack off `f64` — `Box3` / `NurbsSurface::project` /
`certify_branch` and what they force — so `Pcurve::Fitted` becomes
admissible; land `Pcurve::Fitted`; and close M5-walk row 2 by
making the fitted-cache-at-rest clause NON-VACUOUS. Blocker map =
M5-LOG PR 9c deviation 2 (docs/M5-LOG.md:1739-1756) + the S13
NURBS re-gate (crates/topo/src/boolean/mod.rs:556-568). This spec
is binding: deviations are REPORTED (numbered, with the executed
blocker), never improvised silently.

## 1. Scope, in three legs

**Leg A — the lift.** Make the between-samples envelope machinery
generic over the scalar:

- `ssi/enclose.rs`: `Box3` stays a C9-ring object (its fields are
  `RingInterval`s and `RingInterval` deliberately does NOT
  implement `Real` — geom-core/src/real.rs:~455). The lift is at
  the SEAM: constructors/entry points (`around`, `between`, `pad`,
  `ring`, `constv`, `subp`, `implicit_enclosure`,
  `implicit_gradient_enclosure`, `graph_margin`, `NurbsBoxes`)
  accept the generic scalar and cross into the ring via the
  `Enclosure`/`Bounds` seam (`lo`/`hi`), per the documented
  discipline. Widths/centers returned for DECISIONS go through
  `Decide`/`Bounds`-bounded code, not raw `f64` in evaluation
  positions.
- `geom-surfaces/src/projection.rs`: lift `project` /
  `project_seed` / `project_from_seed` off the `impl
  NurbsSurface<f64>` block. House idiom is the split-impl
  (pcurve_cache.rs precedent): evaluation pieces on `T: Real`,
  the Newton/convergence decisions on `T: Decide`.
  `SurfaceProjection` payload becomes `SurfaceProjection<T>`.
  The geom-curves sibling door is OUT of scope unless the lift
  transitively requires it (report if so).
- `ssi/certify.rs`: `certify_branch` + its transitive closure
  (`analytic_limbs`, `nurbs_limbs`, `box_chain`, `probe_tube_*`,
  `tube_ladder`, `refined`, `composite_form`, `tube_boxes`,
  `witness`, `SsiCertificate`, `SsiLimb`) and the public door
  `ssi::certify_rung3`. Carriers follow the ratified
  "f64-structure + T-lift, not T-generic" pattern (M5 PR 10
  dev 3; `sweep::skin::lift_surface` precedent): construction
  stays f64, evaluation is generic.
- `ssi/jet.rs`, `ssi/march.rs`, `ssi/system.rs` stay f64-only BY
  DESIGN (untrusted candidate generation; CI-allowlisted). Do not
  lift them.
- Scalar coverage: the certified lane must exist at `f64` and
  `Interval` (the point of the lift). `Probe` should come along
  where the bounds allow. `Dual` may take the explicit
  PropsQuadLane-style posture (`topo/src/props.rs:337-400`
  precedent: an impl that instantiates none of the certified
  machinery and says so) if the certificate cannot exist there —
  typed and documented, never a silent die.

**Leg B — `Pcurve::Fitted`.** Per the standing doc
(pcurve_cache.rs:150-183): a second variant carrying the fitted
pcurve, an `Arc` payload (the `Surface` M5-PR3 precedent),
dropping `Copy` from `Pcurve`/`PcurveCache` — the ~35-site ripple
dev 2 sized (pcurve_cache.rs ~24, topo/body.rs ~7,
topo/pcurves.rs ~2). Laws, binding:

- `PcurveCache::recertify` RE-DERIVES the whole certificate at
  rest — never trusts the stored one. For a fitted pcurve the
  between-samples envelope is the C2.2 hull bound computed by the
  now-generic `ssi::enclose`/`ssi::certify` machinery. The full
  C2 certificate = hull sup-norm + uniqueness tube; a
  schedule-max-only cache at rest is a spec violation.
- `PcurveCertifyError::UnsupportedCarrier` retires for the rung-3
  class it gated, via the S9 flip pattern (the refusal pin flips
  to pin the answer, history kept in the test name/docs). Any
  carrier class still unsupported keeps a typed refusal with
  honest, updated text.
- Persistence posture is unchanged: caches re-derive on load,
  nothing pcurve-shaped in the persisted bytes
  (editor-core/tests/m5_pr6_pcurve_persistence.rs) — `Fitted`
  adds no schema; the load path exercises `recertify`.

**Leg C — walk row 2 goes non-vacuous (the unit's acceptance).**
Quoting the walk (docs/M5-EXIT-WALK.md:111-118): the row is "an
acceptance obligation of that unit, not a follow-up to it."

- A cylinder×sphere rung-3 edge reaches a body AT REST carrying a
  `Pcurve::Fitted` cache with the full C2 certificate. Path of
  least invention: the `body_with_rung3_edge()` scaffold
  (topo/tests/m5_pr7_split_meter.rs:105-140) grown into an
  at-rest body row; a real constructor path is better if one is
  honestly reachable, but do NOT wire the cyl×sphere join window
  to get it (non-goal, see §3).
- `no_body_at_rest_carries_a_nurbs_carrier_or_face`
  (step-export/tests/m5_pr13_curved.rs:597) FLIPS with history —
  it currently pins the vacuity positively. Its successor states
  the new law (which bodies may carry fitted caches, and that
  they carry full certificates).
- The row runs multi-ε (1e-6 / default / 1e-12; band-relative
  placement per the m5_pr7_ssi.rs:52-58 discipline — the
  `FitSampleBudget` stand-down at 1e-12 is an acceptable typed
  outcome if it fires, pinned as such) AND in the Interval lane
  (loud-skip + inner `mod certified` pattern,
  m6_surgery_interval.rs precedent). The Interval-lane row is
  the non-negotiable one: it is the evidence the lift happened.

## 2. Discipline constraints (will fail CI if ignored)

- `Real` may not grow bounds inline: no `T: Real + …` anywhere
  (ci.yml:201-214). Use `Real`, `Bounds`, `Decide`, or a
  ratified compound.
- `T: Decide + Bounds` (or `+ Bounds` on any compound) in
  `geom-brep/src/ssi/*` requires extending the ratified
  compound-bound allowlist: the design paragraph in
  geom-core/src/real.rs:325-400 AND both allowlists
  (ci.yml:155-170, scripts/ci-local.sh:133-156), naming the
  exact files. This is authorized by this spec (precedented:
  PR 11/PR 12 extensions); document it as the spec-authorized
  extension it is. Prefer the narrowest file set that works.
- Interval squares in `src/`: `powi(2)`, never `x*x` (the
  tripwire greps for it; `a * a.dot(x)` false-positives get a
  named-binding restructure, git 7dd9425/e46e749 precedent).
- Fail loud everywhere: no escape hatches, no silent f64
  fallbacks inside generic code, refusals typed with honest
  recourse text.

## 3. Stale-claims sweep (in scope) and non-goals

In scope — the lift falsifies standing prose; update it honestly:

- The S13 re-gate text (topo/src/boolean/mod.rs:556-568 + the
  Display at :779-790) says projection "exists at f64 ONLY" —
  after the lift that clause is false. The GATE ITSELF STAYS
  (retiring it needs a written NURBS extent test — non-goal);
  reword to name what now actually blocks it.
- boolean/mod.rs:530-533, :708-711, :764-767 and
  boolean/ops.rs:1305-1324 blame "the SSI enclosure stack being
  f64-only" — after the lift the honest blocker is the unwired
  join lane. Reword; the refusals stay.
- pcurve_cache.rs:150-183 (the "storage item…lands with the PR
  that first needs it" block) is superseded — rewrite as the
  variant's real doc.
- DESIGN.md frontier entries (b)/(d)/(e) (:496-538): (b) closes,
  (d)/(e) re-point at their remaining blockers.

Non-goals (typed refusals stay, docs re-pointed, no code):
cyl×sphere fitted-chord join window (`run_azimuth_window`/
`chart_pcurve` wiring; M6-PLAN banks it past M6 — "chase the
lift"), cyl×cyl equal-radius germs (PR 9c dev 4), writing the
NURBS extent test / retiring `NurbsExtentUnsupported`,
sphere×sphere seams, cone/torus operands, canal blend, curved
REST, any recipe/corpus vocabulary work (that is unit 5), any
jet/march/system lift.

## 4. Acceptance summary (the review will attack exactly these)

1. Lifted enclosure/certify/projection stack compiles and its
   certified lane RUNS at `Interval`: the fitted-cache C2
   certificate derived at the interval scalar on at least one
   rung-3 branch, enclosure-style asserted (bracketing, not
   equality).
2. `Pcurve::Fitted` exists; `recertify` re-derives at rest; the
   at-rest body row (Leg C) green at default ε with the full
   certificate present; multi-ε posture per §1C.
3. The vacuity pin flipped with history; persistence rows still
   green (nothing pcurve-shaped serialized).
4. Discipline job green (allowlist extension documented in
   real.rs, both CI lists, narrowest file set).
5. Stale-claims sweep done; every remaining refusal's text names
   its TRUE current blocker.
6. No `T: Real + …`; no new unallowlisted compound bounds; no
   silent deviations — every deviation numbered in the report
   with its executed blocker.

## 5. Battery scope (iteration-speed, per
## memories/local-battery-scope.md)

Local: touched-crate suites (geom-brep, geom-surfaces, topo,
sweep, editor-core, step-export) at default ε, PLUS the Interval
lane rows for geom-brep/geom-surfaces/topo (the change is
scalar-generic — the Interval lane is where the likely failures
are), plus `scripts/ci-local.sh` discipline() once before the
final push. Multi-ε and the rest of the matrix ride the hosted
gate — hosted CI is the only gate.
