---
id: a-boolean-reading-two-bodies-of-one-arrival-re-decides-their-contact
kind: issue
title: A boolean reading two bodies of one arrival re-decides a contact the source's census already decided
status: open
opened: 2026-10-19
priority: P2
cost: M
design: true
refs: [each-space-runs-its-census-between-arrivals]
---

Found by the `a-space-decides-its-own-contacts` designers (PR 4666 follow-up, 2026-10-19). Space S carries a bolt and a plate, never booleaned, and S's census decides their contact at rest. A boolean in a space T that places S reads two picks of one copy of S, the bolt and the plate. It decides their coincidence again at its own door, in T's frame.

D1 `[topology.records]` says operand coincidences are "carried through the op's lineage". But S's record sits on S's aggregate, not on either operand body, so the boolean has no carried record to read. The final state probably has the boolean read the carried record between its operands, keeping D10's order-free reading. Weigh it when `each-space-runs-its-census-between-arrivals` lands.
