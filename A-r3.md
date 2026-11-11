# Fork `clear-enclosure` — designer A, round 3

Shown: designer B's round-2 report. Asked: PR 2701's reach, what a door cannot do that the type can, the type's cost, and a final recommendation.

## For Ev

**Recommendation (likely): no separate cell-bound type.** The engine keeps its cell readings out of the evaluation's verdict log itself. The sweep's two public entry points, `Sweep::run` and `min_separation`, open the scope that keeps decisions out of the log (the `unlogged` scope), so no caller has to. Designer A now joins designer B on this. The type is the alternative.

**PR 2701 (sure).** What ruling 1 retired was `RingInterval`, a second interval type. Your words: "i lean towards a more aggressive unification", "the aggressive unification plan sounds good!".
- The ruling passed through "a newtype over `DInterval`" only as a waypoint. Its step (iii) reads: "the newtype dissolves into `Interval`".
- `RingInterval` was not even a `Real`. When it was introduced (commit `83cd14985fe`) it had ring operations only, no `Decide`, and its transcendentals were hand-rolled.
- A cell-bound type would have to be a full `Real` by delegation, because the kernel's vector type gives sums, dot, cross and norm only to a `Real`. That makes it more of a second interval scalar than the type you retired.
- So "no `Decide`" does not take it outside your ruling. Designer A's round-2 distinction does not hold.

**What a door cannot do that the type can (likely).**
- Every engine decision calls the ordinary decision funnel directly today: `k_stats::decide` in `clearance.rs`, `validate::decide` in `injective.rs`.
- Under the door design they stay unlogged because the scope is dynamic. It covers everything under the two entry points, and the engine's internals are reachable only through them (`injective` is crate-private).
- What the type alone would catch is a future cell reading made outside those entry points, for example by the at-rest gate itself. No such site exists in production code today. The only callers of `cell_region` outside the engine are test code.

**What the type costs beyond compile time (likely).**
- **The whole `Real` surface by delegation.** It must also pick a side of the certification split on its held-record associated type, `Real::Carries`.
- **A second funnel door**, plus rows in the margin-dimension audit (`docs/predicate-dimension-audit.md`) and the flagged-site gate.
- **A hand-kept seam where carrier output turns into a cell bound.** About 80 typed vector sites in the engine change.
- **It does not cover the whole population.** The walk's station and region readings are made at the body's own scalar (audit row F28's readings). Those would still log, so F28's scope stays hand-kept. That makes the type incomplete, not merely costly.
- It need not duplicate the `geom` evaluators: the cell bound can wrap their output instead.

**The final state.**
1. One crossing door, held by the lane.
2. No second reading: equalities come from topology or from tier 3. For separators the engine builds itself, the separator vanishes on the shared edge by construction.
3. Each refusal names which width blocked it: the family's or the cell's.
4. The engine's entry points own the unlogged scope, so F28–F30's hand-kept scopes and the shell's missing one go. The one reading made at the body's own scalar that the drive should see is logged by the caller after the sweep returns, as F30's exception does today.
5. The `[topology.tiers]` rewording.

No gate on the engine's files: one door is enough, and a gate can be added if a site outside the entry points ever appears.

## For the orchestrator

- **Designer A and designer B have converged.** Put the door (change 4) to Ev as the recommendation. Put the delegating `CellBound` as the alternative. Its one advantage is a compile-time check against cell readings made outside the entry points, which no production site makes today. Its costs are above, the decisive one being that F28's region readings would still log. Designer A can live with either but recommends the door.
- **For the spec.** A finding's check made inside the verifier callback runs under the scope, so it is unlogged. Any reading meant for the drive must be logged after the sweep returns.
- **Checked in the clone:**
  - audit rows F28–F30 (`docs/predicate-dimension-audit.md`);
  - `cavity_embedded` has no scope (`topo/src/shell.rs`);
  - window building makes no decisions;
  - `RingInterval` was introduced (commit `83cd14985fe`) with ring operations and no `Real` impl;
  - H5's §RATIFIED at commit `e8d0883eda`.
- **The shell's missing unlogged scope is still unfiled.** It belongs in `work/clear/`.
- Nothing was built. There is no target directory.
