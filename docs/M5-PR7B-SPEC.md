# M5 PR 7b — tensor-product Bernstein composition; plane×NURBS retirement (binding spec)

Branch `ev/m5-pr7b-tensor-compose` from current main. Origin: PR 7
review finding M2, RULED ACCEPT-AND-BANK (M5-LOG 2026-07-31) — the
reviewer confirmed the centered second-order tightening is
constructible but likely not ε-practical; tensor-product Bernstein
composition is the clean fix and its own reviewed unit. **EXIT-
GATING**: shape (iii)'s substrate row rides this PR (M5-PLAN R5).
Sequence alongside/after PR 9; independent merge order — nothing
here depends on the zip, and PR 9 wires the boolean arm so this
PR's flag flip alone makes it live.

## 1. Scope and trigger

The plane×NURBS SSI arm is complete except limb 2 against the NURBS
operand (geom-brep/src/ssi.rs:42-89 is the binding in-code
statement): the current per-span first-order enclosure
(ssi/certify.rs:432-499) is sound but scales with span width —
~1e-2 m reported where the true residual is ~1e-10 m — because it
encloses the two variation terms separately and throws away the
cancellation that IS the content of S(P(t)) = C(t). The fix:
enclose the difference S(P(t)) − C(t) as ONE composite in Bernstein
form. `geom_core::spline::compose` is curve-only by design; this PR
grows the tensor-product surface side.

## 2. The tensor composition (geom-core::spline::compose)

- Same contract as the curve module, one dimension up: **data in,
  bounds out** — nothing evaluates or samples anything (C2.2, OQ2);
  structure (knots, degrees, spans) from f64; coefficients in
  `RingInterval` (C6); entry-point structural errors typed
  (`ComposeError` grows arms as needed, closed enum per D3);
  downstream degeneracy poisons the bound, never panics.
- Pipeline, generalizing compose.rs:15-31 verbatim where possible:
  (1) center-shift before any product, weight channel carried;
  (2) Bézier-decompose the SURFACE by knot insertion in u and in v
  (tensor product of the two univariate decompositions — structure
  f64, α-coefficients as ring quotients of knot enclosures);
  (3) composition with the curve pair P(t) = (u(t), v(t)): per-span
  exact Bernstein products with binomial-quotient weights, degrees
  compose multiplicatively — document the degree/size budget and
  where it lands for the SSI fit degrees actually in use;
  (4) per-span coefficient hulls, rational quotient per span
  (zero-touching denominator poisons loudly), hulled across spans.
- The composite this PR must serve: `S(P(t)) − C(t)` per coordinate
  (difference formed AT THE COEFFICIENT level so the cancellation
  survives), sup-norm bound off the hull. API shape mirrors
  `CurveRingData`/`CompositeForm` (compose.rs:103-162, :526-572);
  scaling conventions section extends the m/m²/m⁴ table.
- Rustdoc derivation note pinning WHY the cancellation survives
  composition-then-hull but not hull-then-difference (the review's
  M2 finding, made a doc obligation).

## 3. Limb 2 flips to the tight bound

- ssi/certify.rs:432-499's per-span loop is replaced by the
  composite sup bound for NURBS operands; the K predicate name
  `ssi_hull_sup_chart` is UNCHANGED (telemetry continuity across
  the swap; the verdict-log row keeps passing untouched). The
  `aligned` fallback (:443-447, OQ4 contract) stays, now over the
  composite bound.
- `SSI_MAX_FIT_SAMPLES` / typed `FitSampleBudget` (the #146 ε-fix)
  stays as-is; re-derive whether the ε=1e-12 row's ~4015-sample
  demand changes under the tight bound and record the answer in the
  report (the budget row itself remains pinned either way).

## 4. The arm retires (C12.1: an arm retires WITH its proof)

- `plane_nurbs_ssi` (ssi.rs:730-745): the limb-2 refusal path flips
  to certification "by deleting nothing" — the arm's `implemented`
  flag flips; `trace_plane_nurbs_uncertified` (ssi.rs:934) becomes
  the certified path's substrate or is absorbed by it; the C5 table
  note (intersect.rs:336-350) rewrites to record the retirement and
  its date. The two-arm status table (ssi.rs:37-40) updates.
- All 13 ssi_* predicates keep their names; exclusion-cannot-lie
  probes re-run against the new bound (the PR 7 review's probe
  suite is in-tree — run it, don't re-derive it).

## 5. Acceptance

- **Shape (iii) substrate row, verbatim from M5-PR7-SPEC §6**: a
  directly-authored NURBS wall (lofts are PR 10) cut by a plane —
  rung-3 marched + fitted + certified carrier, ALL THREE limbs,
  both lanes, bit-replay. This row was left UNMET at PR 7
  (deviation 1); it lands GREEN here. Exit-gating.
- The bound-improvement is MEASURED and pinned order-of-magnitude:
  the fixture where the first-order enclosure reported ~1e-2 m
  certifies at the true ~1e-10 m scale (assert a conservative
  ceiling, e.g. ≤1e-8 m, not the exact value).
- Tensor-compose unit rows: hull bound is a true sup bound
  (dense-scan falsification probe, ≥1e5 samples, ratio ≥ 1.0);
  poison-on-zero-denominator row; degree-budget row (the largest
  SSI-realistic composition completes within the documented
  budget); ring-lane bit-replay.
- Multi-ε honesty (the #146 lesson): probe placements and
  corruptions scale from the resolved band; sample counts derived
  from the governing law; explicit skip-with-reason only where
  scaling is dishonest; verify locally at 1e-6/1e-12/Interval
  before push.
- Two-tolerance message shape for any NEW error arms, INCLUDING
  definite arms (S9 lesson).

## 6. Out of scope

NURBS×NURBS SSI retirement (limb 2 against BOTH operands — same
machinery, but its exhaustiveness/seeding story is not banked and
no acceptance shape needs it; the refusal stays typed and names
this PR's machinery as the substrate); the pcurve storage variant
(PR 9's zip); degree reduction (§5.6, plan R6); any marcher or
exhaustiveness change.

## 7. Process

One implementer + one blinded adversarial reviewer + one fix pass.
Review charter must include: independent hand-derivation of the
tensor Bézier-decomposition coefficient algebra (the α ring
quotients in two directions); an adversarial cancellation probe
family (constructed S, P, C where naive separate enclosure fails by
many orders — verify the composite tracks truth); a falsification
sweep on the sup bound; the CODE QUALITY REPORT with the fixed
rubric. Touched-crate local battery (geom-core, geom-brep; default
ε + Interval); hosted CI is the gate. Push per unit. Deviations
numbered. K-funnel: k-lint clean, no new funnel entries below the
one sanctioned door.
