# WALKS — the log

## 2026-09-20 — opened

Cut out of TOPO, which was carrying 132 budget points in one directory
— about four and a half sittings — when Ev ratified the priority and
track-size conventions in chat the same day. The cut divided TOPO on
its PRIORITY seam rather than on another territory seam, per
`work/README.md` "Track size": five successors plus the Euler-operator
remainder TOPO keeps.

9 rows arrived, each by `git mv` with its id, body and history
unchanged. Band 6600-6699 claimed in this commit
(`docs/MODEL-AB-LOG.md`). Nothing dispatched.

## 2026-09-29 — seam: TOPO amended `producer-closing-mint-is-a-convention-with-thirteen-copies`

TOPO's PR 3160 (`half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete`) retires one premise of the convention — `mev`, `mef` and `mekr` no longer leave a complete face half-minted — and appends that to the row. All thirteen calls stay and the row's finding is unchanged. No file of this program's moved.
- 2026-10-11 — From TOPO's second cut: `face-boundary-walks-spell-the-linked-prelude-by-hand` moved here, because the linked prelude is one relation spelled by hand. (`work/topo/log.md`)
- 2026-11-22 — `walks/curve-placeholder`: the curve NURBS placeholder is deleted with its reader arms. KNOT's `nurbs-curve-has-no-net-state-door` moved here by `git mv` and closes with it; `placeholder-question-has-two-spellings-across-topo` keeps its surface half open for Ev.

## 2026-11-22 — sitting opened (WALKS orchestrator)

Track claimed (`status: active`). Ev, in chat: Fable usage is out, so every lane runs on Opus 5.5, and a dual review or design weighing uses two Opus agents. The model-compare draws are therefore not taken this sitting; any reviewed unit's row records the forced arms as a relaxation (protocol rule 8) and is not counted. Ev also flagged that several rows look stale, so the sitting opens with a read-only staleness audit of all 12 rows (four Opus auditors, by theme) before any dispatch.

## 2026-11-22 — staleness audit and re-cut

Four read-only auditors re-read all 12 rows against origin/main 7709b0ea. Outcome, row by row (details appended to each row):
- `loop-walk-is-respelled-across-topo` — CLOSED: fixed by ONECOPY's PR 6460, and a duplicate of `a-faces-loops-…`.
- `a-faces-loops-are-walked-by-hand-in-thirty-four-places` — headline fixed (PRs 6460/6552); re-titled to its surviving half, what a non-Cycle loop means to each reader. M, design.
- `face-boundary-walks-spell-the-linked-prelude-by-hand` — PR 4099 merged 2026-10-06; one site left (boolean/records.rs `incident`). E-bundle.
- `unclaimed-half-edge-read-as-a-minus-half-unowned` — 2 sites, `live::claimed` exists. E-bundle.
- `void-birth-marking-at-insert-void` — DEFERRED at P3. The row records Ev's 2026-08-25 ruling "not to be implemented yet", so `open` at P1 was a board error; the ruling is untouched.
- The other seven are still live in some form, re-priced off legacy `D` with `design: true`: edge-carrier (M), placeholder (M), refusal variants (E), closing mint (M), Euler carriers (M), shell glue (M), loop-in-loop (M).

Decisions taken unilaterally (sequencing/triage, per the orchestration model):
- Seven design forks (F1–F7) weighed at once, each by two Opus designers on identical briefs (`~/.local/share/cad-work/walks/forks/`, not committed). Euler carriers and shell glue go as one fork (F5) because the counts' component relation is the glue relation. These are not design-fork-experiment rows unless one becomes an `[ev]` PR; any that does is recorded with both designers Opus (Fable unavailable), a relaxation of rule 2.
- E-bundle unit (`walks/ebundle`): records.rs `incident` onto the face-boundary doors, the two minus-half reads onto `live::claimed`, and three stale docs (`ring_nesting`, the pcurves.rs closing-mint list, `Solid`'s role doc). Tier: single STYLE review (mechanical, but code in four files). Implementer Opus 5.5 (forced, not drawn).
- The movefac/seqgen glue extraction and extending the pcurve posture guard to value-returning producers are mechanical halves, held for F5 and F4 respectively, because the designs may reshape the reader they would extract.

## 2026-11-22/23 — the seven forks weighed; three to Ev, answered; lanes in flight

Fork outcomes (two Opus designers each; briefs and reports in `~/.local/share/cad-work/walks/forks/`, not committed):
- F1 edge carrier — converged: `EdgeView` beside `FaceView` (PR 6490 precedent); `edge_carrier_ref`/`CarrierAbsence`/`edge_curve_linked` retire; kind is the curve's; `query::edge_carrier_matches` stays the selector. Residual split (does `stored_curve` survive?) ruled by the orchestrator: only as sugar for operator doors on a caller's key. Unit `walks/edge-view` (cloud session), also discharging the clear/tesslie/export/topo "read through stored_curve" rows.
- F2 placeholder — converged → `[ev]` PR 7423; Ev approved the slot-tag enum ("in progress surface") 2026-11-22; merged. The curve placeholder (no producer outside tests) deleted first by `walks/curve-placeholder` (local). Unit `walks/surface-slot` (cloud).
- F3 refusal variants — converged: one variant per meaning (handed description, kept carrier, pcurve split, blend's mint; tolerance out of the certification arm). Unit `walks/refusal-split` (local).
- F4 closing mint — split in round 0 (completion vs re-derive at the gate), converged in round 1 on completion → `[ev]` PR 7420; Ev approved, sentence folded into the at-rest validity row. Unit `walks/finish-completes` (cloud), stacked on 7420.
- F5 Euler/glue — crossed in round 1, converged in round 2 except the door's name → `[ev]` PR 7424; Ev ruled the rename `genus()` → `h()` with no `c`, "presuming that it is correct" (the unit must pin the theorem red-first). Merged. Units `walks/glue-home` (local; movefac + seqgen + shell.rs share one routine; the validator keeps its own walk) and `walks/euler-theorem` (cloud).
- F6 loop-in-loop — converged; orchestrator rulings: `point_in_loop` takes the plane's `Frame`. Unit `walks/loop-in-loop` (cloud).
- F7 lone loops — converged: one meaning per question in live.rs, a region facet, a `CycleLoop` handle. Unit `walks/face-region` (cloud). One designer proposed a binding clause in crates/topo/README.md stating the three meanings; held for a later `[ev]` PR once the unit lands.

E-bundle PR 7418 green; single STYLE review dispatched (Opus). Its implementer kept `run_edges`' typed `Corrupt` instead of `claimed()`'s panic (argued in the PR) and filed `unclaimed-half-edge-reads-the-3532-table-does-not-list` (P3, 27 sites).

Review tiers at dispatch (letter, tier, reason): refusal-split M single FULL (error mapping across many sites); edge-view H DUAL (tree-wide architectural handle, hard to change later); face-region H DUAL (a meaning stated once that many readers take); loop-in-loop M single FULL (check 9 changes a verdict); glue-home M single FULL; euler-theorem M single FULL (a public variant deleted, theorem pin); curve-placeholder S single STYLE; finish-completes H DUAL (the at-rest invariant moves); surface-slot H DUAL (a type change across five crates). Dual = two Opus 5.5 reviewers (Fable usage out). Model-compare draws not taken: every arm is Opus by Ev's instruction, so these rows are recorded with the forced arms as a relaxation and not counted.

Friction: (1) the hosted runner queue held 28 runs at once on 2026-11-22; a tracker-only PR waited ~1 h for its lint job. (2) Every lane and designer stopped on the weekly usage limit at the same moment and resumed after the reset; cloud sessions had to be messaged one by one. (3) `git add -A && git commit` after a conflicted merge staged conflict markers once (caught at the next merge, fixed in a follow-up commit) — the hazard `memories/agent-lane-operations.md` names.
Seam notes owed to other programs' logs: ONECOPY (its parked `not-crossed-means-one-side-at-a-pinch` waits on 6592, which merged with `Alike` dropped; and `walks/face-region` builds on its dispatched `loop-doors-resolve-a-records-loop-key-through-proven-or-typed`).
