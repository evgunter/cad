---
id: a-reduction-row-landing-on-two-minted-cells-refuses-join-desync
kind: issue
title: A reduction-found record landing on two cells the join minted refuses JoinDesync; the boolean's door decides no such meeting
status: closed
opened: 2026-11-26
priority: P3
cost: M
closed: 2026-11-26
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

## 2026-11-26 — closed: no reduction-found row reaches it (PR 7808 review, m4)

The search for a witness found that none can exist.

- **The suite.** No run reaches the refusal text: the full default
  profiles of topo and sweep on head and base (the slow set included),
  `ci` at 1e-12, and the review's mutant runs.
- **The code.** `records::landings` marks a landing `minted` only where
  both record cells' images are cells their divisions minted. `place_found`
  divides at most one of a found row's two cells. A row naming two faces
  the join divided refuses first, with its own `JoinDesync` ("names two
  faces the join divided"), and the other cell's fate is `Kept`, which
  mints nothing.

So the refusal is a guard on a case the placement cannot build, not a
missing door. A row whose two faces the join both divides is that other
refusal's question, which no row here holds.
