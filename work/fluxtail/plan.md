# FLUXTAIL — the plan

The curved arms' numeric honesty: rim levels, chord lengths and span folds that cancel, drop a refusal or depend on the frame.

Opened 2026-10-10 by FLUX's priority-seam cut (`work/README.md`,
Track size), when FLUX measured 125.5 budget points against 30. FLUX
kept the curved closed-form arms. Nothing dispatched.

## The slate

**22.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | status | title |
|---|---|---|---|---|
| P3 | `a-cylinder-rims-level-is-recovered-by-a-dot-product-that-loses-a-short-faces-height` | E | open | A cylinder face's rim levels are recovered as (centre − origin)·axis, which on a tilted frame far from its origin costs a short face up to 1.8e-8 of its area |
| P3 | `a-loop-area-nurbs-segment-integrates-an-inverted-window-as-zero` | E | open | props loop_area: a NURBS boundary piece with t0 > t1 integrates to zero per span (half_len.max(0)) where the signed integral is negative |
| P3 | `a-planar-face-sums-its-area-about-a-far-carrier-origin` | M | open | A planar face's closed form sums its vector area about the carrier plane's stored origin, so a small face far from that origin loses its area to cancellation |
| P3 | `an-edges-extent-has-four-dispatchers` | M | open | an edge's extent has four per-Curve3 dispatchers and no home, so each per-site fold of a face's extent picks a different one |
| P3 | `piece-monotone-span-drops-a-refused-step-through-f64-min` | E | open | props/quad.rs piece_monotone: the span fold's f64::min drops a refused step's NaN, so the span is the other steps' minimum |
| P3 | `props-rim-incidence-decides-axis-and-centre-one-at-a-time` | E | open | props' require_rim_incidence decides the rim's axis and centre one at a time, not their sum |
| P3 | `props-sphere-side-in-arc-is-levered-by-arc-length-not-the-roots-slope` | M | open | props: the sphere side's in-arc reading is levered by arc length, so near a graze a root within its own error of a span end is read decided |
| P3 | `sphere-rim-level-cosine-cancels-near-a-pole` | M | open | A sphere rim level's cosine is recovered as √(1 − s²) and cancels near a pole |
| P3 | `the-gating-corpus-reaches-no-collapsed-arm-gate` | M | open | The gating corpus reaches no collapsed-arm gate: five of the eight predicates have zero rows, three have only positive |
| P3 | `trim-piece-monotone-span-fold-drops-a-refused-window` | E | open | props_trim_piece_monotone folds its span with f64::min, so a refused chord window drops out of the margin instead of escalating it |
| P3 | `trim-walk-chord-lengths-are-rooted-to-nearest` | M | open | The trim walk's chord lengths are f64 roots to nearest, read as a pad, a refusal threshold and a direction |
| P4 | `props-reads-vector-norm-bounds-off-per-coordinate-hulls` | M | open | props: the area pads and the curve second-derivative bound read a vector norm off per-coordinate hulls, so they depend on the frame |

## Order

The three `E` rows whose fix is written (`a-cylinder-rims-level…`,
`piece-monotone-span…`, `trim-piece-monotone-span…`) are one PR's
worth and a good opening. `a-planar-face-sums-its-area-about-a-far-carrier-origin`
is the row with a measured witness behind it. `a-loop-area-nurbs-segment…`
starts by establishing whether any caller can hand it an inverted
pair. `the-gating-corpus-reaches-no-collapsed-arm-gate` is test work
and can run beside anything. FLUX's lanes edit `props/curved.rs` at
the same time; check `work/flux/log.md` before dispatching on it.

## Review posture

FLUX's, inherited: the review tiers of `memories/orchestration-model.md`
(orchestrator's read, single style or full review, or a dual under
`docs/DUAL-REVIEW-PROTOCOL.md`, logged in `docs/DUAL-REVIEW-LOG.md`),
named with its reason at dispatch. FLUX's posture holds here too: a
spec requires a decide where it would otherwise assert a premise, and a
narrower bound is not evidence a width fix is right; containment,
tested against exact arithmetic, is.

## Exit

The slate is closed or re-homed. This plan sets no `## Exit criteria`,
so the program closes without an exit walk.
