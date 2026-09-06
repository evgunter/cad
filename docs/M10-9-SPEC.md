# M10-9 — the registered-identity door: discharge by provenance (E12's reserve, taken)

STATUS: BINDING (dispatched 2026-09-06; opened from M10-8's measured
miss — the plate's real study unmoved at `7.81e2 · ε` under every rule
the atom algebra can afford — on Ev's ruling in chat:
"proceeding with 1"). Unit branch `m10/m10-9-registered-identity`.
Program plan `work/m10/plan.md`; design record `docs/ERROR-DESIGN.md`
E12 (read in full; its "kept in reserve — discharge by provenance"
clause is what this unit takes) with E6 as substrate; the item
`work/m10/M10-9.md`; the two findings that opened it,
`work/m10/plate-rim-residual-needs-the-wide-coefficient-ring.md` and
`work/m10/declared-tangency-needs-the-registered-identity-door.md`
(read both in full — their numbers are this unit's baselines).

## Grounding (substrate facts; verify each at the site)

- **The tier as merged** (M10-8, #1828): `geom_core::sym` — the
  per-leaf session, the lazy quotient normal form over an
  arbitrary-precision coefficient ring bounded at `COEFF_BITS = 256`
  (`crates/geom-core/src/sym.rs`, the `Rat` block), the shipped set
  `SymRules::shipped()` (the constant fold A0 in a second walk
  ALONGSIDE the plain form), rules A/B/C and `early_ab` built and
  dial-off, `SymCounts { symbolic_zero, sign_gated, numeric, frozen }`,
  `SampleOutcome::ALL` as the K vocabulary's one home (seven tokens;
  `tools/k-lint/tests/outcome_vocabulary.rs` pins the set across the
  workspace boundary), the driver replaying at `Sym<Interval>` through
  `eval::replay_leaf`. Read `sym.rs`'s header in full: it is the
  diagnosis this unit acts on.
- **The miss, measured** (M10-8's fix pass, `sym.rs` header "What
  still bounds the plate"): the two-hole plate's real study (±0.05 mm
  spacing, σ = 0.01 mm radii; `demos/tour` stop 1) certifies whole
  only below `7.81e2 · ε`, bounded by `carrier_endpoint_start` — the
  rim residual `carrier.eval(t0) − q`, zero iff `‖q − c‖ = r`. Its
  plain form is an outer `sqrt` over a degree-12 polynomial in the
  radius with `sqrt((a + 2r)²)²`-shaped atoms nested inside; rule C
  reaches it only per node and only at ~640 coefficient bits, and at
  4096 bits a bracket leaf costs 229 s against 5.9 s. The ring width
  is a COST wall, not a soundness one.
- **Why `‖q − c‖ = r` is a theorem of the construction, not a
  coincidence.** A profile arc's center and radius are the sagitta
  closed forms (`crates/profile/src/seg.rs:140-152`: `apothem =
  len·(1 − b²)/(4b)`, `signed_radius = len·(1 + b²)/(4b)`, `center =
  mid + n·apothem`, `radius = signed_radius.abs()`), so
  `‖q − c‖² = (len/2)² + apothem² = signed_radius²` is a rational
  identity for EVERY parameter value where the arc is defined, and
  both `‖q − c‖` (a `sqrt`) and `r` (an `abs`) are non-negative by
  construction — the two sides are the same non-negative root. The
  tier already proves the squared identity where the ring affords the
  expansion; what it cannot afford is the expansion. The sweep then
  builds the carrier as `Circle { u_ref: (q_from − c_world).normalize(),
  radius, … }` (`crates/sweep/src/swept.rs:355-373`), where
  `normalize()` mints the `sqrt(v·v)` atom the residual carries.
- **The second registrant.** A `Fillet(r)` step's arc carrier is
  built tangent to the arrival carrier BY CONSTRUCTION
  (`crates/profile/src/path.rs:2356`, `fillet_arc_carrier`: `center =
  t2 + n̂·(σ·r)`), and the joint classifier decides
  `carrier_line_circle` on `radius − |h|`, `h = unit.perp_dot(center −
  a)` (`crates/profile/src/seg.rs:326-343`). R2's rounded-corner pad
  (`crates/editor-core/tests/m10_8_r2_probes_interval.rs`, `pad`) is
  bounded by exactly that predicate at `2.083e-6` of its study.
- **E12's reserve, verbatim** (`docs/ERROR-DESIGN.md`, "Kept in reserve
  — discharge by provenance"): a typed "built as …" token, verified at
  the f64 witness point, discharged structurally over the box; exact
  and simple for same-OBJECT identities; taken only if the census
  shows a family the symbolic tier misses. The family is shown
  (M10-7 D4, M10-8 by measurement); this unit takes it.
- **Rulings that bind here**: a symbolic `Zero` is a theorem; a
  registered identity is an AXIOM about the construction stated by
  the constructor that guarantees it, verified at the witness, and
  COUNTED apart from both theorems (§3); no funnel site is edited; the
  numeric channel's bits are untouched by a registration; the frontier
  (iterated quantities) stays S-CERT's.

## Scope

### 1. The door (`geom_core::sym`)

A session-level registration `register_equal(a, b)` on two DAG nodes
of the same scalar, reachable from generic constructor code through a
`Real`-level hook that is a NO-OP on every non-`Sym` scalar (the
constructors are generic over `T: Real`; `f64`, `Probe`, `Interval`
and the dual scalar never see the door). On `Sym<T>` it records, in
the current session, that the two nodes denote one function of the
parameters, and the normal form consults the record so that
`NF(a) − NF(b)` is the zero form. Properties, each pinned:

- **The value channel is untouched.** A registration changes no
  numeric value, at any scalar: the `Sym<T>` value of `a` and of `b`
  stay what their ops produced (`u_ref` is still `v / sqrt(v·v)` at
  every lane). Pin: the serialized verdict vectors and every f64
  witness value byte-identical with the door on and off on every M10
  fixture (M10-8's differential harness, extended). This is why the
  cheaper spelling — the constructor normalizing by the declared
  radius, `(q − c) / r` — is REJECTED: it changes the f64 lane's bits
  and the numeric enclosure's dependency structure, and the review
  should confirm the rejection stands.
- **A lying registration is caught, typed.** At registration the
  session compares the two values in the lane scalar: enclosures that
  do not meet, or f64 values that differ beyond a tight point
  tolerance stated at the impl, REFUSE the registration typed
  (`SymRegistration::Contradicted`, or the impl's spelling) — never
  silently, never as a fold. The f64 witness pass then still evaluates
  every residual at the point (E12's soundness paragraph), so a
  constructor that fails to build what it claims is caught where
  widening cannot hide it. Pin with a PLANTED lie (register
  `‖q − c‖ ≡ 2r`): the refusal is typed and the decision stays numeric.
- **Registration order and memoization** are stated and pinned:
  either a registration invalidates forms memoized before it in the
  session, or the registrant registers before any consumer builds —
  the impl chooses, says which, and pins the other order's behaviour
  as a refusal rather than a silent miss.
- **D9.** Registrations are keyed by content-hash node ids and applied
  per leaf replay; the outcome is bit-identical across repeats and
  the rayon schedule (pin on the plate's leaf).
- **Same-object only.** The door aliases NODES; it does not accept a
  form-level equation. An identity between two independently built
  expressions (the fillet's `|h|` built at the joint against the
  radius the constructor holds) discharges only if the registrant
  builds the SAME node the consumer builds — content hashing makes
  that testable (pin: the registered id equals the consumer's id on
  the pad). Where it does not, say so in numbers; do not widen into a
  form-level axiom store.

### 2. The registrants (two), and their consumers

- **The profile arc's `‖q − c‖ = r`.** The registrant is the site that
  GUARANTEES the identity — the sagitta construction (`seg.rs`) or the
  sweep's carrier builder inheriting it (`swept.rs:355-373`); the impl
  chooses, and the registrant's doc comment carries the two-line
  proof above (squares equal as a rational identity; both sides
  non-negative by construction). Consumer: `carrier_endpoint_start/end`
  on every swept arc, and whatever else `u_ref·u_ref = 1` reaches
  (`carrier_on_surface_*`, the cylinder residual — measure it). The
  plate, R2's bracket and R1's annulus re-measured with this
  registrant alone.
- **The fillet's tangency `|h| = r`.** Registrant `fillet_arc_carrier`
  (`path.rs:2356`) or the step that calls it; consumer
  `line_circle_joint`'s `carrier_line_circle` (`seg.rs:335`). The pad
  re-measured. If the node-identity condition of §1 fails here, the
  unit ships the arc registrant, files the fillet's exact obstacle
  with the two rendered forms, and does not build a form-level store.
- **Not shipped**: any registrant outside these two; a registration
  the f64 witness cannot check; edits at any `decide` site.

### 3. Honesty instruments

- `SymCounts` gains `registered` (decisions answered `Zero` through a
  registration) beside `symbolic_zero` and `sign_gated`; never mixed.
  `SampleOutcome` gains the matching outcome through the ONE home
  (`SampleOutcome::ALL`/`token()`, k-lint's cross-workspace test, the
  CLI-contract row); the driver K row must LINT it non-zero
  (`two_hole_plate_narrow` carries arc geometry — read the per-file
  and TOTAL lines from the hosted log, never a step conclusion).
- `render()` and the receipt on `ParamBoxVerdict` show the count; the
  census (`work/cert/symbolic-tier-census.md`) gains the bucket
  "registered" with the rows the two registrants discharge.
- **The ceilings, re-measured at three ε rows** (default, 1e-6, 1e-12)
  on the plate, R2's bracket, R1's annulus and the pad: the widest
  whole-certifying box, the first refusal beyond it named with its
  predicate and enclosure. M10-8's pins that state the plate at
  `7.81e2 · ε` and the pad at `2.083e-6` FLIP by design — re-cut them
  as positive pins asserting the mechanism (the registration that
  discharged, the predicate that bounds now). A ceiling that stops
  scaling with ε is the E12 claim itself: state each ceiling's
  ε-dependence explicitly.
- **The tour's stop 1 becomes the certified study** if the plate
  certifies; if not, the caption says what bounds it, in numbers.
- `work/m10/real-margin-dependency-widening.md` is the expected next
  ceiling once the identities are gone: when a document is bounded by
  a non-identity margin, say so with the predicate and enclosure, and
  do not widen this unit into it.

### 4. The ring width, measured not assumed (bounded)

One table, on the plate and R2's bracket: `COEFF_BITS` at 256 (the
baseline), 1024 and 4096 with the shipped set, cost per leaf and the
whole-certifying box at the default ε. It records whether the
alternative to the door would ever have been affordable. A bound
change ships ONLY if it moves a ceiling at ≤ 2× the leaf cost on the
bracket; the expected outcome is that it does not, and the table then
closes the wide-ring item as measured. No other ring work.

## Out of scope

The real-margin dependency-widening class; implicit quantities
(S-CERT); a form-level axiom store; any change at a funnel site; the
`dot`-as-square question (`interval-self-dot-straddles-before-rule-a`
— measure only if a ceiling lands on it, then file); the `Fillet`
discoverability finding (PATHS); the GUI.

## Review claims to falsify

1. **Soundness of the door**: a registration never changes a numeric
   value (bit-identical value channel and verdict vectors, door on and
   off, every M10 fixture, both ε rows); a planted lie refuses typed
   and the decision stays numeric; the f64 witness still evaluates
   the residual at the point.
2. **The count is honest**: `registered` decisions never appear in
   `symbolic_zero` or `sign_gated`; the K token lints in the driver
   row (log lines quoted, non-zero); k-lint's vocabulary test pins the
   eighth token.
3. **The plate certifies its real study** at all three ε rows with
   refusals only among flips, slivers and the recorded tail, and its
   ceiling no longer scales with ε — or, if it does not, the bounding
   predicate and enclosure are named at each row and the reason is a
   predicate, not prose.
4. **The pad**: `carrier_line_circle` decides through the door (the
   node identity pinned) and the ceiling moves from `2.083e-6`; or the
   exact obstacle is filed with both rendered forms.
5. **Zero impact with the door off**: a `SymbolicDials` switch
   reproduces M10-8's tier bit-identically (serialized verdicts and
   receipts) on every M10 fixture.
6. **D9**: registration outcomes identical across repeats and the
   rayon schedule; registration order pinned as §1 states.
7. **Cost**: leaf cost with the door on, per document, against
   M10-8's numbers (plate 0.54 s, bracket 2.75 s).
8. **The ring table is real** (§4) and no bound change shipped without
   its measured factor.
9. **Each registrant carries its theorem** (doc comment + a probe
   showing the SQUARED identity is a plain-form theorem on a small
   instance), so the axiom is consistent with the tier where the tier
   can reach it.
10. Every deviation in the PR body with the argument; D-numbering from
    D1.

## Acceptance

Hosted CI green on the full matrix on the final head with the driver
K row LINTED (per-file counts and TOTAL quoted from the log, the
`registered` token non-zero); the re-cut pins green at three ε rows;
the four ceilings measured with their ε-dependence stated; the ring
table; every deviation in the body. After merge the orchestrator
re-cuts `docs/M10-EXIT-WALK.md` (#1700) against the measured state:
"a macroscopic box certifies" is the program's exit condition, and
this unit is the one that either meets it or names the next ceiling.
