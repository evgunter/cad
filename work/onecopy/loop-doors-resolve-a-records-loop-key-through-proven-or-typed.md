---
id: loop-doors-resolve-a-records-loop-key-through-proven-or-typed
kind: issue
title: shell.rs's loop doors (cycle_of, loop_points, ring_rows, loop_rekeyed) take a bare LoopKey and resolve it through proven, so a torn link a face record holds panics without naming its holder
status: open
priority: P3
cost: E
opened: 2026-11-03
refs: [a-face-is-read-through-a-resolved-handle]
---

Found by the sweep of `a-face-is-read-through-a-resolved-handle` (branch
`onecopy/face-view`). That unit moved every outer-loop read in
`crates/topo/src/shell.rs` (the glue's `guest_outer`/`host_outer`, the hole
rim's `outer`, the wrap check's `anchor`, `host_cycle`, the guest's rekeyed
outline and its twin search) onto `FaceView::outer`, so the face→loop hop
is now a link that names the face's `outer` field.

The doors those keys are handed to still take a bare `LoopKey` and resolve
it themselves through `proven`: `shell::cycle_of` (`proven(&body.loops, …)`,
then `loop_walk(first).closed`), and `loop_points`, `ring_rows` and
`loop_rekeyed` beside it. So the loop is resolved twice, and a door's own
miss reads as "minted, resolved, or read out of a record" rather than naming
the record. The same shape PR 6460 retired for faces (`FaceView`, the
`BoundaryMember` walk through `Body::loop_walk_of` + `Walk::closed_in`).

Fix: hand these doors the resolved loop (`(LoopKey, &Loop)` as
`FaceView::outer` / `FaceView::loops` yield it) and walk it through
`Body::loop_members_linked`, which proves the claim. Rings-only reads in the
file (`rings_of`, `rings.is_empty()`) ask a question of the rings and are
not this row.

`sphere_chart_trim`'s outer-loop `CorruptFace` is TOPO's
(`work/topo/torn-body-refusal-families-beyond-the-six-doors.md`), not this
row.

**The same shape in the split, found by the second pass** (an outer key
read off a face record by `get_face(…)?.outer`, which neither the
`.outer`/`.rings` chain pin nor the `loop_keys` pin reads):
`splitting/join.rs` `SectionJoiner::cut` reads the `Completed` face's
outer loop as `get_face(face).ok_or_else(|| corrupt_face(face))?.outer`,
though the face is one `cut_core` just returned (a proven key, so a miss
is a kernel bug, not `Corrupt`); `splitting/finish.rs` (the slit homing
beside `ring_representative`) reads `get_face(slit).ok_or_else(corrupt)?.outer`
the same way. Both would read `body.proven_face(face).outer()`.
