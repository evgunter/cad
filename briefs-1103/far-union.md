# Your unit: `splice/far-union-volume`

Item: `work/splice/main-red-the-lofted-cylinders-far-union-builds-the-wrong-volume-at-1e-6.md` (P1, cost M). Branch `splice/far-union-volume`.

`sweep` `spline_spiric_operands::no_spline_or_spiric_operand_builds_wrong` at `CAD_TOLERANCE_EPS=1e-6` builds the lofted cylinder's far union `O ∪ B` at V = 7.283217721708709 against the closed form 7.283185307179586 (3.24e-5 high, band 1e-6). It is pinned red (`test_utils::pinned_red::row`). A built body with the wrong volume is a wrong answer.

What closing it takes: find which face of the far union carries the excess volume, and whether the BUILD is wrong (fix in the kernel) or the VOLUME READING is (fix the reading, and say why the reading was wrong). Then take the pin out and re-measure what runs after `O ∪ B` in the row (each pin stops its row at main's first stop; whatever follows is unmeasured) — pin or file any new stop by identity. The 1e-12 sibling, `the-lofted-cylinder-operand-runs-out-of-quadrature-budget-at-1e-12` (P2), is not yours unless your cause is its cause; say which.

Phase 0: measure on current main first (is it still red; the per-face volume contributions; the default-ε and 1e-12 behaviour). If the fix needs a design choice the row does not settle, report and wait. A wrong volume is a class until shown otherwise: once you know the cause, sweep for its siblings.

Review tier: a single FULL review after your PR is green.
You are an IMPLEMENTER lane of the SPLICE program in evgunter/cad (a B-rep CAD kernel in Rust), dispatched by the SPLICE orchestrator. You run in your own cloud session; the orchestrator reads your transcript and your PR, and may message you.

## Read first, in full, by path
- `CLAUDE.md`
- `docs/prompts/implementer-discipline.md` — binding on you.
- `work/README.md`, "Rules" ("Filing and ownership"), for filing what you find.
- Your item file(s) under `work/splice/` (named below), and `work/splice/program.md`.
- `docs/DESIGN.md` and the crate README design pages for the ground you touch; do not re-litigate ratified clauses.

## Branch and git
- Work on the branch named below, created from main at `d1ac6881d459081ca86718c0c8aa8acb7dcb81a3` (or later main). You have explicit permission to push to it. Push after every coherent piece of work.
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

## Sibling SPLICE lanes running at the same time (be aware where coupled; say so in your PR)
- `splice/far-union-volume`: the lofted cylinders' far union at 1e-6 builds a wrong volume (`crates/sweep/tests/spline_spiric_operands.rs`); cause unknown.
- `splice/spline-join`: `topo::boolean::edge_join` — a kept edge run on along a spline carrier.
- `splice/split-parameter`: `topo::boolean::reduce`'s point split on a spline boundary edge, carrying the meeting's parameter from `geom_brep::curve_face::boundary`.
- `splice/held-pierce`: `topo::boolean::vtxfac::held_reads` / `faces_holding_pierce` on a spline wall; pcurve rows of split edges mid-boolean.
- `splice/crease-window-bound`: a creased spline's second-order bound (`geom::NurbsCurve3::accel_upper_bound`, `boolean::carrier_touch`, `curve_pair::walk`).
- `splice/kind-reads`: `slit::carrier_on_surface`, `splitting::join::certify_section_area`, `splitting::finish::describe_section_boundary`, `reduce::curved_face_arm` with `geom_brep::dihedral_side`, and geom-core's certified endpoint census.
Other programs' open PRs on shared ground: SECTARMS 6412 and 6405, ONEHOME 6371 (a finished-body constructor at every door — expect merge conflicts if it lands).
