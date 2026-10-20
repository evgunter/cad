---
id: an-at-rest-shared-crescent-reads-as-a-contradicted-contact-not-an-interference-finding
kind: issue
title: At rest, a shared crescent on a two-carrier face-face record reads ContactContradicted rather than an interference finding
status: open
opened: 2026-10-12
priority: P3
cost: E
---


## What

census reads a shared crescent (both materials fill the thin region beside a tangent locus) on a two-carrier face-face record as ContactContradicted { Tangent }; D10's Assertions paragraph makes an overlap between two copies an interference finding. Decide which the census reports.

Found by a designer of the overlapping-tangency fork (fork log row 118), read from the code at 63288134b8, not measured.

## Note (PR 4831)

Untouched by the claim-free glue build: the census confirm
(`tangent_table_at_rest` → `contact_verify::tangent_locus_relation`) still
reads the crescent as the `Tangent` contact's contradiction. The ruling on
PR 4775 places the crescent check there and calls a shared crescent
interference; which refusal the census reports is this row's.

## Answered (Ev, `[ev]` PR 5395, 2026-10-20)

Between two distinct solids a shared crescent is interference (C6), not
`ContactContradicted`: the census reports it as an interference
finding. Inside one solid, along a line it is a missing edge (the op's
defect), and at a point touch it is legal (C4).
