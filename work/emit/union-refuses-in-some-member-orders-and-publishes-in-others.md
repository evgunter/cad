---
id: union-refuses-in-some-member-orders-and-publishes-in-others
kind: issue
title: A union refuses in some member orders and publishes in others, over PR 3112's review corpus and the #3168 review fixtures
status: open
opened: 2026-09-24
priority: P1
cost: H
---


## What

The same union, with the same members and declarations, refuses in some
member orders and publishes in others. A document that fuses as written
can therefore refuse after the author reorders its members. The refusal
is loud, so there is no wrong answer, but the order still decides
whether there is an answer.

## Measured (EMIT, 2026-09-24, on `emit/rim-piece-ranks`)

Source: `crates/editor-core/tests/emit_union_rim_piece_ranks.rs`, whose
rebind row now checks this. It runs PR 3112's review corpus (as
`probe_corpus`) and the #3168 review fixtures, every order.

25 cases refuse in some orders and publish in others. The row pins
them in `KNOWN_MIXED`, so a new one, or a change in any of these,
turns it red.

| refusal | cases | owner |
|---|---|---|
| `DeclareResolve` (Vanished): a declared face the fold has consumed | `abg`, `abgids`, `abglow`, `abgg2`, `fam0{00,01,02,12,22}`, `fam1{00,01,02,12,22}`, `fam2{00,01,02,12,22}`, `r1flush`, `r2endsg`, part of `row`/`rowids`/`r4trig` | `work/gather/member-space-look-through-stops-at-splits-containment-and-fragmented-merges.md` (P0) |
| `UndeclaredContact`: `h` (x 1.0..1.1) meets `a`'s x = 1 wall flush when it is folded before `b` covers that wall | part of `row`, `rowids` | none; this row |
| `Boolean(Containment(RayExhausted))`: `c` (x 0.8..2.0), flush with both `a` and `b`, folded between them | `r4tri` (`[0,2,1]`, `[2,0,1]`), part of `r4trig` | `work/reach/a-contained-flush-operand-with-every-vertex-on-the-boundary-refuses-as-ray-exhausted.md` (P0) |

`UndeclaredContact` is a fold-order dependence of the contact check
itself: whether a contact is flush depends on whether another member
has covered it yet. `RayExhausted` is REACH's: the containment
fallback's vertex probe runs out when every vertex of the joining
member lies on the accumulation's boundary. It shares that shape with
`work/zip/two-parts-of-one-body-at-one-boolean-refuse-as-ray-exhausted.md`
but is not the same defect, because the operands here are distinct and
legal.

## The contact rule: pairwise, before the fold (Ev's direction on the `[ev]` PR; measured 2026-09-25, origin/main `2139eefa8e`)

**Where the check runs.** `wire_union` (`crates/editor-core/src/eval/wire.rs`)
hands each step's two operands, the accumulation and the joining
member, to the kernel pair verb (`run_pair`). The contact check is that
verb's census. The union adds no check of its own, so a contact is
judged against whatever the fold has accumulated. DM4 as ratified before this rule described exactly
this: "evaluates as a fold of the kernel's pair verb in member order",
and "two members that touch refuse `UndeclaredContact` exactly as a
pair boolean's operands do". Every contact the fold can see is between
two members, because the accumulation's boundary is made of member
faces. So a pairwise member-space check would see everything the fold
sees, plus the contacts another member covers. The dependence on order
comes from those covered contacts.

**Measured on `{a, b, h}` with `(a, b)` declared flush, all six orders.**
Scratch probe; the same pattern holds across the 24 orders of `row`.

| `(a.x1 wall, h.x0 wall)` | `[a,h,b]` `[h,a,b]` | `[a,b,h]` `[b,a,h]` | `[b,h,a]` `[h,b,a]` |
|---|---|---|---|
| undeclared | `UndeclaredContact` | fuse | `Vanished`: a face of `b` split by `h` (GATHER's row) |
| declared | `Emission` (seam-vertex parentage; `two-emitter-refusals-a-legal-declared-union-reaches`) | `Vanished`: `a`'s wall, consumed by containment in `b` | `Vanished`: a face of `b` split by `h` |

`a ∪ h` alone refuses `UndeclaredContact` in both orders and fuses in
both once the pair is declared.

**Ev's direction.** Every pair of members that touch must be declared,
even where the fold has already merged a third member over the
contact. Contact is judged pairwise in member space before the fold.
A declared contact that another member covers is satisfied, not
refused. DM4 in `crates/editor-core/REFERENCES.md` states this. The
wording waits on Ev's confirmation (`needs_ev`).

Rule 2, "a contact another member covers is not a contact", is
rejected. Ev's objection: it lets a set get out of declaring a contact
just because no single pair is blamed for it.

**Measured pairwise (scratch probe, every member pair of every flat
case in `emit_union_rim_piece_ranks.rs`, 153 two-member unions, with
the case's declarations for that pair).** Five pairs refuse
`UndeclaredContact`: `row` (a, h), `rowids` (a, h), `cross` (g, cross),
`r1three` (s1, s3) and (s2, s3). All the others fuse. `cross` and
`r1three` already refuse in every order today (`cross` with a mix of
four refusal kinds, `r1three` 24 of 24 `UndeclaredContact`). So the
pre-pass adds new refusals only to `row` and `rowids`, and turns
`cross` into a uniform `UndeclaredContact`.

**What moves once this is built.**
- Predicted: `row` and `rowids` undeclared refuse `UndeclaredContact`
  in all 24 orders, and they leave `KNOWN_MIXED`.
- Predicted: with (a, h) declared, they fuse in the 6 orders that fuse
  today. The 8 orders where `a` and `h` meet before `b` keep today's
  declared outcome: 2 `Emission` and 6 `Vanished`, the emitter row and
  GATHER.
- The 10 orders that refuse `Vanished` undeclared today stay
  `Vanished`, except where the consumed face is contained whole, which
  is now satisfied. Which of those 10 are containment and which are
  splits is not measured.

**Cost.** There is no contact-only door in the kernel. The
cross-operand `UndeclaredCoincidence` is raised inside the boolean
(`crates/topo/src/boolean/rest.rs`, `vtxfac.rs`, `recl.rs`). The
detector `topo::flush::find_flush_candidates` compares carriers over
every face pair with no region test, so it reports cosurface faces
that never meet. The pre-pass is therefore the pair verb run on each
member pair: at most n(n−1)/2 two-member unions, with pairs whose
closed boxes are disjoint skipped. The kernel's BVH prunes within each
of those pair booleans, but `wire_union` has no member-level box
pruning today, so that has to be added.

## Ruled (2026-09-25)

Ev ruled the pairwise contact rule on #3200. The `UndeclaredContact`
arm of this row is built by `union-contact-is-judged-pairwise-before-the-fold`.
The row stays open for the arms other programs own:
- `DeclareResolve` belongs to GATHER's look-through row;
- `RayExhausted` belongs to REACH's row.
