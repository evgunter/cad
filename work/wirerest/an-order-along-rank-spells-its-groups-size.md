---
id: an-order-along-rank-spells-its-groups-size
kind: issue
title: Qualifier::OrderAlong { rank, of } spells the group's size in the name, so a third same-sense crossing renames the first two
status: closed
opened: 2026-10-20
priority: P3
cost: M
refs: [naming-has-no-single-strategy-and-each-fork-patches-one-entity]
design: true
closed: 2026-10-26
---


Found independently by both FORK-NAMESTRATEGY designers (2026-10-20).
- `crates/editor-core/src/names/role.rs`, `Qualifier::OrderAlong`. Its doc
  says `of` is "How many crossings the ordering ranked". The producer is
  at `crates/editor-core/src/names/emit_topo.rs` ~4946; there are `of: 2`
  literals at `emit_union.rs` ~3599 and ~3763.
- A sibling count in a name breaks locality: N2 and N5 say a piece's name
  "holds no count". A third same-sense crossing renames the first two even
  when it lands after them along the line.
- Repair: drop `of` and keep `rank`. Row 73 ruled on the rank; the count
  was never ruled on separately. N5's group-size rung reads the recorded
  group, not the name.

## Not reachable from today's corpus (WIRE orchestrator, 2026-10-20)

`wire-nameinv` (PR 5460) probed every union, intersect and cut in its
corpus and found no `OrderAlong` name. With the planes and cylinders that a
profile extrudes to, a ranked group holds at most two crossings: a line
meets a plane once, and two circles, or a line and a cylinder, meet twice
with alternating senses. So no far feature can grow a group. A witness
needs a carrier a line meets three or more times with one sense (a torus, a
spline wall, a revolved profile with several bumps). Building one is part
of this row's acceptance.

## The build stopped at a fork (WIRE orchestrator, 2026-10-21)

`wire-orderalong` stopped before writing code. Two findings.

**`of` has a reader.** `names/canonical.rs` `RankRule::Reverse` computes a reversed rank as `of − 1 − rank`. It fires when a rewrite puts a seam line's pair in the other name order. There are two consumers:
- **The union's collapse** (`emit_union::collapse_one`). It has the fold table, so it could count the group itself.
- **The refactor split/inline remap** (`refactor.rs` `remap_name` → `canonical::rewritten`). This is a pure name function with nothing to count. `a_union_seam_vertex_follows_its_edge_parents_line` needs this reversal. This is ASM-4 ground, next to the D10 hold.

No count-free spelling was found. Recording which side a rank is read along gives two spellings of one vertex. The only orientation that never reverses is a geometric axis, which `seamrank/seam-line-ranks-keep-a-sided-axis-and-a-collapse-correction` proposed and Ev turned down.

**The defect is unreachable today.** A line enters one face of a plane, cylinder, cone, sphere or torus at most twice. A random search over circles against tori never found three entries. A wavy NURBS wall would reach it, but booleans refuse NURBS×plane (`CurvedPairUnsupported { kind: Nurbs, other_kind: Plane }`). A group of two is the most anything reaches, and the 1→2 step renames anyway, because a lone crossing carries no `OrderAlong`. The lane's probe is kept in the WIRE scratch notes; it was not committed.

**The fork, for whoever takes this up once a three-entry carrier exists:**
- **(A)** Drop `of`. The collapse counts from its fold table, and the refactor remap refuses loudly where it would reverse (a new `Unrankable` reason). This regresses a remap that is tested today, on ASM-4 ground.
- **(B)** Drop `of`, and pass an evaluated count into `remap_name`.
- **(C)** Keep `of`. N2/N5's "holds no count" and N0 NS3 then state the reversal as their exception. That is a ratified-text change, so it is Ev's.

It goes through the designer pair before Ev.

N5's group-size rung (`resolve::group_resized`) already reads the recorded group, not `of`; only its doc comment mentions `of`.

**Re-priced P3 with `design`.** Unreachable defects sit behind reachable ones. Moved to WIREREST.

## Moved to WIREREST (2026-10-21)

Re-priced P3 when the build found it unreachable today, with a fork; WIRE kept its reachable P2 rows. Signed (WIRE orchestrator).

## Parked (2026-10-26, tracker sweep)

The fork above is answered by a wider ruling: FORK-ORIENT (fork-log row
197; Ev, 2026-10-26, "cool, seems good!"; PR 5705, `964083e5`) rules
that nothing is ranked along a line, and lists `Reverse` and `of` among
what retires. N2 in `crates/editor-core/src/names/README.md` now reads
"Nothing is ranked: crossings, and vertices of one run, that their names
do not tell apart tie (N4)". So `OrderAlong { rank, of }` has no rank to
carry, and neither option (A)–(C) is needed. The code is unchanged on
main (`role.rs`'s `Qualifier::OrderAlong`, `canonical.rs`'s
`RankRule::Reverse`); WIRE's `a-unions-line-of-several-edges-is-oriented-by-its-least-edge`
builds the ruling. This row closes as superseded when that build
removes `OrderAlong`, and opens again only if a ranked spelling survives
it.


## Closed by PR 5758 (2026-10-26)

`wire/nothing-is-ranked` builds Ev's PR 5705 rule: an orientation is read,
never chosen, and nothing is ranked. `Qualifier::OrderAlong` is deleted with its `of`; N5's group-size rung reads the recorded group and stays. The status change is the
orchestrator's.

Closed by PR 5758 (2026-10-26, "nothing is ranked", Ev's ruling on PR 5705). `OrderAlong` and its `of` are deleted, and canonical's `RankRule::Reverse` with them. The fork dissolved.
