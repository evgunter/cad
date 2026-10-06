You are an IMPLEMENTER for the REACH program in `evgunter/cad`, a greenfield B-rep CAD kernel in Rust, finishing an existing PR. The REACH orchestrator dispatched you and reviews and merges your work.

**PR:** evgunter/cad#4122, branch `reach/operand-gate-separating-direction` (head `2bbba71676`). The operand gate and the narrow phase read a separating direction per face pair (`topo::boolean::separating`, `apart` / `apart_along`, `operand_axes`, `circle_box`). Read:
- the PR body (GitHub MCP `pull_request_read` method `get`);
- its unit item under `work/reach/` (the PR body names it);
- the full review on `analysis/reach-review/4122` (`review.md` and `probes/`).

This is the PR's LAST fix pass. The review raised no MAJOR. You have explicit permission to commit and push to `reach/operand-gate-separating-direction`, with merge commits only (never rebase or force-push). Do not merge the PR, and post no GitHub comments.

**Step 1:** merge `origin/main` first, and resolve any conflict keeping both sides' behaviour.

**Required fixes:**
1. **MINOR-1: the rows cannot catch a reach that under-covers.** Add rows at the exact support: plates set on the support plane of the frustum and torus solids (full and 270°) along oblique directions, at δ ∈ {0, −ε} and a small positive δ.
   - Every op must be right; check the result membership with `point_in_solid` against analytic membership, as the reviewer's probe (b) does in `probes/review4122_probes.rs`.
   - The reviewer's mutant M3 shrinks every reach in `apart_along` (`separating.rs:142`) by 0.1% of its width. Your rows must turn red under M3. Promote the probe; don't re-invent it.
   - Exclude the pre-existing 270° torus cut-cap rim case (NOTE-2) and file it (below).
2. **NOTE-4: the Approx arm asks about the sphere *face* where its question is the *ball*** (`ops.rs:3353–3357`). It is sound today only because a sphere face's reach is `WholeBall` (`boxes.rs:1431`).
   - PR 4123, in flight, tightens sphere face reaches to a latitude/azimuth rectangle. Once it lands, this arm would be asking about a smaller set than the ball its escape argument needs.
   - Make the arm ask about the whole ball explicitly, independent of the face's reach, and add a row that would go red if a sphere face's reach were tighter than its ball.
3. **NOTE-3: doc overclaim** at `reduce.rs:271`, "one the gate refuses the sweep would examine". The gate reads face × face; the sweep reads edge × face and skips an edge whose own reach is apart. Re-word it to state what actually holds.
4. **NOTE-5: the test oracle cancels near an axis** (`boxes.rs:3223`, `rho*(1 - ni*ni).sqrt()`). Use a cancellation-free form, e.g. the norm of the cross components, and add the n ≈ ẑ case to that row.
5. **MINOR-2 / S8: `operand_axes` is recomputed in hot loops** (`ops.rs:3174`, `ops.rs:3357`; eagerly in every `sweep_direction`, `reduce.rs:1084`). Compute it once per operation (or per operand pair) and pass it down. Deduplicate ± normals.
6. **S1:** spell the circle once at `ops.rs:3162–3170` (derive `circle_box`'s arguments from the `Item::Circle`, or give `circle_box` a circle).
7. **S6:** rename `germ_interior_oval.rs:861` so the name says what it now guards: that the certificate is *not* asked.
8. **S5:** `n3r1_prune.rs:136` is now an empty exemption list behind live filter code. Remove the dead filter, or say in one line why it stays. Note this on its hone item.

**File, don't fix** (`python3 scripts/work.py new …`, each with file:line citations; add evidence to an existing item instead of filing where one fits):
- (a) NOTE-2: a plate touching the 270° torus at its cut-cap rim (δ = 0) builds an `Assembly`, and ∩ comes back `Empty`. Main does the same. Say whether the contact is recorded. Do not touch the contact machinery: that is under the D10 hold.
- (b) NOTE-1: add the reviewer's refusal measurements as evidence on the filed seam-anchor residue item: the bar 2%·s off a cone wall, the 270° cone at the cut, and 30 of 120 frustum 270° plates at δ = 1e-3·s.
- (c) S3: a row asserting gate/sweep agreement, unless you add one here.

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
