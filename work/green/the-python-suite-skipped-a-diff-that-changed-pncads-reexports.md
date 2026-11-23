---
id: the-python-suite-skipped-a-diff-that-changed-pncads-reexports
kind: issue
title: ci: the python suite was skipped on a PR whose diff changed pncad::document's re-exports, which the binding census reads, and main went red
status: open
opened: 2026-11-22
priority: P3
cost: E
---

Found by OFFER (2026-11-22).

PR #7346 made `editor_core::NameCarrier` public and re-exported it from `pncad::document` (`crates/pncad/src/document.rs`'s `pub use` list). Its gate run 44986752586 reported the job `python suite (wheel + guide + north-star)` as **skipped**. After the merge, `crates/pncad-py/tests/test_binding_census.py::test_every_curated_name_is_bound_or_listed` failed on main with `['NameCarrier'] != []`. That test reads exactly that re-export list.

The failure first surfaced on #7362. #7377 fixes the census row. This row covers the coverage gap.

`scripts/ci-filter.py`'s header says the python suite "runs on every code-tier run in both halves", and that `RUN_PNCAD_PY` is reported and gates nothing. The run record contradicts that for this diff. Read the run's jobs API, not the workflow source, to see what decided the skip. Then make a diff that changes `pncad`'s public surface run the census, at least `test_binding_census.py`, which is pure-Python and takes about 17 s with no wheel.

A guard that cannot go red is P3 (`work/README.md`, Priority). (OFFER orchestrator)
