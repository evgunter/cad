---
id: box-door-census-misses-a-point-free-read
kind: issue
title: boxes.rs's DOORS census needles end in '(', so a point-free box read goes uncounted
status: open
opened: 2026-09-29
priority: P4
cost: E
---


Filed by ORIGIN from PR 3425's needle sweep: `DOORS` in
`crates/topo/src/boolean/boxes.rs` matches `(`-terminated needles, so a
point-free read (`.map(face_box)`) is not counted. `source_walk::tokens`
(PR 3425) is the whole-token matcher that sees it. Signed (ORIGIN
orchestrator).
