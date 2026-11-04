# Your unit: `splice/teapot-1e6-floor`

Item: `work/splice/teapot-wall-3-at-1e-6-floors-the-meridian-against-a-spout-wall.md` (P0, cost M). Branch `splice/teapot-1e6-floor`, from main at `7072c784bfd4974c7c307ca569e848e50cb9307b`.

Teapot wall 3 (the lofted spout ∪ the vessel, `demos/tour/src/teapot.rs` `wall_probes`, `wall3_pinned`) stops at different places by ε:
- **Default ε and 1e-12:** past its seam split (PR 6441) and its held pierce (PR 6435), it stops at `GermFrameUnsupported { a_kind: Sphere, b_kind: Nurbs }`. That is SECTARMS' section and is not yours.
- **1e-6:** it stops one wall earlier, at `SplineCrossingUnsettled { operand: A, edge: 9v1, face: 9v1, face_of: B, why: Floor }`. Edge 9v1 is the belly's quarter meridian. Face 9v1 is one of the spout's bore walls. `Unsettled::Floor` means the interior walk (`reduce::spline_interior` → `geom_brep::NurbsLane::chart_meetings`) reached its depth floor holding a piece whose enclosure is wider than the band.

The goal: at 1e-6, teapot wall 3 reaches the stop the other two rows reach, or a typed refusal that is honest about the geometry, pinned by identity.

**Phase 0 is required, and you wait for my reply before building:**
1. On main, at 1e-6, measure the floored piece:
   - its parameter interval on the meridian, and its distance from the fragment's ends and from any seam the meridian was split on;
   - the piece's enclosure of the distance to the bore wall, against the band at 1e-6.
   - Also measure the true clearance between the meridian and the bore wall near the piece (a dense sample or a local minimisation): the minimum distance and where it falls.
2. Classify the floor as one of:
   - **(i) depth exhaustion on a clear pair:** the true clearance is well outside the band, and the walk ran out of depth because its floor or its subdivision is too coarse at this ε;
   - **(ii) a genuine near-graze:** the true clearance sits between the band and the enclosure width, so no finite walk at this ε can settle it;
   - **(iii) a fixture artefact:** a fixed offset in the teapot's construction that does not scale with ε puts a gap inside 1e-6's band. Check `SPOUT_BORE`, `SPOUT_R0` and the bore wall's chart corner.
   Say why the default and 1e-12 rows do not floor here. Do they not meet this pair at all, or do they settle it?
3. Propose the fix per class:
   - (i) is a walk fix: where the floor is set, and why deepening it cannot loop or over-run. A floor that scales wrongly with ε is the defect class to look for.
   - (ii) is an honest escalation or tangency refusal, not a deeper walk. Say which typed refusal already exists for it.
   - (iii) is a fixture fix that keeps the demo's geometry honest at every ε.
   Name the sibling sites that share the cause.

Witnesses: teapot wall 3 at all three ε, plus a kernel row that goes red on the old stop. Build that row in `geom-brep` or `sweep` from a meridian-like arc against a bore-like NURBS wall at the measured clearance, scaled by ε. Update teapot wall 3's line in `work/splice/curved-demo-joins-build.md` when its 1e-6 stop moves, and name the new stop; that row's `blocked_on` is mine to edit in the state-sync.

Review tier: a single FULL review once your PR is green.

You are an IMPLEMENTER lane of the SPLICE program in evgunter/cad (a B-rep CAD kernel in Rust), dispatched by the SPLICE orchestrator. You run in your own cloud session; the orchestrator reads your transcript and your PR, and may message you.

## Read first, in full, by path
- `CLAUDE.md`
- `docs/prompts/implementer-discipline.md` — binding on you.
- `work/README.md`, "Rules" ("Filing and ownership"), for filing what you find.
- Your item file(s) under `work/splice/` (named below), and `work/splice/program.md`.
- `docs/DESIGN.md` and the crate README design pages for the ground you touch; do not re-litigate ratified clauses.

## Branch and git
- Work on the branch named below, created from main at `7072c784bfd4974c7c307ca569e848e50cb9307b` (or later main). You have explicit permission to push to it. Push after every coherent piece of work.
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
- `splice/spiric-on-cap-plane`: `slit::carrier_on_surface`, the tilted rod. It is not coupled to your walk.
Shared ground: SECTARMS owns teapot wall 3's default-ε stop. HONE, HONEROOT and TANGRIM own parts of `geom_brep` roots and walk ground: run `work.py territory` and announce the seam in your PR.
