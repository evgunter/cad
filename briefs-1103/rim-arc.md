# Your unit: `splice/rim-arc-on-cylinder`

Item: `work/splice/a-rim-arc-lying-on-a-cylinder-refuses-as-a-curved-pierce.md` (P0, cost M). Branch `splice/rim-arc-on-cylinder`, from main at `27205d771f57a7fa190cdb42a2f970376037e8ab`.

Klein wall 3 (the swept loop ∪ the revolved bulb, `demos/tour/src/klein.rs` `wall3_pinned`) now refuses `CurvedPierceUnsupported { operand: B, face: 4v1, edge: 1v1 }` at the default ε, after FRAME's `turned_onto` start frame moved the loop's seams into the bottle's plane of symmetry. Edge 1v1 is a `Circle` arc of the loop's start-cap rim, from (0.275, 0, 3) to (−0.275, 0, 3). Face 4v1 is the bulb's neck `Cylinder { origin (0,0,3), axis +z, radius 0.275, u_ref +x }`. So the arc lies on the cylinder, and its first vertex is the cylinder's seam point. The loop's cap sits flat on the neck's rim, which is the annular REST mate, so the arc and the neck's top rim are one circle.

The curved pierce door takes no conic lying on the face it is read against ("an undeclared on-carrier edge or conic", `topo::BooleanError::CurvedPierceUnsupported`'s docs).

What the ratified text already says (read it, do not re-litigate it): DESIGN [verdicts] and [coincidence], "A coincidence is a margined verdict … a margin decided Zero glues them", and [coincidence.structural], "a mate is that rung's statement". So an edge whose carrier the kernel's carrier-pair verdict reads as lying on the face (margin Zero) is a coincidence to be read as one, not refused because it is undeclared. Find out what the boolean already does for the analytic cases of this shape: the boolean has a lying-on path (`reduce::lying_on`, `boundary_crossing`), and an edge lying on its partner's face is read there for lines, and perhaps for conics on planes. Ask why a circle on a cylinder misses it.

**Phase 0 is required, and you wait for my reply before building:**
1. Measure klein wall 3 on main at three ε: the refusal, the site that raises it, and why the lying-on path does not take this arc.
2. Read the docs of `CurvedPierceUnsupported` and of the door's "undeclared" clause, and the [coincidence] lint (`unproven-coincidence`). Say whether "undeclared" means a declaration the user owes (a ratified requirement) or a reading the kernel lacks (a gap). Cite the commits that wrote the clause (`git log -S`).
3. Propose the fix: where a conic lying on a curved analytic face (circle on a cylinder, cone, sphere or torus, at its own ring or generator) is read as lying on, and what it does with the arc's endpoint on the cylinder's seam. Also name the sibling cases the same rung should cover.

Witnesses: klein wall 3, plus a kernel row (a disc capped flush on a cylinder's rim, unioned, so the rim arc lies on the cylinder) that goes red on the refusal. Update klein wall 3's line in `work/splice/curved-demo-joins-build.md` when its stop moves, and name the new stop. That row's `blocked_on` for klein wall 3 is mine to edit in the state-sync.

Review tier: a single FULL review after your PR is green (it may rise to DUAL if phase 0 shows a broad new rung).
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

## Other SPLICE lanes live now
- `splice/kind-reads` (PR 6437): `slit::carrier_on_surface`/`holding`, now `Placed::{On, Off, Unread}`. That is the slit pass's reading of an edge lying in a face, and it is COUPLED to your question: read it before you design, and say in your PR how the two relate.
- `splice/nurbs-revert-roster` (PR 6476): admits NURBS to ∖/∩; klein wall 4.
- `splice/held-pierce` (PR 6435, merging): teapot wall 3.
Shared ground: SECT, SECTARMS, CLEAVE, FRAME (its `turned_onto` lane moved this stop).
