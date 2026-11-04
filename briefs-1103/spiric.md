# Your unit: `splice/spiric-on-cap-plane`

Item: `work/splice/a-section-spiric-on-a-cap-plane-reads-unread-against-the-wall-it-ends-on.md` (P1, cost M). Branch `splice/spiric-on-cap-plane`, from main at `7072c784bfd4974c7c307ca569e848e50cb9307b`.

The tilted rod's β 0.7 union (`h ∪ c`, spins 0 and π/2) is pinned at `"unread rider"` (`EdgeInFace { CarrierUnread }`) in `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs` (`OTHER_SPINS_STOPS`).
- The edge is a section chord the join minted: a spiric (torus wall ∩ the rod's cap plane), lying in the cap's plane.
- The face is the rod's cylinder wall, which holds both the chord's ends on the rim circle.
- `slit::carrier_on_surface`'s `(Spiric | Nurbs, non-spline surface)` arm certifies only off a plane (`clears_plane`), so a spiric against a cylinder has no reading.

The item proposes a reading of two curves in one plane. The spiric lies in its plane P. The cylinder meets P along a conic. So the spiric lies on the wall only where it runs along that conic, and `geom_brep::curve_pair::meet` can read the pair. Where the two meet at isolated points only, the carrier is `Placed::Off`. The same shape covers a cone or sphere in P (a conic or circle) and a torus in P (circles).

**Phase 0 is required, and you wait for my reply before building:**
1. Measure the rod's stop on main at three ε, by identity: the edge, the face and the payload.
2. Check the premise of the plan:
   - Is the spiric exactly planar, with its plane known structurally (the cap face's plane, not fitted)?
   - Does `curve_pair::meet` take a spiric against a circle or ellipse in one plane, and with what verdicts: isolated, overlap, refused?
   - What does "Off" need at the shared ends? The chord ends ON the wall, on the rim, so isolated contact at the ends must still read Off over the open arc.
   - Is a meet that is tangent at an end (the chord leaving the rim tangent to the wall's conic) read as isolated or refused?
3. Relation to PR 6481's `spline_off_carrier` (surface × surface, one-way, gated on an allow-list of carriers whose residual is the whole variety) and to PR 6437's `Placed::{On, Off, Unread}`. ONECOPY's `the-spline-polynomial-identity-certificate-has-two-homes` already notes that the certificate has two homes; do not add a third. Say whether your reading is a different certificate (a planar meet, not an identity) or should live in one of those homes.
4. Propose: which surfaces get the planar-section reading (each must meet P in a curve `meet` can read), and how the reading stays one-way (it never says On without a structural or certified reason).

Witnesses:
- the β 0.7 union at both spins moves past `"unread rider"`; re-pin `OTHER_SPINS_STOPS` by identity at its next stop;
- a kernel row where a spiric that does run along the rim conic still reads as lying on the wall (On, or unread, never Off);
- a negative: a spiric crossing the conic (not just touching it at an end) must not read Off over a piece where it lies on the wall.

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
- `splice/teapot-1e6-floor`: the interior walk `reduce::spline_interior` / `NurbsLane::chart_meetings`. It is not coupled to your reading.
Shared ground: `slit.rs` and `curve_pair` are claimed by many programs (CPAIR, ONECOPY, HELDRECORDS, ...): run `work.py territory` and announce the seam in your PR.
