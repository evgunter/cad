---
id: eps-klint-and-shard-counts-are-prose
kind: issue
title: the eps rows, k-lint unifications and test shards are counted in prose in eight places
status: open
opened: 2026-09-11
---


The defect `render-hosted-knows-four-lanes-and-there-are-six` closed for
the render lanes — *a row set restated as a count in prose, with nothing
comparing it to the declaration* — is live for three more row sets, and
this is its hit list. It is the blind spot that unit's sweep declared:
that sweep looked for LANES.

**The roster half is already solved, which is what makes the prose half
the whole of the finding.** `scripts/ci-filter.py:1497,1555` declare
`EPS_ROWS` and `KLINT_ROWS`, and its `--selftest` re-derives ci.yml's own
matrix literals against them (and requires the k-lint job's step
conditions to name every row and no others). So the executable side is
held. What is not held is every sentence that counts those rows:

- `local-scripts/ci-local.sh:172` — "both lanes, all three eps, all five
  k-lint unifications".
- `.github/workflows/ci.yml:46,125` — the same counts in the dispatch
  input's prose and in the matrix's own comment.
- `.github/workflows/ci.yml:701,719,723` — the narrowing warning and the
  step summary, which spell "twelve test jobs" and "all five k-lint
  unifications" into text a reader of a narrowed run acts on.
- `scripts/ci-filter.py:234,3525` — prose counts inside the file that
  declares the rosters.
- `docs/prompts/implementer-discipline.md` §2 — "twelve `test (…)` jobs",
  "all five `k-lint (gate, <row>)`", and the five row names listed
  literally. This one is read by every implementer lane at the start of
  every unit, which is the strongest reason to hold it and the strongest
  reason not to hold it with a fragile reader.

**Shape of a fix**, if it earns a unit: the same one this program just
used — a check that reads the declarations (`EPS_ROWS`, `KLINT_ROWS`,
the shard count) and holds the sentences to them, or prose rewritten so
it names the roster rather than counting it ("every row `EPS_ROWS`
declares"). The second is cheaper and is what `render.yml`'s header and
`render-hosted.sh` now do.
