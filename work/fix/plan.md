# FIX — kernel and façade doors with the fix written (plan)

**STATUS: OPEN (2026-09-03).** Opened 2026-09-03 from `docs/WORK-TRACKS-2026-09.md` (FIX section), which is this
program's charter until this plan supersedes it. Live state is
`work/fix/log.md`'s tail and the item files beside this plan, never
this file.

Branch prefix (the #396 convention): **`fix/`** — unit branches
`fix/<unit>-<slug>`, orchestrator branch `fix/orchestrator`.
Away-channel tag `(FIX orchestrator)`. A/B ordinal band
**FIX = 1700–1799**, claimed in `docs/MODEL-AB-LOG.md`'s banding
entry in the opening commit, per that entry's rule.

## Charter

The items every program filed and none scheduled because the fix was
too small to cut a unit for: typed declines whose payload is named in
the body, `Display` impls the consumers already need, a one-line
finiteness gate, a rename. Each is E. This program exists so that they
land instead of accreting, and it spans fences by construction — the
rule is one item per PR, the fence named in the PR body, and the
owning program told on the away channel.

## Review posture

Batched style review, no A/B row for a doc/Display/rename unit; the
standard row where a unit changes a kernel answer
(`transform-rigid-refuses-described-nurbs`, the census declines).

## Unit order

Order is free; the grouping is by file so one PR can carry two items
that touch the same match.

- `transform-rigid-refuses-described-nurbs` — refuse on
  `is_placeholder()`, map control points under the rigid map (weights
  and knots unchanged); the `NurbsSurface`/`NurbsCurve3` point-map
  helper is Track N / S-CERT ground; remove the tour's workaround.
- `census-decline-consults-one-face-of-pair` and
  `interior-witness-budget-decline-untyped` — one sitting, the same
  exhaustive matches; the topo half SMELL-UV routed to S-BOOL is taken
  back here explicitly; the only decision (#855's case 2) is settled
  in the PR body.
- `tier-3-prime-findings-render-through-debug` — `Display` for
  `CensusContact` and `StaleDeclaration`; `census::witness()` renders
  coordinates; flip the two pins.
- `subject-body-drops-the-declared-contacts` — `checks::subject_body`
  returns the contact records; `py/checks.rs` calls `Body::declared`.
- `unit-admits-non-finite-direction-norm` — one line plus a red-first
  row (a linear pattern with a 1e200 direction mints coincident
  instances).
- `mate-contradiction-names-one-mate-twice` and
  `pin-mismatch-recourse-emitted-twice` — Display-only; the demo pin
  and the Python pin flip in the same change.
- `error-types-with-no-display-class` — `Display`+`Error` on the flagged
  types across viewer, editor-core, topo, mesh and quantity; delete the
  consumer `{:?}` renderings.
- `no-parametric-loop-constructor` — `LoopProgram::polygon_expr`, the
  literal `polygon` delegating to it.
- `coherence-findings-have-no-consumer` — `CheckId::ChartCoherence`
  reading `examine_chart_coherence`; the step-import diagnostics half
  is EXCH's.
- `unify-discipline-machinery-onto-registry` — step 1 only (the
  finding/menu sink through the registry); step 2 waits on the
  parameter-coincidence unit per DS8 and is DOCM's to schedule.
- `split-crossings-skip-pattern-mate-ends` — `is_mate_edge_end` learns
  `Pattern`+`Instance(i)` heads; take the ASM-XSPLIT view alongside.
- `mate-clocking-has-no-gui-path` half (1) — refuse a nonzero clocking
  rider on `FrameCoincidence` at `AddMate`; half (2) (a rotate-mate
  affordance or documented roll conventions) is DOCM's question.
- `nested-pattern-mate-heads-refuse` — the one ruling: nested heads
  compose associatively, or the single-level fence gets its sentence
  in the A11 rider; a small PR either way.
- `boolean-error-has-no-fieldless-kind` — `BooleanErrorKind` on the
  `PathErrorKind` precedent; the checks door carries the kind beside
  the prose.
- `band-linear-spelling-not-swept` — `Band::linear(tol)` at the ~20
  inlined sites, or a per-site reason.

## Exit shape

The slate empties; the walk convention applies. New one-PR findings
filed on unowned ground may be homed here while the program is open.
