# EXCH — exchange: STEP and STL (plan)

**STATUS: OPEN (2026-09-03).** Opened 2026-09-03 from `docs/WORK-TRACKS-2026-09.md` (EXCH section), which is this
program's charter until this plan supersedes it. Live state is
`work/exch/log.md`'s tail and the item files beside this plan, never
this file.

Branch prefix (the #396 convention): **`exch/`** — unit branches
`exch/<unit>-<slug>`, orchestrator branch `exch/orchestrator`.
Away-channel tag `(EXCH orchestrator)`. A/B ordinal band
**EXCH = 2100–2199**, claimed in `docs/MODEL-AB-LOG.md`'s banding
entry in the opening commit, per that entry's rule.

## Charter

Make import recognise what it can certify and make export say what the
caller asked. The recognition work is certified-interval reasoning in
`spline::compose` and the pcurve derivations; the option surface is
three small API decisions Ev signs off.

## Review posture

Full v6 dual with Fable specs for the H units; the option-surface
items are `[ev]` PRs then single-review E builds.

## Unit order

H, in dependency order:

1. `step-import-degree-one-line-promotion` — the `ExtrudedPoint` /
   `PlacedSegment`-over-`Line` rung in `nurbs_iso_derive` (TRIM's file;
   filed there, built by whichever is dispatched first), then promote
   certified degree-1 carriers to `Curve3::Line`; the certificate
   exists.
2. `step-import-curve-recognition-named-exclusions` — a derivative
   channel in `spline::compose` so the turning witness becomes a
   certificate (rung 2 before rung 1), then open arcs, the
   general-quadric arm for ellipse, the helix implicit form; L.
3. `rational-patch-flux-quadrature-budget` route 2 — an algebraic
   cylinder-recognition certificate via exact spline-product hulls
   (`spline::compose::tensor`) so M7-6 promotes rational walls to an
   analytic `Cylinder`; issue 1195 is the second beneficiary; S-CERT's
   Q4 ruling records the route as unclaimed; L. PROPS' dial decision is
   the cheaper alternative for the dm1 flip — take the dial's answer
   first.

D→E, the option surface:

4. `stl-header-refuses-plausible-names` — smallest; probably "fix the
   demos to carry a fallback" and keep the wide sniff.
5. `step-writer-hardcodes-user-header-fields` (`C14`) — which of
   `authorisation` and the `FILE_DESCRIPTION` list are caller-settable.
6. `epsilon-has-no-type-of-its-own` (`C13`; sibling of the `D283`
   ruling) — where an ε-alone type lives and whether `Tolerance::eps`
   becomes it; the three hand-copied finite-positive checks follow.

E: `D343` (typed payloads through `{:?}` in the two STEP crates, with
its two riders) and the step-import diagnostics half of FIX's
`coherence-findings-have-no-consumer`.

## Exit shape

The three H units land, the option surface is ruled and built, Track
U's STEP/STL rows are empty; the walk convention applies.
