# M4 work order: the parametric model layer

Status: **PROPOSED** (ratification conversation; forks F1–F9 carry
firm recommendations, pushback welcome — the M3-PLAN/#42 pattern).
Charter: DESIGN.md's M4 roadmap entry — parameter vector → feature
DAG → solid; provenance-based naming; replay; STEP export — plus
every M4-tagged obligation in the ratified record, inventoried
exhaustively (with quotes and dependency edges) in
`references/notes/m4-obligations-inventory.md`. The two pre-M4
design docs are ratified and binding: NAMING-DESIGN.md (#74, N1–N7)
and SOLVER-DESIGN.md (#79, W1–W9 — M4 takes the *contracts*; solver
implementation is M6). Process conventions inherited from M3
unchanged (one implementer + one adversarial e2e reviewer +
one fix pass per PR; binding orchestrator specs; OUTPUT DISCIPLINE;
merge gate = scripts/gate.sh while hosted CI is down, ci.yml kept in
sync for its return).

## What M4 is, in one paragraph

M3 ended with a kernel that builds real parts through public ops; M4
makes models *documents*: a typed recipe DAG (D8) whose evaluation
is the body, whose node IDs are the substrate of stable names (N1),
whose edits are recorded values (GQ3), and whose save format is the
recipe itself. The naming layer (N1–N7) is implemented against it;
bit-identity comparison retires from production (N6); appearance
attaches by stable names; the document persists with bit-exact
floats and schema versioning; STEP export ships; and the Band 4
model corpus comes online to measure rebuild latency while the
architecture is still cheap to change. Editor-core is born as the
headless layer-2 crate (G1). Sketches remain programmatic profiles
— the solver ships at M6; M4 reserves its contracts (witness datum,
ReWitness, WitnessBifurcation) so recipes never need a breaking
change to receive it.

## Forks to resolve at ratification (recommendations firm)

**F1 — Dimension-algebra extent (GQ5's banked decision).**
*Recommendation: the restrictive lattice.* v1 typed quantities are
{Length, Angle, Count(dimensionless integer), Scalar(dimensionless)};
permitted: same-dimension add/sub/compare, scaling by dimensionless,
trig on Angle (producing Scalar), Count arithmetic. Dimension-
CHANGING products/quotients (Length×Length, Length/Angle…) are typed
refusals in v1. Rationale: covers actual recipe math (dimensions,
counts, angles); totality and fail-loud stay trivial; the full
rational-exponent dimension lattice is a purely additive future
extension, so nothing is foreclosed. Units erase before kernel
scalars (ratified); unit *display* is document metadata.

**F2 — GQ2 result-DAG API shape.** *Recommendation:*
`Evaluation<T> = { order: Vec<RecipeNodeId>, nodes: Map<RecipeNodeId,
NodeResult<T>> }` with `NodeResult = Ok(NodeValue) | Failed(typed
error) | Poisoned { through: RecipeNodeId }`; `NodeValue` carries the
bodies, the per-node name table (N4), the solved assignment slot
(W5; empty until M6), and content keys for cache transfer. Failure
poisons descendants only (ratified); the evaluator is scalar-generic
(`Real`) from day one; epochs + cooperative yield points are part of
the evaluator signature, not a retrofit. Exact field-level types are
PR-spec work under this shape.

**F3 — Persistence concretes.** *Recommendation:* snapshot +
edit-log, serde-based text format with a leading integer schema
version; floats serialized shortest-round-trip (Ryu strings);
NaN/inf REFUSED at persist time (typed — the kernel never produces
them legitimately, so persisting one is a bug surfaced, not data);
migrations = explicit version-to-version functions from schema v1
onward. Format aesthetics (JSON vs RON vs custom) are PR-spec work;
the contract above is what ratifies.

**F4 — v1 feature-node vocabulary.** *Recommendation:* Datum
(plane/axis/point), Profile (programmatic sketch, GQ5-typed
expressions), Extrude, Revolve, Split, Union/Intersect/Subtract,
Transform (rigid placement), LinearPattern/CircularPattern
(structural Count index — A8/N1 Instance(i)), and Declare (the
coincidence-intent carrier, F5 below). Edits, not nodes: Rebind,
ReWitness, SetTolerance, parameter/expression sets. The revolved-
hole sugar node (A6's "may") is DEFERRED — sugar can land any time;
v1 vocabulary stays minimal.

**F5 — Declaration threading (the A7 fix).** *Recommendation:
declarations are recipe data on the consuming boolean node* — a
`Declare` input listing coincidence intents as StableName pairs,
resolved through the operands' name tables at evaluation into the
contact records the boolean already consumes; results carry their
records as today, and a reused 3′ body's declarations re-enter
downstream ops by *name*, never by arena key. This closes the M3
envelope's operand-internal-declaration gap: the loud
`UndeclaredContact` refusal on 3′ reuse becomes a certified pass
whenever the recipe declares the surviving intent. (The M3-era
implicit path — records minted by the op from structural/declared
coincidence — remains; Declare is how intent *persists across* ops.)

**F6 — STEP export approach.** *Recommendation:* an evaluation PR,
decision recorded mid-milestone: spike `ruststep`/`truck-stepio`
(both satisfy the dependency-age rule) against our bodies for the
AP203/AP214 analytic-geometry subset; adopt the syntax layer if the
spike round-trips our planar B-reps cleanly, else write the (small)
subset writer in-house. Export scope v1 = the analytic B-rep we
have (planes; M5 grows it). Acceptance is external: a exported part
imports into FreeCAD/OCC intact (the admesh pattern, STEP-shaped).
Import remains M7 — nothing in the evaluation may grow import scope.

**F7 — Expression AST + ExprPath.** *Recommendation:* a small typed
AST — literals (dimensioned per F1), parameter refs, node-output
refs, arithmetic, trig, min/max — with **no conditionals in v1**:
a conditional is a value-dependent branch, and those stay inside
kernel code where predicates are reified (A5/D8); recipes needing
case analysis use structural parameters. `ExprPath` (GeomSource's
missing type) = (node ID × expression slot × AST path), stable
under edits to *other* expressions by construction.

**F8 — Milestone boundary.** *Recommendation:* the persisted file
IS in M4 (H1–H4 fire — the recipe being the save format is the
milestone's point; a parametric layer that can't save is a demo,
not a milestone), and the Band 4 corpus comes online with rebuild
latency MEASURED and REPORTED but not gated on a threshold number
(it is an architectural property we are instrumenting, not yet a
contract; PERF-PLAN stays advisory).

**F9 — Planning context to note (not really forks).** New CI
obligations (name-table golden, save/load/replay-identity,
bit-identity tripwire evolution) land as ci-local.sh + ci.yml pairs
while hosted CI is down. Q9 (project name) stays parked — the
editor-core crate follows the existing unprefixed crate-name
convention, so nothing forces it; Evan's shortlist is in
memories/name-candidates.md whenever he wants to close it.

## PR sequence

1. **Recipe substrate + editor-core birth.** Crate `editor-core`:
   `Doc` (recipe DAG + metadata) as a value; `RecipeNodeId` (stable,
   never reused, minted at insertion); structural vs continuous
   parameters as types; the expression sublanguage v1 (F1 lattice,
   F7 AST, evaluator generic over `T`, total by construction);
   `ExprPath`; the DocEdit vocabulary + `apply : Doc × DocEdit →
   Result<Doc, EditError>` (undo/redo falls out of values).
   Acceptance: a die-recipe document built by edits, structurally
   diffed, replayed bit-identically.
2. **Evaluation service.** F2's result DAG; feature-node vocabulary
   v1 (F4) wiring the existing kernel ops; memoized incremental
   recompute keyed on content keys (banked principle — the key is
   the proof), downstream-only invalidation, partial
   re-tessellation riding the same keys; cancelation epochs +
   yield points; rayon idiom-1 over independent nodes (advisory,
   cited to the D9 addendum). Acceptance: edit one parameter
   mid-DAG → only downstream nodes re-evaluate (counted), solid
   correct at all ε rows + Interval.
3. **Naming part 1: tables.** RolePath enums per op (N1 vocabulary
   made concrete); eager per-node name-table emission (N4); split
   discriminators as covariant margined predicates with tie marks
   (N2); merge retirement names (N3); the **CI name-table
   invariant** golden test (same verdicts ⇒ same names, f64 AND
   Interval). Acceptance: names stable across parameter motion
   without flips on the corpus-so-far; discriminator flips change
   exactly the names whose derivations pass through them.
4. **Naming part 2: resolution + the diff engine.** `ResolveError`/
   `Diagnosis` (N5) with the verdict-vector diff engine built ONCE
   and shared with `SetTolerance` (H4's ε machinery lands here);
   `Rebind` DocEdit (auto-menu EMPTY); tombstones; hit-testing
   inversion (arena key → stable name over the M2 PR 6 back-refs;
   the GUI never sees an arena key). Solver *contracts* ride here
   (D1: opaque per-node witness datum, `ReWitness` DocEdit + bulk
   certified-same-branch allowance, `WitnessBifurcation` +
   N5's Diagnosis arm) — types and document semantics only.
5. **GeomSource + retirement + Declare.** `GeomSource
   {node, expr, orient}` on every description (composition through
   transforms and `revert`); `merge_faces.rs`/`plane_eq.rs` migrate
   to source comparison; `bit_identity.rs` → debug-only with
   `debug_assert!(same_source ⇒ eq_bits)`; CI tripwires updated to
   an empty production-consumer allowlist. F5's Declare node +
   threading closes the operand-internal-declaration gap
   (acceptance: the M3 closure-corpus rows that 3′-refused now
   certify with recipe-declared intent — the envelope entry
   updates).
6. **Persistence.** F3's snapshot + edit log; schema v1 +
   migration mechanics; bit-exact float round-trip; the
   save/load/replay-identity CI row; ε recorded in-document +
   SetTolerance apply = replay + structural diff (the PR 4 engine).
   Acceptance: full corpus documents round-trip bit-identically;
   an ε change reports exactly its flipped predicates.
7. **Appearance + STEP export** (parallelizable pair). Appearance:
   per-face/body attributes in editor-core keyed by StableName,
   surviving recompute, N3/N5 semantics on retire/vanish, seams
   wrapper-ready (B11). STEP: F6's evaluation + the export it
   lands on, external-import acceptance.
8. **Band 4 corpus + M4 exit sweep.** The model corpus (recipe
   documents spanning the feature vocabulary) with rebuild-latency
   tracking wired into CI reporting; the K-telemetry Probe run the
   M3 addendum specified (the corpus is the missing harness);
   DESIGN.md exit sweep (ratify F1–F8 outcomes, retire the
   operand-declaration envelope entry, update the M4 roadmap line);
   state-doc trim per the standing convention; M4 exit walk
   against the criteria below.

## Deliberately not in M4

Sketch-solver implementation and constraint-driven sketches (M6 —
M4 profiles stay programmatic; only the W-contracts land); GUI
anything (Band 2; "UI ideas" stay non-binding); assemblies (wrapper
seams reserved, nothing built); automatic rebinding policies (menu
empty by decision); STEP import (M7); NURBS/fillets (M5); Python
bindings/docs program (post-M4); BVH unless the corpus forces it;
out-of-family detection (far-future).

## Exit criteria

A recipe document, authored through DocEdits and persisted to disk,
round-trips bit-exactly and replays bit-identically at ε ∈ {1e-6,
1e-9, 1e-12} + Interval; the die (and the boolean-tour bodies)
rebuilt as recipe documents; a mid-DAG parameter edit re-evaluates
only downstream nodes and every unaffected stable name resolves
identically (counted + name-table golden green); a flip-inducing
edit produces the typed ResolveError carrying the correct flipping
predicate; Rebind and SetTolerance work end-to-end with the shared
diff engine; bit_identity has zero production consumers (tripwire
allowlist empty, debug assertion in place); a 3′ body reused
through a Declare-carrying boolean certifies at the 3′ gate;
appearance attributes survive recompute and retire loudly with
their names; a STEP export of a corpus part imports intact into an
external system; the Band 4 corpus runs in CI with rebuild-latency
reporting; solver contract types (witness datum, ReWitness,
WitnessBifurcation) compile into the document layer with tests
pinning their document semantics; new conventions ratified into
DESIGN.md at exit.
