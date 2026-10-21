---
id: patch-face-area-gauge-panics-on-random-bicubic-faces
kind: issue
title: nurbs_patch_face trips the A2 area-gauge debug_assert on 2 of 200 random bicubic faces instead of returning a typed answer
status: open
opened: 2026-10-10
priority: P2
cost: M
---

Found by the NURBS delta review of PR 4485 (probes on
`nurbs/review-4485-delta2-a7c3`, `probes/review-4485-delta2/`). It is
the same on main, on `c5d1c7ae` and on `2ac0994f80`, so it predates
that PR.

`nurbs_patch_face` on random bicubic faces, with a caller-supplied
perimeter below the true one, panics on 2 of 200 faces in a debug build.
The panic is at `debug_assert_area_gauge`
(`crates/geom-brep/src/props/quad.rs:2534`, called at `:3583` and
`:3906`). It does not return a certified area or a typed refusal.

A caller's input reaching a panic at a public door is a D9 defect.
Either the gauge's denominator must not trust a short caller perimeter,
or the door must refuse that perimeter typed. The first step is to
decide which by reading `area_gauge_denominator` against the probe.
