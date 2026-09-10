---
id: gate-roster-and-probe-census-have-no-reader-guards
kind: issue
title: gate-roster.sh reports OK over a reader that read nothing: zero guard sites, and its diagnosing variant names the wrong stage
status: open
opened: 2026-09-10
---


Found by the style review of VIEW's PR 2282, which repaired this exact
class in `scripts/gates/viewer-vocab-declared-once.sh` and then swept
for siblings. `scripts/gates/*` is this program's again since the `gates`
program closed (`work/code-quality/program.md:72-77`), so this is a
report and not a change. **Both halves below were reproduced by the VIEW
orchestrator independently before filing**, at `07fda9803`.

## The false green, reproduced

`scripts/gates/gate-roster.sh` has **zero** guard sites —
`grep -cE 'status=\$\?|reader_failed|gate_reader_died'` returns `0` — and
its one process substitution at `:179-180` is three stages:

```sh
done < <(gate_grep -oE 'scripts/gates/[A-Za-z0-9_-]+\.sh' <<<"$cmds" \
         | sed 's#^scripts/gates/##' | sort -u)
```

A `mapfile`/`done < <(…)` discards the substitution's exit status by
construction, so nothing observes any of the three. Shim only that `sed`
so it **consumes its input and then exits 9** — no `SIGPIPE` travels
upstream — and the gate prints

> `gate-roster OK: ci.yml wires a self-tested step for all 21 gates in
> scripts/gates/ and for 1 under lib.sh's contract sited outside it …
> (22 registered gates scanned)`

and **exits 0**, having read nothing. The loop body never runs, so
"every hosted invocation names a real gate" is satisfied vacuously. That
is the `#1953`/`#2106` shape: a gate printing OK while deciding less
than it claims, and here the thing it claims to have decided is CI's own
wiring.

## The diagnosing variant names the wrong stage

Shim the same `sed` to exit **immediately** and the gate does red —
but on the stage upstream of the one that died:

> `ERROR: gate-roster: grep exited 141, so what it did not read is
> unknown … Call: grep -oE scripts/gates/[A-Za-z0-9_-]+\.sh`

141 is `SIGPIPE`: `grep` died because `sed` closed the pipe, and the
diagnosis sends a reader to look at a `grep` that ran correctly. This is
the second half of the same class —
`work/view/gate-reader-guards-count-six-where-the-stated-rule-yields-nine`,
closed by PR 2282, with the repair `lib.sh`'s own `const_hits` argued
for first: **a guard belongs on a stage and never on a pipeline**, because
`pipefail` reports the rightmost non-zero stage.

## The same shape at `probe-suite-census.sh`, unreproduced

`scripts/gates/probe-suite-census.sh` also has zero guard sites, with
four process substitutions: `:385` (`find | sort`), `:461`
(`gate_grep -rlF | …`), `:539` and `:550` (`printf | awk | sort -u`).
The review could not isolate these with a shim, so this is a **candidate
list and not a result** — stated that way rather than folded into the
reproduced finding. `gate_grep` is itself guarded inside `lib.sh`, which
is why the `gate-roster` red above names it at all; what is unguarded is
every stage after it.

## The sweep arm that could not have found this

Recorded because it is the reason a live false green sat unopened.
PR 2282's §5 sweep had two arms. Arm 1 — *every gate that reads a
markdown section* — is shaped like the class and found its one hit.
Arm 2 was `|| status=$?` / `|| reader_failed` over the directory, which can
only match a pipeline that **already has a guard**, so it structurally
cannot see one with none. A sweep shaped like the symptom finds the
symptom. The population that finds this class is the gate's own rule:
**every stage inside a process substitution whose exit status the shell
discards** — `grep -n '< <('` across the directory, then read each
stage.

## Shapes

1. **A guard per stage in both files**, in the brace-group form
   `{ cmd || reader_failed "…" "$?"; }` that
   `viewer-vocab-declared-once.sh` now uses at eight sites, each naming
   the stage it is on. PR 2282 is the worked precedent, including its
   negative-control practice: each guard owes a case that dies on that
   specific stage and expects that stage's own name, proved red against
   the unguarded reader.
2. **Sweep the directory by the rule above rather than by these two
   files**, since neither was found by looking for it — the census is
   what would say whether 21 gates have this or two do.

## Confidence

`sure` on `gate-roster.sh`: both variants reproduced at `07fda9803`, the
quoted output verbatim, the guard-site count and the process
substitution read at the line. `likely` on `probe-suite-census.sh` —
structurally identical, not reproduced.
