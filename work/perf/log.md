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
