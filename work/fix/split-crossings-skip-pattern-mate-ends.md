---
id: split-crossings-skip-pattern-mate-ends
kind: issue
title: Split interface crossings skip pattern-headed mate ends (is_mate_edge_end lacks the member vocabulary)
status: review
branch: fix/split-pattern-mate-ends
opened: 2026-08-31
github: 1405
refs: [1400]
pr: 1749
---

## From GitHub issue 1405

Opened 2026-08-31; 0 comments.

Found by MATE-1's class sweep (PR #1400, the A11 member-vocabulary rider — genus: mate-head kind dispatch). Not fixed there: the fix is split/refactor ground, outside that unit's fence.

`crates/editor-core/src/refactor.rs`'s `is_mate_edge_end` recognizes only plain `InstantiatePart` mate ends when collecting the split seam's interface crossings. With the rider landed, a mate may head a pattern-placed instance (`Pattern` + `Instance(i)`), and such an end is skipped — so a split whose seam severs a pattern-headed mate would not carry that mate as an interface crossing. Per the MATE-1 sweep report, the fix needs the `Instance(i)` remap through split's node maps (A4's recorded-map contract), not just a second match arm.

Scope note: A4/refactor territory adjacent to ASM-XSPLIT (the banked AQ8 conversion door). Whoever takes either should take both views of the seam into account.

Signed: (S-MATE orchestrator)

## Home

`work/mate/` — S-MATE's charter names assembly composition (mates × patterns, the instantiation seam), and this is the member vocabulary of a pattern-headed mate end; the refactor.rs site itself is in no open program's territory.

## Closed

`is_mate_edge_end` no longer spells the member vocabulary at all: it
asks `mate::member_of_head`, the predicate `head_of` was split out of,
so the split seam's crossing collector, A12's reading edges and A11's
placement clusters admit exactly one set of heads and cannot drift.

**The issue's premise is refuted where it is specific.** A cut that
severs a pattern-headed mate edge does not exist. A pattern-placed head
is a member, so the mate IS an A12 edge; `clusters` (already on
`head_of` since PR #1400) welds it at the pattern's INPUT instance, and
`TornCluster` refuses every cut that would put its ends on opposite
sides — the AQ8 argument that makes a plain edge's crossing unreachable
covers a pattern-headed one for the same reason. Executed in all four
directions in `fix_pattern_mate_crossing.rs`. So the skip cost nothing:
no record was lost, and there was no red to show. What the change buys
is that the collector's gate and A12's edge notion are now ONE
predicate, which is what makes the unreachability argument total and
what a future ASM-XSPLIT conversion door needs to be able to rely on.

**The reachable defect is the opposite one.** A NESTED pattern head is
outside the vocabulary, welds no cluster, and its mate's ends DO reach
opposite sides of an accepted cut. A gate matching a head's SPELLING —
the plausible second match arm, `InstantiatePart | Pattern` — mints a
crossing there for a mate that never solved, which AQ8's (b)-SKIP
ruling forbids. That mutant is executed and killed by
`a_nested_pattern_head_reaches_the_seam_and_still_contributes_no_crossing`.
Asking the vocabulary forecloses it by construction. Nothing here
settles `nested-pattern-mate-heads-refuse` (1411): the nested head
still refuses `DanglingHead`, unchanged, and the row pins today's
answer at this door only.

A4's recorded map is a correspondence between NODE ID SPACES and
nothing else. `Instance(i)`'s `i` is not in its domain and does not
need to be: the pattern node and its rule move into the part verbatim,
so copy `i` denotes the same copy on both sides. Pinned with the index
asserted, not merely its presence.

### Shape sweep

Pattern: `Node::InstantiatePart` matched as a predicate on a mate
reference's HEAD (`doc.node(<name>.node)`), plus every
`Node::InstantiatePart` match site in `crates/*/src` and `pncad/src`.

- `refactor.rs:1225` `is_mate_edge_end` — **fixed** (this unit).
- `mate/solve.rs:164,170` `head_of` — the vocabulary's home; **refactored**
  into `member_of_head`, behaviour unchanged.
- `mate/solve.rs:400` `clusters` — the cluster graph's VERTEX set is
  instances by definition; its head resolution already goes through
  `head_of`. Correct, not this unit.
- `mate/solve.rs:747` `ClusterMaintenance` — cascade over mate and
  instance NODES, not name heads. Not this unit.
- `viewer/src/display.rs:194,212` `is_instance` — the matetool pick
  gate excludes the very heads the A11 rider admits. Already filed as
  issue 1412 (found by MATE-1's R2); **not this unit**, viewer ground.
- `viewer/src/session.rs:3025`, `viewer/src/combine.rs:420`,
  `viewer/src/tree.rs:150` — display/tree presentation of instance
  nodes, no mate head involved. Not this unit.
- `refactor.rs:928,991,1430,1546`, `edit.rs:1668,1701`,
  `persist/check.rs:722`, `update.rs:107,172`, `node.rs` (7 sites),
  `eval/*` (3 sites), `pncad-py/src/py/doc.rs:431,444` — all match a
  NODE the caller already holds by id (cut membership, roots, pin
  targets, wire arms). None reads a name's head. Not this unit.

### What the sweep could not match

- It matched `Node::InstantiatePart` textually, so a head test written
  through a helper that hides the constructor — a `Doc` method, a
  `matches!` on a bound `node` variable named elsewhere, an
  `if let Node::Pattern` arm reached without the instance test — is
  invisible to it. The `mate/solve.rs:747` hit was reached this way and
  read by hand; there may be others outside the two crates swept.
- It did not sweep the Python or GUI surfaces for a head predicate
  expressed in those languages, only their Rust doors.
- It is accurate as of merge base `main`; a lane landing a new head
  predicate before this merges is not covered.
- The unreachability claim is a proof over the checks `split` runs, run
  against four constructed cuts, not an exhaustive search of cut sets.
  Its load-bearing step is that `classify` refuses a straddling name,
  so `!inside` means disjoint; a future arm that admitted a straddling
  payload name would reopen it.
