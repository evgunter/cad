---
id: k-report-still-names-ci-yml-for-the-k-lint-row
kind: issue
title: K-REPORT names ci.yml for the k-lint row at five sites; the row moved to nightly.yml at 49d5b2aee
status: open
opened: 2026-09-29
priority: P3
cost: E
---

## What

`49d5b2aee` ("ci: cut the per-PR gate for latency; move the rest to
nightly", 2026-09-28) moved every `k-lint` row out of
`.github/workflows/ci.yml` into `.github/workflows/nightly.yml`.
`docs/K-REPORT.md` still names `ci.yml` as the row's home at five
places:

- the D17 quotation block, "`.github/workflows/ci.yml`'s k-lint job
  says the same";
- "`ci.yml`'s *K-telemetry probe sweep* runs `scripts/k_probe_sweep.sh`
  into `target/k-fresh`";
- "**What a green `k-lint` row does and does not say about this.**
  `ci.yml` lints exactly `target/k-fresh/k-eps-{1e-6,1e-9,1e-12}.csv`";
- the M10-6 E10 row, "(`ci.yml`'s `driver K-telemetry lint`, on the
  `dev-probe`/`all` draw of the `klint_row` axis)" — the axis draw is
  also gone, every row runs;
- the census note, "the ci.yml step captures `PIPESTATUS` on the
  pipeline line".

The companion line in `tools/k-lint/src/main.rs` — "ci.yml's step
carries the recorded justification" for `--gate-rule-1-only` — is
corrected in the PROPS k-lint baseline PR; these five are in instr's
own document and are left for its owner, because two of them sit
inside dated prose that the report's own "standing class" note says to
treat as a reading rather than a live claim, and deciding which of
those to re-word is an editorial call about this report.

## A sibling, filed the same day on another slate

`work/ciw/ci-prose-and-pins-outlive-the-latency-cut.md` is the same
class — prose and pins that outlived `49d5b2aee` — found by the
2026-09-28 tracker sweep and filed on ciw's slate because the paths it
names are ciw's. These five are in instr's document, so they are here;
whoever takes either should read the other. Discipline's "grep that
program's directory first" would not have found it, because a sibling
instance of one class can sit on a different program's slate.

## Home

INSTR — `docs/K-REPORT.md` is instr's territory
(`scripts/work.py territory`).
