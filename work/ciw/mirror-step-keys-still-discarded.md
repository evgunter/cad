---
id: mirror-step-keys-still-discarded
kind: issue
title: 'shell: and a step-level continue-on-error: still decide a mirrored pair''s semantics and are read as text'
status: open
opened: 2026-09-11
---


## What

`working-directory:` left `STEP_KEYS` for claim 10 in the third slate's unit 4.
The sweep that unit ran over the rest of `STEP_KEYS`
(`scripts/check-ci-mirror-parity.py:812`) leaves two keys whose value decides
what a mirrored pair MEANS and which are still read as text:

1. **`continue-on-error:` on a STEP.** A hosted step that may fail without
   failing its job, paired with a local row that fails the run, is exactly the
   `--no-fail-fast` defect one level up: the two halves report a red
   differently under one row name, which is the sentence at the top of claim
   10. The JOB-level key is read (`Job.continue_on_error`, claims 7 and 8); the
   step-level one is not. Four live sites, `.github/workflows/ci.yml:3225`,
   `:3261`, `:3712`, `:3738` — **none of them on a cited pair** (two are the
   test-cost-report step, two are `upload-artifact`), so the population on
   claim 10's own subject is zero today.
2. **`shell:`.** It decides what a `run:` block IS. `shell: python` or
   `shell: pwsh` on one half would leave `cargo_flags`, `env_prefixes` and
   `work_dirs` reading a non-shell body as shell and answering confidently.
   Zero sites: the string `shell:` appears once in either workflow, in a
   comment at `ci.yml:3128`.

## The shape of the fix, and the argument against taking it now

Both are one-line `Bail`s on a cited pair, the bargain the `$GITHUB_ENV`, the
`uses:`-step, the `defaults:` and the standing-assignment refusals all make: a
line costs nothing and the day someone writes one the gate says so instead of
passing. The counter-argument is the one this file's own items keep making —
`scripts/check-ci-mirror-parity.py` is over 4000 lines and has taken a claim or
an arm in each of the last six units — and a `Bail` over a population of zero
is the cheapest thing in it to add and the easiest to add for its own sake.
Worth taking with the next unit that opens this file for another reason, rather
than as a unit of its own.

## Provenance

The `STEP_KEYS` sweep run by CIW's third slate, unit 4
(`mirror-pairs-context-beyond-env` clause 1), which is where the sweep's hit
list and its blind spot are written.
