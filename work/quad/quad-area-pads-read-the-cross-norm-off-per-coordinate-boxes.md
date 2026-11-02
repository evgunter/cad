---
id: quad-area-pads-read-the-cross-norm-off-per-coordinate-boxes
kind: issue
title: the props quadrature's area pads and sup_g read ‖cross‖ off a per-coordinate box, so a rigid map moves a certified area bracket
status: open
opened: 2026-10-08
priority: P3
cost: M
---


## Found (ENCL §5 sweep, `encl/offset-cert-coefficient-norms`, 2026-10-08)

`crates/geom-brep/src/props/quad/` reads a vector norm as the root of
the sum of per-coordinate squares of a BOX in several certified places:
`rational_patch_face`'s area cell (`g_hull: (ch[0].sqr() + ch[1].sqr() +
ch[2].sqr()).sqrt() / wh.powi(3)`, with `ch = a.cross_num_at(&w, column, over…)`),
`trimmed_patch_face_rounds`' `sup_g` over the trim box
(`(cross[0].sqr() + …).sqrt()`), the helper `norm_hi`, `area_at` and
`area_cell` (their `‖S_u × S_v‖` and pad readings), and the corresponding
folds in `nurbs_patch_face_rounds`. Two L1 folds sit beside them, `p_bound
= s_hull[0].mag() + s_hull[1].mag() + s_hull[2].mag()` in both
`nurbs_patch_face_rounds` and `trimmed_patch_face_rounds`. (`norm_lo`
and `boundary_chord_perimeter_lo` read the same shape for LOWER bounds,
which D4 ¶2 does not cover.) Grep
`.sqr() + .*.sqr() + .*.sqr()).sqrt()` for the set.
A box of one vector field reads between 1× and √3× its norm depending
on how it sits against the axes, so a rigid map of a part moves its
certified area and volume
brackets' pads by up to that factor. DESIGN.md D4 ¶2 now says a
certified upper bound on a vector-valued quantity is read from the
Euclidean norm of each coefficient, never from a per-coordinate box
folded into a norm (`geom_core::spline::compose::tensor`'s
`coefficient_norm_bound` / `coefficient_norm_sup`, and
`PatchSpans::cell_norm_sup` for a tensor-product cell).

**Inferred from the code, not reproduced.** Whether `cross_num_at` over a
cell is available in coefficient form (it is a hull evaluation here),
and which of these sites are upper bounds the sentence covers rather
than midpoint estimates (`g_mid` is a point reading and is not), is
the taker's.

## Measured on main 258dc6c7c2 (QUAD, 2026-10-21)

**Verdict: stands, measured.** A rigid rotation moves the certified
area bracket's width by up to 1.13× on this face. The flux enclosure
moves too.

Probe: `crates/sweep/tests/quad_frame_remeasure.rs`
`row5_rational_wall_rotated`. The face is the rational cubic × linear
wall `rational_wall` from
`quad-rational-patch-cross-grows-with-translation` (weights 1.8 / 0.7),
read whole by `nurbs_patch_face::<f64>` at the default ε. The probe
first carries the wall's centre (0.525, 0.15, 0.4) to the origin
(`ROW5_CENTRED=1`), so that a rotation changes no point's distance
from the flux origin, and then rotates it about three axes through
that centre:

| rotation | about z | about x | about (1,2,3)/‖·‖ |
|---|---|---|---|
| 0° | 0.1045 | 0.1045 | 0.1045 |
| 15° | 0.1038 | 0.1098 | 0.1093 |
| 30° | 0.1038 | 0.1142 | 0.1131 |
| 45° | 0.1039 | 0.1159 | 0.1149 |
| 60° | 0.1043 | 0.1142 | 0.1145 |
| 90° | 0.1045 | 0.1045 | 0.1170 |

The table gives the area bracket's width (area ≈ 0.875). Every
bracket holds the same area: the brackets' common part is
[0.8234, 0.9272]. Two features fit this row's mechanism, which reads a
norm off a per-coordinate box. First, a quarter turn about a
coordinate axis gives back the unrotated width exactly, because it
permutes the box's coordinates. Second, the width peaks near 45°.
Over the rotations measured, the spread is 0.1038 to 0.1170, a ratio
of 1.13 (the row's bound is √3). The flux enclosure's width over the
same rotations moves from 7.3e-7 to 2.06e-6, a 2.8× spread, and its
midpoint is the same to 6 digits.

The area bracket is wide in every frame: about 12 % of the area. It is
the O(h) area rule's own width (the A2 gauge's "wide by design"), and
the rotation moves only a part of it.

Rotated about the uncentred wall's centre instead, so that the wall
sits 0.3 m to 1.2 m from the origin, the same door refuses
`QuadratureBudget` (`width_len` 1.15e-6 to 1.57e-6 against 1.024e-6)
at 30° and beyond about x and about (1,2,3). At 30° about (1,2,3) it
refuses `Escalated { props_quad_converged }` instead. That is this
door's translation dependence
(`quad-rational-patch-cross-grows-with-translation`), and it does not
reach a body through `topo`.

## The rational area cell, 2026-10-22 (QUAD, `quad/rational-quotient-statement`)

`rational_patch_face`'s area hull and pads no longer read a box: they
are the quotient statement's `sup_norm` and `inf_norm`
(`geom_core::spline::cell_measure_quotients`), `max_k ‖a_k‖/b_k` over
each coefficient's own Euclidean norm, per hull block. The site this
row lists first (`g_hull: (ch[0].sqr() + …).sqrt() / wh.powi(3)`) is
gone. The rest of the list stands: `trimmed_patch_face_rounds`' `sup_g`,
`norm_hi` (still read by the integral and trimmed lanes), `area_at`,
`area_cell`, and the `p_bound` L1 folds. The rotation probe was not
re-run.

## Re-checked on main `9267d3ae` (QUAD tracker sweep, 2026-10-27)

Still holds at the sites the 2026-10-22 section left. Their homes after
PR 5815 are:
- `norm_hi`, `crates/geom-brep/src/props/quad/net.rs:764`;
- `area_at` and `area_cell`, `props/quad/trimmed.rs:681` and `:703`;
- `trimmed_patch_face_rounds`' `sup_g`, `trimmed.rs:1543`;
- the integral lane's area cell, `props/quad/patch.rs:288`–`293`;
- the `p_bound` L1 folds, `patch.rs:218` and `trimmed.rs:1534`.

The lower-bound readers are `norm_lo` (`trimmed.rs:343`) and
`boundary_chord_perimeter_lo` (`props/quad/area.rs:92`). The rational
lane's `g_mid` (`props/quad/rational.rs:467`) is a point reading. The
rotation probe was not re-run.
