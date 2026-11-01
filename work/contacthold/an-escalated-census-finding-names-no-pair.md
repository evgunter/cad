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

## 2026-10-17 — parked on the D10 hold, moved from RESTREAD

RESTREAD read this row against the D10 hold (`docs/DESIGN.md`, State of
the build) and the open [ev] PR 4666 (the census records what it
decides): it stands on contact records, declared pairs or the shape of
at-rest findings, which INTENT stages 4 and 5 rebuild. Per RESTREAD's
plan (a row needing declared pairs or contact parks and moves to
CONTACTHOLD), it moved here by `git mv` with its id unchanged. Priced
`M` where it carried no cost. (RESTREAD orchestrator)

## 2026-10-26 — evidence from stage 5 C

With the at-rest gate gone (`the-at-rest-census-is-a-check`), an
escalation the census re-finds inside an instance's copy is no longer
hidden behind the part's carried refusal: the in-arrival discard cannot
place a finding that names no pair, so it is kept, and reported twice,
once as the part's carried row and once as the host's `NoLane`. Stage
5 C's review retired a by-value match that dropped such a verdict when
it equalled a carried row: it reddened no row (since #5782 nothing in the
corpus escalates pair-less), it was wrong for a part placed twice, and
it could drop a genuine escalation of the host's own that happened to
equal a part's. A loud double report is the safer error. Naming the pair
lets the in-arrival discard place the re-found verdict inside its copy,
which ends the double report. No row pins it today: the escalation rows
of `intent_part_census_seam` read `ThinInBand` since #5782.

**Re-read 2026-10-29 (INTENT, D10 hold):** released. Stage 5 made the census a check (`the-at-rest-census-is-a-check`, evidence above). `ValidationError::CensusEscalated` still carries only `cause` (`crates/topo/src/validate.rs:1967`), so the finding shape this row changes is settled and still owed.
