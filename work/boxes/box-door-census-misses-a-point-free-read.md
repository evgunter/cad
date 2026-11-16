---
id: box-door-census-misses-a-point-free-read
kind: issue
title: boxes.rs's DOORS census needles end in '(', so a point-free box read goes uncounted
status: closed
opened: 2026-09-29
priority: P4
cost: E
pr: 6957
closed: 2026-11-14
---


Filed by ORIGIN from PR 3425's needle sweep: `DOORS` in
`crates/topo/src/boolean/boxes.rs` matches `(`-terminated needles, so a
point-free read (`.map(face_box)`) is not counted. `source_walk::tokens`
(PR 3425) is the whole-token matcher that sees it. Signed (ORIGIN
orchestrator).

## Closed (PR 6957, 2026-11-14)

The census is retired into `scripts/gates/cover-kind-match.sh`, whose
`reader` kind counts the doors by whole token in every crate, a
point-free read included. (boxes-cover7 lane)
