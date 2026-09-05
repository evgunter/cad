---
id: fillet-specs-require-a-narrowing-ci-config
kind: issue
title: Two live FILLET specs require a CI-Config trailer that now narrows the gate, in a clause that contradicts itself
status: closed
opened: 2026-09-04
closed: 2026-09-04
refs: [1823]
---

## Filed here rather than in `work/fillet/`, deliberately

`work/README.md`: an item's directory is the program that owns it, and a
program claiming another's item MOVES the file. `docs/prompts/implementer-discipline.md`
§6: a lane does not file on another program's slate from a unit branch,
because it cannot see whether the item already exists. **FILLET owns this and
should claim it by moving this file into `work/fillet/`**; it sits in
`work/issues/` until then so that the finding has a file at all rather than
living only in a merged PR body, which stops being read at merge.

## The finding

`docs/FILLET-RIM-SPEC.md:134` and `docs/FILLET-ATTR-SPEC.md:137` both sit
under **`## Acceptance`**, so an implementer must SATISFY them to accept the
unit. Both require a `CI-Config: lane=interval` trailer on the head commit,
and both describe the run it produces as gating "the drawn point plus the
interval lane asked for".

**PR 1823 (CIW, merged/open on `ciw/reinstate-full-runs`) made both clauses
false, in two different ways at once:**

1. **There is no drawn point.** The lane and eps draws were removed on
   2026-09-04 (Ev's authorisation). An un-narrowed code-tier run gates every
   point of {default, `interval`} x {default, 1e-6, 1e-12} as twelve `test (…)`
   jobs. A clause promising "the drawn point plus the interval lane" describes
   a mechanism that no longer exists.
2. **Satisfying the clause now costs coverage.** A `lane=interval` request
   NARROWS the run to one compile mode — six test legs instead of twelve, and
   no default-lane `clippy`. An implementer obeying the acceptance clause gates
   strictly less than one who ignores it.

**And as of PR 1823 the clause cannot be satisfied at all through that
spelling.** The trailer is additive-only: `ci-filter.py`'s `WHOLE_BY_DEFAULT`
refuses any trailer value that would gate less than no trailer, so
`CI-Config: lane=interval` on a head commit REDS the `classify` step with a
message naming the `workflow_dispatch` input as the place narrowing lives.
The refusal is the intended behaviour for a copied trailer; the consequence
for these two specs is that the acceptance clause is now unsatisfiable as
written.

## What the fix is

FILLET's call, and the shape is small: delete the trailer requirement, and
replace the "which point gated" sentence with what a reader should now check
— that the run carries twelve `test (…)` jobs, i.e. that nothing narrowed it.
`docs/prompts/implementer-discipline.md` §2 carries the standing version of
that instruction and names specs as a stale source, so the specs can point at
it rather than restate it.

## Why it is urgent rather than tidy

FILLET has ~20 open items. Every one of them dispatched against these specs
inherits an acceptance clause that (a) instructs a coverage reduction and (b)
reds the gate if obeyed. The failure is loud rather than silent, which is why
it is an issue and not an incident — but it lands on an implementer who did
what their spec told them to.

## Closed

Fixed in CIW's `delete-config-trailer` unit, in the sweep that deleted the
`CI-Config:` path itself, rather than by FILLET claiming this file. Both
acceptance clauses — `docs/FILLET-RIM-SPEC.md` and `docs/FILLET-ATTR-SPEC.md`
— now say what this item's *What the fix is* section asked them to: no
trailer, and count twelve `test (…)` jobs.

**One sentence above is now false and is left standing as the record of what
was true when this was filed.** The finding said `CI-Config: lane=interval`
on a head commit REDS the `classify` step. It no longer does anything at
all: the trailer parser, the `--config-from-message` flag and the ci.yml
plumbing that fed it are deleted, so a trailer line in a commit message is
inert text. That makes the stale instruction quieter, not safer — an
implementer obeying it would get no error and no interval-lane guarantee
from the line — which is why the specs were swept rather than left to red.

The three other specs that named the spelling live were swept in the same
pass: `docs/PCURVE-P2-SPEC.md`, `docs/EXCH-H1-SPEC.md`,
`docs/FILLET-H5-SPEC.md`. FILLET, PCURVE and EXCH own those files; the
change is one clause each and is announced in the PR body.
