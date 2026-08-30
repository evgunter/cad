# M10-4 — sensitivities and the stackup (E4/E5)

STATUS: BINDING (dispatched 2026-08-30; citations re-verified
against merged M10-2/#1213 and M10-3/#1231). Unit branch
`m10/m10-4-stackup`. Program plan
`docs/M10-PLAN.md`; design record `docs/ERROR-DESIGN.md` E4/E5/E9
(read all three sections in full) and `docs/DUAL-DESIGN.md` DL1–DL6.

## Grounding (substrate facts; verify each at the site before coding)

- **The Dual door is OPEN and pinned** — `e4_dual_door.rs` states it
  as a compiler fact; `m10_di_dual_corpus.rs` is the runtime half (a
  full corpus build at `Dual64`, value channel bit-identical to f64).
  That suite's header says what it deliberately does not build: "No
  seeding surface, no sensitivity API, no stackup reporting — those
  are M10-4's." This unit builds exactly those three, nothing else.
- **No public door can seed a tangent today** (stated at
  `r1_dual_probes.rs`): `param_env` lifts every parameter with
  `T::from_f64`, tangent zero. M10-3's `AxisScalar for Dual<T>`
  binds a box on the VALUE channel with a zero tangent ("a box is an
  enclosure, not a seed" — its words). The seed door this unit opens
  must not disturb either.
- **The memo cannot alias passes** (DL2, shipped by M10-DI):
  `ContentBits for Dual<T>` feeds value THEN tangent channels, so a
  seeded env has different content bits per seeded parameter — one
  parameter's pass cannot be served from another's. Rely on it;
  do not rebuild it.
- **DL3's scalar-policy seam is shipped** (`topo::AtRestPolicy`,
  typed `AtRestOutcome::{Validated, NotRunAtThisScalar}`): certified
  validation is structurally absent at a dual. The OTHER half of
  DL3's sentence — "The E4 driver asserts (cheaply, by content key
  equality of the value channel where it needs a hook) that it is
  differentiating the build the f64 run validated" — is the
  **named unenforced obligation from M10-DI's adjudication**, and
  THIS unit discharges it (§3).
- **The Measure sink evaluates at every scalar** (M10-2, post fix
  pass): `Node::Measure` with `MeasureRef { at, name }` (a measure
  reads the placed geometry at its `at` node), typed quantity out,
  bit-identical value channel at `Dual64` (pinned by the adopted R1
  probe). ∂m/∂pᵢ is read off the measure's evaluated payload's
  PUBLIC tangent field — never off `Bounds` (plan text; E9).
- **The E6 driver is on main** (M10-3): `analysis::ParamBox`,
  `drive -> ParamBoxVerdict` with certified/refused leaves, ADDITIVE
  accounting (`total()`/`unresolved()` add the tail — post fix
  pass), `wire::LaneEnv`, flip naming through `resolve::vdiff`.
  Certificates in this unit are ITS leaves; build nothing parallel
  to it.
- **Profile parameters ride the M10-P lift**: the guided replay puts
  lane-scalar geometry through `LaneEnv`'s one env. A seed on a
  profile dimension must propagate the same way an interval box does
  through M10-3's door (the C6 seam is closed for structure; the
  Dual half is this unit's to exercise). §6's pin is required; if a
  genuine blocker surfaces, the plan's valve applies — dispatch
  holds at magnitude-parameter scope with the profile gap TYPED,
  never silent zeros.

## Scope

### 1. The seed door

- `EvalOptions::seed: Option<ParamName>` — the `param_box` seam's
  twin: scalar-free option, per-scalar capability. Exactly one
  parameter seeded per evaluation (E4: n parameters ⇒ n independent
  passes; a multi-seed vector mode is E11.4, out).
- Capability is compile-time, beside `AxisScalar`: `Dual<T>` carries
  a seed (tangent = exactly `1.0` on the seeded parameter's lift,
  zero elsewhere — including every unseeded parameter and every
  literal); every non-dual scalar refuses typed on every node
  (`NodeErrorKind::Seed`, the `ParamBox`/ε-conflict shape).
- Composition: `seed` and `param_box` TOGETHER are legal exactly at
  `Dual<Interval>` (the certified tier's evaluation: value channel
  carries the leaf's box, tangent channel the seed). `Dual64` +
  `param_box` keeps M10-3's degenerate-only rule; `Interval` + seed
  refuses as above.
- An unknown or non-continuous (Count) seed name refuses typed at
  env construction, before any node runs.

### 2. The n-pass sensitivity driver

- Home: `editor_core::analysis`, beside `drive`. Signature shape
  (names the implementer's): `sensitivities(doc, measure, tol) ->
  Result<Vec<Sensitivity>, _>` — one entry per continuous parameter
  of the document (the fixed-param typed spelling from M10-1's
  adjudication note applies: a parameter without a distribution
  still gets its ∂m/∂pᵢ; distributions matter to §4's report, not
  to the derivative).
- Each pass: evaluate the doc at `Dual64` with that parameter
  seeded, read the measure's tangent. Pure; parallel under rayon
  idiom 1 (D9: result independent of schedule — pin it).
- A pass whose MEASURE refuses (typed, from M10-2's own doors)
  yields a typed per-entry refusal, not a driver failure; a pass
  whose tangent is degraded (non-finite tangent, value finite)
  yields the E9 forfeiture state (§5) — NEVER a refusal.

### 3. The pairing hook — DL3's sentence made a mechanism

- Every `Dual64` pass asserts, against the f64 evaluation the driver
  builds once (or is handed), **value-channel content-key equality**
  at every node — the cheap hook DL3 names. The check is a REAL
  gate, not a `debug_assert`: a mismatch is a typed driver error
  (name it; `PairingViolation` or better), because a mismatch means
  the sensitivity is not of the validated build — the exact silent
  state DL3's availability argument leans on excluding.
- Pin it red-capable: a probe that hands the driver a STALE f64
  evaluation (edit the doc between builds) must get the typed error,
  never a sensitivity.
- This discharges the M10-DI adjudication's named obligation; say so
  in the PR body and cite the adjudication log line.

### 4. `Stackup` — E5 verbatim

The typed report, fields exactly E5's block (measurement /
nominal / per_param / worst_case / rss / coverage):

- **`worst_case` is the only gating number**: the hull of
  value-channel INTERVAL evaluations of the Measure node over E6's
  certified leaves — never the linearized sum. v1 obtains the
  per-leaf enclosures by re-evaluating the measure at `Interval`
  over each certified leaf's env (pure, parallel, memo-served). If
  measured cost makes that prohibitive, the alternative is a
  recording dial on `DriveConfig` — a DISCLOSED deviation with the
  measurement in the PR body, either way.
- **`per_param`**: sensitivity (E4-marked, §5) + contribution
  (|∂m/∂pᵢ|·Δpᵢ over the analyzed box's half-width) — advisory,
  labeled.
- **`rss`**: √Σ(∂m/∂pᵢ·σᵢ)², advisory; available only when EVERY
  contributor carries a measure — one Band parameter ⇒
  `UnavailableBecause` naming ALL Band contributors (M10-1's mass
  doors give σ; the single-param-naming narrowing M10-3 documented
  at `add_mass` does NOT apply here — name them all).
- **`coverage`**: certified + refused + tail from M10-3's
  accounting, verbatim — do not recompute; it sums to 1 there now.
- No persistence, no goldening form, no content-key caching of the
  report — M10-6's (it will want `serialize()` shaped like the
  driver's; leave the door visible, build nothing).

### 5. The mark and the E9 forfeiture — no third state

- Every sensitivity carries `ChamberCertified(<leaf identity>)` or
  `LocalOnly` — a two-variant enum a consumer cannot dodge; no
  unmarked number anywhere in the API. The certificate is the E6
  certified leaf CONTAINING the nominal, from a drive over the box
  asked about; nominal-in-refused-leaf (or no drive run) ⇒
  `LocalOnly`. The classic stackup lie (extrapolating across a
  topology change) must be UNWRITABLE, not discouraged.
- E9 addendum, live: a degraded tangent (straddle hull to the whole
  line, kink-jump enclosure, non-finite tangent at `Dual64`)
  forfeits exactly its uses — the `per_param` entry and `rss` go
  `UnavailableBecause`, `worst_case` untouched (it never reads a
  tangent). Refusal from tangent state must be UNREACHABLE — pin it
  (an `abs`-kink fixture whose value channel certifies cleanly and
  whose stackup still gates on `worst_case`).
- `Dual<Interval>` enclosures are consumed for CONTRIBUTION BOUNDS
  only in this unit (E7's monotonicity pruning is M10-5's); never
  for refusal (E9; DL1 unmoved — nothing here certifies through a
  dual).

### 6. e2e (the worked example's stackup half)

The two-hole plate (M10-2's e2e document, distributions from
M10-1): a full `Stackup` on the web measure — nominal re-derived,
`worst_case` from certified leaves and consistent with the
assertion's verdict, at least one parameter's sensitivity
analytically checked (∂web/∂r = −2 by the plate's own formula),
coverage summing to 1, and one Band variant showing
`rss: UnavailableBecause` naming it. Plus the REQUIRED profile pin:
a parametric-profile extrude (M10-P's shape) where a seed on the
profile dimension produces the analytically-correct nonzero tangent
through the guided lift.

## Out of scope

Reverse mode; vector-forward (E11.4). Persistence/goldening/CI rows
for stackups (M10-6). MC anything (M10-6). E7 pruning consumption
(M10-5). Any change to what certifies (DL1). The arm-floor redesign
(chart_region.rs's standing criticism). GUI surface.

## Review claims to falsify (the dual review's charter draws on
these; write the PR so each is attackable)

1. Zero impact unseeded: `seed: None` leaves every existing
   document's evaluation, memo/content keys and persistence
   bit-identical (merge-base differential).
2. Seed hygiene: tangent exactly 1.0/0.0 by construction; the memo
   never serves one parameter's pass to another (DL2 exercised, not
   trusted); passes schedule-independent (D9).
3. The pairing hook is real: red-capable on a planted stale build;
   no path yields a sensitivity of an unvalidated build.
4. No third state: the mark is structurally unavoidable;
   `LocalOnly` appears exactly when the nominal's leaf is not
   certified.
5. E9: no tangent state reaches a refusal; forfeiture is per-entry
   and loud; `worst_case` provably tangent-free.
6. `worst_case` honesty: the hull is of value-channel interval
   evaluations over certified leaves only; refused + tail mass says
   what it does not cover; it is NOT the linearized sum (construct a
   curvature case where they differ and the hull is right).
7. `rss` totality: one Band contributor kills it entirely, naming
   every Band parameter; no partial RSS exists.
8. The profile pin (§6) holds — a profile-dimension seed propagates
   a correct nonzero tangent; or the typed valve is honestly in
   place and disclosed.

## Acceptance

Hosted CI green on the drawn point (trailer-pin
`lane=interval`/`eps=1e-12` if the draw misses interval — the
certified tier is interval work); suites scoped and named per the
repo's basename conventions; every deviation from this spec
reported in the PR body's deviations section; k_lint untouched
unless a new metered predicate is minted (none is expected — if one
appears, it takes a funnel row per the standing rule).
