---
id: emit-rs-shared-helpers-raise-emission-for-torn-reads
kind: issue
title: names/emit.rs's shared helpers (to_u32, output_body, Incidence::of, the rim consts, vertex_point, face_half_edges, edge_ends) still raise NamingError::Emission for torn-body reads and u32 narrowings no body reaches
status: open
opened: 2026-10-20
priority: P2
cost: M
refs: [names-emission-raises-panic-or-take-their-true-ending]
---


Left by all three lanes of `names-emission-raises-panic-or-take-their-true-ending` (PRs 5501, 5502, 5503). Each lane owned only its own raise sites. These helpers are called from every emitter, so none of the lanes took them.

**The 12 raises** (count from `wire-emis-rest`, 2026-10-20), all in `crates/editor-core/src/names/emit.rs`:
- `to_u32` (also reached through `output_body`);
- `Incidence::of`;
- the rim constants `UNMATED`, `DANGLING_MATE` and `DANGLING_LOOP`;
- `vertex_point`;
- `face_half_edges` (3);
- `edge_ends` (3).

**Expected disposition: row 4.** Each one reads through a body the doors keep tier-1-valid, or narrows to `u32` where no body is that large. Convert each to `#[track_caller]` and infallible.

**Callers to change with them:**
- emit_topo, emit_union, borders;
- pose, measure, mate/pin (D10-held ground: pose, mate and pin are placement-adjacent, so check the D10 hold before touching them);
- the callers in `eval/anchor.rs`, `emit_shell.rs` and `eval/wire.rs` (`loop_coordinates`, and the four `output_body` sites).

**Panics inside closures.** A panic inside an `unwrap_or_else` or `map` closure defeats `#[track_caller]`: PR 5503's review showed the location reported is the helper's own line. Use `match` or `let … else`.

**K and ε.** A row-4 argument must hold for every K > 1 and every ε that `Tolerance` accepts (`geom-core/src/tolerance.rs`). Probe with `CAD_AMBIGUITY_K=1.5` and with `CAD_TOLERANCE_EPS=1e-6`.
