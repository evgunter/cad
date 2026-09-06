---
id: whole-file-skips-do-not-check-their-subject
kind: issue
title: the six converted whole-file skips still do not check that the file they exempt exists
status: open
opened: 2026-09-06
refs: [whole-file-skips-are-hand-spelled-not-anchored]
---

## Finding

`whole-file-skips-are-hand-spelled-not-anchored` closed the SMALLER of
the two halves it measured: every whole-file skip in this directory now
builds its pattern with `gate_record_anchor`, so the `FILE:LINE:` shape
is pinned and the escaping is nobody's job. The other half it measured
is untouched, and it is the half with a live route.

**None of the six gates checks that the file it exempts still exists.**
Each declares its homes as an array (or, for a single home, a
`HOME_FILE`), builds the filter from it and plants it in the clean
fixture — but nothing asserts the path is in the tree:

  * `scripts/gates/bit-identity-consumer.sh` — `NON_CONSUMER_HOMES`, 4
  * `scripts/gates/bit-identity-punning.sh` — `HOME_FILE`, 1
  * `scripts/gates/evalscalar-allowlist.sh` — `SEAM_HOMES`, 2
  * `scripts/gates/interval-square-allowlist.sh` — `ALLOWLISTED_HOMES`, 7
  * `scripts/gates/no-ambient-env.sh` — `ALLOWLISTED_HOMES`, 3
  * `scripts/gates/witness-not-ambient.sh` — `HOME_FILE`, 1

`grep -c gate_require` returns 1 for each, and each of those is
`gate_require_crate_sources` — which proves the CRATE has sources, not
that the skipped path is one of them.

## Why it is worth a row

`lib.sh` argues it at `gate_exact_skip_subject` and
`gate_require_file`, and this directory already reds on the class
twice: **a skip whose home is renamed or deleted exempts nothing, stays
green, and ratifies whatever lands at that path next.** A rename is
ordinary; the colon-in-a-path the conversion closed is not. That is the
direction with the population.

`signed-zero-one-home.sh` is the shape to copy: `gate_require_file
"$HOME_FILE"` before the scan, with a fixture that removes the home and
asserts the diagnosis. Over a list it is one loop, and the clean
fixture that plants every home (landed) is already what makes such a
check pass in the self-test.
