---
id: a-quadrature-faces-pad-scales-with-its-shells-size-and-stored-face-order
kind: issue
title: A quadrature face's pad scales with its shell's size and stored face order
status: open
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
