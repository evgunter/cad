# EDIT-PICK2 — the exact test accepts only barycentrics that carry information (spec)

Binds one unit. Item:
`work/edit/pick-accepts-uncertified-barycentrics-on-a-certified-determinant.md`,
which also closes `pick-closed-acceptance-loses-a-graze-to-rounding`.
Branch: `edit/pick-barycentrics`. Deleted post-merge with a
`docs/DOC-LEDGER.md` entry citing the merge SHA.

Read the item first (it carries the measurement and three shapes
costed), then `crates/editor-core/src/resolve/pick.rs`'s `ray_triangle`
and `certified_determinant` as EDIT-PICK left them (PR #2721), then
`docs/DOC-LEDGER.md`'s entry for EDIT-PICK's spec — the last spec on
this door was wrong at its centre, and this one owes you the same
suspicion. Then `crates/viewer/tests/index_memo.rs` and
`crates/viewer/tests/review_pick_r2.rs` (the two instruments).

## The ruling (orchestrator, 2026-09-16, per the plan's posture on
## sequencing decisions with a recommendation)

Of the item's three shapes, the first (certify inside) kills every
genuine vertex and edge graze and is rejected. The second (accept the
interval) alone does not close this item: a candidate whose
barycentric error bound is 100 has an interval meeting `[0, 1]` and is
still accepted on noise. The third (refuse the uninformative) alone
still accepts a rounded value that the interval reaches outside on
one side. **The ruling is their conjunction, both halves from one
derived bound `err` on `u` and on `v`:**

- **meet**: `[u − err_u, u + err_u] ∩ [0, 1] ≠ ∅`, likewise for `v`,
  and for `u + v` against `[0, 1]` with the summed bound — the closed
  contract with its rounding made explicit. A genuine graze whose
  true `u` is `0` and whose rounded `u` is `−1e-14` is accepted on
  every incident triangle, which closes the graze-loss row.
- **inform**: the interval does not cover the whole admissible range
  (`u − err_u > 0` or `u + err_u < 1`; likewise `v`; likewise the
  sum). An interval covering all of `[0, 1]` says nothing about
  whether the point is inside, and the door refuses to answer from
  nothing — the same posture as the certified determinant. Refused
  candidates are answered by their better-conditioned neighbours or
  not at all; a miss is honest where a face was a guess.

`err` is DERIVED at the site from the operation count (the numerator's
forward bound plus `|u|·bound_det`, over `|det| − bound_det`), the
same style as `DETERMINANT_ERROR_UNITS`, never tuned. No other
constant enters.

**Why this is not the spec's trap.** EDIT-PICK's spec forbade a
tolerance because a tuned `ε` moves tie behaviour by fiat. A derived
rounding bound is not a tolerance: it is the statement of what the
computed number can and cannot certify, and it moves nothing that the
arithmetic did not already move.

**Rejected: leaving it filed.** 203 wrong faces in 441 126 rays is a
per-thousand rate of confidently wrong picks on the corpus, and
`main` had the same class. The item is small, the instruments exist,
and this is the natural second unit on the door.

## What I verified

Against the tree at `origin/main` after PR #2721 merged. Re-derive.

1. The item's example (ring after its bump, `−y` through
   `(0.24519632010080758, ·, 0.04877258050403218)`) answers the flat
   face at `t = 1.4488` today with a barycentric error bound near
   100; the aimed vertex is at `1.48`. Under the ruling that
   candidate refuses at `inform` and the vertex's triangle answers.
   Your first row is this ray, red on main's kernel, green after.
2. `review_pick_r2`'s tally row pins four counts with its re-derive
   command; the ruling changes the "candidates refused" and "rays with
   a refusal" columns. Re-baseline from the run and list old and new
   in the PR (`implementer-discipline.md` §3).
3. The tie-break row (`tie_rays_for`) aims at shared points: under
   `meet` the 273 aimed rays that "answer beyond or miss their aimed
   point" on main should answer AT the aimed point. That is the
   graze-loss row's closing measurement; if any of the 273 still
   misses, say why, per ray class.

## Acceptance

1. The example ray, red on main, green after (the vertex, `t = 1.48`
   to the bit).
2. The graze-loss row: a row over the tie-break aim asserting every
   aimed ray answers at its aimed point (with the count that would make
   it red), or the per-class reason a residue remains, as its own file.
3. `Pruned == Every` stays green on every landing (order independence
   is not traded).
4. The exact test's closed-boundary pins (`one ULP each way`) are
   re-stated for the interval: a hit exactly on a boundary, one ULP
   outside with a bound that reaches it (accepted), one ULP outside
   with a bound that does not (refused), and an uninformative
   candidate (refused) — each with a mutant it kills (drop `meet`,
   drop `inform`, halve the bound).
5. `review_pick_r2`'s tally re-baselined; a row that any accepted
   winner has an informative interval (zero winners with bound ≥ 1
   over the wide aim), with the count.
6. The bound's derivation at the site, in the `DETERMINANT_ERROR_UNITS`
   style; the doc of `ray_triangle` gains one paragraph, not four.

## Fence

`crates/editor-core/src/resolve/pick.rs` (EDIT's); the two viewer test
files by announcement, as EDIT-PICK did; nothing in `viewer/src`,
nothing in `bvh`.

## Stop clauses

- The conjunction refuses an aimed graze on a watertight mesh that no
  neighbour answers (a hit becomes a miss on the tie-break aim): stop,
  push, report the ray and the intervals.
- The bound cannot be derived without a constant you would tune.
