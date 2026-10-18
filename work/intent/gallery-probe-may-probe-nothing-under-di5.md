---
id: gallery-probe-may-probe-nothing-under-di5
kind: issue
title: viewer: r1_gallery_probe's free-move check may never run if the tour world-pins every instance
status: open
opened: 2026-10-18
priority: P3
cost: E
---

Found by the sequential review of PR 5296 (style Q4, unsure).

`crates/viewer/examples/r1_gallery_probe.rs` checks, per instance, that a
committed free-move marks a drawn part. It runs the check only when
`BeginFreeMove` admits the instance. Under DI5 a world-pinned copy refuses
`InTheWorld`, so if the tour's gallery world-pins every instance, the check
never runs and the render lane's step passes having probed nothing. Find out
whether any gallery document has a loose space. If none does, either add one
to the gallery or have the example count admitted probes and fail on zero.
