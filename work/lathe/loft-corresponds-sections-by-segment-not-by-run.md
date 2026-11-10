---
id: loft-corresponds-sections-by-segment-not-by-run
kind: issue
title: loft pairs sections segment by segment, so a section whose straight run is written in two collinear segments (a filleted leg once ROUND retires the ray-extension merge) stops corresponding to the same shape drawn in one
status: open
opened: 2026-11-09
priority: P2
cost: M
---


Filed by the ROUND orchestrator (2026-11-09) from Ev's ruling on PR 6542.

## Finding

`crates/sweep/src/loft.rs` (the section pairing; `crates/sweep/README.md`,
"Walls: one per run", its loft sentence) builds one wall per corresponding
SEGMENT pair across sections. Extrude and revolve already build one wall
per collinear or cocircular RUN "however the author wrote them". So loft
alone makes a section's correspondence depend on how its runs were
spelled: `line(len).continue_to(..)`, a kept station, and — after ROUND's
build of PR 6542 — every `fillet(r)` after a straight leg, which then keeps
the leg's authored end as a vertex with a collinear run-in segment
(Ev, 2026-11-09, PR 6542: the profile's segment list is what the author
emitted; "one curve" lives in the lowering).

## Owed

Loft corresponds sections by RUN (the cosurface runs the lowering already
computes for extrude and revolve), so two sections of the same shape
correspond however their runs were written. No in-repo loft is known to
depend on the segment pairing today (`demos/tour/src/lily.rs` uses no
fillets); the ROUND build lands without waiting for this.
