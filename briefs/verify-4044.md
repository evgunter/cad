You are a VERIFIER for the REACH orchestrator in evgunter/cad, working on the PR named below. Read CLAUDE.md and docs/prompts/implementer-discipline.md first.

Lane conduct: this brief is your whole instruction. It comes from the orchestrator and is pre-confirmed. Do not stop to ask for confirmation, and do not wait for further messages.

The implementation lane has finished its LAST fix pass after review. Your job is to check, independently, that what the lane claims is true. You change no code on the PR branch, you merge nothing and you post no GitHub comments.

Steps:
1. `git fetch origin <branch>`, then check out the frozen head given below. If the branch head has moved past it, say so and verify the new head as well.
2. For each mutant listed, apply it by hand, run the named rows, and record red or green. Then revert it.
   - A mutant the lane says is killed but which stays green is a FINDING.
   - Write each mutant as the smallest source edit that matches the description.
3. Run the PR's new or changed test rows at CAD_TOLERANCE_EPS (or the repo's ε switch, see `scripts/` and the nightly workflow) 1e-9, 1e-6 and 1e-12.
   - Run nextest `-p topo -p sweep` at 1e-9.
   - For each red, check whether it is also red on `origin/main`. If it is, it is not this PR's.
   - Known main reds: `pocket_ring_steep_ellipse` and `pinch_faces_tessellate::a_face_through_two_vertices_on_one_point_tessellates`, both at 1e-6.
4. Read the diff of the last fix pass adversarially against the review findings, i.e. the commits after the review's frozen head 4bd8f5a8b9. Check each claim below. Report any claim that is false, overstated or unsupported.
5. Push a short evidence file, `verify.md`, to branch `analysis/reach-verify/4044`. Make it an orphan or off-main branch, with only that file.
   - Contents: a table of mutant → row → red/green; the ε results; the claim checks; a verdict of VERIFIED, or NOT VERIFIED with the blocking points.
   - Commits end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.
6. End your turn with the same verdict and table as plain text.

Disk is limited. Build only the crates you need, and run `cargo clean -p` between ε runs only if space runs short.

PR: evgunter/cad#4044
Branch: reach/trimmed-sphere-escape
Frozen head to verify: c6ab123203 (resolve the full sha with `git rev-parse`; it merges origin/main with PRs 4046 and 4042)
Reviews to read for the findings: the dual review on analysis/reach-dual/4044-r1 and -r2, frozen at 4bd8f5a8b9. Read review.md on each. If their probes/ directories hold probe files, mount them per their headers and re-run them on the head.

What the PR does: a section circle lying wholly inside a TRIMMED sphere face and a plane face is now cut by `apply_cut_ins`. The cut runs along the meridian of the face's own sphere chart through the circle, and the pipeline re-enters.

The central bar: no wrong body through any boolean, at any ε. A cut that is not on the sphere, or that splits the wrong piece, is a MAJOR. So is a body whose volume misses its closed form.

Lane claims (last fix pass):
- **Pole-to-pole span (F2).** The cut takes the absolute value of the cross term, so the span lies in (0, π]. Row: the θ=4 revolve wedge at lat 0 / az −114.6° and lat −20 / az −160.4°, centred. Mutant: drop the `abs()`. The row goes red with `Euler(IntervalNotForward, −π)`.
- **Stale face across several cut-ins (F1).**
  - `apply_cut_ins` tracks the pieces each cut leaves. A later cut runs on the piece holding both of its circle's crossings, found with `sphere_region`.
  - When both crossings lie on an earlier cut's meridian, the circle takes no cut of its own.
  - Rows: lens ∖ bricks at az 130°/50° and 50°/130°; banded ∩ cube at 45° and 135°; a 90°/90° pair of slabs.
  - Mutants: read only the scanned face key → red. Refuse instead of skipping → red.
  - Check that the skip is sound: does the crossing layer really meet that circle on the earlier cut?
- **Nearest hit (F3).** Row: the ball less the box `x ≥ 0.5`, cut toward lat ±60° az 30°, where the meridian meets the boundary three times on one side. Mutants: keep the farther hit below, above, or both. Each turns this row and the strut row red.
- **R-loop gate (F4).**
  - `cut_holder` is pinned by a unit test over every verdict. Mutant: any verdict cuts → red.
  - The lane found no buildable pose with a non-R-loop verdict once 4046 is in. Try to construct one; if you do, check that it refuses and does not build wrong.
- **Strut flip.** `a_slab_cutting_a_cap_off_a_pole_strut_carve_builds` builds to the closed forms in both directions: ∪ 36.30261762663585, ∩ 8.79155069153e-5, strut ∖ slab 0.30261762663585, slab ∖ strut 35.99991208449308.
- **Docs.** The `CUT_LEVER` text covers turning the plane and stays inside the viewer's 75-word budget. The PR body's "same topology" claim is limited to the operand. Three items filed in `work/reach/`.
- **Merge.** The two `ops.rs` tests from main were adapted to `Recuts`. Check that they still assert what main's versions asserted.

Also widen beyond the rows. Build ≥ 30 random poses of each family:
- a plane or slab cutting a cap inside a trimmed sphere face (lens, banded ball, wedge, pole strut);
- several such cuts on one face;
- for each op, check the body at tiers 1–3 and its volume against the closed form (cap volumes from the radii).
- The bar is 0 wrong bodies. A refusal is not wrong, but report the refusal rate.
