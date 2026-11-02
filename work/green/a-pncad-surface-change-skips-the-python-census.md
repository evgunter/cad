---
id: a-pncad-surface-change-skips-the-python-census
kind: issue
title: A diff that adds to pncad's public surface skips the python suite, so the binding census reddens the next pncad-py PR instead
status: open
opened: 2026-11-02
priority: P3
cost: E
---


## Finding

PR 6390 (FORMS) made `editor_core::slot_admits` public and re-exported
it through `crates/pncad/src/document.rs`. Its CI run (41501646252,
head 891d53ae4) recorded **`python suite (wheel + guide + north-star)`:
skipped**, the gate went green, and it merged. The binding census
(`crates/pncad-py/tests/test_binding_census.py`,
`test_every_curated_name_is_bound_or_listed`) requires every curated
pncad name to be bound or listed, so main was red from then on. It
surfaced only on the next PR that touched pncad-py (#6393, an
unrelated unit), which inherited the red. Fixed by #6397.

`scripts/ci-filter.py`'s own text says the opposite of what the run
did. `RUN_PNCAD_PY` is "REPORTED, GATES NOTHING — the python suite
runs on every code-tier run in both halves", and the `pncad-py` entry
in the closure notes says the same. The run record shows the suite
skipped on a code-tier run whose diff reached pncad's re-export
surface.

## What a fix has to settle

Either the python suite (or at least the binding census, which needs
the wheel) runs whenever a diff's seeds reach pncad's public surface,
or the census moves to a check that does not need the wheel and runs
wherever pncad changes. Correct the filter's prose to match whichever
is true. Silent coverage gap, so P3 (`work/README.md` Priority:
tooling that prevents SILENT bugs).

Signed (FORMS orchestrator).
