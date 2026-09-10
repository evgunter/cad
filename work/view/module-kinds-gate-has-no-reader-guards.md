---
id: module-kinds-gate-has-no-reader-guards
kind: issue
title: viewer-module-kinds.sh has no reader-guard apparatus at all: two unguarded reader stages in two process substitutions, whose death reds telling the author the README heading was renamed
status: closed
opened: 2026-09-10
closed: 2026-09-10
pr: 2287
branch: view/module-kinds
refs: [gate-roster-and-probe-census-have-no-reader-guards]
---

Found by the §5 sweep for #2282. **Split from a combined row** whose
other half is `module-kinds-table-scan-ends-at-any-column-zero-hash`:
that one is `md_fence` plus a length check, this one is apparatus the
gate has never had. Different repairs, different controls.

## The gap

`scripts/gates/viewer-module-kinds.sh:220-227` (at `09b0ef5a8`; that function is gone, see Closed below)'s `readme_table_modules`
is `awk … | sed …` — two stages, both reading `$README`, both inside a
process substitution (`mapfile … < <(readme_table_modules …)` at `:270`
and `:277`), so both exit statuses are discarded. **The gate has no
reader guard at all**:

```
grep -nE 'status=\$\?|reader_failed|gate_reader_died|abort_if' \
  scripts/gates/viewer-module-kinds.sh
```

returns nothing.

## It is a MISDIAGNOSIS and not a false green, and the difference matters

The callers check for an empty roster (`:271-274`, `:278-282`), so a
dead reader **does** red. What it reds with is the defect: *"Either the
heading was renamed or the table was reshaped"* — about a README that
is perfectly fine, sending its author to edit the one thing that is not
wrong. That is
`gate-reader-guards-count-six-where-the-stated-rule-yields-nine`'s
class at a gate that never had the apparatus, rather than one that had
it at the wrong granularity.

## What a fix owes

- Per-stage guards on the `awk` and the `sed`, each named for its own
  stage — `viewer-vocab-declared-once.sh`'s `reader_failed` /
  `abort_if_reader_failed` pair is the worked precedent, and #2282 is
  where the brace-group shape and its controls are argued.
- **A case per stage, run against the unfixed reader and recorded red.**
  Note the harness limit #2282 established: `gate_selftest_case` and its
  broken-tool twin can assert a name is PRESENT and never that one is
  ABSENT, so a repair whose effect is to remove a wrong name has no
  expressible control
  (`work/issues/gate-selftest-cannot-observe-the-identity-a-gate-names`).
  Here every guard is NEW, so every case is a genuine control.
- Killing the right-hand stage of a pipeline needs a shim that CONSUMES
  its input and then exits; a stub that dies at once takes the upstream
  stage down with SIGPIPE and the diagnosis names the wrong reader.

## The wider population is NOT this item's, and is already filed

The sweep arm that found this one could only match a pipeline that
**already has a guard**, so it structurally could not see a gate with
none. Re-run on the right rule — every stage inside a process
substitution whose status the shell discards, i.e. `grep -n '< <('` and
read each — the population is larger: `scripts/gates/gate-roster.sh`
(zero guard sites, one three-stage substitution) and
`scripts/gates/probe-suite-census.sh` are filed on **code-quality's**
slate as `gate-roster-and-probe-census-have-no-reader-guards`, since
`scripts/gates/*` returned to code-quality when the `gates` program
closed. It was named in prose when this row was filed, because it was
not on `main` and the reference would not have resolved; it landed
2026-09-10 and is in `refs:` now.

## Confidence

`sure` on the two unguarded stages, on the no-guard grep and on the
misdiagnosis-not-green reading — all read off the file rather than
inferred.


## Closed (2026-09-10)

`reader_failed` at `scripts/gates/viewer-module-kinds.sh:293-298` and
`abort_if_reader_failed` at `:307-313`, over the population stated at
`:236-270` rather than left to a count: **twelve stages**, with the
rule that produces them written beside the list — every stage on PATH
whose status this file must read for itself. The `gate_grep`,
`gate_rust_code` and `gate_record_awk` sites that diagnose themselves
in `lib.sh` are named there as excluded rather than left looking like
an oversight; the first version of this list said fifteen and certified
a population it did not produce.

**The item names two stages and the gate has twelve.** Its arm could
only see `readme_table_modules`, so it missed the module enumerator's
three (`find | sed | sort`), the manifest reader's four (`awk | sed |
tr | sort`), the kind extractor's `sed`, and the hit union's
deduplicator and `sort`. Two live misdiagnoses were reproduced on the
real tree before the repair: a dead `awk` reported *"Either the heading
was renamed or the table was reshaped"* about a README that is fine —
the item's own example — and a dead `sed` reported *"no modules under
crates/viewer/src … besides lib.rs and bin/"* about a tree holding
forty-five, which the item does not mention.

**A third mode the item does not name: a status the shell KEEPS still
buys no diagnosis.** Three stages sit in `$(…)` rather than in a
process substitution, so errexit does end the gate on them — with no
gate name, no `::error::` framing and nothing said about what was left
undecided. Their cases show it: on the base reader those three fail as
*"the gate exited non-zero WITHOUT a gate_error diagnosis"*, which is
`lib.sh`'s S157 second half exactly.

Twelve cases, one per guarded stage, each wanting its own stage by
name, and each run against the base reader with the same shims: twelve
red there, twelve green here. The right-hand stages use a shim that
CONSUMES its input before exiting, per the item; the two `sort` stages
that carry no distinguishing argument are told apart by what they are
READING, which is the one thing that differs between them.
