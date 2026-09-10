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
