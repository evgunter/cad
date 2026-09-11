---
id: step-adopt-let-ok-iso-discards
kind: issue
title: step-import adopt.rs takes let Ok(iso) at two recognizer sites - S394's undecided half (EXCH's file)
status: open
opened: 2026-09-06
refs: [S394, 2095]
---


## What

`S394` (closed by TRIM-1, PR #2095) converted the three `map_err(|_|`
swallows in `pcurve_cache.rs`. Its finding named two more discards of
the same `SplineError` through a different idiom —
`crates/step-import/src/adopt.rs`'s `let Ok(iso) = …` inside the
recognizers — and asked that they be decided separately: a recognizer's
refusal may be an answer ("this shape is not the one I think it is")
rather than a fault. TRIM-1's Closed section left them undecided.
`adopt.rs` is EXCH's file (Track U), so this is the handoff.

## Decision wanted

For each site: is a `SplineError` from `boundary_iso_*` a
"not-this-shape" answer (keep the `let Ok`, say so in a comment) or a
fault that should surface (convert to a typed refusal)?

## Home

Unowned at filing — EXCH's ground; filed by the TRIM orchestrator from
PR #2095's dual (R1 NOTE-3).
