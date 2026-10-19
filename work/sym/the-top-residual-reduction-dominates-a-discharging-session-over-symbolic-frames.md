---
id: the-top-residual-reduction-dominates-a-discharging-session-over-symbolic-frames
kind: issue
title: With a frame's axes symbols, the top-residual reduction is 57-92% of a discharging Sym<f64> session and its memos hold 2.4-4.6 GB
status: open
opened: 2026-10-15
priority: P2
cost: M
refs: [revalue-rows-exhaust-memory-with-profiles-elaborated-at-sym]
---

Found by INLANE's `revalue-rows-exhaust-memory-with-profiles-elaborated-at-sym`
(PATHS' #5068). A corpus document evaluated at `Sym<f64>` with every
continuous variable a symbol, in a session that discharges each decision as
it is taken (`with_session_rules` at the drive's budget, 4096 terms / degree
128), costs minutes and gigabytes. The identity tier is the whole cost: the
same build in a deferring session (`sym::deferred`) or at a zero-term budget
takes under 3 s and about 100 MB on every row document.

The mechanism. A corpus frame's six axis components are free variables
(`test_support::frame` mints each through `scl`). The sketch plane is
minted at the lane by Gram-Schmidt (`wire::frame_axes`), so each profile
point is a rational function of nine symbols over two `sqrt` atoms. The
predicates over several points multiply these forms up to the term budget.
Most of them then freeze, after the walk and the reduction have paid for
them.

Measured with `revalue_corpus::revalue_cost_profile` (release, `p0` build
only, `REVALUE_DOC=<name>`):

| | loft_prism | face_sketch |
|---|---|---|
| decisions | 1031 | 1486 |
| wall | 61 s | 149 s |
| peak RSS | 4.6 GB | 2.4 GB |
| DAG nodes | 65 225 | 15 696 |
| forms built (plain + early, both origins) | 79 921 | 23 766 |
| largest form | 4088 terms, degree 84 | 3809 terms, degree 94 |
| freezes | 7508 (Terms) | 2513 (Terms, Degree) |
| top-residual reduction (`algebra::reduce` in `rungs`) | 360 calls, 35.1 s | 834 calls, 136.5 s |
| plain + early walks, decision origin | 7.0 s | 6.5 s |
| plain + early walks, assertion origin | 17.9 s | 5.3 s |
| per-node reductions refused at Terms | 28 calls, 11.6 s | |

- **The top reduction is not memoized** and runs on every ladder that
  reaches it. That includes the inexact witness's dispute check on every
  definite decision (`Decide for Sym<T>`, `Witness::Inexact` arm, which
  calls `contradicts`), so most of the population pays it and none of it
  can prove anything.
- **Memory is the session's form memos**: about 80 000 forms of 200
  terms each on loft_prism.

Who pays:
- The coincidence door does not: it replays in a deferring session at
  256 terms. `boss_union`'s door replay takes 131 ms.
- `revalue_corpus` no longer does: it replays in the same deferring
  session.
- A drive pays whenever its box binds a frame's axis components, and so
  does any `Sym` session that discharges as it decides over such a box.
  This is pre-existing: the drive elaborated profiles Guided before the
  pinned lift retired.

Options, for the tier's owner:
- (a) Bound the top reduction per decision, by steps or by the size of
  its input form, refusing typed as the per-node reduction already does.
- (b) Memoize the top reduction by node id per session.
- (c) Skip the top rung on the contradiction path at an inexact witness,
  where the answer is a counted dispute and never a fold. That changes
  what `theorems_disputed` can count.
- (d) Bound the forms memo per session (`RETRY_FORMS` style), trading
  recomputation for memory.

(c) is a receipt-semantics question. (a), (b) and (d) are cost dials and
need measuring against the M10 pins.
