# Review r1 of PR #4008 (frozen head d930c23f)

**Verdict: NOT-MERGEABLE-AS-IS.** MAJOR 1 · MINOR 3 · NOTE 6.

The PR fixes most of what it says it fixes, and the planar batteries really are byte-identical. But
the premise the new order rests on is false on a steep ellipse: *"within one half-turn the chord IS
monotone in the travel"* (`crates/topo/src/boolean/join.rs:956`). My own battery reaches that
regime, where the order picks the wrong partner. In one pose this ships a body that fails tier 3′
and the certificate where main refused.
Method: release builds of main (merge-base 4c9d9385) and the frozen head. I also built an instrumented copy of the head (not committed). It prints every
`partners` candidate (site, arm, chord), every `find_match` pick and every `wall_region` branch, and
has env switches that undo each fix separately (`R1_CHORD_ONLY` gives the chord-only order,
`R1_RECORDED` gives the recorded window). Probe rows are in
`crates/sweep/tests/review_r1_pocket_ring.rs` (all `#[ignore]`): `r1_steep_ellipse_battery` (960
rows by default; `R1_K` and `R1_PSI` override), `r1_steep_ellipse_regressions` and
`r1_steep_ellipse_partner_probe`. The shape is a cylinder (r = 1, z ∈ [−8, 8]) against a convex quad
prism whose cap lies in the plane z = k·x, turned ψ about the axis. Each long edge of the quad cuts
the cap's ellipse twice. Volumes are checked against a polar quadrature of cylinder ∩ prism, good to
about 1e-6 (tolerance 2e-5). The PR head moved to b98a5131 after the freeze; I
reviewed d930c23f only, and read no other lane's branch or PR comment.

## Claims

1. **Arm-then-chord picks the right partner on every conic germ line — FALSIFIED (MAJOR 1).** On the
   ellipse P(φ) = (cos φ, sin φ, k cos φ), take a germ at φ₀. The squared chord to φ₀+s is 2(1−cos
   s) + k²(cos(φ₀+s) − cos φ₀)². When k² > 2 (tilt above 54.7°) and the germ sits near a minor
   vertex, this peaks near s = π/2 and then falls inside the `Ahead` arm. Two runs show it:
   - k=3, ψ=95° (cap sites at φ 90/190/195/260): on the cap ellipse the head picks c(90)↔e′(260)
     (Ahead, chord 2.059) over f(195)↔e′ (2.608) and c↔e(190) (3.328). It is then forced into e↔f
     across the gap (Behind). Every one of the 12 runs refuses (`RingHomingAmbiguous` /
     `SectionLoopMixed`).
   - **k=6, sites [200,230,300,120], ψ=1.75, against, I AB and S AB.** Head ships a body with t2 =
     ok and t3′ = cert = `Err([LoopRoleInverted { face: 9v9, loop: 8v7 }])`. Main refused the same
     pose with `SectionArcWindow{NeitherContained}`. The trace explains why: germ (2,0) at azimuth
     0° has two Ahead candidates, (5,0) at 330.3° (travel 29.7°, chord 2.834) and (9,0) at 220.3°
     (travel 139.7°, chord 2.693). `find_match` takes (9,0)↔(2,0), jumping over (5,0). (5,0) is then
     forced into the Behind pair (8,1)↔(5,0). Undoing either fix alone still ships the BAD body;
     undoing both restores main's refusal. So the wrong pair is a latent defect of main's chord
     ranking, which `nearer` keeps. Main's window happened to stop it, and either fix now lets it
     through. Circles are sound for `find_match`: every wrong pair contains a true pair with less
     travel, so the greedy global order takes that one first. Sites at the half-turn or at the
     germ's own azimuth go `Ahead` correctly. The half-turn test itself is fine on an ellipse, since
     it is an azimuth test. What breaks is the chord used *within* an arm.
2. **Nothing straight-line moved — HOLDS.** I ran main vs head byte for byte on 14 batteries: j3r2
   {tilted, pocket, rand, groove, snowman}; join1_r1 {·, seam, declared, tube, reflex,
   bored_capsule}; join1_delta {arc, brick}; rc_wide. Ten are identical. `j3r2_pocket` has 234
   changed rows, and all 234 go to SOUND (198 `RingHomingAmbiguous`, 36 `SectionArcWindow`).
   `join1_r1_seam` (18) and `join1_r1_tube` (3) change only the refusal's face key. This matches the
   PR's table. *My* battery is a different story (MINOR 1, and MAJOR 1 above).
3. **`wall_region` is right — HOLDS, with a caveat; no wrong-arc construction found.** Instrumented
   branch census: across all 342 runs of the acceptance suite, 1506 calls take *both halves on
   recorded* and 90 take *both halves on another single face*. In `reach_continuation::a_tangency…`
   10 calls are both on recorded and 6 have **one** half on recorded (the shared-carrier fillet
   case). The split-face refusal arm never fires. The code returns `recorded` when **either** half
   is on it (`join.rs:918`), not when both are, as the brief says. A region holding both halves
   holds the chord, so its window holds the right arc. A wrong arc can only come from the one-half
   branch, which is main's old behaviour and fires only on a pose that refuses downstream. Unsure: I
   did not build a mef-divided recorded face with the halves split across its fragments.
4. **Oracle exact; red under each revert — HOLDS.** I polygonized the D independently (200k-gon,
   Sutherland–Hodgman against the square). It agrees with `disc_clip_area` on all 19 groups to about
   2e-11. The suite goes red under each switch: `R1_CHORD_ONLY` gives 198 `RingHomingAmbiguous`, and
   `R1_RECORDED` gives 90 `SectionArcWindow` (the PR says 96; NOTE 3).
5. **Scope — the open row is PARTLY fixed and unmentioned (MINOR 2).**
   `work/join/join-ranks-conic-facing-germs-by-chord.md` names `find_match` and `loose_partners`.
   Its first shape (a back-to-back site across a gap beating a true partner less than π away) is now
   ranked correctly, because Behind loses to Ahead. Its second shape is not fixed: *"true partner
   more than a half turn away … another opposed-sense site past it, nearer by chord"*. Within
   `Behind`, the chord *falls* as the travel grows, so ascending chord picks the furthest site, and
   `nearer` (`join.rs:959`) ranks both arms ascending. Worked case: a CCW germ at 0, the true
   partner at 200°, another CW site at 300° gives chords 1.97r and 1.0r, so 300° wins. `find_match`
   survives this only through its greedy global order. `loose_partners` (`join.rs:1801`) ranks per
   half, so it records the wrong partner in the separation constraint, as main's chord ranking did.
   `germs_face_each_other` (`join.rs:1742`) still accepts back-to-back pairs; they are only ranked
   later. The PR body never names the row, and the row is not updated.
6. **Sweep — no new ranking site found.** `bool_join_nearest` has its single decision site in
   `nearer`. The `min_by`/`sort_by` hits in `boolean/*.rs` are root finders or key sorts. `rest.rs`
   no longer pairs germs (its row is closed). Recorded-face reads after surgery:
   `germ_section_frame` and `field_source_evidence` key on the surface, which a mef fragment shares,
   so they are fine. `bool_planar_curve`'s `partner_face` is a cache key, as the PR says. One stale
   prose premise: `crates/topo/src/boolean/finish.rs:514` says *"the join escalates that waist at
   `bool_join_nearest`"*. Candidates in different arms now never reach that decision (NOTE 4).

## Findings

- **MAJOR 1 — the within-arm chord is not monotone on an ellipse. Wrong pair, wrong body.**
  `join.rs:950-970` (`nearer` and its doc), and the `germ_arm`/`nearer` pair in `find_match`
  `join.rs:1085`. Shown by the instrumented trace, plus `r1_steep_ellipse_regressions` row 5 (k=6,
  ψ=1.75, I AB) printing `LoopRoleInverted`. The R1_K=2,3,4,6 ×
  ψ∈{1.45,1.55,1.658,1.75,1.85,4.6,4.8} scan (1344 rows): main has 22 SOUND and 20 BAD (all k=6,
  cert=false); head has 863 SOUND and 3 BAD. Two of the head's BAD rows are new (refusal→BAD) and
  one is shared. The PR wins big on net, but it ships a wrong body where main refused.
- **MINOR 1 — 7 SOUND→refusal rows in my battery.** One is k=0.5 [200,230,300,120] ψ=0 against U AB:
  main is SOUND at v=55.1547452, head gives `VolumeUnmeasured{RingOnCurvedFace}`. The switches show
  the arm order alone causes it. Here the head's pair is the travel-correct one,
  (6,0)@200°↔(1,0)@180°, where main took the back-to-back (6,0)↔(2,1)@230°, and the head then hits a
  ring-on-curved-face frontier. The other six are k=6 [225,240,320,110] ψ=1.658 and 4.8 against, U
  AB, U BA and S BA, which go to `RingHomingAmbiguous`.
- **MINOR 2 — the Behind arm is ranked backwards, and the open row is left open and unmentioned.**
  Claim 5 has the detail.
- **MINOR 3 — test gap.** No committed battery reaches k² > 2: tilted only turns θ ∈ {0.15, 0.3}
  (`join3_r2_probes.rs:305`), so "byte-identical tilted" cannot see MAJOR 1. In my default battery,
  head is 895 SOUND / 7 BAD and main is 203 SOUND / 2 BAD.
- NOTE 1 — the 7 head "BAD" rows in my default battery are t3′ `CensusUndecidable` on a correct
  multi-lump S BA body (volume matches, cert ok). This is the census's curved-contact limit, not a
  wrong body. Main refused these poses.
- NOTE 2 — the suite header says *"The 37 poses are the ones … refused on"*
  (`pocket_wall_crossing_a_side_face.rs:7`). GROUPS×ENTRIES is 57 poses (342 runs). With both
  switches on, 234 runs fail across 45 poses.
- NOTE 3 — the PR body says reverting the window gives "96 `SectionArcWindow` lines". My
  recorded-only revert gives 90.
- NOTE 4 — `finish.rs:514`'s premise about `bool_join_nearest` (claim 6).
- NOTE 5 — the brief describes `wall_region` as "recorded first if both halves still sit on it". The
  code says *either* (`join.rs:918`). The doc comment matches the code.
- NOTE 6 — the census edit removes `("join.rs","slots","Coincide::Join")` (`offer_rows.rs`). That
  row attributed `find_match`'s `escalate` to the nested `fn slots`. The census attributes decisions
  by text position, so a nested fn steals its parent's rows.

## Style

Exercised: Q1, Q2, Q3, Q4, Q6, Q7. Q8 was partial (I read join.rs's module header and lines 470–1240
and 1740–1840, not all 3444 lines).

- `join.rs:1003-1022` vs `join.rs:1768-1785`: `germ_arm` re-derives the rotational sense, the
  `malformed` closure and the desync string word for word from `germs_face_each_other`, and its doc
  calls this "read again under its own name". Two copies of one decision that must agree. **sure**
- `join.rs:950-958`: the `nearer` doc states an invariant (within-arm monotonicity) that nothing
  enforces and that is false on ellipses (MAJOR 1) and inverted in `Behind`. The justification runs
  longer than the code. **sure**
- `join.rs:940-948`: the `GermArm::Behind` doc says every Behind site is further than every Ahead
  one. True, but it invites the reader to treat Behind like Ahead, which is the MINOR 2 bug.
  **likely**
- `join.rs:886-899`: the recorded-first rule exists, per the PR body, to keep a refusing test's
  refusal flavour (`SectionInvariant` and not a desync). A rule whose measured reason is the wording
  of a refusal reads as fitted to a fixture. **likely**
- `join.rs:924-928`: the split case is a `JoinDesync` (a kernel-bug class) for what may be reachable
  geometry (halves on two fragments). **unsure**
- `join.rs:976-990`: the long argument that `Zero → Ahead` is "conservative, not a tie-break" holds
  on a circle. On a non-monotone ellipse "the furthest any Ahead site can be" is not the largest
  chord. **likely**
- Q3: the acceptance suite fixes pose sets where the chord is monotone (a vertical cylinder against
  a horizontal cap, so circles). It cannot go red on the ellipse defect. **sure**
- Q6: the counts in prose (37, 96) are unguarded numbers (NOTE 2, NOTE 3). **sure**
- Q7: I would rank by azimuth travel (the arm test already has the azimuth plane) rather than by
  chord within an arm. The PR body says turn×radius cost resolution. An azimuth turn order without
  the radius might not. **unsure**
- Class to sweep: every "nearest along a curve, taken by chord" site. Today that is only `nearer`.
  Its callers are `find_match` and `loose_partners`. The REST zip no longer has its own. **likely**

REVIEW COMPLETE

