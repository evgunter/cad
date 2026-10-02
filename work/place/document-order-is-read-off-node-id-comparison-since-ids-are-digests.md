---
id: document-order-is-read-off-node-id-comparison-since-ids-are-digests
kind: issue
title: document order is read off a RecipeNodeId comparison in places, and since #3594 an id is a digest, so that order is arbitrary
status: closed
opened: 2026-10-01
priority: P1
cost: M
closed: 2026-10-02
branch: place/document-order-sweep
pr: 3882
---

## What

Since #3594, a `RecipeNodeId` is a digest of the edit that minted it,
not a counter. Code that orders nodes by comparing their ids, while
meaning "document order", now gets an arbitrary order. The tolerance ε
seeds the digests, so the result can differ between ε rows.

**Instance found.** `admit_mate` (`crates/editor-core/src/mate/solve.rs`
~934, `if wa.member.instance <= wb.member.instance`) picks which part
it asks first "in document order" by comparing ids. EDIT's PR 3676 hit
it: `msolve10_door_admission::a4_a_rider_needs_the_reach_and_a_plain_coincidence_asks_none`
went red at ε 1e-12 once P2's fixture changed the ids. On main the row
passes only because its ids happen to fall in order. PR 3676 fixes this
site by comparing positions in `doc.order()`.

## The work

Sweep every place an order over `RecipeNodeId` stands for document
order:
- `<`, `<=`, `min`, `max` and `cmp` on ids;
- iteration over a `BTreeSet`/`BTreeMap` keyed by id where the order
  reaches a result, a report or a refusal;
- `sort` by id.

For each site, either read `doc.order()`, or show that the order there
is only a deterministic tie-break and that no result, report or
refusal depends on it being document order. A digest order is still
deterministic within one document, so a site that only needs some
fixed order is fine as it is.

Pin each site that needed fixing with a row that is red under
reversed ids.

Filed by the EDIT orchestrator from PR 3676's fix lane.

## Closed

PR 3882 swept the workspace and its excluded roots. Pass 1 was syntax:
id-keyed `BTree*` iteration, sorts and min/max, comparisons, and the
`Ord`-deriving wrapper types. Pass 2 went from the claims and the
consumers: doc comments that promise an order, and id-keyed maps that
cross a crate seam. The full hit list is in the PR body.

Read off the document now, each pinned by a row that fails when ids
run out of document order:
- `split`'s `UnplacedAlone` group and `PartNameReachesRemainder.missing`;
- the STEP export's `Unplaced` parts;
- a part's `PartValue.unplaced`;
- `Evaluation::all_unplaced_below`;
- the viewer's `FusedGeometry.others`;
- the Python `node_map` lists.

Most other sites already read `Doc::order`/`Doc::positions` (the mate
solve, schedule, resolve lanes, roots), or are membership,
canonical-identity or contract-stated tie-breaks. The union pairwise
judgement's "lesser id as operand A" is one of the stated tie-breaks.

Filed: `work/chrome/display-prune-withdrawals-list-instances-in-id-order.md`.
