---
id: pick-a-wide-but-informative-barycentric-wins-over-the-transversal-neighbour
kind: issue
title: a near-coplanar candidate whose barycentrics are informative but wide wins on t over the transversal neighbour at the aimed vertex
status: open
opened: 2026-09-16
---


## The finding

**A ruling before it is a unit.** `ray_triangle`
(`crates/editor-core/src/resolve/pick.rs`) now refuses a barycentric
whose interval covers `[0, 1]`, which removes every candidate whose
`u` or `v` says nothing at all. It does not remove a candidate whose
barycentrics say something WIDE — and the tie-break that picks the
winner compares `t` alone, with no notion of how well either candidate
places it.

The row's own numbers, pinned by
`a_wide_but_informative_candidate_answers_before_the_rings_aimed_vertex`
(`crates/viewer/tests/index_memo.rs`). On the gallery ring after its
bump, the `−y` ray through the tube vertex
`(0.24519632010080758, 0, 0.04877258050403218)` at `reach = 1.48`:

- the winner is a flat face of the ring with `det = 1.66e-19` and
  conditioning `2.7e-11` — the ray lies in that triangle's plane to
  eleven digits. Its intervals are `u = 0.367 ± 0.466`,
  `v = 0.459 ± 0.218`, `u + v = 0.827 ± 0.684`: all inside the closed
  range, none covering it, so the acceptance takes it. It answers
  `t = 1.4487652724897624`, `0.031` short of the vertex;
- two triangles that cross the ray TRANSVERSALLY (`|det| ≈ 2.8e-5`,
  barycentric bounds near `1e-14`) answer `t = 1.48` to the bit — the
  vertex, exactly — and lose the tie-break, because `1.4488 < 1.48`.

So the pick answers the worse-conditioned candidate whenever its noise
happens to point toward the viewer. `main` answered this class with
its own noise (`1.4694` before EDIT-PICK), so nothing regressed here;
nothing closed either, and the acceptance change is why it is worth
stating now — it is what is LEFT once the uninformative candidates are
gone.

## The two shapes, neither ruled

- **An interval on `t`, with the tie-break deciding overlap.** The
  barycentric bounds already give one: the hit point moves by up to
  `err_u·|e1| + err_v·|e2|`, so `t` has a width. Two candidates whose
  `t` intervals overlap are not ordered by `t`, and the tie-break — not
  the rounding — should choose, by the same rule it uses for an exact
  tie. This is the shape that keeps a genuine near-tangent hit.
- **Refuse a candidate whose `t`-interval is wider than the extent of
  its own bounding box along the ray.** Such a candidate cannot place
  the crossing anywhere within the box the tree already proved the
  triangle lives in, so it adds nothing the box did not already say.
  Cheaper, and it refuses rather than re-orders — but it needs the box
  at the test, which `ray_triangle` does not take, and the door that
  DID take a box entry parameter is the one EDIT-PICK withdrew
  (`docs/DOC-LEDGER.md`, "Per-merge deletion — EDIT-PICK's spec"), so
  the second shape owes that history an answer.

Either changes the answers `index_memo`'s differential pins and
`review_pick_r2`'s tally counts, and the first changes what the
tie-break is FOR, which is ratified ground. Ruling first.
