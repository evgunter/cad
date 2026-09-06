# METER — the budget and K instruments (plan)

**STATUS: OPEN (2026-09-06).** Opened in the tracker-wide cut of
2026-09-06 (`docs/WORK-TRACKS-2026-09.md`, addendum 2), claiming
code-quality Track K's `tools/*` half. Live state is
`work/meter/log.md`'s tail and the item files beside this plan, never
this file.

Branch prefix (the #396 convention): **`meter/`** — unit branches
`meter/<unit>-<slug>`, orchestrator branch `meter/orchestrator`.
Away-channel tag `(METER orchestrator)`. A/B ordinal band
**METER = 3200–3299**, claimed in `docs/MODEL-AB-LOG.md`'s banding
entry in the opening commit; infra-only units record no row, so the
band is claimed for bookkeeping.

## Charter

The instruments MEASURE and REPORT; they never gate on a millisecond
and never justify a mesh by its own size
(`memories/tessellation-budget.md`, `memories/perf-measurement-lane.md`).
What this program fixes is where an instrument's claim and its
arithmetic have parted: a join on an allocation ordinal that a face
reorder silently breaks, a scan whose resolution is coarser than the
tolerance it reports against, a doc block transcribed from a sweep it
no longer describes, and rosters copied from the kernel's vocabulary
with no pin. The standing rule from `baseline_census.rs`: **a census
has one executable home and every other site points at it.**

## Review posture

The S-TCOST posture: one style review per unit; no A/B rows. A re-cut
of committed budget data is a PROPS coordination, not a lane's call.

## Unit order

1. `D213` + `D214` — one lane in `tess-lint`: the `Row::nurbs` doc and
   `IDENTITY_COLUMNS`' reason corrected; the missing sizing block
   refused at parse rather than read as `Absent`.
2. `tess-budget-doc-finding-block-stale` — the finding block re-derived
   from the committed baseline (or `tess-lint`'s report header cited),
   with which sweep and when.
3. `D203` — the cross-column invariant rule written once and cited from
   both instrument sites.
4. `cut-prefix-three-unpinned-spellings` — the cut-line prefix pinned
   in one direction (the `CHART_TAGS` precedent); the CIW seam drawn
   in the PR.
5. `k-lint-predicate-roster-unpinned` — the `EPS_COUPLED_PREDICATES`
   roster pinned to the kernel's minted names in one direction; the
   PROPS seam drawn in the PR.
6. `D206` — `SPLIT_SCAN_SAMPLES` raised to put the envelope under the
   gate's margin, and the re-cut coordinated with PROPS.
7. `k-report-baseline-fold-cert1-roster` — the next K-REPORT baseline
   re-derivation, folding `props_meridian_pole` and the re-shaped
   sphere rim margins; a runbook pass.
8. `D201` → `tess-lint-face-ordinal-join` → `C15` — the stable
   face-identity question first (what a DURABLE per-face name is,
   reaching `topo/src/entity.rs` and `demos/`; an issue with a fence
   before a lane), then the meter emits it, then the lint joins on it.
   Until then, `Kind::Reordered` is the cheap tripwire the join item
   names, and can land as its own small unit ahead of the rest.

## Exit shape

The eight land (with `D201`'s answer either built or ratified as
not-now), Track K's `tools/*` half is empty; the walk convention
applies.
