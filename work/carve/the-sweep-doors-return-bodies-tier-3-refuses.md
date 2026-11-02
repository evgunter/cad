---
id: the-sweep-doors-return-bodies-tier-3-refuses
kind: issue
title: extrude, revolve, loft, sweep, fillet and chamfer gate their results by a tier-2 debug assert alone, and extrude returns bodies tier 3 refuses
status: parked
opened: 2026-10-14
priority: P3
cost: M
blocked_on: [the-at-rest-result-gate-and-its-typing-live-in-five-homes]
---


Found by CLEAVE's sweep for `a-split-along-a-seam-at-a-vanishing-tilt-returns-an-invalid-body`
(branch `cleave/seam-split-invalid-ok`). That unit found the split
returning sides that `validate_geometric` refuses, because its result
gate was tier 2 alone (`validate_closed`). It gave the split the
Boolean's at-rest gate (`AtRestPolicy::gate_at_rest`), typed as the
Boolean types it (`SplitFinishError::ResultNotAtRest`, `ResultEscalated`).

The sweep doors gate their result by a
`#[cfg(debug_assertions)] debug_assert_eq!(topo::validate_closed(..), Ok(()))`
alone. The workspace builds release with `debug-assertions = true`
(`Cargo.toml`, `[profile.release]`, until publishing), so the assert does
run, but a breach panics rather than refusing, and nothing reads tier 3.
Each door returns a plain `Body`:

- `sweep::extrude` (`crates/sweep/src/extrude.rs`);
- `sweep::revolve`, full and partial, and the tubes over it
  (`crates/sweep/src/revolve/full.rs`, `revolve/partial.rs`, `revolve/tube.rs`);
- `loft_body`, `loft_profiles` and `sweep_body` (`crates/sweep/src/loft.rs`'s
  shared assembly);
- `sweep::fillet_edges` and `sweep::chamfer_edges` (`crates/sweep/src/blend/surgery.rs`).

Each door's doc states its bar: `extrude` "passes tier 3 except at the
smooth cap rim `Extruded::body` names", `revolve` "passes tiers 1–3"
(`revolve/mod.rs`), `Lofted::body` "tiers 1–3 valid at rest, the
builder's acceptance". Each then says the caller re-validates "per the
workspace convention", which is written nowhere.

The Boolean (`topo::boolean::ops::gate`), `topo::shell` and STEP import
gate at tier 3, and now the split does. The topo offset doors and
`replace_face_offset` are ratified construction state, tier 2 in and
tier 2 out (`crates/topo/README.md`, shell and offset surgery row).

**Measured witness (PR 5100's review):** `sweep::extrude`, release,
ε = 1e-12. A 1 × 1 profile whose top side is an arc of radius
200–400 m, on a tilted `sketch_from_axes` plane, extruded 0.7, returns
`Ok` in 4 of 20 poses while tier 3 refuses the body with
`VolumeUncomputable` on `props_chart_loop_whole`. That falsifies
extrude's own "passes tier 3 except at the smooth cap rim".

The suites mostly finish these results through `AtRestBody::validate`
(`sweep::test_support::finished`), which panics on a refusal, so the
corpus they cover shows none. The editor holds every node's body
unverdicted and gates it only where a later node consumes it
(`editor-core` `finished_operand`), so a leaf node shows such a body
without a refusal.

**What closes it:** gate each door's result at the scalar's at-rest
gate, typed as the Boolean and the split type it
(`topo::boolean::at_rest_refusal`, crate-private today, so this wants a
public door), and correct extrude's stated bar.
Alternatively, write the "workspace convention" down: these doors return
construction state, and the editor gates a leaf node's body.

## Parked (2026-10-31) on ONEHOME's gate home

Its two closes (gate each door at rest, typed like the Boolean and the
split; or write down that the doors return construction state and the
editor gates) are the question ONEHOME's
`the-at-rest-result-gate-and-its-typing-live-in-five-homes` (P1, design)
weighs for all five doors. Building a sixth home first would be what
that row retires. This row waits for that row's answer, then builds
the sweep doors' instance of it. (CARVE orchestrator)
