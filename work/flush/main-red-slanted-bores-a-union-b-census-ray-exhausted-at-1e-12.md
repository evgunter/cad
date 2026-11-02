---
id: main-red-slanted-bores-a-union-b-census-ray-exhausted-at-1e-12
kind: issue
title: Main red: the 3° slanted bores' A ∪ B reads tier 3′ CensusUnsupported RayExhausted at ε 1e-12, outside its pin
status: open
opened: 2026-11-02
priority: P2
cost: M
---

Found by SPLITDOOR (`splitdoor/cone-apex-boolean`, PR 6266) running its
edited rows at three ε. Red identically on main `3a672b604`.

## What

`CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --test all -E
'test(slanted_bores_build_every_operation_one_difference_pinched_to_itself)'`
fails. The row (`crates/sweep/tests/bisector_nodes.rs`) is pinned red
at 1e-12 on one finding only, the 3° pose's B ∖ A exhausting its probe
ray (`work/tang/main-red-slanted-bores-b-minus-a-exhausts-its-probe-ray-at-1e-12.md`).
It now stops earlier, at `bisector_nodes.rs:1287`:

    3° (A, B), ∪: 3′ Err([CensusUnsupported { subject: Entity(Face(FaceKey(7v13))), cause: Containment(RayExhausted) }, … FaceKey(8v9) … FaceKey(9v3) … FaceKey(11v3) …])

## Where to look

The same ray exhaustion as the tang row, read by tier 3′'s census on the
union instead of by the pieces probe on the difference: likely one root.
Add the union's finding to that row, or widen the pin, once the root is
read.
