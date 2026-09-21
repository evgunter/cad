---
id: tier-rule-says-twenty-one-jobs-and-a-docs-run-shows-twenty-two
kind: issue
title: The register's docs-tier marker says 21 jobs and a docs-only run today reports 22
status: open
opened: 2026-09-20
priority: P4
cost: E
---


Filed 2026-09-20 by a VNEWS lane whose receipt was a docs-tier run.
The rule is VIEW's, in `work/view/plan.md`, and four programs read that
register by reference — so the correction belongs in the file, not in
the PR body that carried it. (That is the register's own rule: *"write
it into the row rather than the PR body, which stops being read at
merge"*.)

## The rule as written

`work/view/plan.md:324-326`:

> Verify a tier by: **21 jobs** (docs-only) or **38-39 with 12
> `test (…)` and 5 `k-lint (gate, …)`** (full code), plus `gate ok`
> success. Never by which summarising job reports green.

## What a docs-only run shows today

PR #2954, run 35550934699, a tracker-only diff (no file under
`crates/`): **22 job rows.** Eighteen `skipping`, four green —
`gate ok`, `change filter`, `CI half parity + gate wiring (every
tier)`, `docs-only ok`. The change filter printed **`TIER=docs`** at
its *classify the change set* step and `read reach: n/a — TIER=docs
builds nothing` at the next.

So the number in the rule is one short. A lane that followed it
literally would read 22 and conclude it was NOT on the docs tier —
which is the opposite of the error the rule was written to prevent.

## Why the fix is not "change 21 to 22"

The rule exists because its predecessor — *`docs-only ok` success means
the docs tier* — was a PROXY that agreed with itself on every run (the
thirteenth entry in the register's proxy table). A hard-coded job count
is a weaker proxy of the same shape: it is correct until the workflow
adds a job, it goes stale silently, and nothing in CI reds when it
does. This row is an instance of that staleness fourteen days after the
rule was written, which is the evidence for the general claim.

**The durable half of the rule is already in it and needs no number**:
`gate ok` success plus the `TIER=` the change filter prints. That line
is printed by the run itself, is not a count, and cannot go stale
against a workflow change because the workflow is what prints it.

So the shape of an answer is: keep *"never by which summarising job
reports green"*, make the printed `TIER=` the primary evidence, and
either drop the job counts or write beside them that they are an
as-of-a-date cross-check with no guard. Whoever takes it re-derives all
three numbers — 21, 38-39, 12, 5 — against runs of each tier, because
the docs one being stale is no evidence about the others either way.

## Home

VIEW's: `work/view/plan.md`. Filed here rather than corrected in place
because the register is the file four successor programs inherit by
reference and a number in it is read as measured.
