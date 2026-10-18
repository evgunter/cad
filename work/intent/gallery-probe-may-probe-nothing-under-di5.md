---
id: gallery-probe-may-probe-nothing-under-di5
kind: issue
title: viewer: r1_gallery_probe's free-move check may never run if the tour world-pins every instance
status: closed
opened: 2026-10-18
priority: P3
cost: E
closed: 2026-10-18
---

Found by the sequential review of PR 5296 (style Q4, unsure).

`crates/viewer/examples/r1_gallery_probe.rs` checks, per instance, that a
committed free-move marks a drawn part. It runs the check only when
`BeginFreeMove` admits the instance. Under DI5 a world-pinned copy refuses
`InTheWorld`, so if the tour's gallery world-pins every instance, the check
never runs and the render lane's step passes having probed nothing. Find out
whether any gallery document has a loose space. If none does, either add one
to the gallery or have the example count admitted probes and fail on zero.

## Closed: not an issue (2026-10-18, by execution)

The tour wrote its gallery (`cargo run --release -- gallery <dir>` in
`demos/tour`: 9 standalone documents, plus 7 assembly documents from 3 stops)
on main after PR 5296. `cargo run --release -p viewer --example r1_gallery_probe
-- <dir>` exits `gallery probe: OK`, and the probe check does run. Two assembly
documents (the flat-pack layout and the document one of its refactorings
writes) each admit a free-move on the layout's patterned instance, node
`5bc49e2dcc54`: `free-move committed, scene probe_parts=Some(1)`. Every other
instance refuses typed.

The layout's instance is loose by construction: `layout_doc` in
`demos/tour/src/assembly.rs` inserts it with no placement edit and places the
pattern's copies instead. So the render lane's step probes at least one loose
space. If a later edit to the tour pins that instance, the example's
per-instance check stops running without failing. Counting the probes it
admitted, and failing on zero, would guard against that. It is not needed
today.
