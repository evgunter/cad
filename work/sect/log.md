# SECT — the log

## 2026-10-10 — opened

Opened by JOIN's orchestrator at JOIN's close (Ev approved the plan in
chat on 2026-10-10; `docs/doc-ledger/join-leaves-the-tracker.md`). Its
charter is the section-first join Ev ratified in `[ev]` PR 4313 (fork
row 100), whose build was gated on INTENT stage 4's PR A; PR 4364
merged 2026-10-09T21:06Z, so the track is startable.

Ten rows moved from `work/join/` by `git mv`, ids and bodies unchanged:
the cylinder × sphere lane and tangency rows, the skew-cylinder and
torus frames, the one-site and along-edge rings, the in-band axis
offset, and three order and margin rows. Two unpriced rows were priced
at the move. SECT owns the curved section-frame and lane rows outright:
no per-pair stopgap lane is built elsewhere. Rows dispatchable: 10, for
31 points against a budget of 31.
— (JOIN orchestrator)

- 2026-10-14 — `closed-in-face-section-loop-has-one-site` closed by PATHS on `paths/in-face-section-self-loop` (`one_site_segments`: the wrap crossing and the closed-edge arm in one owner). (PATHS)

- 2026-10-14 — #5159's fix pass files two rows here: `a-closed-edge-lying-in-a-cone-cylinder-or-torus-face-has-no-join-arm` (P1, the one-site arm's residue; refused today at the crossing layer) and `a-closed-edges-germ-read-on-a-planar-face-would-take-the-placeholder-chord` (P3, the PR's disclosed blind spot, unreached). `RimSide::of` is now the one table of pierced faces the join and `join_lone_ring` read. (PATHS)

- 2026-10-25 — **Re-cut (refold approved in chat):** in, `the-lilys-stem-and-arch-walls-have-no-certificate-arm` from ORBIT (P0, lily wall 1) and `c5-plane-torus-cone-cylinder-arms` from GERM (P1), beside the torus frame (P0, teapot wall 2); `cylinder-sphere-germ-pair-has-no-join-lane` to P0 as the section object every P0 here is built on; `in-band-axis-offset-…` to P1 as it rides with step 1. Out, the six P2/P3 rows to SECTTAIL on the priority seam. Budget 36 (`program.md` says why). SPLICE, opened the same day, needs this track's rung-3 arm for klein wall 3 and teapot wall 3.

## 2026-10-25 — a row filed by SPLICE

(SPLICE orchestrator) Filed `a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm`
(P0 H, design open, parked on step 1 and on SPLICE's gate deletion): the
rung-3 arm that klein wall 3 and teapot wall 3 need. Step 4 of this
plan mentioned it only as a sentence. Agreed in chat that SECT should
get its own orchestrator rather than SPLICE taking these rows.

## 2026-10-27 — picked up

(SECT orchestrator) Track claimed, status `ready` → `active`. INTENT E
(`booleans-glue-on-zero`) closed 2026-10-10, so the section door's
coincidence seam the plan names is no longer a wait: E's record exists
and moves to the door with the frame's retirement. First dispatches:

- **Step 1's open choices to the designer pair** (one Opus, one Fable,
  `docs/prompts/designer.md`): the section object's type and home, its
  certificate's rung per kind pair, and the door's coincidence/tangency
  seam with E's record, as the brief for
  `cylinder-sphere-germ-pair-has-no-join-lane` (and its two riders)
  finds them open. Converged and inside fork row 100 → the brief is
  written from it; a split → an `[ev]` PR.
- **`the-lilys-stem-and-arch-walls-have-no-certificate-arm`** (P0 M) to
  an implementer now: a section-certificate arm in `section_cert.rs`,
  which does not wait on the section object. Review tier: single FULL
  (a certificate deciding a pair clear is a soundness claim a read
  cannot believe on its own).

- 2026-10-27 — **C5 torus survey** (read-only lane). Every torus pair but
  plane × torus is `Rung::General`, `implemented: false` at `route`
  (`geom_brep::intersect`), "blocked on the torus's exact meters
  conversion"; the section certificate (`section_cert` `torus_pair`)
  already classifies every torus × sphere pose and the coaxial,
  parallel and square torus × cylinder poses, so the certificate runs
  ahead of C5. The block is real in code (`ssi/certify.rs`
  `composite_form` refuses a torus; so does `ssi/enclose.rs`
  `implicit_enclosure`, which exhaustiveness reads), but its stated
  reason ("certification arithmetic does not take a root") is stale:
  `Certification::sqrt` exists. **Teapot wall 2's critical path is
  therefore CHARTTAIL's `torus-certificate-runs-root-free-through-its-quartic`**
  (P2 H, designed in fork row 30, waits on no ruling) plus a torus SSI
  door; by the blocker-inherits-the-band rule it is P0 work. Whether
  SECT claims it (budget) or it is re-banded in place is decided once
  step 1's design is in, since the section object fixes how C5's torus
  arms are read. Teapot's pose: torus axis +z square to the pot's +y,
  through neither centre (no closed form). Lily's: parallel axes in
  one plane, offset `R₁ − R₂`, spines internally tangent at the fork,
  so "clear" holds only within the faces' windows (sent to the lily lane).

- 2026-10-27 — **Step 1's design converged.** The two designers (blinding
  record on `analysis/design-fork/the-section-object`) reached one final
  state inside fork row 100: a section value in `geom_brep`, built by the
  only kind dispatch, and a per-operation table in `topo::boolean` read
  by the glue records, the guard, the germ mint and the join. They
  differed only in sequencing. One kept the cs frames reading the table
  until step 3; the other retires them in 1a through foot-parameter
  matching. I took the retirement: it removes two of the five offset
  computations and is what the pinned matcher test exercises. No `[ev]`
  PR, and no fork-log row, since nothing went to Ev. Both designers
  proposed re-wording the ratified row ("once per surface pair"; name
  `section_cert::classify` under "kind dispatch only in C5"). I read both
  as elaborations of row 100, not changes to it, and wrote them into the
  1a row. Step 1 is split:
  `the-section-of-a-surface-pair-is-one-object` (1a, P0 H, dispatched,
  dual concurrent review) and the lane row as 1b, parked on it. The two
  riders ride 1a. `in-band-…` is decided: escalate. The tangency row now
  counts five offset computations.

- 2026-10-27 — **`the-lilys-stem-and-arch-walls-have-no-certificate-arm` closed** by PR 5869 (`torus_parallel_torus`, the general parallel-axis tori arm): lily wall 1 builds, both union orders one shell at the Pappus volume, ∩ and the differences too. Review: single FULL (a certified-clear verdict is a soundness claim), APPROVE-WITH-FIXES, no MAJOR; soundness held under a 1,100-pose fuzz against an independent chart trace; the fix pass landed the reviewer's two rows, stated the bare-margin invariants at their sites, moved the chart trace into `search_kit` and the decided insertion sort into `splitting::order`, and filed `circle-pair-crossing-is-spelled-three-times` (P1 E) here and three P2/P3 rows on SECTTAIL. Merged over an inherited red (`demos (tour + wild)`, the tour gallery's chain scene, #5853's), filed on ATREST as `the-tour-gallery-pin-reads-the-chains-old-could-not-look-count`.

- 2026-10-27 — **Claimed `torus-certificate-runs-root-free-through-its-quartic`** from CHARTTAIL, re-banded P0 (teapot wall 2 waits on it). Dispatched to an implementer on `sect/torus-certificate`. Review: single FULL, because the arms are certificate soundness and a read cannot carry the bound. It goes to a dual if the diff reaches the shared composite machinery beyond the torus arms. First route pair: sphere × torus.

- 2026-10-28 — **Step 1a merges** (PR 5872, `the-section-of-a-surface-pair-is-one-object` and both riders closed; 1b, `cylinder-sphere-germ-pair-has-no-join-lane`, is open). Review: dual concurrent (H), both APPROVE-WITH-FIXES, 0 MAJOR. Both reviewers ran as remote cloud sessions because local disk could not hold two builds. Row `docs/dual-review-log/rows/PR-5872.md`. The fix pass is in that row. My delta read caught a foot cache keyed by bare `HalfEdgeKey` across both operands' arenas. It is unreachable today, now keyed `{side, he, pair}` and pinned by a red-first probe. **Classes** recorded for later steps:
  - a cache or table keyed by an arena key must name its body;
  - the cs march near the node costs seconds and refuses by budget (`a-cylinder-sphere-march-near-its-node-costs-seconds-…`);
  - C5's kind dispatch is spelled in `route`, `route_pose`, `serves` and the value (`the-c5-kind-dispatch-is-spelled-twice`).

  Rows filed by the lane: those two, `section-arms-off-the-value-read-radii-unguarded`, and `the-recording-scalar-has-no-ssi-lane` (P3).

- 2026-10-28 — **`torus-certificate-runs-root-free-through-its-quartic` closed** by PR 5886. The torus arms of `implicit_enclosure`, `implicit_gradient_enclosure` and limb 2 use the root-free mean-value form of `g = F/(4rA)`, chosen on the measurement: the quartic as written overran the 200k cell budget, and the factored √ form was about 2× worse on a tilted axis. `sphere_torus_ssi` is retired first: both teapot spheres meet the handle torus in two certified loops in both orders, and `route` answers sphere × torus at rung 3. Teapot wall 2 still stops at the join's germ frame, which is the section object's work.
  - **Review:** single FULL, APPROVE-WITH-FIXES, one MINOR (executed). The break floor was unsound for `A < 0` on a spindle torus, and was reachable because the door skipped the ring check `route_pose` makes. The fix floors at 0 unless `A > 0`; the ring check now sits at the door and at the edge certificate (`SsiError::DegenerateTorus`). Soundness held under 20k adversarial boxes and 600 carriers, and every too-loose mutation went red.
  - **Merge:** main was merged over step 1a. Cylinder × sphere keeps its decided pose, and both march through the shared `implicit_pair_r3`.
  - **Rows filed:** cylinder × torus and torus × torus at rung 3 (P0: the torus germ frame row waits on both); cone × torus and tilted plane × torus (P2); and SPIRICARM's spline-trimmed torus volume lane.
  - **Class:** a door's contract must equal its route's (the ring check lived in `route_pose` only).
- 2026-10-28 — **Step 1b dispatched** (`cylinder-sphere-germ-pair-has-no-join-lane`, branch `sect/cylinder-sphere-chord`). The lane is a section datum per side, from the table. The chord is a sub-arc of the carried component, with `Pcurve::Projected` and `analytic_rung3` at the edge door. Island closure uses one reader (`path_island_winding`), gaining a spline arm in `LoopArc` and the cylinder in `ring_path::Quadric`. Review: dual concurrent (H), because the lane builds bodies. **Decided here (an elaboration, reversible):** a closed fitted component is *seated* once at its first site in match order (knot insertion to full multiplicity, then rotation, then re-certified), so no chord straddles the carrier's seam. Periodic NURBS in `geom` was the alternative. It is a representation change for every curve consumer, and seating does not rule it out later; one step-1 designer leaned seat. Reopen it as a fork if seating proves unsound.

- 2026-10-28 — **PR 5886 merged** (the torus certificate, sphere × torus at rung 3). It took two main merges: once conflicting with no CI run, and once on a slow-set conflict resolved as main's file plus this branch's one line. **The cylinder × torus and torus × torus rung-3 rows go to the designer pair first** (blinding record `analysis/design-fork/torus-pair-tangency`). Both rows name the same open question: how the door decides a pose is not transverse when the pair has no closed-form scalar gap like the sphere's. That must also settle how `pair_section`, the SSI doors and `section_cert`'s torus classifiers share the decision.

- 2026-10-28 — **The torus-pair tangency question converged** (two designers; record `analysis/design-fork/torus-pair-tangency`). Every torus pair decides "not transverse" on one pose value's **critical gap**, `min |D − (ρ₁ ± ρ₂)|` over the critical values of the spine-to-spine distance (offset duality), by a certified 1-D sweep. The symmetric closed forms move from `section_cert` into the value. The designers differed only on the sweep: 1-D over the core angle against 2-D over the torus chart with a separately named tube-circle arm. I took the 1-D form, which covers the tube-circle tangency without a special arm; that tangency is a required Zero row. One designer proposed a C5 sentence stating the principle; that would bind future work, so it is put to Ev, not written. Unit `each-torus-pair-decides-tangency-on-one-pose` (P0 H, dual) is dispatched. The cylinder × torus and torus × torus door rows are parked on it. Sphere × torus's double tangency decision (the 5886 door against `section_cert`) closes inside the unit.

- 2026-10-28 — **A main red traced to PR 5869** (reported by SPLICE; bisected by a SECT lane; suspected 5886, not it): `sweep` `germ_torus_doors::subtract_and_intersect_refuse_where_union_does`. `torus_parallel_torus` refuses the donut pair on an exact top-ownership tie (equal minor radii, h = 0) whose singular point lies outside the arm's triangle, and reports that proof gap as carrier tangency. The pair was traced transverse everywhere, at 23°–52°. A kernel fix is on `sect/two-tori-subtract-red`. **Classes:** a refusal for an arm's own proof gap must not speak as a geometric tangency; and a slow-set row on ground a PR edits is not run by per-PR CI, so 5869's green gate and its single review both missed a row that already pinned this pose. That escape is 5869's (single tier, not in the dual log).
- 2026-10-28 — **The 5869 main red is fixed** (PR 5925, `sweep` `germ_torus_doors::subtract_and_intersect_refuse_where_union_does`). `torus_parallel_torus` refuses a level-end tie only where W's node lies in the triangle or within its neck. Each branch gets its own neck, derived for its shape (`√(2(r+r₂)δ)` for slabs, `√(2·max(r,r₂)·δ)` for top and bottom), and δ comes from the row's own datum. This also closes SECTTAIL's `level-tubes-of-parallel-tori-refuse-at-the-node`. It files `r-tan-carries-the-arms-proof-gaps-as-tangent-carriers` (P2) for the wider wording class.
  - **Review:** single FULL. The first pass found one MAJOR by execution: the slabs neck took one sheet's deviation where both add, so it certified `none()` over a real intersection, a regression to a false certificate. The fix pass corrected it, and the same reviewer re-ran its probes on the fixed head: APPROVE.
  - **Class:** a clearance bound is derived per shape; one constant fitted to two shapes hid this MAJOR.
- 2026-10-28 — **Step 1b merges** (PR 5894, `cylinder-sphere-germ-pair-has-no-join-lane` closed). The cylinder × sphere chord lane and island closure stand on the section object. All 54 fixture runs pass the join and stop typed downstream, so none builds a body yet. **What waits:**
  - props, which has no `Projected` arm on an analytic chart (SPIRICARM, filed);
  - `edge_join`, which cannot join a spline carrier (SPLICE).

  **Review:** dual concurrent (H), R1 APPROVE and R2 APPROVE-WITH-FIXES, with no MAJOR. Row `docs/dual-review-log/rows/PR-5894.md`.

  **Classes** recorded for later steps:
  - an arc a reader trusts on-surface must carry that evidence in its type, not by convention;
  - an undecided nearness must escalate, never fold into a verdict (my delta read caught one);
  - a surface pair's section is carried in operand order (P3, filed).

- 2026-10-28 — **The torus poses merge** (PR 5924, `each-torus-pair-decides-tangency-on-one-pose` closed). Every torus pair decides tangency on one critical gap, and sphere × torus is back on main's closed form. The cylinder × torus and torus × torus door rows are open. Teapot wall 2's union now builds and is refused at validation (`VolumeUncomputable`, a conic trim on the sphere chart, which is SPIRICARM's volume lane).
  - **Review:** dual concurrent (H). Both reviewers were NOT-MERGEABLE on one bilateral MAJOR, by execution: f64 rounding in G′ divided by D ≈ 0 made in-band poses read Transverse, and a transverse joint read Tangent. Both confirmation passes then APPROVE on `e42da4cecb`. Row `docs/dual-review-log/rows/PR-5924.md`.
  - **Classes:** a margin that is divided must carry its own rounding first; a certifying sweep's premise must reach generic frames, grid-aligned critical points and scale.
  - **Filed:** SECTRUNG `the-torus-gaps-rounding-charge-is-first-order-and-reads-the-world-origin` (P2).
- 2026-10-28 — **Split on Ev's request in chat.** SECT was over its budget, and SECTTAIL was at 27/30.
  - **SECTARMS** (new, P0-led, 22 points) takes the section object's remaining transverse arms: NURBS × cone/sphere, the cone frames, skew cylinders and the walk order.
  - **SECTSITE** (new, P1, 16 points) takes the closed edge in a curved face and the C5 plane × torus / cone × cylinder residue (U-H1, D10-held).
  - **SECTRUNG** (new, P2/P3, 22 points) takes the C5 dispatch and rung-3 residue.
  - Points are at the tracker's current weights (E 1, M 3, H 8).
  - **SECT keeps** the torus-pair thread it is driving (teapot wall 2, the torus poses and their two doors), the new P0 main red, and two follow-ups its context makes cheap. The budget override is dropped.
  - All rows moved by `git mv`, ids unchanged. **Main red** `main-red-cylinder-sphere-section-runs-out-of-cells-near-the-node-at-1e-6` (nightly, ε 1e-6, slow-set rows from PR 5872) is dispatched on `sect/near-node-at-1e-6`.

## 2026-10-28 — GREEN pinned four rows on `main-red-cylinder-sphere-section-runs-out-of-cells-near-the-node-at-1e-6`

GREEN's full-suite pins (PR 5958, `green/full-suite-pins`) pin the four
rows that row names at ε 1e-6, with `test_utils::pinned_red::row`, each
on main's exact stop: `pair_section::one_loop_keeps_its_order_near_the_node`,
`pair_section::two_loops_keep_their_sites_and_order_near_the_node`,
`cylinder_sphere_frame::a_ball_across_a_cylinder_wall_wider_than_the_wall`
(written out of `pose_rows!` to carry its pin) and
`cylinder_sphere_frame::every_segment_joins_neighbouring_sites_along_its_loop`.
A pin goes red the day its row passes, so the fix on
`sect/near-node-at-1e-6` removes the four pins (and restores the macro
row) in the same PR, and re-measures what each row runs after the stop.

## 2026-10-28 — PR 5924 merged; the doors and teapot wall 2 dispatched

- **PR 5924 merged** (`each-torus-pair-decides-tangency-on-one-pose`, head
  b3ecfab, CI green; dual concurrent review, both confirmation passes
  APPROVE, its row `docs/dual-review-log/rows/PR-5924.md`). Lane reclaimed.
- **Dispatched `sect-torusdoors`** (branch `sect/torus-doors`): the two door
  rows `cylinder-torus-rung-three-arm-is-unretired` and
  `torus-torus-rung-three-arm-is-unretired` as one unit (H), since both
  owe the same march domain. Tier: **dual concurrent** — a certified rung-3
  march over a new domain, where a soundness slip is silent.
- **Dispatched `sect-teapot2`** (branch `sect/teapot-wall-2`): the sphere ×
  torus half of `torus-germ-pairs-have-no-section-frame`. The row carries
  `design: true`, so the lane measures and shapes first and stops with a
  problem statement if a fork shows (telling components apart at a site;
  the torus face's spline ring path); the designer pair then runs before
  any build. Tier: **dual concurrent** (H, join-side architecture).
- The two run concurrently: the doors are in `ssi`/`pair_section`, the
  teapot lane in the join's carried lane and `ring_path`; the seam is
  `pair_section::torus`, which each lane announces.
## 2026-10-28 — the near-node main red fixed (PR 5961)

- `main-red-cylinder-sphere-section-runs-out-of-cells-near-the-node-at-1e-6`
  closed by PR 5961: an R3 branch whose uniqueness tube its coarse chain
  boxes hold narrow is refined again (`ssi::refine::refine_for_tube`, the
  coarse spans read by `certify::coarse_tube_spans` on the tube ladder),
  keeping a finer carrier only where its tube is wider; a closed spline's
  foot polishes from both ends across its seam. Tier: **single FULL**
  (one Opus reviewer); the fix pass took all ten items, and the
  orchestrator's delta read found the ladder's "next rung up" preserved
  (`filter(r > radius).last()` over a descending ladder equals the old
  `2 · radius`). The four `pinned_red` rows main added at 1e-6 come out,
  since they pass. The near-node cost residue (`CellBudget` at the default
  ε, unmeasured cause) stays on SECTRUNG's
  `a-cylinder-sphere-march-near-its-node-costs-seconds-and-refuses-by-budget`.

## 2026-10-29 — PR 5972 in dual review; the components fork to the designers

- **`sect-torusdoors` opened PR 5972** (head f2bd69c, CI green): the
  cylinder × torus and torus × torus doors over a certified ball; both
  door rows closed in it. The lane stopped on a design fork, filed in the
  PR as `a-general-torus-pose-lists-no-components` (P0, `design: true`):
  a general pose of either pair has no closed-form component list, so
  `pair_section` still refuses it and the join stops at the germ mint.
- **Dual concurrent review** dispatched on the frozen head (H; brief on
  `briefs/sect-torusdoors-review`, both reviewers remote).
- **Designer pair** (Opus + Fable, concurrent, blind) dispatched on the
  fork, framed as the problem rather than the lane's three options, and
  widened to the `Sides::Untold` symmetric poses (square wall, parallel
  tori, a sphere round both tube circles), which ask the same question.
  `sect-teapot2` is told to keep to the sphere × torus poses whose
  closed-form readings already tell components apart.

## 2026-10-29 — the components fork decided; PR 5974 opened

- **The components fork** (`a-general-torus-pose-lists-no-components`):
  both designers, blind, reached one final state. The classification lists
  components only where a closed form counts them (`Counted`) and otherwise
  says they are the march's (`Marched`), never a refusal; `carry` lists the
  march's branches (a counted list becomes a cross-check); a site's
  component is its foot on the carried section at every reader, so
  `component_of`, `Sides` and the `Untold` refusals retire, and the germ
  mint's three-valued `Branch` gives way to a component index for table
  pairs; the certificate carries a `Marched` pose. Decided by the
  orchestrator, no `[ev]` PR: the text it replaces is agent-written (PR
  5872), and fork row 100's "foot parameters replace frames" already reads
  this way. Recorded with PR 5972's state sync; built as its own unit after
  PR 5972 merges.
- **`sect-teapot2` opened PR 5974** (head 0ebc2aa, CI green). Wall 2 was
  already past the join on main (PR 5924; `AroundTube`, told by a plane)
  and stops at SPIRICARM's volume door. The PR adds the torus ring-path arm
  for spline arcs, a matcher check in `join::along_facing`, a parity fuzzer
  and placement pins; the `AroundAxis` reading it had built was reverted
  for the fork. **Re-tiered from dual to single FULL**: the unit shrank to
  two gate widenings, one comparison and tests, where one review reads the
  whole change; dual was set at dispatch for an H join-side build that did
  not happen.

## 2026-10-29 — C5's critical-gap rule ratified (PR 5975)

Ev agreed in chat, then approved the wording on `[ev]` PR 5975 ("looks
good"), merged: every analytic surface pair decides its tangency and
coincidence once, on its pose, before any rung, read by every reader — a
closed-form margin where the pose has one, otherwise its certified critical
gap, never a root formula; NURBS and `Approx` pairs stay the C2
certificate's. It binds the cone pairs next (SECTARMS) and plane × torus
off its axis-aligned arm (SECTSITE).
## 2026-10-29 — the torus doors merge (PR 5972)

- PR 5972 closes `cylinder-torus-rung-three-arm-is-unretired` and
  `torus-torus-rung-three-arm-is-unretired`: both doors march a transverse
  pose over a certified ball holding the whole section; a general pose is
  `implemented: false` until the components unit lands. Dual concurrent
  review (row `docs/dual-review-log/rows/PR-5972.md`): both
  APPROVE-WITH-FIXES, no MAJOR; one fix pass on the union; the
  orchestrator read the `winds` fix. Cost noted: at ε 1e-12 the new
  `torus_doors_join` rows take 230–865 s each in debug, on the nightly.
- The components fork is decided and recorded on its row
  (`a-general-torus-pose-lists-no-components`, `## Decided`) and as fork
  row 212; its build is dispatched next as its own unit.

## 2026-10-29 — three lanes after PR 5972

- **`sect-components`** (branch `sect/components-by-foot`): the decided
  fork `a-general-torus-pose-lists-no-components` (re-costed H: it rewrites
  the value's list, `carry`, the germ mint and the certificate). Tier:
  **dual concurrent** (H, architecture).
- **`sect-oncedecided`** (branch `sect/torus-decided-once`): the two
  decided-twice rows, banded P1 under C5's ratified rule (PR 5975: a pair
  decides tangency and coincidence once, read by every reader):
  `torus-coincidence-is-decided-at-the-carrier-ladder-and-the-torus-pose`
  and `a-tube-circle-tangency-is-read-by-the-gap-and-by-the-glue-doors-tangency-lane`,
  one unit (both make a topo reader read the pose's decision). Tier:
  **single FULL** (M; one reader-redirect pattern, readable in one review).
- **`sect-circlecross`** (branch `sect/circle-crossing-once`):
  `circle-pair-crossing-is-spelled-three-times` (E). Tier: **orchestrator's
  read** (a mechanical extraction behind unchanged predicate names).
- Seams: all three touch `pair_section/torus*.rs` in different places;
  each announces its seam, and the components unit merges first where
  they collide. The cylinder-face ring re-homing row waits for PR 5974 and
  the components unit's germ-mint change in `join.rs`.

## 2026-10-29 — teapot wall 2's sphere × torus half (PR 5974)

PR 5974: a carried ring on a torus face reads its spline arcs (the torus
`ring_path` arm), and `join::along_facing` refuses germs on two components
up front as `facing` does. Wall 2 still stops at SPIRICARM's volume door
(conic trim), now pinned by exact payload; the matcher fix moves its
refused face (`6v1` → `1v5`) and ball × rod's on every op, which restores
the mirror symmetry main broke (row
`a_rod_through_a_ball_pairs_both_loops_alike`). Tier: **single FULL**
(re-tiered from dual at the PR, the unit having shrunk): APPROVE-WITH-FIXES,
no soundness defect (0 wrong of 2617 decided paths over 330 random poses);
one fix pass took every item, with the reviewer's random search landed
as `random_torus_sections`. The `Untold` placements wait on the components
unit.
## 2026-10-29 — two circles cross is spelled once (PR 6001)

`circle-pair-crossing-is-spelled-three-times` closed by PR 6001: the
coaxial-tube, parallel-tori and square-wall rows and profile's two
circle-pair sites read `geom_core::Circles` (`reach`, `secant` added).
Tier: **orchestrator's read** — margins bit-identical (logged decisions
equal across 952 tests, 800k seeded pairs at `f64` and `Interval`);
crossings now placed by `Circles2::crossings`, within 3.2 ulps of both
circles where the old form drifted up to 9.4e5 ulps, same order. The
unclamped `half_chord` is read only past a decided secant, so its square
is a product of decided-positive margins. Left as filed: the sphere ×
tube-circle rows (a verdict would move at `Interval`), P3.

## 2026-10-29 — a torus pair's tangency and coincidence decided once (PR 5994)

PR 5994 closes `torus-coincidence-is-decided-at-the-carrier-ladder-and-the-torus-pose`
(one decision, `geom_brep::tori_coincide`, read first by the pose in every
stance and by the carrier ladder's undeclared arm; the declared reading is
a different question and reads the same displacement) and
`a-tube-circle-tangency-is-read-by-the-gap-and-by-the-glue-doors-tangency-lane`
(two questions, argued at `tangency_reading`, one pose pinned showing both
answers right; no implication between them is claimed). Main read a nearly
coincident tilted pair Tangent (a wrong definite answer); head reads it one
surface at both readers. Disclosed moves: in-band pairs main built now refuse
undecided; corner sites decide One on the bound's sum, not each datum.
Tier: **single FULL**: APPROVE-WITH-FIXES, no MAJOR (the bound held against an
exact-distance oracle and an adversarial search); one fix pass took all 11
items, including one home for the identity walk (`geom_brep::identity_walk`,
which topo's `identity_data` reads — the orchestrator checked its ranking and
non-finite handling match main's) and the ring convention refused at the
ladder.

## 2026-10-29 — the cylinder-face re-homing dispatched beside PR 6011's fix pass

- **`sect-rehome`** (branch `sect/rehome-by-path`):
  `a-cylinder-face-re-homes-its-rings-on-a-chart-that-reads-no-spline-chord`
  (P1 M). Tier: **single FULL**. Dispatched now rather than after PR 6011:
  its ground is `chord_join`'s re-homing functions (`rehome_rings`,
  `chart_ring_side`, `path_ring_side`), which PR 6011 does not edit; the two
  share only the file, and the lane announces the seam. It also measures
  whether PR 6011's `RunImageNotYet` stops are this row's site.
- PR 6011's dual review: both APPROVE-WITH-FIXES, no MAJOR (row written at
  its state sync); fix pass running.
## 2026-10-29 — components are the march's (PR 6011)

PR 6011 closes `a-general-torus-pose-lists-no-components` as fork row 212
decided it. Every P0 consumer passes the germ mint: GERM's radial hole (4
and 3 loops), all six drum × donut placements, the general donut × donut
placements and the ball × donut `AroundAxis` placement stop downstream
(mostly SPIRICARM's volume door); the lily's stem at the march's
`CellBudget` (SECTRUNG); a torus just clear of a tilted cut builds every op
with checked volumes. Dual concurrent review (row
`docs/dual-review-log/rows/PR-6011.md`): no MAJOR; two fix passes and one
confirmation review. The one-way winding cross-check is a filed deviation
(`a-counted-sections-winding-is-a-claim-not-a-reading`, P2). Cost: at ε 1e-12
the new marching rows take up to ~30 min each at opt-level 2 in the nightly
(`sectrung/torus-germ-rows-march-for-an-hour-at-a-fine-band`).

## 2026-10-29 — PR 6011 merged; the frames row closed; the seam-region red re-banded

- **PR 6011 merged** (head 5c5ca02, CI green); lane reclaimed; `sect-rehome`
  told to merge main.
- **`torus-germ-pairs-have-no-section-frame` closed** by the orchestrator:
  every measured pair passes the germ mint, and each stops at another row's
  door (named in its `## Closed`).
- **`main-red-the-seam-region-door-row-runs-15x-slower-since-the-cylinder-sphere-chord`**
  (filed by nightly triage on SECT at P3, GREEN holding the row out of the
  nightly's 1e-12 row) **re-banded P1** and dispatched as `sect-seamslow`
  (branch `sect/seam-region-speed`): the slowdown is SECT's own (PR 5894),
  the guard runs blind meanwhile, and PR 6011's new marching rows put the
  nightly job's headroom in question. Tier: **single FULL**.

## 2026-10-29 — the repeated march dispatched (Ev: "the time seems pretty egregious")

Filed and dispatched `a-surface-pairs-march-is-repeated-per-operation-and-per-germ`
(P1 M) as `sect-marchonce` (branch `sect/march-once`), on Ev's go-ahead in
chat. Measure first, share within an operation, then across operations if
the answer is the pair's and band's alone; a cache that crosses the public
API stops for the designer pair. Tier: **single FULL**.
