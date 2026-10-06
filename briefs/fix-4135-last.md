You are an IMPLEMENTER for the REACH program in `evgunter/cad`, a greenfield B-rep CAD kernel in Rust, finishing an existing PR. The REACH orchestrator dispatched you and reviews and merges your work.

**PR:** evgunter/cad#4135, branch `reach/cone-root-lane` (frozen review head `17c7349bb5`). It admits cones to the conic × quadric and line × quadric root lanes. Read:
- the PR body (GitHub MCP `pull_request_read`, method `get`);
- the two dual-review reports, `review.md` and `probes/` on `analysis/reach-dual/4135-r1` and on `analysis/reach-dual/4135-r2`.

This is the PR's LAST fix pass: the adjudicated union of both reviews. Neither found a wrong answer. You have explicit permission to commit and push to `reach/cone-root-lane`, with merge commits only (never rebase or force-push). Do not merge the PR, and post no GitHub comments.

**Step 1: merge `origin/main` first.** Resolve any conflict keeping both sides' behaviour. After resolving `.config/nextest.toml`, run `cargo nextest list --profile ci`. Re-run every `bounds_census` and box-inventory row.

**Blocking: load-bearing rungs no row guards** (r2 M-1, r1 MINOR-1 and MINOR-2, r2 m-3). The code is right; the guards are missing.
1. **Line root-slack rung** (`reduce.rs` ~3525, `bool_line_cone_root_slack`). With it removed, every row stays green, and r2's probe finds 170 in-span roots misplaced by 3e-9 to 2e-6 m at ε 1e-9, α 0.01, scale 1e3, on steep lines 4 mm to 0.5 m from the apex.
   - Add a row that places each certified root *along the edge* against an independent oracle at that pose class. Start from r2's `probe_r2_line.rs`.
   - It must go red with the rung removed (mutant M9) at ε 1e-9.
2. **Conic root-slack meter** (`conic_quadric/mod.rs` ~367-377, `bool_conic_cone_root_slack`): the same, a row that is red without it (M6/M7). If no oracle can resolve a wrong root there, pin the meter's charge directly (a unit row on its margin against a closed form) and say why.
3. **The `count >= changes` / `off <= eps` shape** (`cone_rows.rs` ~338, 346, 446, 454). It is monotone the wrong way: an invented duplicate root, or a root misplaced along a shallow crossing, still passes. Assert the exact count and the place along the edge or arc.
4. **The conic apex rung** (`conic_quadric/mod.rs` ~384-388, `bool_conic_cone_apex`) never returns `AtApex`, because the floor and slack refuse first. Either:
   - pose a row where it decides, and make `a_root_at_the_apex_refuses` (`cone_rows.rs` ~242) assert the typed `AtApex` rather than accepting `Uncertain`; or
   - if it is unreachable, prove that, delete the dead rung, and say so in the PR body.
5. **The differential `the_cone_arm_changes_no_sphere_or_wall_answer`** (`cone_rows.rs` ~554) is blind to the refactor: mutants M3a and M3b survive it, and its frozen "before" body calls the same helpers.
   - Make its sample reach `first_harmonic_arm`, so M3b goes red.
   - Either make its "before" self-contained, or retire it in favour of the cross-tree fact r2 measured (9 000 byte-identical sphere/wall answers), stated in the PR body. Your call; say which.

Show each new row red without its fix, naming the mutant.

**Cheap fixes:**
- `reduce.rs` ~2141: the comment still says "circle × sphere, × cylinder and × torus"; it now admits `Cone`.
- `implicit.rs` ~1175 and ~1372: the pad literal `1.0 + 64.0 * UNIT_ROUNDOFF` appears twice. Give it one named constant with its derivation.
- `conic_quadric/mod.rs` ~15, 25: the module docs say "floor … is therefore 1" and "`A₂` … in metres". On a cone, both are in `F` units; say so where it is stated.
- `reduce.rs` ~3236 `face_nappe(..).ok()` folds `StaleFace` and escalations alike into "no nappe" (r1 NOTE-3, r2 n-3). Match the expected error kinds explicitly, and let an escalation propagate or refuse typed. Check r2's note that `face_nappe` panics on a torn body (`offset_nappe.rs` ~75-78), now reachable from the crossing layer. If a public door can reach that panic, make it a typed refusal and add a row; if not, say why.

**File, don't fix** (`python3 scripts/work.py new … --program reach`, each with file:line citations):
- (a) The line depth rung's `R` lever (`reduce.rs` ~3504), r2 m-1: near the apex, clean pairs refuse inside δ ≈ √(2εR). Measured: 21/240 sweeps escalate at ε 1e-6.
- (b) The line lead rung, levered by segment length (`reduce.rs` ~3493), r2 m-2: a short edge reads as generator-parallel at about ±15°. Measured: 13/240 sweeps escalate at scale 1e-3, ε 1e-6.
- (c) One home for line × quadric root code. `line_cone_roots` lives in `reduce.rs`; its siblings and its quadratic live in `solid_contain.rs`. Also the cone form's three spellings (`solid_contain.rs` ~4152 `−Q`, `implicit.rs` ~1137 `+Q`, `implicit.rs` ~1206). From r2 S-1/S-2.
- (d) `boolean/mod.rs` ~3741 `sweep_split_admitting_cones`, a third copy of the gate → clone → `sweep_and_settle` preamble (r1 Q1). Also `reduce.rs` ~2615 `conic_clearance`'s cone arm re-spelling `first_harmonic_arm`'s extreme read.
- (e) The width of the apex refusal zone, as a frontier `VERBS-CONE` consumers will meet (r1 NOTE-1, r2 n-2), with both reviewers' measured nearest-answer distances.
- (f) `ConicHarmonics::floor`/`per` are kind-specific knobs on the shared struct (r2 S-6). Fold this into (c) if natural.

The existing filing `the-far-nappe-tell-off-has-no-row-where-it-decides` stands. Add both reviewers' evidence: no partial-face pose reaches it, because `point_on_cone_in_face` refuses the far nappe first (`solid_contain.rs` ~2959-2963). Note that the suggested `PartialConeFace` route may not exist.

**Known main red, not yours:** `sweep` `rest_zip_admission::the_tangent_lever_keeps_building_pure_contacts` at ε 1e-6 (from PR 4128; a fix lane is on `reach/carrier-touch-rest-zip-eps6`). `pinch_faces_tessellate::a_face_through_two_vertices_on_one_point_tessellates` and `pocket_ring_steep_ellipse::…` at ε 1e-6 are also known red on main. Check any other red on `origin/main` before calling it yours.
**Lane conduct.** This brief is your complete instruction. It comes from the orchestrator and is pre-confirmed. Do not stop to ask for confirmation, and do not wait for further messages. The session is UNATTENDED: asking blocks forever.

**The D10 hold (Ev).** Do not extend any mechanism `docs/DESIGN.md` D10 retires: declared contacts or pairs, the placement registry, intent spellings.

**Before pushing**, with `CARGO_INCREMENTAL=0` (check `df -h` first):
- `cargo build --workspace --all-targets`; `cargo clippy --workspace --all-targets -- -D warnings`, also with `--all-features`; `cargo fmt --check`;
- every `scripts/gates/*.sh`, and `python3 scripts/work.py lint`;
- nextest `-p geom-core -p topo -p sweep` at ε 1e-9, 1e-6 and 1e-12 (set with `CAD_TOLERANCE_EPS`).

For each red, check whether it is red on `origin/main` too; report main reds and do not fix them unless this brief names them.

**Then:**
- push, and wait in the foreground until hosted CI on your head finishes. Poll `https://api.github.com/repos/evgunter/cad/commits/<sha>/check-runs`, and do not arm background waiters. Fix anything red that is this PR's;
- update the PR body with a "Last fix pass" section: finding → change → pinning row, the mutant results, and the filed items;
- end your turn by stating the head sha, the CI result and the test counts.

Merge commits only (never rebase or force-push). Do not merge any PR, and post no GitHub comments.

Commit messages end with
```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <your own session link>
```
No email address other than `evgunter@gmail.com` may appear anywhere you write.
