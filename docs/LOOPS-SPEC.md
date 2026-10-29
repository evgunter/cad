# LOOPS: one frontier loop, one budget pair

Two P1 rows, one unit. Program: CLEAR. Branch: `clear/loops`. Review:
a single FULL review (it is a structural refactor with no behaviour
change).
- `work/clear/the-bound-walk-and-min-separation-duplicate-one-frontier-loop.md`
- `work/clear/the-clearance-budget-pair-is-spelled-three-times.md`

## What is true when this lands

1. **One frontier walk.**
   - `topo::clearance`'s bound walk (`Sweep::run`) and `min_separation`
     share one frontier, budget and split loop, and differ only in
     what they decide per cell pair. In injective mode that includes
     SELFX's touching/apart branches (#5955).
   - Their overflow accounting (`refused` against `abandoned`) is one
     accounting. If the two numbers mean different things, name each
     where it is counted, instead of keeping two loops to hold the
     difference.
2. **One budget pair.** `{max_cell_depth, max_cell_pairs}` is one type,
   read by `ClearanceConfig`, the sweep and `MinSeparationConfig`
   alike.
3. **No behaviour change.** Every clearance golden,
   `m10_6_certifying_keys.txt`, every receipt count and every pinned
   row in `sweep/tests/self_intersection_proves_injectivity.rs` stays
   bit-identical. If any of them moves, stop, find out why, and report
   it. A moved value is a finding, not a re-pin.

## Ground

- `crates/topo/src/clearance.rs`, `crates/topo/src/injective.rs` (only
  where the loop is shared), and editor-core's `clearance.rs`/
  `measure.rs` config readers.
- A parallel unit, CLEARTEXT, is editing the refusal types and their
  renderings. Keep your diff out of the refusal types beyond the
  accounting fields.
- Run `python3 scripts/work.py territory --base origin/main`.
- Read `docs/prompts/implementer-discipline.md` in full.
- Never arm a waiter or a background chain for a build or a test.
  Anything that outlives 600 s runs `setsid`-detached and is polled in
  the foreground.
- CI is the verification of record. The diff seeds `topo`, so allow
  25 min.

## Report

≤100 lines, covering:
- the shape of the shared loop;
- what the two accountings mean;
- the differential, showing nothing moved;
- the PR number and its CI result.
