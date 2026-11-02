---
id: a-union-whose-fold-glues-a-run-no-judgement-opened
kind: issue
title: A union whose members glue a face run only together refuses: N2's third clause has no exact descent to read
status: spec
opened: 2026-11-02
priority: P1
cost: H
---


## What

N2's link rule (`crates/editor-core/src/names/README.md`, "A union's face
is named for its PARENT") has three clauses. The first two, a pairwise
judgement's merge (faces of one member included) and a judgement's cover,
are read by `emit_union`'s `Links::judged`. The third, "the finished body
holds area of both in one face, read exactly", is not built: a union
covers material, so its members together can expose and glue a run of
faces no two of them glue (a closed curved run, such as a sunk boss's
two-piece wall, that only several members together open). The union reads
a finished face's descent off the fold's lineage, which over-approximates,
so `Parents::of` refuses such a face as
`NamingError::UnsupportedUnjudgedGlue`, before the edge and vertex passes.

Building clause 3 means reading, per finished face, which member faces
it holds AREA of, exactly. Two candidate sources:

- **The kernel records it.** `topo::MemberLineage`
  (`crates/topo/src/boolean/nary.rs`) gives a piece of a face everything
  the face descends from, and a merged face the union of its constituents'
  sets, so it lists member faces a piece holds no area of, and it only
  grows along the fold, so a lineage link would make names depend on
  member order. The merge would record, per piece of a merged face, which
  constituents' area it holds.
- **A curved face-holds-point door**: read each member face's area at a
  point of the finished face (cf.
  `work/issues/a-face-holds-a-point-has-no-public-door.md`).

Then the edge and vertex passes need an audit once faces linked by clause
3 reach them: they assume every finished edge is a member edge's piece or
a seam between two parents. One shape that broke this is closed: a seam a
later fold step merged is spelled `Merged` of seams, and `name_by_parents`
now reads a `Merged` edge off every member edge as a seam. Its rows,
`emit_union_borders::a_cap_and_a_bar_over_a_sunk_boss_name_alike_in_every_order`
and `emit_union_borders::a_three_member_intersect_of_a_sunk_boss_names_alike_in_every_order`,
reached `Flush::meeting`'s `unreachable!` ("neither a member edge's piece
nor a seam") through clause 1 in 2 of 6 member orders before that. Still
open: a tied row whose candidates lie partly along member edges and
partly off them falls through `group_member_edges` and would reach that
`unreachable!` (no recipe witnesses it yet). Re-run the audit against the
witnesses below with their faces linked, each in every member order.

## Witness

`crates/editor-core/tests/wire_legal_union_refusals.rs`, each in every
member order, each refusing `UnsupportedUnjudgedGlue` and listing both
boss wall pieces:

- `a_flat_union_of_plate_boss_and_bar_refuses_as_unjudged_glue`: flat
  `Union[plate, boss, bar]`, the bar x 1.4 to 1.6, y 0.5 to 2.5, z 0.8 to
  2.0;
- `two_blocks_that_open_a_sunk_boss_wall_only_together_refuse_as_unjudged_glue`:
  `[Union[plate, boss], lo, hi]`;
- `two_bars_that_sever_a_sunk_boss_wall_only_together_refuse_as_unjudged_glue`:
  two bars over the north wall piece, z 0.8 to 1.25 and z 1.2 to 2.0.
- `two_half_bosses_and_a_bar_refuse_as_unjudged_glue`: the boss
  intersected with a south and a north block (y 1 to 1.5, 1.5 to 2) as two
  members and a bar z 0.4 to 1.6: a glue across members, not of one
  member's faces.

Each flips to naming when clause 3 lands, with one table in every member
order.

**One more thing for the audit (PR 6342 delta review, unsure).** `off_members` reads as a seam any `Merged` edge row that lies in neither `flush.edges` nor `flush.spans`. That would include a `Merged` of a member-edge piece and a seam lying partly along a member edge. For planar members this is argued impossible: a face ends at its own member's edge or under another member's material. Curved members are unchecked. No test reads such a mixed edge.

## Decided: the kernel publishes per-face holdings (2026-11-02, REFS orchestrator)

With Ev's per-face rule approved (PR 6381), this row is that rule's prerequisite. Two designers (fork `exact-descent-home`) converged after two rounds; nothing in it is Ev's to rule on.

**What is built.**

- **Who owns it.** The kernel's boolean doors publish `Holdings`: for each finished face, exactly the set of member (operand) faces whose area it holds, or a typed reason it cannot say. Naming reads `Holdings` and nothing of the fold. `topo::MemberLineage` leaves the public API and stays crate-private as the fold's verdict router (`given_pairs`), where a superset is harmless.
- **The pair door** (the subtract, and any two-operand boolean). It fills `Holdings` from its records alone, with no new geometry:
  - a pre-merge fragment (`face_fragments_a/b`) holds its root operand face;
  - it also holds the root of every dropped fragment a `substituted` row says it holds, and the dropped face an `uncut` row names;
  - a merged face (`merge_groups`) holds the union of its constituents.
- **The n-ary door**, per step:
  - member 0's faces hold themselves;
  - at the step joining member k, a fragment whose root is a member-k face holds that face;
  - a fragment whose root is an accumulation face A holds all of `H(A)` when `|H(A)| = 1`, and otherwise exactly the `m ∈ H(A)` that `overlap(fragment, m)` finds it meets in area;
  - the `substituted`/`uncut` arms read the same one-or-overlap rule;
  - merged faces union their constituents.

  Apply the same rule to each step's **discarded** fragments, so each member face's lost region is keyed exactly. `Borders`' obstacles (`editor-core`'s `borders.rs`, `Fold::obstacles`) then read that, not lineage.
- **`overlap`**, a new topo door, the public face-holds door that `work/issues/a-face-holds-a-point-has-no-public-door.md` asks for, generalised from a point to a face. It takes two faces that the verdicts already put on one carrier; it never re-decides coplanarity. It answers `Decided<bool>`: do they meet in positive area? It looks for a witness of three kinds:
  - a vertex of one face strictly inside the other (the existing containment doors: planar `contfp`, curved `curved_face_containment`);
  - a proper crossing of a boundary edge of each (in the carrier's chart where lines and arcs are straight; on a torus or sphere, through the carrier-cross doors);
  - a boundary point of one face lying on the other's boundary, read by sector as `boolean/sectors.rs` reads them (an inscribed triangle in a hexagon has no strictly inside vertex and no crossing).

  With no witness the answer is `false`. Every margin goes through `k_stats` at the operation's band, generic over the scalar. In band it returns a typed refusal, recorded on that face's `Holdings` entry. Spline carriers refuse typed. Where the three private copies of the point dispatch can retire into it, do so; where they cannot, say why.

**Order.** The holdings are order-free by induction: a piece lies inside its source face and can hold only what that face holds, and each reading is the geometric truth. Two things stay order-dependent, and the PR states them as known residuals beside `[operations.operators]` rather than hiding them:

- which pieces exist, and so where an in-band `overlap` refusal can fire;
- which member's carrier description a glued face keeps (DESIGN allows this).

**Rejected.** Rejoining each member to the finished body after the fold. Measured on this row's first witness, rejoining the plate refuses in all six member orders, at the boolean's containment frontier (`BooleanError::CoincidentShell`), because a union's member lies inside the finished body and never crosses it. Keep it as a **test oracle only**: an every-order test that compares step-by-step holdings with the rejoin's `substituted` reading wherever the rejoin builds.

**This unit.** It is the kernel half: `Holdings`, `overlap`, and both doors filling it, with rows for:

- the pair door's records (a sunk coplanar tile held through `substituted`);
- the n-ary door on the fold-only witnesses below and on the planar probe (bricks P [0,1]×[0,2]×[0,1] and Q [1,2]×[0,2]×[0,1] with flush tops, bar [0.8,1.2]×[−1,3]×[0.5,2], tile T [0.2,0.6]×[0.5,1.5]×[0.5,1] sunk under P's top; holdings left = {P.top, T.top}, right = {Q.top} in every member order);
- `overlap`'s witness kinds, including the inscribed triangle;
- the rejoin oracle.

The naming that consumes `Holdings` (union and subtract, the edge classification made total, `UnsupportedUnjudgedGlue` retired) is built in PR 6381 after this lands. Also re-check `VerdictsDisagree` once lineage is only a router, and record what was found.

Review tier: **dual** (kernel architecture every boolean's naming will read). Class H, so a concurrent pair.
