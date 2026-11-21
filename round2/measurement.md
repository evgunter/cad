# The measurement both first reports asked for (gate lane, 2026-11-21)

Instrumented on `origin/embed/gate-measure` at `95facbf3cb` (`CAD_GATE_WALK`, `CAD_GATE_M4`; no verdict changed). Population in `measure/gate/walk-population.json`, raw rows in `walk-rows.tsv` on that branch. Not the whole suite: the 4 tests behind the twelve dearest refusing asks (13 refusing asks); 150 refusing tests drawn by refusal shape, cheapest first within a shape (153 asks); the 22 tests behind the 38 asks that ended in a verified crossing after keeping a refusal (206 more refusing asks).

## M1. Where a refusing ask's seconds go

Seconds. "before 1st" is the spend before the ask's first refusal. After it: **a** = more cells of the face pair that refused first; **b** = face pairs that end certified; **c** = other face pairs that themselves refuse, split into the spend before each one's own first refusal (pre) and after it (post); **d** = cells still open when the budget ran out, which was 0 everywhere (a budget refusal comes at admission, cell by cell). "Stop the ask" saves everything after the first refusal. "Settle per pair" saves a + c-post.

| group | asks | total s | before 1st | a | b | c pre / post | stop the ask saves | settle per pair saves |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| the twelve dearest | 13 | 4,434 | 4.6 | 0 | 27.9 | 3,250 / 1,150 | 99.9% | 25.9% |
| departure at an edge | 89 | 2.8 | 2.5 | 0 | 0 | 0.1 / 0.2 | 10% | 7% |
| departure at a vertex | 24 | 0.7 | 0.5 | 0 | 0.1 | 0 / 0 | 19% | 0.5% |
| `Budget(Pairs)` | 19 | 41.0 | 33.3 | 2.0 | 0.1 | 0 / 5.0 | 17% | 17% |
| `Budget(Depth)` | 13 | 1.5 | 1.4 | 0 | 0 | 0 / 0 | 6% | 6% |
| second order | 8 | 0.2 | 0.1 | 0.1 | 0 | 0 / 0 | 37% | 37% |
| the refusing asks of the 22 crossing tests | 206 | 56.5 | 49.7 | 0.7 | 0.6 | 2.0 / 2.9 | 11% | 6% |

- **The twelve dearest** (torus bodies of 4 to 7 faces, 150 to 564 s each): all 13 asks first refuse at depth 0, within 3 splits, a departure at an edge. Then 4 to 21 OTHER face pairs refuse in their turn (departure and `Budget(Pairs)`), and that is where the time goes: most of it before each of those pairs reaches its own first refusal. The pair that refused first spends nothing more.
- **The cheap asks are the opposite**: their first refusal comes at the depth floor (median depth 40; 10 for the second-order arm), so most of their time precedes it and neither rule saves much.

## M2. How many pairs refuse

Distinct refusing face pairs per refusing ask: 1 pair in 126 asks, 2 in 75, 3 in 32, 4 in 56, 5 to 12 in the rest. By kind: departure at an edge 406 pairs, `Depth` 296, `Pairs` 206, departure at a vertex 67, depth and departure both 225. 38 of 136 crossing asks also hold a refusing pair.

## M3. The 38 asks that crossed after keeping a refusal

The crossing lies in the FIRST refusing pair in 19, in another refusing pair in 2, in a pair that never refused in 17. So settling per pair keeps 17 of the 38 crossings and reports 21 as refusals; stopping the ask at its first refusal reports all 38 as refusals.

## M4. Reading on after a crossing

136 crossing asks re-run walking on past the first verified crossing, the verdict kept: 7.8 s becomes 15.3 s in all (median ×3.0), every one still `Crosses`.

## For scale (the census, whole suite)

341,880 asks; 29,187 s of gate; 17,754 s on asks that hold, 11,434 s on asks that refuse. 1,076 tests refuse on bodies that are valid and the engine cannot yet read; engine units are in flight on them (the cone and torus buckets were traced today into nine causes).
