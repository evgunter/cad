# Review of PR 4051 (frozen head c7ad3125, main 4cf3f8b9)

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 1 · NOTE 4.

The island crossing holds on 130 more refusal→SOUND lines than the PR's 18, across new families. No line moved anywhere else. The PR's three mutants reproduce exactly. One of my two mutants, the choice of which face `kef` kills, survives every row but breaks 38 of the new lines, so that choice wants a row.

Evidence: `review-4051/`. That folder holds the probe `r4051_island_probes.rs`, `instr.patch` (env-switched mutants and shape dump), the main/head/instrumented outputs (`*.txt.gz`), `batteries.txt` and `mutants.txt`. All builds are release, in separate target dirs. Both trees carry the same probe-only re-export of `drill_hole`/`plane_every_face` in `topo::test_support`.

## Claims

1. **The island crossing is right: HOLDS (executed, main vs head, `differential::outcome` with my own signed clip-volume oracle).**
   - `deep`: holed block at h = 2, 6 and 0.6; both hole corners; 140 directions; cube sides 4 and 12.
     - h = 2 reproduces the PR's 18 refusal→SOUND exactly.
     - h = 6 adds 22 lines and h = 0.6 adds 8.
   - `two`: two holes, with planes through one corner of each, giving two islands in one op. 60 refusal→SOUND.
   - `u`: a U-shaped hole, with planes through both arm tips, so one island is pinched twice. 38 refusal→SOUND, plus 2 more at a single tip.
   - `cyl` (curved island, holed block × cylinder r = 3 and r = 1, h = 2 and 6): 0 lines moved.
     - On r2's cylinder set, 5 lines go `PinchUncrossed`→`ResultInvalid{VolumeUncomputable{RingOnCurvedFace}}`, as the PR says.
   - All 148 new SOUND lines: volume within 1e-7; tiers 2 and 3′, certificate, legal operand; `FACE2V=0` (no face meets two vertices at one point); the gate passed them, so no loop role is inverted.
   - No other line moved on any family. 0 SOUND→refusal, 0 →BAD, 0 panics.
2. **Bow-tie is not a separate class: HOLDS for every current line; r2's 184 not re-run (unsure there).**
   - I applied the outer-loop instrument to head and printed full errors. Every head `PinchUncrossed` line refuses `RingMeetsOuter`, and every line that also reads `LoopRoleInverted` carries `RingMeetsOuter` too:
     - u2 staircase: 43/43, both;
     - r2 near-tangent: 4/28;
     - r1 cube: 1/48;
     - my families: 7/1 313.
   - The PR's parity argument also checks out from first principles. Re-pairing the two corners of one loop at a vertex always splits that loop, and re-pairing corners of two loops always merges them. So a single loop through `v` with crossed corners cannot exist (the shape r2 proposed). The kef crossing ships such a loop only because it starts from two faces.
3. **The nested fork is stated correctly: HOLDS (executed).**
   - Shape: I added a dump at the refusal. On all 1 313 residue lines of my new families, the only kept faces through `v` twice are ringless faces whose *outer* loop passes `v` twice. On ∩, one such face comes from each operand.
   - Crossing gives `RingMeetsOuter`, executed on:
     - r2 cube 26/26, r2 near-tangent 28/28, r1 cube 48/48, u2 43/43;
     - my families 1 313/1 313.
   - I found no cheap body. Option 2 is impossible by the pairing argument above. The others change an at-rest invariant or add a topology-only edge, so it is a real design choice.
4. **Nothing else moved: HOLDS (executed, main vs head).**
   - `pierce_runs_battery`: all 4 536 lines byte-identical.
   - `rc_wide_battery`: 84 shards, 40 320 lines, identical, no panics.
   - Pinch batteries, identical per line apart from the r2 cylinder set's 5:
     - r2 cube: 40 320;
     - r2 near-tangent: 40 320;
     - r2 cylinder: 17 280;
     - r1 cube: 28 512;
     - r1 u2: 720.
5. **The mutants are real: HOLDS (executed, one env-switched build, `join_pierce_runs_sweep::`, 11 rows).**
   - The clean tree passes 11/11.
   - The PR's three behave as it says:
     - outer loop crosses: 2 red, and the messages say `RingMeetsOuter`;
     - no pre-pass: 4 red;
     - both corners outer: the island row goes red with `PinchUncrossed`.
   - Mine: `PX_KILLPLUS` (kill `he_plus`'s face, as before) and `PX_ANYFACE` (no ringless test) **survive all 11 rows** (m1, n3).

## Findings

**m1 (MINOR, executed). The choice of which face dies is load-bearing and no row pins it.**
- Code: `crates/topo/src/boolean/zip.rs:417-420`.
- Mutant `PX_KILLPLUS` passes every row. In the island row, `he_plus` already lands on the island.
- But it turns 38 of the new U-hole lines from SOUND to `Euler(FaceHasRings)`, for example `u2tip mid side=12 psi=0 th54 cp S` (`PX_KILLPLUS-u.txt.gz`).
- Fix: pin one such pose. Confidence: sure.

**n1 (NOTE). The row and PR body count 49 bow-tie lines; their own breakdown is 48.**
- The breakdown is 43 + 4 + 1, and the table column also sums to 48.
- Location: `work/join/a-pinch-no-kept-face-can-cross-refuses.md:42`.

**n2 (NOTE). The PR undercounts its own reach.**
- It also builds the deeper-hole, two-island and twice-pinched-island poses (130 lines above). The `## Built` section could name them, and the twice-pinched island is unpinned.

**n3 (NOTE). The ringless test at `zip.rs:375` has no witness among reachable poses.**
- Dropping it (`PX_ANYFACE`) moves no line of my families and reddens no row. On a pose with two ringed faces of one chart at `v`, `kef` would refuse typed (`FaceHasRings`) instead, so the gap is benign.

**n4 (NOTE, pre-existing, unmoved).** Both trees have:
- 786 near-tangent lines that read SOUND but have `FACE2V`;
- 724 holed-block × cylinder results with `FACE2V`.
- These are the class of the filed cleave row `a-boolean-ships-a-face-whose-loop-passes-two-vertices-on-one-point`.

## Style (exercised Q1–Q4, Q6–Q8, `zip.rs` read whole; Q5 found nothing)

- **S1 (Q1, sure). `split_across` holds two closures named `face_of` with different signatures, and spells `ringless` twice.**
  - `face_of`: `zip.rs:349` returns `(LoopKey, FaceKey)`; `zip.rs:406` returns `(FaceKey, bool)`.
  - `ringless`: the closure at `zip.rs:358` and inline at `zip.rs:412`.
  - The arm re-derives what the search already decided, instead of carrying the ringless face out in `Crossing::TwoFaces`.
- **S2 (Q1, sure). Duplicated helpers in `join_pierce_runs_sweep.rs`.**
  - `every_op` (`join_pierce_runs_sweep.rs:594`) is the third spelling of the every-op-both-orders ladder (`:249`, `:507`).
  - The axis-box-to-half-spaces loop now has three spellings (`:194`, `:492`, `:747`). The new island row writes its own `box_planes` closure beside the new `pieces_within`.
  - This is a fix that mints its own copy.
- **S3 (Q6, likely). A measured shape is stated at the claim site as a universal fact.**
  - `PinchUncrossed`'s doc (`mod.rs:2010-2014`): "the one face through the point twice passes it on its outer loop".
  - Nothing goes red if a residue line of another shape appears. My 1 313 lines agree, but that is a measurement, not a guard.
- **S4 (Q2, likely). The second arm of the dying-face choice assumes its face is ringless and does not check.**
  - The comment "which must hold no ring" (`zip.rs:415-416`) covers only the first arm. The second arm (`zip.rs:419`) is held by the search 40 lines up and by `kef`'s refusal.
- **S5 (Q4, unsure). The inverse of the shape this PR builds is the one a P2 row says `pinch_site` cannot read.**
  - `split_across`'s doc still calls it "the inverse direction of `finish::pinch_site`" (`zip.rs:313-317`).
  - It now builds an island merged into a ring through `v` twice. `work/join/a-hole-weld-cannot-tell-a-figure-eight-hole-from-an-island-face.md` says `pinch_site` cannot tell that shape from a figure-8 hole.
  - That row says the island poses "never reached it". It is worth a line saying the pre-pass now builds them by the other route.
- **S6 (Q7, unsure). The tuple `match` with wildcard arms at `zip.rs:417-420` reads as more cases than there are.** There are two outcomes, keyed on one bool. This is a matter of taste.
- **S7 (Q3, sure). The refusal rows can go red, and the mutants show it, but each pins only `yx S` of one pose.** Fine for a residue pin; noted only so the gap in m1 is not read as general.

REVIEW COMPLETE
