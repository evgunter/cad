---
id: step-import-freecad-job-is-named-for-the-wrong-door
kind: issue
title: The 'step import (freecad)' job is named for the import door and gates on the export fixtures
status: open
opened: 2026-09-15
priority: P4
cost: E
---

## From PORT's `python-cannot-set-options-structs` review (PR #2678)

The job id is `step-import` and it renders as **`step import (freecad)`**
(`.github/workflows/ci.yml`, the `step-import:` job). What it actually
does is stated two lines below its own name, in a comment: it builds
nothing, and runs FreeCAD over the committed fixtures under
`crates/step-export/tests/fixtures`, which are byte-golden against the
**export** writer. Its gate is `needs.filter.outputs.run_step_export`.

The gating is correct. The NAME is the defect: a lane whose diff changes
the STEP **import** door reads a skipped `step import (freecad)` on its
green run and has to open `ci.yml` to learn that the row was never about
its change. That happened on PR #2678 twice — once by the implementer,
who flagged the skip as possibly-meaningful in its report, and once by
the reviewer, who read the workflow to answer it. Two readings of the
same file to recover a fact a word would have carried.

A rename to something like `step export fixtures (freecad)` is the whole
fix, plus the job id if the id is worth moving. Cheap, and it is
CIW's ground: the run's job roster is what a lane reads a green from.

## Home

`work/ciw/` — `.github/workflows/ci.yml` is CIW's. Filed by a PORT lane
under `work/README.md`'s rule that a finding goes on the slate of the
program whose ground it lands on.
