---
id: the-junction-end-window-refuses-in-a-belt-around-a-neutral-point
kind: issue
title: An edge ending within about 2e-7 to 2e-4 rad of a junction's neutral point refuses JunctionEndUnread (Window): the order's term decides but no window holds its spread against the quartic tail
status: open
opened: 2026-11-23
priority: P1
cost: M
---


Found in review of `heldrecords/elbow-ends` (PR 7463) and measured on its
fix pass, 2026-11-23.

## Measured

**Union level** (`editor-core`
`a_pipe_unions_with_its_elbow::an_edge_ending_near_a_neutral_point_stops_where_its_distance_says`):
the pipe (ring 1 / 0.6, length 2) and its elbow bent away off the End cap,
revolved about the line through `3·(−sin δ, cos δ)` heading `δ`, so the
bend's neutral point on each rim sits `δ` round the rim from the pipe's
seam ruling and the rim's vertex. At `ε = 1e-9`, `K = 10` (the
figures move with the band: the order's in-band stretch scales with
`ε`, and the belt runs from about `200ε` to about `7√ε`, the spread
`√(6ε/δ)` meeting the window `√(3δ)`):

| δ (rad) | the union stops at |
|---|---|
| 0 | the finish (the end's root of order 4, read past) |
| 2e-10 … 5e-9 | `CurvedPierceEscalated { JunctionEndOrder }` (the quadratic term in band) |
| 1e-8 … 1e-7 | `Escalated { Coincidence(Contact, Moot) }` at the glue door, not this door's |
| 2.2e-7 … 2.2e-4 | **`JunctionEndUnread { gap: Window }`**, the belt |
| ≥ 4.6e-4 | the finish |

**Door level** (`geom_brep::curve_face::end_root`, the ruling at
`θ = π/2 − δ` against the bend, span 2): `Order` at `δ = 1e-8`, `Spread`
from `1.8e-8` to `1.5e-4`, a window from `1.8e-4` on.

## The cause

Off the neutral point the bend's relative curvature across the rim is
`κ ≈ δ/3`, so `c₂` decides nonzero once `δ` clears the band, but the
quartic term `≈ s⁴/36` dominates `c₂·s²` past `s ≈ √(3δ)`: the window
halves to about that. The spread the dropped terms can move is
`√(2ε/κ) ≈ √(6ε/δ)`, so it fits the window only where
`√(6ε/δ) < √(3δ)`, `δ ≳ √(2ε)`, about `4.5e-5` at `ε = 1e-9`; with the
window's factor-two halvings and the tail's half-term margin the
measured edge is `1.8e-4`. Between the band and that edge no window holds.

The cause is the door's shape, not a constant: it certifies one term
against the rest (`|Q| ≥ |c_m|/2`). A window that holds the order's term
and the next ones together (the quotient's lower bound over the window
from `c₂` and `c₄` jointly, as a polynomial's positivity rather than one
term's dominance) would close the belt. That is a redesign of the
window's certificate, not a small fix, so it is not done in PR 7463.

## What the user sees, and the recourse

A normal elbow whose bend axis is turned by `1e-5` rad, so that a
neutral point sits that close to where an edge ends, refuses. The
refusal's sentence (`JunctionEndGap::Window`) says what is true: turning
the parts so the edge ends farther from where the two faces curve alike
across the rim (here, `δ ≳ 5e-4`, or exactly onto it) lets it through.

## Done when

The pose above builds past the crossing layer across the belt (it stops
at the finish, PR 3's subject, as `δ = 0` does), with a row at
`δ = 1e-5`.

## 2026-11-24 — Design: the window pivots past a small order term (`heldrecords/belt`)

### The cause, measured

The pose is the pipe's seam ruling `θ = π/2 − δ` against the bend
(`end_root::rows`' geometry). Its composite, read in metres (`c_j/g`, `g` the
form's gradient at the end), at `δ = 1e-5` is

`F(s) ≈ 2e-16 + 5e-16·s − (δ/6)·s² − 2e-14·s³ + s⁴/72`,

with `c₂ = −δ/6` and `c₄ = 1/72`, both measured. The order is 2, and it
decides once `δ/6·L² = 2δ/3` clears the band (`δ ≳ 15ε`). But the quotient
`Q(s) = c₂ + c₄s²` has a root at `s = √(12δ)` (0.01095 measured, against
0.01095 closed form). So **the carrier meets the bend again at
`z = 2 − √(12δ)`**: a genuine simple root of the edge against the
bend's carrier, below the cap plane, off the bend's face. Between the
end and that root `|F|` peaks at `c₂²/(4c₄) = δ²/2`, measured `5.0e-11` at
`δ = 1e-5`.

The door certifies the end's root alone, so its window has to stop
short of `√(12δ)`, and both refusals follow from that:
- **`Spread`** (`curve_face_end_root_spread`): the spread `√(2ε/|c₂|) = √(12ε/δ)`
  only fits inside a window shorter than `√(12δ)` where `δ > ε^½`. Below
  that, the near root lies inside the spread.
- **`Window`** (`curve_face_end_root_window_clear`): the window's far end
  stands at most `δ²/2` off the bend. So it decides only where
  `δ²/2 > Kε`, `δ ≳ √(2Kε) ≈ 4.5√ε`, and the halvings lose some more.

The door-level scan (prototype probe, the door unchanged) refuses
from `δ ≈ 18ε` to `3.2√ε` at `ε = 1e-6`, to `3.2√ε` at `1e-9` and to
`3.2√ε` at `1e-12`: `Spread` throughout. The union-level scan on main
(the pose `neutral_off_the_seam`, `K = 10`) refuses `Window` from the
row's lower edge (`2.2e-7` at `1e-9`) up to `6.3√ε` (1e-6), `8.4√ε`
(1e-9) and `4.7√ε` (1e-12), measured in quarter-decade steps.

### Why a joint certificate over c₂..c₄ is the wrong object

The row proposed one, and Sturm/Descartes counts and sign-definite
factorisations fail for the same reason. Each still certifies the
end's root alone on a window, but **`Q` has a real root at `√(12δ)`**,
so nothing is sign-definite there. Even a perfect joint lower bound
`|s²Q(s)|` holds only until that root, and over `[0, √(12δ)]` the
composite never leaves `δ²/2` of zero. Below `δ ≈ √(2Kε)` no window
holding only the end's root exists at the band, whatever its
certificate. A joint certificate moves the upper edge by a factor of
about 2 and leaves the belt. A root count reads one root on the
window, in band at the same `δ`, and still does not say where it is.

### The certificate: pivot the window's lower bound

The near root lies on the edge's own face's side of the rim, where the
target face is not: it is the pipe's wall below the cap. The junction
already reads exactly that kind of root by its side, for the roots
inside the spread (`junction_end_window`, `junction_end_side`). So the
fix is not to keep the near root out of the window. It is to **let the
spread hold it**:

- The order walk is unchanged: `m` is the least `j ≥ 1` decided
  nonzero, Zeros are stepped past and charged, and an in-band
  coefficient refuses `Order`. `EndWindow::order` stays `m`.
- The window is read **pivoting on `p = m, m+1, …, deg`**, and the
  first pivot whose window holds is taken. On pivot `p`:
  - every coefficient below `p` is a dropped term:
    - `c₀` and the Zeros are charged as today;
    - each `c_j` with `m ≤ j < p` is charged at `|c_j| + ε·g/L^j`, its
      value plus the zero band's width at its power. That is the most a
      coefficient read at its value admits, and it needs no verdict on
      it: the pivot never reads `c_m`'s sign;
  - the tail (`curve_face_end_root_tail`): `Σ_{j>p} |c_j| s_w^{j−p} < |c_p|/2`,
    halved from `L` as today;
  - the spread (`curve_face_end_root_spread`): the least `σ` with
    `s^p·|c_p|/2 > Σ_{j<p} charge_j·s^j` over `[σ, s_w]`. The excess over
    `s^p` is increasing, so the bisection's monotonicity argument is
    unchanged;
  - the far end (`curve_face_end_root_window_clear`), as today.
- **What it certifies**: on `(σ, s_w]`,
  `|F(s)| ≥ s^p(|c_p| − tail) − Σ_{j<p} charge_j s^j > 0`, so every root of
  `[0, s_w]` lies in `[0, σ]`. That is the door's present claim, with
  "the dropped terms" meaning every term below the pivot.
- Pivot `m` is tried first, so **every pose the door reads today reads
  the same window**. A higher pivot is reached only where `m`'s window
  refuses, which means `c_m` is small against its tail.
- No new predicate on the door, and no new refusal kind. A pose no pivot
  holds still refuses `Spread(None)` / `Window(_)` as today.

At `δ = 1e-5`, `ε = 1e-9`, pivot 4 holds on the whole span, with spread
0.0228 m, which holds the near root at 0.0110. The junction's side then
reads it off the bend: the ruling's departure is 1, so the side margin
at 0.0228 is 0.0225.

### The side at the pivot's spread: its reach, enclosed

The pivot grows the spread from about `√(2ε/|c_m|)` to about
`(2ε/|c_p|)^{1/p}`, up to the near root's scale:
- 0.05 m at `ε = 1e-9` (the neutral point's own order-4 spread, which
  main already reads, is 0.0195);
- 0.28 m at `ε = 1e-6` (main reads 0.111).

The side reading rests on two premises that `junction_end_side` takes
silently today.

1. **The target's carrier is one graph over the junction's tangent
   plane `T` within the spread's reach.** Projection along the normal
   preserves the conormal coordinate exactly. So a root on the carrier
   whose conormal offset beats the projected rim's is on the edge's
   side within the carrier only while that projection is injective.
2. **The rim's offset is read at the root's chord, not its arc.** A
   root `s` along the edge has tangent coordinate at most `s`. The rim
   point with that tangent coordinate is at arc
   `u ≤ s(1 + κ_rim²s²/3)` for `κ_rim·s ≤ ½`. So the rim's term is
   `κ_rim·σ²·(1 + κ_rim²σ²/3)²/2`, not `κ_rim·σ²/2`. `g` stays concave,
   so the `σ*` argument stands.

Both are built into this unit:
- **reach**: a new `prove`, `junction_end_reach`:
  `σ ≤ ½·r_min`, where `r_min` is the target's least radius of
  curvature over the reach. That is `geom_brep::min_radius_of_curvature`
  at the end for the kinds with uniform arms: radius for a cylinder or
  sphere, `min(r, R − r)` for a torus. A cone's parallel radius is read
  less `σ`. Short of proven, it refuses `JunctionEndUnread { Window }`.
  For `σ·k ≤ ½`, no other sheet of a plane, cylinder, sphere, torus
  or cone's nappe comes within `σ` of the end.
- **chord**: the rim's term carries the factor above.

The belt passes both: `σ·k ≤ 0.28` at `ε = 1e-6`, and the factor is at
most 1.03 there. **Not enclosed, and filed rather than built**: the
target face's own other boundary inside the reach, meaning an edge of
the target face (not the rim through the end) passing within `σ` of the
end. Today's side reading assumes none at any spread. It is a premise of
the merged reading, not one the pivot adds, so it gets its own row.

### Measured with the prototype (the pivot, uncommitted)

**Door**: the ruling at `θ = π/2 − δ`, span 2, at `ε` = 1e-6, 1e-9 and
1e-12 (`K = 10`). Every `δ` whose order decides now reads a window,
from about `18ε` up, where today it refuses `Spread`. Only `Order` in
band remains, from about 2ε to 10ε. The spread is:

| ε | at δ = 30ε | at δ = 3√ε | where pivot `m` resumes |
|---|---|---|---|
| 1e-6 | 0.112 | 0.279 | `5.6√ε` |
| 1e-9 | 0.0196 | 0.0496 | `5.6√ε` |
| 1e-12 | 0.00347 | 0.00882 | `5.6√ε` |

**Union** (`neutral_off_the_seam`, `δ` from `0.1ε` to `1e7ε`, half-decade
steps, and quarter-steps over `2…60√ε`):
- at every ε, `δ = 0` and `δ ≤ 0.1ε` build;
- from `0.3ε` to `10ε` the union refuses `CurvedPierceEscalated
  { JunctionEndOrder }`;
- from `32ε` to `20√ε` and beyond it builds, at the closed form (relative
  volume error ≤ 4e-16), with tier 3 and tier 3′ clean.

The one exception is a strip at `ε = 1e-12` below.

### What it leaves refusing, and why

- **`JunctionEndOrder` in band, `δ ≲ 10ε`.** `c₂`'s separation over the
  span is in band. It refuses typed, as the brief requires. *Option F1,
  not taken*: since the pivot never reads `c_m`'s sign, an in-band
  coefficient could be charged at the escalate width and pivoted past,
  so the order is moot to the window. That closes this refusal too, but
  it changes what the walk decides (README's "one in band refused") and
  `EndWindow::order` would lose its meaning, so it is a fork for the
  orchestrator.
- **At `ε = 1e-12` only: `CurvedPierceUnsupported` from `6.3√ε` to
  `15√ε`, on main as well.** It lies above the belt: pivot `m` holds, the
  window stops before the near root at `√(12δ)`, and the rest of the
  span's Line × Torus root door refuses that simple root even though
  `δ²/2 = 30…100·Kε`. That is a root door's reach, not this door's.
  *Option F2, measured, not recommended*: try the pivots top-down
  (highest first), under the reach guard. That closes the strip
  (prototype: every `δ` from `2√ε` to `60√ε` builds at `ε = 1e-12`). But
  for every pose with `δ` up to about `1e-2`, it hands the near root to
  the side instead of the trim, at spreads near the reach's limit.
  Recommended: bottom-up as above, and the strip filed as its own row
  against the root door, with this pose.
- **A spread past the reach** refuses `Window`: a cone near its apex,
  or a small tube. That is honest: the side reading is not local
  there.

### Pins

- **Door** (`end_root::rows`, all three ε in one test via
  `Band::linear_at`, so the scan runs at every eps row): `δ` in band units
  `{30ε, 300ε, 0.1√ε, √ε, 3√ε, 10√ε}`. Each reads a window. Below
  `5.6√ε` the window is the whole span and its spread contains the
  closed-form near root `√(12δ)` (pivot 4); above it the window stops
  short of `√(12δ)` (pivot 2). Neither needs the pivot exposed. Plus `δ = 5ε`, which refuses
  `Order`.
- **The spread meets the charge at the pivot**: the existing row's
  closed form, extended to pivot 4 at `δ = 1e-5`, with `c₂` charged at
  `|c₂| + ε·g/L²`.
- The two door rows the belt pinned flip:
  - `a_tiny_decided_quadratic_term_refuses_on_its_spread` reads a
    pivot-4 window;
  - `a_window_whose_far_end_reads_in_band_is_not_returned` reads one at
    `δ = 1e-4`.
- **Union** (`a_pipe_unions_with_its_elbow`):
  - `an_edge_ending_near_a_neutral_point_stops_where_its_distance_says`
    builds at `√ε/3` (the row's `δ = 1e-5` at `ε = 1e-9`), at the closed
    form, with t3 and t3′;
  - a band-unit row `{30ε, √ε/3, 3√ε}` that builds;
  - `5ε` still refuses `JunctionEndOrder`.
  - Union rows cost about 1 s each, so they go in the slow set.
- **Reach**: a door/side row where the pivot's spread passes `½·r_min`
  (a thin tube, `r = 0.05`, with the same bend) refuses `Window` on
  reach.

### Mutants that must turn a pin red

1. Pivot only at `m`, as today: every belt row goes red.
2. Pivoted-past terms not charged, `charge_j = 0` for `m ≤ j < p`: the
   spread shrinks below `√(12δ)`, so the near-root containment row and
   the charge row go red.
3. Charged at `|c_j|` without the band's width: the charge row goes red.
4. The lead read from `c_m` at pivot `p`: the window refuses, and the
   belt rows go red.
5. Pivots top-down (F2): the `δ = 10√ε` row's "window short of the
   near root" goes red.
6. The reach guard dropped: the thin-tube row goes red.
7. The rim's chord factor dropped: a side-margin row at `σ·κ_rim = ½`,
   with the margin's closed form, goes red.

### What moves

- `crates/topo/README.md`'s end-root paragraph: "a window … that holds
  no root but those the dropped terms can move" gains the pivot (the
  terms below it). The side paragraph gains the reach and the chord.
  Both are agent-written (PR 7463, `368418f6`), not ratified, and
  reworded with the change.
- `end_root`'s module docs: the window and spread bullets.
- No signature changes; `JunctionEndUnread { Window }` gains the reach
  as one more cause in its doc.
