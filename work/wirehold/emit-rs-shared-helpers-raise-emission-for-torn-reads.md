---
id: emit-rs-shared-helpers-raise-emission-for-torn-reads
kind: issue
title: name_family and name_assemble raise Emission for reads the tier-1 shape may make unreachable (three sites in names/emit.rs)
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

## Also in emit.rs, waiting on the D10 hold (2026-10-20)

`wire-emis-rest` (PR 5502) left four more raises in `names/emit.rs`, because they are on placement, pattern and instance ground:
- `flat_body_index` (2);
- `name_placed_union`;
- `name_in_part`.

They are listed here so the parent row can close. Convert them when the D10 hold lifts, under the same ladder.
- `output_body` (added 2026-10-21 by PR 5530's lane): it counts output bodies, which a pattern count sets with no arena cap, so there is no row-4 bound. All its callers are on D10 ground. It stays fallible.

## Partly closed by PR 5530 (2026-10-26)

PR 5530 converted all 11 non-D10 raises to row 4. Each one's why is either `TIER1` (a link a record holds) or `LIVE_ARGUMENT` (a key the caller read off a live record of the same body). Row 0 was answered "no": a live-key brand would spread into every emitter signature.

What remains here is the D10-held list above: `flat_body_index` ×2, `name_placed_union` and `name_in_part`. `output_body` moved to `loop-coordinates-narrowing-has-no-bound`. This row parks on the D10 hold.

## Moved to WIREHOLD (2026-10-26)

What is left is D10-held, so the row parks with WIRE's other D10 rows. Signed (WIRE orchestrator).

## D10 exit walk (2026-10-29)

The four D10-held raises this row lists are gone with their functions (`flat_body_index`, `name_placed_union`, `name_in_part`; stage 3 G, #5954). Their successors raise the same class, `NamingError::Emission`, for a torn member read: `name_family` (`crates/editor-core/src/names/emit.rs:1157`, "a family's member names a body the member does not have") and `name_assemble` (`emit.rs:1255`, `:1264`). The scene still stands under new names. Open, re-pointed at those three.

## Re-checked after D10 (2026-11-01, WIRE orchestrator)

From the read-only `wirehold-triage` against origin/main 8509df2510. INTENT closed on 2026-10-30, and every trigger this row waited on has fired.

**Still real, at new sites.** The four held raises went with G. Their successors raise `NamingError::Emission`:
- `name_family`: `names/emit.rs:1157`
- `name_assemble`: `names/emit.rs:1255` and `:1264`

`output_body` (`emit.rs:1115`) stays fallible, tracked by `wire/loop-coordinates-narrowing-has-no-bound`.

Each site needs the D9 argument: a type change, a typed refusal, or `unreachable!` with its why. Check first whether a member's own value can be multi-body, since the `e.body != 0` check may be reachable.
