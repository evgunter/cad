---
id: a-reduction-row-landing-on-two-minted-cells-refuses-join-desync
kind: issue
title: A reduction-found record landing on two cells the join minted refuses JoinDesync; the boolean's door decides no such meeting
status: open
opened: 2026-11-26
priority: P3
cost: M
---


Found by the `heldrecords/minted-door` lane's sweep of the carry's
callers of `records::landings`.

## What

The boolean's carry decides a minted meeting at its door for an
operand's carried record (`boolean::ops` `carried`, `minted_meeting`).
A record the reduction found takes the same placement on a face the
join divided after it was written (`boolean::ops` `place_found`), and
there every landing on two cells the join minted refuses
`JoinDesync` ("a row the reduction found lands on two cells the join
minted"). The README carry clause says the boolean's tool "decides
every such meeting at its door"; this path decides none.

## Consequence

A reduction-found record whose cells the join re-cut so that two
minted cells meet at a point refuses loudly where it could land citing
a decision. No test is known to reach it: the lane did not search for
a witness.

## Way through

Decide the meeting there as `carried` does (`minted_meeting`), its row
on the found record's own cells, and cite it beside the record.
