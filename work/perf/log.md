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

## PERF-4 merged (2026-09-11, PR 2313) — D1's clause is once per door

Fix pass on `d630d261` (CI green, plus the new per-PR scalpel step in
`clippy (--all-features)`): 13 of 15 guardless sites take the
`Surgery` guard, the surviving pair is `pub(crate)` and `&mut self`
with the reason at each of its two sites, `DOORS_MEASURED` 48, the
vacuous walk arms replaced, `revolve/full` matched to `partial`, five
rotted premises rewritten, the repaired-mid-door class named in D1 and
`euler.rs`, the residue item sized to what cannot fire (the `Err`-arm
close, the lexical read's population, a deleted sweep is not a leaked
scope). Two self-found reds on the way: a panic-hook flake (one
process-global mutex now) and rustdoc links to private items.
`docs/DESIGN.md` D1/D9 now state the once-per-door rule with the
ruling cited. Block PERF-B2 slot 0 done; PERF-5 under dual review,
PERF-6 implementing.

## Side unit merged: `budget_faces` rows (2026-09-11, PR 2321)

Test-only, no A/B row, orchestrator-reviewed: the 70-cell
`fit_offset_at` sweep becomes fourteen `(base, δ)` rows nextest can
spread, every cell's assertion kept, the two witness faces (sample cap;
not-finite with no finite round) asserted by name on the columns that
reach them, and every column refusing if it fitted all five tolerances
— the one way the sweep could rot silently. S-TCOST territory, landed
from PERF by announcement (their census predates the file). The lane
measures the geom-brep binary's wall in its report; recorded there.

## Burst after the box restart (2026-09-11/12)

- **PERF-5** (PR 2315, `61d4ba406`, CI green): implemented — level 1
  reuses a root's `NodePick` when the evaluation memo reused the node,
  level 2 reuses a face's patch under a content digest of the lane's
  inputs; `die_composed_tour` index 235 → 104 ms with tessellation 155
  → 11 ms, and the BVH is now ~95–98 % of what remains (filed:
  `bvh-is-the-index-after-the-memo`). R1 reported APPROVE-WITH-FIXES
  2/6/5: the level-1 key omits the naming key the evaluation memo
  itself requires for reuse (a content hit with a naming miss re-mints
  the body, and the served mesh's face keys then rest on identical
  arena allocation — stronger than the cited theorem); and the key's
  chord positions cannot be falsified by any row (the uniform chord
  schedule makes them a function of inputs already keyed; dropping
  them reds nothing, dropping δ reds both suites). R2 died on the
  Fable usage limit mid-review; re-dispatched on the same frozen head
  after Ev reported the reset. The pair is recorded as interrupted
  (v6 3(e)): full record, excluded from the tally.
- **PERF-6** (PR 2339, `dd0500150`, CI green): implemented — a second
  certification level as a second type, `SignCertificate` with no
  volume field; the sign loop runs round 0 over every face in arena
  order and advances only until the sign settles; the reporting door
  is untouched and digest-pinned; the arc-loft body main refuses at
  1e-12 with `rounds: 1` is admitted at every ε. Costs disclosed: no
  saving on `loft_prism` (its walls take the exact per-span arm), an
  arc loft 2379 → 768 ms at 1e9·ε, +11.5 % on a body whose sign never
  settles. Filed by the lane: the tier 3′ (STEP import) gate still
  couples (`tier3-prime-still-couples-plus-v-to-the-reporting-target`)
  and an EXCH item; it extended two ratified `bounds_allowlist` ledger
  entries in `geom-core/src/real.rs` as a judgement call — the dual is
  asked to rule on whether that owes an `[ev]`. Dual dispatched
  (ordinal 3405).
- **Side units** (opus, no rows): `budget_faces` rows merged (PR 2321;
  geom-brep's binary wall 341 → 208 s in CI's profile — and the item's
  46 cpu-s figure was an optimized-profile reading, the row is 347
  cpu-s in the profile CI runs); the gather-counter cfg (PR 2328,
  green) merged after orchestrator review. S-TCOST findings in PR
  2321's body: this file's ε axis buys nothing (the 70-cell face map
  is identical at all three rows) and its tolerance axis is near-
  degenerate — theirs to act on.

## 2026-09-12 — limit reset: PERF-6 review dispatched, side unit landed

- **PR 2328 merged** (`gathers-counter-blocks-the-assertions-off-build`
  closes with it): the twelve `gathers` counter reads in
  `crates/editor-core/tests/docm5_subject.rs` are `#[cfg(debug_assertions)]`
  per statement; full CI green on run 34557038654; merged at
  `9980a8a7f`.
- **PERF-5 R2 re-dispatched** (fable) on the same frozen head `61d4ba406`
  after Ev reported the limit reset; the pair stays recorded as
  interrupted (v6 3(e)) and is excluded from the tally. R1's report is
  in hand; adjudication waits on R2.
- **PERF-6 dual review dispatched** on frozen `dd0500150` (PR 2339):
  draw byte 109, parity 1 ⇒ R1 = FABLE, R2 = OPUS; ordinal **3405**
  claimed on main (`perf/ab-claim-3405`, docs-only). Both briefs ask the
  reviewer to run Ev's teapot scene (PR 2306's branch merged with the
  frozen head) at ε = 1e-12 — the finding's own body, not the fixture's
  arc loft — and to attack the sign decision's soundness in both
  directions (a valid body newly refused, an inside-out body newly
  admitted).

## 2026-09-12 — limit reset: PERF-6 review dispatched, side unit landed

- **PR 2328 merged** (`gathers-counter-blocks-the-assertions-off-build`
  closes with it): the twelve `gathers` counter reads in
  `crates/editor-core/tests/docm5_subject.rs` are `#[cfg(debug_assertions)]`
  per statement; full CI green on run 34557038654; merged at
  `9980a8a7f`.
- **PERF-5 R2 re-dispatched** (fable) on the same frozen head `61d4ba406`
  after Ev reported the limit reset; the pair stays recorded as
  interrupted (v6 3(e)) and is excluded from the tally. R1's report is
  in hand; adjudication waits on R2.
- **PERF-6 dual review dispatched** on frozen `dd0500150` (PR 2339):
  draw byte 109, parity 1 ⇒ R1 = FABLE, R2 = OPUS; ordinal **3405**
  claimed on main (`perf/ab-claim-3405`, PR 2416, docs-only). Both
  briefs ask the reviewer to run Ev's teapot scene (PR 2306's branch
  merged with the frozen head) at ε = 1e-12 — the finding's own body,
  not the fixture's arc loft — and to attack the sign decision's
  soundness in both directions (a valid body newly refused, an
  inside-out body newly admitted).

## 2026-09-12 — PERF-5 dual review adjudicated; fix pass dispatched

Both arms APPROVE-WITH-FIXES on frozen `61d4ba406` (PR 2315). The pair
is recorded as **interrupted** (v6 3(e): one arm died twice on the
session usage limit and was resumed from its transcript on the same
head) and is excluded from the tally; its row is kept in full
branch-side. Adjudicated union sent to the implementer:

- **MAJOR (1, upheld from one arm):** the level-1 key is the content
  key alone, while the eval memo's reuse condition — the theorem the
  spec (§1) and `pick.rs` cite — is content key AND naming key; a
  content-hit/naming-miss re-mints the body and the served id map is
  right only if re-minting yields identical `FaceKey`s. Fix: key on the
  eval memo's own condition.
- **Downgraded to MINOR:** chord positions and parameters in the key
  cannot be falsified at the body level (both arms: on the uniform
  schedule they are functions of carrier + interval + count, already
  keyed); the key stays a superset, the rows say what they prove.
- **MINOR (7):** seam flag and `carrier_id` read by the curved lane
  and not folded (one arm); no asserted positive hit (both arms);
  `ThreadIndexer` lifetime untested; `patch_memo` digest weaker than
  its comment; measurement columns that do not partition the index
  figure (BVH 98 % vs a re-measured 77 %); `mesh/src/lib.rs` doc rot.
- **NOTE (6):** the FNV digest now in ≥10 copies and `PickMemo`
  re-spelling `PatchMemo`'s picture machinery — to be FILED, not
  consolidated in the fix pass.

Second arm's measurement on the loaded box reproduces the shape:
`die_composed_tour` index 442 → 180 ms (tessellate 292 → 21; BVH 140).

## 2026-09-12 — PERF-6 dual review adjudicated; fix pass dispatched; a ruling for Ev

Frozen `dd0500150` (PR 2339): one REQUEST-CHANGES, one
APPROVE-WITH-FIXES, **two distinct MAJORs, one per arm, both verified
on the branch**. The pair is recorded as interrupted (both arms died on
the session usage limit and were resumed from their transcripts on the
same head) and is excluded from the tally.

- **MAJOR (arm 1, the correctness hole):** `plus_v_invariant` maps
  `PlusVOutcome::Undecided` to no error and nothing in `validate.rs`
  reads `target_refusal()`, so a body whose sign is still undecided
  when the schedule runs out now PASSES tier 3 where main refused
  `VolumeUncomputable` — demonstrated on a thin curved strip at three
  ε; an inside-out twin passes identically. This is the inside-out
  question Ev's finding is about, and the spec's "refuses exactly as
  today" (§1). Fix: carry the outcome out of `sign_certified` as the
  one decision (which also stops the two predicates being minted twice
  per body) and refuse on Undecided.
- **MAJOR (arm 2, the pin):** the reporting-door digest's roster does
  not reach the cylinder lane (`bulged_extrusion` is a closed form;
  three of five rows are ε-invariant), so the lane whose budget exit
  moved has no bit pin. Fix: extend the roster to every certified lane
  and the corpus/tour bodies, cut on the merge base.
- **MINOR (9):** the continuation can name a different refusing face
  than the reporting door (both arms); the cost is understated at the
  site and in the PR (the "one round-0 pass" bound is false; gate +
  continuation wall time is 1.3–1.8× one measurement because per-face
  setup is re-entered per window — to be filed, not fixed); a stale-
  premise class across step-import tests and `props.rs`'s header; `lo`
  reconstruction rounds toward admission now that `lo` is consumed;
  the exit ladder in three copies; a third `quad_verdicts` copy; the
  reuse identity exercised on one body per ε row; three silent spec
  deviations to disclose.
- **Both arms escalate** the two extended `bounds_allowlist` ledger
  paragraphs: a re-argument of the same seam rather than a widening,
  but ledger text binds future lanes, and `SignCertificate` is now a
  public type where the paragraphs say "the same private certified
  half". **Goes to Ev**: the PR carries an `[ev]` section and is
  retitled after the fix pass; it merges on Ev's sign-off.
- **Both arms ran Ev's teapot** (PR 2306 merged with the head) at
  ε = 1e-12: tier 3 passes; the tour then panics at
  `demos/tour/src/main.rs:423` on the reporting call with the same
  `QuadratureBudget` payload. PR 2306's sweep stays red at 1e-12 until
  the riding consumer change lands — and what the tour should do with
  a tier-3-valid body whose reporting quadrature refuses is a demo-
  policy question for Ev (report the sign-level bracket, or drop the
  demo's volume check at tight ε).
- Premise corrections from the arms, for the record: k-lint pins no
  predicate counts (the spec's "mechanical pin" sentence was wrong;
  the count pin is the tests' `gate + refine == one`); the STEP
  per-solid gate is already `validate_geometric` — only the aggregate
  gate is tier 3′, so the filed item overstates.

## 2026-09-12 — PERF-5 merged

Fix pass landed everything in the adjudicated union with nothing
disputed (head `a00a772c4`, CI green on run 34679512564): the node-level
key is the eval memo's own reuse condition; seam flag and split lineage
folded into the face key; asserted hit floors per corpus step; a
`ThreadIndexer` row across landings and a skipped generation; the
measurement table partitioned (index = tessellate + BVH + rest) and the
BVH item corrected to ~85 %; memo footprint measured (24 MB on
`tube_ring`, 7 MB on the tour die); the FNV/memo-machinery duplication
filed as `fnv-digest-and-memo-machinery-copies`. State-sync rode the PR
(`PERF-5` and `index-rebuilds-every-root-on-every-edit` closed); merged
at `768faf5fa` (PR 2315). Plan §1.3 and §5 updated; the lane's clone,
target and scratch reclaimed.

## 2026-09-12 — Ev: the demo reports the bracket

Ev asked whether the reporting quadrature's precision scales with ε
correctly. Answer given (from `quad.rs`'s own envelope docs): the
target `1024·ε` scales with ε, the fixed 12-round schedule's floor does
not — it is a part-size property (the spout's floor is 2.53e-8 m at
every ε; the 1e-12 target sits 25× under it). Options put: let the
round cap grow with ε (about 3× the quadrature per decade of ε; an
`[ev]` change to the reporting contract), or stop asking the reporting
door for what the demo does not need. **Ev chose the second**: the
tour continues tier 3's sign certificate and, on a budget refusal of
a valid body, prints the enclosure and checks the mesh volume against
the bracket. Recorded on `gate-then-measure-pays-two-quadratures`,
which is the unit that lands it (a side unit, single review) after
PERF-6 merges; PR 2306 follows it.

## 2026-09-12 — the ledger extension is not a second decision

Main's CLAUDE.md (PR 2432) now says what waits for Ev is the design
choice — retiring a clause or changing what it decides — and a clause
re-worded because an approved change moved something it describes (a
count, a caller) lands with the change. PERF-6's two `bounds_allowlist`
paragraphs are exactly that: the counts moved (props.rs 14 → 19,
validate.rs 9 → 10) because one certified walk became a hook, a
windowed walk and the certificate that resumes it; scope-by-scalar did
not move (both reviewers, and my reading). So PERF-6 lands the way
every reviewed unit does — its dual review is adjudicated, the fix
pass closes the union, CI is green on the head — with the ledger
section in the PR body as disclosure and the word "private" corrected
(the certificate is public now). The demo-policy question is ruled
(above); the PR carries no `[ev]` ask.

## 2026-09-12 — PERF-6 merged; the demo's ribbon unit dispatched

Fix pass landed the whole union with nothing disputed (head
`d18fd313a`, CI run 34685286220 green). The correctness hole is
closed by construction: `sign_certified` now returns a verdict beside
the certificate, `PlusVVerdict::Uncomputable(source)` is the undecided-
with-exhausted-schedule case and it carries the reporting door's own
refusal; a strip row pins it in both senses at three ε. The digest
roster reaches every certified lane with an ε-coupled body (a quintic
loft on the composite rounds, a tilted cut on the cylinder Green form),
cut on the merge base and byte-identical at three ε. The predicates
are minted once per body again; the continuation short-circuits as the
reporting walk does; the exit ladder is one helper; the stale premises
are swept; `validate.rs`'s ledger count is back at its ratified 9 (the
subject trait is gone) and `props.rs`'s paragraph says the certificate
is public. Two items filed by the lane: `quadrature-setup-is-re-derived-
per-round-window`, `sense-inversion-is-invisible-to-tier-3-on-arc-
capped-lofts`. State-sync rode the PR (`PERF-6` and Ev's finding
closed; the riding item re-homed as a standalone side unit — lint: a
struck row may not delete its passengers); merged at `cfee8f155` (PR
2339). Block PERF-B2 is complete on the code side; its A/B record
folds into `docs/MODEL-AB-LOG.md` next.

**Side unit dispatched**: `gate-then-measure-pays-two-quadratures`
(opus, single review, brief stored): the tour and the Python pair gate
then measure with one quadrature, and the tour's ribbon reports the
sign-level bracket when the continuation refuses budget on a valid
body — Ev's ruling. Its pin is the tour at 1e-12 on the teapot scene
from PR 2306 running to completion; PR 2306 follows it.

## 2026-09-12 — block PERF-B2's A/B record folded

`docs/MODEL-AB-LOG.md` gains the block's concluded record (branch
`perf/ab-block-b2`, docs-only): the draw (byte 181), the three rows
with samples #170–#172, and the tally — four clean pairs toward the
twelve with no unilateral MAJOR among them; PERF-5 and PERF-6 recorded
in full but excluded as interrupted pairs (the usage limit killed every
running reviewer once or twice; each resumed on the same frozen head).
The B2 draw and the row notes leave this branch with that fold; merged
at `01ce06844` (PR 2439).

## 2026-09-12 — the ribbon side unit is up for review

PR 2440 (`perf/side-gate-then-measure`, head `1097d870f`, CI green):
`run_body` takes the certificate from tier 3 and continues it; a
`QuadratureBudget` refusal of an admitted body yields a `Measured::
Bracket` and the ribbon prints `lo`, `hi`, `A` at SIGN level and
checks the mesh volume against the bracket with the chordal slack;
every other refusal still panics; the 3′ arms untouched. The spout at
ε = 1e-12 now runs to completion (`V in [4.994e-5, 6.019e-5]`, the
scene's own oracle 5.505e-5 inside). Main's scenes' stdout is
md5-identical before/after at three ε. Python: `validate_geometric_
measured`, one call that gates and measures, the bracket on its budget
refusal; guide §2.4. Measured: the tour's 78 `run_body` bodies
6.09 → 3.12 s of gate+measure (1.95×): on these bodies the sign does
not settle early, so the continuation re-enters no lane; where it
settles early the saving is nil (stated in the test). Deviations
reported: the saving beyond the item's 1.3–1.8× expectation, the
Python test class at 14.7 s, `ruff format` unapplied. Filed from its
out-of-fence report: `gate-then-measure-class-remains-in-scene-modules`
(the scene modules and an stl example; `validate_probe` stays as the
telemetry instrument — ruled here). Single review dispatched (opus;
brief stored).

## 2026-09-12 — the ribbon side unit reviewed; fix pass dispatched

Single review APPROVE-WITH-FIXES (2 / 8 / 9). Both MAJORs upheld:
the PR's `teapotspout` measurement row was main's octagonal spout
mislabelled — on the round spout the gate settles early and the
continuation re-enters the lane's setup, so the saving there is nil
(1.002× at 1e-9) while the 60 polynomial-walled tier-3 bodies save
1.97–1.99×; the regime, not one number, is the claim to state at the
sites. And `run_body`'s two tier-3′ arms (17 of 78 bodies) still pay
two quadratures although `validate_pseudomanifold_certificate_
certified` hands the number back — in scope by the item's own
citation. MINORs: the slack read the enclosure's midpoint area, the
soundness sentence argued a one-sided check for a two-sided
assertion, the Python refusal row paid a redundant quadrature, a
stale door count, the demo hand-spelling a kernel error pattern two
crates deep (to be filed as a library finding — `target_refusal` is
unreachable once `refine_to_target` consumes the certificate). Ruled
no-change: the tour's bracket arm is unreached on main's corpus until
PR 2306 merges (the next merge, exercised hosted at 1e-12 by k-lint);
a containment check is monotone-wrong by nature and is stated as a
sanity ribbon.

## 2026-09-12 — the ribbon side unit merged; Ev's teapot PR in its lane

Fix pass landed the union with nothing disputed (head `e21a4d6ec`,
CI run 34692336069 green): the measurement re-stated as two regimes
at every claim site (the tour's 61 polynomial-walled tier-3 stops
1.99×; the round spout, whose sign settles early, nil — `refine_to_
target`'s doc and the setup item say which is which); all three
`run_body` arms hand their measurement back from the gate they ran,
stdout byte-identical at three ε; the slack is the mesh's own area;
the Python refusal row pays one quadrature (class 8.0 s); the demo's
hand-spelled kernel error pattern recorded as `budget-refusal-drops-
the-enclosure-the-caller-needs` (kernel territory: a continuation's
budget refusal should carry the enclosure, or the certificate be
continuable by reference). State-sync closed the item; merged at
`f752e9c21` (PR 2440). Lane reclaimed.

**Ev's PR 2306** now has its lane (brief stored): merge main into
`demos/teapot-round-spout` by merge commit, regenerate the tess-budget
baseline with `scripts/tess_budget_sweep.sh`, run the tour at three
ε (the spout's SIGN-level line at 1e-12), un-draft, hosted run green
including `k-lint (gate, release-default)`; I merge on its report.

## 2026-09-12 — Ev's teapot PR merged

PR 2306 (`demos/teapot-round-spout`) merged at `f2a4adf2f` on Ev's
standing request, once the kernel fix (PERF-6) and the ribbon (PR
2440) were on main. The lane merged main by merge commit, took main's
side of the baseline and the tess-lint tests, re-cut the baseline with
`scripts/tess_budget_sweep.sh` (every moved row the spout's: 18 → 10
rows, 9 922 → 71 586 triangles — a round spout costs 7× a faceted one;
two census alarms return to their pre-octagon shape for the same
reason), and retired five narration sites that asserted the tier-3
refusal as live. Tour exit 0 at three ε; at 1e-12 the spout's ribbon
prints the SIGN-level bracket. Hosted run 34694562127 fully green
including `k-lint (gate, release-default)`. The lane also dropped the
now-false `[BLOCKED]` title prefix and rewrote Ev's PR body around
the same argument — flagged to Ev with the offer to restore it.
Lane reclaimed; no PERF lane is running.

## 2026-09-12 — Ev: cut the next block; block PERF-B3 drawn (BRANCH-SIDE)

Ev on the proposed block ("parallel per-face tessellation and mass-
property fluxes, the BVH residual, the fit-delta probe beside it"):
"sounds good … proceed in the order that you see fit". Three units
cut and specced: PERF-7 (`tessellation-is-serial-per-face`,
`docs/PERF-7-SPEC.md`, MESH territory), PERF-8 (`mass-properties-are-
serial-per-face`, `docs/PERF-8-SPEC.md`, PROPS/TOPO plus a composing
door on `geom_core::k_stats` — the funnel's recording is thread-local,
which is the real content of that unit), PERF-9 (`bvh-is-the-index-
after-the-memo`, `docs/PERF-9-SPEC.md`, the index seam in editor-core
and `crates/bvh`). All three are independent of each other and are
dispatched together; the display probe follows as a side unit when a
lane frees its disk.

**Block PERF-B3 pre-draw fields, logged before the byte:** PERF-7 =
M / structural (slot 0); PERF-8 = M–L / structural + numeric (slot 1);
PERF-9 = M / structural (slot 2). Dual review on every unit (kernel
and index-seam code; none is low risk). Ordinals continue in the PERF
band from 3406.
Draw: `/dev/urandom` byte **173** (< 252), 173 mod 3 = 2 ⇒ fable position 2. **Slot 0 PERF-7 = OPUS, slot 1 PERF-8 = OPUS, slot 2 PERF-9 = FABLE.**

## 2026-09-12 — block B3 lanes near their PRs; the display probe dispatched

All three B3 implementers are at or near their PRs an hour in (PERF-8
has PR 2452 up and its item at `review`; PERF-7 is merging main;
PERF-9 is on its pin rows). Beside the block, the display-probe side
unit is dispatched (opus, single review; brief stored): `scene::fit_
delta` must never tessellate more than the picture it sizes — probe
coarse and solve, or move the probe onto the index worker — with
every document's committed δ pinned equal to main's. VIEW territory
(`crates/viewer/*`), announced here; `work/view`'s `ui-thread-work-
after-the-index-seam` names the same cost.

## 2026-09-12 — block B3: all three units under dual review

PERF-9 (PR 2451, `aa64fab67`, ordinal 3406, R1 opus / R2 fable): the
pick index as per-patch trees under a top-level tree, memoized beside
the patches; the tour die's memo'd index build 131 → 34 ms; the
implementer found and filed (docm) that a grazing-ray pick answer
depends on candidate order and reproduced the single-tree sequence
rather than paper over it. PERF-7 (PR 2448, `c26d3220f`, ordinal
3407, R1 fable / R2 opus): the face loop as idiom 1; a third shared
mutable the spec did not name (`mesh::budget`'s thread-local meter)
found and composed through the fold; `tube_ring` 968 → 512 ms at four
threads, the rings' first open 1565 → 1105 ms; a fixed price below a
millisecond filed rather than hidden behind a serial arm. PERF-8 (PR
2452, `4357cfd71`, ordinal 3408, R1 opus / R2 fable): `k_stats`
gains `detached`/`splice`; both walks as idiom 1 with recordings
spliced in arena order; `loft_prism` 154 → 46 ms, the tour 1.76×;
found that editor-core's existing parallel maps lose the funnel and
the symbolic session (filed under `work/wire`); a serial-dispatch arm
under a symbolic session, disclosed — the reviewers are asked whether
it is a twin. PERF-8's review dispatch waited ~40 min for disk behind
PERF-9's reviewers (eight lanes on the box). One PERF-9 arm parked on
a monitor once and was corrected to foreground polling.

## 2026-09-12 — PERF-9 dual review adjudicated: a clean pair; fix pass dispatched

APPROVE (0 / 2 / 2 + 7 style) and APPROVE-WITH-FIXES (0 / 1 / 5) on
frozen `aa64fab67`: **no MAJOR on either arm.** Both held the pick-
identity claim adversarially (one arm with ~450 rays incl. NaN, poison
and empty patches plus a negative control showing the spec's literal
"ascending patch order" reds; the other with 65 279 rays over the
corpus and a moved-shared-vertex row) and both read the same-set
argument and found it correct; both reproduced the tour die's memo'd
index 131 → 33 ms and found no first-open regression. Union to the
fix pass (11 MINORs, all small): the tree memo's memory is not
marginal (the `Arc`s are the node level's) and the unread
`tree_bytes` chain goes; the per-build triangle-table copy and box
recompute — the real residual — filed, not fixed; the third instance
of the generational-memo machinery added to the item that names the
class; the key-coverage overclaim rewritten around `same_boxes`; the
box comparison moved into `bvh`; the keys-short arm made
`unreachable!` or typed; a `tol` row and a tree-for-tree row; one
unreproduced measurement sentence softened. One arm parked on a
monitor once and was corrected (a correction, not a relaxation).

## 2026-09-12 — the display probe is up for review

PR 2464 (`perf/side-fit-delta-probe`, head `251efe3a4`, CI green):
`fit_delta` walks a ladder down from the body's own extent, each rung
priced at `TRIANGLE_BUDGET / PROBE_FACTOR` by construction, so no
probe out-tessellates the picture it sizes (the ladder's total under
`TRIANGLE_BUDGET / 4`). The probe is flat in the requested δ once the
budget binds (`tube_ring` at 1e-6: 15.6 s → 149 ms); unbound
documents pay 12–40 % more tessellation than one probe (+10 ms on the
tour die). The 56-row δ-identity table was cut against main first: 49
rows bit-identical, 7 budget-bound rows moved by −0.4…+4.2 % because
main's answer depended on the REQUEST (one body, two requests, two δ)
and the ladder finds the fixed point — a row asserting request-
independence fails on main. Filed (VIEW): the budget's predicted
count is not one-sided (main draws 1 002 536 against a 10⁶ budget).
The item's own table predates PERF-1's torus sizing (tori are ~26×
cheaper now); the defect needed a request a decade finer to bite, so
the measurement adds a 1e-6 column. Single review dispatched (opus).
