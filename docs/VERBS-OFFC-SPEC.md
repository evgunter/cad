# VERBS-OFF-C — `Surface::Approx`: the approximating surface enters the kernel

Wave 3 unit 3 of `docs/VERBS-PLAN.md`, per the ratified
`docs/OFFSET-DESIGN.md` O2 (the seventh variant, D3-argued) and O5
(the validator's never-trust posture). Branch `verbs/offc`, PR to
main. Difficulty logged pre-dispatch: **L** — a wide, mostly
mechanical enumeration with three load-bearing decisions already
ratified.

## Scope

1. **The triple, lifted from EdgeCurve** (O2's ratified shape):
   - `SurfaceDescription<T>` — intensional layer; first inhabitant
     `Offset { base, d }`. Where the base lives (a key into the
     body's surface arena vs an owned Arc) is the implementer's
     first real decision: EdgeGeometry's `Intersection { s1, s2 }`
     names arena keys — follow that precedent unless a concrete
     obstruction says otherwise, and state the choice.
   - `ApproxSurface<T>` — description + fitted `NurbsSurface` +
     domain window + PRIVATE certificate (uncertified
     unrepresentable — the EdgeCurve invariant lifted). Certify via
     OFF-B's `certify_offset`; the RationalFitUnsupported refusal
     propagates (never bypassed — #1005 owns the capability).
   - **`Surface::Approx(Arc<ApproxSurface<T>>)`** — the seventh
     variant. The compiler then enumerates every dispatch site;
     each must SAY what it does (O2's ratified argument). Expected
     dispositions: eval/derivs/normal delegate to the fitted NURBS;
     boxes likewise (the fit is the geometry; the certificate
     bounds its distance to the intent); transform composes with
     the description (a rigid map of an offset is the offset of the
     rigid map — say it and pin it); the boolean operand gate
     treats it as its fitted kind does NOT — Approx is
     unsupported-kind for germ purposes (honest refusal via the
     pair-scoped gate); STEP export refuses typed (its own future
     conversation); tessellation delegates to the fitted NURBS
     (mesh's nurbs machinery — the certificate's bound may widen
     mesh tolerances and the spec deliberately does NOT wire that
     yet: delegate plainly, note the widening as a scheduled
     follow-on).
   - **The apex-window predicate** (ordinal-73's residue, plan-
     scheduled): where a cone-based description would arise this
     unit only handles NURBS bases — state that Offset{base} is
     NURBS-only HERE (analytic bases go through OFF-A's exact
     mints and never need Approx), which DISSOLVES the apex-window
     predicate for this unit; re-point the plan line at the
     face-replacement unit (OFF-D/shell) where mixed analytic
     surgery actually arises.
2. **Validator: re-derive per face** (O5, ratified): tier 3
   re-runs `certify_offset` against the description per validation
   call — never trusts the stored certificate; a failed re-derive
   is a typed validation error. NURBS-adjacent-edge dihedral
   exemptions apply to Approx faces BY KIND (O5's recorded
   inheritance; narrowing is a future conversation).
3. **A body-reachable consumer, minimal**: a door that replaces a
   test body's NURBS face surface with its certified offset
   (attach-layer `set_face_surface` + re-description per the
   surgery ordering discipline) — NOT the shell verb, NOT face
   removal: the smallest honest path that makes tier-3 validation
   of an Approx face reachable end to end. If even that drags in
   more surgery than the enumeration itself, an integration test
   constructing the body directly is acceptable — say which was
   built and why.

## Fences

- No shell, no offset_inward, no rim surgery (OFF-D's).
- No persistence schema change beyond what a new Surface variant
  forces; if it forces one, the C4 bytes-above-the-boundary rule
  applies and the posture is stated.
- No mesh-tolerance wiring (noted follow-on).
- #1005/#1008's capabilities not built ahead.

## Acceptance

- The enumeration is TOTAL: no `_ =>` arm anywhere dispatches
  Approx silently (the D3 exhaustiveness rule — the compiler is
  the sweep; state any site where a catch-all had to be split).
- An Approx-faced body validates tier 3 end to end, with the
  re-derived certificate; a planted-degraded fit (coarsened
  stored NURBS) goes RED at validation (the never-trust posture's
  red direction).
- Transform: rigid-map-then-offset ≡ offset-then-rigid-map, pinned
  (the description composes).
- The boolean gate refuses an Approx face pair-scoped; tessellation
  produces a mesh through the delegate path.
- Interval rows where the certificate arithmetic runs; both d
  signs; existing suites green (the wide enumeration must be
  behavior-preserving everywhere Approx does not appear —
  bit-identical is the cheap proof).

## Lane obligations

`docs/prompts/implementer-discipline.md` binds. No Co-Authored-By
trailer (blinding). Lane-private PR draft. Merge origin/main before
opening the PR titled "VERBS-OFF-C: Surface::Approx — the
approximating surface enters the kernel"; confirm CI runs STARTED;
note the drawn point and the coverage statement; watch to
completion. Do not merge. STOP for adjudication if the enumeration
surfaces a dispatch site whose honest disposition contradicts a
ratified decision.
