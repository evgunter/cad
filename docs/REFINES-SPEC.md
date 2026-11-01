# REFINES: the window door admits a chart when either half narrows it

Unit: `work/clear/clearance-refines-reads-only-the-lower-halves.md`
(P0/E). Program: CLEAR. Branch: `clear/refines`. Review: a single FULL
review with mutation. The change is small, but it widens what the
injectivity sweep is asked to decide.

Read the row in full; CARVE's experiment there is the evidence.

## Decided shape (CLEAR orchestrator)

`refines` (`crates/topo/src/clearance.rs`) reads BOTH halves on each
axis. An axis refines when EITHER half moves a bound of the window's
box. This was CARVE's experiment `CARVE_EMBED_SCRATCH_REFINES_BOTH_HALVES`.

The door is an admission rule, not a certificate. A chart admitted
wrongly costs only depth or budget, which the engine already refuses
typed, so "either" is sound. Correct the doc comment's premise: the
isotony argument holds of the pair of halves, not of one half alone.

## What is true when this lands

1. **The loft walls are admitted, and the sweep decides them.** On the
   arc spine at 3.5, 6, 9 and 13 rad over 17 stations, on CONTACTHOLD's
   closed spine, and on CARVETAIL's opposite-turning loft, the walls
   are admitted. The sweep answers what CARVE's experiment answered:
   9 and 13 rad, the closed spine and CARVETAIL are `Violated`, naming
   the pairs in the row.
   - Pin these as rows at both scalars, built by sweep's own builders.
     Do not depend on CARVE's scratch branch.
   - A row that runs past budget (the arc at 3.5 or 6 rad is the
     sliver-knot row's case) pins its typed refusal and cites that row.
2. **The helical row goes back.** Rename
   `a_long_helical_sweep_waits_on_the_window_doors_halving` back to a
   certifying row that holds at both scalars. Measure its time; if it
   is ≥ 0.5 s, put it in the slow set.
3. **The two-section loft whose top faces down stays refused.** Its
   wall is a bow-tie whose `u` iso collapses, so neither half narrows
   it. Pin that refusal and say why in the row.
4. **Nothing else moves.** Every self-intersection, pipe and clearance
   row keeps its verdict class at both scalars. List any receipt that
   moves, with why: a face that was refused at the door and is now
   decided is expected, but name it.
5. **Mutation.** Each of these must red a row:
   - going back to lower-halves only;
   - "both halves must move";
   - reading only the upper halves.

## Ground

- `crates/topo/src/clearance.rs` (`refines`, `window_of`) and the sweep
  tests.
- CARVE's scratch is `crates/sweep/tests/carve_embed_scratch.rs` on
  `carve/embedding-certificate`. Read it for the bodies, but do not
  merge that branch.
- Read `docs/prompts/implementer-discipline.md` in full.
- Never arm a waiter or a background chain for a build or a test.
  Anything that outlives 600 s runs `setsid`-detached and is polled in
  the foreground.
- CI is the verification of record. Allow 25 min.

## Report

≤60 lines, covering:
- the change;
- the rows with their verdicts and receipts;
- the moved receipts;
- the mutants;
- the PR number and its CI result.
