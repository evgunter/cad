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

**No A/B row on any unit of this program** (Ev, in-chat, 2026-09-04,
at the orchestrator handoff): the band 1700-1799 stays unclaimed and
`docs/MODEL-AB-LOG.md` is not touched. Every unit gets one style
review (`docs/prompts/reviewer-style-lane.md`). A unit that moves a
kernel ANSWER rather than its rendering gets a second, correctness-
focused reviewer alongside the style lane — the three named at
handoff are `transform-rigid-refuses-described-nurbs`, the two census
declines taken in one sitting, and
`split-crossings-skip-pattern-mate-ends` (a node-map remap, not a
match arm). No unit here carries a full adversarial review; a unit
that would need one is a unit cut wrong, and gets re-cut instead.

**Reviews are LIGHT OR ABSENT from 2026-09-11** (Ev, in-chat, at the
second orchestrator handoff), superseding the style-review sentence
above: the units remaining on this slate are small enough that a
standing review lane costs more than it returns, and the orchestrator
adjudicates from the diff instead. The A/B exemption is unchanged and
still absolute. The three named correctness-review units are all
closed, so nothing on the live slate invokes that clause.

**What the lane owes instead**, because nothing downstream will catch
it: the discipline moves INTO the brief. Every dispatch on this
program now carries the three instructions the log earned the hard
way — *a row citing a clause gets "read the clause, not the row's
summary"*; *a row citing a site list gets "re-derive it at your merge
base"*; and *a unit that changes a RENDERING owes an answer to "does
any existing pin discriminate the old rendering from the new one",
where no is the pin being part of the defect*. A lane running without
a reviewer states every place it was unsure explicitly, in the PR body
and its report, rather than smoothing it over. That is the trade: less
review, more disclosed uncertainty.

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
- `error-types-with-no-display-class` — cut by fence into three, and
  **cuts 2 and 3 are empty**: the item's list was stale. Every type it
  names already carries a `Display`, spelled
  `impl core::fmt::Display for` — which the sweep that produced the
  list could not see, because it grepped `Display for`. `MigrationError`
  does not exist in the tree at all. What was actually owed was the
  other half of "done": the consumer `{:?}` renderings and the comments
  explaining them. Cut 1 (`viewer`, PR 1741) took eight of those, three
  of them not on the item's list. The remainder is ONE small unit, not
  two — `profile::path::Verb` (a sixth crate, reached through
  `viewer::sketch::PreviewError::Transition`, found through the cut-1
  lane's declared macro blind spot) and four `Dimension` debug labels.
  `viewer::frame::Disagreement` is deliberately NOT forwarded and that
  is recorded at the site: `StableName`'s `Display` omits the role path
  on purpose, so forwarding would render two names differing only in
  path identically and erase the disagreement the message exists to
  report.
- `no-parametric-loop-constructor` — `LoopProgram::polygon_expr`, the
  literal `polygon` delegating to it.
- `coherence-findings-have-no-consumer` — `CheckId::ChartCoherence`
  reading `examine_chart_coherence`; the step-import diagnostics half
  is EXCH's.
- `unify-discipline-machinery-onto-registry` — the REFACTOR stays
  held; the **reading pass that resolves the hold is dispatched**
  (2026-09-11). Step 1 (the finding/menu sink through the registry) is
  the one item on the slate whose fix is NOT written in its body: it
  names a seam and an order, not a diff. What is dispatchable is the
  one question DS8 turns on — is there a second real consumer of the
  sink today, given what this program has since landed into that
  vocabulary (PRs 2344, 2354)? Its output is a spec or a `parked`
  header with the trigger named, and nothing else. Step 2 waits on the
  parameter-coincidence unit per DS8 and is DOCM's, recorded as a
  named rider at `work/docm/plan.md:94-96`, so nothing is owed there.
- `split-crossings-skip-pattern-mate-ends` — `is_mate_edge_end` learns
  `Pattern`+`Instance(i)` heads; take the ASM-XSPLIT view alongside.
- `mate-clocking-has-no-gui-path` — **re-homed to DOCM, 2026-09-12,
  both halves.** This program held half (1) (refuse a nonzero clocking
  rider on `FrameCoincidence` at `AddMate`) while DOCM's plan already
  carried half (2), on a fence that has since been claimed twice:
  S-MATE's exit did not free `crates/editor-core/src/mate/*`, it left
  DOCM and MSOLVE both claiming it. An `AddMate` door is the `DocEdit`
  set, which is DOCM's charter; DOCM rules and hands the build back
  here if the ruling is "refuse typed at the door".
  `levered-clash-margins-hide-their-arm` went to MSOLVE in the same
  move, as solve semantics rather than document custody, and because
  MSOLVE's open PR #2116 is on that row's own quantity.
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
