# M5 S2 — arc-leg fillet sugar (binding spec)

Executes M5-PLAN S2 (#101 R4): `LoopBuilder::fillet` grows
**arc-leg corners** — line×arc, arc×line, arc×arc. Branch
`ev/m5-s2-arc-fillet` from main. Standalone planar unit;
exit-listed.

## 1. Construction (solver-free, deterministic, D9)

The fillet center is the offset-carrier intersection: a line leg
offsets by r to the corner's interior side; an arc leg's circle
offsets to radius `R ∓ r` (sign from which side of the arc the
fillet sits — internal vs external tangency). Tangent points
computed exactly from the center (foot on the line; along the
center-to-center direction for circles). Exact where inputs are
exact (dyadic/rational + the usual sqrt forms); libm-only; fixed
evaluation order.

- **Same declared-tangency discipline**: the fillet declares both
  junction tangencies by construction, exactly as the line-line
  case does today. The verify layer stays the authority
  (`TangencyContradicted` unchanged).
- **Branch selection is a documented deterministic rule, never a
  guess**: the candidate whose tangent points lie on the legs'
  corner-side extents. Zero candidates ⇒ typed refusal (the
  corner has no tangent circle of radius r — the PATHS-DESIGN
  `NoCornerForFillet` situation, named consistently with the
  existing radius-does-not-fit taxonomy). Two candidates
  surviving the extent rule ⇒ typed refusal naming the ambiguity
  (do not pick).

## 2. Fit gating extended (the reified predicate)

`fillet_leg_fit` generalizes: on a line leg the setback is linear
(today's rule); on an arc leg the setback is **angular** — the
tangent point must lie strictly within the leg's swept extent,
compared as arc length (`R·Δθ`) in the same exact-order band.
Same reified-predicate route through k_stats (the one documented
sugar.rs exception); K-funnel registered; k-lint clean.
Radius-does-not-fit messages extend to name arc legs and their
angular margin as payload.

- Legs already mutually tangent at the corner (no corner to cut)
  ⇒ typed refusal telling the author to declare/keep the tangency
  instead of filleting.
- In-band configurations (near-tangent legs, near-concentric
  arc×arc, setback within the escalation band of the leg extent)
  ⇒ F6 escalated typed errors, named predicates with named lever
  arms.

## 3. Message discipline

All new/extended error Display text follows the ratified
two-tolerance shape (D4 ¶1 addendum): one message + one recourse
per user situation below eps_input, margins as payload. The S6
sweep (in flight on `ev/m5-s6-messages`) is building the shared
recourse carrier in predicate.rs — compose it if merged by then;
otherwise match its spec (docs/M5-S6-SPEC.md) so the sweep's
pattern absorbs these sites without rework.

## 4. Consistency with PATHS-DESIGN (v2 lowering)

This is v1 `LoopBuilder` surface — no algebra types are built.
But semantics and naming must lower cleanly to the ratified
algebra's `.fillet(r)`: same refusal situations
(`NoCornerForFillet`, trim-eats-anchor →
the existing fit-gate generalization), same
declared-tangency-by-construction posture. Note divergences (if
any prove necessary) explicitly in the report.

## 5. Acceptance

- Fixtures per corner class (line×arc both convexities, arc×arc
  internal + external tangency), each verifying: declared
  tangency verifies clean, tangency residual is
  zero-by-construction in ℝ (enclosure-width small in f64), the
  profile closes and validates.
- Refusal rows: radius too large (both leg kinds), no tangent
  circle, ambiguous branch, already-tangent corner, in-band
  escalations (each named predicate gets its
  definitely/exactly/in-band trio).
- The #100 bracket demo shape re-expressed with an arc leg
  (regression anchor).
- Full battery 3ε + interval; existing line-line fillet rows
  bit-identical; clippy both lanes -D warnings; fmt; doc-neutral.

## 6. Out of scope

NURBS-adjacent fillets (typed refusal stands); variable radius;
the PATHS algebra implementation (banked for v2); REST-contact
(S1); any solver machinery (this is construction, not constraint
solving — M6's line).

## 7. Process

Standard: foreground battery chunked per-package; one other cargo
lane may be active — no concurrent cargo; push per unit;
adversarial e2e review + fix pass; PR by orchestrator. OUTPUT
DISCIPLINE per standing header.
