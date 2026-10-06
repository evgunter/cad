You are an IMPLEMENTER for the REACH program in `evgunter/cad`, a greenfield B-rep CAD kernel in Rust, finishing an existing PR. The REACH orchestrator dispatched you and reviews and merges your work.

**PR:** evgunter/cad#4123, branch `reach/split-gate-sphere-azimuth` (head `0c91849a30`). A sphere face's box and reach become its latitude/azimuth rectangle (`boxes.rs` `sphere_rect`, `FaceBoxRule::SphereRect`, `census::sphere_reach`), falling back to the zone or the ball. Read:
- the PR body (GitHub MCP `pull_request_read` method `get`);
- its unit item under `work/reach/` (the PR body names it);
- the full review on `analysis/reach-review/4123` (`review.md` and `probes/`).

This is the PR's LAST fix pass. The review raised no MAJOR. You have explicit permission to commit and push to `reach/split-gate-sphere-azimuth`, with merge commits only (never rebase or force-push). Do not merge the PR, and post no GitHub comments.

**Step 1:** merge `origin/main` first, and resolve any conflict keeping both sides' behaviour.

**Context:** PR 4122, in flight, changes its Approx arm to ask about the whole ball rather than the sphere face's reach. That is the right question once your rectangle lands. Do not depend on 4122's order. Any other caller that reads a sphere face's reach as the ball must be found and fixed here, not in 4122: grep the callers of `face_reach`, `face_reach_in`, `sphere_reach` and `face_box` for sphere faces, and list what you checked in the PR body.

**Required fixes:**
1. **MINOR 1: the latitude window is tighter than a wrap-alone face carrying a strut.** See `solid_contain.rs:3259-3265`, read by `sphere_rect` (`boxes.rs:1542`).
   - The case: a north cap whose rim is at z = −0.5, with a dangling meridian strut to z = 0.3. `wrap_rims` skips the self-mated strut, so the period gate is skipped, and the box comes out z ∈ [−0.5, 0.3] for a face that reaches z = 1.
   - At-rest validation refuses that body, but `Separation::of(&Body)` (`separation.rs:157,164`, public and unvalidated) grants on box non-overlap with it.
   - Make the rectangle rule fall back to the zone or the ball whenever the loop's levels do not bound the face, so that a self-mated edge never stands in for a rim. Fix the comment's claim about what the period gate guards.
   - Promote the reviewer's `probe_strut_cap` to a row. It must be red without the fix.
2. **MINOR 2:** `work/topo/torn-body-refusal-families-beyond-the-six-doors.md:180-194` points at `classify.rs` `sphere_zone_reach`. Re-point it to `boxes.rs` `sphere_rect` and keep its obligation true.
3. **MINOR 3:** `separation.rs:45` "a whole ball for a sphere band" is stale; a full-turn band is now its zone. Grep the rest of the tree for prose that still says a sphere face's reach is the whole ball, and fix each one.
4. **NOTE 3: outward rounding.** At `T = f64`, `face_box`'s `lo()/hi()` are identities, so `sphere_reach`'s atan2, cos and sqrt are not outward-rounded (`boxes.rs:1829`). Either charge a few ulps outward, or state in the code the bound that every caller's pad covers, and cite the pads.
5. **Style.**
   - Give the seam's unit-length decision its own name, distinct from `bool_box_sphere_axis` (`boxes.rs:1599`). Register it in `decision_words` and the audit.
   - Make `sphere_rect` return an enum of its three outcomes (rectangle, zone, ball) instead of a nested `Option`, and rename `FaceBoxRule::SphereRect` if it then reads wrong. Skip this only if it touches more than `boxes.rs` and `census`; if you skip it, say so in the PR body.

**File, don't fix** (`python3 scripts/work.py new …`, each with file:line citations; add evidence to an existing item instead of filing where one fits):
- (a) NOTE 1: reflex (L/U) loops and faces 1e-4 off a pole fall back to the ball or zone. Add this to the remainder of `work/boxes/sphere-operand-box-is-the-whole-ball.md`.
- (b) NOTE 2: add the second `SliverSector` witness to the filed `split-bisector-side-in-band…` item: two-rim zone Θ = 1.5; s = 1e-3 at ε 1e-9 and s = 1 at ε 1e-6, both poses; n = (−0.320, 0.0435, 0.946), d = −0.2407; margin 5.44e-9 / 5.44e-6; vertex 8v1.
- (c) NOTE 4: `sphere_rect` re-walks the loop, the material sign and the chart trim on every `face_reach_in` call, with no memo. File this as a perf item.
- (d) Q3: boundary-crossing cuts cannot see an over-tight face box. Only crest cuts and the direct box rows guard it. Add one crest-cut row here if it is cheap; otherwise file it.

**The D10 hold (Ev).** Do not extend any mechanism `docs/DESIGN.md` D10 retires: declared contacts or pairs, the placement registry, intent spellings.

**Lane conduct.** This brief is your complete instruction. It comes from the orchestrator and is pre-confirmed. Do not stop to ask for confirmation, and do not wait for further messages.

**Before pushing:**
- with `CARGO_INCREMENTAL=0` (check `df -h` first): `cargo build --workspace --all-targets`; `cargo clippy --workspace --all-targets -- -D warnings`, also with `--all-features`; `cargo fmt --check`;
- every `scripts/gates/*.sh`, and `python3 scripts/work.py lint`;
- nextest `-p topo -p sweep` at ε 1e-9, 1e-6 and 1e-12 (set with `CAD_TOLERANCE_EPS`).
- Known reds on main, both at 1e-6: `pocket_ring_steep_ellipse`, and `pinch_faces_tessellate::a_face_through_two_vertices_on_one_point_tessellates` (JOIN P0 `work/join/pinch-tessellate-row-escalates-at-eps-1e-6.md`). Do not fix either here.

Show each new row red without its fix (name the mutant).

**Then:**
- update the PR body with a "Last fix pass" section listing each finding → change → pinning row, plus the mutant results;
- end your turn by stating the head sha and the test counts.

Commit messages end with
```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <your own session link>
```
No email address other than `evgunter@gmail.com` may appear anywhere you write.
