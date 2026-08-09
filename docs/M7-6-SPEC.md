# M7-6 — Stage-1 NURBS surface recognition (always-promote)

Orchestrator work order for M7's last unit before the exit walk,
recorded in-repo per working convention. Substrate evidence: the
stage-1 substrate inventory (file:line for every claim was verified
against the lane at implementation time).

## The ratified ruling (Evan, issue #256, 2026-08-08 — binding)

D7 stage 1 ships as **always-promote, with NO recognition option**:
every imported NURBS surface is tested for promotion to an analytic
kind; promotion fires iff the per-kind residual CERTIFIES at ε_in
(verified-not-trusted — file annotations like I-DEAS's `.PLANE_SURF.`
may seed candidate order but never bless); a surface that certifies
nowhere STAYS NURBS silently (the M7-3-legal state — recognition
failing must never refuse a face that imports today).

The own-corpus round-trip consequence is ACCEPTED: where promotion
fires on native exports (loft_prism, nonuniform_loft, swept_elbow
walls), the byte-golden first-re-export pin is RE-STATED per fixture:
enumerate exactly which surfaces promoted with their residuals, and
pin the **promoted one-cycle fixed point** (the promoted body's
re-export re-imports byte-identical; censuses and certified volumes
equal across the promotion). Byte-identity stays the pin for every
fixture promotion does not touch. A future kernel-side unit (banked
at #250, design record #256) restores first-cycle byte-identity for
the structure-visible class; no part of it is built here.

## Binding design decisions (orchestrator-ruled)

D-b **Kind scope: Plane + Cylinder only** — the measured need (dm1 is
17 exact planes + 7 exact rational cylinders). Cone/sphere/torus
recognition is UNIMPLEMENTED and stays-NURBS: one honest doc sentence
at the recognizer naming them banked, no dead per-kind code.

D-c **Certification, dual-track**: non-rational patches use the
closed-form control-net hull sup-bound where available (plane:
residual ≤ max control-point deviation from the fitted plane —
whole-patch, total); rational patches certify on the fixed
`CERT_SAMPLES` schedule + envelope (the adopt.rs arc-rim precedent —
never data-dependent iteration; D9-clean estimators only: Newell for
planes, closed-form axis/radius for cylinders, e.g. deg-(2,1) net
column structure + exact circle solve; each estimator's determinism
argument documented).

D-c2 **Selection + ambiguity**: deterministic kind-preference order
Plane > Cylinder for benign double-certification (a patch certifying
as both is canonicalization, not ambiguity — both agree within
2·ε_in on the patch; the argument is documented at the selection
site). A typed `RecognitionAmbiguous`-class error (D7's promised
ambiguity variant) fires only when the ESTIMATOR is ill-conditioned
by its own margin trilean (e.g. cylinder axis from a near-flat patch)
AND the face belongs to a refusing class (rescue needed); importable
faces with marginal recognition stay NURBS, recorded.

D-d **Rings + curves**: rings on promoted PLANES pass the existing
multi-bound gate natively (dm1's whole ring class); rings on promoted
CURVED faces keep the refusal with its message updated to name the
class honestly; curve-level recognition is OUT (the Intersection rung
is carrier-agnostic; dm1's edges adopt via witness+endpoints).

Slotting: promotion happens AT OR BEFORE surface construction or
inside `face()` BEFORE the multi-bound gate — the refusal fires
before normalize, so a normalize-level pass is too late.
`options.eps_in` is plumbed into resolve (the override governs
interpretation). Promotion is RECORDED through the
StructureNormalization/normalizations channel (census-mapped — the
existing reported-model-change mechanism; motion for exact promotions
is ~0, the residual is reported anyway).

## Sub-units (commit + push after each)

1. **QUASI_UNIFORM_CURVE/_SURFACE vocabulary** (implied clamped
   knots, closed-form synthesis) — moves dm1's frontier from parse to
   geometry; re-pin the first-refusal probes.
2. **Recognition core + promotion slot** per the decisions above
   (this spec commits with this sub-unit's first commit).
3. **The dm1 flip**: FreeCAD-derive its `.expect` sidecar (protocol
   per nist_ftc_09's; KERNEL_* overrides only where the kernel census
   legitimately diverges — derived, never fudged); WILD_REFUSALS →
   WILD_IMPORTS; Leg D pin rewrite (the ITEM_DEFINED_TRANSFORMATION
   now traverses); fuzz-seed and refusal-battery updates; the
   analytic re-export rides the own-dialect fixed-point row.
4. **Posture pins**: own-corpus promotion re-pins per the ruling
   (enumerate + promoted fixed point; verify which walls certify at
   ε_in and report which fixtures promote what); near-miss fixture (a
   planted ~2·ε_in-perturbed cylinder patch stays NURBS silently,
   recorded); the ill-conditioned-estimator typed row;
   promotion-recorded-in-normalizations pin; sense-gate/tier-3
   behavior of promoted faces (they gain analytic check-6 arms and
   exact volume — asserted, not assumed).

## Acceptance (all executed, numbers in the report)

- dm1 imports first-class: 24/24 surfaces promoted (17 planes
  residual 0.0, 7 cylinders ≤ ~1e-13 model units), census vs FreeCAD
  oracle, certified volume, tiers 1/2/3, at the wild ε window; the
  QUASI_UNIFORM vocabulary exercised; Leg D traverses.
- step-import suite green at default/1e-6/1e-12; no-panic count
  updated correctly (dm1 leaves the refusal list); every other
  refusal fixture unchanged (TAIL_TURBINE-class stays refused — its
  spline edge problem is not surface recognition).
- Own-corpus: promoted fixed point pinned per affected fixture with
  enumerated divergences; byte-identity pins intact for untouched
  fixtures; own-dialect fixed point holds on the promoted forms.
- Local battery scope: step-import + the recognition unit tests +
  geom-brep only if touched; hosted CI is the gate.
