---
id: no-row-pins-that-the-boolean-union-path-reads-window-copies-alike
kind: issue
title: No row pins that the boolean union path reads an image's window copies alike
status: open
opened: 2026-10-16
---


Found on PR 5225 (PCERT B1, `pcert/azimuth-placement`), whose delta
review asked that an argued claim not stand unscheduled.

`geom_brep::ANGLE_WINDOW_START`'s doc
(`crates/geom-brep/src/pcurve_cache.rs`) claims that a midpoint landing
on the window's cut is harmless: no reader tells the two copies apart.
For validation, the chart boundary, the polygon modulo a turn, and a
face's overlap with itself, a row pins this:
`topo::pcurves::pole_slit_tests::a_meridian_on_the_windows_cut_reads_the_same_either_side_of_it`.

For a boolean's union path the claim is only argued. It was measured
once, by moving the cut onto a cone's seam (to `0` and to `0.5`) under
`editor-core`'s `refusal_concision` cone union, which read the same. No
committed row holds it, so a union-path reader that came to tell the
copies apart would go unnoticed.

What closes it: a row that runs a boolean union (the cone union of
`refusal_concision`, or a smaller cylinder pair) whose operand has a
seam meridian placed on the cut, and on either side of it a hundred
band widths off, and asserts that the three unions are the same body
up to the window's whole turn. A row like that needs a way to put a
body's seam on the cut without moving the window. The cut row's
chart-turning fixture does this for a sheet; a solid needs the same.
