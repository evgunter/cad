---
id: whole-file-skips-do-not-check-their-subject
kind: issue
title: the converted whole-file skips still do not check that the files they exempt exist
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

**None of the seven gates checks that the files it exempts still
exist.** Each declares its homes as an array (or, for a single home, a
`HOME_FILE`), builds the filter from it with `gate_record_anchor_any`
and plants every entry in the clean fixture — but nothing asserts the
paths are in the tree:

  * `scripts/gates/bit-identity-consumer.sh` — `NON_CONSUMER_HOMES`, 4
  * `scripts/gates/bit-identity-punning.sh` — `HOME_FILE`, 1
  * `scripts/gates/evalscalar-allowlist.sh` — `SEAM_HOMES`, 2
  * `scripts/gates/interval-square-allowlist.sh` — `ALLOWLISTED_HOMES`, 7
  * `scripts/gates/no-ambient-env.sh` — `ALLOWLISTED_HOMES`, 3
  * `scripts/gates/register-equal-allowlist.sh` — `DEFINITION_HOMES`, 5
    and `CALLER_HOMES`, 3
  * `scripts/gates/witness-not-ambient.sh` — `HOME_FILE`, 1

`grep -c gate_require` returns 1 for each, and each of those is
`gate_require_crate_sources` — which proves the CRATE has sources, not
that the skipped paths are among them.

**It is also the unproved half of the "one list" claim** those arrays
now carry. The fixture->filter direction reds: a home the clean fixture
plants and the filter stops covering fails the clean case. The
filter->fixture direction does not — planting only some of the homes
while keeping the full filter is green on every gate tried — because
nothing reads the filter's list back against the tree. A subject check
is what closes it, in both senses at once.

## Why it is worth a row

`lib.sh` argues it at `gate_exact_skip_subject` and
`gate_require_file`, and this directory already reds on the class
twice: **a skip whose home is renamed or deleted exempts nothing, stays
green, and ratifies whatever lands at that path next.** A rename is
ordinary; the colon-in-a-path the conversion closed is not. That is the
direction with the population.

`signed-zero-one-home.sh` is the shape to copy: `gate_require_file
"$HOME_FILE"` before the scan, with a fixture that removes the home and
asserts the diagnosis. Over a list it is one loop — and it likely
belongs beside `gate_record_anchor_any` in `lib.sh`, since every caller
of that builder wants exactly this check over exactly that argument.
The clean fixture that plants every home (landed) is already what makes
such a check pass in the self-test.
