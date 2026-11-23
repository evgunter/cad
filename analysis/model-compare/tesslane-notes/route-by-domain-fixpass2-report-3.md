Report part 3 of 3.

**What ran, and on which sha.**
- Hosted CI on 68873a44b5: green in all 13 jobs (your note).
- mesh whole, `--ignore-default-filter`: 517/517 at 1e-9, 1e-6 and 1e-12, on c541c7195a. That head differs from the fix head only by the slow-set line, and these runs finished before the restart.
- mesh whole, default filter: 516/516 on 593f852519 (H).
- geom-brep and mesh: green on 05f1926985 (A–C), apart from the known fuzz seed row `certified_torus_answers_hold`, which is main's.
- Targeted rows, each at its ε: route_rows ×7 at three ε, far_walls at three ε, D9, incidence ×8, mesh11_arc_branch, mesh11r1, mesh12, bool5, mesh11r2.
- Local gates, lint and doc-gate: last green on 650e2ce14e and before. CI's lint job is green on the fix head.
- Cut off by the restart: the geom-brep/topo/step-import whole run, gates, doc-gate and clippy on c541c7195a. They are re-running now, detached, on 68873a44b5.
- Mutants, last run on 650e2ce14e: five route mutants, each red on named rows. M7 and M9 have not yet been re-run on the fix head.

**Result to follow (detached, one chain at a time):**
1. the rest of the pre-push chain;
2. the mutants M1–M11 on 68873a44b5, including the reviewer's four (lift check dropped, `Escalated` to the chart lane, harmonic widened, K splice on a memo hit);
3. the armed mesh differential against new main;
4. the viewer slow-set rows and sweep's meshing modules;
5. the tour (`--cargo-profile ci-tour`).

**Changed PR-body paragraphs.**

(1) "The route": four facts, as sent before, with this added at the end of the paragraph:
"The domain is read in one place (`tessellate::read_domain`): by the route where the doors decided the face no rectangle (a domain the chart lane cannot read is then no lane's, and the doors' verdict is the refusal), and at the face's turn for every other trimmed face (`Route::ready`). The lane runs on `Ready`, which carries the lift by type."

(2) Door classes, replacing the table rows for "contradicts itself":
"`OffSurface` — every incidence miss, decided on the refusing path:
- a circle on a cylinder whose axis class reads zero (four points a quarter turn apart);
- a sphere circle off its own normal or radius (`props_rim_fit`);
- a coaxial torus circle off every rim;
- the torus meridian fit and plane;
- a line or ellipse on a sphere or torus, or a spiric on a cylinder, cone or sphere, more than the band from the surface over nine points of its span (`props_carrier_on_surface`).
An undecided incidence escalates (the torus rim's included). A curved door asked of a plane answers `Defect`."

(3) Branch door, new:
"`require_one_chart_branch` answers `Escalated` with its cause for an apex or pole the band cannot place when no crossing in the loop is definite, so the door's `Ok`, and `route_of`'s `Rectangle`, mean decided."

(4) "Spec deviations", new:
"The unit's spec said D9 passes unedited and a moved byte is a stop. Four D9 refusal rows moved: the hand-built `keyway`, `oblique_lens`, `pole_crossing_half_cap` and `apex_crossing_bowtie` store no chart image, so a face the doors decide no rectangle goes to the chart lane, whose domain read finds none, and each refuses `DomainUnread` (`Gap`) where main gave the doors' verdict:
- keyway `0x9d8e…` → `0xe457…`;
- lens `0xfdb2…` → `0x72fb…`;
- half cap `0xdd6f…` → `0x72fb…`;
- bow tie `0x9cdd…` → `0xbd72…`.
Ruled accepted by the orchestrator: no image is stored and no door builds such a body, and the refusal says so. The same four minted, as a finished body carries them, are D9 bodies of their own (`*_minted`): the keyway face meshes and its complement refuses `TrimLoopOrientation` (the fixture's winding); the lens refuses `OuterOpen` (main's minted answer); the half cap and bow tie digest to main's unminted constants.
The half cap's and bow tie's K streams differ from main's on these refusing bodies, because the route's domain read is recorded after the doors' verdict. The K-stream claim in this body is scoped to bodies that mesh."

(5) Cost, replacing the cost section's lines:
"- mesh suite (container, 4 vCPU): 184.4 s on main (483 tests), 193.8 s on this unit's head (489 tests).
- Tess-budget sweep (release, `--sizing-only`, main 6396c45978 against the frozen head, alternated): main 345 s and 342 s, branch 335 s and 341 s; the meter's 1678 rows byte-identical.
- Memo hit: the route, doors included, runs before the memo is consulted, where the kind route skipped the doors with the lane. Warm hits, medians, main → head: washer 151–161 → 230–267 µs, rounded prism 186–194 → 282–376, cone 89–92 → 123–152, donut 317–533 → 419–510, ball 147–185 → 184–243. Filed P3 with two cures (`work/tesslane/a-memo-hit-pays-for-the-route-the-lane-used-to-skip.md`)."

(6) Mutants table, rows added: "fact 4 off: 8 red"; "split read before the doors: `route_rows::a_split_side_is_not_read_before_the_doors_answer`"; "harmonic widened: `route_rows::an_obliquely_placed_rectangle_meshes_on_the_rectangle_lane_to_mains_bytes`". M1–M11 counts on the fix head to follow.

(7) Filed, added: the memo-hit row; the three-readings row; the conearm branch-door row closed by this pass.
