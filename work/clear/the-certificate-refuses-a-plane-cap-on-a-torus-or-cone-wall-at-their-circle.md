---
id: the-certificate-refuses-a-plane-cap-on-a-torus-or-cone-wall-at-their-circle
kind: issue
title: The injectivity check refuses Uncertified, departure arm at the depth floor, where a plane cap meets a torus or cone wall at their shared circle: seven finished tour bodies would not finish at the at-rest gate
status: closed
opened: 2026-11-12
closed: 2026-11-12
priority: P0
cost: M
branch: clear/plane-cap-circle
---


## Finding

Measured by CLEAR's lane clear-gate (`the-at-rest-gate-certifies-embedding`,
its Measured section). Main at `348ec09727`; the body-level question
the gate would ask (`StrictlyPositive`, one body against itself,
`DEFAULT_MAX_CELL_DEPTH`, 65,536 pairs), at `f64`, on bodies the tour
builds and tier 3 passes.

Seven bodies refuse `EngineRefusal::Uncertified { arm: Departure }` at
depth 40, each on a plane face against a curved wall across their
shared circle:

| body | pair | refuses after | pairs |
|---|---|---:|---:|
| `lily_stem`, `lily_arch`, `lily_pedicel` | plane × torus, at a `Circle` | 0.27–0.62 s | 1,718–3,462 |
| `tube_along_arc` | plane × torus, at a `Circle` | 0.20 s | 1,028 |
| `teapothandle` | plane × torus, at a `Circle` | 0.27 s | 1,598 |
| `hollowelbow` | plane × torus, at a `Circle` | 1.23 s | 18,798 |
| `lily_lantern` | plane × cone, at a `Circle` | 1.79 s | 23,283 |

The circle is a chart iso of the torus (a meridian) and of the cone (a
parallel), and the pair still splits to the floor, so the obstacle is
local to some cells along it, not the edge's presentation at the root.
Where along the circle was not traced. A candidate is where the circle
meets the wall's seam line (a corner of a curved edge and a line):
`departure-reads-no-curved-corner-off-its-shared-edge`.

## Why P0

`the-at-rest-gate-certifies-embedding` (P0) cannot land its gate while
these bodies refuse: the gate would refuse them at their doors.

## Fix shape

Trace one refusing cell (`tube_along_arc` is the smallest), then fix the
reading it lacks; the six other bodies are the rows that prove it.

## Trace (CLEAR, lane clear-planecap, 2026-11-12)

`tube_along_arc`, the same question at `f64` (`R = 2`, `r = 1/2`,
the spine from `1/4` to `7/4` rad). Its faces are two plane caps and
two half-tube torus walls, which share the outer and inner equators
as seams. Each cap is a disc bounded by two semicircles. The
semicircles are the two walls' meridians, and they meet where the
seams land on the cap.

- **The refusing pair** is cap `1v1` against wall `3v1`, across the
  semicircle `5v1`. All 16 refused leaves sit at the cap's two
  vertices, `3v1` on the outer equator and `4v1` on the inner, where
  `5v1` meets the cap's other semicircle `6v1`. Cells off the
  vertices certify through the graph separator.
- **The sub-windows at the floor.** The cap cell is
  `u ∈ [0.49609, 0.5]`, `v ∈ [0, 0.00195]` (chart metres, holding
  the vertex). The wall cell is `u ∈ [1.49947, 1.50023]`, across the
  meridian `u = 1.5`, by `v ∈ [6.27705, 2π]`, ending on the seam.
- **The cap is not presented.** `present_plane` goes to
  `curved_walls`, and `beside_curve` refuses it `Structural`: the
  cell holds a corner of two curved edges, and `reading` needs the
  other edge straight. This is the second bullet of
  `departure-reads-no-curved-corner-off-its-shared-edge`.
- **The wall is presented** with `Zero::Side(XHi)` on the meridian.
  So the one arm that can decide the pair is the carrier arm: the
  cap's plane, on which every point of the cap lies, against the
  wall.
- **What the arm lacked: the plane's value along the wall's meridian
  side.** `zero_on` read it by the centred form, which gave
  `±1.65e-6` m at the floor, against a definite derivative across the
  side (`1.3e-3`–`2.0e-3` m). The centred form multiplies an interval
  `S_v` over the side by the side's half-length. `S_v`'s components
  vary along the side, and the interval dot product with the plane's
  normal does not cancel them, so the error falls only with the
  side's length squared. Within 40 splits shared over four axes, the
  wall's side stayed about 2⁻⁹ of a turn long, and the zero was never
  decided.

So the corner the candidate row names is present, but it is not this
row's cause: the carrier arm needs no presentation of the cap. That
row is left open, with this trace added to it.

## Fix

`on_side` reads `ψ` along an iso side of an analytic chart whose
image is a circle (`iso_circle`) by the circle's closed form,
`Trig::psi`. These are a torus's meridians and parallels, a sphere's,
and a cylinder's or a cone's parallels. `Trig::psi` is the form a
`Zero::Curve` side already reads. It forms the value's trigonometric
coefficients before any parameter is read, so a circle lying on
`ψ`'s zero set reads rounding at any length. It is a rigorous
enclosure of the same quantity, decided at the same funnel site
(`self_intersection_on_shared`) in the same band. A ruling (a line)
and a cone whose lever is not certified positive keep the centred
form.

## Closed

All seven certify, with the gate's question and harness as in
`the-at-rest-gate-certifies-embedding`'s Measured section (release
profile, one core, `f64`):

| body | before | after | pairs | deepest |
|---|---|---:|---:|---:|
| `lily_stem` | refused | 0.22 s | 1,288 | 24 |
| `lily_arch` | refused | 0.57 s | 2,660 | 20 |
| `lily_pedicel` | refused | 0.20 s | 1,040 | 19 |
| `tube_along_arc` | refused, 0.20 s, 1,028 pairs | 0.06 s | 224 | 16 |
| `teapothandle` | refused | 0.11 s | 521 | 14 |
| `hollowelbow` | refused | 0.70 s | 15,334 | 27 |
| `lily_lantern` (plane × cone) | refused | 0.28 s | 6,063 | 27 |

The whole tour walk (85 bodies) now reads 66 holding (was 59), and
16 engine refusals plus the 3 assemblies at their recorded contacts.
The 16 are the other rows' bodies, unchanged: `chute` and the three
buds, `diefillet` and the three heatsinks
(`departure-reads-no-curved-corner-off-its-shared-edge`), and the
eight that run out of pairs
(`the-certificate-runs-out-of-pairs-on-eight-finished-tour-bodies`).
No body that held before moved. The walk's certificate total is
91.7 s against tier 3's 31.4 s.

Pinned by
`a_windowed_tubes_caps_hold_against_its_walls_at_both_scalars`
(`crates/sweep/tests/self_intersection_proves_injectivity.rs`) and
`a_side_on_a_meridian_reads_a_plane_through_it_off_its_closed_form`
(`topo::injective`'s presentation rows).
