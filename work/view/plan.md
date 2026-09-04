# VIEW — viewer architecture (plan)

**STATUS: OPEN, and dispatching (2026-09-04).** Opened 2026-09-03 from
`docs/WORK-TRACKS-2026-09.md` (VIEW section), which is this
program's charter until this plan supersedes it. Live state is
`work/view/log.md`'s tail and the item files beside this plan, never
this file.

Branch prefix (the #396 convention): **`view/`** — unit branches
`view/<unit>-<slug>`, orchestrator branch `view/orchestrator`.
Away-channel tag `(VIEW orchestrator)`. The A/B band **1900–1999**
stays claimed and empty: this program runs no duals (Ev, in-chat,
2026-09-04), and `docs/MODEL-AB-LOG.md`'s band table says so. Reviews
are style reviews against `docs/prompts/reviewer-style-lane.md`, with a
second reviewer on correctness where a unit's failure mode is a
confident wrong answer rather than a refusal — the log's opening-for-work
entry names which units those are and argues each.

**The opening condition is met**: CHROME parked the nine items whose
ground is `session.rs` or `app.rs` behind this program's unit 1
(`work/chrome/log.md`, 2026-09-04), so nothing else is competing for
those files.

## Charter

Decide the viewer's shape before more units accrete into a
3,224-line `session.rs` and a 5,696-line `app.rs`. One conversation
gates the rest; the builds after it are mostly E with one hard
concurrency unit.

## Order

1. `viewer-session-god-module-split` — module boundaries, `Refusal`
   delegation discipline, gesture-safety as data, `Option<OpenTool>`
   for the one-of-six-tools invariant; ratified into the viewer README
   in an `[ev]` PR, then an L-size mechanical refactor. Nothing else
   in this program lands in those files before the split does.
2. `pick-priority-filter-vocabulary` — **parked, trigger unmet.** A
   per-kind admission set replaces the three-variant `PickKinds` when a
   third asymmetric tool (vertex pick) exists; none does or is
   scheduled, and README GQ7 ratifies the deferral. Unit 1 says where a
   tool states what it wants, so the parked row names it.
3. `camera-fold-clears-status-line` — status-line ownership (typed or
   ranked status versus badges); rows in `frame`; `land` stops clearing
   others' messages.
4. `focus-marking-is-per-node-not-per-segment` — the authored-step to
   canonical-segment map door beside the lowering (announce to
   S-BOOL), a focused-slot state, `pick::focus` narrowed through it.
5. `layer3-recipenodeid-aliases-across-rewinds` — DI1's build, now
   ruled: a hold carries its id plus its minting entry, descent is
   checked before liveness, and tools clear on history replacement.
   **Parked** on `next-id-has-no-layer3-door`: DI1's walk reads
   `Doc::next_id`, which is `pub(crate)`, and the door is DOCM's.
   The free-move half of this row was never this program's — DI5
   hands that build to CHROME (`no-persistent-setplacement-session-op`),
   and it is parked there behind unit 1.
6. `pick-index-built-on-ui-thread`, in three:
   **6a** the GUI-3 §5 seam revision — an `[ev]` PR extending the
   frame-state inventory and stating the staleness rule, no code;
   **6b** tessellation and `PickIndex::build` onto the `EvalService`
   worker with cancel-and-restart on δ change, `PickCache`'s retry
   policy travelling with it;
   **6c** the staleness rule as values in `frame`, with rows.
   6b and 6c collapse back into one adversarially-reviewed unit if 6a
   rules the staleness rule is not expressible as frame data.

## Exit shape

The README states the module map and every item above has landed or
been ruled out; the walk convention applies.
