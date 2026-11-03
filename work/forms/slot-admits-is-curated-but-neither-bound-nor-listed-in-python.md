---
id: slot-admits-is-curated-but-neither-bound-nor-listed-in-python
kind: issue
title: pncad's facade curates editor_core::edit::slot_admits (PR 6390) but pncad-py neither binds nor lists it, so test_binding_census reds every python-suite run on main
status: open
opened: 2026-11-02
priority: P2
cost: E
refs: [6390]
---


Found by the BOXES orchestrator (2026-11-02) as a red inherited by PR 6383.

PR 6390 (`forms/held-expression`, merged 2026-11-02T18:48Z) added
`slot_admits` to `crates/pncad/src/document.rs`'s façade re-exports.
Its own run skipped the python suite (the change filter did not
select it). So
`crates/pncad-py/tests/test_binding_census.py`
`test_every_curated_name_is_bound_or_listed` now fails on any run that
does select the suite: `['slot_admits'] != []` (first seen on PR 6383,
job 125482554334, whose diff does not touch `pncad` or the census).

The fix is the test's own instruction: bind `slot_admits` in pncad-py,
or add it to `BOUND_AS` with the Python spelling that answers the same
question, or to `NOT_BOUND` with its family. That choice is FORMS'.

Banded P2: main is red for every PR that touches pncad-py until it lands.
Signed: (BOXES orchestrator)
