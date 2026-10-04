# Review r1: PR 4026 at frozen head `93ecd145`

**Verdict: APPROVE-WITH-FIXES** · MAJOR 1 · MINOR 4 · NOTE 4.
The mechanism holds up. On 15 552 grid runs over six prism corners it ships no BAD body (convex, mirrored and bottom corners included). The four sampled batteries do not move. Every mutant goes red. The ∩ rule's both-In keying is the only reading that survives measurement. One near-tangent pose newly ships a body with a definite tier-3′ violation where main refused. It has to be refused, or filed with this pose as its row, before merge.

**Method.** Release builds of head and of main as the merge brought it in (`e4a0a0d1`, see N4) in separate target dirs. My probe battery is `crates/sweep/examples/r1_pierce_probes.rs`. It prints one `differential::outcome` line per op×order: tier 2, tier 3′, the certificate, a legal operand, and the volume to 1e-7. Its oracle is my own: each convex piece of the prism is clipped by the cube's half-spaces, and the volume comes from the divergence theorem. That is a different algorithm from the PR's vertex enumeration, and it agrees with the kernel on 420/420 built Ltop two-run lines. For a ball, the oracle is an exact chord integral with tanh-sinh quadrature in z. Each line carries `lobes=k`, the sampled number of arcs of the tangent plane inside the corner's cone. Mutants and ∩-rule variants are env switches in `review-r1/instr-mutants.patch`. The evidence lines are in `review-r1/`.

## Claims

1. **No wrong body ships: FALSIFIED, at one near-tangent pose.**
   - Grid: 240 Fibonacci directions plus 192 near-tangent tilts (±1e-3, ±1e-6), × {Ltop (1,1,1), Lbot (1,1,0), Lmirror, notch307 (V-notch, 307°), shallow200 (≈200° reflex), convex corner} × 6 op/orders = 15 552 lines.
   - Head ships 0 BAD among the 240 directions. Main→head moves no line from SOUND to a refusal or to BAD.
   - The ∩ op rule never builds a wrong facing. Under every variant of the rule (claim 2), each failure is a typed refusal.
   - The near-tangent sweep at d ∈ ±{1e-2, 1e-4, 1e-5, 3e-6, 1e-7, 1e-8} (`r1-nt-*.txt.gz`) moves 65 lines from refusal to BAD (M1, m1).
   - Curved pierced face: no reach (N1).
2. **The ∩ rule is a rule, not a fit: PARTLY HOLDS.** The *keying* is measured and correct. The *facing* is a measured constant.
   - Keying (executed, `mode-*.txt`, 340 two-run poses per order):
     - extend the rule to "piercing keeps In": cube∖prism goes from 169 SOUND to 0;
     - extend it to "pierced keeps In": prism∖cube goes from 340 SOUND to 171.
     - So only both-In fits, and ∩ is the only op that keeps both In.
   - Facing: the rule always faces START. The walk itself says START in 171 of the poses and END in 169.
     - Where the walk says START, the rule is a no-op.
     - Where the walk says END, the rule turns 169 `SelfLoopEdge` into 69 SOUND + 100 `JoinDesync` per order.
     - Always facing END (my mutant `inter_end`) refuses all 169 + 171.
   - Nothing derives "start" from germ geometry or from an orientation convention. The vtxfac comment gives a topological purpose, not a derivation (S3). No pose was found where the rule builds a wrong body.
3. **The welds are sound: HOLDS (executed + inspection).**
   - Every built body in the grid passes t2, t3′ and the certificate. That rules out an inverted or non-manifold loop from `Joint::Hole` / `kfmrh` or from `Loops`.
   - The PR's no-`kfmrh` mutant is the one that shows `Hole` is reached.
   - Copies left apart: prism∖cube, {+x,+y}-Out family, two vertices on one point. They pass t3′ by the census's same-point rung, which is legal under PR 3813 (`work/cleave/boolean-declares-no-touching-…md:21-27`).
   - `boolean_pinch_copies.rs:118-138` asserts one vertex per point only on its own declared poses. That is consistent, but see S5.
4. **Nothing else moved: HOLDS (executed).**
   - `rc_wide_battery` 40 320, `join1_r1_reflex_battery` 1 152 and `rw_battery` 2 250 are byte-identical, main vs head.
   - `pierce_runs_battery`, run on main by copying the file in: 127 moved, exactly the PR's 110 + 8 + 9.
   - My own vertex-on-face battery: the head-vs-main diff on the 240-direction grid moves 0 lines from SOUND.
5. **The mutants are real: HOLDS (executed, `instr-mutants.patch`).** Every one goes red against `join_pierce_strut_facing` while clean head is 6/6 green:
   - the PR's "ignore the walk" (`nowalk`), "drop ∩" (`never`), "no weld" (`R1_NOWELD`) and "no twin pick" (`R1_NOTWIN`);
   - my own `inter_end`, `piercing_in` and `pierced_in`.
6. **The first-wrong-state story: HOLDS at the row's tilts, but its "only the lone-edge family" clause is too narrow (m2).**
   - Method: head with main's facing restored (`nowalk`), not main instrumented directly.
   - With main's facing restored, the row's pose fails first at ∩ `SelfLoopEdge`. ∪ and ∖ build. With `R1_NOTWIN` the failure is the `kept_end` refusal, and with `R1_NOWELD` it is the two-vertex face.
7. **The filed rows: MOSTLY HOLD.**
   - Every pose and count I checked reproduces: `i=0 j=3..5`, `i=2 j=5` ∩ "derived ring role order"; `i=6..8 j=0..2` cube∖prism `SelfLoopEdge`; 526 vertex-vertex refusals (243 `PairingMismatch`), against main's 527.
   - Each row's stated family is narrower than what refuses (m3). The P3 row's next door is not built (m4).

## Findings

**M1 (MAJOR, executed). A near-tangent two-run pierce ships a body with an undeclared edge-edge overlap. Main refused it.**
- Pose: `shallow200 nt e0 a3 d1e-7`. The prism profile is (0,0),(4,0),(4,1),(2,0.6),(0,1), with v=(2,0.6,1). The plane is tilted 1e-7 off containing the edge toward (4,1).
- prism ∪ cube and prism ∖ cube build with the exact volume (67.136613616 / 3.136613616), t2 and the certificate. But `validate_pseudomanifold` gives `UndeclaredContact { EdgeEdgeOverlap }`, witness (3.99676, 0.99935, 1.0).
- Main refused both: "null-edge copies have not exactly one kept end".
- Root: the 1e-7 sliver lobe puts the section edge within tolerance of the prism's top edge. The near-coincidence was missed upstream, and the PR's new reach (`finish.rs` `kept_ends`, `ops.rs:613` weld) carries it to rest instead of refusing.
- Fix: a typed refusal, or a P0 row pinning this pose. It must not merge as a silent new BAD reach.
- Repro: `R1_NT_D=1e-7 r1_pierce_probes cube shallow200`.

**m1 (MINOR, executed). 64 more near-tangent poses newly ship bodies that tier 3′ cannot certify.**
- All are `CensusEscalated` (ee_span/ee_parallel/ee_overlap), mostly at d ≤ 3e-6 on notch307 and shallow200. Main refused them.
- Main already ships 138 such escalated bodies in the same band (all escalations, none definite). So the class pre-dates the PR, but this PR widens its reach.
- I found no row tracking boolean results that ship with a tier-3′ escalation; the fix pass should file one or point to it.

**m2 (MINOR, executed, claim-only). Main's facing fails ∪/∖ in more than the lone-edge family.**
- With `nowalk`, ∪/∖ refuse in 171 of 340 two-run poses per order: 70 `SectionLoopMixed` and 101 `JoinDesync` ("derived ring role order" / "every chord arc…").
- On Ltop these are exactly the poses where the walk says START: every pose with +x and +y Out, including runs widened by a side-face bisector (bisector reading, *likely*).
- The PR body's "The facing was wrong for the lone-edge family" and the row's `## Built` should say that family.

**m3 (MINOR, executed). The filed rows' families are narrower than the refusals measured.**
- Cube∖prism `SelfLoopEdge` refuses in all 30 Ltop poses with {+x,+y} Out, not only where both runs are lone edges (`…two-edge-runs-refuses.md:20-24`).
- notch307 and shallow200 ∩ refuse with "every chord arc separates a loose scaffolding pair" in 17/35 and 7/15 two-run poses. No filed row names that message.
- Ltop ∩ "derived ring role order" also appears where only the top bisector and −z read Out, outside the wide-run row's family (*likely*: bisector reading).

**m4 (MINOR, inspection). Three or more runs build unguarded.**
- `vtxfac.rs:760-772` walks from `(i+1) % k` for any k, and the struts hang in run order.
- The P3 row says this order is unmeasured and that the fix is "refuse typed until a row pins the order". No refusal was added, so a ≥3-run pose would build in an order nobody has checked.
- It is unreachable with today's fixtures (I could not build a 3-run vertex). Still, an unguarded wrong-body path rates above P3.

**N1 (NOTE, executed): curved pierced faces are unreachable.**
- A ball (r=3) with v on its surface refuses every two-run pose typed: `SectionNotPolar` / `SectionArcSide` / `RingOffCylinderChart` / `Escalated`.
- That holds with the pole perpendicular to m and with the pole along z (`r1-ball*.txt`, 1 512 + 1 512 lines). The ball oracle's ball volume matches the kernel's to 1e-12.
- The PR's "unexercised on curved faces" stands; there is nothing to falsify yet.

**N2 (NOTE): the ∩ rule's guard.** The row goes red without it (claim 5), so the measurement is guarded. But the claim site does not say it is measured (S3).

**N3 (NOTE): the PR's Report gives `cargo nextest … 4 286 passed` on a pre-final head.** I ran the six rows of `join_pierce_strut_facing` on the frozen head (green); I did not re-run the full suite.

**N4 (NOTE, dispatch premise).**
- The brief's base `f8aabaae` is not the PR's merge base. Head merged main at `e4a0a0d1`, and `git diff f8aabaae 93ecd145` spans 268 files of intent-vars work.
- The PR's own diff is `e4a0a0d1..93ecd145` (16 files). Nothing between the two touches `crates/topo`, so the battery comparisons are unaffected.

## Style

Questions exercised: Q1 (a prose sweep and a parallel-function read), Q2, Q3, Q4, Q5, Q6 and Q7. Q8 only partly: I read `finish.rs` header to `discarded`, not `ops.rs` (5 156 lines).

- **S1 (Q1, likely)** `finish.rs:500` `weld_pierce_copies` vs `finish.rs:575` `weld_pinches`. The docs say it welds "as a pinch weld joins two pierces", but the two have drifted:
  - it admits any face (`|_| true`), where `weld_pinches` excludes section faces and non-lineage faces;
  - it has no `one_vertex` check;
  - it has no "did not fuse its pair" postcondition;
  - it refuses a `Chord`, which `weld_pinches` records as a fragment.
  Two spellings of one weld rule.
- **S2 (Q1, sure)** `vtxfac.rs:788-805` is a new side-follows-facing match, mirroring `vtxfac.rs:671-680`. It is disclosed, and tracked by `strut-side-follows-facing-is-spelled-three-times` (P2). It is the trap the style lane names: the fix mints another copy. Within one function the conventions are opposite: the piercing side's start germ is "UP", the ring's is "DOWN".
- **S3 (Q2/Q6, sure)** `vtxfac.rs:749-751`. The comment justifies the ∩ case ("so the ring's In face passes a copy per run") as if it were derived. The PR body calls it measured, and claim 2 shows the walk disagrees in half the poses. The measured status belongs at the claim site, along with a scheduled row to derive it.
- **S4 (Q7, unsure)** `vtxfac.rs:768`. The walk levers pierced-face germs at the piercing sector's `arm`, a chord from the other body. The units work out, but the arm is not the pierced entry's.
- **S5 (Q4, likely)**
  - `work/cleave/boolean-declares-no-touching-…md:35` records "22 138 results… none holds two vertices on one point… never reached". This PR's prism∖cube family reaches exactly that, and nothing re-cites it.
  - `boolean_pinch_copies.rs`' module doc ("copies… do not reach rest as two touching vertices") now has a sanctioned exception.
- **S6 (Q5, likely)** `finish.rs:1-8`. The module doc lists `weld_pinches` but not `weld_pierce_copies`, which runs after the zips (from `ops.rs:613`). It is a post-zip pass hosted in the pre-zip module.
- **S7 (Q3, likely)** `pierce_runs_battery` is print-only (`#[ignore]`). Nothing in it goes red. Its value depends on someone diffing it by hand, and the PR's 0-BAD claim has no assertion behind it.
- **S8 (Q6, likely)** `finish.rs:724` / `zip.rs:104`. `weld_pinches` now takes `Hole` too, and the PR says "No row reaches that path there". That is a behaviour change on an unreached path with no row and no schedule.

No lane-isolation glimpse: I read no other review branch or session.

REVIEW COMPLETE
