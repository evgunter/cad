---
id: an-escalated-census-finding-names-no-pair
kind: issue
title: An escalated at-rest census finding names no pair, so it cannot be carried or matched to the pair it is about
status: open
opened: 2026-10-16
priority: P2
cost: M
---

An at-rest face pair the census reads in the sliver band escalates as
`ValidationError::CensusEscalated { cause }` (`crates/topo/src/census.rs`,
`rest_door_one` and the pushes beside it), which names no entity. The
assembly gate's own comment says of it that there is "nothing to attribute".

Consequences:
- a child document's escalation cannot be carried across the part seam as a
  finding about a pair, so the parent cannot tell that the pair was already
  answered (see `work/intent/a-part-drops-its-census-findings-at-the-seam.md`);
- the user is told an escalation happened, not between which two cells.

The census knows the two faces when it escalates. The finding should carry
them (as `CensusUndecidable { a, b, .. }` already carries its two entities),
so that every census finding about a pair names the pair.

Found by the census-over-solids designers (PR 4666, 2026-10-16). Not run.
