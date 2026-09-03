# VIEW — viewer architecture (plan)

**STATUS: OPEN (2026-09-03).** Opened 2026-09-03 from `docs/WORK-TRACKS-2026-09.md` (VIEW section), which is this
program's charter until this plan supersedes it. Live state is
`work/view/log.md`'s tail and the item files beside this plan, never
this file.

Branch prefix (the #396 convention): **`view/`** — unit branches
`view/<unit>-<slug>`, orchestrator branch `view/orchestrator`.
Away-channel tag `(VIEW orchestrator)`. A/B ordinal band
**VIEW = 1900–1999**, claimed in `docs/MODEL-AB-LOG.md`'s banding
entry in the opening commit, per that entry's rule.

**Opens after CHROME's slate.** See §Opening condition.

## Charter

Decide the viewer's shape before more units accrete into a
3,060-line `session.rs` and a 5,520-line `app.rs`. One conversation
gates the rest; the builds after it are mostly E with one hard
concurrency unit.

## Order

1. `viewer-session-god-module-split` — module boundaries, `Refusal`
   delegation discipline, gesture-safety as data, `Option<OpenTool>`
   for the one-of-six-tools invariant; ratified into the viewer README
   in an `[ev]` PR, then an L-size mechanical refactor. Nothing else
   in this program lands in those files before the split does.
2. `pick-priority-filter-vocabulary` — GQ7: a per-kind admission set
   replaces the three-variant `PickKinds`; where filters are offered
   and what the picture shows for an active filter. The trigger is a
   third asymmetric tool (vertex pick).
3. `camera-fold-clears-status-line` — status-line ownership (typed or
   ranked status versus badges); rows in `frame`; `land` stops clearing
   others' messages.
4. `focus-marking-is-per-node-not-per-segment` — the authored-step to
   canonical-segment map door beside the lowering (announce to
   S-BOOL), a focused-slot state, `pick::focus` narrowed through it.
5. The viewer builds of DOCM's layer-3 identity rule and free-move
   answer, when ruled.
6. `pick-index-built-on-ui-thread` — D→H: revise the GUI-3 §5 seam,
   then tessellation and `PickIndex::build` on the `EvalService`
   worker with cancel-and-restart on δ change and a staleness rule.

## Exit shape

The README states the module map and every item above has landed or
been ruled out; the walk convention applies.
