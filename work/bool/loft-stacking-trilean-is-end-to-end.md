---
id: loft-stacking-trilean-is-end-to-end
kind: issue
title: Loft stacking trilean is end-to-end — planar spines turning past pi refuse ReversedStacking (measured at the M8-14 close)
status: open
opened: 2026-08-11
github: 368
refs: [222, 316, BOOL-6]
---

## From GitHub issue 368

Opened 2026-08-11; 0 comments.

Filed per the inclusion-or-follow-up rule at the #222 (M8-14) close.

**Executed signature** (lily leaf-A geometry, `try_lofted_blade`, 17 stations, measured on the M8-14 branch): spine curls 0.45–3.0 rad build end-to-end — including 2.8 and 3.0, which PR #316's table measured refusing at `nurbs_span_meter` before the per-span meter — but curls 3.5/4.7/6.0 rad refuse typed `LoftError::ReversedStacking`. The wall is exactly spine turn π: `loft.rs`'s stacking trilean is an END-TO-END statement (mean last-section vertex displacement of the outer loop dotted with the FIRST section's normal), which for a planar arc spine is `cos(curl/2)` — negative past a half turn of total position stacking regardless of station count. Pinned from both sides in `demos/tour/src/lily.rs::review_probes::the_spine_curl_wall_re_measured`.

**Not a meter artifact and not station-dependent**: every slab advances honestly; only the end-to-end summary reverses. Helical SWEEP paths clear the same check through their pitch (the axial displacement keeps the dot positive), which is why #222's half/full/two-turn helical sweeps certify while a planar loft spine walls at π.

**SHOULD it be fixed**: eventually, if lofts along strongly-curled planar spines are wanted (full-curl fronds, C-channels said as lofts). The fix shape is a per-slab (or per-adjacent-pair) stacking fold instead of the ends-only statement — the per-slab margins all exist already; the design question is what the trilean's margin MEANS once it is a fold (min over slabs is the natural sound choice) and whether any consumer reads the current end-to-end value as a feature. Unscheduled; queues with the sweep-vocabulary family.

## Home

S-BOOL: `crates/sweep/src/loft.rs` is in the program's `paths` territory and the issue is already scheduled there as `BOOL-6` (the per-slab stacking fold, ruled 2026-09-01, scheduled on Helix demand); VERBS explicitly excludes the loft gate as not verb-gating.
