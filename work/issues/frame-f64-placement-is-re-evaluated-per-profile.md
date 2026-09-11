---
id: frame-f64-placement-is-re-evaluated-per-profile
kind: issue
title: A frame's f64 placement is re-derived from its slots by every profile drawn on it although the frame's own evaluation held the same nominal values
status: open
opened: 2026-09-08
---


(EVAL orchestrator) From EVAL-10's style review (PR 2194, Q7). After
EVAL-10 the nominal environment is built once per evaluation, but a
frame node's nine slots are still evaluated at f64 TWICE: once by
`eval_node` (`crates/editor-core/src/eval/mod.rs`, the nominal slot
values fed to the content key and then discarded) and again by
`wire::profile_plane_f64` for every profile drawn on that frame
(`crates/editor-core/src/eval/wire.rs`), which re-derives the f64
placement from the same values. `slots.rs`'s header cannot say "once
per node per environment" while this holds. The fix carries the
frame's f64 placement on its `NodeResult` beside the `T`-valued
`DatumValue::Frame`, so the profile plane is a READ of the frame's
result rather than a re-evaluation — which touches values and possibly
keys (the pre-pass reads the placement from a result instead of the
document), so it is a unit with a correctness arm, not a threading
change. EVAL's files; re-homed by the exit walk if EVAL closes first.
