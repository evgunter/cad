---
id: capend-top-bottom-contradicted-by-negative-extrude
kind: issue
title: CapEnd::{Top, Bottom} makes a spatial claim a negative extrude distance contradicts
status: open
opened: 2026-08-30
github: 1306
refs: [961, 1301]
---

## From GitHub issue 1306

opened 2026-08-30, 0 comments.

**Raised by BLEND-5's review round** (PR #1301, executed by a reviewer probe: `the_top_cap_can_lie_below_the_bottom_cap`).

An extrude's distance is explicitly signed (`crates/sweep/src/extrude.rs:100`, "a signed distance along the sketch plane's normal", gated only on a definite non-zero normal component), and `CapEnd::Top` is minted for the cap on the plane *translated by* the extrusion vector (`emit_sweep.rs:90`). Under a negative distance the face persisted as `Cap(Top)` lies strictly below `Cap(Bottom)`.

This is the same class BLEND-5 fixed for `RimSupport` — a persisted name-vocabulary variant asserting something about the geometry that a legal construction contradicts — and BLEND-5's own sweep disposed it as "structural in the minting op's own parameterization", the very defence that had been available for `RimSide::Plane` (it was the `plane_walk` slot) and was judged insufficient there. The two dispositions apply different standards; this issue is the record that the class has a second live instance.

Like #961 the names stay unique (the two caps still take different variants), so nothing collides — the defect is a misleading name, and fixing it is a naming-contract change with its own migration story (`CapEnd` is persisted). A fix could rename to the extrusion's own parameterization honestly (e.g. start/end of the sweep vector) or argue at the declaration why Top/Bottom is the right reading of a signed sweep.

Not scheduled to any live program; kernel naming territory.

## Home

`work/issues/` — the issue says so itself: kernel naming territory (`names/role.rs`, `emit_sweep.rs`) scheduled to no live program, and the raising program S-BLEND is closed.
