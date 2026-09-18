---
id: pick-a-wide-but-informative-barycentric-wins-over-the-transversal-neighbour
kind: issue
title: a near-coplanar candidate whose barycentrics are informative but wide wins on t over the transversal neighbour at the aimed vertex
status: closed
opened: 2026-09-16
closed: 2026-09-16
---


## The finding

**A ruling before it is a unit**, and the ruling is filed:
`what-t-the-pick-door-answers-and-with-what-width`, which this row is parked on. `ray_triangle`
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

- the winner is a flat face of the ring with `det = 1.66e-19`,
  conditioning `7.19e-16` and `|det| / bound_det = 5.72` — the
  candidate sits ON the certification's own noise floor, a few times
  its bound rather than a decade above it. Its intervals are
  `u = 0.367 ± 0.466`,
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

**This row is the EDGE of
`pick-refuses-a-crossing-within-rounding-of-a-plane`, not a separate
country.** That row records what the certification refuses: a ray
within rounding of the plane gets no answer at all. This one records
what happens one step to the admitted side of the same line —
`|det| / bound_det = 5.72`, a handful of times the bound rather than
below it — where the determinant's sign is vouched for, nothing else
is, and the door answers anyway because the barycentrics land in range
with intervals under `1`. The two rows should be ruled with one eye:
moving the certification's line moves this row's population, and
giving `t` a width moves it the other way.

## The three shapes, none ruled

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
- **Refuse where the interval covers at a DOUBLED bound** (review lane
  pick2-r2). It removes this winner — the sum's `0.827 ± 0.684`
  covers `[0, 1]` at `±1.368` — and keeps the candidate one behind it
  on the same ray, item 45 at `t = 1.5112`, whose sum is
  `0.633 ± 0.252` and still informs at `±0.504`. One line, and it
  answers this ray. What it is NOT is derived: a factor of two chosen
  because it separates these two candidates is the tuned constant the
  door's whole derivation exists to avoid, and `admits` would then
  read a bound nothing counts. It earns its place only if the doubling
  IS derived — the obvious candidate being the mesh's own coordinate
  error, which `crossing`'s bound explicitly does not cover.

Each changes the answers `index_memo`'s differential pins and
`review_pick_r2`'s tally counts, and the first changes what the
tie-break is FOR, which is ratified ground. Ruling first.

## Measured (EDIT-PICK3, 2026-09-16)

The first shape landed — `t` is an interval and the tie-break decides
overlap — and it resolves this class **where the two crossings are
closer together than the near-coplanar triangle is large**, which the
ring's own ray is not. The certified width is
`err_u·|e1| + max(err_u, err_v)·|e2|` over `|d|`: relative to the
triangle, not to the scene. The ring's triangles are `0.016` on a side,
so the winner's interval is `0.030` across and lies wholly before the
vertex `0.031` further on — it PRECEDES the transversal neighbours and
the tie-break never runs. The fixture
`a_wide_but_informative_candidate_answers_before_the_rings_aimed_vertex`
(`crates/viewer/tests/index_memo.rs`) now records that, with the
arithmetic that makes it true.

Where the intervals DO overlap the tie-break takes the better-certified
claim, which is what this row asked for: `tube_arc` at open, `+y`
through `(1.2534, 0.3843, −1.9521)`, where `main` answered
`1.4759_048_527_723_095` — `0.0041` short — and the door now answers
the vertex. Pinned by
`a_wide_candidates_interval_reaches_the_aimed_vertex_and_the_tie_break_takes_it`.
Over the wide aim's 441 126 rays the interval order moves 502 answers,
every one FARTHER, and gains 3 aimed vertices while losing none.

**What is left of this row** is the ring's own ray: a candidate at the
certification's noise floor, certified to a piece of the ray `0.03`
long, answering in front of a vertex `0.031` away. Nothing in the
derivation says that is wrong — the arithmetic vouches for the order —
so what would close it is a bound that covers the MESH's own coordinate
error, which `crossing`'s explicitly does not
(`crossing`'s "What the bounds bound"). That residue is scheduled as
its own row,
`pick-wide-candidate-needs-a-bound-over-mesh-coordinate-error`, with
the ring's numbers carried onto it; nothing else of this row remains,
so it closes at EDIT-PICK3's merge.

The corrected count, from the fix pass's re-measurement (2026-09-16,
`crates/viewer/tests/pick3_acceptance.rs`): the interval order moves
**503** of the wide aim's 441 126 answers, every one farther, and gains
3 aimed vertices while losing none. The `502` above was measured before
the early-out carried its derived margin and before the clamp's `v`
bound became exact.
