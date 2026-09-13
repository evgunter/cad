---
id: k-lint-gate-described-as-diffing-the-committed-baselines
kind: issue
title: two sites say the k-lint gate diffs the fresh sweep against docs/k-report-data/; nothing is diffed
status: open
opened: 2026-09-08
refs: [2140]
---

Found by unit 7 while re-deriving the K baseline. A self-declared
two-site class: the same false premise, written twice, in two
documents neither of which owns the fact.

**The fact.** `docs/k-report-data/README.md` rule 2 states it
outright — *"Nothing reads these files as a gate. CI's `k-lint` runs
`scripts/k_probe_sweep.sh` into a scratch dir and lints THAT; the
committed files supply the thresholds in `tools/k-lint/src/lib.rs` and
are never compared against."* `tools/k-lint/src/lib.rs` says the same
in its own voice: the lint *"lints the fresh rows it was handed and
never compares them to the committed files"*.

**Site 1 — `.github/workflows/ci.yml:2467`**, inside the opt-level
note: *"`k-lint` deliberately does NOT get this — it regenerates the K
sweep CSVs that are **diffed against the committed baselines** in
`docs/k-report-data/`, and that lane stays on the exact configuration
those baselines were produced under."* Nothing is diffed. The
*conclusion* is still right for a different reason (a lane that
re-derives thresholds from those files should run under the
configuration they were cut under), but the reason given is not a
reason.

**Site 2 — `docs/GENERICS-BUILD-COST.md:389`**: *"The consumer is
`scripts/k_probe_sweep.sh` (feeding the live `k-lint` gate **against
the committed baselines** in `docs/k-report-data/`)"*. Same premise,
same error.

**Why it matters more than a wording slip.** This premise is what makes
a stale predicate name in `docs/k-report-data/` look like a gate
failure waiting to happen, which is precisely the fear rule 1 and that
README exist to answer. A reader who believes either site will read a
roster drift as a red risk and reach for a re-cut nobody owes.

**Not fixed here**: `.github/workflows/` and `docs/GENERICS-BUILD-COST.md`
were outside unit 7's fence.

**The sweep, and what it could not match.** Pattern:
`rg -n 'k-lint|k_probe_sweep' -g '!docs/k-report-data/**' -g '!work/**' .`
piped through `rg -i 'baseline|diff|compar'`, run at `c39a904e`. Hits
outside `tools/k-lint`, dispositioned:

- `.github/workflows/ci.yml:2467` — **the defect** (site 1 above).
- `docs/GENERICS-BUILD-COST.md:389` — **the defect** (site 2 above).
- `docs/K-REPORT.md:175-180`, `:678-679` — **correct, and explicitly
  so**: *"Nothing under `docs/k-report-data/` is opened at gate time …
  Its neighbour `tess-lint` DOES diff a committed baseline; k-lint
  deliberately does not."* Two sites already state the true fact, which
  is what makes the two above a contradiction inside the repo rather
  than a gap in it.
- `docs/K-REPORT.md:1163`, `:1228`, `:1242` — *"the k-lint baseline
  floor"*, meaning the constant `BASELINE_FLOOR_MARGIN`, not the files.
  Not this shape.
- `tools/tess-lint/tests/baseline_census.rs:84` — a true statement
  about `threshold_provenance.rs` reading out of its own crate.
- `docs/MODEL-AB-LOG.md` (nine rows) — historical unit log entries;
  none asserts the gate diffs the committed files.

**Blind spot**: a site that describes the behaviour without using the
word *baseline*, *diff* or *compare* — "checked against the committed
rows", "validated against `docs/k-report-data/`" — is unmatched, and so
is any statement inside `work/` (excluded above, since tracker items
about this class are not instances of it).

## Moved to INSTR (2026-09-08)

Moved from `work/meter/` to `work/instr/` by `git mv` when METER's exit
walk opened the successor (`docs/METER-EXIT-WALK.md` §4, ratified by Ev on
2026-09-08 at PR #2212). Id, header and body unchanged; the directory is
the claim. This row is one of the twenty on INSTR's opening slate.

## Refs at METER's sweep (2026-09-09)

METER closed and its item files left the tracker (`docs/DOC-LEDGER.md`, sweep 10); `k-report-baseline-fold-cert1-roster` is now cited by its closing PR 2140 — which this row's `refs:` already carried, so the dying id was dropped rather than substituted into a duplicate.
