# M10-6 — reporting, CI rows, the advisory lanes, the demo (E10/E11)

STATUS: DRAFT (binding at dispatch, after M10-5 merges — its engine
API is this unit's last input; re-verify every cited name then).
Unit branch `m10/m10-6-reporting`. Program plan `work/m10/plan.md`;
design record `docs/ERROR-DESIGN.md` E10/E11 and the worked example
(read all three in full), with E2/E5/E6/E7 as consumed substrate.
This is the program's LAST unit: the exit walk that follows it is
the orchestrator's, not this lane's.

## Grounding (substrate facts; verify each at the site)

- **Persisted, in-document, already**: distributions (M10-1),
  `Node::Measure` with `MeasureRef { at, name }` and `Node::Assertion`
  report-only with the THREE-state verdict `Holds / Violated /
  Unevaluated { reason }` (M10-2; schema v17). E10's persisted set is
  complete except one item: the `min_clearance` primitive, which
  M10-2 deliberately shipped without ("no variant, no placeholder")
  and M10-5 deferred here by name.
- **Derived, never persisted, already**: `ParamBoxVerdict` with
  `serialize()` (exact-bits goldening form) and `content_key()`
  (M10-3); `ClearanceReport::serialize()` (M10-5); `Stackup` (M10-4)
  has NEITHER — the reviewers' consumer friction was that its only
  rendering is `Debug` and the verdict's masses render as hex bits.
- **Accounting is additive and honest** (M10-3 post fix): per-reason
  refused mass + tail sums to 1. But "priced" and "set-theoretically
  forced" mass are ONE type — `box_mass(Band, covering) = 1` is
  correct and measure-free, so an unresolved-mass budget over
  Band-only parameters would read "fully priced" while no shape was
  ever stated (M10-1 adjudication, R2 MINOR-1 — this unit's named
  obligation).
- **`NothingCertified` now carries the nominal, the `LocalOnly`
  sensitivities, coverage and receipt** (M10-4 fix pass) — a real
  ±0.1 mm study's answer is legible from the refusal. The honest
  limit stands: certification widths are ε-scale, so the certified
  worst case exists only over boxes a few ε wide today (class home:
  issue 1191).
- **k_stats**: M10-3 shipped `KProbe::CertifiedMidpoints` behind
  `probe,interval`, off by default, with the sweep wiring
  (`scripts/k_probe_sweep.sh`, `driver/<fixture>` CSV beside the
  linted one). No hosted row runs it. Solver predicates are vacuous
  (ruling Q1).
- **The tour** (`demos/tour/src/*.rs`, one file per scene, `stops`
  per scene) is evidence written from a user's seat
  (`memories/demo-purpose.md`): awkwardness is a library finding,
  never hidden.
- **Rulings that bind here**: Q3 (MC rides this unit, never gates,
  never persists as assertions); Q4 (schema steps are clean breaks);
  E10's open sub-question (should a failing assertion gate
  `build()`?) is answered "no" for v1 and is NOT re-opened here.

## Scope

### 1. The `min_clearance` Measure primitive (E3's last primitive; schema v18)

- `MeasurePrimitive::MinClearance` over two selections in M10-5's
  `Selection` vocabulary (body-at-node, face scope), persisted with
  its own wire form; schema **v18** clean break with a populated
  golden; load-door re-check like every other measure fault.
- **Evaluation semantics, stated because it is the unit's one
  design choice**: `min_clearance` is a CERTIFIED quantity (E7) with
  no closed form at a point scalar. At `f64`/`Probe`/`Dual` the
  measure refuses TYPED (`MeasureUnsupported`-class, naming the
  scalar and the door that can answer it), so an `Assertion` over it
  reports `Unevaluated { reason }` in an ordinary build — E10's
  third state, used for exactly what it exists for. At `Interval`
  over a leaf's env, the measure's value is the engine's certified
  bracket (`clearance_over`/`clearance_with` at that leaf), so the
  E6 driver and §2's assertion row certify it leaf by leaf and the
  assertion reads `Holds`/`Violated` there. No sampling, no
  degraded f64 number, ever. (If the implementer finds a sound
  f64 value channel — a certified bracket collapsing exactly — it
  is a DISCLOSED deviation with the argument, never a silent one.)

### 2. Serialization, content keys, the cache seam (E10 "derived, never persisted")

- `Stackup::serialize()` (exact-bits goldening form, the driver's
  idiom) and `Stackup::content_key()` over the bit-content of
  (recipe slice, box, ε, K) — D9 makes the key the proof. Same two
  doors on the MC report (§4) and the histogram (§5).
- A **human-readable rendering** for every report (`Display` or a
  `render()` door): masses as percentages, values as numbers, the
  tail on every line, `LocalOnly` and every `UnavailableBecause`
  spelled out — distinct from the goldening form, never a
  substitute for it.
- **The cache seam**: one in-process, never-persisted content-key
  cache for derived reports (verdict, stackup, clearance, MC), with
  a pure key function a consumer can call without the cache. Keep
  it small; the cache is a door, not a subsystem.
- **Priced vs forced** (the M10-1 obligation): the accounting's
  budget report distinguishes, as a TYPE, mass that is priced under
  a stated measure from mass that is set-theoretically forced (Band
  contributors — shape only). A Band-only document's "unresolved
  mass" reads as forced, never as "fully priced".

### 3. The three E10 CI rows

1. **Assertion gating on corpus assertions**: for every corpus
   document carrying assertions, drive its recorded analyzed box;
   every assertion must `Holds` over the certified leaves, with
   refused + tail mass within that document's RECORDED
   unresolved-mass budget (a goldened per-document constant beside
   the corpus entry, priced vs forced stated). `Violated`,
   `Refused`, or a budget overrun fails the row loudly, naming the
   document, the assertion and the mass. The two-hole plate (M10-2's
   e2e, `measured_web`) is the first entry; register at least one
   `min_clearance` assertion (§1) and one Band-carrying document.
2. **Goldened accounting on a margin-thin fixture**: the serialized
   refusal/tail-mass accounting of M10-3's planted-flip and
   terminal-sliver fixtures goldened bit-exact — the honesty metric
   is itself regression-tested. Re-bless procedure documented at the
   golden, per the fence precedents.
3. **k_stats rows for driver predicates**: a hosted row on the
   k-lint sampled axis (`klint_row`) that runs the driver's
   `CertifiedMidpoints` sweep and lints its distribution per
   `docs/K-REPORT.md`'s existing instrument — the K re-examination
   evidence E6 promised, reporting per the runbook (a fired lint is
   distribution evidence, never a reason to change geometry). State
   at the row that solver predicates are vacuous in v1 (Q1).

The rows ride the existing sampled matrix honestly: say which axis
each rides and what a green means (the sampling contract).

### 4. The MC advisory estimator lane (E11.1, ruling Q3)

- `analysis::mc` (or beside `stackup`): pure f64 replay over N
  parameter samples drawn from the document's distributions —
  PRODUCT measure (E11.2), the FULL distribution including the tail
  the certified box excludes (that is what MC adds); a Band
  parameter has no density and refuses the lane typed, naming it.
- Reports, per measure: sample mean/σ/min/max, and per assertion
  the empirical violation fraction — every number LABELED advisory
  with the sample count and the recorded seed; D9-deterministic
  (a named PRNG already in the dependency set or in-tree; the seed
  recorded in the report and its content key); rayon over samples.
- Never gates, never persists as an Assertion, never enters the
  accounting. The report's rendering places the certified numbers
  first and the MC estimate after, labeled — the E5 "labeling and
  ordering, not omission" rule.

### 5. The E11.6 histogram note

Per certified leaf, its mass and its measure enclosure — a typed
table (leaf, mass, `[lo, hi]`) with the two doors of §2. An ADVISORY
visualization datum, zero new soundness claims, no rendering beyond
`render()`; the GUI is out of scope.

### 6. The demo — the two-hole plate as a tour cell

`demos/tour/src/tolerance.rs` (name the implementer's), through the
public doors the way a user would author it: the plate with its
distributions, the web measure, the `web ≥ 0.5 mm` assertion; drive;
stackup; MC. Two stops, both honest:
- the REAL study (±0.1 mm on the width, σ = 0.02 mm on the holes):
  what a user gets today — `NothingCertified` with its `LocalOnly`
  sensitivities, the coverage, and the MC estimate labeled advisory;
  the ε-scale ceiling stated in the caption (issue 1191);
- an ε-scale box where the certified worst case exists: certified
  worst-case vs RSS optimism printed side by side, the tail riding
  every line — the MVP's reason to exist, rendered.
The cell is evidence: every awkwardness (assembling
`analyzed_box → drive → eval → stackup` by hand, positional node
ids, the hex goldening form) is stated in the caption as a library
finding, never smoothed over.

## Out of scope

A gating mode for `build()` (E10's open sub-question stays "no" —
an `[ev]` ruling if anyone wants it re-opened); joint distributions;
distributions on Count parameters; reverse mode / vector-forward;
GD&T; true output densities; GUI rendering of any report; the
Python authoring vocabulary (B-MEASURES stays chartered) — a Python
READ door for the goldening/rendered forms is welcome if cheap, not
owed.

## Review claims to falsify

1. Zero impact: documents without a `min_clearance` measure
   evaluate, key and persist bit-identically (merge-base
   differential; v17 refuses typed with the recourse; the schema
   step is exactly one).
2. `min_clearance` honesty: refuses typed at every point scalar;
   its assertion reads `Unevaluated` in an ordinary build; at
   `Interval` over a certified leaf its value IS the engine's
   bracket (re-derive on a fixture the PR did not use); no path
   yields a sampled or degraded number.
3. Goldening forms are exact-bits and D9-deterministic across
   repeats and rayon schedules; content keys move when and only
   when (recipe slice, box, ε, K) bits move; the cache serves only
   equal keys.
4. Priced vs forced is a type, not prose: a Band-only document's
   budget cannot read "fully priced".
5. Row 1 fails loudly on a planted `Violated`, a planted `Refused`,
   and a budget overrun — mutate the fixture and watch each red.
6. Row 2's goldens are re-blessable by the documented procedure
   and go red on a one-bit accounting change.
7. Row 3 actually EXECUTES on its sampled axis in hosted CI (read
   the run's step conclusions, not the job name — the
   green-name-over-skipped-step class) and lints the driver
   population.
8. The MC lane: never gates, never persists, refuses a Band typed,
   samples the full distribution (its tail fraction converges to
   the accounting's tail), seed-deterministic; every advisory
   number carries count + seed in the rendering.
9. The demo runs through public doors only and its captions state
   the findings the reviewers' own consumer walks reported.

## Acceptance

Hosted CI green on the drawn point, plus the three new rows
demonstrably executed at least once on this PR (trailer-pin the axes
that need pinning, and say so); the schema step's goldens populated;
every deviation in the PR body; the tour cell rendered by the
existing tour lane. After merge the orchestrator writes
`docs/M10-EXIT-WALK.md` — this unit's PR must leave the exit shape
(plan §"Exit shape") checkable line by line.
