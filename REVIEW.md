# Review r2 — PR 4026 `join/pierce-two-out-runs` at 93ecd145

Base: `e4a0a0d` (the head's merge parent, current main; f8aabaae + main's later merges). Lane isolation kept: I read no other review branch or session. Probes: `crates/sweep/tests/join_pierce_r2_probes.rs` on this branch (each probe's command is in its doc comment). Every build line is `differential::outcome`: tier 2, tier 3′, the certificate, a legal operand, and a kernel-free volume.

**Verdict: APPROVE-WITH-FIXES** · MAJOR 1 · MINOR 2 · NOTE 4

The PR does what it says on its own battery, and I reproduced that exactly. Off its battery, one narrow MAJOR: 3 poses that main refuses now ship a body that fails tier 3′. Main already ships the same failure at neighbouring poses. The fix pass can file the class and pin these poses; it doesn't have to cure it here.

## Claims

1. **No wrong body ships: FALSIFIED, narrowly** (M1). My `r2_shapes_battery` has 13 607 lines: the L, reflex corners of 315° and 225° plus their mirrors, the bottom corner, a convex corner; a 24×9 grid plus near-tangent tilts at 1e-2, 1e-4 and 1e-6; every op, both orders. Main→head: 1 656 refusal→SOUND, 714 refusal→refusal, **3 refusal→BAD**, 0 SOUND→refusal. My `r2_holed_battery` (3 359 lines) puts a holed block's inner corner on the face, which closes an island pinched at `v`. It has 0 BAD on either tree, 306 refusal→SOUND and 0 SOUND→refusal. Not reachable: a curved face (the ball refuses all 156 lines upstream on both trees; N2) and a 3-run vertex (N3).
2. **The ∩ rule is a rule: UNSURE, not falsified.** I found no derivation. The vtxfac comment (`vtxfac.rs:744-752`) gives a reason (the ring's In face must pass one copy per run). The edge family's ∪ needs In-side copies, so "copies on the kept side" can't be the general rule behind it. I tried to separate the readings by running ∩ → end-facing and ∩ → walk (env mutants) over all of shapes and holed, 16 966 lines. Neither ever turns a head refusal into SOUND, and the op rule never builds BAD. So no pose separates the readings; it stays a fit that holds on every reachable 2-run pose.
3. **The welds are sound: HOLDS, with caveats.** `kfmrh` is necessary: without it the row's ∪ ships BAD in 9 of 9 lines (`t3p=false`). The weld is not what causes M1: with the weld or `kfmrh` turned off, M1 persists, and it disappears only with main's facing. Leaving copies apart is legal at rest under PR 3813 (t3p passes), but it reaches beyond the disclosed family (m2) and contradicts `boolean_pinch_copies.rs:1-4` (S4). The weld refuses loud, and that residue is unfiled (m1).
4. **Nothing else moved: HOLDS.** `rc_wide_battery`: 40 320 lines, byte-identical. `join1_r1_reflex_battery`: 1 152, identical. `pierce_runs_battery` reproduces exactly: 127 moved, 110 refusal→SOUND and 17 refusal→refusal (8 + 9), 0 BAD. My single-run convex corner (Lcvx) matches on 1 942 of 1 944 lines; the other 2 are near-tangent ∩ lines that go refusal→SOUND (S9).
5. **The mutants are real: HOLDS.** These are runtime switches on a copy of head, and each was run against the row (`join_pierce_strut_facing`):

   | mutant | red rows |
   |---|---|
   | struts ignore the walk | 1 |
   | no pierce weld | 2 |
   | no one-pierce guard | 2 |
   | hole weld without `kfmrh` | 2 |
   | mine: ∩ faces end | 2 |
   | mine: ∩ follows the walk | 1 |

   Head is green, 7 of 7.
6. **First-wrong-state story: HOLDS.** At the row tilts main refuses 12 × `kept_end` and 6 × `SelfLoopEdge` (two) and 18 × `SectionLoopMixed` (edge), as the row says. On head with main's facing restored, ∪ and ∖ at the two tilts stay SOUND. ∩ gives `SelfLoopEdge`, and the edge family gives `SectionLoopMixed` in every op. So the facing was wrong only for the lone-edge family. I instrumented head with main's facing toggled rather than main's own source (N4).
7. **Filed rows: HOLD as written, but incomplete.** The vertex-vertex table reproduces exactly: 526 = 368 + 158, and 243 / 111 / 45 / 40 / 33 / 30 / 24. The wide family (i=0 j=3..5 and i=2 j=5, ∩) and the edge family (i=6..8 j=0..2, cube ∖ prism) match; every other face-placement line is SOUND. The PR body says the residue is categorized in the filed rows. That is untrue off the L (m1).

## Findings

**M1 (MAJOR, executed): head ships 3 tier-3′-failing bodies where main refused.**
- Pose: `R315m`, `m` = −(the `e_in` wall normal) + 1e-6·kick2 (`R2_SHAPE=R315m R2_TAG=nf1_1e-6k2s-1 … r2_one_pose`). Prism ∪ cube, cube ∪ prism and prism ∖ cube go from `JoinDesync "derived ring role order…"` on main to `OK BAD t2=true t3p=false`, with the volume exact.
- t3p is `CensusEscalated { Indeterminate, margin −1.34e-9, predicate pm_census_ee_span }`.
- Seat: the walk facing, `vtxfac.rs:756-776`. With main's facing (`R2_MUT=fixed`) all 3 refuse again; with the weld or `kfmrh` off they stay BAD.
- Main ships the same t3p failure at 6 neighbouring 1e-6 poses (`R315 nf1_*`, `R315m nf*/ne*`, identical on both trees). This PR unmasks that class; it doesn't create it.
- Fix: file the near-tangent tier-3′ class (P0) naming these 9 poses, and scope the PR body's "0 BAD".

**m1 (MINOR, executed): the weld's own refusals are unfiled residue.**
- `JoinDesync "a pierce's copies divide a face the zips kept"` (`finish.rs:529`): 214 shapes lines, all ∪, both orders, every one on the 315°/225° corners (R315 82, R315m 70, R225m 32, R225 30). On main they were `kept_end` refusals.
- `"two fragments of a pierced face meet one pinch"` (`pinch_site`, through the weld): holed ∖ cube, edge-run family.
- None of the four filed rows names either message.

**m2 (MINOR, executed): ∪ ships two vertices at one point beyond the disclosed family.** In the holed-block island pose (`r2_holed_inspect`, `R2_POSE=0,0.75` / `3,0.9` / `6,1.2`), ∪ is SOUND in both orders. It has 2 vertices at `v`: the cube face's ring passes one, the island face the other, and no face meets both. Under PR 3813 that is legal. But the PR body lists only the lone-edge prism ∖ cube under "Deviations", and says ∪'s apart copies are fixed.

**N1 (NOTE, executed, pre-existing):** main and head identically ship 25 BAD on the single-run convex corner. Examples: `Lcvx g4.6 cp U` → `UndeclaredContact VertexOnFace (2,1,1)`; `g16.2 pc I` → `StaleContactDeclaration`. This is contact bookkeeping at a kissing corner, outside this diff; it is worth a row. Some near-tangent `Lcvx ne*1e-6` lines read BAD only by a volume error of about 2e-7, with t3p true. My oracle at 1e-6 is the likely cause.

**N2 (NOTE):** for a curved pierced face, `ball_poled` with the L refuses all 156 lines identically on both trees (`SectionNotPolar` / `SectionArcSide`). The walk about a curved face's normal is still unexercised, as the PR says.

**N3 (NOTE):** a 3-run vertex is out of reach. A prism vertex's link meets a plane at most 4 times, so it has at most 2 runs, and the holed corner is the same shape. The P3 row is right that nothing reaches it. Walk start-independence (the claim in the PR body) holds on all 3 695 two-run traces I took (`R2_TRACE`).

**N4 (NOTE, method):** claim 6 was measured on head with env-switched facings, plus main's own refusal strings at the row tilts. I did not instrument main's source.

## Style (questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7; Q8 partial)

- **S1 (Q1, likely).** `weld_pierce_copies` (`finish.rs:499-540`) and `weld_pinches` (`finish.rs:575-655`) are two welds of coincident copies with drifted rules.
  - One admits any face (`|_| true`). The other admits only the pierced face's lineage and excludes section faces ("meet only on a section face, stay apart").
  - Only `weld_pinches` checks `one_vertex` and the post-fuse "did not fuse its pair".
  - Look also at the zip's fusion.
- **S2 (Q1, likely).** `setopfinish` groups pierce runs twice from one `red.null_pairs` walk: `pierce_runs` by A survivor (`finish.rs:357`) and `pierces` by site (`finish.rs:428`).
- **S3 (Q5/Q7, likely).** `finish.rs:1-8` still describes the welds as `setopfinish` steps. `weld_pierce_copies` is a post-zip pass that `ops.rs:613` calls but that lives in `finish.rs`.
- **S4 (Q4, likely; doc rot).**
  - `boolean_pinch_copies.rs:1-4` says copies "do not reach rest as two touching vertices".
  - The closed cleave row says the case is "never reached".
  - Both are now false (lone-edge prism ∖ cube, and the holed ∪ of m2). Neither changed.
- **S5 (Q3, sure).** No battery can go red on a duplicate vertex: `outcome` never counts vertices at a point. "The pierce point is one vertex wherever a face meets it" is guarded only by the row's tilts.
- **S6 (Q6/Q5, likely).**
  - `both_in` (`vtxfac.rs:756`) is spelled through `kept_side` as if general, but it is `op == Intersect`.
  - "Backed by measurement, not derived" is said only in the PR body. At the claim site the comment reads as a derivation.
- **S7 (Q2, likely).** "Any run can start the walk" is unenforced: the code always starts at run i+1's start germ (`vtxfac.rs:764`). It held in every trace I took (N3), and the P3 row itself says arcs may nest.
- **S8 (Q5, unsure).** The `Joint::Hole` arm (`finish.rs:724`, then `zip.rs:156` `kfmrh`) reads only "one ring holds both copies". It can't tell a figure-8 hole from an island face pinched off inside the hole; there, `kfmrh` would turn a face's outer loop into a ring. My island pose never reached it, because the copies landed on separate faces.
- **S9 (Q5, unsure).** The twin-edge pick (`finish.rs:860-882`) is documented for "a vertex several runs of one pierce cut". It also flips 2 near-tangent single-run convex-corner ∩ lines (`Lcvx ne1e-6k0s1`, `k2s1`) from refusal to SOUND, so it reaches further than its doc says.
- **S10 (Q7, unsure).** At `vtxfac.rs:769`, `strut_order` is levered by the piercing sector's `arm` while it reads signs about the pierced face's normal. That mixes frames.
- **S11 (Q8, partial).** I read the first half of `finish.rs` plus every touched hunk, not the whole file. `ops.rs` (5 156 lines) I did not read.

REVIEW COMPLETE
