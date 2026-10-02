# Review of PR #3814, frozen head 08708983c7

Lane `reach-dual3814-r2`. Wall clock 05:59–07:02 UTC, 2026-10-02. **Verdict: NOT-MERGEABLE-AS-IS** (one MAJOR, and the fix is small). Counts: MAJOR 1 · MINOR 3 · NOTE 3. No wrong body found in 1,300+ executed rows. Glimpses: none. I read only the PR body via `get`, never its comments or reviews, and fetched no `analysis/reach-dual/*` branch but this one.

## Findings

**MAJOR-1: no row observes the crossing layer, so the certificate behind the new silent no-event is untested.** `carrier_cross.rs:61-151`, `reduce.rs:1413-1419,1543-1546,2294-2306`. DEMONSTRATED BY EXECUTION, `sure`:
- **Forced `Clear`.** I made `boundary_crossing` return `Clear` on entry (env `MUT_CLEAR`). All 3,888 `topo`+`sweep` tests stay green, and so does the lily row `the_curved_rungs_declare_the_socket…`, including every `full_turn_bore_mate` row at all 3 azimuths × 2 spans × 6 poses.
- **Forced `Unread`.** The same site returning `Unread` turns 5 rows red, so the path is live.
- **Where crossings come from.** Instrumented, the PR's rows return `At` 2,790 times, every one from a boundary-vertex candidate (vertices other pairs had already split in). Across all of topo+sweep, the closed-form meeting candidates return `At` 5 times, all in `mate7a_torus_rest` rows that refuse anyway.

So what unblocks the mate is `Placement::declared` reading all-`Elsewhere` as `None`. The item's premise, that the crossing is "recorded by nobody", is false for these fixtures: with zero crossings found, every body comes out right (`refusal-text-is-not-cause`).

The widening is licensed only by `interior_clear`. No row goes red when that certificate is a lie, and that certificate is the subject of root cause 1 (review policy: an untested subject blocks). The unit rows test `meetings`, never `boundary_crossing` on a body.

I tried to build a fixture where only the crossing layer could see the crossing: a rim next to a convex torus round, or next to chamfer cones (P7). Every case refused typed earlier (`CurvedPierceUnsupported`, `CurvedPairUnsupported` at the operand gate). Inspection of the closed forms is in claim 1 below.

**MINOR-1: the union depends on operand order, and the failing order is not filed.** `shaft ∪ collar` refuses `Merge(Pcurve{LoopDiscontinuity})` at the merge stage (`rest.rs:360-362`) where `collar ∪ shaft` unions:
- "through" at azimuths 17°, 60°, 119.99°, 180° and 300°;
- spans flush at one rim only, at every azimuth including 0°, and at scales ×1e-3 and ×1e3.

DEMONSTRATED (P1, P5), `sure`. It is not a regression. On the merge base the same cases refuse at earlier doors, and the arc-split class's swapped order already refused `JoinDesync` there (P6 on both trees). Still, "unions at any azimuth" holds in one order only.

**MINOR-2: the vtxfac declared-`Rest` gate is pinned by nothing.** `vtxfac.rs:389`. With `if declared_rest` mutated to always-true (`MUT_GATE`), all 3,888 topo+sweep tests stay green. The comment's "an undeclared planar one reaches here and keeps this door" has no row. DEMONSTRATED, `sure`.

**MINOR-3: the ∩/∖ claim isn't asserted.** `full_turn_bore_mate.rs:159-188` accepts any `Err` and only `eprintln!`s it, so "refuses `FallbackExtentUnsupported`" (PR body, filed P1 item) is pinned nowhere. The row also runs at the identity pose and azimuths 0°/60° only. By inspection; my P3 measured `FallbackExtentUnsupported` on every ∩/∖ at 0°/60°/90°. `sure`.

**NOTE-1: the blind-shaft filing is narrower than measured.** `work/zip/blind-shaft-in-a-full-turn-bore-revisits-the-seam-vertex.md` names `ChordEndpointRevisited`. At 0°, a shaft blind from ABOVE, or one wholly inside the bore, refuses `RestZipUnsupported{ChordBetweenIsolatedPierces}` instead. Typed in all 36 blind/inside rows (P3). DEMONSTRATED, `sure`.

**NOTE-2: the misaligned probe still reports instead of asserting.** `mate2_r1_probes.rs:35`: the PR body cites "now UNIONS, additive to 2.7e-15", but the row cannot go red (Q3). `sure`.

**NOTE-3: the retired row's comment called it the only cargo guard for `Undecided`.** `mate2_r2_probes.rs:210` (re-pinned) said it was "the unit's only `cargo test` guard that `Undecided` keeps the typed frontier". After the flip, that is guarded only at rule level (`undeclared_rule_rows`), with no end-to-end row (Q4). `likely`.

## Claims

1. **Candidate set: complete, by inspection, for line/circle pairs** (`sure`):
   - line × circle uses the plane hit;
   - line × line uses the closest point on line 1 (a skew candidate then fails the pre-pass, which is harmless);
   - crossing-plane circles use the common-line point `((h1−h2c)n1+(h2−h1c)n2)/(1−c²)` with unit axes, plus the sphere of circle 1;
   - coplanar circles use the radical chord;
   - `param_near` for a circle stays within ±π of mid, which covers any span ≤2π.

   `Unread` is returned for spiric/NURBS/ellipse carriers, uncertified boundary curves, a line parallel to a circle's plane, and `param_near` = None. One gap (`unsure`): near-parallel pairs read `Zero` inside the band are treated as exactly parallel. Line×line is levered at a fixed 1 m (`carrier_cross.rs:261`), which is not scale-relative.
2. **`Placement::declared`: sound both ways, by inspection** (`sure`). `Elsewhere` is a certified `Out` with no vertex hit (`reduce.rs:2268`). Ends outside plus no boundary meeting in the open span means wholly outside. `[R,E]` with a clear interior is consistent (the crossing is at the end). Sound only if `Clear` is true; see MAJOR-1.
3. **Mirror chords: none survive.** Across 198 bodies I checked tier 3, the census, and 300 `point_in_solid` samples per body against my analytic oracle. Results carry exactly the expected 9 line edges (P5). `MUT_MIRROR` (skip the mirror) turns the 3 `full_turn_bore_mate` rows and `r2_full_period_bore_unions` red. DEMONSTRATED, `sure`.
4. **vtxfac gating matches C4.** Main's C4 has `Rest` as opposed senses, and continuation is merged (#3662/#3736): only a verified `Rest` reaches the carrier ladder; an undeclared curved sector refuses at :362. Unpinned (MINOR-2). `likely`.
5. **Bodies** (own oracle: closed form + Monte Carlo PIS; `probes/reviewer_3814_r2.rs`):
   - **P1** (radii 0.25/2.0; scales ×1e-3/×1/×1e3; azimuths 0/17/45/119.99/180/300; 4 spans; up to 6 poses; both orders): 198 right, 114 refused, 0 wrong at ε 1e-9 and 1e-6; 196/116/0 at 1e-12.
   - **P2 misfits** (eccentric 0.1/1e-7, slim/fat 0.49/0.51, tilt 0.05/1e-6, every op): all `ContactContradicted` (AxesApart / RadiiDiffer / AxesNotParallel). Tilt 1e-9 is `Escalated`. In-band hairs (ecc 1e-11, r−1e-11, tilt 1e-12) union right. 0 wrong at all 3 ε.
   - My first 1e-6 run showed 9 "wrong" rows. They were my oracle's margin sitting inside the band (`OnBoundary` is correct there); I widened it and re-ran.
6. **Moved pins: legitimate** (`sure`). The r2 re-pin asserts union, the r1 doc is fixed, the four `offer_rows::SITES` rows match the new decision sites, and the lily claim moved to an asserting review row (its oracle is the kernel's part volumes: additivity, not ground truth).

ε runs (default, 1e-6, 1e-12): PR rows `full_turn_bore_mate`, `mate2_r{1,2}_probes`, `meetings_rows`, `undeclared_rule_rows` and `offer_rows` are all green. CI for the frozen head: not verified by me. The PR's head has moved past the frozen SHA.

## Style

- **Q1** `carrier_cross.rs:281-311` (`unsure`): circle × circle via the planes' common line plays nearly the role of `splitting/classify.rs:284` `conic_plane_crossing_roots` (circle × the other circle's plane) and of the 2-D `profile/src/path/arc_fillet.rs:257` `circle_circle`. A third closed form for circle meetings; I found no shared home.
- **Q2** `reduce.rs:1533-1535` (`likely`): "a covered line meets a curved carrier twice only by lying on it" is unenforced and passes `interior_clear = true` unconditionally. A line touching a torus at two tangency points is a counter-shape. Probably unreachable under `tangent_locus`, but nothing asserts it.
- **Q3**: MAJOR-1, MINOR-3, NOTE-2.
- **Q4**: NOTE-3.
- **Q5** `carrier_cross.rs:13-15` (`likely`): "so the sweep can split both edges there" describes a split no row needs (MAJOR-1).
- **Q7** `reduce.rs:2293` (`unsure`): `declared(ends, interior_clear: bool)` is a bool where the certificate could be a type. The (Zero,±) arms pass a literal `false` that can never matter.
- **Q8**: read `carrier_cross.rs` (492 lines) end to end; did NOT read `reduce.rs` (3,586 lines) whole.
- **Not exercised**: Q6.

## Probes

`probes/reviewer_3814_r2.rs` holds P1–P7. To run it, include it in `crates/sweep/tests/all.rs` with `#[path]`.

The mutants were env-gated, one-line edits, since reverted:
- `MUT_CLEAR` / `MUT_UNREAD` at the top of `boundary_crossing`;
- `MUT_GATE` at `vtxfac.rs:389`;
- `MUT_MIRROR` at the top of `mirror_edges`.

The instrumentation was an `eprintln!` on each `At` and `Clear`, tagging each candidate as vertex, ring vertex or meeting.
