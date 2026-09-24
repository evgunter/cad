---
id: naming-index-casts-saturate-silently-at-u32-max
kind: issue
title: The sweep emitter's index cast saturates at u32::MAX instead of refusing
status: open
opened: 2026-09-24
priority: P4
cost: E
---

`names/emit_sweep.rs::ix` (`:63` at filing) turns a loop, segment or vertex
index into the `u32` the role vocabulary stores with
`u32::try_from(i).unwrap_or(u32::MAX)`. It has 9 callers in that file.
Past `u32::MAX` two different indices would spell one name, so a profile
entity would alias another without anything saying so. This is the shape
`name-counts-saturate-silently-at-u32-max` fixed at its three sites, where
the emitters now refuse with `NamingError::Emission` (`emit_topo.rs::group_count`).
That row listed only the count sites. This index site turned up in its sweep.

It cannot be reached: no profile holds that many loops or segments. It
still breaks the fail-loud rule. The fix is `ix` returning
`Result<u32, NamingError>` and its callers using `?`.

Checked and left alone: `names/emit.rs::name_pattern`'s
`output_body(usize::try_from(j).unwrap_or(usize::MAX))` saturates only to
hand the value to `output_body`, which refuses anything over `u32::MAX`
anyway. So that site is already loud, although it reads oddly.
