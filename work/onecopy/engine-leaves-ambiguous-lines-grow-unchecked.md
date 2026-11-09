---
id: engine-leaves-ambiguous-lines-grow-unchecked
kind: issue
title: engine-leaves: an ambiguous NAME line is added freely, so a namesake can take a name out of the count without a pin moving
status: open
opened: 2026-11-06
priority: P3
cost: E
---


From the review of PR 6554 (`onecopy/engine-leaves-pins`).

`scripts/gates/engine_leaves.py` (`census`, the AMBIGUOUS rule) counts an item an engine declares, when the same name is declared outside the engines too, only where a path or an import through the engine spells it. The gate reds an `ambiguous NAME` line that is new or gone (`AMBIGUOUS-NEW`, `AMBIGUOUS-GONE`), but the fix it asks for is to add or drop the line. So a change can do the following without any pin rising:
- declare a namesake at the root for an engine item;
- add its `ambiguous` line.

Every bare use of that name then leaves the count. `engine-leaves.sh`'s base comparison (`gate_base_file`) covers the `KIND COUNT FILE` lines only.

The open question is how to hold it, for example:
- an `ambiguous` line new against the merge base reds unless it cites a row;
- or the counter counts a name declared in both places wherever it resolves through the engine.

## Resolved

The first option. `engine_leaves.py` `raised`, the one place a pin is compared with the merge base, now reads `ambiguous NAME` lines as well. One absent from the base is `RAISED ambiguous NAME`, unless its note cites the row that retires the namesake (`CITED`). `pin-cites-an-open-row.sh` already holds such a cited row open.
