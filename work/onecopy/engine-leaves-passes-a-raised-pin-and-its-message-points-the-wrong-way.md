---
id: engine-leaves-passes-a-raised-pin-and-its-message-points-the-wrong-way
kind: issue
title: engine-leaves.sh passes a raised pin (it 'reads only in review') and its failure text sends a root reader into the engine
status: open
opened: 2026-11-06
priority: P2
cost: E
refs: [the-census-sort-of-a-boolean-refusal-has-no-legal-home-under-topo-homes]
---


From the census-sort fork (2026-11-06; both designers).

## What

1. **A raised pin passes.** `scripts/gates/engine-leaves.counts`' header says a raised pin "reads only in review", because CI checks out one commit. So PR 6451's 506 → 510 passed. CI's PR job checks out the test merge; if the gate job has the base available (the merge's first parent), the gate can compare each pin against the base's counts file and fail a raise. First check whether the `lint` job has the history.
2. **The failure text points the wrong way.** `engine-leaves.sh` prints "code naming an item declared under an engine belongs to that engine". That sends a root reader into the engine, which is the misreading behind the census-sort row. State the README's remedy instead: the named item is a reading the engine hosts, so move it to the root or below.
3. **The header should state the migration practice.** A new site is paid in the reader's `names-*` pin, citing the unit-5 move row that retires it, never by a new `pub(crate)` in the engine.

All of this is agent text (PR 6458), not ratified.
