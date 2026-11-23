---
id: split-and-inline-name-refusals-do-not-project-the-missing-node
kind: issue
title: pncad-py: split/inline name refusals carry a missing-node id the Python projection drops
status: open
opened: 2026-09-20
priority: P4
cost: E
---



## Finding

Disclosed by FIX's
`remap-name-misses-lose-the-id-they-caught-at-six-refactor-sites` lane,
which added the field. LIB's ground (`crates/pncad-py/` is LIB's by
`paths`), so the projection decision is not FIX's to make.

`SplitError::NameStraddlesCut`, `SplitError::PartNameReachesRemainder`
and `InlineError::StrandedPartName` now carry, beside the name they
already named, the NODE the remap could not map — `missing`. It is not
redundant with the name: a `StableName` embeds other names in its
`path`, so for a nested name the raised error names the OUTER name
while `missing` names the node that actually failed, and those are
different nodes. That is the whole reason the field exists.

`crates/pncad-py/src/py/refactor.rs` projects both doors' errors
arm-by-arm into the `(node, consumer, input, gauge, instance, param,
name, doc_id)` tuple. FIX's PR kept that projection UNCHANGED — the
three arms bind `..` — so a Python caller reading `split_err` or
`inline_err` sees the name and nothing else, and the `node` slot these
three arms already fill with `None` is exactly where the id would go.

The tag inventory is untouched: `split_error_tag` and
`inline_error_tag` match `{ .. }`, no tag was added or renamed, and
`crates/pncad-py/src/tests.rs`'s tag tables did not move.

## What a taker owes

The `missing` id projected on the three arms — the `node` slot is free
on all three, and `PartNameReachesRemainder` already spends it on the
CUT NODE carrying the reference, which is a different node, so that arm
needs a slot decision rather than a copy of the other two.

With it: the `pncad.pyi` stub, the binding census and the stub tests,
which is the per-door cost
`pncad-py-seven-doors-lack-field-projection` describes for every door
it lists. That row is about doors with NO projection; this one is a
projected door that gained a field, so it is a different unit and a
much smaller one.

- 2026-11-03 — moved from LIB to BIND by `git mv` when LIB closed, id and body unchanged; banded P4 and priced E in the move (LIB had three P1 rows left behind 124 closed ones; its library-door rows went to PYDOOR, opened for them, and its tag, stub, census and projection rows to BIND; Ev, in chat).
