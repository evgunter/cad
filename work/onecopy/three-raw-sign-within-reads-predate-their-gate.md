---
id: three-raw-sign-within-reads-predate-their-gate
kind: issue
title: edge_join's crease_at reads sign_within raw at the least band, and whether its straddle is a crease is a design call
status: open
opened: 2026-11-06
priority: P3
cost: E
---


Filed by the unit that added `scripts/gates/raw-sign-within.sh` (the `<unnamed>` K samples' class, after `geom_core::running::Rounded::divisor` was routed through `k_stats::decide`). The gate registers these three as they stood; whether each is a decision was not audited:

- `crates/editor-core/src/coincide/replay.rs` (the `zero` closure): a coincidence lane's zero test, whose sign picks discharged or deferred.
- `crates/geom-brep/src/pair_section/cylinders.rs` (`placement`): a region's placement against a ball; its doc says it is "not a decision about the pair".
- `crates/topo/src/boolean/edge_join.rs` (×2): a parameter on a knot or crease, and a walk past its end, both at `Band::least()`.

For each: if no outcome changes what is certified, say so where it is read (or take `check_unlogged` with an audit row); if one does, route it through `k_stats::decide` under a name, and measure the logs it moves. Lower the gate's register as each goes.

**`edge_join::carried_far` is done** (2026-11-09): routed through `k_stats::choose(.., "F39")`, a choice between two spellings of one parameter, its premise pinned by `edge_join::carry::the_carried_far_parameter_is_the_one_the_carrier_reads`. Its 22 samples were the corpus dump's last `<unnamed>`. `edge_join.rs`'s knot test remains.

**`edge_join::crease_at` does not route cleanly** (2026-11-10, `onecopy/link-bisector`). It reads `t − k` at `Band::least()` and takes `Zero` as a crease; anything else, an enclosure straddling `k` included, reads "no crease". None of the least-band doors keeps that split:
- `k_stats::choose`'s `Pick::Tie` is "decided zero, or straddles zero undecided" in one arm, so it cannot tell a crease from a straddle.
- `k_stats::check_unlogged` at `Band::least()` keeps the split, but `scripts/gates/least-band-on-a-door.sh` refuses a least-band decision through it.
- `k_stats::prove` keeps the split too, but it is logged. It would put one row per knot compared into every join's log, and an escalation for each straddle.

So the read stays registered.

**An open question it raises.** Reading a straddle as "no crease" lets a join run a spline carrier on through a knot it cannot tell it is not on. The module docs rest the join on Ev's PR 5913 ruling, that every edge carrier is C¹ by its knot structure strictly inside its cell. A straddle read as "crease" (keep the other edge, or do not join) is the side that ruling allows. That reading is `choose` with its tie taken as a crease, and it moves outcomes only at an interval scalar, where a straddle can occur. Whether to take it is a design call, not a routing one.

**Two of the three are done** (2026-11-10, ONECOPY roster-reach):
- `coincide/replay.rs::one_cylinder` decides (discharged or deferred), so it goes through `k_stats::decide_reported`. The metre data are under `replay_one_cylinder`, and the axis sines under `replay_one_cylinder_axis` at an unsourced metre of lever: bit-identical, audit F40. The lever's real source is `the-replays-cylinder-axis-is-levered-at-an-unsourced-metre`. editor-core's `ci` suite does not move.
- `pair_section/cylinders.rs::placement` decides nothing (a choice of listing, each sound), and says so at the read: `// NO-DECISION: …`. `raw-sign-within.sh` now reads that marker on the line directly above a raw read and does not count the read. Its selftest plants a marked read, one with no why, and one not directly above.

**What remains is `edge_join::crease_at`**, registered with the reason above. Its next step is the design call on whether a straddle reads as a crease, not a routing.
