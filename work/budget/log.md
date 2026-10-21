# BUDGET — the log

## 2026-09-20 — opened

Cut out of INSTR, which was carrying 47.5 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed INSTR's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

14 rows arrived by `git mv` with their ids, bodies and history
unchanged. INSTR keeps its band 3300-3399; band 9700-9799 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.
- 2026-10-21 — **Seam (NURBS):** PR 5531 merged (`845daf18`, NURBS build C, FULL). The trimmed NURBS lane's CDT runs in the **lattice metric**: one face-wide scale, a power of two nearest the geometric mean of the realized band aspects, applied at `cdt.insert` only (`cdt_point`, an exact multiply). Certificates, chords, creases, `sub_arc_joint` points and emission stay on unscaled UV. `is_malign` is two-sided on the residual over the metric; `SAFE_ASPECT` is unchanged and its docs now say the measured arm is the upper (raw-UV) one. Over the metric the column snap goes dormant: 0 bands across the tour against 2573 on main. That is the whole cause of the moved counts: d9 on 4 of 40 pairs (`loft_prism`, `swept_elbow`), the lily `finding_13` table (`lily_leaf_b` at δ 2e-3, 5374 → 5254) and the tour budget (+1.3%), all re-baselined and named. Lofted creased frustums and cylinders mesh at all 20 review δ (main: 9/20, 4/20, 5/20, 2/20 refused); the 8-arc controls are bit-identical. `Crossings` is now minted in `planar`. It closes `work/tess/lofted-rational-walls-refuse-at-some-deltas` and opens P3s `the-lower-malign-arm-is-unwitnessed` (witness or retire the snap) and `curved-lane-triangulates-in-raw-chart-coordinates`. Three new tess-lint rows are recorded on `work/budget/tess-lint-gate-red-on-main-23-findings`. (NURBS orchestrator)
