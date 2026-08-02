# M5 PR 11 — curved tessellation + certified mass properties (binding spec)

Branch `ev/m5-pr11-tessellation` from current main. Plan line 11
(docs/M5-PLAN.md:283-292); CURVED-DESIGN C12.6 (:666-672) and
C12.7 (:673-677). **This is the M5 demo moment by Evan's standing
ruling** — the tiltedcut tour stop flips from staged to rendering
here. Consumes PR 6 (pcurves ARE the trim-loop source — C4's
consumers begin here), PR 5/S9 (exact conic sections), PR 2/C9
(hull-bounded derivatives), PR 8 (BVH where useful). Binding;
deviations numbered, never improvised.

## 1. Curved tessellation (C12.6)

- UV-grid + CDT extends to CURVED faces with pcurve-driven trim
  loops: the face's half-edge pcurves (PR 6 caches; exact conic
  and Harmonic lanes) provide the trim polyline in UV; spade CDT
  in UV space per the mesh crate's existing pattern; exterior
  classification stays ours (#116 even-odd).
- The chordal bound generalizes: closed-form sagitta (planar arcs
  today) → hull-bounded second derivatives (C9 machinery) for
  curved charts. CONSERVATIVE, certified: chord error ≤ bound or
  the face refuses typed. δ is the caller's, as today.
- Watertightness keeps compute-chords-once-per-edge: shared edges
  tessellate once in 3-D and both faces consume the same chords —
  including curved-curved shared edges (the boss∪plate seam arcs)
  and seam-edge pairs (distinct pcurves, one 3-D chord set).
- mesh::walk's UnsupportedCurve ("conic cut boundary on a curved
  chart — the trimmed-face lane lands at M5 PR 11") RETIRES; its
  refusal row flips to a construction row (S9 pattern). Faces
  whose support is genuinely beyond scope (NURBS wall faces cannot
  exist at rest until the loft assembly lands) keep typed refusals
  naming the assembly unit — flip what is constructible, never
  claim what is not.

## 2. Certified mass properties (C12.7)

- Divergence-theorem contributions for curved-CUT faces (analytic
  charts with curved trim loops — the tiltedcut halves, the
  boss∪plate walls with their seam trims) via certified
  quadrature: interval/hull-bounded remainder — the kernel's
  FIRST quadrature. Certified bounds or typed refusal; no silent
  Gaussian trust (D4).
- Scope-boxed to what the R5 shapes need (plan text): the
  machinery is written for hull-bounded integrands generally, but
  acceptance only demands the constructible-at-rest classes.
  PropsError::Unimplemented for Surface::Nurbs may stay IF the
  quadrature lane's door for it is honest about the blocker (no
  NURBS faces exist at rest; the loft assembly unit flips it) —
  numbered deviation either way.
- tier-3 check 7 (VolumeUncomputable) flips for the newly
  integrable classes; the volume backstop consumes the certified
  bounds.
- K-funnel: quadrature refinement/acceptance decisions go through
  named predicates (props_quad_* family), telemetry from birth;
  no raw comparisons in the refinement loop.

## 3. The demo moment

- demos/tour curvedcut.rs `pin_frontier`: all three retire-on-
  closure panics fire and are retired per their own instructions
  (drop SceneBody::staged, join the standard ladder, K-probe
  sweep join with the Scalar-generic lift its module doc names).
  The tiltedcut stop RENDERS: STL, scene manifest, montage panel
  (the montage refresh deferred at the demo unit lands here).
- The rocker stop's tighter-crop note (demo review NOTE) rides
  along if the FreeCAD lane cooperates; matplotlib fallback
  acceptable.
- A NEW showcase demo stop for shape (ii): boss∪plate rendered —
  the milestone's first curved boolean, visible. Narration at
  demo altitude.
- Exit-criteria anchor: "the die-with-pips fillet demo builds,
  certifies, tessellates watertight, and exports" is PR 12's —
  this PR delivers the tessellation half the die will consume.

## 4. Acceptance

- tiltedcut: both halves tessellate WATERTIGHT (admesh row) at
  δ from the standard schedule; certified props (volume enclosure
  brackets the closed form V = πr²H/2 ± the cut wedge; area
  bound stated honestly); renders + montage panel; the three
  staged pins retired as construction rows; bit-replay at
  1e-6/1e-12 + Interval enclosure rows.
- boss∪plate: tessellates watertight across the curved-curved
  seam arcs (compute-once-per-edge pinned by a shared-chord
  assertion); props enclosure brackets 16+π·0.25·0.6; renders.
- The corpus Band-4 rows extend with tessellation/props columns
  for the curved documents; latency table refreshed.
- T6 tripwire (PERF-PLAN): if CDT dominates the corpus rows,
  say so in the report — the bulk-loading PR dispatches
  M5-adjacent per the plan.
- Multi-ε honesty throughout; every new error arm carries the
  two-tolerance shape INCLUDING definite arms; probe placements
  scale from the resolved band.

## 5. Out of scope

NURBS-face tessellation/props at rest (no such faces exist —
the loft assembly unit, which THIS PR unblocks, brings them);
the loft/sweep assembly itself; fillets (PR 12); STEP (PR 13);
bulk CDT loading (T6's separate PR); HLR. Frontier errors name
the real blocker.

## 6. Process

One implementer + one blinded adversarial reviewer + one fix
pass. Review charter musts: independent quadrature-bound
verification (dense numerical integration vs the certified
enclosure on adversarial faces — bound below truth = automatic
MAJOR); watertightness attack (construct a shared-edge
configuration trying to split chords); a hand re-derivation of
the hull-bounded sagitta generalization; demo-render eyeball;
CODE QUALITY REPORT (fixed rubric). Local runs by the
iteration-speed principle: touched crates default ε + the
Interval rows the change makes meaningful; the tour battery
(NOT in CI) runs locally ×3ε; hosted CI proves the matrix.
Push per unit; foreground verification only; OUTPUT DISCIPLINE
per standing process.
