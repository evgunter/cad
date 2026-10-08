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

`crates/geom-brep/src/props/quad.rs` reads a vector norm as the root of
the sum of per-coordinate squares of a BOX in several certified places:
the rational lane's area cell (`g_hull: (ch[0].sqr() + ch[1].sqr() +
ch[2].sqr()).sqrt() / wh.powi(3)`, with `ch = a.cross_num(&w, over…)`),
`sup_g` over the trim box (`(cross[0].sqr() + …).sqrt()`), and the
helpers around lines 2103, 4126, 4474, 4511 and 4944 (names drift; grep
`.sqr() + .*.sqr() + .*.sqr()).sqrt()`), and one L1 fold beside them,
`p_bound = s_hull[0].mag() + s_hull[1].mag() + s_hull[2].mag()`.
A box of one vector field reads between 1× and √3× its norm depending on how it sits against the
axes, so a rigid map of a part moves its certified area and volume
brackets' pads by up to that factor. DESIGN.md D4 ¶2 now says a
certified upper bound on a vector-valued quantity is read from the
Euclidean norm of each coefficient, never from a per-coordinate box
folded into a norm (`geom_core::spline::compose::tensor`'s
`coefficient_norm_bound` / `coefficient_norm_sup`, and
`PatchSpans::cell_norm_sup` for a tensor-product cell).

**Inferred from the code, not reproduced.** Whether `cross_num` over a
cell is available in coefficient form (it is a hull evaluation here),
and which of these sites are upper bounds the sentence covers rather
than midpoint estimates (`g_mid` is a point reading and is not), is
the taker's.
