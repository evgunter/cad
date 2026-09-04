# M10-8 — the arc family: atom algebra for the symbolic tier (E12's reserve, executed as measurement first)

STATUS: BINDING (dispatched 2026-09-04; opened from M10-7's deviation
D4 and widened by both M10-7 reviews; plan approved by Ev in chat,
2026-09-04). Unit branch `m10/m10-8-arc-family`. Program plan
`work/m10/plan.md`; design record `docs/ERROR-DESIGN.md` E12 (read in
full) with E6 as substrate; the item `work/m10/M10-8.md` (read in full
— its second half is the reviews' measurement and its caution is this
spec's first deliverable).

## Grounding (substrate facts; verify each at the site)

- **The tier as merged** (M10-7, #1725): `geom_core::sym` — `Sym<T>`,
  the per-leaf session (`with_session`), the lazy exact-rational
  QUOTIENT normal form (`form_in`, `Form`, `Poly`), opaque atoms keyed
  by their arguments' forms with the fold-over-a-zero-form rule
  (`sqrt 0 = 0`, `cos 0 = 1`, …), `is_identically_zero`, the two-clause
  `sign_within`, `SymCounts { symbolic_zero, numeric, frozen }`, and
  `SymOp::Opaque` minting one indeterminate per untracked value. The
  receipt rides `ParamBoxVerdict`; the K instrument records
  `SampleOutcome::SymbolicZero` and the driver row lints it (the gate
  can red again: `tools/k-lint`, `ci.yml`'s status capture).
- **The miss, measured three ways.** M10-7 D4: the two-hole plate's
  real study certifies whole only below 7.81e-7, tier on or off, first
  refusal `carrier_endpoint_start` `[0, 1.25e-9]`. R2's filleted
  L-bracket with two bores: 0 certified at every scale, factor exactly
  1.0, with `carrier_on_surface_1/2` and `witness_on_surface_1/2` at
  0 symbolic. R1's bracket: `carrier_endpoint_start [0, 1.5e-7]` at
  ±5e-8. All three on ARC-derived geometry.
- **Why, at the algebra.** A profile arc's carrier is built as
  `Curve3::Circle { center: c, radius: r, u_ref: (q − c).normalize() }`
  (`crates/sweep/src/swept.rs:351-356`), with `r` and `c` the
  profile's exact sagitta closed forms from the chord and the bulge
  (`crates/profile/src/seg.rs:146-149`: `signed_radius = len·(1+b²)/(4b)`
  and its `abs()`; the center along the chord normal). So
  `‖q − c‖² − r²` IS a rational identity the quotient form already
  proves zero; what it cannot prove is `‖q − c‖ − r`, because
  `‖q − c‖ = sqrt(X)` is an OPAQUE ATOM and `sqrt(r²) = r` needs the
  radius's SIGN. And `normalize()` is `v / sqrt(v·v)`, so every point
  the circle evaluates carries `u_ref·u_ref = (v·v)/sqrt(v·v)²`, which
  the form cannot reduce to 1 because the atom squared is not
  syntactically its argument; the cylinder's implicit residual
  `(w² − r²)/(2r)` (`crates/geom-brep/src/implicit.rs:96-104`) then
  inherits the un-reduced atom, which is why `carrier_on_surface_*`
  collapses too. The revolve arms add `sin_cos` atoms of one argument
  (`cos² + sin² = 1`).
- **Rulings that bind here**: E12 verbatim — a symbolic `Zero` is a
  theorem; nothing in the normal form reads a value EXCEPT where this
  spec adds clause 3 below, stated as such; no funnel site is edited;
  the frontier (iterated quantities) stays S-CERT's.

## Scope

### 1. Measure before mechanism (the item's caution, binding)

Before any rule ships, instrument the tier to REPORT, per decide site
that stays numeric, the SHAPE of the residual form that blocked it:
which atoms appear in the numerator (`sqrt` of what argument form,
`sin`/`cos` of what, `Inv` of what) and whether the form is zero
modulo the three candidate rules of §2 (apply them in a scratch copy;
count). Run it over the plate, R2's L-bracket (rebuild it from its
probe suite on `m10/m10-7-r2-probes`, adopted in-tree by M10-7) and
R1's bracket, at the real study's box. The output is a table: per
predicate, numeric decisions; of those, how many go symbolic under
rule A alone, A+B, A+B+C. That table is the FIRST commit and the PR
body's first section; it decides which rules §2 ships (a rule that
moves nothing on the three documents is not shipped — filed).

### 2. The rules (ship those §1 justifies)

- **A. `sqrt(X)² = X`** — the atom under squaring reduces to its
  argument. A pure algebraic rewrite, sound for every real `X ≥ 0`,
  and `X ≥ 0` holds wherever the value is real (clause 1: a `sqrt` of
  a negative enclosure is `Invalid` and never reaches the identity
  test). Representation is the implementer's (a `sqrt` atom carried
  with its argument form so that `s² → X` is a normal-form step, or a
  reduction pass over monomials); the property is what the review
  falsifies.
- **B. `sin(θ)² + cos(θ)² = 1`** for atoms of the SAME argument form
  — the Pythagorean reduction (eliminate `sin²` in favour of
  `1 − cos²` in the form, or equivalent). Sound unconditionally.
- **C. `sqrt(Q²) = Q` when `Q > 0` over the box — CLAUSE 3.** When a
  `sqrt` atom's argument form is a PERFECT SQUARE `Q²` of a rational
  function `Q` (exact polynomial square root of numerator and
  denominator, or `Q` recovered from a candidate the residual itself
  offers: for `sqrt(X) − R` test `NF(X) = NF(R)²`), the atom equals
  `|Q|`, and it equals `Q` iff `Q > 0`. That sign is decided
  NUMERICALLY: evaluate `Q` over the leaf's parameter box in the
  lane's interval arithmetic (the session holds every `Param` node's
  value) and accept the fold only if the enclosure is definitely
  positive by the funnel's own `sign_within`. This is the ONE place
  the normal form reads a value, and E12's "nothing reads a value" is
  amended to say so: the decision is a theorem CONDITIONAL on a
  certified sign over this leaf, width-dependent only through that
  sign (a radius `r ± w` is positive at any study width). The
  receipt counts these separately (§4).
- **Not shipped**: general radical simplification, factoring, `Inv(b)·b`
  beyond what the quotient form already gives, any trig identity
  beyond B. Each is an opaque atom, documented as a limit.

### 3. The registered-identity door (E12's reserve — only if §1 shows a family A–C miss)

`session.assert_equal(a: Sym<T>, b: Sym<T>)`: a constructor registers
an identity it guarantees; the f64 witness pass VERIFIES it at the
point (a point residual is tight — a constructor that lies is caught
there, and the registration is refused typed); the normal form
rewrites `NF(a) − NF(b) → 0` (an axiom indeterminate substitution).
Ship it ONLY for a family the §1 table shows rules A–C cannot reach,
with that family's registrant named; otherwise document it as the
reserve and leave the door unbuilt (a door nobody consumes is
machinery for zero certificate content — E6's own rule).

### 4. Honesty instruments

- `SymCounts` gains `sign_gated` (rule C decisions) beside
  `symbolic_zero`; `SampleOutcome` gains the matching K outcome (one
  home: `SampleOutcome::ALL`/`token()` and k-lint's cross-workspace
  test — M10-7's vocabulary rule; the driver row must LINT it, read
  from the log).
- The census (`geom_core::sym`'s table, 107 rows) gains a column: which
  rule discharged each explicit row on the three documents.
- **The ceiling, re-measured** on the plate AND on R2's L-bracket:
  the widest whole-certifying box, the first refusal beyond it named
  with its predicate and enclosure. The M10-7 rows that pin the plate
  at 7.81e-7 and the bracket at factor 1.0 FLIP by design — re-cut
  them as positive pins asserting the mechanism (the rule that
  discharged, the predicate that bounds).
- The tour's stop 1 becomes the certified study IF the plate certifies;
  if it does not, the caption says what still bounds it, in numbers.
- `work/m10/real-margin-dependency-widening.md` (M10-7 R1's class)
  is expected to be the next ceiling on curved geometry: when it is,
  say so with the predicate and enclosure, and do not widen this unit
  into it.

## Out of scope

The real-margin dependency-widening class (its own item); implicit
quantities (S-CERT); Taylor/affine forms; branch enumeration; any
change to the funnel sites; the GUI; the guided lift's refusal of a
parametric profile chain (R1's finding — file it if no item exists,
do not fix it here).

## Review claims to falsify

1. **§1's table is real**: the per-predicate counts reproduce; a rule
   that ships moved something on the three documents; a rule that did
   not ship is filed with its count.
2. **Rule A is sound and complete for its shape**: `sqrt(X)² − X`
   decides Zero at every width; `sqrt(X)·sqrt(X) − X` likewise;
   `sqrt(X)³ − X·sqrt(X)`; and NOTHING with a negative or straddling
   `X` reaches the identity test (clause 1) — attack with `sqrt(x − x)`,
   `sqrt(−y²)`, a box where `X` straddles zero.
3. **Rule B**: `cos²θ + sin²θ − 1` at every width and for every
   argument form; `cos²θ + sin²φ − 1` NEVER (different arguments);
   `sin(2θ) − 2 sinθ cosθ` never (not in scope, stays numeric).
4. **Rule C is a theorem conditional on a certified sign**: `sqrt(r²)
   − r` decides Zero when `r`'s enclosure over the box is definitely
   positive and NEVER when it straddles or is negative (build `r` as a
   parameter whose box crosses zero); `sqrt(r²) + r` decides Zero when
   `r` is definitely negative; the sign read is the funnel's
   `sign_within` (an `Indeterminate` sign means no fold); the count
   lands in `sign_gated`, never in `symbolic_zero`; the K row lints the
   new token (read the log).
5. **The plate and the bracket**: the re-cut M10-7 rows are green and
   assert the mechanism; the ceilings are numbers in the PR body with
   the first refusal beyond each named; if either document still does
   not certify its real study, the reason is a predicate and an
   enclosure, not prose.
6. **Zero impact with the rules off**: a dial (`SymbolicDials`) turns
   A/B/C off individually; all off reproduces M10-7's tier
   bit-identically (serialized verdicts and receipts) on every M10
   fixture; every non-`Sym` scalar unchanged.
7. **Cost**: the rules' cost per leaf measured on the three documents
   (M10-7's D17 numbers are the baseline); `Poly` operations added
   for A–C are pre-bounded by the budget.
8. **§3 was NOT built unless §1 required it**, and if built, its
   registrant is named, the f64 witness refuses a lying registration
   typed, and the rewrite is counted distinctly.
9. Every deviation in the PR body with the argument; D-numbering
   continues from this unit's first (D1).

## Acceptance

Hosted CI green on the drawn point plus `lane=interval / eps=1e-12 /
klint=all` by trailer on the final head with the driver K row LINTED
(per-file counts and TOTAL quoted from the log, the new token
non-zero); the §1 table; the re-cut pins green; the ceilings measured;
every deviation in the body. After merge the orchestrator re-cuts
`docs/M10-EXIT-WALK.md` (#1700) against the measured state.
