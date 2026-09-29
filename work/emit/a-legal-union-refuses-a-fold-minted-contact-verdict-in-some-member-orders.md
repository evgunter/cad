---
id: a-legal-union-refuses-a-fold-minted-contact-verdict-in-some-member-orders
kind: issue
title: A legal three-member union refuses as a fold-minted contact verdict (UNION_FOLD_CONTACT_VERDICT) in some member orders and fuses in others
status: closed
opened: 2026-09-25
priority: P0
cost: M
branch: emit/fold-contact-verdict
closed: 2026-09-29
---


## What

A union whose every touching member pair is declared (or needs no
declaration) refuses in some member orders with
`Naming(Emission { "a union fold step minted a contact verdict the pairwise judgement did not" })`
and fuses in others. That message is `UNION_FOLD_CONTACT_VERDICT`,
raised by `fold_step_refusal` (`crates/editor-core/src/eval/wire.rs`)
when a fold step's pair verb refuses `UndeclaredContact` or
`UndeclarableContact` after `judge_pairwise_contact` passed every
member pair. So either the pairwise judgement misses a contact the fold
step meets, or the fold step reports a contact no member pair has. By
DM4's contact rule (#3200, built by #3213) the second should not
happen, so this is a legal document refused as an emission bug.

## Repro (found by the dual review of PR 3143, on its head `24c97a729`)

Blocks as `docm7_union_declare::block(doc, x, y, z0, h)`:

- `a = block((0, 1), (0, 1), 0, 1)`
- `s = block((0.2, 0.4), (0, 1), 0.5, 1)` — rises through `a`'s top cap
  across its whole depth
- `big = block((-1, 2), (-1, 2), 0.8, 1.4)` — contains `a`'s top cap
  and the part of `s` above z = 0.8

Declared pairs: `a.wall(0)` against `s.wall(0)` and `a.wall(2)` against
`s.wall(2)` only (`fixture::wall`). Order `[s, a, big]` refuses as
above; order `[a, s, big]` fuses. Re-measured on PR 3143's fix head
(main at `cf0001b9b` merged in), all six orders:

| order | outcome |
|---|---|
| `[a,s,big]`, `[s,big,a]`, `[big,s,a]` | fuse |
| `[s,a,big]` | this `Emission` |
| `[a,big,s]`, `[big,a,s]` | `SeamVertexParentage` (`two-emitter-refusals-a-legal-declared-union-reaches`) |

Over the reviewers' four-member fixtures (the same three plus a `p`
resting on `a`'s cap, `p = block((0.6, 0.9), (0.2, 0.8), 1.0, 0.2)`,
declared against `a.cap(End)`), the refusal appears in 10 of 24 orders
(`[s,a,big,p]`, `[s,big,a,p]`, `[s,p,a,big]`, `[big,s,a,p]`,
`[p,s,a,big]`, …). The second reviewer's repro: R1's split fixture
(`docm8_flat_merged::split_fixture`, members `a`, `c`, `s`) plus
`big = block((-0.5, 1.2), (-0.5, 1.5), 0.8, 1.2)`; orders
`[s,a,big,c]`, `[s,big,a,c]` and `[big,s,a,c]` refuse.

The refusal is identical with PR 3143's fold-consumption arm disabled
(measured by the reviewers), so the fold-consumption refusal does not
cause it; every refusing order folds `s` before `a`, so `a` joins an
accumulation that already holds `s`.

## Owner

`wire.rs` is WIRE's territory (`work.py territory`), but the rule the
refusal guards, and the pairwise pre-pass it trusts, are EMIT's
`union-contact-is-judged-pairwise-before-the-fold` (#3213), so it is
filed here.

## Root cause (EMIT, 2026-09-29): neither pairwise nor the union's

Neither of the two readings above holds. At `b4e425801a` (the first
parent of #3377) every refusing order's fold step refused
`UndeclaredCoincidence` on a pair of faces that are **both operand A**:
two same-oriented coplanar faces inside the accumulation, e.g.
`[s,a,big]` step 1, `[(A, 3v1), (A, 4v3)]`. No member pair has such a
contact. The step that had just joined `a` to an accumulation holding
`s` glued nothing. The declared y-walls overlap in area, not flush, so
the step left the two faces as separate rows, and the next boolean's
census refused them. That is
`area-overlap-contact-admitted-but-unmerged-refuses-at-the-next-step`,
which CONTACT-8 carried. `fold_step_refusal` then reported it as a
fold-minted contact verdict.

CONTACT-8 (#3377) fixed it at the merge: `merge_coplanar_faces` now
prunes the dangling seam and glues the group. A first-parent bisect
from #3143's merge to `f207b7e118` puts the change at #3377's merge
`e056f1affb`. On main no order of any of the three fixtures refuses
this way. The rows are
`wire_legal_union_refusals::no_order_of_…_refuses_a_fold_contact`, which
cover every order of all three fixtures. Each one is red at `b4e425801a`
on this refusal.

## Closed — fixed by #3377, pinned by PR 3476

The refusal was not a contact that the pairwise judgement missed. The
step that joined `a` to an accumulation holding `s` left the declared
y-walls unglued: they overlap in area rather than meeting flush. The
next boolean's census then refused that same-operand pair, and
`fold_step_refusal` reported it as fold-minted.

CONTACT-8 (#3377, `merge_coplanar_faces`) glues the group. Three rows in
`wire_legal_union_refusals.rs` now pin every member order of the three
fixtures against an outcome table:

- the three-member fixture;
- the four-member fixture;
- R1's split fixture plus `big`.

All three rows are red at `b4e425801a`.

The `SeamVertexParentage` orders remain. They belong to
`work/wire/a-merged-face-with-several-same-side-constituents-has-no-chord-rule.md`
and `two-emitter-refusals-a-legal-declared-union-reaches`.
