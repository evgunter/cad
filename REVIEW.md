# Review r1: PR #4345 at a92e7cbd (base f3755cd4, the main it merged)

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 2 · NOTE 5 · style 8. I found no wrong body. Every pose that moved goes refusal → right volume, or refusal → another typed refusal.
The fixes: pin the refusal that remains, and correct the prose.

**Method.** Release builds of base, head, and head plus env-switched mutants (`R1_MUT`), each in its own target dir.
Every probe build is read through `differential::outcome` (t2, t3′, cert, legal operand, volume) and tessellated with `check_mesh`.
Probes: `crates/sweep/tests/review_r1_wrap_probes.rs`, 527 lines per tree.
I also copied `fan_end_review_rim_battery` from `join/fan-end-one-spelling-review` (167784d6) as `fan_end_review_probes_r1.rs`.
That branch is not a lane of this review. I read no other `join/wrap-edge-section-loop-review-*` branch, and none of the PR's comments.

## Claims

**1. The witnesses are right bodies: holds.**
- I re-derived all six closed forms:
  - tube annulus π(0.25−0.09)·2 = 0.32π;
  - `below(..)` rotation R·ẑ = (sinθ, 0, cosθ) gives z = c + m(1−x), so ∫ = π(c+m);
  - outer segment 0.25·acos0.8 − 0.4·0.3.
- Through `outcome`, every op in both orders holds its volume to 1e-7, and `check_mesh` passes.
- New families, all at the right volume on head (they refuse `SingleSiteSectionLoop` on base):
  - off-centre (0.37, −0.61) with tilts about x, y and the diagonal, up to ±44°;
  - the seam turned 90/200/333°;
  - r = 0.05 and r = 7;
  - tilts of 0.02° and 0.46° with c down to 1e-4 at the end vertex, and shallow seam crossings at 43.5°;
  - tubes r∈[0.05, 0.9], a box over the inner wall and part of the outer, a box between the walls, a tube tilted 20° (0.16π/cos20°);
  - holed plates and nested pockets (claim 3).
- The c=0.3, tilt 44.97° pose fell outside my closed form (the plane passes z=2). Its volume 4.045505723 matches a clamped-integral oracle (4.045505722). My error, not the kernel's.
- Two non-SOUND classes appear. Both reproduce without the arm, and both are already filed:
  - every tilted result fails the legal-operand union (`Containment(VolumeUncertified)`). A 4-arc cylinder under the same plane does too (`work/contact/at-infinity-probe-measures-in-closed-form-only.md`, open);
  - B∖A with a core rod left in the hole fails t3′ (`CensusUndecidable`). A 4-arc tube does too (`census-cross-solid-curved-pairs-undecidable-on-shell-results.md`, open).
- Refusals that should build: one. `P3 tilted tube B∖A` moves from `SingleSiteSectionLoop` to `Pieces(Probe{Escalated})` (N3).

**2. The gate is exactly right: holds for every pose I could reach. Its negative clauses are unexercised (MINOR-1).**
- Conic in the partner's face: the flush cap and the flush tube end refuse `UndeclaredCoincidence` before the join, base = head.
- Site on a seam vertex: a plane through the wrap edge's end vertex (tangent to the cap rim there) still refuses `Join(SingleSiteSectionLoop{1})`, base = head. The far rim point builds right on both.
- Two loops on one locus pair: a C-shaped torus wall at y=0.7 gives two circles on one face pair. It refuses at `GermFrameUnsupported` before the gate.
- Curved partner:
  - ball × coaxial seam cylinder at 5 turns: `GermFrameUnsupported`;
  - cone wall under a box, and cone × tube: `CurvedPairUnsupported` at the operand gate;
  - balls coaxial, and balls poled ±x / y / xy against z at 5 turns (P9, P11, 186 lines): identical on base and head, no `SingleSiteSectionLoop`.
- No true wrap crossing that I built fails the gate.

**3. `join_lone_ring` is sound: holds.**
- The face sense: `mfkrh` derives `!parent` (`euler_kill.rs:2375-2383`). `set_face_sense(promoted, old.sense)` hands that bit to both walled faces through `mef`'s Inherit, so the disc carries the old face's bit. The disc is the face whose conic winds with the outer loop, so the two are coherent.
- Rings, all six ops SOUND:
  - a through-hole inside the conic, and one outside it, each cut by a 2-arc tool and by a 1-arc one-site tool (the hole is itself built by this arm);
  - a blind pocket ring inside, and one outside, a later pocket's conic;
  - two seam walls on one plane;
  - a seam wall nested in a seam bore, and the CLEAVE tube (a pierce ring enclosed by the other conic).
- Preconditions: the two-faces walled check and the definite-winding check are explicit (`chord_join.rs:3406-3433`). `kfmrh`'s "no rings" is checked by the door. `lone_ring` is structural.
- A trace (`R1_TRACE`) shows the lone ring is taken on whichever operand holds the plane, in every op.

**4. Nothing else moved: holds.** Lines compared base vs head, timings stripped:

| battery | lines | diff |
|---|---|---|
| `pinch_runs` / `corner_pairs` / `pierce_runs` | 3025 / 16381 / 4537 | 0 / 0 / 0 |
| `rc_wide` shards 0, 7, 13, 29, 41, 55, 70, 83 of 84 | 8×481 | 0 |
| `near_tangent`, `join1_r1_seam`, `j3r2_r1_seam` | 7201, 12151, 12151 | 0 |
| **`join1_r1_tube`, `j3r2_r1_tube`** (not run by the PR; one-face wall pierces) | 6901 each | 282 each |
| fan rim battery (copied) | 1441 | 102 |
| files: one_segment_loop, germ_coplanar_conic, tang_circle_cylinder, review_cleave_wrongarc, split_across_a_revolve_seam | 30 tests | only the PR's rename and deletion |

- Tube batteries: all 282 moves are `SingleSiteSectionLoop` → build at the right volume. 270 are SOUND; 12 fail only t3′, all B∖A with a core rod in the hole (the filed class). No SOUND → anything.
- Fan rim battery:
  - 72 refusal → right volume (operand class only);
  - 6 refusal → `VolumeUnmeasured`;
  - 24 `{count:2}` → `{count:1}`.
- CI `gate ok` is green on a92e7cbd.

**5. The mutants are real: partly falsified.** Three of the four named mutants survive (MINOR-1).

| mutant | PR rows red | my probes changed | rim battery changed |
|---|---|---|---|
| hole choice flipped | 6/6 in `a_plane_across…` | 99 rows build → typed refusal (`LoopNotClosed`, `ResultInvalid`) | 39, all → refusal |
| opposite-senses check removed | 0 | 0 | 0 |
| "no other record on the locus pair" dropped | 0 | 0 | 0 |
| `wrap_site` test dropped (`crossed` always true) | 0 | 0 | 0 |
| planar-pierce requirement dropped | 0 | 0 | 0 |

- `one_segment_loop` and `germ_coplanar_conic` stay green under the hole flip. The flip bites mostly when the plane is operand A, and those files only put the plane at B (N4).
- The unmutated mutant binary's probe output is byte-identical to head's.

**6. The sweep is complete: holds for code, with prose residue (MINOR-2).** Code readers:
  - `find_match` / `partners` (`join.rs:1411`, `1462`): kept, correctly;
  - `cut_pair` dedup;
  - `chord_spec`'s guard: only `BoolPlanar` is newly opened, since `Split` already was;
  - `read_segments` (`rest.rs:534`) runs only after a join `Err` (`ops.rs:903-929`), so the REST residue is as the PR says;
  - the residue is scheduled on the open parent row.

## Findings

- **MINOR-1 (test gap; executed).** Nothing in the tree pins the refusal the gate leaves.
  - The PR deletes `a_closed_section_loop_with_one_site_refuses_typed`. Head's batteries hold 0 `SingleSiteSectionLoop` lines.
  - The refusal is still reached: the end-vertex plane above, and 72 rim-battery lines.
  - Four gate clauses survive every row I could run (table 5): `join.rs:645` (shared), `:658-667` (wrap site, planar), `:670-691` (senses).
  - Each clause is either redundant with the earlier locus/frame clauses, or reached only by shapes that today refuse upstream. No row shows which.
- **MINOR-2 (prose class; inspection plus executed contradiction).**
  - `chord_join.rs:1077-1080`: `lone_site_placeholder` says `chord_spec` mints a placeholder on every self-loop "but a curved split face's whole conic". The boolean's planar pierce is now a second exception, and the PR's prose sweep missed it.
  - The `SingleSiteSectionLoop` message (`chord_join.rs:463-465`) says "seam crossed by a plane". The gate also admits a wall × wall site (`na>0 && nb>0`, `join.rs:663`).
  - `boolean/mod.rs:4270` and `lib.rs:467` call `section_segment_sites` "the join's section segments". They no longer include the wrap segments.
  - The row's `## Built` names "a sphere pair's radical circle" as a curved pierce that keeps the refusal. My 186 lines of sphere pairs never reach it.
- **NOTE-1.** The witness file checks tiers 1–3 and volume, not t3′ or the legal operand. Both filed classes therefore pass invisibly in W1/W4/W5/W6 (see 1).
- **NOTE-2.** `work/paths/circle-lowers-to-one-segment.md:27-31` still says the slab "refuses `Join(SingleSiteSectionLoop)` … parked on D10". It is PATHS territory, and nothing schedules the fix.
- **NOTE-3.** The tilted tube, B∖A, now refuses `Pieces(Probe{Escalated})`. Typed, but a new frontier for this shape.
- **NOTE-4.** The hole rule is load-bearing mostly with the plane as operand A. With the plane as B, the flipped choice still builds the right volume. Unexplained (unsure).
- **NOTE-5.** My own oracle errors, disclosed: a degrees/radians slip in the first run, and the cap-crossing pose. Both are corrected in the committed probes.

## Style lane (questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7; Q8 partial)

- **S1 (Q7, likely).** `chord_join.rs:3393-3396` promotes with `mfkrh(Inherit)` and then overwrites the derived bit with `set_face_sense`. That steps round the door's `SenseContradictsChart` check rather than stating the bit through a door, and it is `set_face_sense`'s only production caller in topo.
- **S2 (Q1, unsure).** `lone_ring` (`chord_join.rs:3182`) and `is_pierce_ring` (`:3610`) are near-parallel structural readers of "a ring of null edges".
- **S3 (Q1, unsure).** `wrap_site_segments` re-derives a record's partner test outside `partners`. Unlike `partners`, it never checks that the B germs mirror the A loci; it reads `rec.b[0]` only for the site (`join.rs:660`).
- **S4 (Q1, likely).** The witness file's `every_op` is another copy of the six-ops-both-orders helper, and it skips `differential::outcome` (`a_plane_across_a_one_face_wall.rs:66`).
- **S5 (Q2, likely).** The gate is restated in prose four times: the fn doc, the row, the PR body, and `SplitJoinError`'s doc. They already disagree on "plane".
- **S6 (Q3, sure).** The new across-the-wall rows in `one_segment_loop.rs:303-314` cannot go red on the hole flip (plane as B only).
- **S7 (Q6, likely).** The "unblocked" PATHS row is left with a false sentence and no schedule (NOTE-2).
- **S8 (Q8, partial, sure).** I read `chord_join.rs`'s module header and every touched region, not all 5645 lines.

REVIEW COMPLETE
