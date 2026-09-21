---
id: pickindex-tie-break-rests-on-a-comment
kind: issue
title: The pick tie-break's NaN disposition is a comment, not a guard
status: open
opened: 2026-09-16
priority: P1
cost: E
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

## The invariant this row asks for now holds (2026-09-21, `vgeom/p0-fields`)

Recorded here rather than closed, because this row was not dispatched
and the judgement of whether it is answered is the orchestrator's.

`a-nan-edge-distance-wins-its-boundary-rather-than-losing` (P0, closed
on `vgeom/p0-fields`) took the second of the two repairs this row
names — *the candidates are filtered before the sort*. The per-edge
walk is now `pickindex::best_segment`, whose admission is
`distance.is_finite() && distance <= EDGE_PICK_RADIUS_PX`, so every
`Candidate` reaching `candidates.sort_by` carries a finite distance
and `partial_cmp` cannot fail on the first key.

What this row asked for and did NOT get: the finiteness is not carried
in the TYPE, and `unwrap_or(Ordering::Equal)` still stands — which the
row itself says is correct, since `sort_by` needs a total order. The
comment three lines above the sort no longer offers *a projected
distance is a finite pixel measure* as the evidence; it names
`best_segment` instead.

