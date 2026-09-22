---
id: tier-rule-says-twenty-one-jobs-and-a-docs-run-shows-twenty-two
kind: issue
title: The register's docs-tier marker says 21 jobs and a docs-only run today reports 22
status: closed
opened: 2026-09-20
priority: P4
cost: E
closed: 2026-09-21
branch: view/cut-residue
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

## Closed — 2026-09-21, `view/cut-residue`

The rule is rewritten in `work/view/plan.md` around a discriminator
that is not a count, which is what this row asked for.

**What it says now.** *Which tier am I on* is answered by
`python3 scripts/ci-filter.py --base <base>`, which prints
`TIER=docs|closure|all`; that is named as the authoritative answer,
because the workflow classifies the change set with the same script, so
the word cannot go stale against a change to `ci.yml`. The caveat is
written beside it: run it on the **committed** branch, because over an
uncommitted tree it reports `falling back to TIER=all: empty change
set`, which is the tree's answer and not the branch's.

**On the run itself the docs tier is a SHAPE: `gate ok` green with
every code row skipped.** `docs-only ok` is green on both tiers — the
thirteenth proxy in the register above — so it is never the marker; the
job total is not one either, and the rule now says why rather than
quoting a better number.

**The two counts that stayed, and the reason they are not the same
mistake.** Twelve `test (…)` and five `k-lint (gate, …)` each carry an
enumeration rule that `ci.yml` states, so either can be re-derived
instead of believed: twelve is `{default, interval}` x
`{default, 1e-6, 1e-12}` x `shard: [1, 2]` across the two matrix jobs
(`ci.yml`'s `test-default` and `test-interval`, `shard: [1, 2]` over
`needs.filter.outputs.eps_rows`), and five is the literal `klint_rows`
list — `dev-default`, `release-default`, `release-budget`,
`dev-budget`, `dev-probe`. The workflow's own narrowing annotation
states both in one sentence, so a run gating fewer says so out loud.
**The total went.** "38-39" was this rule's other count and had exactly
the defect the 21 had; it is deleted rather than re-measured, which is
this row's *whoever takes it re-derives all three numbers* answered by
removing the two that have no rule and keeping the two that do.

**The un-mergeable-PR signature is spelled out as NOT a tier**: `gate
ok` RED on a diff that touches `crates/`, diagnosed with
`git merge-tree --write-tree origin/main origin/<branch>` and never
from the logs. Three quarters of that signature is also true of a
healthy docs tier, which is why the count could never have separated
them. The register's own rule at the end of the file carries the whole
shape and the tier rule now points at it; the `~22` in that rule is
marked as an observation on one run rather than a marker.

**The sweep for the same stale count, and what it could not match.**
`git grep -n -E '\b21 jobs|21-job|twenty-one jobs|38-39|38 jobs|22 jobs|22-job' origin/main -- '*.md'`
finds the number in four populations. The two in
`work/view/plan.md` are the rule and its narrative and are both
rewritten. The rest are **receipts, not rules**: `work/view/log.md`'s
entries (`:8579`, `:9035`, `:9882`, `:10230` and the `38 jobs` run
notes), `work/ciw/log.md`, `work/census/log.md`, `work/door/D306`,
`work/edit/…`, `docs/MODEL-AB-LOG.md` and `docs/CI-MINUTES-2026-08.md`
each record what a named run showed on a named day, and a log is
append-only narrative — rewriting a receipt would make it a receipt for
a run nobody took. `work/ciw/tree-wide-guards-outside-the-change-closure`
is the same shape inside an item body. **Nothing outside
`work/view/plan.md` states the count as a RULE**, checked with
`git grep -n -E 'docs-only ok|docs tier|docs-tier|TIER=docs' origin/main -- 'work/vnews/*' 'work/vgeom/*' 'work/vseam/*' 'work/vdoc/*' 'docs/prompts/*'`:
the four successors' plans do not restate it, and
`docs/prompts/implementer-discipline.md`'s one mention of `TIER=docs`
is about a file class, not a tier marker, and is already count-free.
`work/vnews/plan.md:341` names this row by id and resolves when it
closes.

**What that sweep cannot match:** a job count written as a bare number
beside a run id with no "jobs" or "-job" token next to it, and any
count stated in a PR body rather than in the tree. The first is why the
rule now says a total cannot be a marker at all, rather than fixing
every total.
