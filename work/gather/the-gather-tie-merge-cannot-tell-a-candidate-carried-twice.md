---
id: the-gather-tie-merge-cannot-tell-a-candidate-carried-twice
kind: issue
title: the gather's tie merge cannot tell two pieces of a tie from one candidate carried twice by two sources
status: open
opened: 2026-09-25
priority: P3
cost: D
---

Found by the GATHER split-halves lane (`gather/split-halves-tie-merge`),
while sweeping the tie bit the product's carry reads.

`CarriedRows::carry` (`crates/editor-core/src/names/defer.rs`) defers
every row whose source entry is `Tied`, and since the split-halves fix
every `Unique` row marked as one piece of a separated tie
(`NameTable::project`). The flush then merges everything deferred under
one name into one `Entry::Tied`. That is right when the sources hold
DIFFERENT candidates of the one upstream tie — the two halves of a
split. It is also what happens when two sources hold the SAME
candidate: a split root beside a root over the split's target (the
"intact pass-through" route in `ProductError::Naming`'s doc,
`crates/editor-core/src/product.rs`) carries a tie's uncut candidate
from both, and the merge writes it into the product's tie twice, once
per copy, without a refusal from that name.

Today the document still refuses, because the two sources share the
uncut candidate's boundary verbatim and a strictly named entity of
that boundary collides in the per-root carry first
(`wire_product_gather_tie::two_roots_aliasing_a_strict_name_still_refuse`
pins that route's strict collision, on a block with no tie). A document whose whole shared boundary is
tied would gather instead, with the product's tie listing one face
twice over. None is built. The merge would need candidate identity
(which upstream entity a row descends from), not only a tie bit, to
refuse it at the name. The checks' separation resident reports the
overlapping solids either way, so the geometry is not silent.
