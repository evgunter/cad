# SYM-5 — a stored unit vector does not double the degree (spec)

**Program:** SYM (`work/sym/plan.md`, the cost lane's reach half).
**Item:** `work/sym/derived-frame-placement-freezes-on-the-symbolic-lane.md`,
with `real-margin-dependency-widening` and `interval-self-dot-straddles-before-rule-a`
as context. **Track:** kernel change — the standard v6 unit (binding
spec, drawn implementer arm, cross-model dual review, union fix pass,
record-at-merge; §Review). Block SYM-B1 slot 1. **Pre-draw fields,
logged before the draw:** difficulty **H**, task-class **NUMERIC**.

- **H** — a new mechanism in the atom algebra (or a change to how a
  normalisation reaches the DAG), argued as an equality of reals under
  clause 1 like rules A–D, on a document class no shipped fixture
  exercises; the tier's history is three wrong premises corrected by
  execution.
- **NUMERIC** — what the tier decides moves (more identities
  discharge), and the reviews falsify soundness first.

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
item (DOCM's diagnosis: `SymCounts` per evaluation, the freezes at
`Powi 2`/`Mul`/`Add` on kids at rational degree 65–128, the `Sqrt`
keyed by its argument's form, each normalisation adding a denominator
and each square doubling the degree; the local experiment that emitting
already-unit `u`, `n × u` without re-normalisation still refuses);
SYM-1's freeze table in `symbolic-tier-costs-95-percent-of-the-m10-3-drive`
(the plate's 1,032 `Degree` freezes are on `Powi`/`Mul`/`Sub` with kids
at degree 40–117 and 1–7 terms — the same shape, on the document the
tier was built for); `crates/geom-core/src/sym.rs`'s header (`# The
arc family`, `# The form-level algebra (M10-10)` — how a rule is
argued and dial-gated), `sym/trig.rs` (rule D as the template of a
form-level rule: what folds, what never does, the bounds),
`sym/algebra.rs` (rules A/B per node), `sym/form.rs`, `sym/profile.rs`
(the instrument, with the `Origin` split); `crates/geom-core/src/linalg/vec.rs`
`normalize` (`self / self.norm()`, so a unit vector reaches the DAG as
three `Div` nodes over one `Sqrt(dot)` atom); the DOCM R1 rows on
`origin/docm/1-review-r1` (`crates/editor-core/tests/docm1_review_r1_probes_interval.rs`:
`r1_c7_an_extrude_on_a_widened_derived_frame_versus_the_authored_guided_twin`
and `r1_c7_the_prs_transform_lifted_shape_with_an_extrude_above_it`,
red until this unit answers). DOCM has since left the tracker
(`docs/DOC-LEDGER.md` sweep 14); the item's "Home" and DOCM references
are re-pointed by this unit.

## The claim

**A profile placed on a derived frame does not certify on the
symbolic lane where its authored-frame twin does, and the reason is
degree.** A derived frame's axes are the kernel's ALREADY-normalised
stored vectors — the cap normal `w.normalize()` and the profile
plane's `u_ref`, each a rational form over a `sqrt(v·v)` atom — and
the boss extrude normalises them again and squares them in
certification. Every normalisation adds a denominator; every square
doubles the degree; the forms reach 65–128 and freeze at the budget,
and a frozen subtree cancels nothing. Raising the budget to 4,096
leaves seven freezes at degree 433 and 670. The identity is real
(`u·u = 1` for a unit `u`; the endpoint is on the carrier) and the
plain form cannot see it through the degree.

**The unit makes a normalised vector cheap to carry**, so that the
derived-frame twin certifies where the authored one does, at any box
width the authored one certifies at, without moving a decision on any
measured document except upward.

**Ratified and not re-litigated:** E12; every rule is an equality of
reals under clause 1; the freeze discipline; D9 content-hash ids; the
budgets' values (`DEFAULT_SYM_MAX_DEGREE` = 128 is not the lever —
measured).

## Amendment A1 (2026-09-14, after PR-1's review) — the fixture

Phase 1 ran on DOCM's own document (PR #2568, PR-1) and found it the
NARROW case: its derived placement is a pure translation, every
normalised quantity is constant in the parameter, and M10-8's constant
fold collapses the chain — no rule of the atom algebra moves a decision
there, and the one refusal is the numeric channel's (filed on PROPS).
The review built the case the item was filed on — **a derived frame
whose axes carry the parameter** (an authored
`Datum::Frame { u: (1,0,0), v: (0,1,t) }`, `t = 0.25 ± half`, a cube
extruded from it, a `FaceFrame` on its cap, the boss on that; the twin
is the boss on the tilted frame directly) — and there the mechanism
stands on every rung: the derived boss refuses under `none`/A0/A on
`carrier_endpoint_start` (`[0, 1.8e-2]` at half `1e-3`) and under the
shipped set on `newell_plane_residual` with a plain straddle, frozen
632 on `Degree` with kids at degree 69–128; the refused residual's
early form is non-zero with a NON-constant `sqrt(S)` and two frozen
`Mul` nodes on its path; raising the budget to 4096/65536 leaves 483
frozen and the same refusal. Under `Guided` the authored twin
certifies on every rung while the plain lane refuses it.

**So Phases 2 and 3 run as PR-2 on the tilted document** (its rows are
adopted in PR-1 as `m10_derived_frame_tilted_interval.rs` or the file
PR-1 names): Phase 1's table and chain are re-taken there as PR-2's
starting point (the rendered chain naming the non-constant `sqrt(S)`
and which square first crosses the budget), and the acceptance below
reads "the tilted parity row" wherever it says "the two DOCM rows" —
the derived boss certifies where the authored twin does, under BOTH
lifts, at the widths the twin certifies at. DOCM's height document's
rows stay as PR-1 left them (pins at their measured state). Everything
else in this spec stands, including the stop condition.

## Phase 1 — measure before touching anything

1. **Port the two DOCM rows** as `#[ignore]`d evidence rows under
   `crates/editor-core/tests/m10_*` (subject-named,
   `m10_derived_frame_interval.rs`), with the fixtures they need
   (`boss_on_widened_box`, `boss_on_widened_authored_frame`, the
   twins under both `ProfileLift`s), unchanged in what they assert.
   Run them; they are red; say exactly how (which predicates, which
   enclosures, at which widths).
2. **Profile the freeze** with SYM-1's instrument on the derived-frame
   document at the nominal and at one widened box: freezes by origin,
   walk, cause, op, and the kids' degree and term count; then render
   the frozen kids (`sym::report::render_of`, `explain_depth`) far
   enough to name the normalisation chain — how many `Sqrt` atoms are
   nested, over what, and which square first crosses the budget.
   Compare with the plate's freeze table (SYM-1): same shape or not.
3. **State the identity that is lost** as a form: for the first
   refused predicate, the residual's early form with its atoms, and
   the real identity it would need (`u·u = 1`? `n·u = 0`?
   `‖q − c‖ = r` through a unit `u_ref`?).

One table and one rendered chain in the PR body BEFORE Phase 2.

## Phase 2 — the remedy, chosen by the measurement

Three candidate mechanisms, in the order the item names them; the
measurement picks, and the PR body says which and why the others are
not it:

- **(a) A unit-vector atom, rule E.** Recognise the shape
  `a_i · Inv(Sqrt(S))` where `S`'s form is `Σ a_j²` over the same
  nodes, and mint the component as an atom `U_i(S)` — three atoms
  keyed by one argument form, degree ONE downstream — with the rules
  `Σ U_i² = 1` (the analogue of rule B, `sin² + cos² = 1`) and
  `U_i · Sqrt(S) = a_i` (the analogue of rule A, unfolding on demand),
  applied per node in the early walk like A/B and bounded like them.
  Sound because both are equalities of reals wherever `S > 0`, which
  clause 1 guarantees before the identity test is asked (the vector
  has a real unit direction over the whole box). The cost is a
  cancellation that needed the inside of the normalisation and does
  not fit the two rules — name one if the measurement shows one.
- **(b) Normalisation simplified before squaring.** Keep the plain
  quotient form but let the early walk cancel the common factor
  `S / Sqrt(S)²` under rule A at the `Powi 2` node BEFORE the product
  is built (a targeted use of the ring `algebra` already closes),
  so the square of a unit component is the form `a_i² / S` and its
  degree does not double. Cheaper to build, narrower in reach: it
  helps `u·u` and may not help `n × u`.
- **(c) A degree-resetting `Sqrt` of a value-exact norm** — the
  item's first spelling: the `Sqrt` atom over a `dot(v, v)` whose
  form is a sum of squares is re-keyed so that forms built over it
  start again at degree one. This is (a) without the two rules;
  measure whether the rules are needed or whether the re-keying
  alone discharges the two rows.

Whichever is taken: **a dial on `SymRules`** (`unit_vector`, or the
name the mechanism earns), off = today's tier bit for bit, ON in
`SymRules::shipped` only if the measured cost on the five measured
documents (plate, annulus, link, bracket, pad — the M10-10 evidence
rows) is affordable (state the per-leaf cost before/after, the
affordability line is the header's 1.6 s), and the per-node bounds
that keep it from running away, stated like `EARLY_STEPS` /
`EARLY_AB_TERMS`. The soundness argument goes in the rule module's
header in the shape `trig.rs` uses: the identities, and why each is
unconditional under clause 1.

**The interval channel's own half** (`interval-self-dot-straddles-before-rule-a`:
`v·v` as a product of independent copies straddles zero on a wide box
where `powi(2)` is exact, so clause 1 refuses before any rule runs) is
NOT this unit's — it is a one-line change in PROPS' `linalg/vec.rs`
announced there — but this unit MEASURES whether the two DOCM rows
hit it at their widths (a `Trv` on the norm's `sqrt`) and says so, so
the two rows are known to be reachable by this unit's rule alone or
not.

## Phase 3 — the record

- The two DOCM rows green as this unit's pins (they become unit rows,
  un-ignored, subject-named), plus the per-predicate split at the
  nominal on the derived-frame document under the shipped set and
  `without_the_algebra` (the M10-10 pin shape).
- Every M10-8/9/10 pin unmoved or moved UPWARD (a theorem count that
  rises is re-baselined and named; one that falls is a bug); the
  five documents' whole-certifying ceilings re-measured under the new
  set (`m10_10_evidence_interval`), the table in the item body.
- `sym.rs`'s header: one section for the mechanism in the M10-10
  shape (what it reaches, measured; what it costs); the item body
  gains `## What stands, and what moved (SYM-5)`; the item's DOCM
  references re-pointed to the ledger's sweep 14.
- K: the new discharge is a THEOREM (rule E's kind) and lands in
  `symbolic_zero` / `SymbolicZero` — no new spelling of the discharge
  vocabulary (`registered-is-spelled-five-times-and-pinned-once` is
  why); if the mechanism needs its own column, stop and say so.

## Scope

- Files: `crates/geom-core/src/sym.rs`, `sym/*` (a new rule module
  if (a)); `crates/editor-core/tests/m10_*`; the item. **No change to
  `linalg/vec.rs`, `real.rs`, `topo::UnitVec3` or the derived-frame
  door in `editor-core`** — the fix is the tier's; if the measurement
  says the tier cannot reach it without a door change, stop after
  Phase 1, write what the door would have to hand the tier, and
  report (that is a design conversation, not this unit).
- `DEFAULT_SYM_MAX_DEGREE` / `MAX_TERMS` / `COEFF_BITS` do not move.
- Test cost: the new pins run at the nominal and one width; state
  their cost; nothing else gating is added.

## Acceptance

- The two DOCM rows green at every ε row on the full hosted matrix;
  the derived-frame document certifies where its authored twin does.
- No measured document's theorem count falls; the ceilings table
  re-taken; the per-leaf cost on the five documents stated
  before/after and under the affordability line, or the dial ships
  off with the number that says why.
- The rule's soundness argument in its module header; every rule
  pinned by a `sym_theorem`-shaped row (an identity it proves) and a
  negative row (a shape it must NOT fold — a non-unit vector, a zero
  vector, a straddling box).

## Review

The full v6 dual (`docs/MODEL-AB-LOG.md` protocol v6; ordinal from
SYM's band at dispatch). Claims for the reviewers to falsify: (1)
soundness — every fold is an equality of reals wherever clause 1
holds; the reviewers hunt a box where the rule folds a non-identity
(a nearly-unit vector, a vector whose norm straddles zero, a
parameter that makes `S` zero at one point); (2) the two DOCM rows
green for the stated reason (the rendered chain before/after); (3)
no measured document lost a theorem, and the ceilings table
reproduces; (4) the cost, re-taken; plus
`docs/prompts/reviewer-style-lane.md` in full. Union fix pass on the
implementer's lane; delta by R1; the row lands at merge.

## Landing

Status `review` on `work/sym/SYM-5.md` when the PR opens; the
orchestrator closes the unit and deletes this spec at merge; the item
closes with it or stays open on what the measurement says the tier
cannot reach.
