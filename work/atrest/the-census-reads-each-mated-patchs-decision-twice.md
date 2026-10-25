---
id: the-census-reads-each-mated-patchs-decision-twice
kind: issue
title: The at-rest census decides each mated patch's carrier pair twice: once to record its CensusAtRest row, once as the gate
status: closed
opened: 2026-10-10
priority: P4
cost: M
closed: 2026-10-23
---


## What

`contact-records-cite-their-decision` (B2) makes every mate-minted
`PatchContact` cite the at-rest census's decision of its two faces: the
product's gather (`editor-core` `assembly::mint`) runs the census's
Door 1 (`topo` `census`, the `Rest` carrier pair read through
`carrier_pair_reading`) on each mated pair, records a
`Coincidence { site: CensusAtRest }` on the `Product`, and the record
cites it. The at-rest gate (`gate_at_rest_declared`) then confirms
every record it is handed, which runs Door 1 on the same pair again.
So each mated patch puts two identical decisions on the K stream, and
the census decides the pair in two places (orchestrator ruling on B2,
2026-10-10: accepted for B2, no skip in the general gate).

## What closing it takes

The census becomes the door (stage 4 I, `mates-declare-no-contact`):
mates mint no records, the census records every contact it finds
between copies once, and the gate reads that decision rather than
re-deciding it. Then the mint-time read retires and one decision
remains per pair. If I lands without it, the gate reads the rows'
decisions for the records that cite them.

## Closed

2026-10-23, by `mates-declare-no-contact` (stage 4 I). The mint and its
mint-time Door 1 read are gone; a mated patch is decided where every
other touch between copies is, by `topo::record_at_rest`, and the gate
reads the census findings the gather kept (`Product::census`) without
deciding anything again. What remains is `record_at_rest`'s own confirm
pass, which re-runs the census over the records it wrote (at most four
passes, `census.rs` `record_at_rest`): that is the door's design for
every recorded touch, the unmated ones included since before this unit,
and not a second site deciding the mated pair.
