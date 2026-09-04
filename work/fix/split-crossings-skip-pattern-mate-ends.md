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

## Closed — premise refuted, alignment kept

Not "fixed": no behaviour changed, because the defect this item names
does not exist. What landed is a predicate unification that forecloses
a different, reachable hazard.

### 1. The stated defect does not exist

The item says a split whose seam severs a pattern-headed mate would not
carry that mate as an interface crossing. **No such split exists.** A
pattern-placed head IS a member of A11's vocabulary, so the mate is a
proper A12 edge; `mate::clusters` has resolved heads through `head_of`
since PR #1400, so it welds the mate at the pattern's INPUT instance,
and `TornCluster` refuses every cut that would put its two ends on
opposite sides. AQ8's unreachability argument — the one
`asm_r2b_assembly::row5_a` executes for a plain instance-instance edge
— covers a pattern-headed edge for exactly the same reason.

Executed against the four-legs-one-top document (`leg`, a linear
`Pattern` of it, a `top`, a seat mate from copy 2 onto the top):

```
clusters:      [[leg, top]]
reading_edges: [(mate, leg), (mate, top)]

cut {leg,pattern}       => Err(TornCluster { gauge: leg, instance: top, gauge_is_cut: true })
cut {leg,pattern,mate}  => Err(TornCluster { gauge: leg, instance: top, gauge_is_cut: true })
cut {top}               => Err(TornCluster { gauge: leg, instance: top, gauge_is_cut: false })
cut {top,mate}          => Err(TornCluster { gauge: leg, instance: top, gauge_is_cut: false })
cut {pattern}           => Err(SeveredEdge { consumer: pattern, input: leg, consumer_is_cut: true })
```

So the skip cost nothing: no record was lost and no pin-move
re-verification was missed. The load-bearing step is that `classify`
refuses a straddling payload name, so `!inside(name)` means DISJOINT
from the cut rather than merely not contained — a head's derivation set
and its member instance therefore always land on one side together.
Disclosed limit: this is a proof over the checks `split` runs, executed
against four constructed cuts, not an exhaustive search of cut sets.

The `refactor.rs` comment that asserted the opposite — that a
pattern-placed end "contributes no crossing record, and so loses the
pin-move re-verification the record buys" — was wrong on its
consequence and is corrected in the same change.

### 2. The reachable hazard is the opposite one, and the obvious fix creates it

A NESTED pattern head (a pattern of a pattern) is OUTSIDE the member
vocabulary — `head_of` refuses it `DanglingHead` — so it welds no
cluster, both members stay singleton clusters, and its mate's two ends
DO reach opposite sides of an ACCEPTED cut:

```
clusters:      [[leg], [top]]
reading_edges: [(mate, top)]            <- no edge at the nested head
cut {leg,inner,outer} => Ok, crossings: []
```

This is currently the one head kind whose mate can straddle an accepted
cut. A gate matching a head's SPELLING mints a crossing there — for a
mate that never solved, which AQ8's (b)-SKIP ruling forbids as
trusted-at-rest state.

**The plausible fix would have created the defect this item feared.**
Mutant M1, the second match arm a reader of the item would write —
`matches!(doc.node(name.node), Some(Node::InstantiatePart { .. } | Node::Pattern { .. }))`
— is EXECUTED and KILLED by
`a_nested_pattern_head_reaches_the_seam_and_still_contributes_no_crossing`.
Mutant M2, `remap_seg`'s `R::Instance { i: *i }` shifted to `i: *i + 1`,
is killed by the index row (`Instance { i: 3 }` against the expected
`i: 2`, node ids remapped, the `InPart` argument crossing verbatim), so
the index is pinned and not merely its presence.

### 3. What landed: one home for a predicate that had three copies

`is_mate_edge_end` no longer spells the member vocabulary at all. It
asks `mate::member_of_head`, the predicate `head_of` was split out of,
so the split seam's crossing collector, A12's reading edges and A11's
placement clusters admit exactly one set of heads and cannot drift.
The vocabulary had three copies before: `head_of`, this collector, and
the viewer's `is_instance` pick gate (issue 1412, viewer ground, still
open).

That identity is what makes the unreachability argument in (1) TOTAL
rather than a coincidence of two definitions that happened to agree in
one direction, and it forecloses the hazard in (2) by construction
rather than by a second arm someone must remember to keep in step.

**Correct under either outcome of the open ruling.** PR #1731 asks
whether the member vocabulary should extend through nested pattern
heads. Ruled OUT, the nested head keeps refusing and this gate keeps
skipping it. Ruled IN, nested heads become members, weld clusters, and
the straddle in (2) stops being reachable by that route — and because
the gate IS the vocabulary rather than a copy of it, the collector
follows the ruling with no second edit. Nothing here waits on it, and
nothing here settles it: `nested-pattern-mate-heads-refuse` (1411) is
untouched, the nested case is neither accidentally better nor
accidentally worse, and the new row pins today's answer at this door
only.

### A4's recorded map, since the item asks for the remap

The map is a correspondence between NODE ID SPACES and nothing else.
`Instance(i)`'s `i` is not in its domain and must not be rewritten:
the `Pattern` node moves into the part with its `count` and `kind`
slots verbatim, so copy `i` denotes the same copy on both sides. An `i`
that moved would name a copy the rule never mints.

### Owed to S-MATE: issue 1405's premise is wrong

**This correction is for S-MATE, routed by the FIX orchestrator; it is
recorded here rather than filed into `work/mate/`.**

Issue 1405 (and the MATE-1 sweep report behind it, PR #1400) states
that the fix "needs the `Instance(i)` remap through split's node maps
(A4's recorded-map contract), not just a second match arm." Both halves
are wrong, and the root cause is one inference:

- The sweep inferred a remap REQUIREMENT from a reachability that the
  AQ8 addendum had already refuted. A pattern-headed mate is a proper
  A12 edge, and the addendum's argument for proper edges — welded
  cluster, `TornCluster` — applies to it unchanged. The crossing the
  remap would serve is never minted.
- `remap_seg` already handled `Instance { i, of }` correctly before
  this change: it rewrites `of` and preserves `i`. There was no remap
  to add.
- A second match arm is not merely insufficient, it is HARMFUL: the
  obvious spelling admits nested heads, which weld no cluster and can
  straddle an accepted cut, minting a never-solved record (mutant M1,
  executed).

Other units may be reading that sweep report, so the correction is owed
whether or not this PR lands.

### Shape sweep

Pattern: `Node::InstantiatePart` matched as a predicate on a mate
reference's HEAD (`doc.node(<name>.node)`), plus every
`Node::InstantiatePart` match site in `crates/*/src` and `pncad/src`.

- `refactor.rs:1225` `is_mate_edge_end` — **this unit**.
- `mate/solve.rs:164,170` `head_of` — the vocabulary's home;
  **refactored** into `member_of_head`, behaviour unchanged.
- `mate/solve.rs:400` `clusters` — the cluster graph's VERTEX set is
  instances by definition; its head resolution already goes through
  `head_of`. Correct, not this unit.
- `mate/solve.rs:747` `ClusterMaintenance` — cascade over mate and
  instance NODES, not name heads. Not this unit.
- `viewer/src/display.rs:194,212` `is_instance` — the matetool pick
  gate excludes the very heads the A11 rider admits. Already filed as
  issue 1412 (MATE-1's R2 reviewer); viewer ground, not this unit.
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
- It swept only the Rust doors of the Python and GUI surfaces, not a
  head predicate expressed in those languages.
- It is accurate as of merge base `main`; a lane landing a new head
  predicate before this merges is not covered.
- The unreachability claim in (1) is a proof over the checks `split`
  runs, executed against four constructed cuts, not an exhaustive
  search of cut sets. Its load-bearing step is `classify`'s refusal of
  a straddling name; a future arm admitting a straddling payload name
  would reopen it.
