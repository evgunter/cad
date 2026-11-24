---
id: a-read-record-is-consumed-by-any-removed-remainder-so-a-lineage-gap-drops-it-silently
kind: issue
title: The carry consumes a read, unlanded record wherever any remainder was removed, so a lineage gap drops the record silently instead of refusing it stale
status: open
opened: 2026-11-23
priority: P1
cost: M
design: true
refs: [a-sphere-cut-in-divides-a-face-the-carry-reads-by-its-pre-cut-key, 7593]
---

Found by the review of ORBIT's PR 7593 (`orbit/cut-in-carry`), which
demonstrated it on main; filed by that lane.

## What

`boolean::records::landings` (`crates/topo/src/boolean/records.rs`, the
tail of its per-row loop: `if read { if removed || moved { continue; } ...
Stale }`) consumes a record that landed on no pair, after some pair was
read, whenever ANY of its cells' fates removes something
(`crate::Fate::removes`): a cell removed whole, or divided with its
remainder removed. It does not ask whether the removed part is where the
touch was. So a lineage that hands a cell's images incompletely, and
also removes some unrelated piece of it, reads the record against the
wrong pieces, finds it nowhere, and drops it, where the rule would
otherwise refuse it `CarryRefusal::Stale`. Every lineage gap of that
shape therefore surfaces as a silent drop, not a refusal.

Demonstrated on main before PR 7593: the banded ball carrying a
vertex-on-face rest on the piece a sphere cut-in minted, united with
or subtracted by the cutting slab, returned a body with no record. The
fates there were `[Kept { vertex }, Divided { pieces: [Face(14v1)],
minted: [], remainder: Removed }]`: the carry read the face by its
pre-cut key, so its only image was the OTHER piece, and the removed
remainder (the cap the slab took) licensed the drop. PR 7593 closes that
gap (the re-entered op reads the cut-ins' divisions), so this pose no
longer reaches the rule, but the rule still hides the next gap.

## Why it is a design question

The rule is ratified text: `crates/topo/README.md`'s carry paragraph,
"consumed where one of its cells was removed, divided with its remainder
removed, or moved". Requiring the removed part to have held the touch
(for instance, consuming only where the record's point, read against the
removed part, lies in it) changes what that clause decides, so it goes
through designers and an `[ev]` PR.

## Way through

Weigh the fork first. Then measure what moves: every boolean and
surgery-verb test whose records are consumed through a removed
remainder, to see which drops were touches the op really removed and
which were gaps.


