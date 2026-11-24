# Delta read: PR 7628 (`oneround/sizes-from-geometry`), the fix pass R1 to R11

You reviewed this unit on `fc2bf2d7cf` (APPROVE-WITH-FIXES, one MAJOR, six MINOR; your report at `3874d0b7ec` on `oneround/sizes-from-geometry-review`). Its fix pass is in. Read it on the FROZEN head `7bd212f895cf1f22998e4471498e58eafdd5049d` (whatever is pushed later); it holds `origin/main` merged twice, and the PR's body has one section per R item. A short read: the fixes, not the unit again. The same rules hold: your branch `oneround/sizes-from-geometry-review`, pushed as you go; never push to the unit's branch; your own `CARGO_TARGET_DIR` outside the checkout; no waiters or background chains, a long job `setsid`-detached and polled in the foreground; no trailer in your commits and no model named anywhere; do not look at any other session; no helper subagent on a model other than your own, and any helper named in your report. No hosted run has concluded on the frozen head as far as I know; your own runs are the evidence.

## What you are asked

Section A is how your findings were ruled. Section B is the implementer's report of the pass. Both are claims. For each of R1 to R11: is it carried out on the tree as section B says; did the fix mint a fresh instance of what it closes (`docs/prompts/reviewer-style-lane.md`, the stance's last bullet); does anything you raised remain open. Report a ruling you think wrong as a finding.

Falsify by execution, these above all:

D1. **Re-plant your six surviving mutants** (the patch in your report) on the new rows; each is now red, or still survives and section B says so with a reason you judge. Two survive by the lane's own account: the plane mate's call-site multiplier (reach × 0.5 passes 3,187 of 3,187 editor-core `ci` tests; "I did not build a mate posed at the box's edge") and the apex ball halved in `RuledFaceRegion` (431 sweep rows; "only a near-generator ray would show it, and no row builds one"). For each: build the pose (a mate at the box's edge; a ray near a generator) and show the mutant red, or say why the read is unobservable and whether a row or a type should hold that. A converted read no test can tell from its halved self is the finding this unit was reviewed for.
D2. **R2's reach claim.** "No door builds an unreadable wall or cone face: a spline ruling certifies as `Unimplemented` on the chart and `DescriptionNotAdjacent` as a plane intersection", so the new `partial_rows` build the record "with the same functions on a readable face". Is `FaceGeo::PartialWall` / `PartialCone` then reachable through any public door at all (STEP import, a sweep along a spline, a healed face, a face whose cover is unknown at one ε only)? If none: the arms are dead code the unit converted, and say so; if one: a row through it. The cone rows are f64 only "because the cone fixture is f64's": is that a reason or a gap.
D3. **R3's poison.** "A poison clear now stays poison in the no-wedge arm too: the fold folds the clears into zero." Read the fold: does a poison clear reach its reader as poison (NaN, a refusal, a typed absence), or as 0, which is what the finding was. Plant a poison clear and show what `Link::of` holds.
D4. **R4's door.** `geom_core::tolerance::session_box_reach_from` (√3·E + d) "in the box's home", "same bits at `arm_of`". Is √3·E + d a bound from above on the far distance of the box from a point at distance d from the origin, on every lane (rounding outward); is it a second spelling of something the box's home already has (grep for the constant and for √3); who else should read it and does not.
D5. **R5(b)'s new refusal.** "A non-finite net now refuses as `SectionVerdict::Unsupported` naming the net; `net_reach_from` returns `None`." Is `Unsupported` the family a non-finite net belongs to (a defect in the caller's data, a limit, or an unsupported case: read what `Unsupported` promises its readers and which recourse they offer); does any body that built before now refuse.
D6. **R7.** `RuledFaceRegion` holds an apex ball "built from its boundary covers' hull"; `RuledFaceRegion::reach()` is gone; "3,475 runs move by ≤ 6.3e-16 of the window's width, both directions". A run that moves inward by a rounding is a bound that got smaller: is each side still a bound (from above where it must hold the face), and is "both directions" a finding. The sweep for the class your review named (a bare scalar called reach, arm or extent, documented as a lever): the lane lists `pieced_arc`, `span.rs`, `chart_singularity`, `cell_region`, `pair_section::gap`, two census sites as "bare-T geometry reads, none a lever". Sample three and say.
D7. **R8 and R9.** `run_on` is gone and `ExtentBall::range_along` is "the one home" with `Reach::range_along`'s ball arm calling it: one statement or two; `min_radius_toward` reading "the inner radius of every kind through `principal_radius`": did any verdict's lever get longer (a bound from below that rose) and is that right. `link`'s cleared fan and `census`'s edge fan now read `face_ball_about(..).radius()`, each with a row red at × 2: is × 2 the unsound direction for each, and is there a row for the other direction where it matters.
D8. **The body and the rows** (R6, R9, R10): the two filed rows (`work/ssiedge/chart-lane-domains-carry-a-half-extent-only-checked-and-keyed.md`, `work/ssiarith/a-certified-tube-and-the-idealized-step-are-sized-by-a-levers-length.md`) say what you measured; the body's "what now flows where" statement is true of the records as they now are (the condition on a records change is that an odd flow is said, not found); totals (100 calls at the merge base, 90 at the head) by your own count.
D9. **What ran.** By the lane, on the frozen head: fmt; both clippy lines with `--force-warn clippy::disallowed_methods` and the conic-centre census; `GATE_BASE=origin/main scripts/run-gates.sh` (68 OK); `payload-rung-sweep --check`; `work.py lint`; the doc gate; the `ci` profile over geom-core, geom-brep, topo, editor-core and sweep at the default ε, 1e-6 (14,141 of 14,141 each) and 1e-12 (one red, sweep's `a_cylinder_wall_through_a_prism_corner…`, filed as main's). NOT run: the recourse rosters of the crates it touches (R5(b) adds a refusal; does it add a decision name or a funnel), the slow set, `cargo test -p mesh --features budget`, step-import, the demos ("the only public signature changes are additions": check `demos/tour` and `demos/wild` still build if `RuledFaceRegion::reach()` or `run_on` was public). Run the doc gate, the rosters and mesh yourself, and whichever of the others your reading says the diff can move.

Two deviations the lane disclosed, for the record and for you to check the tree against: a container restart left one of its mutants applied (P10c), "caught and reverted before any commit, never pushed" (confirm no mutant is in the frozen head: diff each mutant site against its intent); several of its cargo calls built into the in-checkout `target/`, deleted, nothing committed.

Verdict on the ladder APPROVE · APPROVE-WITH-FIXES · NOT-MERGEABLE-AS-IS · REJECT; findings MAJOR / MINOR / NOTE, each DEMONSTRATED or BY-INSPECTION with `sure` / `likely` / `unsure`; a short `## Style` section naming the questions exercised; the three ratings 1 to 5 with a line of evidence each, for the unit as it now stands; what you EXECUTED and what you ONLY READ, and for every count what it is judged against. Commit the report as `analysis/oneround/sizes-from-geometry/delta-report.md` on your branch, push, and send the orchestrator its text (the verdict, every MAJOR and MINOR in full, the rest in brief, with the commit sha). Then stop.

---

# Section A: the rulings on your review

- **R1 (your MAJOR, taken):** the head failed the rustdoc gate the lint job runs (a doc link to a renamed function). Fix it and run the lint job's steps one by one.
- **R2 (taken):** `FaceGeo::PartialWall` is built by no test on any lane, so three of the five converted reads in `solid_contain` are unpinned and a wall ball that does not hold the face survives, where the body said "reached at every ε". Rows through a door, or the site named unreachable with the reason.
- **R3 (taken):** `Link::of`'s radius is unpinned from above, and its no-wedge arm turns a poison clear into 0. A row; poison propagates, or the site says why none exists.
- **R4 (taken):** the plane mate's site replaced a lever's length with a bare constant that becomes a lever again, where it means a lever from the session box read about the frame's origin. Through the box's door, with a row red on a halved extent.
- **R5 (taken as a row, not code):** the slabs' half-extent on a chart lane is computed for a number nothing there reads (`check()`, the pass key), so the conversion is right and unobservable. The comments say so; filed on the slate that owns `ssi.rs` with your measurement and the surviving mutants; `SsiDomain` not re-typed here.
- **R6 (taken as a row):** three hits the sweep called a margin's business size the certified tube's radius ladder and the march's step: sound, on no row. Filed, tied to the later unit on the same sites.
- **R7 (taken):** in the commit that lifts the run formula into `ExtentBall::run_on` the unit inlines a copy over a bare `T` reach read as a ball's radius about the apex. Convert it; sweep for the shape (a bare scalar named reach, arm or extent, documented as a lever), which a sweep keyed on `Lever::length` cannot see; add the blind spot to the gate row.
- **R8 (taken):** your style findings on the new doors (one home for the run formula; `net_reach_from`; `min_radius_toward`'s kinds; `curvature_arm`'s module doc).
- **R9 (taken):** four of its five rows carried no band or cost; two sites on the sector-bounds row (`link`'s ruled fan, `census`) are the shape of the unit's own site 2 and convert now, each with a row.
- **R10 (taken):** your notes N1 to N6 (totals, the 1e-12 re-take, "five of ten", the cone ray row, the "what now flows where" statement, the row file's corrections).
- **R11:** merge `origin/main` (the conic-centre census has landed there) and run the lint job's steps one by one on the merged tree.

---

# Section B: the implementer's report of the pass

sizes-from-geometry fix pass: done. PR #7628 head is 7bd212f895cf1f22998e4471498e58eafdd5049d. The PR body has one section per R item.

R11 – merged origin/main twice: once before the fixes, and again before the final checks. No file of mine moves a conic-centre-reads count; the census step passes.
R1 – the PartialCone doc now links partial_ball. doc-gate --workspace-pass --skip-viewer-toolkit exits 0.
R2 – face_geo's None arms now build through partial_wall/partial_cone. No door builds an unreadable wall or cone face: a spline ruling certifies as Unimplemented on the chart and DescriptionNotAdjacent as a plane intersection. So the new partial_rows build the record with the same functions on a readable face.
  - Wall face_holds and wall ray rows: f64 and Interval.
  - Cone face_holds and ray row (N4): f64 only, because the cone fixture is f64's.
  - Mutants all red: ball halved, ball moved off its pivot, cone ray stand-off forced Negative, wall ray stand-off forced Negative.
  - Reach account corrected in the body.
R3 – new row the_links_ball_reaches_its_nearest_wedge (ball ×2: red). A poison clear now stays poison in the no-wedge arm too: the fold folds the clears into zero.
R4 – new door geom_core::tolerance::session_box_reach_from (√3·E + d), in the box's home. Its row is red with the extent halved. Same bits at arm_of. The call-site multiplier still SURVIVES: reach ×0.5 passes 3 187/3 187 editor-core ci tests. I did not build a mate posed at the box's edge.
R5 –
  - (a) The comments at both slab sites now say the chart lane only checks and keys the slab.
  - (b) A non-finite net now refuses as SectionVerdict::Unsupported naming the net; net_reach_from returns None.
  - (c) Filed work/ssiedge/chart-lane-domains-carry-a-half-extent-only-checked-and-keyed.md (P4, E) with your measurement and the three survivors. SsiDomain is untouched.
R6 – edge_nurbs 1200/1865/2152 and offset_derive's two SsiDomain::extent sites are moved from M to L5. Filed work/ssiarith/a-certified-tube-and-the-idealized-step-are-sized-by-a-levers-length.md (P1, M); it names reach-from-the-read-point's row (diameter-levers-of-a-datum-read-at-a-point).
R7 – the inlined run is converted, and the comment that did not parse is gone with it.
  - RuledFaceRegion now holds an apex ball built from its boundary covers' hull. The trimmable cone arm reads its run through that ball. RuledFaceRegion::reach() is gone.
  - Measured: 3 475 runs move by ≤6.3e-16 of the window's width, both directions. All tests pass.
  - The apex ball halved SURVIVES 431 sweep rows plus a row I tried and dropped. line_cone_roots uses the run only as its lead rung's lever and still places roots outside it; only a near-generator ray would show it, and no row builds one.
  - pieced_arc, span.rs:943 and chart_singularity:800, plus my grep's cell_region, pair_section::gap, census 4402 and census::point: all bare-T geometry reads, none a lever.
  - The blind spot is added to the gate row.
R8 – run_on is gone: ExtentBall::range_along is the one home and Reach::range_along's ball arm calls it. net_reach_from = of_cover_from(net, at).radius(). min_radius_toward now reads the inner radius of every kind through principal_radius (parallel_radius for a cone, uniform_radius otherwise), and curvature_arm's module doc now says it also reads radii as lengths.
R9 – four rows banded: pierce P1 M, ray-crossings P1 M, apex P1 M, gate P3 M. link's cleared fan and census's edge fan now read face_ball_about(..).radius(); each has a row, red at ×2. Both are struck from the sector-bounds row, and cover-kind-match.readers is updated. Nothing else on that row was touched.
R10 –
  - N1: totals fixed (100 calls at the merge base, 90 at the head).
  - N2: 1e-12 re-taken on the final merge.
  - N3: five of ten.
  - N4: the cone ray row, under R2.
  - N5: the "what now flows where" statement is in the body.
  - N6: the corrections are in the row file.

Ran on the final merged tree (7bd212f8; the last commit lowered two gate pins and touched no Rust):
- fmt + benches fmt: clean.
- Both clippy lines with --force-warn disallowed_methods, plus the conic-centre census: clean.
- GATE_BASE=origin/main run-gates.sh: exit 0, 68 OK. The first run caught cover-kind-match pins this pass cleared and lever-fixtures +2; both moved.
- payload-rung-sweep --check: ok. work.py lint: ok. doc-gate: exit 0.
- No Python touched, so ruff not run. demos not run: the only public signature changes are additions.
- nextest ci over geom-core, geom-brep, topo, editor-core and sweep:
  - default: 14 141/14 141.
  - 1e-6: 14 141/14 141.
  - 1e-12: 14 140/14 141. The one red is sweep a_cylinder_wall_through_a_prism_corner, main's, at margin 2.813857544654365e-12. The GREEN ruling row passes here.

Deviations:
- During this pass a container restart left one mutant applied (P10c). It was caught and reverted before any commit, and it was never pushed.
- Several of my earlier cargo calls built into the in-checkout target/ (the env var did not persist between calls). I deleted that cache to free disk. It is git-ignored, and nothing from it was committed.
