# M7-8 — Plane × NURBS intersection certification (declare-and-check)

Orchestrator work order for M7's last code unit, reconstructed
in-repo per working convention. Enabling context: PR #264 (D7 stage-1
NURBS recognition), which pinned the class this unit retires.

## The ratified ruling

Evan, PR #264 comment 5227046200:

> for 2, definitely certify plane×NURBS intersections. it'll
> presumably have to be a "declare and check that it actually works
> out" case rather than by construction, but that's ok

The orchestrator's recorded shape (binding):

- The imported file's carrier curve is adopted as **EVIDENCE** and
  certified against **both** surfaces — never derived by
  construction.
- **on-PLANE residual**: closed form (signed distance at samples,
  exact).
- **on-NURBS residual**: foot-point projection at a fixed schedule,
  plus a **certified between-samples envelope**.
- **transversality margin**: metered at the analytic side's lever
  arm (sin of the normal angle × the honest lever arm; the plane's
  normal is exact, the NURBS side's from `ders`).
- Enabling substrates to REUSE, not reimplement: the M6-2 generic
  projection (`NurbsSurface::project`, D9-fixed iteration) and
  #264's envelope machinery (span-aware schedules; rational-safe
  derivative-coefficient hulls with the min-weight denominator
  floor). A 1D-carrier-vs-surface adaptation of the envelope is
  expected; a parallel reimplementation is not.

## The concrete target — the SEAM-ORPHAN class

PR #264 §2 pinned it: a mixed promoted/stays-NURBS body — a promoted
PLANE wall adjacent to a stays-NURBS wall — whose shared edge lost
the bitwise IsoCurve rung (the promoted side's boundary columns are
gone) and had **no certification path**, so adoption refused typed
(`crates/step-import/tests/recognize_pins.rs`,
`cylinder_envelope_refuses_and_the_seam_orphan_is_pinned`; the
arc-prism fixture, edge #130).

This unit gives that edge its honest path: the file-stated carrier
certifies as an `EdgeGeometry::Intersection { s1, s2, witness }` (or
the appropriate D2 variant) through the declare-and-check
certification. **Acceptance is the pin FLIPPING**: the arc-prism
mixed body imports first-class, its seam edge carrying a certified
intersection. The S9 duty applies — the refusal's message and every
doc stating the gap get their retiring updates; no stale "no
certification path" text survives.

## Scope and bounds

- The certification lane lives where the adoption ladder's other
  rungs live (`crates/step-import/src/adopt.rs` plus the geom-brep
  certify machinery it calls). Touch geom-brep only for the reusable
  certification pieces (a carrier-vs-NURBS-surface residual bound).
  Modifying SSI/march or kernel construction paths is a STOP.
- **Transversality**: the D2 `Intersection` variant requires a
  transversality margin, metered per the ratified convention. An edge
  failing transversality at ε_in is NOT this lane's problem — it
  refuses with the existing tangency/ambiguity vocabulary. Do not
  invent a `TangentIntersection` adoption rung here; if the corpus
  needs one, report it as banked.
- **ONE new certification rung, evidence-first.** If the whole-curve
  envelope cannot certify some legitimate carrier (the bound too
  loose at ε), the honest outcome is the existing typed refusal WITH
  the measured bound in the payload — never a widened gate. Any such
  case is reported.
- The tier-at-import gate from #276 may merge mid-flight; re-merge
  main when it does. The new first-class body must be tier-valid at
  rest, which is the point.

## Acceptance

1. The seam-orphan pin FLIPS: the arc-prism mixed body imports
   first-class at default / 1e-6 / 1e-12; its seam edge carries the
   certified intersection (residuals + transversality margin
   reported); tier-3 valid at rest.
2. A planted falsifier: a doctored carrier displaced > ε off the
   true intersection REFUSES with the measured residual (the
   declare-and-check contract — never trusted).
3. A planted tangential case (normals near-parallel) refuses with
   the tangency/escalation vocabulary, not a silent accept.
4. Full step-import suite green × 3 ε rows; geom-brep suite green;
   no other fixture changes behavior (any delta reconciled).
5. D9: import twice, byte-compare.

## Process

Commit and push per sub-unit. Merge `origin/main` immediately before
opening the PR and re-merge whenever main moves while open (build the
union explicitly — the #274 lesson). Confirm checks started after
every push. Full writeup; the PR is NOT merged by this lane.
