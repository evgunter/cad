---
id: pickindex-tie-break-rests-on-a-comment
kind: issue
title: The pick tie-break's NaN disposition is a comment, not a guard
status: dispatched
opened: 2026-09-16
priority: P1
cost: E
branch: vgeom/pick-distance
---



## Finding

`crates/viewer/src/pickindex.rs`, the candidate tie-break:

```
        candidates.sort_by(|left, right| {
            left.distance
                .partial_cmp(&right.distance)
                .unwrap_or(core::cmp::Ordering::Equal)
                .then(...)
        });
```

#2788's sweep disposed of this as *not a member* of the
value-it-did-not-compute class, on the strength of the comment three
lines above it: *"all integers after the first, and the first is never
NaN (a projected distance is a finite pixel measure)."* The review of
that PR pushed back and is right: **a textual justification is not a
defence.** Nothing enforces the invariant the comment asserts, and
`Ordering::Equal` for a pair that does not compare is an ordering the
comparison did not produce — with a consequence, since the `.then(...)`
chain then decides the pick by boundary and segment index, quietly, on
a comparison that failed.

`sort_by` does need a total order, so the repair is not to delete the
`unwrap_or`. It is to make the invariant hold where it is claimed:
either the candidate's `distance` carries its finiteness in its type,
or the candidates are filtered before the sort and the filtered ones
are reported rather than ranked.

## What is NOT claimed

No reachable producer of a non-finite projected distance was found.
The finding is about the guard, not about a live wrong pick: a sweep
that accepts a comment as evidence has accepted the one kind of
evidence that cannot go stale loudly.

## Fence

`crates/viewer/src/pickindex.rs` — VIEW's.
