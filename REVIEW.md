# Review r2 — PR 4050, "join: a six-crossing vertex pair nests its pairing in B and builds every op"

Head `0e0d871a` vs main `6e57858c`, release, one target dir per tree, every build through `differential::outcome`. No
other review branch or session read. Probes `crates/sweep/tests/review_sixx_r2_probes.rs` (`--test sixx`; PR 4036 r2's as
`--test r2probe`), scripts in `review-r2-tools/`. Each line carries a crossing count computed from the face arcs on the
sphere, kernel-free (6 at the PR's traced pose, pinned by a row).

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 5 · NOTE 2.

No wrong body ships in about 265k runs. Every definite tier-3′ failure I found is byte-identical on main. The defects
are a depth-2 nest that refuses with an invariant error, and three gaps in what the PR's own rows can see.

## Claims

1. **No wrong body ships — holds (executed), with MINOR-4's escalated bodies.**
   - Shapes: 16 prism corners (notch343, 330, 352 and 300; wedges 300 to 359°; L; mirrored notch343, w335, notch5 and
     reflex315) on a cube's edge and corner, and against a second posed notch or wedge (`SX_OTHERS=notch343,mnotch,w335`).
   - Cube grid, n=6: 39 738 runs, all `PairingMismatch` on main, all SOUND on head.
   - Shape against shape, n=8 and 10 (the PR's batteries stop at 6): 37 680 runs. 33 058 go to SOUND, 4 616 go to
     `ClassificationInvariant` (MINOR-1) and 6 are identical.
   - Shape against shape, n≥6 sample: 41 730 runs. 12 754 go to SOUND, 290 to `ClassificationInvariant` and 6 to
     `JoinDesync` (NOTE-2); the rest are identical.
   - Near-tangent: each change of the crossing count along ψ, bisected to float precision, posed at ±1e-3, 1e-5, 1e-7 and
     1e-9 rad: 57 048 runs. 1 187 go `PairingMismatch`→BAD (MINOR-4). The other changes are refusal→SOUND or
     refusal→typed refusal (`MarginDiag`, `ResultInvalid`, `Pieces`).
   - **0 SOUND→refusal, 0 definite refusal→BAD:** the 119 definite tier-3′ BADs (`StaleContactDeclaration`,
     `UndeclaredContact`, d=±1e-9) are byte-identical on main. apex4/valley4 at a cube corner: 9 576 runs, identical.
2. **The nesting argument — holds for `b_runs`; the mint is wrong at depth 2 (executed).**
   - `b_runs`, ported line for line, was checked over every non-crossing matching at n=4, 6, 8 and 10, every rotation and
     both orientations of every pair (15 504 readings; `review-r2-tools/bruns.py`):
     every interval holds whole pairs, a wrap-adjacent run none, the holder is innermost, and the refusal fires iff two
     pairs cross (all pairings, n≤8). Depth reaches 2 at n=8, 3 at n=10. The PR's meander check of the start: not re-run.
   - A trace classified every held run at head (`review-r2-tools/classify.py`).
     - At n=6 on a cube, every holder is a fan and every shape builds: 22 574 strut mid-run, 1 606 strut in the first
       corner, 1 722 in the last, 13 033 fans ending in the last corner. 803 fans end mid-holder, a shape outside the
       PR's five, and they build too.
     - Shape against shape adds struts held by struts (394 runs) and depth-2 fan/fan chains, all SOUND, and the one
       failing class: **a strut held by a strut held by a fan** (MINOR-1).
3. **The guard still guards — holds; the new refusal is reached (executed).**
   - None of my ~208k head probe runs gives `PairingMismatch`. Guard 1 fires only in the unit row; guard 2 is unreached.
     That fits "a misread walk order only".
   - The nested-plan `SharedVertexCrossings` is reached and right (NOTE-1).
4. **Nothing else moved — holds (executed).** Byte-identical main vs head: `rc_wide_battery`, all 84 shards (40 320
   lines); `pierce_runs_battery` (4 536); `join1_r1_reflex_battery`, `j3r2_r1_reflex_battery` (1 152 each); r2's hex and
   apex grid. My own batteries change only lines that were `PairingMismatch` on main.
5. **The mutants are real — holds for the PR's; two of mine survive (executed, `review-r2-tools/mut.py`).**
   - PR "root" reddens the row, 18/18, and PR "order" 16/18, matching the PR.
   - Mine, "holder" (`max_by_key` at `insert.rs:459`), survives the PR's row and `f12_b_runs_nest_a_pair_and_refuse_a_crossing`
     (MINOR-2).
   - Mine, "lastcorner" (`insert.rs:1167` → `next`), survives everything (MINOR-3).

## Findings

- **MINOR-1 (executed). A strut held by a strut held by a fan refuses `ClassificationInvariant` "an earlier run at the
  vertex carried a strut's corner to its copy" (`insert.rs:1344`).**
  - Scope: 4 616 + 290 + 744 runs, all at n=8, all `PairingMismatch` on main. Repro: `notch343` vs `notch343` grid
    `i=13 j=8 k=2`, ba U and ba S.
  - Trace: B pairs at positions (5,2) (1,6) (3,4). Pair 1 (a fan) holds pair 0 (a strut), which holds pair 3 (a strut).
  - Cause: `fan_holder` (`insert.rs:621-635`) reads only the immediate holder. For a strut holder it mints at
    `vertex(i)` (`:646`), the original vertex, but the fan has already taken that corner to its copy. `nest()` (`:580`)
    counts the whole chain; `fan_holder` does not.
  - Control: walking `holder` up to the nearest fan before the lookup turns all 4 616 SOUND and moves nothing else in
    37 680 runs. The outermost-holder mutant also turns 1 242 of them SOUND.
  - Fail-loud and not a regression, but it falsifies "a run held by a strut keeps the existing hang-at-the-tip path"
    beyond depth 1. Unchecked by inspection: whether such a strut in the fan's first corner could pass `corner_bound`
    at the original vertex (none did here; unsure).
- **MINOR-2 (executed). Nest depth ≥ 2 and the innermost-holder choice are unpinned.**
  - Swapping `min_by_key` for `max_by_key` (`insert.rs:459`) leaves the PR's row (`join_pierce_runs_sweep.rs:486`) and
    the unit row (`insert.rs:2096`) green, since both stop at n=6, depth 1.
  - Only my n=8 battery sees it: 3 766 SOUND→`ClassificationInvariant`.
- **MINOR-3 (executed). The last-corner departure arm (`insert.rs:1167`) is unobservable.**
  - Over 17 505 reaches, it changes the departure half 2 205 times, and `strut_faces_first` returns the same facing
    every time.
  - Replacing it with `next` changes no line of the PR's row or of 25 698 probe runs.
  - The PR names "strut in the last corner" as covered, and its row cannot see this arm.
- **MINOR-4 (executed). "refusal→BAD: 0, definite or escalated" does not extend to near-tangent six-crossing poses.**
  - 1 187 runs at d=±1e-7 and ±1e-9 rad go `PairingMismatch`→BAD, over notch343, mnotch343, notch300, notch5,
    reflex315 and w359, on edge and corner. Repro: `mnotch343 corner i=13 j=3 k=72 d=+1e-7 ab U`, `SX_SWEEP=near`.
  - Every one is `CensusEscalated` (`pm_census_*`, margins 1.4e-9 inside the band), with exact volume, and t2, cert and
    operand true.
  - Checked without the census, each body is right. Main ships 2 such bodies in this set, so the class is parked
    door-census exposure (as PR 4036 r2's MINOR 1), not a new defect. The PR body's measurement should say so.
- **MINOR-5 (inspection). Prose still states the adjacency premise.**
  - `insert.rs:8-10`: each run "holds no third germ in that solid's own walk order", which a nested B run now does.
  - `m3_pr6_saddle.rs:2-3`: "F12 guards: B-cyclic adjacency".
  - `mod.rs:59-60`: the inner "mints at the outer's copy", which is not so for a strut holder.
- **NOTE-1 (executed). The nested-plan `SharedVertexCrossings` (`insert.rs:838`) is reached, and refusing is right.**
  - Probe `review_sixx_pinch`: a 290° wedge and a 40° wedge pinched on the z-axis, cut at the shared corner by a cube
    crossing them 6 and ≥2 times.
  - Pinch-first U/I/S: 51 runs refuse here, `slot=1` (traced). Cube-first U/S refuse in the pre-existing reconcile, and
    cube-first I builds SOUND. Main: `PairingMismatch` on all 102.
  - The PR says no battery reaches it, and no row pins it.
- **NOTE-2 (executed).** w359 vs mnotch, at n=6 with nested plans, goes `PairingMismatch`→`JoinDesync` "every chord arc
  separates a loose scaffolding pair" on 18 runs. Main gives the same refusal on 96 neighbouring poses, so the class
  pre-exists. It is the message of the filed valley4 issue; whether it is the same cause is unsure.

## Style

Exercised Q1–Q8; Q8 read `insert.rs` (2 367 lines) header and every changed region, not every line.

- **Q1/Q7 (`insert.rs:136, :1098, :1119`), sure.** "holder" is a run index (`SideRun::holder`), a fan's null half (the
  parameter) and a hung strut (the local) within `mint_directed`'s reach; `nest`, `nests`, `nested` sit beside them.
- **Q1 (`insert.rs:459` vs `:726`), likely.** Strut-in-strut nesting is decided twice, combinatorially (`b_runs`
  holder) and geometrically (`holds_whole` in the hang path), and nothing checks that they agree.
- **Q7 (`insert.rs:580` vs `:621`), likely.** `nest()` walks the holder chain while `fan_holder` reads one link. MINOR-1
  lives in that gap.
- **Q4/Q5 (`insert.rs:8-10`, `m3_pr6_saddle.rs:2`, `mod.rs:59`), sure.** The stale premises of MINOR-5. Class: the PR
  edited the F12 bullet and left the lead paragraph a screen above it.
- **Q3 (`join_pierce_runs_sweep.rs:486`), sure.** The row's three poses cannot go red for the holder choice, depth 2 or
  the last-corner arm (MINOR-2 and 3).
- **Q6 (`insert.rs:838`), likely.** A new refusal named unwitnessed in the PR body, with no row and no scheduled
  follow-up; NOTE-1's probe witnesses it.
- **Q5 (`insert.rs:838`), unsure.** The check reads `r[slot].holder` in both slots, but only B's side ever holds one, so
  the slot-0 pass is vacuous. It also sits before the `same_reading` invariant it used to follow.
- **Q2 (`insert.rs:620-635`), unsure.** The fan-or-strut decision is re-derived from `run_fan` at mint time rather than
  carried in the plan beside `holder`.
- **Dispatch premise (brief), sure.** It is accurate. "Every holder was a fan" holds for the PR's batteries but not in
  general: strut holders are reached at n=8.

REVIEW COMPLETE
