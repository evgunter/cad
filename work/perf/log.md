# PERF log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/perf/plan.md`. A/B band 3400–3499
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening for work (2026-09-10)

PERF has been a register since 2026-07-21 (no orchestrator, no units).
Ev opened it for work in-chat on 2026-09-10 with these rulings, which
this entry is the record of:

- **Territory.** Few other orchestrators are running, so PERF lands
  fixes in whichever program's territory holds the cost and records the
  seam here; the owner's dispatched/review rows are checked first, and
  very new code is deprioritized (it will change anyway, and a perf
  change on it is work done twice).
- **Developer-side pain is in scope** (debug profile, CI, test wall
  time), not only the GUI and kernel-API seats.
- **"Stop doing this" beats "do this faster"** whenever both are
  viable — but a change of that kind that removes work a ratified
  clause asks for (D1's per-op tier-1 postcondition is the standing
  example) is an `[ev]` PR before it lands, not after.
- **Review posture: full v6 dual** on every kernel unit unless the
  orchestrator judges a unit unusually low-risk and says so in its row;
  measurement-only and instrument-only units record no row.
- **Merging.** The orchestrator merges its own units after the dual
  concludes (Ev, in-chat).
- **Ev's own pain point:** the GUI lags after edits, location unknown.
  That is the first thing the exploration wave measures.
- **Off-box measurement is allowed**: temporary GitHub Actions
  workflows may be used to take quiet timings while this 4-core box is
  running several lanes.

Opening commit: `prefix: perf/`, `tag`, band 3400–3499 claimed in
`docs/MODEL-AB-LOG.md`, this log. Exploration wave dispatched next
(Opus lanes, per Ev's budget steer): a GUI-seat lane on the edit→repaint
path, a kernel-seat lane over the demos and the Python binding, a
developer-seat lane over the debug/CI profile, and a static lane
re-verifying `plan.md` §1.3 against today's tree and the tracker's
already-filed perf findings. Each finding becomes one item file here.

## Static lane reported (2026-09-10)

Re-verification of `plan.md` at 50ac576, no builds. What changed the
ranking, recorded here so the measuring lanes' reports can be read
against it; `plan.md` is rewritten once all four are in.

- **Delivered, to delete from the plan:** the Python `evaluate` takes
  `prior=` (`crates/pncad-py/src/py/value.rs:2098`, memo documented at
  `:2039-2076`); the product-path double tier 3 (`product.rs` has no
  tier call since DOCM-5); `crates/bvh`'s header says four duties
  wired; five `par_iter` sites exist, not one (`eval/mod.rs:2380`,
  `drive.rs:1184`, `stackup.rs:513,1831`, `mc.rs:473`), and
  `mc.rs:94` defaults `parallel: true`.
- **The plan's #1 cost center is now cheap.** `mint_pcurves_of(body,
  &[FaceKey], tol)` exists at `crates/topo/src/pcurves.rs:1392` with
  one production consumer (`offset_axial.rs:639`). The whole-body
  re-mint survives at three callers: `splitting/mod.rs:650` (per split
  side, inside every boolean), `shell.rs:1704`, `transform.rs:650`.
  The remaining work is a face list per caller. Territory TRIM (the
  file) + BOOL + SHELL; TRIM has one dispatched unit, checked before
  cutting. Related open items: `topo/producer-closing-mint-is-a-
  convention-with-thirteen-copies`, `shell/shell-launders-a-stale-
  operand-row`.
- **Holds, moved:** boolean gate double tier 1 (`boolean/ops.rs:1423-
  1426`); kill-direction arena scans (`body.rs:427,450,495`, `:473`
  allocates per curve; `body.rs` is UNOWNED per TOPO's keep_out);
  `merge_group` rescan (`merge_faces.rs:1634`); `join` O(n³)
  (`join.rs:282,520,539`); `graft_solid` O(E²) (`combine.rs:415-419`);
  pass-13 `filter().count()` (`validate.rs:5385-5390`, the most-churned
  kernel file, 45 commits since 08-25); `point_in_loop` per-query
  re-walk (`containment.rs:238,246`, quiet since 08-21); `StableName`
  `Box` nesting (`names/role.rs:396+`, DOCM is editing it — deprioritize);
  two `#[inline]` in geom-core (`ring_interval.rs:87,93`); D1 per-op
  postcondition (`euler.rs:62`, ratified — `[ev]` only).
- **Already filed elsewhere, candidates to claim or ride with:**
  `mesh/torus-grid-step-one-step-both-directions` (~65× the triangles
  the chord asks for — the largest raw multiplier in the tracker, and
  it lands directly on the viewer's index build);
  `view/ui-thread-work-after-the-index-seam` (a full tessellation at
  `scene.rs:764` plus an 8×-δ probe tessellation at `:953` per
  landing); `topo/attach-postconditions-validate-the-whole-body-and-
  panic`; `shell/doors-still-read-the-whole-body-for-tier1`;
  `msolve/mate-solve-rebuilds-the-nominal-environment-per-check`;
  `issues/frame-f64-placement-is-re-evaluated-per-profile`;
  `curved/arc-aware-point-in-loop`. Closed and not to re-file: every
  product-gather duplicate (`gathers_on_this_thread()` pins the count),
  `pick-index-built-on-ui-thread`, `SHELL-10`, `TCOST-K3`.
- **Drift to look at before claiming a boolean win:** criterion
  `kernel/boolean/two_bricks` moved 130 → 150 µs between 2026-08-27
  and the latest entry, above the lane's cross-run floor.
- **Sweep blind spots stated by the lane:** quadratics behind helper
  calls (found by reading, not grep — `boolean/`, `splitting/`,
  `blend/` not read function by function); maps rebuilt per call
  inside a caller's loop (needs a call graph); traversal cost of the
  `Box` chain; and no wall-clock at all — every class above is read off
  the code.

## GUI lane reported (2026-09-10) — Ev's lag is the index build

Harness `crates/viewer/examples/perf_gui_stages.rs` on branch
`perf/explore-gui` (raw data beside it); release profile as Ev runs it
(the workspace's `[profile.release]` keeps `debug-assertions = true`),
4-vCPU box, medians of 3. It drives the app's own doors:
`DocSession::perform` → `evaluate` with `prior` → `land` →
`PickIndex::build` → `scene_focused`.

- **Where the time goes.** On every document that lags, evaluation is
  noise (0.1–0.3 ms on the million-triangle rings) and **the pick
  index is 83–97 % of the edit→picture wait**: `PickCache::sync` keys
  on `(generation, δ)` (`pickcache.rs:266,285`), every committed edit
  bumps the generation, `PickIndex::build` re-walks every root
  (`pickindex.rs:742`) and `mesh::tessellate`s each body from scratch
  (`resolve/pick.rs:354`). No content-keyed memo on that path. Split
  ≈55/45 tessellator vs BVH+ids (the BVH half is a subtraction, not a
  direct reading). Numbers: `die_composed_tour` 1375 ms of which index
  1231; `gallery_ring` 1566/1517; `tube_ring` 2192/2130. Real app under
  Xvfb on `gallery_ring`: median 3.5 s typed-edit to new picture, the
  rest being software rasterisation this instrument cannot size.
- **The memo is wired and works** (`evalseam.rs:238`; `die` 82 → 7.9
  ms) — except on `die_composed_tour`, where it saves nothing (88 ms
  either way), confirming `plan.md` §1.2's dev-profile claim at
  release. Still only 6 % of that document's wait.
- **`fit_delta`'s probe is worse than its item says.** It probes at
  8 × the *requested* δ (`scene.rs:952`), so whenever the budget has to
  coarsen by more than 8× the probe is BIGGER than the picture it sizes:
  2.1 M triangles on `tube_ring` (2744 ms frozen) and 3.3 M on
  `hollow_tube_ring` (4080 ms), on the UI thread, before the index is
  submitted. Once per opened document.
- **`scene_focused`** (`pickindex.rs:894`) is 49–62 ms per landing and
  per hide/focus change at 1 M triangles, inside `ui()`. Real but
  third-order; `app.rs` was touched today — deprioritized.
- **Refuted:** latency (repaint is requested every busy frame,
  `app.rs:1270,1302`; the spinner animates unprompted and the picture
  lands seconds later); the edit door (0.01–0.47 ms); the landing's
  gather/registry (≤ 13.6 ms worst); the closed twice-gather item.
- **Release-profile debug assertions cost 3–4.5× on evaluation**
  (`die` 82 → 28 ms, `die_composed_tour` 88 → 20, `corner_table` 9.0 →
  2.9) and 1.1–1.2× on tessellation. `Cargo.toml`'s stanza says it
  comes out before publishing; the per-op tier-1 sweep (D1) is most of
  it. Whether Ev's own binary should carry it is Ev's call — folded
  into the `[ev]` question the developer lane's numbers will complete.
- **Not measured:** the gesture path (a slider drag submits an index
  build per preview under restart-without-cancel, `evalseam.rs:79` —
  plausibly several full builds per drag) and `Open` end to end.
- **The corpus is a vocabulary corpus:** 24 of 29 documents draw under
  5 000 triangles and finish an edit in under 15 ms; the dev-profile
  rebuild-latency files are ~13× slower than release and must not be
  read as lag.

Ranking consequence: the two GUI units are (1) incremental
re-tessellation keyed on face bit-content, with the torus-sizing item
upstream of it (fewer triangles before caching any), and (2) the probe
δ. Both "stop doing this".

## Developer lane reported (2026-09-10) — D1's price, and one test row

Branch `perf/explore-dev`; CI's exact dev/test profile (opt-level 1,
line-tables-only, debug-assertions ON — no CI knob turns them off), 4
vCPU under the build-slot mutex, nextest execution wall per binary
(build excluded), 4 reps. Only `assert_euler_postcondition`'s tier-1
arm (`crates/topo/src/euler.rs:2405`, 26 call sites) was switched for
the middle column.

| binary | on (CI) | D1 sweep off | all debug-asserts off | D1 share |
|---|---:|---:|---:|---:|
| editor-core | 10.85 s | 7.47 | 6.59 | **31 %** |
| sweep | 10.63 | 10.16 | 9.72 | 4.4 % |
| topo | 3.86 | 3.55 | 3.52 | 7.9 % |
| mesh | 4.46 | 4.42 | 3.74 | nil |
| geom-brep | 46.4 (±5 %) | 46.2 | 45.1 | nil |
| six binaries | 76.2 | 71.9 | 68.7 | **5.7 %** |

- **The D1 clause costs 5.7 % of test execution and it is one crate's
  bill**: 78 % of it is editor-core, where every corpus row rebuilds
  documents through the Euler doors. Free on mesh and geom-brep,
  confirming the plan against the suite. The benches' 6.5×/5.2× and
  this 5.7 % are both true — surgery-only rows versus rows a developer
  waits on. Same family, unmeasured alone: the whole-body tier-1 sweep
  after `set_face_surface`/`set_edge_curve` (`attach.rs:93,332`),
  bounded by the residual (≤0.9 s in editor-core).
- **`crates/geom-brep/tests/budget_faces.rs:22` is one `#[test]` that
  is the whole geom-brep binary's 46 s wall**: a 2 × 7 × 5 = 70-cell
  `fit_offset_at` sweep in one row, three cores idle. Added 2026-09-08,
  after S-TCOST's census; no gate marker, runs on every code-tier run.
  Splitting it per base or per δ saves ~30 s of every geom-brep run,
  local and hosted — the largest developer item found.
- **Interval lane is 2.47× wall and 96 % of the delta is editor-core**:
  two M10 driver rows (`m10_3_r2_probes_interval.rs:638`, 93 cpu-s;
  `m10_3_r1_probes_interval.rs:450`, 85 cpu-s). `DriveConfig::default()`
  has `parallel: false` (`drive.rs:361`); the indexed map at `:1182`
  gives 3.66× on one row solo, but over the saturated binary it
  regresses 6 % — the binary's floor is its longest serial row. M10 /
  S-TCOST ground; recorded, not claimed.
- **Nobody can turn debug assertions off today**: `docm5_subject.rs`
  calls `gathers_on_this_thread`, which is `#[cfg(debug_assertions)]`
  (`product.rs:375`), so the assertions-off build fails with 12 ×
  E0425. Filed.
- **CI wall is compile, not runtime**: test jobs run a prebuilt nextest
  archive at 46–74 s per shard; a D1-shaped win is 2–4 s per shard.
- Minor, logged not filed: `mesh/src/walk.rs:826`'s O(junctions²)
  declared-junction guard is inside mesh's 0.72 s debug-assert bill;
  `demos/tour` builds release with debug assertions on and was not
  measured.

## `[ev]` opened on D1's price (2026-09-10)

Ruling `d1-per-op-tier1-sweep-price` (`needs_ev`), branch
`perf/ev-d1-price` off main carrying only the ruling item; the same
file sits on this branch, edits after Ev's answer go on the `[ev]`
branch. Recommendation put to Ev: keep the clause (option 1); take
"once per public door with replay localization" (option 3) only if
editor-core's suite wall is worth a surgery scope in `topo`. Waits for
sign-off; not self-merged.

## Kernel lane reported (2026-09-10) — the kernel-API seat

Branch `perf/explore-kernel` (stage spans in `geom-core`, corpus, demo
and Python harnesses). Release, both ways: `da_on` is the shipped
`[profile.release] debug-assertions = true` every user runs; `da_off`
is what `benches/` measures. 4 vCPU under the mutex, medians of 5,
cross-round spread ±5–8 %.

- **D1's per-op sweep is the kernel-API user's wait.** `die` 78–82 ms
  `da_on` vs 35–37 ms `da_off` (tier 1 fires 1470× for 37 ms, 46 %);
  `die_composed_tour` 90–95 vs 26 (tier 1 1704× for 60 ms, 65 %);
  every corpus row 2.2–5.5×; `demos/wild` 226 → 54 ms (72 % tier 1);
  `demos/tour` 9.5 → 7.0 s. Folded into `[ev]` #2305.
- **`StableName` keying is quadratic and measured**: `wire.boolean.
  name_emitter` is 21.4 of `die`'s 36 ms `da_off` (55–60 % of boolean
  time, ~40 % of the rebuild); per-step 0.061 → 2.451 ms along the
  21-chain. `StableName` is a recursively boxed value used as a
  `BTreeMap` key (`names/role.rs:396`, `names/emit_topo.rs:518-524`,
  `names/table.rs:69,96-107`), so each insert is O(depth) compares plus
  a deep clone. Filed.
- **`assemble`'s tier-3′ census over the aggregate is quadratic**:
  heat sink at 10/40/160 fins → 11 ms / 80 ms / 1.3 s (n^1.96); the
  gather is 23 ms at 161 solids in release (the closed LIB issue's
  250–372 ms was dev profile). `run_checks` is linear. Filed.
- **Curved-face grid density**: on a torus the grid is 110–145× the
  naive chordal-sagitta cell count at every δ, correct 1/δ scaling,
  constant ~11× per direction — the analytic-surface sibling of
  `mesh/torus-grid-step-one-step-both-directions`, which it confirms
  from a second direction. Tessellation is 71 % of the tour
  (`lily` 2.5 s, 15 bodies at 2 mm) and 100 % of the ring rows.
- **Mass properties / tier 3 on NURBS-walled bodies**: 160 ms and 157
  ms on `loft_prism` — tier 3 IS a full certified quadrature; the tour
  pays 2.2–2.7 s over 193 mass-props calls; and callers that gate then
  measure pay it twice although `validate_geometric_certificate`
  exists to hand the gate's certificate back (`demos/tour/src/main.rs:
  402,423`, the Python `validate_geometric()` + `mass_properties()`
  pair). Filed. Per-face parallel fluxes (idiom 2) is the "faster" half.
- **Python**: `prior=` buys 12× on a tail edit (89 → 7.2 ms); a
  memo-hit evaluate is 0.29 ms; the seat's own cost is mesh egress —
  `positions`/`triangles` build per-vertex tuples and `Length` objects
  (`pncad-py/src/py/mesh.rs:325,339`), 51 + 41 ms on a 333 k-triangle
  mesh (~22 % of its tessellation). Filed, low.
- **Small on the corpus, plan rank not supported**: boolean gate double
  tier 1 ≈ 2.5 % of `die` `da_off`; whole-body pcurve re-mint 0.1 % of
  `die`, 12 % of `die_composed_tour`, 0.2 % of the tour; the arena-scan
  family jointly ≤ 17 % and not separable. CDT bulk-load's trigger is
  still unmet — no corpus document reaches the holed-planar quadratic.
- **The memo is worthless where the DAG is one wide node**
  (`die_composed_tour`, `heat_sink`): the recomputed cone is the
  document. Not a defect; a fact about those documents.
- Criterion's six rows do not predict this seat (planar microseconds
  where curved bodies are 100+ ms) — a curved row is owed there.

## Block PERF-B1 drawn (2026-09-10) — BRANCH-SIDE RECORD, merges at block end

Protocol v6, implementer ratio 1:2. Pre-draw fields logged before the
byte: PERF-3 = S / structural; PERF-1 = M / numeric; PERF-2 = M /
structural. Draw: `/dev/urandom` byte **64** (< 252), 64 mod 3 = 1 ⇒
fable position 1. Slots by dispatch order: **slot 0 PERF-3 = OPUS,
slot 1 PERF-1 = FABLE, slot 2 PERF-2 = OPUS.** Review ordinals claim
from 3400 at each review dispatch and are recorded on main then; this
record stays on `perf/orchestrator` until the block concludes, and
anything that must reach main earlier goes on its own branch off main
(the SHELL layout). Lanes clone with `local-scripts/new-lane.sh`, own
target dirs, unit branches off main carrying the item and spec.
Beside the block, no row (Ev: unusually low risk, single review with a
correctness arm): the probe δ, gate-then-measure, `budget_faces` split,
and the `gathers_on_this_thread` cfg — dispatched as lanes free up.

## Ev ruled on D1 (PR 2305, 2026-09-10) — PERF-4 cut; block PERF-B2 drawn (BRANCH-SIDE)

Ev: "this change sounds great!" — option 3, once per public door with
localization recovered on failure; the attach setters join. Ruling
closed on the `[ev]` branch. PERF-4 (`docs/PERF-4-SPEC.md`, branch
`perf/4-door-postcondition`) executes it and revises DESIGN.md D1/D9
and the `euler.rs` docs. Dispatched beside B1's three lanes (four
builders under the width-1 mutex; disk at 22 GB free): it is the
kernel-API seat's largest single item and Ev is engaged.

**Block PERF-B2 pre-draw fields, logged before the byte:** PERF-4 =
M–L / structural (slot 0); slots 1 and 2 are the per-face patch memo
and the +V sign-certified door, cut after PERF-3 lands and drawn
here now so the block is a triple.
Draw: `/dev/urandom` byte **181** (< 252), 181 mod 3 = 1 ⇒ fable position 1. **Slot 0 PERF-4 = OPUS.**

## PERF-1 merged (2026-09-10, PR 2307)

Dual: R1 (opus) APPROVE-WITH-FIXES 0/5/10, rubric 4/4/2; R2 (fable)
APPROVE-WITH-FIXES 0/4/3, rubric 4/4/3; both attacked the bound
independently (60 000 and 4 764 triangles, worst ratio 0.99994 and
0.99959 — tight) and both found the same real defect: `cert_torus`'s
`A = R + r cos_max` was a sup only for R ≥ r (a spindle-torus probe
under-certified 3.5×). Fix pass (union, 12 items) landed on
`bd831cded`, CI green; state-sync closed the unit and the claimed
torus item. Measured: `hollowring` 3 984 276 → 164 940 triangles at
the viewer's δ; `tessellate/torus/1e-3` 178 → 8.5 ms, `/1e-4` 1.95 s →
79 ms; tour wall 10.1 → 7.3 s; tess-budget sweep 1 653 556 → 299 066
triangles, NURBS rows byte-identical. Main's render lanes re-baseline
the torus scenes post-merge. Method note: R1 took the express lane for
its test runs under mutex starvation (its own choice under the same
brief; not an orchestrator grant, so the pair stands). Residue filed:
`torus-sizing-reads-no-phi-window`. Reviewer class findings carried
forward: the exact-torus-distance sampler is spelled in four places
(test side), and history-narrating comments around triangle counts
are a class — swept in the fix pass with the hit list in the PR body.

## PERF-3 merged (2026-09-10, PR 2308); PERF-4 implemented (PR 2313)

PERF-3 dual: R1 (fable) APPROVE 0/2/4, rubric 5/4/4; R2 (opus)
APPROVE-WITH-FIXES 0/5/6, rubric 4/4/3. Both held every claim (one
planted `base = shared_below` and the goldens went red 14/40; one
built an `Approx` body and got identical digests on both trees). R2's
sharpest finding was real and in the shipped profile: the new census
helper copied every patch's triangles before the census's empty-set
short-circuit. Fix pass (union, 8 items) on `3f3f8ba0a`, CI green;
merged main (PERF-1) under it with a no-refactor control attributing
the two moved `donut` digests to the torus grids. Item
`index-rebuilds-every-root-on-every-edit` un-parked: the per-face
memo is dispatchable. Carried into the memo unit's spec: the three
lanes' argument shapes and the trimmed lane's `&mut FaceBounds`.

PERF-4 (Ev's D1 ruling) is implemented on PR 2313, CI green: a
debug-only `SurgeryDepth` (atomic, so `Body` stays `Sync` for rayon),
a `Surgery` guard that decrements on drop and sweeps only on explicit
close at the outermost level, `Body::adopt` for the staging doors, the
`per-op-postcondition` feature as the scalpel, and the door walk's new
`SurgeryPosture` needle (48 doors: 16 per-call, 4 once-per-door, 28
allowlisted). One deviation outside its file fence, judged in the
rule's spirit: `step_import::build_one_solid` is a public door that
runs operators directly, and scoping it is what moved `demos/wild`
(246 → 76 ms). Contended-box readings: `die` 88 → 45 ms in the shipped
profile, editor-core's test wall 56 → 20 s. Filed by the lane:
`door-scopes-outside-topo-are-unguarded`. Dual review next.

## PERF-2 and PERF-4 duals concluded; fix passes running (2026-09-11)

**PERF-2** (PR 2311): R1 (opus) APPROVE-WITH-FIXES 1/3/4, rubric
3/2/3; R2 (fable) APPROVE-WITH-FIXES 0/4/5, rubric 4/3/4. Both held the
order-invariance claim under mechanical attack (a structural oracle
over 722 770 ordered pairs, 15 836 unstamped probes, post-seal inserts,
forward/reverse/rayon corpus digests; an 8-thread shared-handle probe)
and both reproduced the win (`name_emitter` 18–20 → 4.3–5.1 ms, per-
name cost flat where it grew 2.7× before). The one finding both made,
rated MAJOR by one: the stamped-order invariant the repo-wide
`clippy.toml` exception rests on is enforced by nothing — no test, no
tripwire, `seal_order` unreachable from tests. Not a unilateral MAJOR.
Adjudication: the shape stands (a per-evaluation interner was argued
both ways by the two reviewers and is recorded, not re-litigated); the
fix pass adds an O(n)-per-seal debug tripwire (not per-compare — the
shipped release profile keeps assertions on) and unit tests, stops
stamping at epoch saturation, routes the four iteration readers that
bypass `upstream_name` through the sharing doors, corrects the
`two_bricks` attribution (the bench runs no editor-core code), states
the `RoleSeg` field-type change as source-breaking for library users,
files the Θ(names) residual, and updates `names/README.md`.

**PERF-4** (PR 2313): R1 (fable) APPROVE-WITH-FIXES 1/6/4, rubric
3/4/4; R2 (opus) APPROVE-WITH-FIXES 1/5/6, rubric 4/4/4. Both held the
mechanism (neutered sweeps red the three door rows; the scalpel arm
fires; clones reset; every staging write is `adopt`). Both found one
MAJOR and it is the same class from two sides — the guardless
`enter_surgery`/`leave_surgery` pair: one proved by mutation that a
deleted `leave_*` at any of 15 guardless sites goes undetected and the
walk covers only the population where the guard already makes a leak
impossible; the other that `enter_surgery` is a public `&self` method
that silences D1 on any body from any crate, that `&mut self` compiles
clean, and that only two of eleven sites need the guardless form. Not
unilateral. Paired floor on one box: `die` 86.4 → 43.5 ms against a
38.6 ms no-tier-1 floor (4.9 ms over 84 nodes, ~58 µs per door sweep);
`demos/wild` 261 → 46 ms. Fix pass: guards everywhere they fit, the
pair `&mut self` and `pub(crate)`, the hole sized honestly in the filed
item and the D9 sentence narrowed to what fires, a per-PR CI row that
runs the corruption rows under the scalpel feature, `DOORS_MEASURED`
re-measured, five rotted premises, `revolve/full`'s double check, and
the repaired-mid-door class named in `surgery.rs` and D1.

Method note for both: PERF-2's R2 and PERF-4's R1 (both fable) parked
on background monitors once and were corrected to foreground polling
— corrections, not relaxations; the pairs stand. A reviewer's shared-
target-dir hazard (one target serving two trees' example binaries by
path-independent hash) is already in `memories/agent-lane-operations.md`.

## PERF-2 merged (2026-09-11, PR 2311) — block PERF-B1 concludes

Fix pass on `cb0621a0` (CI green): the seal walk's debug tripwire
(mutation-checked both ways, 1.5 % with assertions on), a `table.rs`
test module, epoch saturation, the four bypassing readers routed
through the sharing doors, `NameTable`'s `Debug` without the cache
flag, clones re-sealing, `names/README.md` present-tense, the residual
filed (`naming-a-boolean-chain-is-theta-names-per-step`). Final: `die`
33.7 → 18.8 ms, `name_emitter` 18.5 → 4.7 ms. All three B1 units are
on main; the block record and the three A/B rows go to
`docs/MODEL-AB-LOG.md` on their own branch off main now that no
unstarted slot remains to leak.

## Ev's teapot PR rides PERF-6 (2026-09-11)

Ev (in-chat): merge PR 2306 — the teapot's spout goes back to circles
and exhibits the `VolumeUncomputable` refusal at ε = 1e-12 instead of
dodging it with octagons — once the fix it requires lands. That fix is
PERF-6 (the sign-certified volume door). At PERF-6's merge: a lane
merges main into `demos/teapot-round-spout` (PERF-1 re-cut the same
tess-budget baseline, so the CSV is re-cut on the merged tree with the
sweep, never merged by hand), undrafts, confirms the ε sweep green at
1e-12, and the orchestrator merges. If the sweep stays red after
PERF-6, that is a PERF-6 gap to report, not a demo to bend.
