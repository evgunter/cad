# Review r2 of #4008: pocket ring re-homing (frozen head d930c23f)

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 3 · NOTE 3.

Frozen `d930c23f` vs main `4c9d9385`, release; no other review branch or PR
comment read. Probes: `crates/sweep/tests/pocket_ring_rehoming_r2_probes.rs`.
Wrong picks measured by uncommitted instrumentation in `find_match` logging
each candidate's true central angle in the germ's sense beside the chosen one.

## Claims

1. **Arm-then-chord picks the right partner on every conic: FALSIFIED on
   steep ellipses; holds on circles.**
   - **Circles hold.** Ahead is travel in (0, π), where the chord
     2r·sin(φ/2) increases with travel. A non-adjacent same-locus pair
     travels further than the adjacent pair it skips, and with two or more
     arcs open one is shorter than π, so the global minimum is a true pair.
   - **Behind is unguarded.** There the chord *falls* as travel grows, so
     `nearer` prefers the farther Behind site. I found no reachable pose
     (it needs Behind vs Behind with no Ahead pair left on the locus), and
     no docstring says it.
   - **Ellipses fail.** From a minor-axis end, d(dist²)/dt =
     2 sin t·((a²−b²)cos t + b²) has a root inside the Ahead half whenever
     a ≥ √2·b. So at tilts of 45° or more the chord is **not monotone within
     the arm**: it rises to about √(a²+b²) and falls back to 2b at the
     half-turn.
   - **Measured** by `r2_steep_ellipse_battery` (a plate pierced by a rod
     tilted θ; 3240 runs): **300 poses mispair**, all at θ ∈ {60, 70, 78}°,
     none at 30° or 50°. In 216 of the 516 wrong selections the chosen site
     is at φ = π exactly: the `Zero → Ahead` antipode (chord 2b) beat a
     nearer site with a longer chord. Example: θ=70°, x=(−1.1695, −0.15),
     phi=1.57, U AB chose φ=2.659 (chord 0.996) over φ=1.063.
   - **Every mispaired pose refuses**: `RingHomingAmbiguous` (276) or
     `SectionLoopMixed` (24), the symptom the row fixes for circles. **No
     wrong body ships**; on main the same 300 also refuse. Two docstrings
     are false for ellipses: `join.rs:950-958` ("within one half-turn the
     chord IS monotone… a straight germ line and a conic one meter the same
     quantity") and the `Zero` argument at `join.rs:983-990`.
   - `j3r2_tilted_battery` cannot see this: its tilts are at most 0.3 rad
     (aspect 1.047 < √2), so it being byte-identical is no evidence here.
   - Covered: both orders, ∪ ∖ ∩, both senses (mirrored plate); the arm is
     symmetric in cand/entry (analytic).
2. **Nothing straight-line moved: HOLDS.**
   - 14 batteries, main vs head, same row counts: `j3r2_pocket` 10800,
     `rc_wide` 40320, `j3r2_tilted` 2160, `join1_r1` 42336, `_declared`
     27000, `_seam` 12150, `_tube` 6900, `_bored_capsule`, `_reflex`,
     `join1_delta_arc`, `join1_delta_brick`, `j3r2_rand`, `j3r2_groove`,
     `j3r2_snowman`.
   - Byte-identical except: **pocket**, 234 rows (198 `RingHomingAmbiguous`
     and 36 `SectionArcWindow`, all now OK SOUND); **seam**, 18 rows
     (`SectionNotPolar` names 6v1/4v1, not 2v1); **tube**, 3 rows
     (`SectionArcWindow` names 3v1, not 5v1).
   - No SOUND→refusal. BAD/WRONG count is 494 on each tree, and no changed
     row is BAD.
   - My own batteries (3430 rows): SOUND goes 66 → 192, with no SOUND→worse.
     The U plate goes from 30 invalid bodies on main (`t3p=false
     cert=false`) to all 192 SOUND. 420 refusals become `OK BAD
     operand=false` (NOTE 2); all 1571 head BADs have the right volume and
     pass t2, t3′ and the certificate.
3. **`wall_region` is right: HOLDS where exercised; UNSURE on corners.**
   - A region holding both halves contains both endpoint azimuths, so its
     window cannot exclude the true arc. I could not build a wrong region
     whose window still holds the arc.
   - Unguarded corners (MINOR 3): `f1==recorded≠f2` returns a window that
     holds only one end (`join.rs:918`), and `f1==f2≠recorded` is accepted
     with no lineage or `gb`-carrier check (`join.rs:920`), though the
     window is read in `gb`'s chart.
   - The `reach_continuation` fillet case was not rerun separately.
4. **The oracle is exact and the suite reds under each revert: HOLDS.**
   - Independent check: a 200 000-gon D clipped against the square agrees
     on all 19 groups to 8.1e-11 (discretisation).
   - Reverting the window read: 90 `SectionArcWindow` lines (the PR says
     96). Forcing every arm to Ahead: 198 `RingHomingAmbiguous`. On main:
     198 + 36.
5. **`join-ranks-conic-facing-germs-by-chord`: FIXED for circles, OPEN for
   steep ellipses.**
   - Its back-to-back example (CCW at 0.2, CW at 0.1, partner at 3.0) now
     reads Behind vs Ahead and pairs right.
   - The facing test still *accepts* back-to-back germs (`join.rs:1781-1785`);
     the arm ranks them out rather than filtering them.
   - The row's U-plate fixture went from 30 invalid bodies to all SOUND.
     The PR neither closes nor amends the row (MINOR 2).
6. **Sweep.** Conic rankings: only `find_match`/`loose_partners` (the
   `min_by` hits in `circle_torus`/`ellipse_roots` rank roots). Recorded
   face read after surgery: `join.rs:630`, `:1265-1281`, `AuxDatum::Partner`,
   `CurvedBooleanUnsupported`; I agree with the PR's classification, but it
   missed the sibling `rest.rs:969` `fragment_holding` (Style).

## Findings

- **MINOR 1: steep-ellipse mispair** (`join.rs:950-958`, `:983-990`,
  `:992-1046`; claim 1). Measured on 300 poses; they refuse and ship no
  wrong body. Owed: correct the two docstrings, and either rank by central
  angle or file a row with this battery as its fixture.
- **MINOR 2: open row untouched**
  (`work/join/join-ranks-conic-facing-germs-by-chord.md`). Its circle half
  is fixed, measured on the row's own U-plate shape, yet it still reads
  open and "unmeasured", and the ellipse half has no schedule.
- **MINOR 3: `wall_region` has no lineage or carrier guard**
  (`join.rs:901-927`). Found by reading; I could not build a failing pose.
  `rest.rs:969` `fragment_holding` guards the same question by lineage.
  Confidence: unsure.
- **NOTE 1:** the suite's header and the PR body say "the 37 poses"
  (`pocket_wall_crossing_a_side_face.rs:7`); the table holds 19×3 = **57**.
  The PR's revert count (96) also differs from mine (90).
- **NOTE 2:** `operand=false` bodies (union with a far brick refuses)
  already exist on main (1151 rows of my battery). I did not trace why;
  `outcome` counts them as BAD.
- **NOTE 3:** the PR head moved to `b98a5131`, which I did not review.

## Style

Questions exercised: Q1–Q7. For Q8 I read `join.rs` 600–1250 and
1700–1850, not end to end.

- **Q1, sure.** Three spellings of "which fragment of a divided face holds
  the segment": `wall_region` (`join.rs:901`, both halves, recorded first);
  `chord_join::segment_curve` (`chord_join.rs:3566`, first half only); and
  `rest.rs:969` `fragment_holding` (lineage plus sole common face). The PR
  cites the second as precedent but not the third. Look also at other
  readers of a stored `FaceKey` after `mef`.
- **Q1, sure.** `germ_arm` recomputes `germs_face_each_other`'s
  `axis·(u×dir)`, band, `malformed` closure and `Zero` desync string word
  for word (`join.rs:1003-1021` vs `:1765-1785`). Its comment admits it
  ("read again under its own name"). One decision, computed twice per pair.
- **Q2/Q5, sure.** `nearer`'s "the chord IS monotone in the travel" and
  `germ_arm`'s "`Zero`… sorts ahead" hold only for circles, and are why the
  byte-identical tilted battery was read as covering ellipses.
- **Q2/Q7, likely.** Per the PR body, the recorded-first rule exists to keep
  one test's *refusal kind* (`SectionInvariant` rather than a desync): a
  rule shaped by an expected error, not by geometry. It also makes the
  two-faces case asymmetric — refused unless one face is the recorded one.
- **Q3, sure.** The acceptance suite is circles only; no row in the change
  can go red on the ellipse arm, and the tilted battery's θ ≤ 0.3 excludes
  that regime.
- **Q3, likely.** No row exercises `nearer`'s Behind-vs-Behind branch, which
  prefers the farther site.
- **Q6, likely.** `CurvedBooleanUnsupported` citing a stale fragment is
  disclosed as cosmetic and unfiled ("the tracker is not comprehensive"):
  disclosed but unscheduled.
- **Q4, unsure.** The census dropped `("join.rs","slots","Coincide::Join")`,
  which had pinned `find_match`'s closure to its inner fn `slots`. The
  census attributes decisions to functions by position, so it moves with a
  refactor — weak evidence about decisions.
- **Q7, unsure.** `Reach`/`GermArm` hand-code a two-level lexicographic
  order. I would have reached for the angle in the germ's sense, monotone on
  any centred conic. The PR's turn×radius attempt (which cost 60 tilted
  rows) was a single key, so the angle as a secondary key was not tried.

REVIEW COMPLETE
