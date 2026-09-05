---
id: perf-plan-is-cited-by-twenty-nine-files-and-absent-from-tree-and-ledger
kind: issue
title: docs/PERF-PLAN.md is cited by path from 29 tracked files and exists in neither the tree nor the ledger's deleted-doc records
status: open
opened: 2026-09-04
---


## What

Found by DOCM-5's implementer lane (PR 1871) following a citation:
`docs/PERF-PLAN.md` — the performance discipline document ("brute force
until a measurement says otherwise", the deferred-quadratic record) —
is cited by path from 29 tracked files (`grep -rln PERF-PLAN crates
docs demos scripts memories work`), among them `crates/editor-core/src/checks.rs`'s
cost note (rewritten in that PR), `docs/DOCM-5-SPEC.md` (the
orchestrator's own spec, written from those citations), and
`docs/DOC-LEDGER.md:186`, which names it as the SUCCESSOR of an
archived milestone record. The file is not in the tree, and the ledger
records no deletion of it — the one document whose job is to say
where deleted docs went does not know this one is gone.

The live statement of the same rules is `memories/perf-measurement-lane.md`
(hosted producer, append-only history, a counter before a stopwatch),
which DOCM-5's measurement followed instead.

## What it wants

Either (a) the ledger gains the deletion record (the SHA at which the
file last existed, and what superseded it — presumably the memory
above and `docs/perf-data/`'s history), and the 29 citations are
repointed or rewritten to the live home; or (b) the document is
restored from its last SHA if it still says something nothing else
does. Either way, a citation to a path is a claim the path exists, and
today 29 of them are false. Not a program's slate: the ledger is the
repo's.
