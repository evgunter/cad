# Your unit: `splice/spline-region-charge`

Item: `work/splice/spline-region-decides-off-the-carrier-on-an-uncharged-f64-box-floor.md` (P1, cost M). Branch `splice/spline-region-charge`, from main at `2c73aabf29acf5cf83e11abf4cf4293bfa176d2a`.

`topo::boolean::spline_region::read_capped` decides a point **off the carrier** when every Bézier cell's floor decides past the band (`decide(CARRIER, Margin::of(cell.hull_gap(q)), band)`). At `f64`, `Patch::hull_gap` reads a control-point box built from points cut by `refine_knots` and de Casteljau (`Patch::split`), and nothing charges their rounding. PCERT's PR 6498 measured the same uncharged box floor giving false Offs at band 1e-12, on patches 1e2–1e5 across or placed up to 1e5 away: up to 5.8e-12 past the true distance, 913 of 4000 corner samples in its probe. A false Off places a point outside a face it lies on, a wrong placement, not a refusal.

**Phase 0 is required, and you wait for my reply before building:**
1. Reproduce on main: a spline face at the scales PCERT measured, at 1e-12, where `read_capped` reads Off for a point that lies on the sheet (within the band). Give the count and the worst excess, and say whether any boolean or demo reaches it today.
2. Read how `geom_brep::pcurve_cache::spline_incidence` charges the same rounding:
   - its per-point first-order bound through every blend (`blend`, `point_rounding`);
   - `rounding_charge` of the reach.
   Say whether that charge can be reused as is (one home) or must be copied. The item's better fix is "read both walks through one home" (`work/splice/the-spline-chart-cell-walk-is-written-twice.md`, P3). Propose whether to do that here or charge in place and leave the P3 row.
3. Propose the fix. The charge must widen the floor, never narrow it, so the reading stays one-way: a point is Off only past the band plus the charge. Name every other reader of `hull_gap` or of the cut boxes that decides a side or an Off: sweep for it, and say which of them share the defect.

Witnesses: a kernel row red on main at 1e-12 (a point on the sheet reading Off), green after, with a mutant (drop the charge) that turns it red; scale the probes with the band. Run the touched crates' rows at three ε.

Review tier: a single FULL review once your PR is green. It may rise to DUAL if Phase 0 shows the fix moves a shared geom-brep home.

You are an IMPLEMENTER lane of the SPLICE program in evgunter/cad (a B-rep CAD kernel in Rust), dispatched by the SPLICE orchestrator. You run in your own cloud session; the orchestrator reads your transcript and your PR, and may message you.

## Read first, in full, by path
- `CLAUDE.md`
- `docs/prompts/implementer-discipline.md` — binding on you.
- `work/README.md`, "Rules" ("Filing and ownership"), for filing what you find.
- Your item file(s) under `work/splice/` (named below), and `work/splice/program.md`.
- `docs/DESIGN.md` and the crate README design pages for the ground you touch; do not re-litigate ratified clauses.

## Branch and git
- Work on the branch named below, created from main at `2c73aabf29acf5cf83e11abf4cf4293bfa176d2a` (or later main). You have explicit permission to push to it. Push after every coherent piece of work.
- Merge-only: never rebase, squash or force-push. Merge `origin/main` whenever it moves and immediately before you open the PR.
- No model identifiers in commits, PR titles or bodies, or code comments. No email address other than `evgunter@gmail.com` anywhere on GitHub.
- PR title starts `splice: `. The PR body documents the logic of the change (it is the record), and ends with the attribution footer from your session's instructions.

## Phase 0, before you build
A lane measures its row's repro on current main before it builds. Report the measurement in your PR body. If the premise of the row is wrong, or the fix needs a design choice beyond what the row and the ratified text say, write your Phase 0 finding as a comment-free DRAFT PR body (or a commit of notes on the branch), say plainly "PHASE 0: waiting for the orchestrator", and end your turn: the orchestrator reads your transcript and replies by message. Where your unit section below asks for a Phase 0 report, do the same even if you think the answer is clear.

## Foreground rule (both halves)
- Never arm waiters, monitors or background chains for your own builds or tests, and never end a turn with background work running.
- Launch any job that outlives a 600 s foreground call `setsid`-detached, writing to a file, record its PID, and poll it in the foreground (an until-loop within one call).
- Hosted CI: poll the run's jobs in the foreground until it concludes; do not end your turn "waiting for CI".

## Verification (lessons from this program's earlier sittings)
- Hosted CI is the verification of record; `gate ok` is the check to read. The PR gate runs no slow set at the ε rows, so before reporting green run the FULL slow set of every crate you touch among geom-brep, topo and sweep (`cargo nextest run -p <crate>`, default ε), and the 1e-6 and 1e-12 rows of the rows you add or edit (`CAD_TOLERANCE_EPS=1e-6` / `1e-12`). If a demo (`demos/tour`) is your witness, run its three ε rows too.
- Probes and fixtures scale with ε: a fixed 1e-6 m offset sits inside the band at ε 1e-6.
- Pin a demo stop or a refusal by identity (the edge's endpoints, the faces by identity, the exact refusal variant and payload), never by a loose predicate: four times last sitting a loose pin let a stop move unseen.
- A demo-only guard on a per-PR change is no per-PR guard: every new door gets a kernel row that goes red when the door regresses.
- The crease class (on curves AND surfaces) and feet on another sheet of a spline face are recurring defects here: say how your change handles each.
- A claim that something is "a composition of existing operators" needs a whole-arena equivalence row.

## When you are done
1. Open the PR (ready for review, not draft), confirm CI jobs are actually RUNNING (read the workflow runs list), poll to conclusion, fix reds you caused. A red inherited from main is reproduced on main's own tree and named in the PR body; do not absorb it.
2. Do NOT close your item or edit `work/splice/log.md`: the orchestrator records state last, after review. DO file what you find outside your fence (discipline §6) as new item files, in the PR that found them.
3. Then stop. Reviews are dispatched by the orchestrator against a frozen head; a fix pass comes back to you by message.
4. Final report, ≤150 lines, as your last message: PR number and head SHA; what changed and why; Phase 0 measurement; slow sets and ε rows run, with results; the sweep's hit list with dispositions and blind spots (§5); rows filed and where; where any demo witness stops now (exact refusal and site).

## Other SPLICE lanes live now
None of SPLICE's are building. PCERT (`pcert/spline-chart-incidence`, PR 6498) wrote the charge in `spline_incidence` and filed this row. Read its PR and say how your change relates to it. Shared ground: `spline_region.rs` is SPLICE's; `geom_brep::pcurve_cache` is PCERT's: run `work.py territory` and announce the seam in your PR.
