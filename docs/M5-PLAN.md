# M5 work order: curved geometry (NURBS depth, SSI, fillets)

Status: **DRAFTED, proceeding without overnight block** (opened
2026-07-27 per Evan's explicit authorization at the M4 handoff: "it
seems safe for it to just write that and then go" — he reviews on
his morning pass; his standing patchable-not-fatal stance applies,
and any fork-dependent PR that dispatches before his reaction flags
the assumed answer prominently in its writeup). This document is
the **sequencing layer only**: the design layer is DONE —
`docs/CURVED-DESIGN.md` C1–C12 is the ratified contract (#85,
2026-07-24), every OQ1–OQ9 fork already carries a DECIDED entry
from the #85 conversation, and nothing here re-derives or
re-litigates any of it. Where this plan "resolves" anything, it is
the small residue the ratified record left to planning time
(staging triggers, scope-boxing of banked openers, schema
mechanics), each marked R1–R6 below with a firm recommendation.

Charter: DESIGN.md's M5 roadmap entry — NURBS depth (sweeps/lofts);
first SSI marching; constant-radius fillets — as mechanized by
CURVED-DESIGN's thesis: exactness where closed forms exist (D3),
intensional descriptions with certified caches where they don't
(D2/D4), and every completeness claim backed by an exclusion
certificate rather than an algorithm's diligence. Marching produces
candidates; certification produces truth.

Process conventions inherited from M4 unchanged: one implementer +
one adversarial e2e reviewer + one fix pass per PR; binding
orchestrator specs before dispatch; OUTPUT DISCIPLINE headers;
foreground-battery clause verbatim in every prompt; hosted Actions
merge gate with full-matrix watcher floors (18 rows at M5 open;
floors bump in the PR that grows the matrix); push-per-unit; the
Opus/Fable A/B experiment continues (blocked pairs, blinded
reviewers, fixed rubric — docs/MODEL-AB-LOG.md; block 4 opens with
the first M5 implementation dispatch).

## Inputs, with status (nothing here is new work)

- **CURVED-DESIGN.md C1–C12** — ratified #85. The decided forks,
  restated one line each so this plan is self-contained: OQ1 conics
  = (b) staged via (a) (`Ellipse` lands first; trio rides demand);
  OQ2 = hull bounds are an ENTRY requirement + uniqueness tube on
  every fitted `Intersection` at rest; OQ3 = exhaustiveness gate is
  IN-OP (found-or-typed before the op returns); OQ4 =
  carrier-primary; OQ5 = census stays exact-on-planar, touching
  curved results refuse typed at 3′; OQ6 = closed smooth chains +
  three-convex-edge sphere-octant corner, die-with-pips demo is the
  acceptance target, run-out vocabulary as refusal-payload names
  only; OQ7 = two-level tangency enforcement (mark + jet-determinate
  must-carry), rename `TangencyLocus` → `TangentIntersection`;
  OQ8 = in-house interval ring approved, temporary
  inari-on-default-path allowance while it lands; OQ9 = curvo audit
  verdict lands as a lean DESIGN.md Q5 revision.
- **Post-ratification obligations** (CURVED header): the DESIGN.md
  D2-sharpening pass (TangencyLocus → TangentIntersection rename
  sweep; Q5 lean revision once the audit reports), the inari
  quarantine boundary-text update for the transition state, and the
  LGPL-before-publish exit condition. Scheduled below (PR 0, PR 2).
- **Interval-crate adoption is GREEN-LIT** (Evan, in-session
  2026-07-27: "replace it whenever it's convenient") — no longer a
  plan-gated decision. The `interval-transcendentals` crate (#115,
  dual-oracle certified) replaces inari as the kernel's
  `T = Interval` transcendental backend as a standalone unit; the
  M0 poison-channel contract is the acceptance bar. Scheduled as
  PR 1.
- **Banked M5 openers** (DESIGN.md M5 line, M4-LOG): curved STEP
  subset; arc-leg fillet sugar (#101 R4 follow-up); REST-contact
  join lane (the crosslap frontier, #102 R7); #89 K-revisit at the
  M5 exit — with its baseline banked by 8b: the K-probe over corpus
  + demos (~2.56M samples/ε-row) is sharply bimodal (zero mode
  ≤ 5.33e-15, definite floor 1.689e-3, 12-decade empty gap), so
  K = 10 is unpressured on the ANALYTIC kernel; T5 says M5's
  boolean/SSI predicates are the corpus that could change that.
- **PERF-PLAN riders**: C10's BVH trigger fires at M5; T6 predicts
  CDT bulk-loading's trigger ("first real fine-δ export need, or
  corpus CDT dominance") very likely fires mid-milestone — planned
  as an M5-adjacent PR, not discovered as a regression.
- **Persistence schema v1 is FROZEN** (#112) — M5's new feature
  nodes interact with it only through the explicit migration chain
  (R3 below).

## Residual decisions resolved at planning (R1–R6; recommendations firm)

**R1 — plane×cone staging trigger (OQ1's rider).** OQ1 decided
(b)-staged-via-(a): `Ellipse` ships with plane×cylinder booleans;
the parabola/hyperbola decision "rides on whether plane×cone
acceptance shapes make M5." *Resolution: no plane×cone acceptance
shape joins the M5 corpus, so the trio does NOT land in M5.* The
C5 table's plane×cone arm handles the exact-degenerate cases
closed-form (apex-through plane, axis-normal circle cut, tangent
plane) and routes generic tilt to rung 3 explicitly and permanently
(C5's no-runtime-fallback rule: a documented arm decision, moved
only by a future PR that adds the variants). Grounds: the M5
acceptance shapes (R5) are cylinder/sphere/torus/NURBS-dominated;
adding two unbounded conic variants for a configuration nothing
exercises would be speculative enum growth — exactly what
(b)-staged-via-(a) was designed to avoid paying early.

**R2 — Interval ring vs. crate adoption: two units, one trait.**
With #115 in-repo the two could be conflated; they stay separate as
C9 specified: PR 1 swaps the `T = Interval` transcendental backend
(inari → interval-transcendentals) behind the existing `interval`
feature — pure backend swap, no semantic change, poison contract
preserved; PR 2 lands the small geom-core interval RING (±, ×, ÷,
ulp-widened outward rounding, MIT-clean, default build path) that
C2/C3/C9 certification consumes. They meet at the `Bounds` trait;
certification code never knows which it got. Consequence recorded:
once BOTH land, no LGPL dependency remains anywhere (inari retires
entirely rather than returning behind the feature — the C9
transition text's exit condition is satisfied by removal, which is
strictly cleaner than re-quarantine), and the crate-table quarantine
text retires with it. If PR 1's review surfaces a poison-contract
gap that delays adoption, the C9 transition allowance (inari
temporarily default-path) stands as ratified — certification (PR 2)
does not block on PR 1 either way.

**R3 — Schema evolution for M5 feature nodes.** New persisted node
kinds (loft, sweep, fillet) and the `TangentIntersection` variant
require a schema bump. *Resolution: schema v2 is minted ONCE, at
the first PR that persists a new node kind (PR 10, sweeps/lofts),
via the F3 explicit migration chain (v1 → v2 function; v1 documents
load forever); later M5 PRs extend v2 before any release freezes
it.* Per-PR schema versions are noise; deferring all persistence to
milestone end would leave sweeps/lofts demo-only for weeks — the
single mid-milestone bump is the honest middle.

**R4 — Banked-opener scope-boxing.** REST-contact join lane and
arc-leg fillet sugar are IN M5 as standalone planar units (both
close recorded frontiers; both are small; neither touches the
curved chain) and both are exit-listed. The curved STEP subset is
exit-gating (CURVED's envelope commits export growth to conics +
NURBS entities). The save/load shared-validator consolidation
(DESIGN convention 2's migration note) rides along as NON-gating
hygiene, scheduled opportunistically into a lull.

**R5 — The M5 acceptance-shape set** (corpus additions; each an
exit-criteria anchor): (i) tilted-plane×cylinder cut — the exact
`Ellipse` carrier, zero-residual-by-construction; (ii) cylinder
boss ∪ plate — the first transverse curved boolean end-to-end;
(iii) a loft/sweep body (definitional NURBS surfaces) subsequently
cut by a plane — rung-3 SSI against a NURBS wall, exhaustiveness
in-op; (iv) a small-loop fixture (near-tangent cylinder×torus or
equivalent) where naive marching MISSES a branch — must either find
it via the exclusion subdivision or refuse typed, never silence;
(v) the **die-with-pips fillet upgrade** (OQ6's named target:
closed chains on pip rims, open chains terminating in sphere-octant
corners on the cube edges). All five join the Band 4 corpus with
the standard persistence/latency rows.

**R6 — What "first SSI marching" must NOT grow into.** The
roadmap's M5 line is honored at CURVED's scope exactly: no offsets/
shelling, no variable-radius fillets or chamfer features, no curved
census, no HLR, no import, no repair/adoption op, no preview lane
(OQ3(c) stays collapsed to (a)), no i_overlay, no degree reduction,
no scattered-data fitting. Each is a typed frontier or a later
milestone per the ratified envelope.

## PR sequence

Dependency edges in brackets; per-PR acceptance rides the cited
C-decisions. PRs 0–3 are fork-independent and dispatch immediately
(they depend on nothing Evan could still want to patch); the first
fork-touching PR is PR 5 (R1's cone-arm routing), by which point
the morning review will almost certainly have happened.

0. **CURVED post-ratification docs pass** (this PR): the D2
   sharpening in DESIGN.md (`TangencyLocus` → `TangentIntersection`
   rename sweep — doc-level; no code exists yet), the quarantine
   boundary-text transition state + LGPL-before-publish exit
   condition, M5-PLAN + M5-LOG seeded. [none]
1. **interval-transcendentals adoption** (green-lit): swap the
   kernel `interval` feature's transcendental backend inari →
   #115's crate; M0 poison-channel contract is the acceptance bar
   (decoration-equivalent refusal behavior, poison through values
   never decisions); full battery at 3ε + Interval; the crate joins
   the workspace (un-excluded). inari retires from the dependency
   tree when PR 2 also lands (R2). [none; parallelizable]
2. **The C9 interval ring** in geom-core: `IntervalRing` (name at
   PR spec), f64 endpoints, outward ulp-widening, ring ops only;
   the `Bounds` trait seam; control-coefficient hull-bound
   primitives (per-span convex-hull enclosure of a spline in
   B-spline form — the Eq. 9.81 mechanism, C2.2/C9); differential
   tests against inari/#115 as oracles. Entry requirement for all
   fitted-cache certification (OQ2). [none; parallelizable]
3. **geom-core::linalg + NURBS substrate part 1** (C11, C12.8):
   fixed-order small dense/banded LSQ + SVD (Givens/Householder,
   D9 fixed-shape); NURBS curve (2-D/3-D) and surface types,
   de Boor evaluation + derivatives generic over `Real` (ring ops
   only), positive-weights invariant enforced at construction; knot
   insertion §5.2 / refinement §5.3 / removal §5.4 with Tiller
   bounds / degree elevation §5.5. Acceptance: evaluator
   differential suite vs closed forms (circles as rational
   quadratics etc.), bit-replay rows at 3ε + Interval from day one
   (T4). [none]
4. **NURBS substrate part 2** (C11, C6): point projection/inversion
   §6.1 with certified orthogonality residuals; the global fitting
   stack (LSQ Eqs. 9.63–9.67, bounded Type-2 loop A9.10) under C6's
   pinning rule — structure f64-selected, certification
   scalar-generic; 2-D pcurve fitting on the same machinery.
   Acceptance: fit-then-certify round trips on known loci; a
   deliberately-corrupted fit FAILS certification (the hull bound
   catches a planted between-samples excursion). [3, 2]
5. **`Ellipse` carrier + the C5 dispatch table** (C1 rung 2, C5,
   C12.1): `Curve3::Ellipse`; closed forms for tilted
   plane×cylinder, equal-radius cylinder×cylinder, plane×cone
   degenerate cases (R1: generic cone tilt routes to rung 3
   explicitly); the face-intersection seam refactors into THE
   exhaustive (SurfaceKind × SurfaceKind) table with within-pair
   degeneracy trileans (named lever arms) classified BEFORE any
   rung runs; `CurvedBooleanUnsupported` retires per-arm;
   `split_edge` conic lane (C12.3, parameter-interval split,
   bounded like circles); pcurves for conic carriers closed-form
   per chart where exact, fitted where transcendental (C4).
   Acceptance shape (i); M2 pairs enter the table unchanged with
   their certificates. [3; 4 for fitted pcurves]
6. **Pcurves as per-half-edge certified caches** (C4): storage at
   the (edge, face-side) incidence; he_plus-forward parameter;
   certification IN METERS through the map (|S(P(t)) − C(t)| ≤ ε,
   hull-bounded per C2.2); domain validity (trim containment,
   one-branch periodic unwrap pinned at start); planar faces keep
   the derive-on-demand status. [4, 5]
7. **SSI: march-then-certify + in-op exhaustiveness** (C2, C3,
   OQ3): the Hoffmann §6.2 stepper (third-order Frenet approximant,
   SVD, Newton refinement — f64, libm-only, UNTRUSTED candidate
   generator); ℝ⁴ trace for parametric×parametric (§6.3.2), per-arm
   chart choice documented in the C5 table; rung-3 fitted caches
   with the FULL C2 certificate — on-locus residuals, hull-bound
   sup-norm honesty, uniqueness-tube component selection (OQ2:
   both, always); exhaustiveness subdivision in-op (exclusion /
   accounted / refine, typed `SsiExhaustivenessInconclusive` at the
   floor), doubling as the marcher's seed generator; closure/loop
   topology as named trileans; σ₂-sliver refusal toward C7 (never
   desingularize). Idealized/realized differential suite (tiny-h
   tangent stepper) in CI from this PR (T4, PERF-PLAN §4.4).
   Acceptance shape (iv): the planted small-loop fixture is found
   or typed-refused. [4, 5, 6; BVH from 8 optional at this stage —
   brute-force subdivision acceptable, swapped under 8's
   differential suite]
8. **The BVH crate** (C10): deterministic AABB tree
   (arena-order build, fixed split rule, total tie-breaks);
   conservative-superset contract; consumers wired = boolean
   edge×face sweep (retiring M3's documented quadratic), SSI
   seeding/subdivision cells carrying C9 enclosures; certified
   boxes for curved entities (analytic closed-form, NURBS control
   hulls). Idealized/realized (brute-force all-pairs) differential
   suite from day one. [3; parallelizable with 6/7 until wiring]
9. **Curved booleans end-to-end + the tangency regime** (C7,
   C12.2/4/5, OQ5/OQ7): SSI wired into splitting/boolean;
   second-order sector classification (normal-curvature trilean
   tie-break where first order ties, in-band osculation escalates —
   the new K-funnel predicate family, telemetry from birth);
   `TangentIntersection` variant landed (D2-sharpened name) with
   jet-schedule certification; tier-3 tangency MARK on every
   definitely-tangent edge + `TangentNotIntrinsic` must-carry on
   jet-determinate tangencies only (G2 joins exempt by predicate);
   cosurface `merge_coplanar_faces` generalization (C12.5, same
   ladder, never-numeric); census boundary text names the C7/OQ5
   deferral (C12.4); touching curved results refuse typed at 3′.
   Acceptance shapes (ii) and (iii). [5, 6, 7, 8]
10. **Sweeps/lofts** (C11 ch. 10): skinned §10.3 / swept §10.4
    surfaces as DEFINITIONAL feature nodes (Q8: the produced NURBS
    is the definition; no residual obligation; derived items carry
    certificates); recipe vocabulary + **schema v2 minted here**
    (R3, F3 migration chain, v1 loads forever); corpus + demos.
    Acceptance: shape (iii)'s loft body; round-trip + replay rows
    at 3ε + Interval. [3; 4 for derived-item certificates]
11. **Curved tessellation + mass properties** (C12.6/7): UV-grid +
    CDT extends to NURBS faces with pcurve-driven trim loops;
    chordal bound from hull-bounded second derivatives (C9
    machinery); watertightness keeps compute-chords-once-per-edge;
    divergence-theorem props for NURBS-walled faces via certified
    quadrature (interval/hull-bounded remainder — the kernel's
    first quadrature, scope-boxed to what R5 shapes need; certified
    bounds or typed refusal, no silent Gaussian trust). T6 tripwire
    lives here: if CDT dominates the corpus rows, the bulk-loading
    PR dispatches as M5-adjacent. [6, 9, 10]
12. **Fillets** (C8, OQ6): the validity-predicate battery FIRST,
    each a named Q1 trilean over the INPUTS (r vs 1/κ_max, face
    consumption, spine regularity, chain G1 closure, convexity-sign
    consistency, corner configuration) with fixtures that fire
    every one pre-construction; analytic-first blends
    (plane–plane → cylinder, straight spine → cylinder, arc spine →
    torus, vertex ball → sphere, cone cases per configuration);
    general spine → canal-surface fitted blend, the kernel's first
    approximating SURFACE (intensional `Blend { s1, s2, r }`, C2
    certificate lifted one dimension, envelope-system residuals);
    trimlines stored `TangentIntersection` from birth
    (prefer-intrinsic, D7 leave-room); sphere-octant corner patch;
    `FilletCornerUnsupported` with the OQ6 refusal-payload
    vocabulary. Acceptance shape (v): the die-with-pips upgrade.
    [9; 11 for the demo's export]
13. **Curved STEP subset**: AP214 writer grows conic + NURBS
    entities (§12.3.2 exact forms; conics round-trip through
    rational-quadratic export form per C1); FreeCAD import
    acceptance on the R5 corpus shapes incl. the filleted die;
    demo-tour curved narration updates. [10, 12]
14. **M5 exit sweep**: the T5 K-telemetry snapshot over the curved
    corpus (the #89 revisit fires — decision recorded or explicitly
    continued with grounds); envelope/DESIGN.md sweep (M5 line →
    done, new conventions proposed, quarantine text retired if PR
    1+2 landed, frontier entries updated); Band 4 corpus rows for
    all R5 shapes green; exit walk against the criteria below;
    state-doc trim; A/B log readout continues. [all]

**Side units** (standalone, slot into lane availability, planar):
- **S1 — REST-contact join lane** (#102 R7): the crosslap mate's
  pure rest contact gets its join-stage lane; `crosslap_rest.rs`
  doors flip from pinned refusal to certified pass. Exit-listed.
- **S2 — arc-leg fillet sugar** (#101 R4): `LoopBuilder::fillet`
  grows arc-leg corners; same declared-tangency discipline; fit
  gating extended. Exit-listed.
- **S3 — curvo audit** (C12.9, Q5/OQ9): depend/vendor/study verdict
  written BEFORE PR 4 dispatches (it informs the fitting stack);
  lands as the lean DESIGN.md Q5 revision + audit note. truck +
  opencascade-rs join as SSI/boolean e2e test oracles per standing
  review policy.
- **S4 — save/load shared-validator consolidation** (DESIGN
  convention 2 migration note): non-gating hygiene.

## Deliberately not in M5

R6's list verbatim: offsets/shelling (Q8 stands; PR 12 builds the
reusable machinery); variable-radius fillets, chamfers-as-features
(Band 3; two-plane composition is the documented dodge); curved
census (own design doc, OQ5); HLR/silhouettes; STEP import (M7);
the repair/adoption op for near-tangent operands (M5+ per F6 —
error text names the front door, D7 leave-room); preview-lane
exhaustiveness relaxation; i_overlay; degree reduction §5.6;
scattered-data surface fitting (M7's tool). Python bindings and the
usability program stay post-kernel-milestones per the sequencing
stance.

## Exit criteria

A tilted plane×cylinder boolean carries an exact `Ellipse` carrier
with residual identically zero by construction, replaying
bit-identically at ε ∈ {1e-6, 1e-9, 1e-12} + Interval; a transverse
curved boolean (cylinder boss ∪ plate) certifies end-to-end at tier
3 with every fitted cache carrying the full C2 certificate (hull
sup-norm + uniqueness tube — no schedule-max-only cache at rest);
the small-loop fixture is found by exclusion subdivision or refuses
typed `SsiExhaustivenessInconclusive` — verified by a fixture where
naive marching provably misses; every curved edge at rest carries
per-half-edge pcurves certified in meters, seam edges with distinct
pcurves; a NURBS-wall boolean (cut loft) marches, fits, certifies,
and passes in-op exhaustiveness; second-order sector classification
resolves a first-order tie with the normal-curvature trilean and
escalates in-band osculation typed; definitely-tangent edges carry
the tangency mark and jet-determinate tangencies enforce
`TangentIntersection` (G2 conventional joins exempt by predicate,
pinned both directions); the die-with-pips fillet demo builds,
certifies, tessellates watertight, and exports; every C8 validity
predicate has a fixture firing it as a typed pre-construction
error; `FilletCornerUnsupported` payloads pinned; sweeps/lofts
persist under schema v2 with v1 documents still loading; curved
STEP exports (conics + NURBS) of the R5 corpus shapes import intact
into FreeCAD; touching curved boolean results refuse typed at the
3′ gate (envelope pinned); the BVH differential suite is green
(realized ⊇ idealized, bit-equal results) and the M3 boolean-sweep
quadratic is retired; SSI bit-replay CI rows exist from the first
SSI PR onward; the interval backend swap is complete with the M0
poison contract intact and no LGPL dependency in any build
configuration (quarantine text retired); REST-contact crosslap
certifies through its join lane; arc-leg fillet sugar ships; the
M5 exit K-telemetry snapshot over the curved corpus is taken and
the #89 decision is recorded (or explicitly continued with
grounds); new conventions ratified into DESIGN.md at exit.

## Q9 note

Name still open (Evan's call; #107 shortlist). M5 does not gate on
it.
