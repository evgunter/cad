---
id: k-report-era-witnesses-have-no-guard
kind: issue
title: k-report: the M7-era claim rests on three witnesses in the committed .gz and nothing asserts them
status: open
opened: 2026-09-08
refs: [k-report-baseline-fold-cert1-roster, 651, 2140]
---

Found by unit 7's style review, and it is the review's strongest point:
the unit argued at length that a **name-shaped k-lint exemption** would
be the wrong repair (it would be) and never considered the guard that
would have been right.

## The claim that is unguarded

`docs/K-REPORT.md`'s M11 addendum and `docs/k-report-data/README.md`
both say M7 is still the era the shipped thresholds were cut from, and
`tools/k-lint/tests/threshold_provenance.rs` **computes with that**: it
re-derives `EPS_COUPLED_FLOOR_RATIO`, `AMBIENT_BAND_MIN` and
`PROXIMITY_FACTOR`'s consequence-interval against `M7` on every
`k-lint (gate)` run. If M7 ever stopped being the right era, that test
would go on re-deriving against the wrong one and stay green — which is
the one shape #651 says a measurement-backed claim must not have.

Nothing covers it today. `k-lint` lints a fresh sweep and never
compares it to the committed files (`docs/k-report-data/README.md`
rule 2), and no row asserts that the era's witnesses are still what
K-REPORT says they are.

## The guard, which costs almost nothing

The era decision rests on three `f64`s in the committed `.gz`, and
`threshold_provenance.rs` already opens exactly those files through
`gzip -dc` for a different derivation. Measured at `c39a904e`
(2026-09-08), identical in `m7-eps-*.csv.gz` and in a fresh sweep:

| quantity | witness | value |
| --- | --- | --: |
| definite-side floor, excluding the ε-coupled family | `volume_backstop` | 4.79652e-5 |
| ε-coupled ratio's P0 at the binding row (1e-9) | `props_quad_converged` | 164.674 |
| zero-side ceiling inside the ambient band | `pm_census_ee_span` | 5.32907e-15 |

(`volume_backstop` and `pm_census_ee_span` are row-invariant; the
coupled ratio is 839.524 / 164.674 / 335.953 at 1e-6 / 1e-9 / 1e-12.)

One pass extracts all three:

```sh
gzip -dc docs/k-report-data/m7-eps-1e-9.csv.gz | awk -F, '
  NR>1 { m = ($3+0 < 0 ? -($3+0) : $3+0); bz = $4+0 }
  bz >= 1e-13 && $2 != "props_quad_converged" && ($6=="positive" || $6=="negative") \
    { if (fl == "" || m < fl) { fl = m; fn = $2 } }
  $2 == "props_quad_converged" { r = m/bz; if (qc == "" || r < qc) qc = r }
  bz >= 1e-13 && $6 == "zero" { if (m > ce) { ce = m; cn = $2 } }
  END { printf "%.6g (%s)  %.6g  %.6g (%s)\n", fl, fn, qc, ce, cn }'
```

**`$4+0` is load-bearing.** `mawk` compares the string `5e-324` as
`>= 1e-13` unless it is coerced, which silently pulls the tie-break
family (`canonical_order_x`, `split_join_order_u`, both recording
`band_zero = 5e-324`) into the ambient population and returns
5.55112e-17 as the floor. That is a wrong answer with no error, and it
is exactly the confusion the prose floor argument exists to prevent.

**What the row should assert**, and why it is not a baseline to
preserve: that the three witnesses of **the era `M7` names** are what
`docs/K-REPORT.md` states. It goes red on a re-cut that moved a witness
without updating the report, and on a report edit that moved a number
without a re-cut. When a later era supersedes M7, `M7` moves in that
test and the expected values move with it in the same PR — the same
contract every other constant there carries.

## Why it is filed rather than landed

`tools/k-lint/*` was outside unit 7's fence (held by METER unit 3's fix
pass), and `docs/` and `work/` contain no home a test can reach:
nothing outside `tools/k-lint` and `scripts/` reads
`docs/k-report-data/` at all. So the values and the extraction are
written down here, and the row belongs beside
`threshold_provenance.rs`.

## Two stale rows this same class has already produced

Neither is unit 7's to edit; both are recorded here so the sweep sees
them.

- **`tools/k-lint/src/lib.rs:65`** — *"233 committed here, 231 at
  today's main"*. A sweep at `c39a904e` carries **281**, so the second
  number is off by 50 and the phrase "today's main" is the defect
  rather than the digit. Same class as the four sites unit 7 dated in
  `docs/K-REPORT.md`.
- **`work/code-quality/measurements-have-no-mechanical-guard.md:467`** —
  *"the roster has already drifted 233 → 231"*, in the register whose
  whole subject is this class. 281 at `c39a904e`. That file belongs to
  `work/code-quality/`, so it is reported here rather than edited.

Refs: `docs/K-REPORT.md` (M11 addendum, 2026-09-08),
`docs/k-report-data/README.md`,
`tools/k-lint/tests/threshold_provenance.rs`.
