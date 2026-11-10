---
id: three-raw-sign-within-reads-predate-their-gate
kind: issue
title: Two production files read sign_within raw, outside a named k_stats door (replay.rs, cylinders.rs): each is either no decision (say so) or an unnamed one (route it)
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

**`edge_join::crease_at` is done** (2026-11-10, `onecopy/crease-at`; orchestrator ruling (i)). It reads three values through `k_stats::prove("edge_join_crease_at", ..)` at the least band (`crease_read`): `Crease::Decided` where `t − k` is decided zero, `No` where every difference is decided nonzero, `Undecided` where one straddles. `prove` is the existing door that keeps the zero apart from the straddle, and its doc fits: a refusal is an outcome a narrower box can answer.
- `joinable` takes the edges' pair through `kept_edge`: **an undecided knot stops the join; only a decided crease picks the kept edge.** A straddle therefore leads only to the split, a vertex left on a smooth edge, which is valid topology. Ev's PR 5913 ruling holds: no spline runs on through a knot the join could not rule out.
- Pinned by `edge_join::crease::a_straddled_knot_reads_undecided_and_stops_the_join`.
- Measured: no join moved anywhere in `topo`, `sweep` or `editor-core`. The fast suites ran 9595 tests at each ε, and the join-related slow set (`join|crease|spline|nurbs|knot`) 727, both instrumented to record every `Undecided` that stops a join; none fired. No test reaches a straddled knot.
- The read is no longer raw, so `edge_join.rs` leaves `scripts/gates/raw-sign-within.counts`. Two sites remain (`replay.rs`, `cylinders.rs`).
