# Your unit: `splice/same-operand-rider`

Item: `work/splice/an-unread-rider-related-only-by-its-own-vertices-refuses.md` (P0, cost M). Read it in full: the finding, the conditions under which the exemption is sound (by lineage, not by shared topology), and what it rests on. Branch `splice/same-operand-rider`, from main at the SHA named in your session prompt; PR 6437 (`slit::Placed`) is on main by then.

Four builds that shipped (the lofted cylinder's far `O ∪ B`, `B ∪ O` and around `B ∖ O`, and possibly the tilted rod's β 0.7 union) now refuse, because `slit::holding` refuses an unread carrier wherever anything relates it to a face, and `riders` relates an edge to every face around its own vertices. The way back is a STRUCTURAL exemption, never a sample. An unread edge `e` and a face `f` are exempt only where (1) both cells' split roots (`Body::split_root`) lie in ONE operand's input, (2) their shared vertex is unfused (no vertex-vertex partner), and (3) no record names the pair. The exemption rests on each operand being at rest, which nothing checks today for spline or spiric carriers: there is no validation at the boolean's entry, and tier 3/3′ does not read an edge lying in a face it does not bound for those kinds (`restfront/check-9-meeting-arms-silent-off-a-plane-and-on-ellipse-spiric-nurbs-edges`).

**The exemption and the guarantee ship TOGETHER, or neither ships.**

**Phase 0 is required, and you wait for my reply before building:**
1. Read the four builds on main: is each stop exactly this case (same operand, unfused shared vertex, no record)? Read the tilted rod's stop first, as the row asks.
2. The guarantee: what can establish "this operand is at rest, so its edges meet its faces only along their boundaries" at the point `slit::holding` runs? Candidates are a check at the boolean's entry for spline and spiric carriers (does one exist, and what would it cost?), the operand TYPE carrying the guarantee (`AtRestBody` and ONEHOME's finished-body constructor, PR 6371: what does that type promise today?), or completing RESTFRONT's check-9 arms. Name which one you propose, and whether it is SPLICE's to build or another program's (if another's, the exemption waits on it, and I sequence the two).
3. Lineage: is `split_root` reliable for both edges and faces through every split, join and zip the boolean does before the slit pass? Does a fused vertex always leave a vertex-vertex record?

Rows owed: the four builds build again (or a stated reason why one does not); the notch keel (`a_spline_keel_across_a_notch_refuses_unread`) still refuses; a cross-operand pair that shares a FUSED vertex still refuses; and mutants that drop each of conditions 1–3 must turn a row red.

Review tier: DUAL (a concurrent pair): a structural exemption on the soundness path.
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
- `splice/nurbs-revert-roster` (PR 6476, in dual review): admits NURBS to ∖/∩. Your `B ∖ O` row runs through it.
- `splice/rim-arc-on-cylinder`: `parents_distinct_from`'s NURBS arm (a face beside an edge read as distinct from an analytic face). Related ground in reduce.rs.
Shared ground with RESTFRONT (check-9) and ONEHOME (finished body).
