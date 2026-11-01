# NETCACHE: a cell's restricted net, jets and normal are computed once per cell, not per pair

Unit: `work/clear/clearance-recomputes-a-cells-net-and-jet-per-pair.md`
(P0/M). Program: CLEAR. Branch: `clear/netcache`. Review: a single
FULL review. A cache on the certificate path is sound only if it is
keyed exactly, so its claims are about the key, the invalidation and
bit-identical verdicts.

Read the row in full; PRICEMEASURE's profile is there.

## What is true when this lands

1. **What is cached.** Each cell's restricted net is computed at most once
   per cell per walk: the net read by `read_net`/`rect_net`/`rect_jet`,
   and also the jets and the centre normal where they are read per pair.
   That holds wherever departure, the monotone arm, the support or
   presentation read it.
   - Choose where the cache lives: on the cell (`Cell` is `Copy` today), in
     a per-walk arena keyed by (face, cell rectangle), or by carrying the
     restriction down the split tree, so that a child restricts from its
     parent's net, not from the face's.
   - Weigh these in the PR body. The last may also cut `Interval::between`'s
     47%, because a child's restriction is one blossom step from its
     parent.
2. **Verdicts and receipts are bit-identical.** Every self-intersection,
   pipe, cavity, shell and clearance row, at both scalars, gives the same
   verdict and the same receipt (splits, certified, deepest). Only time
   moves. A cache that changes a receipt is a finding: stop and report.
3. **The numbers.** Re-run PRICEMEASURE's 14 tour bodies, using its
   harness from the row and a local scratch, not committed. Report each
   body's time before and after, and its ratio to the build, in a table
   in the PR body and appended to the row.
4. **Mutation.** A cache keyed too loosely must red a row. Examples:
   - one key per face instead of per cell;
   - the parent's net reused for a child without restricting it.

   Show that a row catches each.

## Ground

- `crates/topo/src/clearance.rs` (the walk, `Cell`, `support_separation`);
- `crates/topo/src/injective.rs` (`departure`, `monotone`, the
  presentations);
- `crates/geom/src/surfaces/nurbs.rs` (`read_net`, `rect_net`, `rect_jet`)
  and `geom::net`.
- Read `docs/prompts/implementer-discipline.md` in full.
- Never arm a waiter or a background chain for a build or a test.
  Anything that outlives 600 s runs `setsid`-detached and is polled in
  the foreground. Watch the disk.
- CI is the verification of record. Allow 25 min.

## Report

≤80 lines, covering:
- the cache's home and the shape weighed;
- the receipt check;
- the before/after table;
- the mutants;
- the PR number and its CI result.
