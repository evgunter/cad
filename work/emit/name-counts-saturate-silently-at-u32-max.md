---
id: name-counts-saturate-silently-at-u32-max
kind: issue
title: Three naming sites convert a usize count to u32 with unwrap_or(u32::MAX), saturating silently instead of refusing
status: dispatched
opened: 2026-09-23
priority: P4
cost: E
branch: emit/small-rows
---


Three sites convert a `usize` count to the `u32` the vocabulary stores,
with `u32::try_from(n).unwrap_or(u32::MAX)`:

- `names/emit_topo.rs::insert_ranked_or_tied` (`of` for a ranked group)
- `names/emit_topo.rs::name_split_faces` (`of` for split fragments)
- `resolve/mod.rs::group_size` (the `GroupResized` counts)

Above `u32::MAX` members each would saturate silently: a ranked name would
carry a wrong `of`, and two different oversized groups would compare
equal. None is reachable at any realistic body size, and that is why this
is P4. But the kernel's rule is fail-loud, and a saturating conversion is
a silent substitute. The fix is a typed refusal
(`NamingError::Emission`-shaped at the emitters; at the diagnosis rung,
declining is honest). Found by the code review of the EMIT
group-resized unit (PR 3115).
