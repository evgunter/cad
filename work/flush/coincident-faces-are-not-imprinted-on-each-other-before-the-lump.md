---
id: coincident-faces-are-not-imprinted-on-each-other-before-the-lump
kind: issue
title: A same-oriented coincident face pair is lumped before either is cut along the other's in-plane edges, so the kept edge topology depends on operand order
status: dispatched
opened: 2026-10-19
priority: P1
cost: H
branch: flush/coincident-fate-cut
---


## What

Filed from PR 5153 (`[ev]`, EMIT), where Ev ruled out a record-only
stopgap and asked which program the imprint fits. The answer was FLUSH,
because this row is the structural fix under three of FLUSH's closed
rows, each closed with a record instead:
- `a-corner-pair-with-an-edge-in-the-partners-face-plane-…`, PR 4650;
- the L-prism top edge, PR 4650;
- `an-apex-edge-in-the-partners-face-plane-…`, PR 4737.

**The asymmetry.** For a same-oriented coincident face pair, the
boolean keeps one copy, operand A's, and drops the other uncut (Eq.
15.3's lump: `crates/topo/src/boolean/tables.rs`, `vtxfac.rs:757`).
Only the kept copy is cut along the other operand's in-plane edges. At a
vertex-on-face contact the kernel does no surgery at all: `vtxfac.rs:1108`,
"tangential touch: 3′ contact only, no surgery". So whether an operand
edge lying in the partner's face plane becomes an edge of the result's
topology depends on which operand is A.

**What reaches it.**
- EMIT's pocket floor
  (`work/borders/a-held-edge-wholly-inside-a-dropped-face-is-recorded-nowhere.md`,
  ignored test
  `editor-core` `emit_union_borders::a_pocket_floor_held_inside_a_dropped_face_names_alike`):
  `a ∪ b` and `[a, b]` refuse `Naming(Emission)`, while `[b, a]` names.
  The cause is that `b`'s top under the plate is one discarded fragment,
  never carved along the floor's outline.
- That row's three-box reproducer (`A`, `C` flush at x = 1, `M` covering
  the contact). It refuses in four of the six member orders.
- The tier-3′ contact records FLUSH's earlier rows had to carry by hand.
  With the imprint, the edge exists in both copies, so the record follows
  the topology.

## The shape to give

Before the Eq. 15.3 lump, imprint each face of a coincident pair with
the other operand's edges that lie in it. Each copy is then cut along
the union of both copies' in-plane edges, so the lump's choice of copy
no longer decides the result's edge topology. A vertex-on-face contact
whose edge runs in the partner's face becomes surgery, not a contact
record only.

**Owed by the unit:**
- Its paths. The surgery sits in `vtxfac.rs` and the sector lump
  (`sectors.rs`), which CLEAVE (`boolean/*`) and GERM/APEX
  (`sectors.rs`) also claim. Add `vtxfac.rs` to FLUSH's paths and
  coordinate on `sectors.rs`.
- Which of PR 4650's and PR 4737's records become derivable, and
  whether they retire.
- What the merge door then folds back. An imprinted edge with the same
  face on both sides is a redundant edge (FUSE's ground).
- Its pins:
  - EMIT's pocket fixture, un-ignored and naming alike in all three
    spellings;
  - the three-box reproducer building in all six orders;
  - `corner_pairs_battery`, `join1_delta_arc_battery` and the r2 pinch
    probes unmoved or better.

`design: true`: whether the imprint runs for every coincident pair or
only where a lumped copy would otherwise be dropped uncut is the
unit's opening question.

## Design (converged, 2026-10-19)

Two designers (Opus and Fable, fork labels on
`analysis/design-fork/flush-coincident-imprint`) converged in round
2. The design changes no ratified text and does not go to Ev: Ev's
ruling on PR 5153 already asked for the kernel fix.

**The rule.** A face is divided wherever the fate of its region
changes, and nowhere else. This is the same test every transversal cut
already passes, and it is `[topology.records]`' lineage clause (one
fate per cell: kept, substituted, divided or removed; Ev, fork-log row
117).

Over a coincident region a fragment's fate is one of:
- *kept*;
- *substituted* (the result holds its interior through the other
  operand's coincident copy);
- *removed*.

Today the dropped copy is thrown away as one uncut fragment, even
where a substituted region sits beside a removed one. `covered` then
states one fate for a piece that has two.

**The row's opening question closes.** "Every coincident pair, or
only where a copy is dropped" is the wrong test. The test is a fate
change.

**The final state.**
- **Where a cut is decided:** at the classifier's sites, where the
  lump's verdict and the neighbourhood's fates are both already known.
  - At a vertex-on-face site, today's "tangential touch, no surgery":
    a lumped coplanar sector `S` lies on the kept copy, and the
    pierced face's region under `S` is substituted while its region
    beyond is removed. The pierced face is cut along `S`'s bounds.
  - At a vertex–vertex site (where `HeldEdge` is written today): the
    same test on the dropped sector.
  - **Never cut:** a kept face beside more of its own kept region, and
    a dropped copy substituted over its whole face.
- **What the cut is:** a one-sided chord on the dropped copy, along
  the kept copy's existing edge, between vertices the reduction
  already mints. There is no null-edge pair. On a curved carrier the
  chord follows the edge's curve.
- **The lineage:** at the carve, the substituted fragment is written
  into `substitutions`, substituted by the kept copy. The removed
  fragment is an ordinary discard whose `bordered` reads the kept
  edge through that substitution. `BooleanNaming::covered`,
  `HeldEdge` and `DiscardRow::held` retire.
  - **Before deleting `held`:** enumerate every site that writes it
    and make sure each gets the cut.
  - **Naming:** the `names/README.md` N2 sentence citing `covered` is
    reworded to the substitution lineage (a renamed mechanism, so it
    is not a decision).
- **Symmetry:** the lump keeps operand A's copy, so the fates over the
  overlap swap with the operands. The finished body, the substitution
  rows, the discards' borders and naming's parent are the same in
  both orders up to that swap. The operand pieces themselves are not
  swap-identical. Example: a box sunk flush in a block gets a
  kept-beside-substituted cut on the block's top when the box is A,
  and the merge removes it. No reader sees the pieces.
- **Rejected:** imprinting both copies at the glue door. The glue door
  knows no vertices, so it would need a second, two-dimensional
  intersection engine in carrier coordinates. Every extra cut it made
  on a kept copy would be between two kept fragments. On a curved
  carrier the merge may skip such a cut, and a redundant edge would
  ship.

**Premise corrections, adopted.**
- **The three-box reproducer is not this row's.** In the four orders
  that refuse, no binary step meets a coincident pair. It goes back to
  EMIT's n-ary question.
- **PRs 4650 and 4737's tier-3′ records stay.** They record genuine
  edge-on-face touches with material on one side only. Carving them
  would leave a slit. They do not become derivable.

**Scope** (CLEAVE claims `boolean/*`; a crossing note is on CLEAVE's
log):
- `vtxfac.rs`: the tangential-site decision;
- `recl.rs`: the vertex–vertex twin; `held` retires;
- `insert.rs` and `join.rs`: the one-sided chord;
- `finish.rs` and `discard.rs`: the substitution row at the carve,
  and `bordered` through it;
- naming's `Obstacles`, which already reads substitutions.

`sectors.rs` should need little.

**Pins:**
- EMIT's pocket fixture
  (`emit_union_borders::a_pocket_floor_held_inside_a_dropped_face_names_alike`),
  un-ignored and naming alike in all three spellings;
- the sunk-box pair in both orders: the same body and the same
  parent/fragment content;
- a partial overlap of equal-height block tops in both orders;
- the apex-edge and L-prism fixtures unmoved (no slit);
- `boolean_covered` re-baselined (its held edge becomes a bordered
  one);
- `corner_pairs_battery`, `join1_delta_arc_battery` and the r2 pinch
  probes unmoved or better;
- EMIT's `HeldInto` entry-vertex residue closes on landing.
