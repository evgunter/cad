# PHASE 0: waiting for the orchestrator

Lane `splice/crease-window-bound`, item
`work/splice/a-creased-spline-gets-no-second-order-clearing.md`, measured on
main at d1ac6881. (This file is lane scratch; it comes out before the PR.)

## Measurement (premise holds)

Probe rows (not committed; diff kept in the lane's scratchpad):

- **Touch localizer** (`boolean::carrier_touch::clusters` / `off_face`), the
  unit cylinder's quarter sheet, touch at azimuth −1/2 (off the face) and
  7/10 (on it), span (0, 1):
  - degree-1 three-span net (knots 0, 1/3, 2/3, 1; middle span tangent at
    s = 1/2);
  - nine-point rational circle, r = 0.3, touching at its parameter 1/8.

  | ε | net | circle |
  |---|---|---|
  | 1e-9 | one cluster, `off_face` right (true / false) | same |
  | 1e-6 | one cluster, right | same |
  | 1e-12 | `clusters = None` (PIECE_BUDGET), `off_face = Ok(false)`: door stays | same |

  So at 1e-9 and 1e-6 the first-order walk alone still answers; the refusal
  is at 1e-12 only.

- **Pair walk** (`curve_pair::walk::meet`), witness band:
  - degree-1 net (middle span tangent to the unit circle at (0, 1)) × circle;
  - nine-point unit circle touching the line y = −1 at its parameter 1/8.

  Both refuse `MeetFault::Unsettled(Unsettled::Exhausted)` at **all three ε**
  (1e-9, 1e-6, 1e-12), carrying the touch in `found`. Floor is zero
  (curvature infinite), so pieces stop only at the band.

## Why I stopped: the ratified text points the other way

Ev's ruling on [ev] PR 5913 (2026-10-28, Decision 2 "S, then"), now
`docs/DESIGN.md` [topology] (line ~956, "the kernel adds none [cuts], except
where a face or edge carrier is not C¹ by its knot structure"), is carried by
two tracker rows:

- `work/measuredoor/tier-3-refuses-a-carrier-with-an-interior-c0-knot.md`
  (parked on `lathe/the-skin-cuts-an-arc-wall-at-each-sub-arc-joint`, open):
  tier 3 refuses any face chart or edge carrier whose `crease_runs` is
  nonempty.
- `work/tesssize/the-crease-arms-retire-once-no-carrier-carries-a-c0-knot.md`
  (parked on the above): "no reader carries a crease arm ... Each crease arm
  is then unreachable; **delete it, do not generalise it**", and it names
  THIS row: "`a-creased-spline-gets-no-second-order-clearing` ... become[s]
  unreachable. Close [it], naming this row."

The item's two options sit differently against that:

1. **A window-scoped bound** (`accel_upper_bound` on a parameter window,
   max of per-span bounds, infinite iff a crease run lies strictly inside
   the window; `curvature_hi` likewise; each consumer reads its own piece).
   This is not really a crease arm: it is the per-span bound read on the
   span asked for, refusing across a crease exactly as the whole-domain read
   does today. It stays meaningful after tier 3 only if a carrier may still
   hold a crease OUTSIDE its edge's cell (DESIGN says C¹ "strictly inside its
   cell"; the tier-3 row says `crease_runs` empty on the whole carrier —
   those differ for, e.g., a quarter edge on a nine-point-circle carrier).
   Under the strict tier-3 reading it is dead code once tier 3 lands, but
   harmless; it also tightens a smooth spline's bound on a sub-window
   (max over the spans the window touches, not the whole curve).
   Caveat measured in the design: a bisection piece that straddles a
   non-dyadic crease (knots 1/3, 2/3) never gets a finite bound, so a touch
   AT such a crease still walks to the band.
2. **Splitting the walks at the creases first** (seed `bisect` with the
   window cut at its interior creases). This is a crease arm in every
   reader that does it — exactly what the ruling says not to generalise.

## The question

Which do you want?

- (a) Build option 1 only (window bound, no crease splitting in the walks),
  argued as "the bound on the span asked for", which the ruling does not
  retire. Rows: the creased touches at a touch away from the crease, clearing
  at all three ε; the two corner rows stay right; a mutant leaking the bound
  across the crease reds them. **My recommendation.**
- (b) Build 1 + 2 (also cut each walk at the creases), as the item text
  allows, accepting it is a crease arm the tesssize row will delete.
- (c) Park the item on `tier-3-refuses-a-carrier-with-an-interior-c0-knot`
  (the tesssize row already says it becomes unreachable) and close the lane.

Surface side: the analogue exists and is already filed twice —
`work/splice/a-seam-on-a-creased-spline-chart-reads-no-arm.md` and
`work/encl/the-patch-bound-refuses-an-integral-chart-with-a-crease-anywhere.md`
(the rational chart already reads per-span cells with `CreaseWithinReach`;
the integral arm refuses a crease anywhere). The tesssize row lists the
`patch_bound` crease arms for deletion too. One mechanism serving both is
only "`crease_runs` is the one statement of where a crease is"; the bound
objects differ (curve quotient pieces vs. patch cells), so I would cite, not
merge.
