---
id: a-legal-declared-union-reaches-the-seam-vertex-parentage-residue-emission
kind: issue
title: A legal declared union reaches the seam-vertex-parentage residue Emission once a covered contact is declared
status: closed
opened: 2026-09-25
priority: P1
cost: D
closed: 2026-10-17
---


## What

`name_boolean_edges`'s seam-vertex pass (`crates/editor-core/src/names/emit_topo.rs`,
the catch-all arm after `SeamVertexParentage`) refuses
`Emission("seam vertex parentage underdetermined from incident edges")`,
the class reserved for a kernel bug, on a legal declared union.
`two-emitter-refusals-a-legal-declared-union-reaches` (closed, PR 2688)
gave one arm of that pass a typed refusal (`NamingError::SeamVertexParentage`)
and left this catch-all as "the unenumerated residue". No open row owns
the residue, and a legal document reaches it.

## Measured (EMIT, 2026-09-25, on `emit/pairwise-contact`)

The review corpus's `row`: `a` = [0,1]³, `b` = x 0.5..1.5 over the
same y and z, `g` = x 0.3..0.4, y -1..2, z 0.5..3.5, `h` = x 1.0..1.1,
y -1..2, z 0.5..3.5; `(a, b)` declared flush on all four families and
`(a, h)` declared on `a`'s x = 1 wall against `h`'s x = 1.0 wall. The
orders `[a, h, b, g]` and `[h, a, b, g]` refuse this `Emission`: `a`
and `h` fuse first, then `b` joins across the fused wall. `{a, b, h}`
does the same in `[a, h, b]` and `[h, a, b]`. Pinned by
`emit_union_rim_piece_ranks::an_undeclared_covered_contact_refuses_in_every_order_and_declared_fuses_where_b_covers_it`,
`a_contact_b_covers_refuses_undeclared_and_is_satisfied_declared_where_b_consumed_the_face`,
and `emit_seam_junction::a_junction_is_named_the_same_in_every_order_that_fuses`,
which admits it in those orders only.

Before DM4's pairwise contact rule these orders refused
`UndeclaredContact` undeclared, and nobody declared the covered
contact, so the residue was reached only by the orders GATHER's row
lists. The rule makes declaring `(a, h)` necessary, so every author of
such a document now meets this refusal in the orders where the two
declared members fold first.

## Why it matters

It is the two-emitter row's argument again: the document is legal, and
`Emission` tells its author the crate has a bug. A typed refusal naming
the construction, or a naming rule for the shape, is owed; which is the
two-emitter row's own question.

## More orders once declarations stop refusing by order (EMIT, 2026-10-06)

Today `DeclareResolve` (`ConsumedByFold`) refuses first in most orders
and hides this `Emission`. Once that refusal goes (INTENT's stage 4
retires declarations; a scratch fan-out of the refused pair to every
descending face did the same), this row's measured orders grow:
- `row`/`rowids` with `(a, h)` declared: from 2 orders to 8 (0231 0312
  0321 2031 2301 3012 3021 3201);
- `emit_seam_junction`: from `{ahbg, habg}` (`SEAM_VERTEX_RESIDUE`) to 8 orders;
- `{a, b, h}` declared: stays at 2.

Measurement and fan-out: `work/emit/union-refuses-in-some-member-orders-and-publishes-in-others.md`,
"Re-measured on main (2026-10-06)".

## An undeclared witness: a quadrilateral void in a quadrilateral arch (TANG, 2026-10-07)

PR 4256 reaches the same `Emission` ("seam vertex parentage
underdetermined from incident edges") with no declaration. The
quadrilateral arch is the pyramid over `bearing(40|80, 0.45|0.25, 0.5)`
at `MEET`, and the void in it is `nest_polygon(arch, 0.7)`
(`topo::test_support::meeting`). The probe "over" is
`corners(50, 0.7, 0.5)` standing at `MEET`.

`x ∪ y` (over first) refuses at every pose of `meeting::poses`, at
ε = 1e-9:
- "over a quad void in a quad arch": the plate united with the arch,
  less the void. It builds from PR 4256 on; main refused it.
- "over a quad void in a bare quad arch": the arch less the void, pairs
  alone. It refuses the same way on main.

`y ∪ x` (the arch first) refuses the same way in both scenes, at every
pose, since PR 4249 (JOIN's one vertex per cone at insertion) let it
build; before that it refused `ClassificationInvariant` in the boolean.

The boolean's edge classes at the vertex hold against an analytic germ
in `x ∪ y`, and every op in both orders builds sound in both scenes at
every pose (`a_vertex_read_by_two_sector_passes::every_scene_builds_sound_or_refuses_typed_at_every_pose`),
so the refusal is the emitter's. `crates/editor-core/src/names/emit_topo.rs`,
`touch_reread_rows::no_rule`, allows exactly these cells and cites this
item.

## Reached undeclared (INTENT stage 4 PR E, 2026-10-10)

PR E glues `(a, h)`'s flush walls undeclared. In
`emit_shared_rim_several::no_order_of_the_probe_corpus_refuses_several_shared_rims`
only `(a, b)` is declared, and `row` and `rowids` now reach this
residue in `[a, h, b, g]` and `[h, a, b, g]`. That is the same pair of
orders the declared row measured. The test admits it by name
(`RESIDUES`).

## More orders reach it (stage 4 F, 2026-10-11)

With declarations retired, the member orders that used to refuse `DeclareResolve` first now run, and more of them reach this residue. `emit_seam_junction`'s four members: 2 orders → 8 (every order that folds `a` and `h` before `b`). `emit_union_rim_piece_ranks`' covered contact: 2 → 8. `row` and `rowids`: 2 → 8 each. In each of these the order refused before and refuses now, and no order that fused moved (`emit_shared_rim_several`'s 256 fused cells hold).

## Every witness builds (EMIT, 2026-10-12, branch `emit/touch-crossing`)

A vertex that is one of an operand's own vertices is now named as that
vertex at every operand key the boolean leaves at the point (the touch
ruling, PR 4284). In the orders that fold `a` and `h` before `b`, the
fold reaches `a`'s corners resting on `h`'s wall only through a null
copy, which is where the residue came from. Measured:
- `row` and `rowids`: all 24 orders fuse and publish one table
  (`emit_union_rim_piece_ranks::a_covered_contact_decides_alike_in_every_order`);
- `{a, b, h}`: all 6 orders fuse
  (`a_contact_b_covers_decides_in_every_order`);
- `emit_seam_junction`'s 8 orders fuse and publish one table;
- `emit_shared_rim_several` fuses 272 cells, up from 256;
- the quadrilateral arches of `touch_reread_rows` name at every pose.

No witness reaches the catch-all arm now. The arm itself stays, as the
unenumerated residue.

## No longer reached by the probe corpus (EMIT, PR 4949, 2026-10-13)

`emit_shared_rim_several` now fuses all 282 cells, up from 272. The
`RESIDUES` admission is gone, and any refusal fails the row. The
orders that reached this residue (`row` and `rowids` in
`[a, h, b, g]` and `[h, a, b, g]`, among others) now emit. The cause
was the `Borders` chase. It stopped at a seam that a later step's join
had killed, so the chase now follows `EdgeJoin` and each step's split
lineage. Whether any other document still reaches this row is WIRE's
call; it is left open here.

## Closed (2026-10-17, WIRE triage)

Read against the tree at `fd218851f9`. The row's own last sections found no witness reaching the arm; `SEAM_VERTEX_RESIDUE` and `no_rule` are gone, the sibling emit row is closed, and what remains in `emit_topo.rs`'s seam-vertex pass is the deliberate unenumerated-residue arm argued at its site.

Signed (WIRE orchestrator).
