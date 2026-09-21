---
id: pickkinds-doc-states-its-disclaimer-twice
kind: issue
title: PickKinds' doc says it is not a filter vocabulary twice, one paragraph per tool that arrived
status: open
opened: 2026-09-21
priority: P4
cost: E
refs: [3007]
---

Found by the whole-file read (`docs/prompts/reviewer-style-lane.md`
Q8) during #3007's review. Not #3007's doing — both paragraphs
predate it.

## Finding

`crates/viewer/src/pickindex.rs`, `PickKinds`' doc comment, says

> **This is deliberately not a filter vocabulary.** It is two answers
> to the one question the shipped tools actually ask…

and then, thirteen lines later,

> **This is still not a filter vocabulary.** It is three answers to
> one question, one per shape of tool the GUI has shipped…

The second paragraph arrived with the blend tool and restates the
first with the count moved from two to three. Q8's accumulation shape
exactly: no single diff was unreasonable, and nothing in the process
reads a whole doc comment. A reader now meets the same disclaimer
twice with two different counts beside it, and the first count is
stale.

One paragraph, with the count derived from the variants rather than
written twice, is the repair. `work/vseam/pick-priority-filter-
vocabulary` (deferred, P3) owns the vocabulary QUESTION; this is only
its prose.

## Fence

`crates/viewer/src/pickindex.rs` is VGEOM's, VSEAM's, CHROME's, FIT's
and VIEW's; the sentence is VDOC's per this program's `keep_out` — *a
prose or citation defect lands wherever the sentence is* — and a unit
here announces the edit to the owners.
