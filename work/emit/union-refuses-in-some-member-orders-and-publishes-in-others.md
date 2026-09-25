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

## The contact refusal needs a ruling (measured 2026-09-25, origin/main `2139eefa8e`)

**Where the check runs.** `wire_union` (`crates/editor-core/src/eval/wire.rs`)
hands each step's two operands, the accumulation and the joining
member, to the kernel pair verb (`run_pair`). The contact check is that
verb's census. The union adds no check of its own, so a contact is
judged against whatever the fold has accumulated. DM4 describes exactly
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

**Why this is a fork and not a wrong operand.** Neither order-free rule
follows from ratified text alone.

- *Judge pairwise in member space* (the literal "two members that
  touch"). `row` would refuse `UndeclaredContact` in every order. The
  recourse DM4 offers is to declare the pair, and a declaration refuses
  in every order: the containment case of the GATHER row, which Ev
  ruled "refuse, no offer" (PR 2677), plus the Emission row. The
  document would then fuse in no order under any declaration. To avoid
  that, the pairwise rule also has to change DM4's routing ("each pair
  is fed to the fold step at which both its sites are in the
  accumulation"), so that a declared contact another member has
  consumed is accepted rather than refused.
- *A contact another member covers is not a contact.* `row` would
  refuse `UndeclaredContact` in no order; the `Vanished` orders remain
  GATHER's. The fold cannot evaluate this at the step
  that refuses today, because `b` has not been folded yet. It needs a
  pass over all members before the fold, or a fold that defers contact
  refusals to the finished body. Both depart from "a fold of the pair
  verb in member order". The union would also fuse orders whose
  pairwise chain refuses (`(a ∪ h) ∪ b`), so "exactly as a pair
  boolean's operands do" would no longer describe a step.

Which one DM4 means is Ev's call. The recommendation is the second.
The finished union is the same body either way, a covered flush
contact leaves no trace on its boundary, and under the first rule
`{a, b, h}` fuses in no order today, with or without the declaration.
Once the Emission row is fixed it would fuse, declared, only in the two
orders where `a` and `h` meet before `b`, which is order-dependent
again.
