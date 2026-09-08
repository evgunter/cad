---
id: k-report-baseline-fold-cert1-roster
kind: issue
title: k-report: fold the CERT-1 roster changes into the next baseline re-derivation (props_meridian_pole; sphere props_rim_level margins)
status: closed
branch: meter/k-report-cert1-fold
opened: 2026-08-29
closed: 2026-09-08
github: 1251
refs: [1220]
track: K
---

## From GitHub issue 1251

Opened 2026-08-29; 0 comments.

**Scheduling register for PR 1220's K-telemetry consequences** (S-CERT CERT-1), so the roster change has a place that executes instead of an "if the census flags it" hedge.

What moved, for the next K-REPORT runbook pass:

- **New recorded name `props_meridian_pole`** (`props/curved.rs`, `sphere_meridian_span_levels`): two samples per sphere meridian arc; margin = signed chord from the pole's span-relative direction to the nearer span end, × R. Its indeterminate outcome **folds rather than refusing** (the decide still records; PR 1220's body carries the continuity argument), so its in-band population is expected and benign — the baseline should not read in-band samples on this name as a landing.
- **Sphere `props_rim_level` / `props_rim_level_group` margins re-shaped**: the axial `|Δ sin v|·R` became the direction chord `2·sin(Δv/2)·R` (larger wherever rims are distinct), and rims sitting at their own extreme now record a rounding-scale second-component residual instead of bitwise 0 — a new near-zero cluster in those populations.
- `rim_dim_scale_twins.rs`'s sphere twin now pins the chord and the two-population shape (nothing in the ambiguity band).

Per the K-REPORT runbook this is the re-derive-the-baseline case, not a geometry change; the sampled k-lint axis had not drawn a fresh row between PR 1220's merge base and its head, so the first draw lands whenever the schedule next picks it up.

Refs: PR 1220, `docs/K-REPORT.md`, `docs/predicate-dimension-audit.md` (the `props_meridian_pole` row and retired note N7).

## Home

`work/cert/` — the roster that moved is `props/curved.rs`'s, inside S-CERT's `crates/geom-brep/src/props/*` territory, and the change is CERT-1's own consequence.

## Re-homed (2026-09-06)

Moved from `work/cert/` to `work/code-quality/` on S-CERT's exit walk PR
(#1924, its handoffs ledger; merged by Ev 2026-09-06 = ratified), before
`work/cert/` was deleted at sweep 7 of `docs/DOC-LEDGER.md`. Id, body
and header are unchanged; the directory is the claim (`work/README.md`).
The `## Home` section above naming `work/cert/` is superseded by this
line and is kept as the record of why the file was filed there.

## Claimed by METER (2026-09-06)

Moved from `work/code-quality/` to `work/meter/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, `track:` letter and body unchanged. Track K; `docs/K-REPORT.md`'s runbook is METER's territory; the `props/curved.rs` names are read only.

## Closed (2026-09-08)

**Re-derived at `c39a904e` — `origin/main`, not the branch's merge
base.** The measurement was first taken at `ada6bba9` and had to be
re-taken: MESH-12 rewrote `crates/geom-brep/src/props/curved.rs`
between that commit and main (129 insertions, 74 deletions), adding
`require_meridian_span_within_period` — a door that decides a meridian
span *before any margin is formed* and can refuse it — and two recorded
names with it. A sweep is accurate as of the tip it was taken at, and
the tip that matters is the PR's base.

`scripts/k_probe_sweep.sh` reproduces the `k-lint (gate)` configuration
with no substitution (that job carries no `env:` block, so it is
cargo's plain dev/test default, and `rust-toolchain.toml` pins the
compiler); 603 s wall from a cold target directory.

**The measurement, in `docs/K-REPORT.md`'s M11 addendum
(2026-09-08).** Three ε rows, 2 076 056 / 080 / 104 samples, **0
flags** at every rule; committed `m7-eps-*.csv.gz` re-lint clean at the
same tip.

- `props_meridian_pole`: 7 940 samples per row, identical split at all
  three — 7 552 `zero` (largest |m| 8.16278e-17 m, 862 bitwise 0) and
  388 `negative` (smallest |m| 5.47723e-2 m, 1 370× the baseline
  floor). **Zero `indeterminate`.** The in-band population the item
  warned would be benign is EMPTY in the gated corpus — measured, not
  read off a green — and it is empty on *both* sides of MESH-12, so the
  new span door filters nothing here.
- Two per-shape breakdowns, because they are different populations:
  all 7 940 are `die_composed_tour` 7 272 / `die_tool` 432 /
  `demo/lily` 72 / `die_composed` 64 / `die_pips` 32 / `die_fillet` 32
  / `demo/vase` 24 / `demo/budrim` 12; the **388 definite** are
  184 / 72 / 48 / 24 / 24 / 16 / 12 / 8 over the same eight shapes in a
  different order.
- The reading the item exists for is recorded anyway, because a future
  landing is what it is for: the fold arm means an in-band sample on
  this name is **not** a landing, and the M7 addendum's dimension
  caveat now has a sibling — check the deciding site's DISPOSITION too.
  Rule 1 keeps gating the name; a name-shaped exemption would be a
  threshold adjusted to restore a number.
- Rim margins: `props_rim_level` 790 (all `zero`; 492 bitwise 0, 298
  rounding residuals, largest 1.24127e-15 m), `props_rim_level_group`
  306 (262 `positive`, smallest 1.90693e-2 m; 44 `zero`, all bitwise
  0). The near-zero cluster is the re-shaping's expected consequence
  and lands only on the sphere- and torus-bearing shapes.
- The roster is **281**, not the 279 the first pass reported: the two
  names MESH-12 added (`props_meridian_span_forward`,
  `props_meridian_span_winding`, 3 970 samples each per row, wholly
  `positive`) are the difference. Their 7 940 samples equal the row
  growth between the two tips exactly, though that is an identity of
  totals, not a per-name diff.

**No new era cut, and that is the measured answer rather than a
default.** Both calibration witnesses are pointwise identical to M7's
at all three rows (`volume_backstop` 4.79652e-5;
`props_quad_converged` 164.674·ε at 1e-9), the zero side's ceiling is
identical too (`pm_census_ee_span`, 5.32907e-15), and the gap is not
filled — the only ambient-band definite margins under 1e-3 anywhere in
the sweep are `volume_backstop`'s and the ε-coupled family's.
`docs/k-report-data/README.md`'s rule 1 cuts a new file when the
DISTRIBUTION moves. **A roster growth is unlisted there rather than
excluded** (its one explicit non-trigger is a rename), so it is decided
by measuring the witnesses, which is what was done; rule 1 now names
the three witnesses so it is the single home for that decision.

**Residue, each with its own file** (`work/README.md`):

- `work/meter/k-lint-rule-1-prose-assumes-every-in-band-site-refuses.md`
  — rule 1's prose in `tools/k-lint/src/lib.rs` asserts that a recorded
  `indeterminate` means the kernel refused typed, which the folding
  site falsifies. Not fixed here: `tools/k-lint/*` was held by unit 3's
  fix pass.
- `work/meter/k-report-era-witnesses-have-no-guard.md` — the era claim
  rests on three `f64`s in the committed `.gz` and nothing asserts
  them, though `threshold_provenance.rs` computes with the claim on
  every gate run. Carries the values and the one-line extraction, plus
  two stale rows of the same class (`tools/k-lint/src/lib.rs:65`'s
  "231 at today's main", now 281, and
  `work/code-quality/measurements-have-no-mechanical-guard.md:467`).
- `work/meter/k-lint-gate-described-as-diffing-the-committed-baselines.md`
  — `.github/workflows/ci.yml:2467` and `docs/GENERICS-BUILD-COST.md:389`
  both say the gate diffs the fresh sweep against `docs/k-report-data/`;
  nothing is diffed, and two other sites in K-REPORT say so outright.
- `work/meter/cert1-notes-pr-body-tracked-on-main.md` — a CERT-1 lane's
  PR-body draft is committed on main as `.cert1-notes/pr-body.md`.

**One class fixed in place, in `docs/K-REPORT.md`.** Four sites in that
one document carried a census count phrased as though it were live —
231, 231, 232 and 279 at once, in three subsections. Each is now dated
to the change it records, and a standing note says every count in the
report is a dated figure. The same phrasing at
`tools/k-lint/src/lib.rs:65` is the class's fifth instance and is
filed, not edited.

**One frozen label corrected.** The D15/M2 block's *"the harness runs
in no CI row (D17)"* has been false since D17 closed on 2026-08-20:
`k_report.rs` is dumped by `k_probe_sweep.sh` on every code-tier run,
in `k-lint (gate)`'s `dev-probe` leg, and the same document says so
170 lines further down. The dated-FIGURE half of that label is sound;
the CI-coverage half rotted like any other claim, and "labels read
first" did not separate them.
