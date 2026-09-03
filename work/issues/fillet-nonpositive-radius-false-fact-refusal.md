---
id: fillet-nonpositive-radius-false-fact-refusal
kind: issue
title: fillet_edges at nonpositive radius emits a false-fact refusal; the two blend doors validate size asymmetrically
status: open
opened: 2026-08-31
github: 1336
refs: [1328, 1278]
---

## From GitHub issue 1336

opened 2026-08-31, 0 comments.

**Raised bilaterally by BLEND-6's review round** (PR #1328; R1 executed it, R2 flagged the same asymmetry as style).

`chamfer_edges` refuses `NonpositiveSize` at its door with a stated rationale ("a nonpositive size levers the margins that quote it… a false fact about the body is worse than no diagnosis"). `fillet_edges` has no such check, and the rationale's prediction comes true by execution: at radius `0.0` on a cube the caller reads

> "fillet: radius 0 m exceeds the curvature headroom of support FaceKey(3v1) — margin 0 m at lever arm 0 m; reduce the fillet radius…"

— false in both halves for the input, with a recourse that has nowhere to go from zero (the #1278 unfollowable-recourse class).

BLEND-6 adopted a characterization probe pinning the current behavior and cites this issue at both doors' size-validation sites; the behavior change itself was declined there as beyond that unit's ratified scope (vocabulary, not door semantics). The fix is small and its shape is already written: the chamfer door's check and rationale, applied at the fillet door (BLEND-6 also extracted the shared `repeated_edge_gate` preamble, so the natural home exists). Whoever takes it flips the characterization probe to pin the typed refusal.

`crates/sweep/src/blend/build.rs`, both `_inner` doors. S-BLEND territory; small enough to ride a future blend unit.

## Home

The issue names S-BLEND territory (`crates/sweep/src/blend/build.rs`), but `work/blend/` is a closed program and may hold only closed items, and no open program's territory covers the blend module — so it lands in `work/issues/` until a program claims the blend doors.
