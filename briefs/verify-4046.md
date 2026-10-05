You are a VERIFIER for the REACH orchestrator in evgunter/cad, working on the PR named below. Read CLAUDE.md and docs/prompts/implementer-discipline.md first.

The implementation lane has finished its LAST fix pass after review. Your job is to check, independently, that what the lane claims is true. You change no code on the PR branch, you merge nothing and you post no GitHub comments.

Steps:
1. `git fetch origin <branch>`, then check out the frozen head given below. If the branch head has moved past it, say so and verify the new head as well.
2. For each mutant listed, apply it by hand, run the named rows, and record red or green. Then revert it.
   - A mutant the lane says is killed but which stays green is a FINDING.
   - Write each mutant as the smallest source edit that matches the description.
3. Run the PR's new or changed test rows at CAD_EPS (or the repo's ε switch, see `scripts/` and the nightly workflow) 1e-9, 1e-6 and 1e-12.
   - Run the changed crates' nextest suites at 1e-9.
   - For each red, check whether it is also red on `origin/main`. If it is, it is not this PR's.
4. Read the diff of the last fix pass (the commits after the review's frozen head) adversarially against the review findings listed. Check each claim below. Report any claim that is false, overstated or unsupported.
5. Push a short evidence file, `verify.md`, to branch `analysis/reach-verify/4046`. Make it an orphan or off-main branch, with only that file.
   - Contents: a table of mutant → row → red/green; the ε results; the claim checks; a verdict of VERIFIED, or NOT VERIFIED with the blocking points.
   - Commits end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.
6. End your turn with the same verdict and table as plain text.

Disk is limited. Build only the crates you need, and run `cargo clean -p` between ε runs only if space runs short.

PR: evgunter/cad#4046
Branch: reach/carved-sphere-classify
Frozen head to verify: 66818a125e3 (resolve the full sha with `git rev-parse`; origin/main merged at 515e79ebfd)
Reviews to read for the findings: the dual review on analysis/reach-dual/4046-r1 and -r2, frozen at b793623189 (review.md each; their probes/ dirs include reviewer_4046_*_probe.rs files — mount them per their headers and re-run them).

This PR makes trimmed sphere faces readable for point classification and the pierce check through one module, `topo::boolean::sphere_region` (a geodesic closest-crossing rule). The central bar: no wrong In/Out from `point_in_solid` or the face door, and no wrong body through any boolean, at any ε.

Lane claims (last fix pass):
- **Seam row.** A new row pins the seam exclusion at default ε: deleting `arcs.retain(|a| !seams.contains(&a.edge))` in `sphere_region.rs` turns it red at 1e-9.
- **Antipodal vertex fixed.** `SphereFaceRegion::contains` now answers when a face vertex sits at the point's antipode (reverse rays). The reviewer's setup — union of the unit ball at (2,2,0.5) and the r 0.5 ball along (0.3,0.9,0.3), point 1e-9 from A's +y pole — answers In at 1e-9 and ×1e3.
- **Strut row reads a region.** `join1_r1_rows::a_pole_struts_halves_face_their_own_meridians` now crosses/nests a carved sphere face; red under the heading-flip and parity mutants.
- **Tilted-cut row.** `carved_sphere_operand::a_tilted_cut_of_a_ball_is_an_operand` goes red under the leave/enter swap.
- Mutant table in the PR body ("noseam reds the seam row; flip 14/19; parity 15/19"): reproduce each.
- An Interval-scalar region row exists.
- `sphere_chart_trim`'s azimuth gate is kept, and the code says why (rim/meridian loop stepping around a pole). Check the claim by constructing such a loop if you can.
- Docs fixed (`KindUnsupported`, `PartialSphereFace`), orphaned decision words removed, the sphere residual has its own decision name, no `unreachable!` left at the root-door call.

Also: widen beyond the rows. Re-run both reviewers' probe sets on the head, plus your own random points (≥ 2,000 per body, three ε) on at least three carved bodies against distance-function oracles: 0 wrong answers is the bar. Suites: topo and sweep at three ε (`pocket_ring_steep_ellipse` at 1e-6 is main's).
