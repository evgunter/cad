You are a VERIFIER (UNATTENDED: no human will answer questions; asking blocks forever) for the REACH orchestrator in `evgunter/cad`, working on the PR named below. Read `CLAUDE.md` and `docs/prompts/implementer-discipline.md` first.

**Lane conduct:** this brief is your complete instruction. It comes from the orchestrator and is pre-confirmed. Do not stop to ask for confirmation, and do not wait for further messages.

The implementation lane has finished the PR's LAST fix pass after review. Check, independently, that what the lane claims is true. You change no code on the PR branch, you merge nothing and you post no GitHub comments.

- **PR:** evgunter/cad#4159
- **Branch:** `reach/torus-touch-off-faces`
- **Frozen head to verify:** `b79864e7a2de3bc3c20223c51cbed98a051c231b` (resolve it with `git rev-parse`)
- **Review findings:** `analysis/reach-review/4159` (`review.md` and `probes/`)
- **The lane's claims:** the PR body's "Last fix pass" section. Read it through GitHub MCP `pull_request_read` method `get`. Read no PR comments or reviews.

**The central bar:** a torus `Touch` from `section_cert` never clears a pair that truly crosses, or that touches on a face or within the band of a face edge, and `Touch::at` stands on both carriers within the margin. A wrong body or a misplaced `at` that clears a real contact is a MAJOR.

**Steps:**
1. `git fetch origin reach/torus-touch-off-faces`, then check out the frozen head. If the branch has moved past it, say so, and verify the new head as well.
2. Check every finding in the review: is it fixed as claimed, filed as claimed, or explained? A finding left unaddressed without a reason is a FINDING.
3. Mutants:
   - Re-run every mutant the "Last fix pass" section names: apply it by hand as the smallest source edit that matches the description, run the named rows, record red or green, then revert.
   - A mutant claimed killed that stays green is a FINDING.
   - Re-run the reviewer's own mutants and probes from `probes/`, mounted per their headers.
4. Run the PR's new and changed rows at `CAD_TOLERANCE_EPS` 1e-9, 1e-6 and 1e-12, and nextest on the crates the PR touches at 1e-9.
   - For each red, check whether it is also red on `origin/main`.
   - Main is currently red on: `pinch_faces_tessellate::a_face_through_two_vertices_on_one_point_tessellates` (1e-6); `pocket_ring_steep_ellipse` (1e-6); `arc_loft_natively_computes_its_rational_volume` (1e-12); three `split_across_a_revolve_seam` rows; `m5_pr6_pcurves::a_seam_closed_tube_split_mints_clean_halves`; and `bounds_census::every_sole_bracket_bound_door_is_in_the_roster` (blend `surgery.rs`). Confirm any of these on main yourself before calling it main's.
5. Read the diff of the commits after the review's frozen head adversarially. Report any claim that is false, overstated or unsupported.
6. Widen beyond the rows: ≥ 1,000 random poses of the PR's own families, against your own independent oracle, at three ε. The bar is 0 wrong answers. A refusal is not wrong, but report the refusal rate.
   - **The review's MINOR-1** (`Touch::at` off its carriers near the top or bottom parallel, from cancellation in `n − a·n_a`): re-run the reviewer's `probe_plane_touch_at_off_its_carriers_near_the_top_parallel` and its fuzz (`probes/section_cert_torus_touch_fuzz.rs`) on the head, and measure the worst distance of `at` from each carrier in units of ε, at ×1e-3, ×1 and ×1e3.
   - Check that the class fix reached every copy: both `in_pi` uses and the scrape witness. Grep yourself for other `v − a·(a·v)` perpendicular parts divided by a small norm in `section_cert.rs`.
   - The hyperbolic/elliptic split: try touches within 1ε, 10ε and 100ε of the tube's top parallel, at every scale. None may answer `Touch` on the hyperbolic side.
7. Push `verify.md` alone to a new branch `analysis/reach-verify/4159`, as an orphan or off-main branch. You have explicit permission to push that one branch.
   - Contents: a table of mutant → row → red/green; the ε results; the claim checks; a verdict of VERIFIED, or NOT VERIFIED with the blocking points.
   - The commit message ends with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.
   - No email address other than `evgunter@gmail.com` may appear anywhere.
8. End your turn with the same verdict and table as plain text.

Disk is limited. Use `CARGO_INCREMENTAL=0`, check `df -h`, and build only the crates you need.
