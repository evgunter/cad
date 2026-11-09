---
id: tangent-arcs-construction-reads-across-on-its-stored-unit
kind: issue
title: tangent_arc reads its chord along and across the stored unit, beside Line2
status: open
opened: 2026-11-09
priority: P4
cost: M
---


Found by the PR 6594 review's planar sweep. `crates/profile/src/path.rs` `tangent_arc` (~`:3509`) reads `d = b − a` along and across the departure `u` as `(u·d, u×d)` by hand. Its test twin `cancelling_tangent` (~`:7460`) does the same. `tangent_arc_geom`'s decision reads the same pair, and was ported onto `Line2::height`/`Line2::side`. The construction was not ported: its centre is `a + perp(u)·ρ`, built on the same stored `u`, and its registered endpoint facts (`‖a − c‖ = |ρ|`, `‖b − c‖ = ρ`) rest on `across` being read on that `u`. Reading through `side`'s `/|u|` moves two stored-bit pins, `fillet_stored_tangency::the_corpus_stored_loops_dump_to_the_bit` and `path_program::the_equator_through_tangent_arc_to_is_arc_continues_table_bit_for_bit`.

Fix: build the centre from the same reading. One way is `a + n̂·ρ` with `n̂` the departure line's unit left normal and `ρ` from `side`. Then re-pin the two dumps with the measured movement. Or decide that the construction's own reading on a decided unit is the right one. That would wait for a typed unit direction (`an-axis-reads-its-direction-as-unit-by-convention-not-type`).
