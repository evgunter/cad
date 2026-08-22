# VERBS-RIM — a sound lever arm for closed rims (#554)

Unit 1 of `docs/VERBS-PLAN.md` Wave 1. Closes **#554**. Branch
`verbs/rim`, PR to main. Difficulty logged pre-draw: **M**.

## The defect (verified on main, 2026-08-21)

`extent_of` (`crates/sweep/src/fillet/battery.rs:198`) meters a
link's lever arm as the endpoint chord
`(carrier.eval(t1) − carrier.eval(t0)).norm()`. A closed rim's start
and end vertex coincide, so the chord is ~0, every angular predicate
folds against a collapsed lever, and `convexity_at` decides `Zero` →
`FilletError::TangentialEdge` — "the supports share a tangent plane"
— on supports meeting at 30°. **The verdict is false, not merely
unhelpful**: the same profile revolved partially (chord 0.27 m)
refuses `SpineUnsupported`, the honest answer. Consequence: no full
solid of revolution can be filleted at all, and the refusal
misdescribes why. Live probes: `klein::wall_probes` walls 1–2
(`demos/tour/src/klein.rs`).

## The design ruling (this spec's call — the issue asked for one)

**The lever arm becomes the diameter of a deterministic parameter
sample of the edge: the maximum pairwise chord over samples at
minimum `{t0, (t0+t1)/2, t1}`.** Properties that decide it:

- **Sound**: every chord lower-bounds arc length, so the lever never
  over-reports extent; margins in meters stay conservative in the
  direction K-margin discipline needs.
- **Exact reduction on the straight case**: for a collinear carrier
  the endpoint chord dominates both half-chords, so open straight
  edges meter bit-identically to today (this chooses among equivalent
  implementations, per the output-stability rule — it is not the
  justification for the change).
- **Honest on closed rims**: a full circular rim meters ~its
  diameter; the false `TangentialEdge` becomes today's honest
  `SpineUnsupported` until VERBS-ARMS lands the curved arms.
- **D9-clean**: fixed sample schedule, no adaptivity, no new metered
  predicate name.

Alternatives considered and declined: arc length (needs quadrature
machinery for a lever whose only job is a scale), a "half-period
chord" special case for closed edges (a closed-case bolt-on, exactly
what #554 says not to do). The implementer may EXTEND the sample set
(deterministically) if a 3-sample degeneracy is demonstrable on a
reachable carrier; say so in the PR body.

## Scope

1. Replace `extent_of`'s functional per the ruling.
2. **Enumerate and re-examine every consumer of the lever** — the
   issue's own widening: `convexity_at`'s levered margin,
   `chain_g1`'s min-of-extents arm, `Link::arm_len` and anything
   folding against it downstream (surgery/clearance included). The
   PR body carries the enumeration; a consumer whose semantics the
   new lever changes gets its own line and, where behavior shifts, a
   pin.
3. **Reword `TangentialEdge`'s doc comment and refusal prose**
   (battery.rs:343 region): stop asserting "the supports share a
   tangent plane" as an established geometric fact — a decided
   `Zero` establishes "no definite wedge side at this lever";
   genuine tangency is one cause. Sweep other sites restating the
   claim (the issue names the doc comment; grep for the sentence,
   not just the symbol).
4. Update the affected probes/demos honestly: `klein::wall_probes`
   wall 1 currently RECORDS the false verdict — it flips to record
   the honest refusal (a probe re-baseline; say what moved and why).

## Fences

- **No new fillet arms** (VERBS-ARMS's unit) and no corner-code
  changes (#644 is deliberately untouched here).
- **No signature tightening** — #883 is parked; stay on
  `T: Decide + Bounds`.
- **No `MappedCurve` reach** to make closed-rim parameterisation
  convenient (#554 thread; the prefer-intrinsic obligation lives on
  `attach_contact`).
- Nothing else from the smell scan or the register.
- Cite design documents precisely — #554's thread flags a three-way
  D7 collision (CURVED-DESIGN §D7 ≠ DESIGN D7 ≠ smell-scan D7); name
  the document with the number.

## Acceptance

- **The #554 pair goes honest**: a full-revolve latitude rim at a
  ~30° dihedral refuses `SpineUnsupported` (not `TangentialEdge`),
  while the partial revolve of the same profile keeps its current
  refusal — pinned as a back-to-back pair (the Klein probes are the
  template; the pin lives in a test, not only the demo).
- **Genuine tangency still detected**: the lily's wall-6 shape — a
  co-surface seam meridian whose dihedral sine is exactly 0 — still
  refuses `TangentialEdge` with the new (nonzero) lever. This is the
  differential row proving the fix removed the false positive, not
  the detector.
- **Convexity classification on closed rims**: at least one closed
  rim each deciding Convex and Concave correctly (a concave fixture
  may need authoring — #644 notes only `cube(l)` exists; keep it
  minimal, it is a fixture not the corner work).
- Existing fillet suites green on the hosted matrix (the record);
  plane–plane paths expected bit-identical per the ruling's
  reduction property.
- If `fillet3_*` k-lint baselines fire, follow the K-REPORT runbook
  — the lever change legitimately moves margin distributions; do not
  tune geometry to silence it.

## Lane obligations

Read `docs/prompts/implementer-discipline.md` in full before
starting — output discipline, CI-first verification, target dirs,
comment style. Merge `origin/main` immediately before opening the
PR and re-merge whenever main moves. **No Co-Authored-By trailer in
lane commits** (review blinding overrides the harness convention; if
one lands in a pushed commit, note it in the PR body and carry on —
never rewrite history). PR/issue body drafts go to lane-private
paths (`~/.local/share/cad-work/verbs-rim-*.md`), never the session
scratchpad. Push after every coherent unit.
