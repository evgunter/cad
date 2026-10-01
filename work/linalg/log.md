# LINALG — the log

## 2026-09-20 — opened

Cut out of PROPS, which was carrying 108.5 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed PROPS's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

8 rows arrived by `git mv` with their ids, bodies and history
unchanged. PROPS keeps its band 2400-2499; band 9400-9499 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)

## 2026-10-01 — picked up; #2468 is landing, not holding

An orchestrator holds the track (status `active`). Ev asked on #2468
"what's up with this?"; the answer, posted there: the hold's cause
cleared 2026-09-22 when DECIDE-3 (#3039) merged into `props/sign-hull`,
and the PR then sat with no owner — this row moved here on 2026-09-20
and LINALG had no orchestrator — while DECIDE used the branch as its
base (SYM-9, DECIDE-4…7 merged into it; DECIDE-8 #3282 open against
it). Last main merge 2026-09-21; 34 files conflict against `main`;
no hosted run since 2026-09-20, so the post-DECIDE-3 green has never
been seen hosted.

Dispatched: a lane merges `main` into `props/sign-hull`, resolves the
34, takes the two seam measurements the merge owes (SYM's
`the-negative-arms-denominator-clause-widens-with-decide-3s-definite-quadratic`;
PATHS's `arc_span` against `turned_span`, `work/decide/log.md`
2026-09-25) and drives hosted CI green. **Review tier: single FULL
review of the merge resolution** — every unit riding the branch was
reviewed at its own merge, and what has not been read by anyone is
the resolution of 34 files against ten days of `main`, where a wrong
side taken compiles fine. Then land; DECIDE-8 re-targets to `main`.

Process finding: a held PR became another program's integration
branch, which turned "unblocked" into a 3 900-commit merge debt with no
owner. Nothing new targets `props/sign-hull`.
## 2026-10-01 — #2468 lands

- **Merge-forward lane** (`linalg-forward`): four rounds. It made two
  merges of `main`, plus #3636 and #3661 (below), and re-measured every
  pin that moved, with the cause in each commit. Seam measurements:
  - PATHS's `arc_span` won, as announced.
  - SYM-12 × `definite_quadratic`: all eight ceilings and seven splits
    are identical. The pad's split was not taken; its shape report
    OOMs on a 15 GB box. The row is closed.
- **Review**: single FULL review of the merge resolutions on
  `f6f387f6b`, MERGEABLE with 0 MAJOR and 0 MINOR. Six claims survived,
  including by plants: the Duff plant restores main's seat digests
  exactly, and a `copysign` plant reds the re-aimed rows. The style
  findings S1, S2, S4, S5, S7, S8 and S9 were fixed. S3 and N2 were
  filed (`work/sym/signed-rs-ring-helpers-…`,
  `work/clear/clearance-diagonal-seam-…`).
- **Ruling (orchestrator)**: SYM-12's gating row is re-aimed rather
  than held. Its claim holds and only the `copysign` mechanism is gone,
  so no reach is lost, unlike the 09-15 hold under #2728. Filed for
  SYM: `the-negative-arm-lost-its-document-consumer` (`design: true`).
- **Two reds that were not this PR's**, each fixed at its own PR and
  ported in:
  - #3636 (REACH): `reach_volume_backstop` was red at 1e-6/1e-12 on
    `main`. Its rows were calibrated at ε = 1e-9; the fix re-spells
    them in ε.
  - #3661 (CIW): `ci-filter.py` blanked SEEDS on TIER=all, so a diff
    touching `.config/nextest.toml` silently skipped its own slow set.
    That is why #2468's slow set never ran hosted.
- Hosted run 36845857746 on `b3d386d10` is green. Its slow-set step
  executed: 133/133 over the seeded crates.
- Process finding: this session cannot dispatch `nightly.yml`
  (403 for the integration), so a branch-ref full run is unavailable
  from a remote box.

## 2026-10-01 — three units land; the slate re-ordered

- **#2468 landed (`dc39bce`).** `props/sign-hull` is retired, and
  nothing targets it any more: DECIDE-8 had already closed at Phase 1
  on Ev's #3283. The landing record is on the item and in the entry
  above.
- **#3687 landed (`ca1ae66`).** It carries the D9 NaN qualifier at
  linalg's home, `Mat3`'s product pinned to literal values, and the
  interval backend's zero signs chosen by rule (lower takes −0, upper
  takes +0).
  - Single FULL review: APPROVE.
  - One MINOR: the guard's `transform_point` arm had no teeth until a
    cancellation case was added. Fixed.
  - The review measured 0 value differences against the old backend
    over a biased random corpus, and 0 debug-vs-release bit
    divergences in the new backend over 5.56M results. The old
    backend had 104.
  - D9's text is unchanged, because NaN code motion does not break
    same-build replay.
- **#3686 landed (`14f6143`).** `torus_meridian_orient` and
  `sector_shape` now take the decided door.
  - Single FULL review: REQUEST CHANGES, with one MAJOR confirmed by
    rendering the refusal. Negating `sector_straight`'s margin to fit
    `decide_positive` inverted the Boolean's "tighten the tolerance"
    advice. `refusal_routes`' table reads that sign, and about 1,500
    committed K rows recorded it.
  - Ruling: the margin keeps its sign. `geom_core::k_stats::decide_negative`
    is added beside `decide_positive`, because a sign other tables
    read is not a door's to flip. The delta review returned APPROVE.
  - Filed from the sweep: on BAND, CHART, CLEAVE, OFFSET, RESTFRONT
    and TESS. On RESTFRONT: topo's crate-funnel bypasses.
- Priced: the svd fold row is P4, cost E. The array doors are cost M,
  because the population is large. The point-order and literal-door
  rows are marked `design: true`. The plan now states the present
  order.

## 2026-10-01 (later) — the pole row, the doors, a designer pair

- **Restart.** The worker restarted mid-wave, and three lane reports
  were lost with the inbox. I recovered them from the pushed branches
  and PRs, and had both designers re-deliver.
  - The doors lane (#3710) left no surviving dispatch record, so I
    treated its PR as a claim to verify. A fresh lane took it over
    and found its red was main's: `bounds_census` was missing a
    roster line that main had since added.
- **#3711 landed (`49da0ea`).** The pole branch pick is a sound hull,
  not a refusal. The measurement is on the closed row.
  - Review: single FULL, APPROVE WITH FIXES. Two MINORs: at `Interval`
    the readers escalate rather than decline; and nothing pinned
    "the Interval window encloses f64's". A new row now pins it, and
    a planted one-branch pick turns it red.
- **#3710 landed (`330e305`).** It adds:
  - the array doors;
  - `norm_inf` (the review caught that the first name, `norm_sup`, is
    `interval::norm_sup`'s, which is a bound on the Euclidean norm —
    the unsafe direction to confuse);
  - escalation recourse with no declare menu, and one
    `DIRECTION_LENGTH_SUBJECT` where four copies stood;
  - svd folds on `Real::min/max`.

  Review: single FULL, APPROVE WITH FIXES. Every retired lowering was
  differentially bit-identical, and there was no MAJOR.
- **Designer pair on geom_core's point surface** (`Point3` order; an
  f64 literal constructor). Blinding byte 165, recorded on
  `analysis/design-fork/linalg-point-surface`.
  - First reports split: A recommended `total_cmp` plus `from_f64`; B
    recommended neither.
  - On reconciliation A moved to B's position, and B held. This was
    not a crossover.
  - Converged: no door. This is the status quo that `linalg.rs`'s
    "Deliberate omissions" already states, so it is not a fork for Ev
    and not a design-fork log row. I added one sentence to that bullet
    (clouds are compared by matching under a tolerance, never by
    sorting and zipping).
  - The #3710 reviewer noted that the sentence is a mild extension of
    the bullet and binds tests (the tint row). That is said to Ev in
    the session summary rather than held.
- Filed from these units:
  - TINT: sort-and-zip clouds.
  - TOPO: a `Body` vertex-point door.
  - CLEAVE: a strut-bearing operand reaching the join.
  - On this slate: the test adoption of the array doors, the Chebyshev
    chains, and Mat4.
- **Friction.** The session's disk allowance (~39G) holds about three
  concurrent building lanes. One lane ran it to zero twice. Reclaim
  each target as its report lands.
- Main's nightly is red on k-lint only (`chart_bound_outer_span` NaN
  and `bool_circle_torus_root_slack`). Those belong to CHART, GERM and
  PROPS, and none of it is a linalg K name.
