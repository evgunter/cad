# Review of PR #3846, frozen head 00085e001d

Lane `reach-dual3846-r2`. Started 16:38 UTC, finished 17:30 UTC, 2026-10-02. No glimpse: I read no other lane's branch, scratch, report or PR comment. The PR was read with `pull_request_read get` only.

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 2 · NOTE 4. No wrong body anywhere. In P1–P4, 272 cells built or came back empty, and every one matches my oracle; so does every cell of P5 and P6, at all three ε. The fixes are test rows.

## Exercise (DEMONSTRATED BY EXECUTION: `probes/r2_3846_probes.rs`, mounted as a module of `crates/sweep/tests/all.rs`)
Each pose runs in both orders through ∪, A∖B, B∖A and ∩. The oracle is my own: closed-form slab volumes (area wh − 4(1−π/4)r², plus the z-overlap term), and an analytic rounded-rectangle membership test. 200–300 jittered `point_in_solid` samples per built body are checked against that membership, with a 1e-6 margin at the boundary. Every built body also passes tier 3 and tier 3′.
- **P1.** Sharp over rounded and rounded over sharp, r ∈ {0.01, 0.1, 0.25, 0.5, 1, 1.5, 1.9, 1.999}: head 128/128 correct, main 64/128 (refused in one order only).
- **P2.** Mismatched radii (0.1/1.9, 1.9/0.1, 1/1.5, 0.3/0.5) and thin or thick plates (t = 0.01, 3, 0.05) all correct.
- **P3.** ×1e-3, ×1e3, and three rigid rotations (about z, about (1,2,3) by 0.7, and x by π/2) all correct, sampled in the mapped frame.
- **P4.** Offset footprints (dx = −3, ±0.25, −5.5, −5.499) and a sunk stack overlapping 0.5 in z: no wrong cell, and every refusal is identical in both orders.
- **P5.** The result reused as an operand (a rounded plate on top, a sharp one below), both orders, every op: correct. Main refuses building the base union.
- **P6.** A declared `Tangent` box beside the fillet at four heights, declared and undeclared: correct or refused, the same in both orders.
- Suites `topo` + `sweep` (3968 rows, my probes included): green at ε 1e-9 and 1e-6. At 1e-12 the only red is the known `rigid_map_near_eps_plane_nurbs::the_certificate_re_derives_within_rounding_under_the_map`.

## Claims
1. **Hold-then-settle is sound:** not falsified. A held pair is accepted only if the ordinary arms clear or record every fragment. A pierce, or a touch still inside a fragment, returns the typed refusal. `split_edge` keeps the key on the first child (`crates/topo/src/split.rs`, the parent edge is repointed to the first-child curve), and the chain check fails loud (`reduce.rs:1471-1491`). Splits by both operands, edges split twice, and splits at the touch (P1–P5) are all correct. By inspection, the hold replaces only `None` from `Placement::declared` (`reduce.rs:1885`, where `[None, None]` with an uncleared interior was a frontier), and `LiesOn` keeps its frontier. Confidence: likely (the reasoning is inspection; the bodies are executed).
2. **Order no longer decides whether it builds:** confirmed. All PR poses, and my widened ones, are symmetric across orders (P1–P5); main is not (P1 64/128, P5).
3. **The hold only widens what the other operand's vertex resolves:** confirmed. P9 puts the declared box at z ∈ [0.25, 0.75], so its edges graze the fillet mid-ruling where the plate has no vertex. It refuses `CurvedPierceUnsupported` in both orders, as on main. The undeclared graze refuses in both orders (P6).
4. **README:** the change describes the code. `git log -S'recorded at its endpoints'` finds the clause in `[ev]` commit 62af8f98. That commit never said a mid-edge covered touch refuses; that was only the old `reduce.rs` doc. Recording still happens at endpoints, on the fragments.
5. **Mutants (`probes/r2_3846_mutants.py`):** M1 (hold disabled) and M4 (the covered-line `(Pos,Pos)` arm not held) are red in both PR rows and P5. The settle-path mutants are not: see MINOR-1.
6. **Filed items** are measured honestly. P7 reproduces filed item 1's payload exactly (`face 4v1, edge 2v1`, operand A or B by order). P8 reproduces item 2: `CurvedBooleanUnsupported` for ∪ and ∩, both box directions, both orders.

## Findings
**MINOR-1. The settle path has no row that can go red.** `reduce.rs:1439-1497`. DEMONSTRATED BY EXECUTION: three mutants pass every topo and sweep row, the PR's included.
- M2: `settle_held` returns `Ok` without reading any fragment.
- M7: a fragment the arm reads as `Interior` is accepted (`:1469`).
- M3: only the leading fragment is read (`:1471`).

Under M2 and M7, the unsplit declared mid-ruling box (P9) builds a body. It passes tier 3, tier 3′ and the volume check, where the head refuses. So no gate catches it, and the refusal that claim 3 rests on is unpinned. P9 goes red under M2 and M7, and should land as a row. M3 stays green even with P9. I found no pose where a later fragment keeps an interior touch: in every extruded stack, the touch carries a vertex of the other operand. So the walk past the first fragment is unexercised. sure.

**MINOR-2. Two of the three hold sites are reached by no row.** `reduce.rs:1885` (the covered circle) and `reduce.rs:2188` (`(Pos,Pos)` on a torus, or along any circle). DEMONSTRATED: I planted a panic at both sites, and all 3968 topo + sweep rows stayed green. The PR body discloses only the torus site as unreached. Both sites can only add acceptances through settle, so neither can ship a wrong body without MINOR-1's path. sure.

**NOTE-1. An offset stack's union refuses with a refusal nobody filed.** With dx = ±0.25 (the sharp plate shifted 0.25 in x over the r = 0.5 plate), the head refuses `Join(UnpairedLooseEnds { count: 8 })` in both orders. On main that order refused `CurvedPierceUnsupported`, and the other order refused `Join`. The other three ops build correctly. dx = −5.5 refuses `Join` in both orders on main and on the head. Not this PR's defect, but no work item exists for it. DEMONSTRATED (P10). sure.

**NOTE-2. A near-equal-radius stack escalates in both orders, on main and head.** For r 0.5 over 0.5001, `Escalated { Containment, bool_contact_arc }` with a margin of 9.998e-9 against the 1e-8 band. That is an honest band refusal. DEMONSTRATED (P10). sure.

**NOTE-3. CI's `test` job is red on this exact SHA.** Run 37033060639, head 00085e00. The 400-line log tail I could read shows only passes. The PR attributes the red to `pncad` `the_north_star_audit…`; I did not verify that. My local topo + sweep runs at all three ε are reported above. unsure about the cause.

**NOTE-4. The trace's `accepted` channel never sees pairs accepted at settle.** `reduce.rs:1053` drops `Interior`, and `settle_held` pushes nothing to the trace. So the sweep-testing differential (`mod.rs:2876`) is blind to settle-accepted pairs. Inspection. likely.

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q7 and Q8 (I read `settle_held` and the `curved_face_arm` covered arms end to end, not the whole of `reduce.rs`).
- **Q1/Q7.** The sequence "sweep A, sweep B, settle" is written out four times: `mod.rs:2876`, `:2936`, `:3053`, and `coplanar_conic_rows.rs`. The PR justifies this with a gate count ("a combined driver would add a Bounds site"). That is a gate shaping the code, the fourth copy is one more place a future change has to be made by hand, and the justification sits in the PR body, not at the code. likely.
- **Q3.** This is MINOR-1 in style form: the rows prove only "builds". Nothing proves "refuses when unsplit" or "reads every fragment". sure.
- **Q2.** The doc for the `:2188` arm and the PR body both call it the torus arm. Its guard is `!on_line || Torus`, so it also holds covered circles that reach it. I could not tell whether any covered circle can get past `:1885` to reach it. unsure.
- **Q4.** The old `reduce.rs` doc, "a tangency in the middle of an edge does too [refuse]", is updated. `rg 'middle of an edge|mid-edge' crates` finds no other kernel citation of the old premise; its hits are unrelated uses or the updated rows. The work items are updated too. likely.
- **Q7.** `HeldPair.refusal` stores a fully built error at hold time, and `end` is redundant with walking to `start(he_minus)` of the original edge. Both are fine, just heavier than needed. unsure.
