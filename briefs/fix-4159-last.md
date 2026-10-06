You are an IMPLEMENTER for the REACH program in `evgunter/cad`, a greenfield B-rep CAD kernel in Rust, finishing an existing PR. The REACH orchestrator dispatched you.

**PR:** evgunter/cad#4159, branch `reach/torus-touch-off-faces` (head `f3cc968f3c`). Read:
- the PR body (GitHub MCP `pull_request_read` method `get`);
- the unit item `work/reach/torus-touch-off-the-faces-refuses-at-the-section-pass.md`;
- the full review on `analysis/reach-review/4159` (`review.md` and `probes/`). It is APPROVE-WITH-FIXES, with 0 MAJOR.

This is the PR's LAST fix pass. You have explicit permission to commit and push to the branch, with merge commits only (never rebase or force-push). Do not merge the PR, and post no GitHub comments.

**Step 1:** merge `origin/main`. The review saw a conflict in `.config/nextest.toml`; keep both sides' slow-set entries.
- Main currently has a compile break in `crates/topo/src/tier3_tests.rs` (around :2856, `upright.revert().expect(...)`): TOPO's #4154 made `revert()` infallible, and FUSE's #4108 still calls `.expect` on it. If main has not fixed this by the time you merge, port the one-line fix (drop the `.expect(...)`), and say so in the PR body. It becomes a no-op once main carries the fix.

**MINOR-1 (fix): the plane `Touch::at` stands far off both carriers near the top or bottom parallel** (`section_cert.rs:744`, `:774`).
- `in_pi` computes `n_perp = n − a·n_a` and divides by `s`. For a generic axis the subtraction cancels, and the direction error grows as `ulp/s`. Measured: 8.9ε off each carrier at ×1e3 with a 1e-5 rad tilt, and 1.2e4 ε at a 1e-8 rad tilt. This breaks `Touch::at`'s contract (`:294–296`, "on both carriers within the margin").
- Fix it in its general form: compute the perpendicular part without cancellation, e.g. `a × (n × a)`, or from the axis frame's own components.
- `in_pi` is written twice (`:744` and `:817`, for `n_perp` and `w_perp`). Give it one home and fix it there.
- The pre-existing scrape witness at `:796` uses the same formula; fix it too.
- Grep `section_cert.rs` and its siblings for other `v − a·(a·v)` perpendicular parts that are then divided by a small norm, and fix each one or file it with a reason.
- Promote the reviewer's `probe_plane_touch_at_off_its_carriers_near_the_top_parallel` to a committed row. It must be red on the head.
- Promote the useful part of `probes/section_cert_torus_touch_fuzz.rs` as a row (a bounded subset is fine), and add the ×1e-3 and ×1e3 scales to the build rows. The current rows are all at ×1.

**NOTEs and style:**
- NOTE-2: the `!tan.zero` guards at `:763` and `:822` repeat `Pinch::touch`'s own check (`:394`). Remove them, or say in one line why they stay.
- NOTE-3: `:103`, `:289` and `:392` still promise "one small loop γ about `at`". Re-word them to what the argument actually needs, as the PR body says: a connected region, not a small disc.
- `:893` re-spells `"section_torus_sphere_near_tube"` as a literal beside the caller's `NEAR` const (`:811`). Use the const.
- σ is recovered by string-comparing `tan.name` (`:757`, `:893`). Carry it as data (an index or an enum from `signs`). If that is more than a local change, file it.
- Q3: `extent_scan_off_face_tangency.rs:174` uses the volume bound `1e-9·want.max(1.0)`, which is 10% of a ×1e-3 donut. Make it relative.
- Q2 (`:945–950`): if nothing local enforces that `r` and `e` are decided, add a debug assertion or a one-line note citing `require_ring_torus` and `axis_pose`.

**The D10 hold (Ev).** Do not extend any mechanism `docs/DESIGN.md` D10 retires: declared contacts or pairs, the placement registry, intent spellings.

**Lane conduct.** This brief is your complete instruction. It comes from the orchestrator and is pre-confirmed. Do not stop to ask for confirmation, and do not wait for further messages.

**Before pushing:**
- with `CARGO_INCREMENTAL=0` (check `df -h` first): `cargo build --workspace --all-targets`; `cargo clippy --workspace --all-targets -- -D warnings`, also with `--all-features`; `cargo fmt --check`;
- every `scripts/gates/*.sh`, and `python3 scripts/work.py lint`;
- nextest `-p topo -p sweep` at ε 1e-9, 1e-6 and 1e-12 (set with `CAD_TOLERANCE_EPS`).
- For each red, check whether it is red on `origin/main` too. Known main reds include the pinch row, `pocket_ring_steep_ellipse` and the four seam-split rows (all at 1e-6), and `arc_loft` (at 1e-12).

Show each new row red without its fix (name the mutant).

**Then:**
- push, and wait in the foreground until hosted CI on your head finishes. Poll `https://api.github.com/repos/evgunter/cad/commits/<sha>/check-runs`, and do not arm background waiters. Fix anything red that is this PR's;
- update the PR body with a "Last fix pass" section listing each finding → change → pinning row, plus the mutant results;
- end your turn by stating the head sha, the CI result and the test counts.

Commit messages end with
```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <your own session link>
```
No email address other than `evgunter@gmail.com` may appear anywhere you write.
