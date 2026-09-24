---
id: viewer-disabled-control-population-has-no-gate
kind: issue
title: The viewer's disabled-control population is a measurement with nothing reading it
status: open
opened: 2026-09-19
refs: [measurements-have-no-mechanical-guard]
priority: P3
cost: E
---

Filed by a VNEWS census lane (`work/vnews/a-disabled-control-says-why-
in-four-shapes`) at merge base
`2654cc111417da806d9786c40136106469096fec`. An announced crossing:
`scripts/gates/*` is GUARD's. **Nothing under `crates/` or `scripts/`
was touched.**

## The measurement

That census's mechanical pass is: **every `add_enabled` /
`add_enabled_ui` call site under `crates/viewer/src` — 30 today**, each
classified into one of five dispositions (a hit owing a refusal's own
sentence, a control that already reads one, a chrome-policy gate, a
draft gate, and a control with no operation behind it at all). Three of
the 30 are open defects with rows of their own; the other 27 are
asserted correct.

**Nothing goes red when a thirty-first lands.** No test enumerates
them, no gate greps for them, no register re-measures. The claim lives
in a tracker file that is deleted when VNEWS closes, and the clause
`work/vdoc/a-disabled-controls-reason-has-one-home` asks
`crates/viewer/README.md` for carries the sweep rule but not the
membership.

This is the §Q6 shape — *"a mechanical guard, or a scheduled register
that re-measures it, or a written reason it can have neither"* — and it
is **unguarded, not unguardable**.

## Why it is gateable, with the precedent

- `scripts/gates/viewer-vocab-declared-once.sh` greps
  `crates/viewer/src` for a hand-written membership list and reds
  unless `crates/viewer/README.md` ratifies it. Same tree, same
  instrument, same "a README clause with nothing reading it" motive —
  its own header says the standard it rejected was *"a rule sold as
  mechanically checkable [that] spent its first life with nothing
  reading it."*
- `scripts/gates/viewer-module-kinds.sh` is the same shape one level
  up.
- `scripts/gates/probe-suite-census.sh` is the precedent for a gate
  that holds a **census's membership** rather than a number in a
  comment.

## What the gate would hold

Not a count — a count is a baseline to preserve, which this repo does
not do. The falsifiable version is an **allowlist keyed by site**: each
`add_enabled` / `add_enabled_ui` call site in `crates/viewer/src` is
listed with its disposition, and a call site not on the list reds with
*"a new disabled control: say which of the five it is"*. A lane that
adds one adds its row in the same diff, which is the point — the
classification is made at the moment the control is written rather than
re-derived by the next census.

Open questions for whoever takes it, none settled here:

- **Where the list lives.** Beside the gate, as the other allowlist
  gates do (`bounds-allowlist.sh`, `evalscalar-allowlist.sh`,
  `interval-square-allowlist.sh`), or in `crates/viewer/README.md`
  where the clause is, read through `viewer-readme-fence.awk` — the
  CommonMark fence tracker both existing viewer gates already load to
  read that page.
- **Whether the disposition is checkable or only recorded.** A gate can
  see that a site is listed; it cannot see that "draft gate" is the
  right answer for it. That is the honest limit and belongs in the
  gate's header, per this directory's convention of saying what each
  matcher cannot see.
- **`lib.sh`'s Rust reader** already offers a `code_only` view with
  comments stripped, which is what makes the two prose `add_enabled` lines inside
  `pane/profile.rs`'s own comment — the census's one manual exclusion —
  a thing the gate would not need to special-case.

## Relation to `measurements-have-no-mechanical-guard`

That row's sweep is over **numbers written into `crates/*/src` doc
comments**. This one is a population claim in the **tracker** about a
population in the tree, which that sweep does not range over, and its
repair is a gate rather than a `const _: () = assert!`. Filed
separately for that reason and cross-referenced rather than appended.

## Home

GUARD's: `scripts/gates/*`. The clause the gate would hold is VDOC's
(`crates/viewer/README.md`) and the population is VNEWS's; both are
hand-offs, not diffs from here.
