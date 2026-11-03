# Your unit: `splice/nurbs-revert-roster`

Item: `work/splice/a-spline-face-has-no-subtract-or-intersect-lane.md` (P0, cost H). Branch `splice/nurbs-revert-roster`, from main at `c89d725d52bc25a245e8c82fdd012bf0063aa003`.

For any op but ∪, the boolean's front door (`topo::boolean::ops`) refuses a pair through `reduce::first_unsupported_pair` with `reduce::revert_arm_exists`, whose roster has no `Nurbs`; the union's `boolean_arm_exists` has it. So every ∖ and ∩ of a loft or sweep body whose spline face may meet the other operand refuses `CurvedPairUnsupported { op: Some(_), site: RevertRoster, kind: Nurbs, .. }` before any of SPLICE's union lanes run. Witnesses: klein wall 4 (`demos/tour/src/klein.rs` `wall4_pinned`), the saddle cut by its tangent plane (∩ and ∖), PARTING's `the-spline-rider-arm-is-pinned-by-no-fixture`.

The precedent is the torus's admission (`torus-onto-the-subtract-and-intersect-roster`, PR 3416): measure ∖ and ∩ over the union's fixtures with the kind admitted, find the classes that return WRONG bodies, gate or fix them, then admit — staged by partner kind if need be.

**Phase 0 is required, and you wait for the orchestrator's reply before admitting anything:**
1. Read why the revert roster exists at all: what ∖ and ∩ need that ∪ does not (material sides, reversal of one operand, the result's orientation), and re-read the docs on `revert_arm_exists` and `BooleanError::CurvedPairUnsupported` (`crates/topo/src/boolean/mod.rs`), which predate SPLICE's crossing lanes. Name what, if anything, a spline face lacks for each.
2. On a scratch branch with `Nurbs` admitted to the revert roster, run ∖ and ∩ (both orders) over every union fixture that builds with a spline operand today (sweep's `spline_spiric_operands`, the lofted cylinder and prism, the wavy loft, the saddle, editor-core's loft rows, the tour's klein wall 4), at all three ε, and tabulate per partner kind: builds right (volume against closed form or inclusion–exclusion), refuses typed (where), or builds WRONG. A wrong body is the finding that decides the staging.
3. Propose the admission: which partner kinds admit now, what gates the rest, and what each wrong class needs. Ask whether the roster should exist as a second list at all, or be derived from the union's (two hand lists of one fact is the parallel-role shape).

Review tier: DUAL (a concurrent pair) after your PR is green — wrong bodies are the risk.
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

## Other SPLICE lanes live now (be aware where coupled; say so in your PR)
- `splice/held-pierce` (PR 6435): one loop lift with a scaffolding policy; the wavy loft and the saddle now stop at `GermFrameUnsupported { Nurbs, Plane }` under ∪.
- `splice/spline-join` (PR 6434): the join's carried far parameter, in `edge_join`.
- `splice/kind-reads` (PR 6437): `slit::carrier_on_surface` gains a certified-off reading for spline carriers (the slit pass every op runs).
- `splice/split-parameter` (PR 6441, merging): the point split carries a held parameter (`carrier_cross::Held`).
Other programs' open PRs on shared ground: SECTARMS 6412 and 6405, ONEHOME 6371. The demo-join stops are tracked in `work/issues/curved-demo-joins-build.md`: update klein wall 4's line there when its stop moves.
