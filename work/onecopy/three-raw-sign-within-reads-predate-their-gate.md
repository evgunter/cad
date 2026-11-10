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

**`edge_join::crease_at` is done** (2026-11-10, `onecopy/crease-at`, PR 6709; the orchestrator's ruling, revised on that PR's review). It reads whether an edge's end parameter `t` lies on a crease of its spline carrier as two parameters compared exactly: `crease_read`, through `geom_brep::curve_pair::param_order`, the one home it shares with the curve pair's walks (`k_stats::prove`, the least band).
- A zero difference is `Crease::Decided`; every difference decided nonzero is `Crease::No`.
- A straddle is the join's undecided reading, `JoinReading::Crease` through `JoinUndecided`, at the join and at tier 3's check 11 alike, as every other undecided join reading is. It is not a guess either way: "no join" would leave a non-maximal edge where the truth is no crease, and check 11 could not see it.
- Only a decided pair picks the kept edge (`kept_edge`). Ev's PR 5913 ruling holds: no spline runs on through a knot the join could not rule out.
- Pinned by `edge_join::crease::a_crease_is_read_decided_or_its_straddle_escalates` and, end to end, by `a_split_straddling_a_crease_refuses_the_join_undecided`: a spline split at an interval straddling its crease refuses undecided. That row is red where `joinable` drops the undecided channel.
- Measured: no outcome moved anywhere in `topo`, `sweep` or `editor-core`, at both ε, fast suites and the join-related slow set. No test reaches a straddled knot.
- **The per-knot logging, accepted.** `prove` is logged, so each knot compared at a join vertex adds a verdict row. That is the reading's own record: the comparison now decides an outcome, the join or its refusal, so the verdict-diff engine should see it. A straddle is an escalation a narrower enclosure answers, which is the subdivision driver's cue by design. The rows are bounded by the knots of the two edges at a valence-2 vertex.
- The read is no longer raw, so `edge_join.rs` leaves `scripts/gates/raw-sign-within.counts`. Two sites remain (`replay.rs`, `cylinders.rs`).
