# BOXFINISH M8 — content keys per node over a drive, the split axis against the deciding node's reads (2026-11-22)

Headline: on the tour's certified study, 87% of the node build time outside finishes and 91% of the finish time is spent on
a node whose content key the drive has already seen; the plate reads 1 distinct key in 1,023 replays; 292 of the 511 splits
are on an axis the deciding node does not read, and a tree that split only axes in the deciding node's read set would have
127 replays, not 1,023. No pair of replays has a node with equal content keys and different symbolic ids in its result.

## Tree, probe, how it was measured
- Same tree as M7 (probe + `origin/main` ae076e0507, switch's box arm off), S1 ON (`BOXFINISH_S1=1`), plus one probe
  commit: `BOXFINISH_M8=1`. Build as before (`CARGO_INCREMENTAL=0`, own target dir). Report commit: this file's.
- `[m8-replay]`, printed in `drive::classify` inside the symbolic session, once per replay: the leaf's box (every axis's span),
  and the deciding node, taken as `classify_replay` takes it: the first node in evaluation order, not poisoned, that failed or
  holds an escalation.
- `[m8-node]`, one per document node per replay: state (ok / failed / poisoned), the content key (`eval_node`'s
  `content_key(..)` after `members::member_key`, as it is computed today; for a FAILED node the key it computed before its op
  ran, which no `NodeValue` holds), the op's wall time (`wire::run_op`) and the part of it inside finishes
  (`AtRestBody::validate`, summed on the thread), and for an ok node the sorted multiset of every `SymId` in the debug text
  of its `payload`, hashed, with how many of them are `SymOp::Opaque` in the session.
- The replay tree is rebuilt from the boxes: a replay's parent is the replay whose box equals its own on every axis but one,
  where it is twice as wide and contains it; that axis is the split axis. Every non-root replay found exactly one parent.
- A node's **read set** is measured, not read from the document: the smallest set of axes such that the node's content key
  is a function of the box restricted to them, over the drive. It covers reads through inputs, because the key hashes the
  upstream keys. It can under-read an axis the drive never split.
- Drives: the tour (`demo-tour certified`, 3 axes: ax0 read by both holes, ax1 by the first only, ax2 by the second only (inferred: the spacing and the
  two radii); 1,023 replays, 0 certified, every replay refusing at the holes); `m10_8_the_a0_set_is_a0_alone` (the same plate,
  2 drives of 1 replay each); `docm9_range::the_first_boundary_stops_the_walk_and_a_seed_on_the_island_certifies_it` (1 axis,
  2 drives, 178 replays); and, added because the a0 drive never bisects, `m10_3_driver_interval::
  the_two_parameter_drive_certifies_after_bisection_and_accounts_for_all_of_it` (2 axes, 31 replays).

## 1 and 4. The tour: distinct keys per node, the time on repeats, and where the repeat sits
| node (evaluation order) | state in all 1,023 | distinct keys | repeats | op time: build outside finish / finish | of it on repeats | repeat matches the PARENT / only elsewhere | read set |
|---|---|---|---|---|---|---|---|
| 12 | ok | 1 | 1,022 | 0.4 s / 0 | 0.4 / 0 | 1,022 / 0 | none |
| 20 (the plate's extrude) | ok | **1** | 1,022 | 6.0 s / 15.5 s | 6.0 / 15.5 | 1,022 / 0 | none |
| 25 | ok | 127 | 896 | 0.2 / 0 | 0.2 / 0 | 584 / 312 | ax0, ax1 |
| 29 (first hole's extrude) | FAILED | 127 | 896 | 37.7 s / 20.0 s | 32.2 / 17.5 | 584 / 312 | ax0, ax1 |
| 33 | ok | 127 | 896 | 0.2 / 0 | 0.2 / 0 | 292 / 604 | ax0, ax2 |
| 37 (second hole's extrude) | FAILED | 127 | 896 | 33.9 s / 17.1 s | 29.1 / 15.0 | 292 / 604 | ax0, ax2 |
| 77 | ok | 1 | 1,022 | 3.0 / 0 | 3.0 / 0 | 1,022 / 0 | none |
| 47, 62, 87, 91 | poisoned | no key | | 0 | | | |
- The plate reads 1 distinct key in 1,023 replays: yes (measured). Its 1,022 repeats are 6.0 s of build and 15.5 s of finish.
- The holes: 254 distinct (node, key) pairs over 2,046 runs, as the designers said: 127 each. Each refuses every time, so a
  memo keyed as today, which stores a `NodeValue`, holds nothing for them: what it would save as it stands is nodes 12, 20,
  25, 33 and 77 (9.8 s of build, 15.5 s of finish). Keeping a refusal under its key would add 61.3 s of build and 32.5 s of
  finish (measured as the time spent on runs whose key was already seen; the refusal's payload was not compared across them).
- **Parent against anywhere:** nodes that read no axis always match the parent. A hole node's repeats match the parent in
  584 of 896 (first hole) and 292 of 896 (second hole); the rest, 312 and 604, match only a replay elsewhere in the tree. So
  a memo handed from parent to child catches 4,818 of the 6,650 repeats and misses 1,832 (916 of the 1,792 hole-extrude repeats); one shared
  by the whole drive catches all of them.

## 2. Share of a replay's build outside the finishes that is of nodes whose key repeats
- Tour, all nodes, op time: build outside finishes 81.4 s, of it 71.1 s (87%) on a key already seen; finishes 52.6 s, of it
  48.0 s (91%). Counting only nodes that build (what today's key could serve): 9.8 of 81.4 s (12%) and 15.5 of 52.6 s (29%).
- Against the whole drive: M7 summed 214.9 s of leaf time on this binary with S1 on; the node ops are 134 s of it. The other
  ~81 s is outside `run_op` (each node's work before its key, the session, the classification) and was not split here.
  So "the other two thirds is repeated as well" holds for the part inside node ops (87%); it is unmeasured for the rest.

## 3. The split axis against the deciding node's read set
- Tour: the deciding node is node 29, the first hole's extrude, in every one of the 511 bisected replays (read set ax0, ax1).
  Splits: ax2 292, ax1 146, ax0 73. **ax2 is not in node 29's read set: 292 splits, 57%, are on an axis the deciding node
  does not read.**
- The read-set tree: keeping only the splits whose axis the deciding node reads, the drive's replays collapse to **127**
  distinct paths against 1,023 (measured by collapsing the traced tree; inferred as the size such a driver would reach,
  since node 37 would become the deciding node only if node 29 stopped refusing, which it never does here).
- Two-parameter drive (31 replays, 15 splits): 3 splits are on an axis outside the deciding node's read set (node 13 reads
  ax0, split ax1); read-set tree 23 against 31. docm9 (one axis): 0 of 88; nothing to save.

## 5. The key at the symbolic scalar
- **Equal content key, different symbolic ids in the result: 0 pairs**, in every drive. Tour: 5,115 ok results (nodes 12,
  20, 25, 33, 77) over 257 distinct keys; each key's results carry one and the same multiset of `SymId`s (the plate: 225
  ids, 139 distinct, one hash in all 1,023). docm9: 250 ok results, every key distinct, so nothing to compare. Two-parameter:
  31 results over 7 keys, 0 pairs.
- **Opaque ids:** no ok result holds an `Opaque` node in any of these drives (0 of 5,115 on the tour), so "the same id across
  replays" has no instance to test.
- What this compares: the node's RESULT payload. A node's evaluated inputs are its upstream results (compared, as nodes of
  their own) and its slot values, which were not hashed apart. Failed nodes have no result to compare, and those are the
  holes, the only nodes here that read a box axis and finish a body.

## Contrast drives, items 1, 2 and 4
- a0 plate: 1 replay per drive, 10 nodes, 10 keys, no repeat inside a drive.
- docm9: node 11 has 178 keys in 178 replays, node 20 has 129 in 129 keyed runs: no repeats, 0% of 1.3 s build and 3.6 s
  finish. A one-axis drive whose nodes all read the axis has nothing for a memo.
- Two-parameter: node 13 (reads ax0) 7 keys in 31 replays, 24 repeats (20 match the parent, 4 only elsewhere), under 0.1 s;
  node 21 (reads both, fails every time) 31 keys in 31; build 4.6 s and finish 1.5 s with nothing repeated.

## Not measured, and why
- The certifying tour: on this tree it certifies nothing, so every figure is of a drive that refuses at the holes.
- The ~81 s of tour leaf time outside node ops; the trace timed `run_op` only.
- Whether two failed runs under one key refuse with the same payload.
- Symbolic ids of slot values and of anything a `NodeValue` holds besides its payload (`outputs` is private to the evaluator).
- Read sets from the document's own reads; the measured ones are functional dependence of the key on the box.
