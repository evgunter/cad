---
id: the-cylinder-certificate-reads-a-ruling-sided-triangle-as-zero
kind: issue
title: cert_cylinder and cert_sphere clamp a NaN to 0: a cylinder triangle with a side on a ruling certifies at 0 in some corner orders
status: open
opened: 2026-11-16
---


## Finding

`cert::cert_cylinder` is `(r − dist_line_triangle(axis, T)).max(0.0)`
(`crates/mesh/src/cert.rs`), and `f64::max` drops a NaN. One arises on a
sound triangle. A triangle with a side along a ruling projects, across the
axis, onto a segment: `dist_line_triangle` hands `dist_point_triangle` a
triangle of no area, whose interior branch divides by zero for some corner
orders and answers NaN. The certificate is then 0.

Measured: the cylinder of radius 1 about `y`, the triangle
`(1, 1, 0)`, `(1, 0, 0)`, `(cos π/6, 0, −sin π/6)` (a half-cell of a column
a twelfth of a turn wide, as the curved lane's `nu == 1` branch emits it for
`tools/tess-meter`'s `mesh5_r2probe::r2_wedge_bytes`, the frustum wedge at
δ `0.25`): `r − dist` is NaN and the certificate 0; the triangle deviates by
the chord's sagitta, `1 − cos(π/12) = 0.0341`, which its sibling half-cell
`(1, 1, 0)`, `(cos π/6, 0, −sin π/6)`, `(cos π/6, 1, −sin π/6)` certifies at.
`cert::tests::no_certificate_reads_an_input_that_is_not_finite_as_a_bound`
pins both, and a corner that is NaN, which reads 0 the same way on the
cylinder and on `cert_sphere`.

## Why it matters

An under-reporting certificate passes a mesh that is over δ. On the curved
lane a cell's two halves share the chord, so the sibling reads the cell's
deviation and the face's verdict is right. Nothing says the same of the
trimmed cylinder arm's triangles, where a triangle with one side on a
ruling need not have such a sibling. Not measured: whether any accepted
mesh holds a triangle read as 0 that is over δ.

## Not fixed where found

Found by PR 6965's fix pass, which made the two certificates answer NaN
where `r − dist` is not finite: that refused the frustum wedge above
(`CertificateExceeded` with the sibling's bound, `0.034` against δ `0.25`),
a face of the rectangle lane, which that unit may not move. The cure is the
distance, not the clamp: `dist_point_triangle` total on a triangle of no
area (the nearest of its sides), after which the clamp can poison on a NaN.
It moves cylinder certificates from 0 to their true value, so its PR owes
the sweep that shows which verdicts move.
