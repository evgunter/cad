---
id: a-fitted-cap-cut-by-planes-has-a-sub-range-trim-image
kind: issue
title: A moved fitted cap's plane x fit edges span a sub-range of their sections, which the trimmed lane refuses (QuadratureUnsupported), so its volume is not taken
status: open
opened: 2026-10-09
priority: P3
refs: [a-moved-fitted-faces-corners-have-no-root-on-a-derived-spline-section, a-fitted-face-trimmed-by-a-section-has-no-volume-rule]
---

Found by SHELL's fitted-corners unit. `crates/sweep/tests/encl_curved_loft_shell.rs`'s
`a_moved_fitted_cap_stands_its_corners_on_the_held_sides` moves the unit
box's cap (a bilinear NURBS patch over `[-1, 3]²` at `z = 1`, wider than
the face) by ±0.05. The move builds, every corner lands on the closed
form `(x, y, 1 + d)`, and tier 3 passes every phase but check 7, which
refuses the cap alone:

`VolumeUncomputable { source: Face { source: QuadratureUnsupported { what:
"a General trim image whose carrier interval is not its own knot domain —
the trimmed lane subdivides the stored image whole, and a sub-range would
integrate along chart the face does not bound" } } }`

(`crates/topo/src/props/quad_lane.rs`, the trim-piece guard). Each cap
edge is a side plane's section with the fit, whose carrier is the whole
branch across the fit's window; the edge spans the part between its two
corners, so its stored image's carrier interval is a sub-range of the
image's knot domain. The guard's own comment says "NO ROW AND NO KNOWN
PRODUCER … nothing at rest stores a sub-range"; the moved fitted cap is
a producer, and the row above reaches the guard at ε = 1e-9.

Owed: the trimmed lane integrating a sub-range of a stored image (split
the image at the carrier interval's ends), or the edge storing its image
over its own span. The closed form is waiting for it: the moved box's
volume is `4 (1 + d)`, which the row above can then assert in place of
the refusal. `a-fitted-face-trimmed-by-a-section-has-no-volume-rule` is
the window refusal the same fit reaches when only a side moves.
