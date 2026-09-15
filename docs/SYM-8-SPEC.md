# SYM-8 — the manifest sign: `abs` and `copysign` atoms whose sign the form already shows (spec)

**Program:** SYM (`work/sym/plan.md`, the ceiling lane). **Items:**
`work/sym/derived-frame-placement-freezes-on-the-symbolic-lane.md`
(SYM-5 PR-2's review found the next wall on it: the frame tilted about
the OTHER axis) and `work/sym/coefficient-ring-width-is-not-monotone-in-reach.md`
(the warning this unit is measured against). **Track:** kernel change —
the standard v6 unit (binding spec, drawn implementer arm, cross-model
dual review, union fix pass, record-at-merge; §Review). Block SYM-B2
slot 1. **Pre-draw fields, logged before the draw:** difficulty **H**,
task-class **NUMERIC**.

- **H** — a rule of the atom algebra whose soundness is an identity of
  reals with a signed-zero edge, measured against a recorded case
  where opening an atom LOST discharges.
- **NUMERIC** — it moves what the tier decides (more identities
  discharged, or, if the measurement says so, none), and every
  per-document split and ceiling is the acceptance.

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
two items whole; `docs/SYM-5-SPEC.md`'s shape at
`git show <SYM-5's fix-pass head>:docs/SYM-5-SPEC.md` (the ledger's
per-merge deletion entry names it) — Phase 1 first, the remedy the
measurement picks; `sym.rs`'s header (`# The form-level algebra`,
`# Freezing`, the rule-E section), `sym/trig.rs` (`manifestly_nonneg`
— the predicate this unit starts from, and rule D's shape),
`sym/signed.rs` whole (rule C: `abs(R) → ±R` where `R` is a polynomial
with a CERTIFIED sign — what it folds and what it declines, and why a
sign READ is counted `sign_gated`), `sym/quotient.rs` (rule E),
`sym/algebra.rs`; `crates/geom-core/src/linalg/vec.rs`'s orthonormal
basis (`s = 1.copysign(n.z)`, `r = 1/(1 + s·n.z)` — where the atoms
this unit is about are minted, and its D9/DL6 argument);
`crates/editor-core/src/eval/wire.rs`'s `FaceFrame` arm (`u_ref`,
`frame_axes`); the SYM-5 PR-2 review's tilt-U row and render (the
R2 rows adopted into `m10_derived_frame_tilted_interval.rs` by the fix
pass, and `crates/editor-core/tests/m10_bulge_renders.txt`).

## The claim

On a `FaceFrame` on the cap of a body extruded from a frame tilted
about the u axis (`u = (1,0,t)`), the derived boss refuses
`carrier_endpoint_end` with rule E on exactly as off: the frozen
`Powi ^2` kid goes 606 terms / degree 60 → 440 terms / degree 28 — the
degree wall becomes a TERM wall (440² > 4096) — and every term carries
the atoms `copysign(1, 1/sqrt(P(t)))` and `abs(1/sqrt(P(t)))` with
`P(t) = 17/16 + t/2 + t²`. Those are the orthonormal basis's
`s = 1.copysign(n.z)` and the `|·|` its denominator was spelled with,
taken of a quantity the FORM already shows positive: `1/sqrt(P)` is
`Inv` of a `sqrt` atom, and `P` is a sum of a positive constant and
squares. **A `copysign(1, X)` is the constant 1 and an `abs(X)` is `X`
wherever `X` is manifestly POSITIVE as a form** — an identity of
reals, no value read. Rule C cannot take these: its `R` must be a
polynomial in the parameters it can enclose, and `1/sqrt(P)` is not.

**Ratified and not re-litigated:** E12; rule C's contract (a sign READ
is `sign_gated`, never `symbolic_zero`); rules A/B/D/E; the freezing
budget; the ring item's finding that opening an atom can lose a
discharge (this unit measures against it, it does not argue it away).

## Phase 1 — before touching anything (the measurement)

1. **The wall, named.** On the tilt-U document (R2's construction, in
   the suite after SYM-5's fix pass), at `half = 1e-3` and `5e-2`,
   `Guided` and `Pinned`: render the refused residual at `explain_depth
   6`; count, over the early walk's forms on the decision path, the
   terms that carry a `copysign(1, ·)` or `abs(·)` atom of a manifestly
   positive argument, and the terms that would remain if those atoms
   were the constants they denote (a hand-planted fold, reverted; or
   arithmetic on the render). One table: form, terms before / after,
   degree, whether it would fit the budget.
2. **The manifest-positive predicate.** Write it down before coding it:
   which shapes count (an `Inv` of a `sqrt` atom; a `sqrt` atom of a
   manifestly positive form; a positive constant plus manifestly
   non-negative terms; an even power; a product of manifestly positive
   forms) and which do NOT (a sum of squares alone — it can be zero,
   and `copysign(1, +0.0) ≠ copysign(1, −0.0)`; anything with a
   parameter of unknown sign). State the signed-zero edge and why
   strict positivity closes it. This is `manifestly_nonneg` sharpened
   to positivity; say what it shares with it and put the shared part
   in one home.
3. **The recorded loss, re-taken.** The ring item's patch C (`abs(X) =
   X` for a syntactically non-negative `X`) was measured leaving R2's
   bracket bit-identical and, with patch A, losing ten decisions on
   R1's boss. Re-take the bulge boss and both D-tabs with a hand-planted
   fold of the shapes in 2 (reverted): every split and the boss's
   ceiling, before/after. **If a split moves DOWN or a ceiling falls,
   that is the unit's first finding and the rule's predicate is
   narrowed until it does not** — a rule that opens an atom the door
   was closing on is measured wrong on the record.

## Phase 2 — the rule the measurement picks

**Rule F — the manifest sign**, behind `SymRules::manifest_sign`
(or the name it earns), a new module `sym/manifest.rs` in the split's
shape: in the early walk, per node, `copysign(1, X) → 1`,
`copysign(Y, X) → abs(Y)` for manifestly positive `X`, and `abs(X) →
X` likewise; the module header carries the soundness argument as
equalities of reals under clause 1 (as `trig.rs` and `quotient.rs`
do), the predicate, and the signed-zero edge. Rule C stays what it
is (a READ, gated); this rule reads nothing and lands in
`symbolic_zero`. Ordering against A/B/E stated with the reason and
pinned by the walk ledger, not by a comment. A theorem row (the
tilt-U residual, or the simplest form that reaches it), the negative
rows (a sum of squares is NOT folded; an unknown-sign parameter is
not; poison is not), and the per-document splits and ceilings on the
six measured documents plus tilt-U, before/after, with the cost on
the header's leaf instrument (release) beside the ceiling instrument.
**Shipped on only if no split or ceiling moves down anywhere and the
cost is disclosed against the line** — a document class reached is
the reason to pay, as SYM-5 argued; a decision lost anywhere is the
reason not to ship, whatever is gained.

If Phase 1 shows the wall is not these atoms (the fold leaves the
forms over the term budget), STOP after Phase 1 and report what the
wall is; that is the unit's result.

## Scope

- Files: `crates/geom-core/src/sym.rs`, `sym/manifest.rs` (new),
  `sym/trig.rs` (the shared predicate's home), tests under
  `crates/geom-core/tests/m10_*` and
  `crates/editor-core/tests/m10_derived_frame_tilted_interval.rs`,
  `m10_bulge_interval.rs`, `m10_10_pins_interval.rs` (the announced
  tests-family overlap).
- No change to `linalg/vec.rs` (PROPS'): the atoms are minted there by
  a ratified spelling and the tier folds them; nothing at the value
  channel moves.
- No new tolerance, no value read.

## Acceptance

- Phase 1's three tables in the PR body; the predicate stated before
  the code.
- If shipped: every pinned split moved UP or not at all on the six
  documents; no ceiling moved down; the tilt-U parity row (the derived
  boss certifies where its authored twin does) green at the widths the
  twin certifies at, or the width it stops at pinned by name with the
  reason; the walk ledger re-baselined with its reason; the cost on
  both instruments.
- The header's rule table carries rule F.

## Review

The full v6 dual. Claims to falsify: (1) soundness — the signed-zero
edge (`copysign(1, X)` at a real zero of `X`; a manifestly positive
form whose value channel yields `−0.0`; poison) and any argument the
predicate calls positive that is not; (2) nothing lost — the six
documents' splits and ceilings, re-taken; the ring item's patch-A
case; (3) the wall is what the chain says; (4) the ordering against
rules A/B/E; (5) the cost; plus `docs/prompts/reviewer-style-lane.md`
in full. Union fix pass on the implementer's lane; delta by R1; the row
lands at merge.

## Landing

Status `review` on `work/sym/SYM-8.md` when the PR opens; the
orchestrator closes the unit and deletes this spec at merge; the
derived-frame row records the tilt-U outcome, the ring row a third
measurement.
