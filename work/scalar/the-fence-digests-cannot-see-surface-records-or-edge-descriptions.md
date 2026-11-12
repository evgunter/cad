---
id: the-fence-digests-cannot-see-surface-records-or-edge-descriptions
kind: issue
title: the m10_p_fence digests read only points and outcomes, so a moved surface record or edge description passes every fence row
status: open
opened: 2026-11-11
---


Filed by the onedoor fix-3 lane on PR 6505 (rv3 review m2). Not fixed there.

## What

`crates/editor-core/tests/m10_p_fence.rs` digests each corpus
document's evaluation through its points and outcomes only (the
digest fold over vertex points and node outcomes, around the f64,
Interval and ids-masked rows). A change that moves which surface
record a face wears, or how an edge is described (`OnPair` against
`InChart`, its pair's keys), moves no point and no outcome, so every
fence row stays green over it.

## Evidence

On PR 6505 the `seat4`/`seat7`/`seat8` digests of `crossing_slots`,
`kitchen_sink` and `corner_table` moved, and the reviewer's document
dump (`zz_rv3_dump_moved_documents`) showed faces, senses and volumes
unchanged: only edge descriptions moved. The fence rows held at all
three ε over the same change, so "the fence held" said nothing about
records or descriptions.

## What closing it takes

A fence row (or a field of the existing digest) that folds in, per
face, its record's key class (which faces share one) and its sense, and
per edge, its description's kind and the records it names, in a
key-order-free form so the ids-masked row can carry it too.
