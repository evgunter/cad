---
id: a-quadrature-faces-pad-scales-with-its-shells-size-and-stored-face-order
kind: issue
title: A quadrature face's pad scales with its shell's size and stored face order
status: closed
closed: 2026-10-20
priority: P2
cost: M
refs: [spiric-face-volume-budget-depends-on-world-position]
opened: 2026-10-19
---

(Filed by the unit that fixed
`work/flux/spiric-face-volume-budget-depends-on-world-position.md`, PR
5442, and raised to P2 on its review.)

## What

Every face's flux is read about its shell's reference, a vertex of the
shell (`crates/topo/src/props.rs`, `shell_reference`): the start of the
outer loop of the shell's first face, in its stored order. A quadrature
face's width scales with its lever from that point. So a face's pad, set
against main's (where every face was read about the world origin),
scales by about

    diameter(shell) / (that face's own distance from the world origin)

That ratio is unbounded. It is not "up to 2× on a body centred on the
origin", which is what the unit first claimed. It also depends on which
vertex is first, so one solid stored in two face orders carries
different pads.

Measured by the PR 5442 review (`review/5442-01DwbEsp`,
`crates/sweep/tests/review_5442_probes.rs`), head against merge base:

| ε 1e-9 | pad ratio, head / merge base |
|---|---|
| torus + 12 m bar, `P ∪ T` | 6.3× |
| the `x = 1.1` cut, `P ∖ T` | 9.1× |
| torus + 900 m bar | 13.7× |
| one solid in two orders (`T ∪ P` vs `P ∪ T`, 900 m bar) | 4.10e-6 vs 5.88e-5 (14×) |

At 1e-12 the ratios are at most 2.6× in the probes. Nothing refused.
The row
`crates/sweep/tests/a_spiric_cut_far_off_and_turned_reads_as_at_home.rs`
`one_solid_in_two_face_orders_carries_pads_apart_as_known` pins the
order dependence as known, measured at 14.3× (1e-9), 15.9× (1e-12) and
911× (1e-6). A fix here turns it red on purpose: then assert the two
pads within 2×.

Pad ceilings re-measured under the vertex rule. The values stand, and
their sites state this item as the cause:

| what | about the origin | about the shell's vertex |
|---|---|---|
| `m8_3_rational_volume` arc prism volume pad (1e-9) | 0.98e-6 | 2.127e-6 |
| `cert5_offgrid_knot_rational` blade-8 volume pad (1e-9) | under 2.048e-6 | 2.127e-6 |
| `m5_s11_concave_sense` lofted concave prism (1e-6) | under 2.048e-3 | 2.07e-3 |
| step-export `loft_prism` pad (mm³) | 1.0436e-5 | 1.0695e-5 |
| step-export `nonuniform_loft` pad (mm³) | 2.0428e-5 | 2.7534e-5 |
| step-export `swept_elbow` pad (mm³) | 1.1657e-5 | 1.6727e-5 |

## Options to weigh

Both are still to be designed. Neither is chosen here.

1. **The shell's box centre as `c`.** It is order-free, and its lever is
   about half the diameter for every face. The catch is `corner_of`'s
   docs (`crates/topo/src/props.rs`): a mean point is not a point of the
   body, and its coordinates are not the body's own. Moving a disc's wall
   by one read the wall's height, exactly `Kε`, across the band's edge in
   the re-derivation's closed-form decisions. Its lever is still the
   shell's size, not the face's.
2. **A `c` per face, with an exact carry.** Read each face about a point
   of its own (one of its vertices), so its lever is its own size. Then
   carry each term to one point per shell, exactly or at a stated width,
   before the sum: the terms of a shell must be summed about one point.
   The carry is `(c_f − c)·A⃗_f`, at `A⃗_f`'s width times the lever, so it
   moves the width back unless `A⃗_f` is exact or narrow. Measure that
   first.

Related, worth one home: "read a closed-form face about a point" is
spelled twice, `closed_form_about` (`crates/topo/src/props.rs`, at the
walk's scalar) and `quad_lane::closed_form` (at the interval scalar).
Whichever option lands should give it one.

## Done when

An option is measured against the table above and lands with its rows,
and the order row is restated to assert the two pads within 2×. Or this
item is closed with the measurement that says neither helps.

## Closed (2026-10-20, branch `flux/pad-scale-face-order`)

Measured both options and five candidates as local spikes, pads at
ε 1e-9 (1e-12, 1e-6 in the PR body):

| body | vertex (5442) | (1) shell box centre | (2) own vertex, no carry | qfirst (landed) |
|---|---|---|---|---|
| home `P ∪ T` / `T ∪ P` | 2.58e-5 / 4.10e-6 | 1.29e-5 / 1.29e-5 | 4.10e-6 | 4.10e-6 / 4.10e-6 |
| 900 m bar `P ∪ T` / `T ∪ P` | 5.88e-5 / 4.10e-6 | 2.93e-5 / 2.93e-5 | 4.27e-6 | 4.27e-6 / 4.10e-6 |
| `x = 1.1` `P ∖ T` | 4.04e-5 | 2.07e-5 | 1.02e-6 | 1.02e-6 |
| s = 100 turned `P ∪ T` / `T ∪ P` | 2.58e-5 / 2.48e-5 | 1.37e-5 / 1.37e-5 | 2.77e-5 | 2.23e-5 / 2.48e-5 |
| `m8_3` arc prism | 2.13e-6 | 1.51e-6 | 1.90e-6 | 2.00e-6 |
| `m5_s11` lofted concave | 2.02e-6 | 1.58e-6 | 1.90e-6 | 1.35e-6 |

- **(2) is not a fix.** A face read about its own vertex converges at
  an early round with a wide `A⃗`. The carry `(c − c_f)·A⃗_f` multiplies
  that width by the full lever, so it puts the width back unless `A⃗`
  is refined to target/lever, which costs as much as reading about `c`.
  A pad is not linear in the lever (each face refines to its target:
  one face read 7.46e-5 at a 12 m lever and 1.90e-5 at 100 m), so the
  carry cannot be measured by finite differences.
- **(1), and every mean point, loses.** That is the shell's box centre,
  the box centre of its quadrature faces, and of its curved quadrature
  faces. A point off the body's coordinate grid rounds every moved
  coordinate, and over a long face that rounding is width: the curved
  faces' box centre sat by the torus and still read 5.9e-8 on the 900 m
  bar at 1e-12, against 3.8e-9 about a vertex. The mean point also cost
  `fillet_h5_r2_probes` its tolerance.
- **qfirst landed.** `c` is the start vertex of the shell's first face
  that takes the quadrature (`takes_quadrature`, the walk's own
  dispatch). It is exact and needs no decision. Only a quadrature face
  carries a pad at the walk's scalar, so it reaches the per-face lower
  bound on every probe above except the turned pose. "Nearest vertex to
  the quadrature box centre" read about the same, but needs recorded
  comparisons in kernel code.
- **qfirst is still order-dependent, and the dependence is unbounded.**
  Which quadrature face is first is the stored order's choice. On the
  bodies above it moves the pad 1.04× (the 900 m bar) and 1.10–1.55×
  (turned), pinned per ε by
  `one_solid_in_two_face_orders_carries_nearly_one_pad`. On one shell
  whose quadrature faces lie a kilometre apart (two tori on one bar), it
  moves the pad 2.58× at 1e-9, 2.71× at 1e-12 and 90.2× at 1e-6 across
  rotations of the stored face list, the same as the vertex rule. That is
  filed as P2
  `work/flux/a-shells-pad-still-depends-on-which-quadrature-face-it-stores-first.md`.
- **Cost that stays:** a quadrature body centred on the origin is read
  about a vertex on its rim, where the origin was its centre. `m8_3`
  reads 2.00e-6 against 0.98e-6 on main, and its ceiling states the
  measure.
